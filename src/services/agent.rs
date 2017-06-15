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
    connections: Vec<(i32 /* connection_handle*/, String /* identity */)>,
    listener_handle: i32,
    socket: zmq::Socket,
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
        let agent = self.agent.try_borrow_mut()?;
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

    pub fn listen(&self, endpoint: &str, pk: &str, sk: &str) -> Result<i32, CommonError> {
        let agent = self.agent.try_borrow_mut()?;
        let listen_handle = SequenceUtils::get_next_id();
        let listen_cmd = AgentWorkerCommand::Listen(ListenCmd {
            listen_handle: listen_handle,
            endpoint: endpoint.to_string(),
            pk: pk.to_string(),
            sk: sk.to_string()
        });
        agent.cmd_socket.send_str(listen_cmd.to_json()
                                      .map_err(|err|
                                          CommonError::InvalidState(format!("Can't serialize AgentWorkerCommand::Listen {}", err.description())))?
                                      .as_str(), zmq::DONTWAIT)?;
        Ok(listen_handle)
    }

    pub fn send(&self, conn_id: i32, msg: Option<&str>) -> Result<i32, CommonError> {
        let agent = self.agent.try_borrow_mut()?;
        let send_handle = SequenceUtils::get_next_id();
        let send_cmd = AgentWorkerCommand::Send(SendCmd {
            cmd_id: send_handle,
            conn_handle: conn_id,
            msg: msg.map(str::to_string),
        });
        agent.cmd_socket.send_str(send_cmd.to_json()
                                      .map_err(|err|
                                          CommonError::InvalidState(format!("Can't serialize AgentWorkerCommand::Send {}", err.description())))?
                                      .as_str(), zmq::DONTWAIT)?;
        Ok(send_handle)
    }
}

