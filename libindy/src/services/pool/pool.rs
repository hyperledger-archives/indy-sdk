use super::zmq;
use std::thread::JoinHandle;
use std::thread;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;
use services::pool::types::LedgerStatus;

use services::pool::commander::Commander;
use services::pool::events::*;
use services::pool::networker::Networker;
use services::pool::request_handler::RequestHandler;
use services::pool::merkle_tree_factory;
use services::pool::rust_base58::{ToBase58, FromBase58};
use super::indy_crypto::bls::VerKey;
use services::pool::types::RemoteNode;
use errors::pool::PoolError;
use errors::common::CommonError;
use commands::Command;
use commands::CommandExecutor;
use commands::pool::PoolCommand;
use utils::crypto::box_::CryptoBox;
use services::ledger::merkletree::merkletree::MerkleTree;
use domain::ledger::request::ProtocolVersion;


trait PoolState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct InitializationState<T: Networker> {
    networker: Rc<RefCell<T>>
}

impl<T: Networker> PoolState for InitializationState<T> {}

struct GettingCatchupTargetState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handler: R,
    cmd_id: i32,
    refresh: bool,
}

impl<T: Networker, R: RequestHandler<T>> PoolState for GettingCatchupTargetState<T, R> {}

struct ActiveState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handlers: HashMap<String, R>,
    nodes: HashMap<String, Option<VerKey>>,
}

impl<T: Networker, R: RequestHandler<T>> PoolState for ActiveState<T, R> {}

struct SyncCatchupState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handler: R,
    cmd_id: i32,
    refresh: bool,
}

impl<T: Networker, R: RequestHandler<T>> PoolState for SyncCatchupState<T, R> {}

struct TerminatedState<T: Networker> {
    networker: Rc<RefCell<T>>
}

impl<T: Networker> PoolState for TerminatedState<T> {}

struct ClosedState {}

impl PoolState for ClosedState {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct PoolSM<T: PoolState> {
    pool_name: String,
    id: i32,
    state: T,
}

impl<T: Networker> PoolSM<InitializationState<T>> {
    pub fn new(networker: Rc<RefCell<T>>, pname: &str, id: i32) -> PoolSM<InitializationState<T>> {
        PoolSM {
            pool_name: pname.to_string(),
            id,
            state: InitializationState {
                networker
            }
        }
    }
}

// transitions from Initialization

impl<T: Networker, R: RequestHandler<T>> From<(R, i32, PoolSM<InitializationState<T>>)> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from((request_handler, cmd_id, pool): (R, i32, PoolSM<InitializationState<T>>)) -> PoolSM<GettingCatchupTargetState<T, R>> {
        trace!("PoolSM: from init to getting catchup target");
        //TODO: fill it up!
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: GettingCatchupTargetState {
                networker: pool.state.networker,
                request_handler,
                cmd_id,
                refresh: false,
            }
        }
    }
}

impl<T: Networker> From<PoolSM<InitializationState<T>>> for PoolSM<ClosedState> {
    fn from(pool: PoolSM<InitializationState<T>>) -> PoolSM<ClosedState> {
        trace!("PoolSM: from init to closed");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ClosedState {}
        }
    }
}

