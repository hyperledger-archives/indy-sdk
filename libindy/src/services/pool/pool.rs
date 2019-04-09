use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;

use failure::Context;

use commands::Command;
use commands::CommandExecutor;
use commands::ledger::LedgerCommand;
use commands::pool::PoolCommand;
use domain::ledger::request::ProtocolVersion;
use domain::pool::PoolOpenConfig;
use errors::prelude::*;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::pool::commander::Commander;
use services::pool::events::*;
use services::pool::merkle_tree_factory;
use services::pool::networker::{Networker, ZMQNetworker};
use services::pool::request_handler::{RequestHandler, RequestHandlerImpl};
use services::pool::rust_base58::{FromBase58, ToBase58};
use services::pool::types::{LedgerStatus, RemoteNode};
use utils::crypto::ed25519_sign;

use super::indy_crypto::bls::VerKey;
use super::zmq;

struct PoolSM<T: Networker, R: RequestHandler<T>> {
    pool_name: String,
    id: i32,
    timeout: i64,
    extended_timeout: i64,
    state: PoolState<T, R>,
}

/// Transitions of pool state
/// Initialization -> GettingCatchupTarget, Active, Terminated, Closed
/// GettingCatchupTarget -> SyncCatchup, Active, Terminated, Closed
/// Active -> GettingCatchupTarget, Terminated, Closed
/// SyncCatchup -> Active, Terminated, Closed
/// Terminated -> GettingCatchupTarget, Closed
/// Closed -> Closed
enum PoolState<T: Networker, R: RequestHandler<T>> {
    Initialization(InitializationState<T>),
    GettingCatchupTarget(GettingCatchupTargetState<T, R>),
    Active(ActiveState<T, R>),
    SyncCatchup(SyncCatchupState<T, R>),
    Terminated(TerminatedState<T>),
    Closed(ClosedState),
}

struct InitializationState<T: Networker> {
    networker: Rc<RefCell<T>>
}

struct GettingCatchupTargetState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handler: R,
    cmd_id: i32,
    refresh: bool,
}

struct ActiveState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handlers: HashMap<String, R>,
    nodes: HashMap<String, Option<VerKey>>,
}

struct SyncCatchupState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handler: R,
    cmd_id: i32,
    refresh: bool,
}

struct TerminatedState<T: Networker> {
    networker: Rc<RefCell<T>>
}

struct ClosedState {}

impl<T: Networker, R: RequestHandler<T>> PoolSM<T, R> {
    pub fn new(networker: Rc<RefCell<T>>, pname: &str, id: i32, timeout: i64, extended_timeout: i64) -> PoolSM<T, R> {
        PoolSM {
            pool_name: pname.to_string(),
            id,
            timeout,
            extended_timeout,
            state: PoolState::Initialization(InitializationState {
                networker
            }),
        }
    }

    pub fn step(pool_name: String, id: i32, timeout: i64, extended_timeout: i64, state: PoolState<T, R>) -> Self {
        PoolSM { pool_name, id, timeout, extended_timeout, state }
    }
}

// transitions from Initialization

impl<T: Networker, R: RequestHandler<T>> From<(R, i32, InitializationState<T>)> for GettingCatchupTargetState<T, R> {
    fn from((request_handler, cmd_id, state): (R, i32, InitializationState<T>)) -> GettingCatchupTargetState<T, R> {
        trace!("PoolSM: from init to getting catchup target");
        //TODO: fill it up!
        GettingCatchupTargetState {
            networker: state.networker,
            request_handler,
            cmd_id,
            refresh: false,
        }
    }
}

impl<T: Networker> From<InitializationState<T>> for ClosedState {
    fn from(_state: InitializationState<T>) -> ClosedState {
        trace!("PoolSM: from init to closed");
        ClosedState {}
    }
}