impl AgentWorker {
    fn run(&mut self) {
        'agent_pool_loop: loop {
            trace!("agent worker poll loop >>");
            let cmds = self.poll().unwrap();
            for cmd in cmds {
                debug!("AgentWorker::run received cmd {:?}", cmd);
                match cmd {
                    AgentWorkerCommand::Connect(cmd) => self.connect(&cmd).unwrap(),
                    AgentWorkerCommand::Listen(cmd) => self.start_listen(cmd.listen_handle, cmd.endpoint, cmd.pk, cmd.sk).unwrap(),
                    AgentWorkerCommand::Response(resp) => self.agent_connections[resp.agent_ind].handle_response(resp.msg),
                    AgentWorkerCommand::Request(req) => self.agent_listeners[req.listener_ind].handle_request(req.identity, req.msg),
                    AgentWorkerCommand::Send(cmd) => self.send(cmd.cmd_id, cmd.conn_handle, cmd.msg).unwrap(),
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

    fn start_listen(&mut self, handle: i32, endpoint: String, pk: String, sk: String) -> Result<(), CommonError> {
        let res = self.try_start_listen(handle, endpoint, pk, sk);
        let cmd = AgentCommand::ListenAck(handle, res.map(|()| (handle)));
        CommandExecutor::instance().send(Command::Agent(cmd))
    }

    fn send(&mut self, cmd_id: i32, conn_handle: i32, msg: Option<String>)
            -> Result<(), CommonError> {
        let res = self.try_send(conn_handle, msg);
        let cmd = AgentCommand::SendAck(cmd_id, res);
        CommandExecutor::instance().send(Command::Agent(cmd))
    }

    fn try_send(&mut self, handle: i32, msg: Option<String>) -> Result<(), CommonError> {
        let msg = msg.unwrap_or(String::new());

        let remote_agent: Option<&RemoteAgent> =
            self.agent_connections.iter().find(|ac| ac.conn_handle == handle);
        let listener_with_identity: Option<(&AgentListener, &String)> =
            self.find_listener_by_conn_handle(handle);

        if remote_agent.is_some() && listener_with_identity.is_some() {
            return Err(CommonError::InvalidState("duplication connections".to_string())) //TODO
        }
        if let Some(remote_agent) = remote_agent {
            return remote_agent.socket.send(msg.as_bytes(), zmq::DONTWAIT).map_err(From::from)
        }
        if let Some(li) = listener_with_identity {
            let agent_listener: &AgentListener = li.0;
            let identity: &String = li.1;
            return agent_listener.socket
                .send_str(identity.as_str(), zmq::DONTWAIT | zmq::SNDMORE)
                .and_then(|()|
                    agent_listener.socket.send(msg.as_bytes(), zmq::DONTWAIT))
                .map_err(From::from)
        }
        /* if remote_agent.is_none() && listener_with_identity.is_none() */
        Err(CommonError::InvalidStructure(format!("Connection with id {} not founded", handle)))
    }

    fn find_listener_by_conn_handle(&self, handle: i32) -> Option<(&AgentListener, &String)> {
        for listener in &self.agent_listeners {
            if let Some(&(_, ref identity)) = listener.connections.iter().find(|&&(conn_id, _)| conn_id == handle) {
                return Some((listener, identity));
            }
        }
        return None;
    }

    fn try_start_listen(&mut self, handle: i32, endpoint: String, pk: String, sk: String) -> Result<(), CommonError> {
        let listener = AgentListener::new(handle, endpoint.clone(), pk, sk).map_err(map_err_trace!("AgentListener::new"))?;
        self.agent_listeners.push(listener);
        info!("Agent listener started at {}", endpoint);
        Ok(())
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

        for agent_listener in &self.agent_listeners {
            let agent_listener: &AgentListener = agent_listener;
            poll_items.push(agent_listener.socket.as_poll_item(zmq::POLLIN));
        }

        zmq::poll(poll_items.as_mut_slice(), -1).map_err(map_err_trace!("agent poll failed"))?;

        if poll_items[0].is_readable() {
            let msg = self.cmd_socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
            trace!("Input on cmd socket {}", msg);
            result.push(AgentWorkerCommand::from_json(msg.as_str()).unwrap());
        }

        for i in 0..agent_connections_cnt {
            if poll_items[1 + i].is_readable() {
                let msg = self.agent_connections[i].socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
                trace!("Input on remote agent socket {}: {}", i, msg);
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
                trace!("Input on agent listener socket {}: identity {} msg {}", i, identity, msg);
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
        // TODO check state
        let cmd: AgentCommand = if msg.eq("DID_ACK") {
            AgentCommand::ConnectAck(self.conn_handle, Ok(self.conn_handle))
        } else {
            AgentCommand::MessageReceived(self.conn_handle, Ok((self.conn_handle, msg)))
        };
        if let Err(err) = CommandExecutor::instance().send(Command::Agent(cmd)) {
            error!("RemoteAgent::handle_response got incoming msg, but can't send to user {}", err);
        };
    }
}

impl AgentListener {
    fn new(handle: i32, endpoint: String, pk: String, sk: String) -> Result<AgentListener, zmq::Error> {
        let sock = zmq::Context::new().socket(zmq::SocketType::ROUTER)?;
        //TODO use forked zmq and set cb instead of raw keys
        sock.set_curve_publickey(zmq::z85_encode(pk.from_base58().unwrap().as_slice()).unwrap().as_str())?;
        sock.set_curve_secretkey(zmq::z85_encode(sk.from_base58().unwrap().as_slice()).unwrap().as_str())?;
        //TODO /cb instead keys
        sock.set_curve_server(true)?;
        sock.bind(endpoint.as_str())?;
        Ok(AgentListener {
            connections: Vec::new(),
            listener_handle: handle,
            socket: sock,
        })
    }

    fn handle_request(&mut self, identity: String, msg: String) {
        if let Some(&(conn_handle, _)) = self.connections.iter().find(|&&(_, ref id)| identity.eq(id.as_str())) {
            CommandExecutor::instance().send(Command::Agent(AgentCommand::MessageReceived(
                conn_handle, Ok((conn_handle, msg))))).unwrap();
            return;
        }

        info!("New connection to agent listener from {} with msg {}", identity, msg);
        let conn_handle = SequenceUtils::get_next_id();
        self.connections.push((conn_handle, identity.clone()));
        let cmd = AgentCommand::ListenerOnConnect(self.listener_handle,
                                                  Ok((self.listener_handle, conn_handle,
                                                      identity.clone(), msg.clone())));
        CommandExecutor::instance().send(Command::Agent(cmd)).unwrap();
        self.socket.send_str(identity.as_str(), zmq::SNDMORE).unwrap();
        self.socket.send_str("DID_ACK", zmq::DONTWAIT).unwrap();
    }
}

#[serde(tag = "cmd")]
#[derive(Serialize, Deserialize, Debug)]
enum AgentWorkerCommand {
    Connect(ConnectCmd),
    Listen(ListenCmd),
    Response(Response),
    Request(Request),
    Send(SendCmd),
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
    endpoint: String,
    pk: String,
    sk: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SendCmd {
    cmd_id: i32,
    conn_handle: i32,
    msg: Option<String>,
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
                    sender.send(recv_soc.recv_string(0).unwrap().unwrap()).unwrap();
                    recv_soc.recv_string(0).unwrap().unwrap();
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
            let conn_handle = agent_service.listen("endpoint", "pk", "sk").unwrap();
            let expected_cmd = ListenCmd {
                listen_handle: conn_handle,
                endpoint: "endpoint".to_string(),
                pk: "pk".to_string(),
                sk: "sk".to_string(),
            };
            let str = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            assert_eq!(str, AgentWorkerCommand::Listen(expected_cmd).to_json().unwrap());
        }

        #[test]
        fn agent_service_send_works() {
            let (sender, receiver) = channel();
            let (send_soc, recv_soc) = _create_zmq_socket_pair("test_send", true).unwrap();
            let agent = Agent {
                cmd_socket: send_soc,
                worker: Some(thread::spawn(move || {
                    sender.send(recv_soc.recv_string(0).unwrap().unwrap()).unwrap()
                }))
            };
            let agent_service = AgentService {
                agent: RefCell::new(agent),
            };
            let conn_handle = SequenceUtils::get_next_id();
            let msg = Some("test_msg");
            let cmd_id = agent_service.send(conn_handle, msg).unwrap();
            let expected_cmd = SendCmd {
                cmd_id: cmd_id,
                conn_handle: conn_handle,
                msg: msg.map(str::to_string),
            };
            let str = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            assert_eq!(str, AgentWorkerCommand::Send(expected_cmd).to_json().unwrap());
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
            trace!("addr {}", addr);

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
        fn agent_worker_poll_works_for_listener() {
            let identity = "identity";
            let (send_soc, recv_soc) = {
                let ctx = zmq::Context::new();
                let recv_soc = ctx.socket(zmq::ROUTER).unwrap();
                recv_soc.bind("tcp://127.0.0.1:*").unwrap();
                let send_soc = ctx.socket(zmq::DEALER).unwrap();
                send_soc.set_identity(identity.as_bytes()).unwrap();
                send_soc.connect(recv_soc.get_last_endpoint().unwrap().unwrap().as_str()).unwrap();
                (send_soc, recv_soc)
            };
            let agent_worker = AgentWorker {
                agent_listeners: vec!(AgentListener {
                    connections: Vec::new(),
                    listener_handle: 0,
                    socket: recv_soc,
                }),
                agent_connections: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };
            send_soc.send_str("msg", zmq::DONTWAIT).unwrap();

            let mut cmds = agent_worker.poll().unwrap();

            assert_eq!(cmds.len(), 1);
            let cmd = cmds.remove(0);
            match cmd {
                AgentWorkerCommand::Request(req) => {
                    assert_eq!(req.listener_ind, 0);
                    assert_eq!(req.identity, identity);
                    assert_eq!(req.msg, "msg");
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

            let server_keys = zmq::CurveKeyPair::new().unwrap();
            let pk = zmq::z85_decode(server_keys.public_key.as_str()).unwrap().to_base58();
            let sk = zmq::z85_decode(server_keys.secret_key.as_str()).unwrap().to_base58();
            let endpoint = "tcp://0.0.0.0:9700".to_string();
            agent_worker.try_start_listen(0, endpoint.clone(), pk.clone(), sk.clone()).unwrap();
            assert_eq!(agent_worker.agent_listeners.len(), 1);

            let msg = "msg";
            let sock = zmq::Context::new().socket(zmq::SocketType::DEALER).unwrap();
            let kp = zmq::CurveKeyPair::new().unwrap();
            sock.set_curve_publickey(kp.public_key.as_str()).unwrap();
            sock.set_curve_secretkey(kp.secret_key.as_str()).unwrap();
            sock.set_curve_serverkey(server_keys.public_key.as_str()).unwrap();
            sock.connect(endpoint.as_str()).unwrap();
            sock.send_str(msg, 0).unwrap();
            agent_worker.agent_listeners[0].socket.poll(zmq::POLLIN, 1000).unwrap();
            agent_worker.agent_listeners[0].socket.recv_bytes(zmq::DONTWAIT).unwrap(); //ignore identity
            let act_msg = agent_worker.agent_listeners[0].socket.recv_string(zmq::DONTWAIT).unwrap().unwrap();
            assert_eq!(act_msg, msg);
        }

        #[test]
        fn agent_worker_try_send_works_for_not_found() {
            let mut agent_worker = AgentWorker {
                agent_connections: Vec::new(),
                agent_listeners: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };
            let conn_handle = SequenceUtils::get_next_id();

            let res = agent_worker.try_send(conn_handle, None);

            assert_match!(Err(CommonError::InvalidStructure(_)), res);
        }

        #[test]
        fn agent_worker_try_send_works_for_duplicate() {
            let conn_handle = SequenceUtils::get_next_id();
            let mut agent_worker = AgentWorker {
                agent_connections: vec![RemoteAgent {
                    conn_handle: conn_handle,
                    socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
                    public_key: Vec::new(),
                    secret_key: Vec::new(),
                    server_key: Vec::new(),
                    addr: String::new(),
                }],
                agent_listeners: vec![AgentListener {
                    socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
                    connections: vec![(conn_handle, String::new())],
                    listener_handle: SequenceUtils::get_next_id(),
                }],
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };

            let res = agent_worker.try_send(conn_handle, None);

            assert_match!(Err(CommonError::InvalidState(_)), res);
        }

        #[test]
        fn agent_worker_try_send_works_for_listener() {
            let (send_soc, recv_soc) = _create_zmq_socket_pair("aw_poll_cmd", true).unwrap();
            let conn_handle = SequenceUtils::get_next_id();
            let mut agent_worker = AgentWorker {
                agent_connections: Vec::new(),
                agent_listeners: vec![AgentListener {
                    socket: send_soc,
                    connections: vec![(conn_handle, "test_identity".to_string())],
                    listener_handle: SequenceUtils::get_next_id(),
                }],
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };

            agent_worker.try_send(conn_handle, Some("test_str".to_string())).unwrap();
            assert_eq!(recv_soc.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "test_identity");
            assert_eq!(recv_soc.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "test_str");
        }

        #[test]
        fn agent_worker_try_send_works_for_connection() {
            let (send_soc, recv_soc) = _create_zmq_socket_pair("aw_poll_cmd", true).unwrap();
            let conn_handle = SequenceUtils::get_next_id();
            let mut agent_worker = AgentWorker {
                agent_connections: vec![RemoteAgent {
                    conn_handle: conn_handle,
                    socket: send_soc,
                    public_key: Vec::new(),
                    secret_key: Vec::new(),
                    server_key: Vec::new(),
                    addr: String::new(),
                }],
                agent_listeners: Vec::new(),
                cmd_socket: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
            };

            agent_worker.try_send(conn_handle, Some("test_str".to_string())).unwrap();
            assert_eq!(recv_soc.recv_string(zmq::DONTWAIT).unwrap().unwrap(), "test_str");
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