impl<T: Networker> From<PoolSM<InitializationState<T>>> for PoolSM<TerminatedState<T>> {
    fn from(pool: PoolSM<InitializationState<T>>) -> PoolSM<TerminatedState<T>> {
        trace!("PoolSM: from init to terminated");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<(PoolSM<InitializationState<T>>, HashMap<String, Option<VerKey>>)> for PoolSM<ActiveState<T, R>> {
    fn from((val, nodes): (PoolSM<InitializationState<T>>, HashMap<String, Option<VerKey>>)) -> PoolSM<ActiveState<T, R>> {
        trace!("PoolSM: from init to active");
        PoolSM {
            pool_name: val.pool_name,
            id: val.id,
            state: ActiveState {
                networker: val.state.networker,
                request_handlers: HashMap::new(),
                nodes,
            }
        }
    }
}

// transitions from GettingCatchupTarget

impl<T: Networker, R: RequestHandler<T>> From<(R, PoolSM<GettingCatchupTargetState<T, R>>)> for PoolSM<SyncCatchupState<T, R>> {
    fn from((request_handler, sm): (R, PoolSM<GettingCatchupTargetState<T, R>>)) -> Self {
        trace!("PoolSM: from getting catchup target to sync catchup");
        PoolSM {
            pool_name: sm.pool_name,
            id: sm.id,
            state: SyncCatchupState {
                networker: sm.state.networker,
                request_handler,
                cmd_id: sm.state.cmd_id,
                refresh: sm.state.refresh,
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<(PoolSM<GettingCatchupTargetState<T, R>>, HashMap<String, Option<VerKey>>)> for PoolSM<ActiveState<T, R>> {
    fn from((sm, nodes): (PoolSM<GettingCatchupTargetState<T, R>>, HashMap<String, Option<VerKey>>)) -> Self {
        trace!("PoolSM: from getting catchup target to active");
        let pc = if sm.state.refresh {
            PoolCommand::RefreshAck(sm.state.cmd_id.clone(), Ok(()))
        } else {
            PoolCommand::OpenAck(sm.state.cmd_id.clone(), sm.id.clone(), Ok(()))
        };
        CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
        PoolSM {
            pool_name: sm.pool_name,
            id: sm.id,
            state: ActiveState {
                networker: sm.state.networker,
                request_handlers: HashMap::new(),
                nodes,
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<GettingCatchupTargetState<T, R>>> for PoolSM<TerminatedState<T>> {
    fn from(sm: PoolSM<GettingCatchupTargetState<T, R>>) -> Self {
        trace!("PoolSM: from getting catchup target to terminated");
        PoolSM {
            pool_name: sm.pool_name,
            id: sm.id,
            state: TerminatedState {
                networker: sm.state.networker
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<GettingCatchupTargetState<T, R>>> for PoolSM<ClosedState> {
    fn from(mut pool: PoolSM<GettingCatchupTargetState<T, R>>) -> Self {
        trace!("PoolSM: from getting catchup target to closed");
        pool.state.request_handler.process_event(Some(RequestEvent::Terminate));
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ClosedState {}
        }
    }
}

// transitions from Active

impl<T: Networker, R: RequestHandler<T>> From<(PoolSM<ActiveState<T, R>>, R, i32)> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from((pool, request_handler, cmd_id): (PoolSM<ActiveState<T, R>>, R, i32)) -> Self {
        trace!("PoolSM: from active to getting catchup target");
        //TODO: close connections!
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: GettingCatchupTargetState {
                networker: pool.state.networker,
                cmd_id,
                request_handler,
                refresh: true
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<ActiveState<T, R>>> for PoolSM<TerminatedState<T>> {
    fn from(pool: PoolSM<ActiveState<T, R>>) -> Self {
        trace!("PoolSM: from active to terminated");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<ActiveState<T, R>>> for PoolSM<ClosedState> {
    fn from(mut pool: PoolSM<ActiveState<T, R>>) -> Self {
        pool.state.request_handlers.iter_mut().for_each(|(_, ref mut p)| {
            trace!("Termintating ongoing request");
            p.process_event(Some(RequestEvent::Terminate));
        });
        trace!("PoolSM: from active to closed");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ClosedState {}
        }
    }
}

// transitions from SyncCatchup

impl<T: Networker, R: RequestHandler<T>> From<(PoolSM<SyncCatchupState<T, R>>, HashMap<String, Option<VerKey>>)> for PoolSM<ActiveState<T, R>> {
    fn from((pool, nodes): (PoolSM<SyncCatchupState<T, R>>, HashMap<String, Option<VerKey>>)) -> Self {
        trace!("PoolSM: from sync catchup to active");
        let pc = if pool.state.refresh {
            PoolCommand::RefreshAck(pool.state.cmd_id.clone(), Ok(()))
        } else {
            PoolCommand::OpenAck(pool.state.cmd_id.clone(), pool.id.clone(), Ok(()))
        };
        CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ActiveState {
                networker: pool.state.networker,
                request_handlers: HashMap::new(),
                nodes,
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<SyncCatchupState<T, R>>> for PoolSM<TerminatedState<T>> {
    fn from(pool: PoolSM<SyncCatchupState<T, R>>) -> Self {
        trace!("PoolSM: from sync catchup to terminated");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<SyncCatchupState<T, R>>> for PoolSM<ClosedState> {
    fn from(mut pool: PoolSM<SyncCatchupState<T, R>>) -> Self {
        trace!("PoolSM: from sync catchup to closed");
        pool.state.request_handler.process_event(Some(RequestEvent::Terminate));
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ClosedState {}
        }
    }
}

// transitions from Terminated

impl<T: Networker, R: RequestHandler<T>> From<(PoolSM<TerminatedState<T>>, R, i32)> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from((pool, request_handler, cmd_id): (PoolSM<TerminatedState<T>>, R, i32)) -> Self {
        trace!("PoolSM: from terminated to getting catchup target");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: GettingCatchupTargetState {
                networker: pool.state.networker,
                cmd_id,
                request_handler,
                refresh: true
            }
        }
    }
}

impl<T: Networker> From<PoolSM<TerminatedState<T>>> for PoolSM<ClosedState> {
    fn from(pool: PoolSM<TerminatedState<T>>) -> Self {
        trace!("PoolSM: from terminated to closed");
        PoolSM {
            pool_name: pool.pool_name,
            id: pool.id,
            state: ClosedState {}
        }
    }
}

enum PoolWrapper<T: Networker, R: RequestHandler<T>> {
    Initialization(PoolSM<InitializationState<T>>),
    GettingCatchupTarget(PoolSM<GettingCatchupTargetState<T, R>>),
    Active(PoolSM<ActiveState<T, R>>),
    Closed(PoolSM<ClosedState>),
    SyncCatchup(PoolSM<SyncCatchupState<T, R>>),
    Terminated(PoolSM<TerminatedState<T>>),
}

impl<T: Networker, R: RequestHandler<T>> PoolWrapper<T, R> {
    pub fn handle_event(self, pe: PoolEvent) -> Self {
        match self {
            PoolWrapper::Initialization(pool) => match pe {
                PoolEvent::CheckCache(cmd_id) => {
                    //TODO: check cache freshness
                    let fresh = false;
                    if fresh {
//                        PoolWrapper::Active(pool.into())
                        unimplemented!()
                    } else {
                        match _get_request_handler_with_ledger_status_sent(pool.state.networker.clone(), &pool.pool_name) {
                            Ok(request_handler) => PoolWrapper::GettingCatchupTarget((request_handler, cmd_id, pool).into()),
                            Err(err) => {
                                CommandExecutor::instance().send(
                                    Command::Pool(
                                        PoolCommand::OpenAck(cmd_id, pool.id.clone(), Err(err)))
                                ).unwrap();
                                PoolWrapper::Terminated(pool.into())
                            }
                        }
                    }
                }
                PoolEvent::Close(cmd_id) => {
                    _close_pool_ack(cmd_id);
                    PoolWrapper::Closed(pool.into())
                },
                _ => PoolWrapper::Initialization(pool)
            }
            PoolWrapper::GettingCatchupTarget(mut pool) => {
                let pe = pool.state.request_handler.process_event(pe.clone().into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolWrapper::Closed(pool.into())
                    },
                    PoolEvent::CatchupTargetNotFound(err) => {
                        let pc = PoolCommand::OpenAck(pool.state.cmd_id, pool.id,Err(err));
                        CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
                        PoolWrapper::Terminated(pool.into())
                    },
                    PoolEvent::CatchupTargetFound(target_mt_root, target_mt_size, merkle_tree) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle_tree) {
                            pool.state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            let mut request_handler = R::new(pool.state.networker.clone(), _get_f(nodes.len()), &vec![], &nodes, None, &pool.pool_name);
                            request_handler.process_event(Some(RequestEvent::CatchupReq(merkle_tree, target_mt_size, target_mt_root)));
                            PoolWrapper::SyncCatchup((request_handler, pool).into())
                        } else {
                            PoolWrapper::Terminated(pool.into())
                        }
                    },
                    PoolEvent::Synced(merkle) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle) {
                            pool.state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            PoolWrapper::Active((pool, nodes).into())
                        } else {
                            PoolWrapper::Terminated(pool.into())
                        }
                    },
                    _ => PoolWrapper::GettingCatchupTarget(pool)
                }
            }
            PoolWrapper::Terminated(pool) => {
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolWrapper::Closed(pool.into())
                    },
                    PoolEvent::Refresh(cmd_id) => {
                        if let Ok(request_handler) = _get_request_handler_with_ledger_status_sent(pool.state.networker.clone(), &pool.pool_name) {
                            PoolWrapper::GettingCatchupTarget((pool, request_handler, cmd_id).into())
                        } else {
                            PoolWrapper::Terminated(pool)
                        }
                    },
                    _ => PoolWrapper::Terminated(pool)
                }
            }
            PoolWrapper::Closed(pool) => PoolWrapper::Closed(pool),
            PoolWrapper::Active(mut pool) => {
                match pe.clone() {
                    PoolEvent::PoolOutdated => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolWrapper::Closed(pool.into())
                    },
                    PoolEvent::Refresh(cmd_id) => {
                        if let Ok(request_handler) = _get_request_handler_with_ledger_status_sent(pool.state.networker.clone(), &pool.pool_name) {
                            PoolWrapper::GettingCatchupTarget((pool, request_handler, cmd_id).into())
                        } else {
                            PoolWrapper::Terminated(pool.into())
                        }
                    },
                    PoolEvent::SendRequest(cmd_id, _) => {
                        trace!("received request to send");
                        let re: Option<RequestEvent> = pe.into();
                        let req_id = re.clone().map(|r| r.get_req_id()).expect("FIXME");
                        let mut request_handler = R::new(pool.state.networker.clone(), _get_f(pool.state.nodes.len()), &vec![cmd_id], &pool.state.nodes, None, &pool.pool_name);
                        request_handler.process_event(re);
                        pool.state.request_handlers.insert(req_id.to_string(), request_handler);
                        PoolWrapper::Active(pool)
                    }
                    PoolEvent::NodeReply(reply, node) => {
                        trace!("received reply from node {:?}: {:?}", node, reply);
                        let re: Option<RequestEvent> = pe.into();
                        let req_id = re.clone().map(|r| r.get_req_id()).expect("FIXME");
                        let remove = if let Some(rh) = pool.state.request_handlers.get_mut(&req_id) {
                            rh.process_event(re);
                            rh.is_terminal()
                        } else {
                            false
                        };
                        if remove {
                            pool.state.request_handlers.remove(&req_id);
                        }
                        PoolWrapper::Active(pool)
                    }
                    PoolEvent::Timeout(req_id, _) => {
                        if let Some(rh) = pool.state.request_handlers.get_mut(&req_id) {
                            rh.process_event(pe.into());
                        }
                        PoolWrapper::Active(pool)
                    }
                    _ => PoolWrapper::Active(pool)
                }
            }
            PoolWrapper::SyncCatchup(mut pool) => {
                let pe = pool.state.request_handler.process_event(pe.clone().into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolWrapper::Closed(pool.into())
                    },
                    PoolEvent::NodesBlacklisted => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::Synced(merkle) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle).map_err(map_err_err!()) {
                            pool.state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            PoolWrapper::Active((pool, nodes).into())
                        } else {
                            PoolWrapper::Terminated(pool.into())
                        }
                    },
                    _ => PoolWrapper::SyncCatchup(pool)
                }
            }
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            &PoolWrapper::Initialization(ref pool) => pool.state.is_terminal(),
            &PoolWrapper::Closed(ref pool) => pool.state.is_terminal(),
            &PoolWrapper::Terminated(ref pool) => pool.state.is_terminal(),
            &PoolWrapper::GettingCatchupTarget(ref pool) => pool.state.is_terminal(),
            &PoolWrapper::Active(ref pool) => pool.state.is_terminal(),
            &PoolWrapper::SyncCatchup(ref pool) => pool.state.is_terminal(),
        }
    }
}

pub struct Pool <S: Networker, R: RequestHandler<S>>{
    _pd: PhantomData<(S, R)>,
    worker: Option<JoinHandle<()>>,
    name: String,
    id: i32,
}

impl<S: Networker, R: RequestHandler<S>> Pool<S, R> {
    pub fn new(name: &str, id: i32) -> Self {
        Pool {
            _pd: PhantomData::<(S, R)>,
            worker: None,
            name: name.to_string(),
            id,
        }
    }

    pub fn work(&mut self, cmd_socket: zmq::Socket) {
        let name = self.name.as_str().to_string();
        let id = self.id.clone();
        self.worker = Some(thread::spawn(move || {
            let mut pool_thread: PoolThread<S, R> = PoolThread::new(cmd_socket, name, id);
            pool_thread.work();
        }));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

struct PoolThread<S: Networker, R: RequestHandler<S>> {
    pool_wrapper: Option<PoolWrapper<S, R>>,
    events: VecDeque<PoolEvent>,
    commander: Commander,
    networker: Rc<RefCell<S>>,
}

impl<S: Networker, R: RequestHandler<S>> PoolThread<S, R> {
    pub fn new(cmd_socket: zmq::Socket, name: String, id: i32) -> Self {
        let networker = Rc::new(RefCell::new(S::new()));
        PoolThread {
            pool_wrapper: Some(PoolWrapper::Initialization(PoolSM::new(networker.clone(), &name, id))),
            events: VecDeque::new(),
            commander: Commander::new(cmd_socket),
            networker,
        }
    }

    pub fn work(&mut self) {
        loop {
            self._poll();

            if self._loop() {
                break;
            }
        }
    }

    fn _loop(&mut self) -> bool {
        while !self.events.is_empty() {
            let pe = self.events.pop_front();
            trace!("received pool event: {:?}", pe);
            match pe {
                Some(pe) => {
                    self.pool_wrapper = self.pool_wrapper.take().map(|w| w.handle_event(pe));
                }
                _ => ()
            }
        }
        self.pool_wrapper.as_ref().map(|w| w.is_terminal()).unwrap_or(true)
    }

    fn _poll(&mut self) {
        let events = {
            let networker = self.networker.borrow();

            let mut poll_items = networker.get_poll_items();
//            trace!("prevents: {:?}", poll_items.iter().map(|pi| pi.revents));
            poll_items.push(self.commander.get_poll_item());

            let ((req_id, alias), timeout) = networker.get_timeout();
//            trace!("next timeout: {:?}", timeout);

            let poll_res = zmq::poll(&mut poll_items, ::std::cmp::max(timeout, 0))
                .map_err(map_err_err!())
                .map_err(|_| unimplemented!() /* FIXME */).unwrap();
//            trace!("poll_res: {:?}", poll_res);
            if poll_res == 0 {
                self.events.push_back(PoolEvent::Timeout(req_id, alias)); // TODO check duplicate ?
            }
//            trace!("poll_items: {:?}", poll_items.len());

            let mut events = networker.fetch_events(poll_items.as_slice());
//            trace!("events: {:?}", events);
            if poll_items[poll_items.len() - 1].is_readable() { //TODO move into fetch events?
                events.extend(self.commander.fetch_events());
            }

            events
        };

        self.events.extend(events);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::pool::networker::MockNetworker;

    #[test]
    pub fn pool_new_works() {
        let networker = MockNetworker::new();
//        Pool::new(&Commander::new(), networker, MockConsensusCollector::new(&networker));
    }
}

fn _get_f(cnt: usize) -> usize {
    if cnt < 4 {
        return 0;
    }
    (cnt - 1) / 3
}

fn _get_request_handler_with_ledger_status_sent<T: Networker, R: RequestHandler<T>>(networker: Rc<RefCell<T>>, pool_name: &str) -> Result<R, PoolError> {
    let mut merkle = merkle_tree_factory::create(pool_name)?;
    let (nodes, remotes) = match _get_nodes_and_remotes(&merkle) {
        Ok(n) => n,
        Err(err) => {
            match merkle_tree_factory::drop_cache(pool_name) {
                Ok(_) => {
                    merkle = merkle_tree_factory::create(pool_name)?;
                    _get_nodes_and_remotes(&merkle)?
                }
                Err(_) => {return Err(err);}
            }
        }
    };
    networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
    let mut request_handler = R::new(networker.clone(), _get_f(nodes.len()), &vec![], &nodes, None, pool_name);
    let protocol_version = ProtocolVersion::get();
    let ls = LedgerStatus {
        txnSeqNo: merkle.count(),
        merkleRoot: merkle.root_hash().as_slice().to_base58(),
        ledgerId: 0,
        ppSeqNo: None,
        viewNo: None,
        protocolVersion: if protocol_version > 1 { Some(protocol_version) } else { None }
    };
    request_handler.process_event(Some(RequestEvent::LedgerStatus(ls, None, Some(merkle))));
    Ok(request_handler)
}

fn _get_nodes_and_remotes(merkle: &MerkleTree) -> Result<(HashMap<String, Option<VerKey>>, Vec<RemoteNode>), PoolError> {
    let nodes = merkle_tree_factory::build_node_state(merkle)?;

    Ok(nodes.iter().map( |(_, txn)| {
        let node_alias = txn.txn.data.data.alias.clone();
        let node_verkey = txn.txn.data.dest.as_str().from_base58()
            .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field dest in genesis transaction: {:?}", err)) })?;

        if txn.txn.data.data.services.is_none() || !txn.txn.data.data.services.as_ref().unwrap().contains(&"VALIDATOR".to_string()) {
            return Err(PoolError::CommonError(CommonError::InvalidState("Node is not a Validator".to_string())));
        }

        let address = match (&txn.txn.data.data.client_ip, &txn.txn.data.data.client_port) {
            (&Some(ref client_ip), &Some(ref client_port)) => format!("tcp://{}:{}", client_ip, client_port),
            _ => return Err(PoolError::CommonError(CommonError::InvalidState("Client address not found".to_string())))
        };

        let remote = RemoteNode {
            name: node_alias.clone(),
            public_key: CryptoBox::vk_to_curve25519(&node_verkey)?,
            zaddr: address,
            is_blacklisted: false,
        };
        let verkey: Option<VerKey> = match txn.txn.data.data.blskey {
            Some(ref blskey) => {
                let key = blskey.as_str().from_base58()
                    .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field blskey in genesis transaction: {:?}", err)) })?;
                Some(VerKey::from_bytes(key.as_slice())
                    .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field blskey in genesis transaction: {:?}", err)) })?)
            }
            None => None
        };
        Ok(((node_alias, verkey), remote))
    }
    ).fold(
        (HashMap::new(), vec![]), |(mut map, mut vec), res| {
            match res {
                Err(e) => {
                    error!("Error during retrieving nodes: {:?}", e);
                },
                Ok(((alias, verkey), remote)) => {
                    map.insert(alias.clone(), verkey);
                    vec.push(remote);
                }
            }
            (map, vec)
        }
    ))
}

fn _close_pool_ack(cmd_id: i32) {
    let pc = PoolCommand::CloseAck(cmd_id, Ok(()));
    CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
}