use super::zmq;
use std::thread::JoinHandle;
use std::thread;
use services::pool::commander::Commander;
use services::pool::events::PoolEvent;
use services::pool::networker::Networker;
use services::pool::events::RequestEvent;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use services::pool::request_handler::RequestHandler;
use std::rc::Rc;

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
}

impl<T: Networker, R: RequestHandler<T>> PoolState for GettingCatchupTargetState<T, R> {}

struct ActiveState<T: Networker, R: RequestHandler<T>> {
    networker: Rc<RefCell<T>>,
    request_handlers: HashMap<String, R>,
}

impl<'st, T: Networker, R: RequestHandler<T>> PoolState for ActiveState<T, R> {}

struct SyncCatchupState<T: Networker> {
    networker: Rc<RefCell<T>>
}

impl<T: Networker> PoolState for SyncCatchupState<T> {}

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
    state: T,
}

impl<T: Networker> PoolSM<InitializationState<T>> {
    pub fn new(networker: Rc<RefCell<T>>) -> PoolSM<InitializationState<T>> {
        PoolSM {
            state: InitializationState {
                networker
            }
        }
    }
}

// transitions from Initialization

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<InitializationState<T>>> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from(pool: PoolSM<InitializationState<T>>) -> PoolSM<GettingCatchupTargetState<T, R>> {
        let mut request_handler = R::new(pool.state.networker.clone());
        request_handler.process_event(Some(RequestEvent::LedgerStatus));
        PoolSM {
            state: GettingCatchupTargetState {
                networker: pool.state.networker,
                request_handler,
            }
        }
    }
}

