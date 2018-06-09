extern crate zmq;

use self::zmq::PollItem;
use self::zmq::Socket as ZSocket;

use errors::common::CommonError;
use errors::pool::PoolError;
use services::pool::events::*;
use services::pool::types::*;

pub trait Networker {
    fn new() -> Self;
    fn fetch_events(&self, poll_items: &[PollItem]) -> Vec<PoolEvent>;
    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent>;
    fn get_timeout(&self) -> i64;
    fn get_poll_items(&self) -> Vec<PollItem>;
}

pub struct ZMQNetworker {
    pool_connections: Vec<PoolConnection>,
    nodes: Vec<RemoteNode>,
}

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {
            pool_connections: Vec::new(),
            nodes: Vec::new(),
        }
    }

    fn fetch_events(&self, poll_items: &[PollItem]) -> Vec<PoolEvent> {
        let mut events = Vec::new();
        for pc in &self.pool_connections {
            events.extend(pc.fetch_events(poll_items).into_iter());
        }
        events
    }

    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        match pe {
            Some(NetworkerEvent::SendAllRequest(_)) | Some(NetworkerEvent::SendOneRequest(_)) => {
                if self.pool_connections.last().map(PoolConnection::is_full).unwrap_or(true) {
                    self.pool_connections.push(PoolConnection::new(self.nodes.clone()));
                }
                self.pool_connections.last().unwrap().send_request(pe);
                None
            },
            Some(NetworkerEvent::NodesStateUpdated(nodes)) => {
                self.nodes = nodes;
                None
            }
            _ => unimplemented!()
        }
    }

    fn get_timeout(&self) -> i64 {
        self.pool_connections.iter()
            .fold(::std::i64::MAX, |acc, cur| ::std::cmp::min(acc, PoolConnection::get_timeout(cur)))
    }

    fn get_poll_items(&self) -> Vec<PollItem>{
        self.pool_connections.iter()
            .flat_map(PoolConnection::get_poll_items).collect()
    }
}

pub struct PoolConnection {
    nodes: Vec<RemoteNode>,
    sockets: Vec<Option<ZSocket>>,
    ctx: zmq::Context,
    key_pair: zmq::CurveKeyPair,
}

impl PoolConnection {
    fn new(nodes: Vec<RemoteNode>) -> Self {
        let mut sockets: Vec<Option<ZSocket>> = Vec::new();
        // FIXME restore for _ in 0..nodes.len() { sockets.push(None); }
        // FIXME should be lazy:
        let ctx = zmq::Context::new();
        let key_pair = zmq::CurveKeyPair::new().expect("FIXME");
        for node in &nodes {
            sockets.push(Some(node.connect(&ctx, &key_pair).expect("FIXME")))
        }

        PoolConnection {
            nodes,
            sockets,
            ctx,
            key_pair,
        }
    }

    fn fetch_events(&self, poll_items: &[zmq::PollItem]) -> Vec<PoolEvent> {
        let mut vec = Vec::new();
        let mut pi_idx = 0;
        let len = self.nodes.len();
        assert_eq!(len, self.sockets.len());
        for i in 0..len {
            if let (&Some(ref s), rn) = (&self.sockets[i], &self.nodes[i]) {
                if poll_items[pi_idx].is_readable() {
                    if let Ok(Ok(str)) = s.recv_string(zmq::DONTWAIT) {
                        vec.push(PoolEvent::NodeReply(
                            str,
                            rn.name.clone(),
                        ))
                    }
                }
                pi_idx += 1;
            }
        }
        vec
    }

    fn get_poll_items(&self) -> Vec<PollItem> {
        self.sockets.iter()
            .flat_map(|zs: &Option<ZSocket>| zs.as_ref().map(|zs| zs.as_poll_item(zmq::POLLIN)))
            .collect()
    }

    fn get_timeout(&self) -> i64 {
        -1 //FIXME
    }

    fn is_full(&self) -> bool {
        false //FIXME
    }

    fn send_request(&self, pe: Option<NetworkerEvent>) {
        match pe {
            Some(NetworkerEvent::SendOneRequest(msg)) => {
                let socket: &ZSocket = self.sockets[0].as_ref().expect("FIXME");
                socket.send_str(&msg, zmq::DONTWAIT).expect("FIXME");
            }
            _ => unimplemented!()
        }
    }
}

impl RemoteNode {
    fn connect(&self, ctx: &zmq::Context, key_pair: &zmq::CurveKeyPair) -> Result<ZSocket, PoolError> {
        let s = ctx.socket(zmq::SocketType::DEALER)?;
        s.set_identity(key_pair.public_key.as_bytes())?;
        s.set_curve_secretkey(&key_pair.secret_key)?;
        s.set_curve_publickey(&key_pair.public_key)?;
        s.set_curve_serverkey(
            zmq::z85_encode(self.public_key.as_slice())
                .map_err(|err| { CommonError::InvalidStructure(format!("Can't encode server key as z85: {:?}", err)) })?
                .as_str())?;
        s.set_linger(0)?; //TODO set correct timeout
        s.connect(&self.zaddr)?;
        Ok(s)
    }
}

#[cfg(test)]
pub struct MockNetworker {}

#[cfg(test)]
impl Networker for MockNetworker {
    fn new() -> Self {
        unimplemented!()
    }

    fn fetch_events(&self, poll_items: &[zmq::PollItem]) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        unimplemented!()
    }

    fn get_timeout(&self) -> i64 {
        unimplemented!()
    }

    fn get_poll_items(&self) -> Vec<PollItem>{
        unimplemented!()
    }
}


#[cfg(test)]
mod networker_tests {
    use super::*;
    
    #[test]
    pub fn networker_new_works() {
        Networker::new();
    }

    #[test]
    pub fn networker_process_event_works() {
        let networker = Networker::new();
        networker.process_event(None);
    }
}