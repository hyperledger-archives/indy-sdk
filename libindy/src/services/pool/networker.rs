extern crate zmq;
extern crate time;

use self::zmq::PollItem;
use self::zmq::Socket as ZSocket;

use errors::common::CommonError;
use errors::pool::PoolError;
use services::pool::events::*;
use services::pool::types::*;
use std::collections::HashMap;
use std::cell::RefCell;
use super::time::Duration;
use utils::sequence::SequenceUtils;
use time::Tm;

pub trait Networker {
    fn new() -> Self;
    fn fetch_events(&self, poll_items: &[PollItem]) -> Vec<PoolEvent>;
    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent>;
    fn get_timeout(&self) -> ((String, String), i64);
    fn get_poll_items(&self) -> Vec<PollItem>;
}

pub struct ZMQNetworker {
    req_id_mappings: HashMap<String, i32>,
    pool_connections: HashMap<i32, PoolConnection>,
    nodes: Vec<RemoteNode>,
}

const POOL_CON_ACTIVE_TO: i64 = 5;

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {
            req_id_mappings: HashMap::new(),
            pool_connections: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn fetch_events(&self, poll_items: &[PollItem]) -> Vec<PoolEvent> {
        let mut cnt = 0;
        self.pool_connections.iter().map(|(_, pc)| {
            let ocnt = cnt;
            cnt = cnt + pc.sockets.iter().filter(|s| s.is_some()).count();
            pc.fetch_events(&poll_items[ocnt..cnt])
        }).flat_map(|v| v.into_iter()).collect()
    }

    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        match pe.clone() {
            Some(NetworkerEvent::SendAllRequest(_, req_id)) | Some(NetworkerEvent::SendOneRequest(_, req_id)) | Some(NetworkerEvent::Resend(req_id)) => {
                trace!("current mappings: {:?}", self.req_id_mappings);
                let num = self.req_id_mappings.get(&req_id).map(|i| i.clone()).or_else(|| {
                    self.req_id_mappings.values()
                        .fold(HashMap::new(), |mut acc, pc_id| {
                            *acc.entry(pc_id).or_insert(0) += 1;
                            acc
                        }).iter()
                        .filter(|&(pc_id, cnt)|
                            cnt < &5 && self.pool_connections.get(pc_id).map(|pc| pc.is_active()).unwrap_or(false))
                        .last().map(|(pc_id, _)| **pc_id)
                });
                match num {
                    Some(idx) => {
                        trace!("send request in existing conn");
                        self.pool_connections.get_mut(&idx).map(|pc| {
                            pc.send_request(pe);
                        });
                        self.req_id_mappings.insert(req_id.clone(), idx);
                    }
                    None => {
                        trace!("send request in new conn");
                        let pc_id = SequenceUtils::get_next_id();
                        let mut pc = PoolConnection::new(self.nodes.clone());
                        pc.send_request(pe);
                        self.pool_connections.insert(pc_id, pc);
                        self.req_id_mappings.insert(req_id.clone(), pc_id);
                    }
                }
                None
            }
            Some(NetworkerEvent::NodesStateUpdated(nodes)) => {
                self.nodes = nodes;
                None
            }
            Some(NetworkerEvent::ExtendTimeout(req_id, node_alias)) => {
                self.req_id_mappings.get(&req_id).map(
                    |idx| {self.pool_connections.get(idx).map(
                        |pc| {pc.extend_timeout(&req_id, &node_alias);}
                    );}
                );
                None
            }
            Some(NetworkerEvent::CleanTimeout(req_id, node_alias)) => {
                let idx_pc_to_delete = self.req_id_mappings.get(&req_id).and_then(
                    |idx| {
                        let delete = self.pool_connections.get(idx).map(
                            |pc| {
                                pc.clean_timeout(&req_id, node_alias);
                                pc.is_orphaned()
                            }
                        ).unwrap_or(false);

                        if delete {
                            Some(idx)
                        } else {
                            None
                        }
                    }
                );
                if let Some(idx) = idx_pc_to_delete {
                    self.pool_connections.remove(idx);
                }
                None
            }
            Some(NetworkerEvent::Timeout) => {
                let pc_to_delete: Vec<i32> = self.pool_connections.iter()
                    .filter(|(_, v)| v.is_orphaned())
                    .map(|(k, _)| *k)
                    .collect();
                pc_to_delete.iter().for_each(|idx| {
                    self.pool_connections.remove(idx);
                });
                None
            }
            _ => None
        }
    }

    fn get_timeout(&self) -> ((String, String), i64) {
        self.pool_connections.values()
            .map(PoolConnection::get_timeout)
            .min_by(|&(_, val1), &(_, val2)| val1.cmp(&val2))
            .unwrap_or((("".to_string(), "".to_string()), ::std::i64::MAX))
    }

    fn get_poll_items(&self) -> Vec<PollItem> {
        self.pool_connections.iter()
            .flat_map(|(_, pool)| pool.get_poll_items()).collect()
    }
}

pub struct PoolConnection {
    nodes: Vec<RemoteNode>,
    sockets: Vec<Option<ZSocket>>,
    ctx: zmq::Context,
    key_pair: zmq::CurveKeyPair,
    resend: RefCell<HashMap<String, (usize, String)>>,
    timeouts: RefCell<HashMap<(String, String), Tm>>,
    time_created: time::Tm,
}

