use super::consensus_collector::ConsensusCollector;
use super::consensus_collector;
use super::zmq;
use std::thread::JoinHandle;
use std::thread;
use services::pool::commander::Commander;
use services::pool::events::PoolEvent;
use services::pool::networker::Networker;
use services::pool::events::ConsensusCollectorEvent;
use services::pool::consensus_collector::ConsensusCollectorImpl;
use services::pool::request_handler::RequestHandler;
use services::pool::networker::ZMQNetworker;
use services::pool::events::RequestEvent;
use std::collections::HashMap;
use std::marker::PhantomData;

trait PoolState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct InitializationState<'st, T: Networker + 'st> {
    networker: &'st T
}

impl<'st, T: Networker> PoolState for InitializationState<'st, T> {}

struct GettingCatchupTargetState<'st, T: Networker + 'st, R: RequestHandler<'st, T>> {
    networker: &'st T,
    request_handler: R,
}

impl<'st, T: Networker, R: RequestHandler<'st, T>> PoolState for GettingCatchupTargetState<'st, T, R> {}

struct ActiveState<'st, T: Networker + 'st, R: RequestHandler<'st, T>> {
    networker: &'st T,
    request_handlers: HashMap<String, R>,
}

impl<'st, T: Networker, R: RequestHandler<'st, T>> PoolState for ActiveState<'st, T, R> {}

struct SyncCatchupState<'st, T: Networker + 'st> {
    networker: &'st T
}

impl<'st, T: Networker> PoolState for SyncCatchupState<'st, T> {}

struct TerminatedState<'st, T: Networker + 'st> {
    networker: &'st T
}

impl<'st, T: Networker> PoolState for TerminatedState<'st, T> {}

struct ClosedState {}

