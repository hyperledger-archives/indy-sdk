extern crate zmq;
extern crate time;

use errors::common::CommonError;
use errors::pool::PoolError;
use self::zmq::PollItem;
use self::zmq::Socket as ZSocket;
use services::pool::events::*;
use services::pool::types::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use super::time::Duration;
use time::Tm;
use utils::sequence::SequenceUtils;

pub trait Networker {
    fn new() -> Self;
    fn fetch_events(&self, poll_items: &[PollItem]) -> Vec<PoolEvent>;
    fn process_event(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent>;
    fn get_timeout(&self) -> ((String, String), i64);
    fn get_poll_items(&self) -> Vec<PollItem>;
}

pub struct ZMQNetworker {
    req_id_mappings: HashMap<String, i32>,
    pool_connections: BTreeMap<i32, PoolConnection>,
    nodes: Vec<RemoteNode>,
}

const POOL_CON_ACTIVE_TO: i64 = 5;
const POOL_ACK_TIMEOUT: i64 = 10;
const POOL_REPLY_TIMEOUT: i64 = 50;
const MAX_REQ_PER_POOL_CON: usize = 5;

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {
            req_id_mappings: HashMap::new(),
            pool_connections: BTreeMap::new(),
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
                let num = self.req_id_mappings.get(&req_id).map(|i| i.clone()).or_else(|| {
                    self.pool_connections.iter().next_back().and_then(|(pc_idx, pc)| {
                        if pc.is_active() && pc.req_cnt < MAX_REQ_PER_POOL_CON && pc.nodes.eq(&self.nodes) {
                            Some(*pc_idx)
                        } else {
                            None
                        }
                    })
                });
                match num {
                    Some(idx) => {
                        trace!("send request in existing conn");

                        match self.pool_connections.get_mut(&idx) {
                            Some(pc) => pc.send_request(pe).expect("FIXME"),
                            None => error!("Pool Connection not found")
                        }
                        self.req_id_mappings.insert(req_id.clone(), idx);
                    }
                    None => {
                        trace!("send request in new conn");
                        let pc_id = SequenceUtils::get_next_id();
                        let mut pc = PoolConnection::new(self.nodes.clone());
                        pc.send_request(pe).expect("FIXME");
                        self.pool_connections.insert(pc_id, pc);
                        self.req_id_mappings.insert(req_id.clone(), pc_id);
                    }
                }
                None
            }
            Some(NetworkerEvent::NodesStateUpdated(nodes)) => {
                trace!("ZMQNetworker::process_event: nodes_updated {:?}", nodes);
                self.nodes = nodes;
                None
            }
            Some(NetworkerEvent::ExtendTimeout(req_id, node_alias)) => {
                self.req_id_mappings.get(&req_id).map(
                    |idx| {
                        self.pool_connections.get(idx).map(
                            |pc| { pc.extend_timeout(&req_id, &node_alias); }
                        );
                    }
                );
                None
            }
            Some(NetworkerEvent::CleanTimeout(req_id, node_alias)) => {
                {
                    let idx_pc_to_delete = self.req_id_mappings.get(&req_id).and_then(
                        |idx| {
                            let delete = self.pool_connections.get(idx).map(
                                |pc| {
                                    pc.clean_timeout(&req_id, node_alias.clone());
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
                        trace!("removing pool connection {}", idx);
                        self.pool_connections.remove(idx);
                    }
                }

                if node_alias.is_none() {
                    self.req_id_mappings.remove(&req_id);
                }

                None
            }
            Some(NetworkerEvent::Timeout) => {
                let pc_to_delete: Vec<i32> = self.pool_connections.iter()
                    .filter(|(_, v)| v.is_orphaned())
                    .map(|(k, _)| *k)
                    .collect();
                pc_to_delete.iter().for_each(|idx| {
                    trace!("removing pool connection {}", idx);
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
    req_cnt: usize,
}

impl PoolConnection {
    fn new(nodes: Vec<RemoteNode>) -> Self {
        trace!("PoolConnection::new: from nodes {:?}", nodes);

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
            req_cnt: 0,
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
            .map(|(key, value)| (key, (*value - time::now()).num_milliseconds()))
            .min_by(|&(_, ref val1), &(_, ref val2)| val1.cmp(&val2)) {
            ((req_id.to_string(), node_alias.to_string()), timeout)
        } else {
            let time_from_start: Duration = time::now() - self.time_created;
            (("".to_string(), "".to_string()), POOL_CON_ACTIVE_TO * 1000 - time_from_start.num_milliseconds())
        }
    }

    fn is_active(&self) -> bool {
        trace!("time worked: {:?}", time::now() - self.time_created);
        time::now() - self.time_created < Duration::seconds(POOL_CON_ACTIVE_TO)
    }

    fn send_request(&mut self, pe: Option<NetworkerEvent>) -> Result<(), PoolError> {
        trace!("send_request >> pe: {:?}", pe);
        match pe {
            Some(NetworkerEvent::SendOneRequest(msg, req_id)) => {
                self.req_cnt += 1;
                self._send_msg_to_one_node(0, req_id.clone(), msg.clone())?;
                self.resend.borrow_mut().insert(req_id, (0, msg));
            }
            Some(NetworkerEvent::SendAllRequest(msg, req_id)) => {
                self.req_cnt += 1;
                for idx in 0..self.nodes.len() {
                    self._send_msg_to_one_node(idx, req_id.clone(), msg.clone())?;
                }
            }
            Some(NetworkerEvent::Resend(req_id)) => {
                let resend = if let Some(&mut (ref mut cnt, ref req)) = self.resend.borrow_mut().get_mut(&req_id) {
                    *cnt = *cnt + 1;
                    //TODO: FIXME: We can collect consensus just walking through if we are not collecting node aliases on the upper layer.
                    Some((*cnt % self.nodes.len(), req.clone()))
                } else {
                    error!("Unknown req_id for resending {}", req_id); //FIXME handle at RH level
                    None
                };
                if let Some((idx, req)) = resend {
                    self._send_msg_to_one_node(idx, req_id, req)?;
                }
            }
            _ => ()
        }
        trace!("send_request <<");
        Ok(())
    }

    fn extend_timeout(&self, req_id: &str, node_alias: &str) {
        if let Some(timeout) = self.timeouts.borrow_mut().get_mut(&(req_id.to_string(), node_alias.to_string())) {
            *timeout = time::now() + Duration::seconds(POOL_REPLY_TIMEOUT);
        } else {
            debug!("late REQACK for req_id {}, node {}", req_id, node_alias);
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
                keys_to_remove.iter().for_each(|key| { self.timeouts.borrow_mut().remove(key); });
            }
        }
    }

    fn has_active_requests(&self) -> bool {
        !self.timeouts.borrow().is_empty()
    }

    fn is_orphaned(&self) -> bool {
        !self.is_active() && !self.has_active_requests()
    }

    fn _send_msg_to_one_node(&mut self, idx: usize, req_id: String, req: String) -> Result<(), PoolError> {
        trace!("_send_msg_to_one_node >> idx {}, req_id {}, req {}", idx, req_id, req);
        {
            let s = self._get_socket(idx)?;
            s.send_str(&req, zmq::DONTWAIT)?;
        }
        self.timeouts.borrow_mut().insert((req_id, self.nodes[idx].name.clone()), time::now() + Duration::seconds(POOL_ACK_TIMEOUT));
        trace!("_send_msg_to_one_node <<");
        Ok(())
    }

    fn _get_socket(&mut self, idx: usize) -> Result<&ZSocket, PoolError> {
        if self.sockets[idx].is_none() {
            debug!("_get_socket: open new socket for node {}", idx);
            let s: ZSocket = self.nodes[idx].connect(&self.ctx, &self.key_pair)?;
            self.sockets[idx] = Some(s)
        }
        Ok(self.sockets[idx].as_ref().unwrap())
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

    fn fetch_events(&self, _poll_items: &[zmq::PollItem]) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn process_event(&mut self, _pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
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
pub mod networker_tests {
    use services::pool::rust_base58::FromBase58;
    use services::pool::tests::nodes_emulator;
    use std;
    use std::thread;
    use super::*;
    use utils::crypto::box_::CryptoBox;

    const REQ_ID: &'static str = "1";
    const MESSAGE: &'static str = "msg";
    const NODE_NAME: &'static str = "n1";

    pub fn _remote_node(txn: &NodeTransactionV1) -> RemoteNode {
        RemoteNode {
            public_key: CryptoBox::vk_to_curve25519(&txn.txn.data.dest.as_str().from_base58().unwrap()).unwrap(),
            zaddr: format!("tcp://{}:{}", txn.txn.data.data.client_ip.clone().unwrap(), txn.txn.data.data.client_port.clone().unwrap()),
            name: txn.txn.data.data.alias.clone(),
            is_blacklisted: false,
        }
    }

    #[cfg(test)]
    mod networker {
        use super::*;
        use std::ops::Sub;

        #[test]
        pub fn networker_new_works() {
            ZMQNetworker::new();
        }

        #[test]
        pub fn networker_process_event_works() {
            let mut networker = ZMQNetworker::new();
            networker.process_event(None);
        }

        #[test]
        fn networker_process_update_node_state_event_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();

            assert_eq!(0, networker.nodes.len());

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            assert_eq!(1, networker.nodes.len());
        }

        #[test]
        fn networker_process_send_request_event_works() {
            let mut txn = nodes_emulator::node();
            let handle = nodes_emulator::start(&mut txn);
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();
            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            assert!(networker.pool_connections.is_empty());
            assert!(networker.req_id_mappings.is_empty());

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            assert_eq!(1, networker.pool_connections.len());
            assert_eq!(1, networker.req_id_mappings.len());
            assert!(networker.req_id_mappings.contains_key(REQ_ID));

            let messages = handle.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);
        }

        #[test]
        fn networker_process_send_all_request_event_works() {
            let mut txn_1 = nodes_emulator::node();
            let handle_1 = nodes_emulator::start(&mut txn_1);
            let rn_1 = _remote_node(&txn_1);

            let mut txn_2 = nodes_emulator::node_2();
            let handle_2 = nodes_emulator::start(&mut txn_2);
            let rn_2 = _remote_node(&txn_2);

            let mut networker = ZMQNetworker::new();

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn_1, rn_2])));
            networker.process_event(Some(NetworkerEvent::SendAllRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            let messages = handle_1.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);

            let messages = handle_2.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);
        }

        #[test]
        fn networker_process_send_six_request_event_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            for i in 0..5 {
                networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), i.to_string())));
                assert_eq!(1, networker.pool_connections.len());
            }

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), "6".to_string())));
            assert_eq!(2, networker.pool_connections.len());
        }

        #[test]
        fn networker_process_send_six_request_event_with_timeout_cleaning_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            for i in 0..5 {
                networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), i.to_string())));
            }
            assert_eq!(1, networker.pool_connections.len());

            for i in 0..5 {
                networker.process_event(Some(NetworkerEvent::CleanTimeout(i.to_string(), None)));
            }
            assert_eq!(1, networker.pool_connections.len());

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), "6".to_string())));
            assert_eq!(2, networker.pool_connections.len());
        }

        #[test]
        fn networker_process_extend_timeout_event_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));
            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            thread::sleep(std::time::Duration::from_secs(1));

            let (_, timeout) = networker.get_timeout();

            networker.process_event(Some(NetworkerEvent::ExtendTimeout(REQ_ID.to_string(), txn.txn.data.data.alias)));

            let (_, timeout_2) = networker.get_timeout();

            assert!(timeout_2 > timeout);
        }

        // Roll back connection creation time on 5 seconds ago instead of sleeping
        fn _roll_back_timeout(networker: &mut ZMQNetworker) {
            let conn_id: i32 = networker.pool_connections.keys().cloned().collect::<Vec<i32>>()[0];
            let conn: &mut PoolConnection = networker.pool_connections.get_mut(&conn_id).unwrap();
            conn.time_created = time::now().sub(Duration::seconds(5));
        }

        #[test]
        fn networker_process_timeout_event_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);
            let conn = PoolConnection::new(vec![rn.clone()]);

            let mut networker = ZMQNetworker::new();
            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            networker.pool_connections.insert(1, conn);

            _roll_back_timeout(&mut networker);

            networker.process_event(Some(NetworkerEvent::Timeout));

            assert!(networker.pool_connections.is_empty());
        }

        #[test]
        fn networker_process_clean_timeout_event_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();
            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));
            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            _roll_back_timeout(&mut networker);

            networker.process_event(Some(NetworkerEvent::CleanTimeout(REQ_ID.to_string(), Some(txn.txn.data.data.alias))));

            assert!(networker.pool_connections.is_empty());
        }

        #[test]
        fn networker_process_second_request_after_cleaning_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();
            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));
            networker.process_event(Some(NetworkerEvent::CleanTimeout(REQ_ID.to_string(), None)));

            assert_eq!(1, networker.pool_connections.len());

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), "2".to_string())));

            assert_eq!(1, networker.pool_connections.len());
        }

        #[test]
        fn networker_process_second_request_after_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();
            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            assert_eq!(1, networker.pool_connections.len());

            _roll_back_timeout(&mut networker);

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), "2".to_string())));

            assert_eq!(2, networker.pool_connections.len());
        }

        #[test]
        fn networker_get_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut networker = ZMQNetworker::new();

            networker.process_event(Some(NetworkerEvent::NodesStateUpdated(vec![rn])));

            let (_, timeout) = networker.get_timeout();

            assert_eq!(::std::i64::MAX, timeout);

            networker.process_event(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            let (_, timeout) = networker.get_timeout();

            assert_ne!(::std::i64::MAX, timeout);
        }
    }

    #[cfg(test)]
    mod remote_node {
        use super::*;

        #[test]
        fn remote_node_connect() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let _socket = rn.connect(&zmq::Context::new(), &zmq::CurveKeyPair::new().unwrap()).unwrap();
        }

        #[test]
        fn remote_node_connect_works_for_invalid_address() {
            let txn = nodes_emulator::node();
            let mut rn = _remote_node(&txn);
            rn.zaddr = "invalid_address".to_string();

            let res = rn.connect(&zmq::Context::new(), &zmq::CurveKeyPair::new().unwrap());
            assert_match!(Err(PoolError::CommonError(_)), res);
        }
    }

    #[cfg(test)]
    mod pool_connection {
        use std::ops::Sub;
        use super::*;

        #[test]
        fn pool_connection_new_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            PoolConnection::new(vec![rn]);
        }

        #[test]
        fn pool_connection_is_active_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            assert!(conn.is_active());

            conn.time_created = time::now().sub(Duration::seconds(POOL_CON_ACTIVE_TO));

            assert!(!conn.is_active());
        }

        #[test]
        fn pool_connection_has_active_requests_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            assert!(!conn.has_active_requests());

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            assert!(conn.has_active_requests());
        }

        #[test]
        fn pool_connection_get_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            let ((req_id, node_alias), timeout) = conn.get_timeout();
            assert_eq!(req_id, "".to_string());
            assert_eq!(node_alias, "".to_string());
            assert!(POOL_CON_ACTIVE_TO * 1000 - 10 <= timeout);
            assert!(POOL_CON_ACTIVE_TO * 1000 >= timeout);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            let (id, timeout) = conn.get_timeout();
            assert_eq!((REQ_ID.to_string(), NODE_NAME.to_string()), id);
            assert!(POOL_ACK_TIMEOUT * 1000 - 10 <= timeout);
            assert!(POOL_ACK_TIMEOUT * 1000 >= timeout);
        }

        #[test]
        fn pool_connection_extend_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            thread::sleep(std::time::Duration::from_secs(1));

            let ((msg, name), timeout) = conn.get_timeout();

            conn.extend_timeout(&msg, &name);

            let ((_, _), timeout_2) = conn.get_timeout();

            assert!(timeout_2 > timeout);
        }

        #[test]
        fn pool_connection_clean_timeout_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            assert!(conn.has_active_requests());

            conn.clean_timeout(REQ_ID, Some(NODE_NAME.to_string()));

            assert!(!conn.has_active_requests());
        }

        #[test]
        fn pool_connection_get_socket_works() {
            let txn = nodes_emulator::node();
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            let _socket = conn._get_socket(0).unwrap();
        }

        #[test]
        fn pool_connection_get_socket_works_for_invalid_node_address() {
            let txn = nodes_emulator::node();
            let mut rn = _remote_node(&txn);
            rn.zaddr = "invalid_address".to_string();

            let mut conn = PoolConnection::new(vec![rn]);

            let res = conn._get_socket(0);
            assert_match!(Err(PoolError::CommonError(_)), res);
        }

        #[test]
        fn pool_connection_send_request_one_node_works() {
            let mut txn = nodes_emulator::node();
            let handle = nodes_emulator::start(&mut txn);
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();
            conn.send_request(Some(NetworkerEvent::SendOneRequest("msg2".to_string(), "12".to_string()))).unwrap();

            let messages = handle.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string(), "msg2".to_string()], messages);
        }

        #[test]
        fn pool_connection_send_request_one_node_works_for_two_active_nodes() {
            let mut txn_1 = nodes_emulator::node();
            let handle_1 = nodes_emulator::start(&mut txn_1);
            let rn_1 = _remote_node(&txn_1);

            let mut txn_2 = nodes_emulator::node_2();
            let handle_2 = nodes_emulator::start(&mut txn_2);
            let rn_2 = _remote_node(&txn_2);

            let mut conn = PoolConnection::new(vec![rn_1, rn_2]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            let messages = handle_1.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);

            let messages = handle_2.join().unwrap();
            assert!(messages.is_empty());
        }

        #[test]
        fn pool_connection_send_request_all_nodes_works() {
            let mut txn_1 = nodes_emulator::node();
            let handle_1 = nodes_emulator::start(&mut txn_1);
            let rn_1 = _remote_node(&txn_1);

            let mut txn_2 = nodes_emulator::node_2();
            let handle_2 = nodes_emulator::start(&mut txn_2);
            let rn_2 = _remote_node(&txn_2);

            let mut conn = PoolConnection::new(vec![rn_1, rn_2]);

            conn.send_request(Some(NetworkerEvent::SendAllRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            let messages = handle_1.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);

            let messages = handle_2.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);
        }

        #[test]
        fn pool_connection_resend_works() {
            let mut txn = nodes_emulator::node();
            let handle = nodes_emulator::start(&mut txn);
            let rn = _remote_node(&txn);

            let mut conn = PoolConnection::new(vec![rn]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            conn.send_request(Some(NetworkerEvent::Resend(REQ_ID.to_string()))).unwrap();

            let messages = handle.join().unwrap();

            assert_eq!(vec![MESSAGE.to_string(), MESSAGE.to_string()], messages);
        }

        #[test]
        fn pool_connection_resend_works_for_two_nodes() {
            let mut txn_1 = nodes_emulator::node();
            let handle_1 = nodes_emulator::start(&mut txn_1);
            let rn_1 = _remote_node(&txn_1);

            let mut txn_2 = nodes_emulator::node_2();
            let handle_2 = nodes_emulator::start(&mut txn_2);
            let rn_2 = _remote_node(&txn_2);

            let mut conn = PoolConnection::new(vec![rn_1, rn_2]);

            conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string()))).unwrap();

            conn.send_request(Some(NetworkerEvent::Resend(REQ_ID.to_string()))).unwrap();

            let messages = handle_1.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);

            let messages = handle_2.join().unwrap();
            assert_eq!(vec![MESSAGE.to_string()], messages);
        }

        #[test]
        fn pool_connection_send_works_for_invalid_node() {
            let txn = nodes_emulator::node();
            let mut rn = _remote_node(&txn);
            rn.zaddr = "invalid_address".to_string();

            let mut conn = PoolConnection::new(vec![rn]);

            let res = conn.send_request(Some(NetworkerEvent::SendOneRequest(MESSAGE.to_string(), REQ_ID.to_string())));

            assert_match!(Err(PoolError::CommonError(_)), res);
        }
    }
}