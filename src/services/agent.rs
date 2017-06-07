#![warn(unused_variables)]
extern crate rust_base58;
extern crate serde_json;
extern crate zmq;

use self::rust_base58::FromBase58;
use std::cell::RefCell;
use std::error::Error;
use std::thread;

use commands::{Command, CommandExecutor};
use commands::agent::AgentCommand;
use errors::common::CommonError;
use utils::json::{JsonDecodable, JsonEncodable};
use utils::sequence::SequenceUtils;

struct RemoteAgent {
    socket: zmq::Socket,
    addr: String,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
    server_key: Vec<u8>,
    conn_handle: i32,
}

struct AgentListener {
    socket: zmq::Socket,
    connections: Vec<(i32, String)>, // (connection_handle, identity for zmq (did)
}

struct AgentWorker {
    cmd_socket: zmq::Socket,
    agent_connections: Vec<RemoteAgent>,
    agent_listeners: Vec<AgentListener>,
}

struct Agent {
    cmd_socket: zmq::Socket,
    worker: Option<thread::JoinHandle<()>>,
}

impl Drop for Agent {
    fn drop(&mut self) {
        trace!("agent drop >>");
        self.cmd_socket.send_str(AgentWorkerCommand::Exit.to_json().unwrap().as_str(), zmq::DONTWAIT).unwrap(); //TODO
        self.worker.take().unwrap().join().unwrap();
        trace!("agent drop <<");
    }
}

pub struct AgentService {
    agent: RefCell<Agent>,
}

impl Agent {
    pub fn new() -> Agent {
        let (send_soc, recv_soc) = _create_zmq_socket_pair("agent", true).unwrap();
        let mut worker = AgentWorker {
            cmd_socket: recv_soc,
            agent_connections: Vec::new(),
            agent_listeners: Vec::new(),
        };
        Agent {
            cmd_socket: send_soc,
            worker: Some(thread::spawn(move || { worker.run() }))
        }
    }
}

impl AgentService {
    pub fn new() -> AgentService {
        AgentService { agent: RefCell::new(Agent::new()) }
    }

    pub fn connect(&self, sender_did: &str, my_sk: &str, my_pk: &str, endpoint: &str, server_key: &str) -> Result<i32, CommonError> {
        let agent = self.agent.borrow_mut();
        let conn_handle = SequenceUtils::get_next_id();
        let connect_cmd: AgentWorkerCommand = AgentWorkerCommand::Connect(ConnectCmd {
            did: sender_did.to_string(),
            secret_key: my_sk.to_string(),
            public_key: my_pk.to_string(),
            endpoint: endpoint.to_string(),
            server_key: server_key.to_string(),
            conn_handle: conn_handle,
        });
        agent.cmd_socket.send_str(connect_cmd.to_json()
                                      .map_err(|err|
                                          CommonError::InvalidState(format!("Can't serialize AgentWorkerCommand::Connect {}", err.description())))?
                                      .as_str(), zmq::DONTWAIT)?;
        Ok(conn_handle)
    }

    pub fn listen(&self) -> Result<i32, CommonError> {
        let agent = self.agent.borrow_mut();
        let listen_handle = SequenceUtils::get_next_id();
        let listen_cmd = AgentWorkerCommand::Listen(ListenCmd { listen_handle: listen_handle });
        agent.cmd_socket.send_str(listen_cmd.to_json()
                                      .map_err(|err|
                                          CommonError::InvalidState(format!("Can't serialize AgentWorkerCommand::Listen {}", err.description())))?
                                      .as_str(), zmq::DONTWAIT)?;
        Ok(listen_handle)
    }
}