impl<T: Networker> From<InitializationState<T>> for TerminatedState<T> {
    fn from(state: InitializationState<T>) -> TerminatedState<T> {
        trace!("PoolSM: from init to terminated");
        TerminatedState { networker: state.networker }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<(InitializationState<T>, HashMap<String, Option<VerKey>>)> for ActiveState<T, R> {
    fn from((state, nodes): (InitializationState<T>, HashMap<String, Option<VerKey>>)) -> ActiveState<T, R> {
        trace!("PoolSM: from init to active");
        ActiveState {
            networker: state.networker,
            request_handlers: HashMap::new(),
            nodes,
        }
    }
}

// transitions from GettingCatchupTarget

impl<T: Networker, R: RequestHandler<T>> From<(R, GettingCatchupTargetState<T, R>)> for SyncCatchupState<T, R> {
    fn from((request_handler, state): (R, GettingCatchupTargetState<T, R>)) -> Self {
        trace!("PoolSM: from getting catchup target to sync catchup");
        SyncCatchupState {
            networker: state.networker,
            request_handler,
            cmd_id: state.cmd_id,
            refresh: state.refresh,
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<(GettingCatchupTargetState<T, R>, HashMap<String, Option<VerKey>>)> for ActiveState<T, R> {
    fn from((state, nodes): (GettingCatchupTargetState<T, R>, HashMap<String, Option<VerKey>>)) -> Self {
        ActiveState {
            networker: state.networker,
            request_handlers: HashMap::new(),
            nodes,
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<GettingCatchupTargetState<T, R>> for TerminatedState<T> {
    fn from(state: GettingCatchupTargetState<T, R>) -> Self {
        trace!("PoolSM: from getting catchup target to terminated");
        TerminatedState {
            networker: state.networker
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<GettingCatchupTargetState<T, R>> for ClosedState {
    fn from(mut state: GettingCatchupTargetState<T, R>) -> Self {
        trace!("PoolSM: from getting catchup target to closed");
        state.request_handler.process_event(Some(RequestEvent::Terminate));
        ClosedState {}
    }
}

// transitions from Active

impl<T: Networker, R: RequestHandler<T>> From<(ActiveState<T, R>, R, i32)> for GettingCatchupTargetState<T, R> {
    fn from((state, request_handler, cmd_id): (ActiveState<T, R>, R, i32)) -> Self {
        trace!("PoolSM: from active to getting catchup target");
        //TODO: close connections!
        GettingCatchupTargetState {
            networker: state.networker,
            cmd_id,
            request_handler,
            refresh: true,
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<ActiveState<T, R>> for TerminatedState<T> {
    fn from(state: ActiveState<T, R>) -> Self {
        trace!("PoolSM: from active to terminated");
        TerminatedState { networker: state.networker }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<ActiveState<T, R>> for ClosedState {
    fn from(mut state: ActiveState<T, R>) -> Self {
        state.request_handlers.iter_mut().for_each(|(_, ref mut p)| {
            trace!("Termintating ongoing request");
            p.process_event(Some(RequestEvent::Terminate));
        });
        trace!("PoolSM: from active to closed");
        ClosedState {}
    }
}

// transitions from SyncCatchup

impl<T: Networker, R: RequestHandler<T>> From<(SyncCatchupState<T, R>, HashMap<String, Option<VerKey>>)> for ActiveState<T, R> {
    fn from((state, nodes): (SyncCatchupState<T, R>, HashMap<String, Option<VerKey>>)) -> Self {
        trace!("PoolSM: from sync catchup to active");
        ActiveState {
            networker: state.networker,
            request_handlers: HashMap::new(),
            nodes,
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<SyncCatchupState<T, R>> for TerminatedState<T> {
    fn from(state: SyncCatchupState<T, R>) -> Self {
        trace!("PoolSM: from sync catchup to terminated");
        TerminatedState { networker: state.networker }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<SyncCatchupState<T, R>> for ClosedState {
    fn from(mut state: SyncCatchupState<T, R>) -> Self {
        trace!("PoolSM: from sync catchup to closed");
        state.request_handler.process_event(Some(RequestEvent::Terminate));
        ClosedState {}
    }
}

// transitions from Terminated

impl<T: Networker, R: RequestHandler<T>> From<(TerminatedState<T>, R, i32)> for GettingCatchupTargetState<T, R> {
    fn from((state, request_handler, cmd_id): (TerminatedState<T>, R, i32)) -> Self {
        trace!("PoolSM: from terminated to getting catchup target");
        GettingCatchupTargetState {
            networker: state.networker,
            cmd_id,
            request_handler,
            refresh: true,
        }
    }
}

impl<T: Networker> From<TerminatedState<T>> for ClosedState {
    fn from(_state: TerminatedState<T>) -> Self {
        trace!("PoolSM: from terminated to closed");
        ClosedState {}
    }
}

impl<T: Networker, R: RequestHandler<T>> PoolSM<T, R> {
    pub fn handle_event(self, pe: PoolEvent) -> Self {
        let PoolSM { pool_name, id, state, timeout, extended_timeout } = self;
        let state = match state {
            PoolState::Initialization(state) => match pe {
                PoolEvent::CheckCache(cmd_id) => {
                    //TODO: check cache freshness
                    let fresh = false;
                    if fresh {
                        //                        PoolWrapper::Active(pool.into())
                        unimplemented!()
                    } else {
                        match _get_request_handler_with_ledger_status_sent(state.networker.clone(), &pool_name, timeout, extended_timeout) {
                            Ok(request_handler) => PoolState::GettingCatchupTarget((request_handler, cmd_id, state).into()),
                            Err(err) => {
                                CommandExecutor::instance().send(
                                    Command::Pool(
                                        PoolCommand::OpenAck(cmd_id, id.clone(), Err(err)))
                                ).unwrap();
                                PoolState::Terminated(state.into())
                            }
                        }
                    }
                }
                PoolEvent::Close(cmd_id) => {
                    _close_pool_ack(cmd_id);
                    PoolState::Closed(state.into())
                }
                _ => PoolState::Initialization(state)
            }
            PoolState::GettingCatchupTarget(mut state) => {
                let pe = state.request_handler.process_event(pe.clone().into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolState::Closed(state.into())
                    }
                    PoolEvent::CatchupTargetNotFound(err) => {
                        let pc = PoolCommand::OpenAck(state.cmd_id, id, Err(err));
                        CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
                        PoolState::Terminated(state.into())
                    }
                    PoolEvent::CatchupRestart(merkle_tree) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle_tree) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            state.request_handler = R::new(state.networker.clone(), _get_f(nodes.len()), &vec![], &nodes, None, &pool_name, timeout, extended_timeout);
                            let ls = _ledger_status(&merkle_tree);
                            state.request_handler.process_event(Some(RequestEvent::LedgerStatus(ls, None, Some(merkle_tree))));
                            PoolState::GettingCatchupTarget(state)
                        } else {
                            PoolState::Terminated(state.into())
                        }
                    }
                    PoolEvent::CatchupTargetFound(target_mt_root, target_mt_size, merkle_tree) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle_tree) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            let mut request_handler = R::new(state.networker.clone(), _get_f(nodes.len()), &vec![], &nodes, None, &pool_name, timeout, extended_timeout);
                            request_handler.process_event(Some(RequestEvent::CatchupReq(merkle_tree, target_mt_size, target_mt_root)));
                            PoolState::SyncCatchup((request_handler, state).into())
                        } else {
                            PoolState::Terminated(state.into())
                        }
                    }
                    PoolEvent::Synced(merkle) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            _send_open_refresh_ack(state.cmd_id, id, state.refresh);
                            PoolState::Active((state, nodes).into())
                        } else {
                            PoolState::Terminated(state.into())
                        }
                    }
                    _ => PoolState::GettingCatchupTarget(state)
                }
            }
            PoolState::Terminated(state) => {
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolState::Closed(state.into())
                    }
                    PoolEvent::Refresh(cmd_id) => {
                        if let Ok(request_handler) = _get_request_handler_with_ledger_status_sent(state.networker.clone(), &pool_name, timeout, extended_timeout) {
                            PoolState::GettingCatchupTarget((state, request_handler, cmd_id).into())
                        } else {
                            PoolState::Terminated(state)
                        }
                    }
                    PoolEvent::Timeout(req_id, node_alias) => {
                        if "".eq(&req_id) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::Timeout));
                        } else {
                            warn!("Unexpected timeout: req_id {}, node_alias {}", req_id, node_alias)
                        }
                        PoolState::Terminated(state)
                    }
                    _ => PoolState::Terminated(state)
                }
            }
            PoolState::Closed(state) => PoolState::Closed(state),
            PoolState::Active(mut state) => {
                match pe.clone() {
                    PoolEvent::PoolOutdated => PoolState::Terminated(state.into()),
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolState::Closed(state.into())
                    }
                    PoolEvent::Refresh(cmd_id) => {
                        if let Ok(request_handler) = _get_request_handler_with_ledger_status_sent(state.networker.clone(), &pool_name, timeout, extended_timeout) {
                            PoolState::GettingCatchupTarget((state, request_handler, cmd_id).into())
                        } else {
                            PoolState::Terminated(state.into())
                        }
                    }
                    PoolEvent::SendRequest(cmd_id, _, _, _) => {
                        trace!("received request to send");
                        let re: Option<RequestEvent> = pe.into();
                        match re.as_ref().map(|r| r.get_req_id()) {
                            Some(req_id) => {
                                let mut request_handler = R::new(state.networker.clone(), _get_f(state.nodes.len()), &vec![cmd_id], &state.nodes, None, &pool_name, timeout, extended_timeout);
                                request_handler.process_event(re);
                                state.request_handlers.insert(req_id.to_string(), request_handler); //FIXME check already exists
                            }
                            None => {
                                let res = Err(err_msg(IndyErrorKind::InvalidStructure, "Request id not found"));
                                _send_submit_ack(cmd_id, res)
                            }
                        };
                        PoolState::Active(state)
                    }
                    PoolEvent::NodeReply(reply, node) => {
                        trace!("received reply from node {:?}: {:?}", node, reply);
                        let re: Option<RequestEvent> = pe.into();
                        match re.as_ref().map(|r| r.get_req_id()) {
                            Some(req_id) => {
                                let remove = if let Some(rh) = state.request_handlers.get_mut(&req_id) {
                                    rh.process_event(re);
                                    rh.is_terminal()
                                } else {
                                    false
                                };
                                if remove {
                                    state.request_handlers.remove(&req_id);
                                }
                            }
                            None => warn!("Request id not found in Reply: {:?}", reply)
                        };

                        PoolState::Active(state)
                    }
                    PoolEvent::Timeout(req_id, node_alias) => {
                        if let Some(rh) = state.request_handlers.get_mut(&req_id) {
                            rh.process_event(pe.into());
                        } else if "".eq(&req_id) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::Timeout));
                        } else {
                            warn!("Unexpected timeout: req_id {}, node_alias {}", req_id, node_alias)
                        }
                        PoolState::Active(state)
                    }
                    _ => PoolState::Active(state)
                }
            }
            PoolState::SyncCatchup(mut state) => {
                let pe = state.request_handler.process_event(pe.clone().into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close(cmd_id) => {
                        _close_pool_ack(cmd_id);
                        PoolState::Closed(state.into())
                    }
                    PoolEvent::NodesBlacklisted => PoolState::Terminated(state.into()),
                    PoolEvent::Synced(merkle) => {
                        if let Ok((nodes, remotes)) = _get_nodes_and_remotes(&merkle).map_err(map_err_err!()) {
                            state.networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
                            _send_open_refresh_ack(state.cmd_id, id, state.refresh);
                            PoolState::Active((state, nodes).into())
                        } else {
                            PoolState::Terminated(state.into())
                        }
                    }
                    _ => PoolState::SyncCatchup(state)
                }
            }
        };
        PoolSM::step(pool_name, id, timeout, extended_timeout, state)
    }

    pub fn is_terminal(&self) -> bool {
        match self.state {
            PoolState::Initialization(_) |
            PoolState::GettingCatchupTarget(_) |
            PoolState::Active(_) |
            PoolState::SyncCatchup(_) |
            PoolState::Terminated(_) => false,
            PoolState::Closed(_) => true,
        }
    }
}

pub struct Pool<S: Networker, R: RequestHandler<S>> {
    _pd: PhantomData<(S, R)>,
    worker: Option<JoinHandle<()>>,
    name: String,
    id: i32,
    timeout: i64,
    extended_timeout: i64,
    active_timeout: i64,
    conn_limit: usize,
    preordered_nodes: Vec<String>,
}

impl<S: Networker, R: RequestHandler<S>> Pool<S, R> {
    pub fn new(name: &str, id: i32, config: PoolOpenConfig) -> Self {
        trace!("Pool::new name {}, id {}, config {:?}", name, id, config);
        Pool {
            _pd: PhantomData::<(S, R)>,
            worker: None,
            name: name.to_string(),
            id,
            timeout: config.timeout,
            extended_timeout: config.extended_timeout,
            active_timeout: config.conn_active_timeout,
            conn_limit: config.conn_limit,
            preordered_nodes: config.preordered_nodes,
        }
    }

    pub fn work(&mut self, cmd_socket: zmq::Socket) {
        let name = self.name.as_str().to_string();
        let id = self.id.clone();
        let timeout = self.timeout;
        let extended_timeout = self.extended_timeout;
        let active_timeout = self.active_timeout;
        let conn_limit = self.conn_limit;
        let preordered_nodes = self.preordered_nodes.clone();
        self.worker = Some(thread::spawn(move || {
            let mut pool_thread: PoolThread<S, R> = PoolThread::new(cmd_socket, name, id,
                                                                    timeout, extended_timeout,
                                                                    active_timeout, conn_limit,
                                                                    preordered_nodes);
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
    pool_sm: Option<PoolSM<S, R>>,
    events: VecDeque<PoolEvent>,
    commander: Commander,
    networker: Rc<RefCell<S>>,
}

impl<S: Networker, R: RequestHandler<S>> PoolThread<S, R> {
    pub fn new(cmd_socket: zmq::Socket, name: String, id: i32, timeout: i64, extended_timeout: i64, active_timeout: i64, conn_limit: usize, preordered_nodes: Vec<String>) -> Self {
        let networker = Rc::new(RefCell::new(S::new(active_timeout, conn_limit, preordered_nodes)));
        PoolThread {
            pool_sm: Some(PoolSM::new(networker.clone(), &name, id, timeout, extended_timeout)),
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
                    self.pool_sm = self.pool_sm.take().map(|w| w.handle_event(pe));
                }
                _ => ()
            }
        }
        self.pool_sm.as_ref().map(|w| w.is_terminal()).unwrap_or(true)
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
            if poll_items[poll_items.len() - 1].is_readable() {
                //TODO move into fetch events?
                events.extend(self.commander.fetch_events());
            }

            events
        };

        self.events.extend(events);
    }
}

fn _get_f(cnt: usize) -> usize {
    if cnt < 4 {
        return 0;
    }
    (cnt - 1) / 3
}

fn _get_request_handler_with_ledger_status_sent<T: Networker, R: RequestHandler<T>>(networker: Rc<RefCell<T>>, pool_name: &str, timeout: i64, extended_timeout: i64) -> IndyResult<R> {
    let mut merkle = merkle_tree_factory::create(pool_name)?;

    let (nodes, remotes) = match _get_nodes_and_remotes(&merkle) {
        Ok(n) => n,
        Err(err) => {
            match merkle_tree_factory::drop_cache(pool_name) {
                Ok(_) => {
                    merkle = merkle_tree_factory::create(pool_name)?;
                    _get_nodes_and_remotes(&merkle)?
                }
                Err(_) => { return Err(err); }
            }
        }
    };
    networker.borrow_mut().process_event(Some(NetworkerEvent::NodesStateUpdated(remotes)));
    let mut request_handler = R::new(networker.clone(), _get_f(nodes.len()), &vec![], &nodes, None, pool_name, timeout, extended_timeout);
    let ls = _ledger_status(&merkle);
    request_handler.process_event(Some(RequestEvent::LedgerStatus(ls, None, Some(merkle))));
    Ok(request_handler)
}

fn _ledger_status(merkle: &MerkleTree) -> LedgerStatus{
    let protocol_version = ProtocolVersion::get();

    LedgerStatus {
        txnSeqNo: merkle.count(),
        merkleRoot: merkle.root_hash().as_slice().to_base58(),
        ledgerId: 0,
        ppSeqNo: None,
        viewNo: None,
        protocolVersion: if protocol_version > 1 { Some(protocol_version) } else { None },
    }
}

fn _get_nodes_and_remotes(merkle: &MerkleTree) -> IndyResult<(HashMap<String, Option<VerKey>>, Vec<RemoteNode>)> {
    let nodes = merkle_tree_factory::build_node_state(merkle)?;

    Ok(nodes.iter().map(|(_, txn)| {
        let node_alias = txn.txn.data.data.alias.clone();

        let node_verkey = txn.txn.data.dest
            .as_str()
            .from_base58()
            .map_err(|err| Context::new(err))
            .to_indy(IndyErrorKind::InvalidStructure, "Invalid field dest in genesis transaction")?;

        let node_verkey = ed25519_sign::PublicKey::from_slice(&node_verkey)
            .and_then(|vk| ed25519_sign::vk_to_curve25519(&vk))
            .to_indy(IndyErrorKind::InvalidStructure, "Invalid field dest in genesis transaction")?;

        if txn.txn.data.data.services.is_none() || !txn.txn.data.data.services.as_ref().unwrap().contains(&"VALIDATOR".to_string()) {
            return Err(err_msg(IndyErrorKind::InvalidState, "Node is not a validator")); // FIXME: review error kind
        }

        let address = match (&txn.txn.data.data.client_ip, &txn.txn.data.data.client_port) {
            (&Some(ref client_ip), &Some(ref client_port)) => format!("tcp://{}:{}", client_ip, client_port),
            _ => return Err(err_msg(IndyErrorKind::InvalidState, "Client address not found")),
        };

        let remote = RemoteNode {
            name: node_alias.clone(),
            public_key: node_verkey[..].to_vec(),
            // TODO:FIXME
            zaddr: address,
            is_blacklisted: false,
        };

        let verkey: Option<VerKey> = match txn.txn.data.data.blskey {
            Some(ref blskey) => {
                let key = blskey
                    .as_str()
                    .from_base58()
                    .map_err(|err| Context::new(err))
                    .to_indy(IndyErrorKind::InvalidStructure, "Invalid field blskey in genesis transaction")?;

                Some(VerKey::from_bytes(&key)
                    .to_indy(IndyErrorKind::InvalidStructure, "Invalid field blskey in genesis transaction")?)
            },
            None => None,
        };
        Ok(((node_alias, verkey), remote))
    }
    ).fold(
        (HashMap::new(), vec![]), |(mut map, mut vec), res| {
            match res {
                Err(e) => {
                    error!("Error during retrieving nodes: {:?}", e);
                }
                Ok(((alias, verkey), remote)) => {
                    map.insert(alias.clone(), verkey);
                    vec.push(remote);
                }
            }
            (map, vec)
        },
    ))
}

fn _close_pool_ack(cmd_id: i32) {
    let pc = PoolCommand::CloseAck(cmd_id, Ok(()));
    CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
}

fn _send_submit_ack(cmd_id: i32, res: IndyResult<String>) {
    let lc = LedgerCommand::SubmitAck(cmd_id.clone(), res);
    CommandExecutor::instance().send(Command::Ledger(lc)).unwrap();
}

fn _send_open_refresh_ack(cmd_id: i32, id: i32, is_refresh: bool) {
    trace!("PoolSM: from getting catchup target to active");
    let pc = if is_refresh {
        PoolCommand::RefreshAck(cmd_id.clone(), Ok(()))
    } else {
        PoolCommand::OpenAck(cmd_id.clone(), id, Ok(()))
    };
    CommandExecutor::instance().send(Command::Pool(pc)).unwrap();
}

pub struct ZMQPool {
    pub(super) pool: Pool<ZMQNetworker, RequestHandlerImpl<ZMQNetworker>>,
    pub(super) cmd_socket: zmq::Socket,
}

impl ZMQPool {
    pub fn new(pool: Pool<ZMQNetworker, RequestHandlerImpl<ZMQNetworker>>, cmd_socket: zmq::Socket) -> ZMQPool {
        ZMQPool {
            pool,
            cmd_socket,
        }
    }
}

impl Drop for ZMQPool {
    fn drop(&mut self) {
        info!("Drop started");

        if let Err(err) = self.cmd_socket.send("exit".as_bytes(), zmq::DONTWAIT) {
            warn!("Can't send exit command to pool worker thread (may be already finished) {}", err);
        }

        // Option worker type and this kludge is workaround for rust
        if let Some(worker) = self.pool.worker.take() {
            info!("Drop wait worker");
            worker.join().unwrap();
        }
        info!("Drop finished");
    }
}

#[cfg(test)]
mod tests {
    use services::pool::networker::MockNetworker;
    use services::pool::request_handler::tests::MockRequestHandler;
    use services::pool::types::{Message, Reply, ReplyResultV1, ReplyTxnV1, ReplyV1, ResponseMetadata};
    use utils::test;

    use super::*;

    mod pool {
        use super::*;

        #[test]
        pub fn pool_new_works() {
            let _p: Pool<MockNetworker, MockRequestHandler> = Pool::new("pool", 1, PoolOpenConfig::default());
        }

        #[test]
        pub fn pool_get_name_works() {
            let name = "name";
            let p: Pool<MockNetworker, MockRequestHandler> = Pool::new(name, 1, PoolOpenConfig::default());
            assert_eq!(name, p.get_name());
        }

        #[test]
        pub fn pool_get_id_works() {
            let name = "name";
            let id = 1;
            let p: Pool<MockNetworker, MockRequestHandler> = Pool::new(name, id, PoolOpenConfig::default());
            assert_eq!(id, p.get_id());
        }
    }

    mod pool_sm {
        use std::fs;
        use std::io::Write;

        use serde_json;

        use utils::environment;

        use super::*;

        const POOL: &'static str = "pool";

        #[test]
        pub fn pool_wrapper_new_initialization_works() {
            let _p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), "name", 1, 0, 0);
        }

        #[test]
        pub fn pool_wrapper_check_cache_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            assert_match!(PoolState::GettingCatchupTarget(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_check_cache_works_for_no_pool_created() {
            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            assert_match!(PoolState::Terminated(_), p.state);
        }

        #[test]
        pub fn pool_wrapper_terminated_close_works() {
            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Close(2));
            assert_match!(PoolState::Closed(_), p.state);
        }

        #[test]
        pub fn pool_wrapper_terminated_refresh_works() {
            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p = p.handle_event(PoolEvent::Refresh(2));
            assert_match!(PoolState::GettingCatchupTarget(_), p.state);
        }

        #[test]
        pub fn pool_wrapper_terminated_timeout_works() {
            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM {
                pool_name: POOL.to_string(),
                id: 1,
                state: PoolState::Terminated(TerminatedState {
                    networker: Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))),
                }),
                timeout: 0,
                extended_timeout: 0,
            };

            let p = p.handle_event(PoolEvent::Timeout("".to_string(), "".to_string()));
            assert_match!(PoolState::Terminated(_), p.state);
            match p.state {
                PoolState::Terminated(state) => {
                    assert_eq!(state.networker.borrow().events.len(), 1);
                    let event = state.networker.borrow_mut().events.remove(0);
                    assert_match!(Some(NetworkerEvent::Timeout), event);
                }
                _ => assert!(false)
            }
        }

        #[test]
        pub fn pool_wrapper_cloe_works_from_initialization() {
            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::Close(1));
            assert_match!(PoolState::Closed(_), p.state);
        }