impl<T: Networker> From<PoolSM<InitializationState<T>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<InitializationState<T>>) -> PoolSM<ClosedState> {
        PoolSM {
            state: ClosedState {}
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<InitializationState<T>>> for PoolSM<ActiveState< T, R>> {
    fn from(val: PoolSM<InitializationState<T>>) -> PoolSM<ActiveState<T, R>> {
        PoolSM {
            state: ActiveState {
                networker: val.state.networker,
                request_handlers: HashMap::new(),
            }
        }
    }
}

// transitions from GettingCatchupTarget

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<GettingCatchupTargetState<T, R>>> for PoolSM<SyncCatchupState<T>> {
    fn from(sm: PoolSM<GettingCatchupTargetState<T, R>>) -> Self {
        PoolSM {
            state: SyncCatchupState {
                networker: sm.state.networker
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<GettingCatchupTargetState<T, R>>> for PoolSM<TerminatedState<T>> {
    fn from(sm: PoolSM<GettingCatchupTargetState<T, R>>) -> Self {
        PoolSM {
            state: TerminatedState {
                networker: sm.state.networker
            }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<GettingCatchupTargetState<T, R>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<GettingCatchupTargetState<T, R>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Active

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<ActiveState<T, R>>> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from(pool: PoolSM<ActiveState<T, R>>) -> Self {
        let mut request_handler = R::new(pool.state.networker.clone());
        request_handler.process_event(Some(RequestEvent::LedgerStatus));
        //TODO: close connections!
        PoolSM {
            state: GettingCatchupTargetState { networker: pool.state.networker, request_handler }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<ActiveState<T, R>>> for PoolSM<TerminatedState<T>> {
    fn from(pool: PoolSM<ActiveState<T, R>>) -> Self {
        PoolSM {
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<ActiveState<T, R>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<ActiveState<T, R>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from SyncCatchup

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<SyncCatchupState<T>>> for PoolSM<ActiveState<T, R>> {
    fn from(pool: PoolSM<SyncCatchupState<T>>) -> Self {
        PoolSM {
            state: ActiveState {
                networker: pool.state.networker,
                request_handlers: HashMap::new(),
            }
        }
    }
}

impl<T: Networker> From<PoolSM<SyncCatchupState<T>>> for PoolSM<TerminatedState<T>> {
    fn from(pool: PoolSM<SyncCatchupState<T>>) -> Self {
        PoolSM {
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<T: Networker> From<PoolSM<SyncCatchupState<T>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<SyncCatchupState<T>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Terminated

impl<T: Networker, R: RequestHandler<T>> From<PoolSM<TerminatedState<T>>> for PoolSM<GettingCatchupTargetState<T, R>> {
    fn from(pool: PoolSM<TerminatedState<T>>) -> Self {
        let mut request_handler = R::new(pool.state.networker.clone());
        request_handler.process_event(Some(RequestEvent::LedgerStatus));
        PoolSM {
            state: GettingCatchupTargetState { networker: pool.state.networker, request_handler }
        }
    }
}

impl<T: Networker> From<PoolSM<TerminatedState<T>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<TerminatedState<T>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

enum PoolWrapper<T: Networker, R: RequestHandler<T>> {
    Initialization(PoolSM<InitializationState<T>>),
    GettingCatchupTarget(PoolSM<GettingCatchupTargetState<T, R>>),
    Active(PoolSM<ActiveState<T, R>>),
    Closed(PoolSM<ClosedState>),
    SyncCatchup(PoolSM<SyncCatchupState<T>>),
    Terminated(PoolSM<TerminatedState<T>>),
}

impl<T: Networker, R: RequestHandler<T>> PoolWrapper<T, R> {
    pub fn handle_event(self, pe: PoolEvent) -> Self {
        match self {
            PoolWrapper::Initialization(pool) => match pe {
                PoolEvent::CheckCache => {
                    //TODO: check cache freshness
                    let fresh = true;
                    if fresh {
                        PoolWrapper::Active(pool.into())
                    } else {
                        PoolWrapper::GettingCatchupTarget(pool.into())
                    }
                }
                PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                _ => PoolWrapper::Initialization(pool)
            }
            PoolWrapper::GettingCatchupTarget(mut pool) => {
                let pe = pool.state.request_handler.process_event(pe.into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::ConsensusFailed => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::ConsensusReached => {
                        //TODO: send CATCHUP_REQ
                        PoolWrapper::SyncCatchup(pool.into())
                    }
                    _ => PoolWrapper::GettingCatchupTarget(pool)
                }
            }
            PoolWrapper::Terminated(pool) => {
                match pe {
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::Refresh => {
                        PoolWrapper::GettingCatchupTarget(pool.into())
                    }
                    _ => PoolWrapper::Terminated(pool)
                }
            }
            PoolWrapper::Closed(pool) => PoolWrapper::Closed(pool),
            PoolWrapper::Active(mut pool) => {
                match pe {
                    PoolEvent::PoolOutdated => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::Refresh => {
                        PoolWrapper::GettingCatchupTarget(pool.into())
                    }
                    PoolEvent::SendRequest => {
                        let re: Option<RequestEvent> = pe.into();
                        let mut request_handler = R::new(pool.state.networker.clone());
                        request_handler.process_event(re);
                        //TODO: parse req_id
                        let req_id = "";
                        pool.state.request_handlers.insert(req_id.to_string(), request_handler);

                        PoolWrapper::Active(pool)
                    }
                    PoolEvent::NodeReply => {
                        //TODO: redirect reply to needed request handler
                        PoolWrapper::Active(pool)
                    }
                    _ => PoolWrapper::Active(pool)
                }
            }
            PoolWrapper::SyncCatchup(pool) => {
                match pe {
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::NodesBlacklisted => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::Synced => PoolWrapper::Active(pool.into()),
                    PoolEvent::NodeReply => {
                        //TODO: Build merkle tree if it is CATCHUP_REP
                        PoolWrapper::SyncCatchup(pool)
                    }
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
        self.worker = Some(thread::spawn(|| {
            let mut pool_thread: PoolThread<S, R> = PoolThread::new(cmd_socket);
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
    commander: Commander,
    networker: Rc<RefCell<S>>,
}

impl<S: Networker, R: RequestHandler<S>> PoolThread<S, R> {
    pub fn new(cmd_socket: zmq::Socket) -> Self {
        let networker = Rc::new(RefCell::new(S::new()));
        PoolThread {
            pool_wrapper: Some(PoolWrapper::Initialization(PoolSM::new(networker.clone()))),
            commander: Commander::new(cmd_socket),
            networker,
        }
    }

    pub fn work(&mut self) {
        self._poll();
        while self._loop() {
            self._poll();
        }
    }

    fn _loop(&mut self) -> bool {
        let pe = self.commander.get_next_event();
        match pe {
            Some(pe) => {
                self.pool_wrapper = self.pool_wrapper.take().map(|w| w.handle_event(pe));
            }
            _ => ()
        }
        self.pool_wrapper.as_ref().map(|w| w.is_terminal()).unwrap_or(true)
    }

    fn _poll(&self) {
        unimplemented!();
    }
}

mod pool_tests {
    use super::*;
    use services::pool::networker::MockNetworker;

    #[test]
    pub fn pool_new_works() {
        let networker = MockNetworker::new();
//        Pool::new(&Commander::new(), networker, MockConsensusCollector::new(&networker));
    }
}