impl AgentWorker {
    fn run(&mut self) {
        'agent_pool_loop: loop {
            trace!("agent worker poll loop >>");
            let cmds = self.poll().unwrap();
            for cmd in cmds {
                info!("received cmd {:?}", cmd);
                match cmd {
                    AgentWorkerCommand::Connect(cmd) => self.connect(&cmd).unwrap(),
                    AgentWorkerCommand::Listen(cmd) => self.start_listen(cmd.listen_handle).unwrap(),
                    AgentWorkerCommand::Response(resp) => self.agent_connections[resp.agent_ind].handle_response(resp.msg),
                    AgentWorkerCommand::Request(_) => unimplemented!(),
                    AgentWorkerCommand::Exit => break 'agent_pool_loop,
                }
            }
            trace!("agent worker poll loop <<");
        }
        trace!("agent poll finished");
    }

    fn connect(&mut self, cmd: &ConnectCmd) -> Result<(), CommonError> {
        let ra = RemoteAgent::new(cmd.public_key.as_str(), cmd.secret_key.as_str(),
                                  cmd.server_key.as_str(), cmd.endpoint.as_str(),
                                  cmd.conn_handle)
            .map_err(map_err_trace!("RemoteAgent::new failed"))?;
        ra.connect().map_err(map_err_trace!("RemoteAgent::connect failed"))?;
        self.agent_connections.push(ra);
        Ok(())
    }

    fn start_listen(&mut self, handle: i32) -> Result<(), CommonError> {
        let res = self.try_start_listen();
        let cmd = AgentCommand::ListenAck(handle, res.map(|endpoint| (handle, endpoint)));
        CommandExecutor::instance().send(Command::Agent(cmd))
    }

    fn try_start_listen(&mut self) -> Result<String, CommonError> {
        let sock = zmq::Context::new().socket(zmq::SocketType::ROUTER)?;
        //FIXME setup keys
        sock.bind("tcp://0.0.0.0:*")?; //TODO configure base IP?
        let endpoint = sock.get_last_endpoint()?
            .map_err(|err|
                CommonError::InvalidState(
                    format!("Can't decode socket endpoint after bind for listener {:?}.",
                            err)))?;
        let al = AgentListener {
            socket: sock,
            connections: Vec::new(),
        };
        self.agent_listeners.push(al);
        info!("Agent listener started at {}", endpoint);
        Ok(endpoint)
    }

    fn poll(&self) -> Result<Vec<AgentWorkerCommand>, zmq::Error> {
        let mut result = Vec::new();
        let mut poll_items: Vec<zmq::PollItem> = Vec::new();
        poll_items.push(self.cmd_socket.as_poll_item(zmq::POLLIN));
        let agent_connections_cnt = self.agent_connections.len();
        let agent_listeners_cnt = self.agent_listeners.len();

        for agent_conn in &self.agent_connections {
            poll_items.push(agent_conn.socket.as_poll_item(zmq::POLLIN));
        }

        zmq::poll(poll_items.as_mut_slice(), -1).map_err(map_err_trace!("agent poll failed"))?;

        if poll_items[0].is_readable() {
            let msg = self.cmd_socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
            info!("Input on cmd socket {}", msg);
            result.push(AgentWorkerCommand::from_json(msg.as_str()).unwrap());
        }

        for i in 0..agent_connections_cnt {
            if poll_items[1 + i].is_readable() {
                let msg = self.agent_connections[i].socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                info!("Input on agent socket {}: {}", i, msg);
                result.push(AgentWorkerCommand::Response(Response {
                    agent_ind: i,
                    msg: msg,
                }))
            }
        }
        for i in 0..agent_listeners_cnt {
            if poll_items[1 + agent_connections_cnt + i].is_readable() {
                let identity = self.agent_listeners[i].socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                let msg = self.agent_listeners[i].socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                info!("Input on listener socket {}: identity {} send msg {}", i, identity, msg);
                result.push(AgentWorkerCommand::Request(Request {
                    listener_ind: i,
                    identity: identity,
                    msg: msg,
                }))
            }
        }

        Ok(result)
    }
}

impl RemoteAgent {
    fn new(pub_key: &str, sec_key: &str, ver_key: &str, addr: &str, conn_handle: i32) -> Result<RemoteAgent, CommonError> {
        Ok(RemoteAgent {
            socket: zmq::Context::new().socket(zmq::SocketType::DEALER)?,
            public_key: pub_key.from_base58()
                .map_err(|err| CommonError::InvalidStructure(format!("invalid pub_key {}", err)))?,
            secret_key: sec_key.from_base58()
                .map_err(|err| CommonError::InvalidStructure(format!("invalid sec_key {}", err)))?,
            server_key: ver_key.from_base58()
                .map_err(|err| CommonError::InvalidStructure(format!("invalid server_key {}", err)))?,
            addr: addr.to_string(),
            conn_handle: conn_handle,
        })
    }

    fn connect(&self) -> Result<(), CommonError> {
        impl From<zmq::EncodeError> for CommonError {
            fn from(err: zmq::EncodeError) -> CommonError {
                CommonError::InvalidState(format!("Invalid data stored RemoteAgent detected while connect {:?}", err))
            }
        }
        self.socket.set_identity(zmq::z85_encode(self.public_key.as_slice())?.as_bytes())
            .map_err(map_err_trace!())?;
        self.socket.set_curve_secretkey(zmq::z85_encode(self.secret_key.as_slice())?.as_str())
            .map_err(map_err_trace!())?;
        self.socket.set_curve_publickey(zmq::z85_encode(self.public_key.as_slice())?.as_str())
            .map_err(map_err_trace!())?;
        self.socket.set_curve_serverkey(zmq::z85_encode(self.server_key.as_slice())?.as_str())
            .map_err(map_err_trace!())?;
        self.socket.set_linger(0).map_err(map_err_trace!())?; //TODO set correct timeout
        self.socket.connect(self.addr.as_str())
            .map_err(map_err_trace!("RemoteAgent::connect self.socket.connect failed"))?;
        self.socket.send_str("DID", zmq::DONTWAIT).map_err(map_err_trace!())?;
        Ok(())
    }