impl PoolState for ClosedState {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct PoolSM<T: PoolState> {
    state: T,
}

impl<'sm, T: Networker> PoolSM<InitializationState<'sm, T>> {
    pub fn new(networker: &'sm T) -> PoolSM<InitializationState<'sm, T>> {
        PoolSM {
            state: InitializationState {
                networker
            }
        }
    }
}

// transitions from Initialization

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<(PoolSM<InitializationState<'sm, T>>, R)> for PoolSM<GettingCatchupTargetState<'sm, T, R>> {
    fn from((pool, rh): (PoolSM<InitializationState<'sm, T>>, R)) -> PoolSM<GettingCatchupTargetState<'sm, T, R>> {
        PoolSM {
            state: GettingCatchupTargetState {
                networker: pool.state.networker,
                request_handler: rh,
            }
        }
    }
}

impl<'sm, T: Networker> From<PoolSM<InitializationState<'sm, T>>> for PoolSM<ClosedState> {
    fn from(val: PoolSM<InitializationState<'sm, T>>) -> PoolSM<ClosedState> {
        PoolSM {
            state: ClosedState {}
        }
    }
}

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<InitializationState<'sm, T>>> for PoolSM<ActiveState<'sm, T, R>> {
    fn from(val: PoolSM<InitializationState<'sm, T>>) -> PoolSM<ActiveState<'sm, T, R>> {
        PoolSM {
            state: ActiveState {
                networker: val.state.networker,
                request_handlers: HashMap::new(),
            }
        }
    }
}

// transitions from GettingCatchupTarget

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, R>>> for PoolSM<SyncCatchupState<'sm, T>> {
    fn from(sm: PoolSM<GettingCatchupTargetState<'sm, T, R>>) -> Self {
        PoolSM {
            state: SyncCatchupState {
                networker: sm.state.networker
            }
        }
    }
}

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, R>>> for PoolSM<TerminatedState<'sm, T>> {
    fn from(sm: PoolSM<GettingCatchupTargetState<'sm, T, R>>) -> Self {
        PoolSM {
            state: TerminatedState {
                networker: sm.state.networker
            }
        }
    }
}

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, R>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<GettingCatchupTargetState<'sm, T, R>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Active

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<(PoolSM<ActiveState<'sm, T, R>>, R)> for PoolSM<GettingCatchupTargetState<'sm, T, R>> {
    fn from((pool, rh): (PoolSM<ActiveState<'sm, T, R>>, R)) -> Self {
        PoolSM {
            state: GettingCatchupTargetState { networker: pool.state.networker, request_handler: rh }
        }
    }
}

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<ActiveState<'sm, T, R>>> for PoolSM<TerminatedState<'sm, T>> {
    fn from(pool: PoolSM<ActiveState<'sm, T, R>>) -> Self {
        PoolSM {
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<ActiveState<'sm, T, R>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<ActiveState<'sm, T, R>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

//impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<(R, PoolSM<ActiveState<'sm, T, R>>)> for PoolSM<ActiveState<'sm, T, R>> {
//    fn from((request_handlers, pool): (R, PoolSM<ActiveState<T, R>>)) -> Self {
//        PoolSM {
//            state: ActiveState {
//                request_handlers,
//                networker: pool.state.networker
//            }
//        }
//    }
//}

// transitions from SyncCatchup

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<PoolSM<SyncCatchupState<'sm, T>>> for PoolSM<ActiveState<'sm, T, R>> {
    fn from(pool: PoolSM<SyncCatchupState<'sm, T>>) -> Self {
        PoolSM {
            state: ActiveState {
                networker: pool.state.networker,
                request_handlers: HashMap::new(),
            }
        }
    }
}

impl<'sm, T: Networker> From<PoolSM<SyncCatchupState<'sm, T>>> for PoolSM<TerminatedState<'sm, T>> {
    fn from(pool: PoolSM<SyncCatchupState<'sm, T>>) -> Self {
        PoolSM {
            state: TerminatedState { networker: pool.state.networker }
        }
    }
}

impl<'sm, T: Networker> From<PoolSM<SyncCatchupState<'sm, T>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<SyncCatchupState<'sm, T>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Terminated

impl<'sm, T: Networker, R: RequestHandler<'sm, T>> From<(PoolSM<TerminatedState<'sm, T>>, R)> for PoolSM<GettingCatchupTargetState<'sm, T, R>> {
    fn from((pool, rh): (PoolSM<TerminatedState<'sm, T>>, R)) -> Self {
        PoolSM {
            state: GettingCatchupTargetState { networker: pool.state.networker, request_handler: rh }
        }
    }
}

impl<'sm, T: Networker> From<PoolSM<TerminatedState<'sm, T>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<TerminatedState<'sm, T>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

enum PoolWrapper<'wr, T: Networker + 'wr, R: RequestHandler<'wr, T>> {
    Initialization(PoolSM<InitializationState<'wr, T>>),
    GettingCatchupTarget(PoolSM<GettingCatchupTargetState<'wr, T, R>>),
    Active(PoolSM<ActiveState<'wr, T, R>>),
    Closed(PoolSM<ClosedState>),
    SyncCatchup(PoolSM<SyncCatchupState<'wr, T>>),
    Terminated(PoolSM<TerminatedState<'wr, T>>),
}

impl<'wr, T: Networker, R: RequestHandler<'wr, T>> PoolWrapper<'wr, T, R> {
    pub fn handle_event(self, pe: PoolEvent) -> Self {
        match self {
            PoolWrapper::Initialization(pool) => match pe {
                PoolEvent::CheckCache => {
                    //TODO: check cache freshness
                    let fresh = true;
                    if fresh {
                        PoolWrapper::Active(pool.into())
                    } else {
                        let mut request_handler = R::new(pool.state.networker);
                        request_handler.process_event(Some(RequestEvent::LedgerStatus));
                        PoolWrapper::GettingCatchupTarget((pool, request_handler).into())
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
                        let mut request_handler = R::new(pool.state.networker);
                        request_handler.process_event(Some(RequestEvent::LedgerStatus));
                        PoolWrapper::GettingCatchupTarget((pool, request_handler).into())
                    }
                    _ => PoolWrapper::Terminated(pool)
                }
            }
            PoolWrapper::Closed(pool) => PoolWrapper::Closed(pool),
            PoolWrapper::Active(pool) => {
                match pe {
                    PoolEvent::PoolOutdated => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::Refresh => {
                        let mut request_handler = R::new(pool.state.networker);
                        request_handler.process_event(Some(RequestEvent::LedgerStatus));
                        PoolWrapper::GettingCatchupTarget((pool, request_handler).into())
                    }
                    PoolEvent::SendRequest => {
                        let re: Option<RequestEvent> = pe.into();
                        let mut request_handler = R::new(pool.state.networker);
                        request_handler.process_event(re);
                        //TODO: put request_handler to map

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

pub struct Pool <S: Networker, R: RequestHandler<'static, S>>{
    _pd: PhantomData<(S, R)>,
    worker: Option<JoinHandle<()>>,
    name: String,
    id: i32,
}

impl<S: Networker + 'static, R: RequestHandler<'static, S> + 'static> Pool<S, R> {
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
            let mut pool_thread: PoolThread<'static, S, R> = PoolThread::new(cmd_socket);
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

struct PoolThread<'pt, S: Networker + 'pt, R: RequestHandler<'pt, S>> {
    pool_wrapper: Option<PoolWrapper<'pt, S, R>>,
    commander: Commander,
    networker: S,
}

impl<'pt, S: Networker, R: RequestHandler<'pt, S> + 'pt> PoolThread<'pt, S, R> {
    pub fn new(cmd_socket: zmq::Socket) -> Self {
        let networker = S::new();
        PoolThread {
            pool_wrapper: Some(PoolWrapper::Initialization(PoolSM::new(&networker))),
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
    use services::pool::consensus_collector::MockConsensusCollector;
    use services::pool::networker::MockNetworker;

    #[test]
    pub fn pool_new_works() {
        let networker = MockNetworker::new();
//        Pool::new(&Commander::new(), networker, MockConsensusCollector::new(&networker));
    }
}