impl PoolConnection {
    fn new(nodes: Vec<RemoteNode>) -> Self {
        //TODO shuffle nodes
        let mut sockets: Vec<Option<ZSocket>> = Vec::new();
        for _ in 0..nodes.len() { sockets.push(None); }

        PoolConnection {
            nodes,
            sockets,
            ctx: zmq::Context::new(),
            key_pair: zmq::CurveKeyPair::new().expect("FIXME"),
            resend: RefCell::new(HashMap::new()),
            time_created: time::now(),
            timeouts: RefCell::new(HashMap::new()),
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

    fn get_timeout(&self) -> ((String, String), i64) {
        if let Some((&(ref req_id, ref node_alias), timeout)) = self.timeouts.borrow().iter()
            .map(|(key, value)| (key, (Duration::seconds(10) - (time::now() - *value)).num_milliseconds()))
            .min_by(|&(_, ref val1), &(_, ref val2)| val1.cmp(&val2)){
            ((req_id.to_string(), node_alias.to_string()), timeout)
        } else {
            (("".to_string(), "".to_string()), POOL_CON_ACTIVE_TO * 1000)
        }
    }

    fn is_active(&self) -> bool {
        trace!("time worked: {:?}", time::now() - self.time_created);
        time::now() - self.time_created < Duration::seconds(POOL_CON_ACTIVE_TO)
    }

    fn send_request(&mut self, pe: Option<NetworkerEvent>) {
        trace!("send_request >> pe: {:?}", pe);
        match pe {
            Some(NetworkerEvent::SendOneRequest(msg, req_id)) => {
                self._send_msg_to_one_node(0, req_id.clone(), msg.clone());
                self.resend.borrow_mut().insert(req_id, (0, msg));
            }
            Some(NetworkerEvent::SendAllRequest(msg, req_id)) => {
                (0..self.nodes.len()).for_each(|idx|
                    self._send_msg_to_one_node(idx, req_id.clone(), msg.clone()))
            }
            Some(NetworkerEvent::Resend(req_id)) => {
                let resend = if let Some(&mut (ref mut cnt, ref req)) = self.resend.borrow_mut().get_mut(&req_id) {
                    *cnt = *cnt + 1;
                    //TODO: FIXME: We can collect consensus just walking through if we are not collecting node aliases on the upper layer.
                    Some((*cnt % self.nodes.len(), req.clone()))
                } else {
                    None
                };
                if let Some((idx, req)) = resend {
                    self._send_msg_to_one_node(idx, req_id, req);
                }
            }
            _ => ()
        }
        trace!("send_request <<");
    }

    fn extend_timeout(&self, req_id: &str, node_alias: &str) {
        if let Some(timeout) = self.timeouts.borrow_mut().get_mut(&(req_id.to_string(), node_alias.to_string())) {
            *timeout = time::now();
        }
    }

    fn clean_timeout(&self, req_id: &str, node_alias: Option<String>) {
        match node_alias {
            Some(node_alias) => {
                self.timeouts.borrow_mut().remove(&(req_id.to_string(), node_alias));
            }
            None => {
                let keys_to_remove: Vec<(String, String)> = self.timeouts.borrow().keys()
                    .cloned().filter(|&(ref req_id_timeout, _)| req_id == req_id_timeout).collect();
                keys_to_remove.iter().for_each(|key| {self.timeouts.borrow_mut().remove(key);});
            }
        }
    }

    fn has_active_requests(&self) -> bool {
        !self.timeouts.borrow().is_empty()
    }

    fn is_orphaned(&self) -> bool {
        !self.is_active() && !self.has_active_requests()
    }

    fn _send_msg_to_one_node(&mut self, idx: usize, req_id: String, req: String) {
        trace!("_send_msg_to_one_node >> idx {}, req_id {}, req {}", idx, req_id, req);
        {
            let s = self._get_socket(idx);
            s.send_str(&req, zmq::DONTWAIT).expect("FIXME");
        }
        self.timeouts.borrow_mut().insert((req_id, self.nodes[idx].name.clone()), time::now());
        trace!("_send_msg_to_one_node <<");
    }

    fn _get_socket(&mut self, idx: usize) -> &ZSocket {
        if self.sockets[idx].is_none() {
            debug!("_get_socket: open new socket for node {}", idx);
            let s: ZSocket = self.nodes[idx].connect(&self.ctx, &self.key_pair).expect("FIXME");
            self.sockets[idx] = Some(s)
        }
        self.sockets[idx].as_ref().unwrap()
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
        MockNetworker {}
    }

    fn fetch_events(&self, poll_items: &[zmq::PollItem]) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        None
    }

    fn get_timeout(&self) -> ((String, String), i64) {
        unimplemented!()
    }

    fn get_poll_items(&self) -> Vec<PollItem> {
        unimplemented!()
    }
}


#[cfg(test)]
mod networker_tests {
    use super::*;

    #[test]
    pub fn networker_new_works() {
        ZMQNetworker::new();
    }



    #[test]
    pub fn networker_process_event_works() {
        let mut networker = ZMQNetworker::new();
        networker.process_event(None);
    }
}