    fn handle_response(&self, msg: String) {
        if msg.eq("DID_ACK") {
            let send_res: Result<(), CommonError> =
                CommandExecutor::instance().send(
                    Command::Agent(
                        AgentCommand::ConnectAck(self.conn_handle, Ok(self.conn_handle))));
            if let Err(err) = send_res {
                error!("RemoteAgent::handle_response got connection ack, but can't send to client {}", err);
            };
        } else {
            //check state, transfer message to client
            unimplemented!();
        }
    }
}

#[serde(tag = "cmd")]
#[derive(Serialize, Deserialize, Debug)]
enum AgentWorkerCommand {
    Connect(ConnectCmd),
    Listen(ListenCmd),
    Response(Response),
    Request(Request),
    Exit,
}

impl JsonEncodable for AgentWorkerCommand {}

impl<'a> JsonDecodable<'a> for AgentWorkerCommand {}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectCmd {
    endpoint: String,
    did: String,
    secret_key: String,
    public_key: String,
    server_key: String,
    conn_handle: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListenCmd {
    listen_handle: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    agent_ind: usize,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    listener_ind: usize,
    identity: String,
    msg: String,
}

fn _create_zmq_socket_pair(address: &str, connect_and_bind: bool) -> Result<(zmq::Socket, zmq::Socket), zmq::Error> {
    let ctx = zmq::Context::new();
    let recv_soc = ctx.socket(zmq::SocketType::PAIR)?;
    let send_soc = ctx.socket(zmq::SocketType::PAIR)?;
    if connect_and_bind {
        let address = format!("inproc://{}", address);
        recv_soc.bind(&address)?;
        send_soc.connect(&address)?;
    }
    Ok((send_soc, recv_soc))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::mpsc::channel;

    use utils::timeout::TimeoutUtils;

    #[test]
    fn agent_can_be_dropped() {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            #[allow(unused_variables)]
            {
                let agent = Agent::new();
            }
            sender.send(true).unwrap();
        });
        receiver.recv_timeout(TimeoutUtils::short_timeout()).expect("drop not finished");
    }

    mod agent_service {
        use super::*;

        #[test]
        fn agent_service_connect_works() {
            let (sender, receiver) = channel();
            let (send_soc, recv_soc) = _create_zmq_socket_pair("test_connect", true).unwrap();
            let agent = Agent {
                cmd_socket: send_soc,
                worker: Some(thread::spawn(move || {
                    sender.send(recv_soc.recv_string(0).unwrap().unwrap()).unwrap()
                }))
            };
            let agent_service = AgentService {
                agent: RefCell::new(agent),
            };
            let conn_handle = agent_service.connect("sd", "sk", "pk", "ep", "serv").unwrap();
            let expected_cmd = ConnectCmd {
                server_key: "serv".to_string(),
                public_key: "pk".to_string(),
                secret_key: "sk".to_string(),
                endpoint: "ep".to_string(),
                did: "sd".to_string(),
                conn_handle: conn_handle,
            };
            let str = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            assert_eq!(str, AgentWorkerCommand::Connect(expected_cmd).to_json().unwrap());
        }

        #[test]
        fn agent_service_listen_works() {
            let (sender, receiver) = channel();
            let (send_soc, recv_soc) = _create_zmq_socket_pair("test_connect", true).unwrap();
            let agent = Agent {
                cmd_socket: send_soc,
                worker: Some(thread::spawn(move || {
                    sender.send(recv_soc.recv_string(0).unwrap().unwrap()).unwrap()
                }))
            };
            let agent_service = AgentService {
                agent: RefCell::new(agent),
            };
            let conn_handle = agent_service.listen().unwrap();
            let expected_cmd = ListenCmd {
                listen_handle: conn_handle,
            };
            let str = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            assert_eq!(str, AgentWorkerCommand::Listen(expected_cmd).to_json().unwrap());
        }
    }

    mod agent_worker {
        use super::*;
        use super::rust_base58::ToBase58;

        #[test]
        fn agent_worker_connect_works() {
            ::utils::logger::LoggerUtils::init();
            let send_key_pair = zmq::CurveKeyPair::new().unwrap();
            let recv_key_pair = zmq::CurveKeyPair::new().unwrap();
            let ctx = zmq::Context::new();
            let recv_soc = ctx.socket(zmq::SocketType::ROUTER).unwrap();
            recv_soc.set_curve_publickey(recv_key_pair.public_key.as_str()).unwrap();
            recv_soc.set_curve_secretkey(recv_key_pair.secret_key.as_str()).unwrap();
            recv_soc.set_curve_server(true).unwrap();
            recv_soc.bind("tcp://127.0.0.1:*").unwrap();
            let addr = recv_soc.get_last_endpoint().unwrap().unwrap();
            info!("addr {}", addr);

            let mut agent_worker = AgentWorker {
                agent_connections: Vec::new(),
                agent_listeners: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };
            let cmd = ConnectCmd {
                endpoint: addr,
                public_key: zmq::z85_decode(send_key_pair.public_key.as_str()).unwrap().to_base58(),
                secret_key: zmq::z85_decode(send_key_pair.secret_key.as_str()).unwrap().to_base58(),
                did: "".to_string(),
                server_key: zmq::z85_decode(recv_key_pair.public_key.as_str()).unwrap().to_base58(),
                conn_handle: 0,
            };

            agent_worker.connect(&cmd).unwrap();

            assert_eq!(agent_worker.agent_connections.len(), 1);
            recv_soc.recv_string(0).unwrap().unwrap(); //ignore identity
            assert_eq!(recv_soc.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "DID");
        }

        #[test]
        fn agent_worker_poll_works_for_cmd_socket() {
            let (send_soc, recv_soc) = _create_zmq_socket_pair("aw_poll_cmd", true).unwrap();
            let agent_worker = AgentWorker {
                agent_connections: Vec::new(),
                agent_listeners: Vec::new(),
                cmd_socket: recv_soc,
            };
            send_soc.send_str(r#"{"cmd": "Exit"}"#, zmq::DONTWAIT).unwrap();

            let cmds = agent_worker.poll().unwrap();

            assert_eq!(cmds.len(), 1);
            assert_match!(AgentWorkerCommand::Exit, cmds[0]);
        }

        #[test]
        fn agent_worker_poll_works_for_agent_socket() {
            let (send_soc, recv_soc) = _create_zmq_socket_pair("aw_poll_cmd", true).unwrap();
            let agent_worker = AgentWorker {
                agent_connections: vec!(RemoteAgent {
                    socket: recv_soc,
                    addr: String::new(),
                    public_key: Vec::new(),
                    secret_key: Vec::new(),
                    server_key: Vec::new(),
                    conn_handle: 0,
                }),
                agent_listeners: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };
            send_soc.send_str("msg", zmq::DONTWAIT).unwrap();

            let mut cmds = agent_worker.poll().unwrap();

            assert_eq!(cmds.len(), 1);
            let cmd = cmds.remove(0);
            match cmd {
                AgentWorkerCommand::Response(resp) => {
                    assert_eq!(resp.agent_ind, 0);
                    assert_eq!(resp.msg, "msg");
                }
                _ => panic!("unexpected cmd {:?}", cmd),
            }
        }

        #[test]
        fn agent_worker_try_start_listen_works() {
            let mut agent_worker = AgentWorker {
                agent_connections: Vec::new(),
                agent_listeners: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };

            let endpoint = agent_worker.try_start_listen().unwrap();
            assert!(endpoint.starts_with("tcp://0.0.0.0:"));
            assert_eq!(agent_worker.agent_listeners.len(), 1);

            let msg = "msg";
            let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
            sock.connect(endpoint.as_str()).unwrap();
            sock.send_str(msg, 0).unwrap();
            agent_worker.agent_listeners[0].socket.recv_bytes(0).unwrap(); //ignore identity
            let act_msg = agent_worker.agent_listeners[0].socket.recv_string(0).unwrap().unwrap();
            assert_eq!(act_msg, msg);
        }
    }

    #[test]
    fn remote_agent_connect_works() {
        let dest = "test_agent_connect";
        let addr: String = format!("inproc://{}", dest);
        let (send_soc, recv_soc) = _create_zmq_socket_pair(dest, false).unwrap();
        recv_soc.bind(addr.as_str()).unwrap(); //TODO enable CurveCP
        let send_key_pair = zmq::CurveKeyPair::new().unwrap();
        let recv_key_pair = zmq::CurveKeyPair::new().unwrap();
        let agent = RemoteAgent {
            socket: send_soc,
            addr: addr,
            server_key: zmq::z85_decode(send_key_pair.public_key.as_str()).unwrap(),
            secret_key: zmq::z85_decode(recv_key_pair.secret_key.as_str()).unwrap(),
            public_key: zmq::z85_decode(recv_key_pair.public_key.as_str()).unwrap(),
            conn_handle: 0,
        };
        agent.connect().unwrap();
        assert_eq!(recv_soc.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "DID");
    }

    #[test]
    fn agent_service_static_create_zmq_socket_pair_works() {
        let msg = "msg";
        let sockets = _create_zmq_socket_pair("test_pair", true).unwrap();
        sockets.0.send_str(msg, zmq::DONTWAIT).unwrap();
        assert_eq!(sockets.1.recv_string(zmq::DONTWAIT).unwrap().unwrap(), msg);
    }
}