        #[test]
        pub fn pool_wrapper_close_works_from_getting_catchup_target() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Close(2));
            assert_match!(PoolState::Closed(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_catchup_target_not_found_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::CatchupTargetNotFound(err_msg(IndyErrorKind::PoolTimeout, "Pool timeout")));
            assert_match!(PoolState::Terminated(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_getting_catchup_target_synced_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            assert_match!(PoolState::Active(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_getting_catchup_target_synced_works_for_node_state_error() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            ProtocolVersion::set(1);
            let p = p.handle_event(PoolEvent::Synced(merkle_tree_factory::create(POOL).unwrap()));
            assert_match!(PoolState::Terminated(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_getting_catchup_target_catchup_target_found_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let mt = merkle_tree_factory::create(POOL).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::CatchupTargetFound(mt.root_hash().to_vec(), mt.count, mt));
            assert_match!(PoolState::SyncCatchup(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_getting_catchup_target_catchup_target_found_works_for_node_state_error() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let mt = merkle_tree_factory::create(POOL).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            ProtocolVersion::set(1);
            let p = p.handle_event(PoolEvent::CatchupTargetFound(mt.root_hash().to_vec(), mt.count, mt));
            assert_match!(PoolState::Terminated(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_sync_catchup_close_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let mt = merkle_tree_factory::create(POOL).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::CatchupTargetFound(mt.root_hash().to_vec(), mt.count, mt));
            let p = p.handle_event(PoolEvent::Close(2));
            assert_match!(PoolState::Closed(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_sync_catchup_synced_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let mt = merkle_tree_factory::create(POOL).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::CatchupTargetFound(mt.root_hash().to_vec(), mt.count, mt));
            let p = p.handle_event(PoolEvent::Synced(merkle_tree_factory::create(POOL).unwrap()));
            assert_match!(PoolState::Active(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_sync_catchup_synced_works_for_node_state_error() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let mt = merkle_tree_factory::create(POOL).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::CatchupTargetFound(mt.root_hash().to_vec(), mt.count, mt));
            ProtocolVersion::set(1);
            let p = p.handle_event(PoolEvent::Synced(merkle_tree_factory::create(POOL).unwrap()));
            assert_match!(PoolState::Terminated(_), p.state);

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_active_send_request_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "reqId": 1,
                "operation": {
                    "type": "1"
                }
            }).to_string();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.request_handlers.len(), 1);
                    assert!(state.request_handlers.contains_key("1"));
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_active_send_request_works_for_no_req_id() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "operation": {
                    "type": "1"
                }
            }).to_string();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.request_handlers.len(), 0);
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_active_node_reply_works() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "reqId": 1,
                "operation": {
                    "type": "1"
                }
            }).to_string();

            let rep = Message::Reply(Reply::ReplyV1(
                ReplyV1 {
                    result: ReplyResultV1 {
                        txn: ReplyTxnV1 {
                            metadata: ResponseMetadata {
                                req_id: 1
                            }
                        }
                    }
                }
            ));

            let rep = serde_json::to_string(&rep).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            let p = p.handle_event(PoolEvent::NodeReply(rep, "node".to_string()));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.request_handlers.len(), 0);
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_sends_requests_to_two_nodes() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "reqId": 1,
                "operation": {
                    "type": "105"
                }
            }).to_string();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.networker.borrow().events.len(), 2);
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_active_node_reply_works_for_no_request() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "reqId": 1,
                "operation": {
                    "type": "1"
                }
            }).to_string();

            let rep = Message::Reply(Reply::ReplyV1(
                ReplyV1 {
                    result: ReplyResultV1 {
                        txn: ReplyTxnV1 {
                            metadata: ResponseMetadata {
                                req_id: 2
                            }
                        }
                    }
                }
            ));

            let rep = serde_json::to_string(&rep).unwrap();

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            let p = p.handle_event(PoolEvent::NodeReply(rep, "node".to_string()));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.request_handlers.len(), 1);
                    assert!(state.request_handlers.contains_key("1"));
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        #[test]
        pub fn pool_wrapper_active_node_reply_works_for_invalid_reply() {
            test::cleanup_storage();

            ProtocolVersion::set(2);
            _write_genesis_txns();

            let req = json!({
                "reqId": 1,
                "operation": {
                    "type": "1"
                }
            }).to_string();

            let rep = r#"{}"#;

            let p: PoolSM<MockNetworker, MockRequestHandler> = PoolSM::new(Rc::new(RefCell::new(MockNetworker::new(0, 0, vec![]))), POOL, 1, 0, 0);
            let p = p.handle_event(PoolEvent::CheckCache(1));
            let p = p.handle_event(PoolEvent::Synced(MerkleTree::from_vec(vec![]).unwrap()));
            let p = p.handle_event(PoolEvent::SendRequest(3, req, None, None));
            let p = p.handle_event(PoolEvent::NodeReply(rep.to_string(), "node".to_string()));
            assert_match!(PoolState::Active(_), p.state);
            match p.state {
                PoolState::Active(state) => {
                    assert_eq!(state.request_handlers.len(), 1);
                }
                _ => assert!(false)
            };

            test::cleanup_storage();
        }

        fn _write_genesis_txns() {
            let txns = test::gen_txns().join("\n");

            let mut path = environment::pool_path(POOL);
            fs::create_dir_all(path.as_path()).unwrap();
            path.push(POOL);
            path.set_extension("txn");
            let mut f = fs::File::create(path.as_path()).unwrap();
            f.write(txns.as_bytes()).unwrap();
            f.flush().unwrap();
            f.sync_all().unwrap();
        }
    }

    mod other {
        use super::*;

        #[test]
        fn get_f_works() {
            test::cleanup_storage();

            assert_eq!(_get_f(0), 0);
            assert_eq!(_get_f(3), 0);
            assert_eq!(_get_f(4), 1);
            assert_eq!(_get_f(5), 1);
            assert_eq!(_get_f(6), 1);
            assert_eq!(_get_f(7), 2);
        }
    }
}
