extern crate serde_json;
extern crate zmq;

use std::cell::RefCell;
use std::thread;

use errors::common::CommonError;
use utils::json::{JsonDecodable, JsonEncodable};
use utils::sequence::SequenceUtils;

struct AgentWorker {
    cmd_socket: zmq::Socket,
}

struct Agent {
    cmd_socket: zmq::Socket,
    worker: Option<thread::JoinHandle<()>>,
}

impl Drop for Agent {
    fn drop(&mut self) {
        trace!("agent drop >>");
        self.cmd_socket.send_str("exit", zmq::DONTWAIT).unwrap(); //TODO
        self.worker.take().unwrap().join().unwrap();
        trace!("agent drop <<");
    }
}

pub struct AgentService {
    agent: RefCell<Option<Agent>>,
}

impl Agent {
    pub fn new() -> Agent {
        let ctx = zmq::Context::new();
        let (send_soc, recv_soc) = _create_zmq_pair("agent").unwrap();
        let mut worker = AgentWorker {
            cmd_socket: recv_soc,
        };
        Agent {
            cmd_socket: send_soc,
            worker: Some(thread::spawn(move || { worker.run() }))
        }
    }
}

impl AgentService {
    pub fn new() -> AgentService {
        AgentService { agent: RefCell::new((None)) }
    }

    pub fn connect(&self, sender_did: &str, my_sk: &str, my_pk: &str, endpoint: &str) -> Result<(), CommonError> {
        let mut agent = self.agent.borrow_mut();
        if agent.is_none() {
            *agent = Some(Agent::new());
        }
        let conn_handle = SequenceUtils::get_next_id();
        let connect_cmd: AgentWorkerCommand = AgentWorkerCommand::Connect(ConnectCmd {
            my_did: sender_did.to_string(),
            my_sk: my_sk.to_string(),
            my_pk: my_pk.to_string(),
            endpoint: endpoint.to_string(),
        });
        agent.as_ref().unwrap().cmd_socket.send_str(connect_cmd.to_json().unwrap().as_str(), zmq::DONTWAIT).unwrap();
        Ok(())
    }
}

impl AgentWorker {
    pub fn run(&mut self) {
        trace!("agent worker run before poll");
        self.cmd_socket.poll(zmq::POLLIN, -1).unwrap();
        let s = self.cmd_socket.recv_string(zmq::DONTWAIT);
        trace!("agent worker run after poll {:?}", s);
    }
}

#[serde(tag = "cmd")]
#[derive(Serialize, Deserialize, Debug)]
enum AgentWorkerCommand {
    Connect(ConnectCmd),
}

impl JsonEncodable for AgentWorkerCommand {}

impl<'a> JsonDecodable<'a> for AgentWorkerCommand {}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectCmd {
    endpoint: String,
    my_did: String,
    my_sk: String,
    my_pk: String,
}

fn _create_zmq_pair(addr: &str) -> Result<(zmq::Socket, zmq::Socket), zmq::Error> {
    let ctx = zmq::Context::new();
    let recv_soc = ctx.socket(zmq::SocketType::PAIR)?;
    let send_soc = ctx.socket(zmq::SocketType::PAIR)?;
    recv_soc.bind(&format!("inproc://{}", addr))?;
    send_soc.connect(&format!("inproc://{}", addr))?;
    Ok((send_soc, recv_soc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_can_be_dropped() {
        let ctx = zmq::Context::new();
        {
            let agent = Agent::new();
        }
        assert!(true, "No fail in agent_worker_can_be_dropped");
    }

    #[test]
    fn agent_service_connect_works() {
        use std::sync::mpsc::channel;
        use utils::timeout::TimeoutUtils;
        let (sender, receiver) = channel();
        let (send_soc, recv_soc) = _create_zmq_pair("test_connect").unwrap();
        let agent = Agent {
            cmd_socket: send_soc,
            worker: Some(thread::spawn(move || {
                sender.send(recv_soc.recv_string(0).unwrap().unwrap()).unwrap()
            }))
        };
        let agent_service = AgentService {
            agent: RefCell::new(Some(agent)),
        };
        agent_service.connect("sd", "sk", "pk", "ep").unwrap();
        let str = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        assert_eq!(str, r#"{"cmd":"Connect","endpoint":"ep","my_did":"sd","my_sk":"sk","my_pk":"pk"}"#);
    }
}