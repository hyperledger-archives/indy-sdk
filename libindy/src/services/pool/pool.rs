use super::consensus_collector::ConsensusCollector;
use super::consensus_collector;
use std::thread::JoinHandle;
use std::thread;
use services::pool::commander::Commander;
use services::pool::events::PoolEvent;
use services::pool::networker::Networker;
use services::pool::events::ConsensusCollectorEvent;
use services::pool::consensus_collector::ConsensusCollectorImpl;
use services::pool::networker::ZMQNetworker;

trait PoolState {
    fn is_terminal(&self) -> bool;
}

struct InitializationState {}

impl PoolState for InitializationState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct GettingCatchupTargetState<'st, T: Networker, CC: ConsensusCollector<'st, T>> {
    consensus_collector: CC
}

impl<'st, T: Networker, CC: ConsensusCollector<'st, T>> PoolState for GettingCatchupTargetState<'st, T, CC> {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct ActiveState {}

impl PoolState for ActiveState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct SyncCatchupState {}

impl PoolState for SyncCatchupState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct TerminatedState {}

impl PoolState for TerminatedState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct ClosedState {}

impl PoolState for ClosedState {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct PoolSM<T: PoolState> {
    state: T,
}

impl PoolSM<InitializationState> {
    pub fn new() -> PoolSM<InitializationState> {
        PoolSM {
            state: InitializationState {}
        }
    }
}

// transitions from Initialization

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<(PoolSM<InitializationState>, CC)> for PoolSM<GettingCatchupTargetState<'sm, T, CC>> {
    fn from((pool, cc): (PoolSM<InitializationState>, CC)) -> PoolSM<GettingCatchupTargetState<'sm, T, CC>> {
        PoolSM {
            state: GettingCatchupTargetState {
                consensus_collector: cc
            }
        }
    }
}

impl From<PoolSM<InitializationState>> for PoolSM<ClosedState> {
    fn from(val: PoolSM<InitializationState>) -> PoolSM<ClosedState> {
        PoolSM {
            state: ClosedState {}
        }
    }
}

impl From<PoolSM<InitializationState>> for PoolSM<ActiveState> {
    fn from(val: PoolSM<InitializationState>) -> PoolSM<ActiveState> {
        PoolSM {
            state: ActiveState {}
        }
    }
}

// transitions from GettingCatchupTarget

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, CC>>> for PoolSM<SyncCatchupState> {
    fn from(_: PoolSM<GettingCatchupTargetState<'sm, T, CC>>) -> Self {
        PoolSM {
            state: SyncCatchupState {}
        }
    }
}

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, CC>>> for PoolSM<TerminatedState> {
    fn from(_: PoolSM<GettingCatchupTargetState<'sm, T, CC>>) -> Self {
        PoolSM {
            state: TerminatedState {}
        }
    }
}

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<PoolSM<GettingCatchupTargetState<'sm, T, CC>>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<GettingCatchupTargetState<'sm, T, CC>>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Active

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<(PoolSM<ActiveState>, CC)> for PoolSM<GettingCatchupTargetState<'sm, T, CC>> {
    fn from((_, cc): (PoolSM<ActiveState>, CC)) -> Self {
        PoolSM {
            state: GettingCatchupTargetState { consensus_collector: cc }
        }
    }
}

impl From<PoolSM<ActiveState>> for PoolSM<TerminatedState> {
    fn from(_: PoolSM<ActiveState>) -> Self {
        PoolSM {
            state: TerminatedState {}
        }
    }
}

impl From<PoolSM<ActiveState>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<ActiveState>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from SyncCatchup

impl From<PoolSM<SyncCatchupState>> for PoolSM<ActiveState> {
    fn from(_: PoolSM<SyncCatchupState>) -> Self {
        PoolSM {
            state: ActiveState {}
        }
    }
}

impl From<PoolSM<SyncCatchupState>> for PoolSM<TerminatedState> {
    fn from(_: PoolSM<SyncCatchupState>) -> Self {
        PoolSM {
            state: TerminatedState {}
        }
    }
}

impl From<PoolSM<SyncCatchupState>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<SyncCatchupState>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

// transitions from Terminated

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<(PoolSM<TerminatedState>, CC)> for PoolSM<GettingCatchupTargetState<'sm, T, CC>> {
    fn from((_, cc): (PoolSM<TerminatedState>, CC)) -> Self {
        PoolSM {
            state: GettingCatchupTargetState { consensus_collector: cc }
        }
    }
}

impl From<PoolSM<TerminatedState>> for PoolSM<ClosedState> {
    fn from(_: PoolSM<TerminatedState>) -> Self {
        PoolSM {
            state: ClosedState {}
        }
    }
}

enum PoolWrapper <'wr, T: Networker, CC: ConsensusCollector<'wr, T>> {
    Initialization(PoolSM<InitializationState>),
    GettingCatchupTarget(PoolSM<GettingCatchupTargetState<'wr, T, CC>>),
    Active(PoolSM<ActiveState>),
    Closed(PoolSM<ClosedState>),
    SyncCatchup(PoolSM<SyncCatchupState>),
    Terminated(PoolSM<TerminatedState>),
}

impl<'wr, T: Networker, CC: ConsensusCollector<'wr, T>> PoolWrapper<'wr, T, CC> {
    pub fn handle_event(self, pe: PoolEvent) -> Self {
        match self {
            PoolWrapper::Initialization(pool) => match pe {
                PoolEvent::CheckCache => {
                    //TODO: check cache freshness
                    let fresh = true;
                    if fresh {
                        PoolWrapper::Active(pool.into())
                    } else {
                        PoolWrapper::GettingCatchupTarget((CC::new(T::new()), pool).into())
                    }
                }
                PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                _ => PoolWrapper::Initialization(pool)
            }
            PoolWrapper::GettingCatchupTarget(pool) => {
                let pe = pool.state.consensus_collector.process_event(pe.into()).unwrap_or(pe);
                match pe {
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::ConsensusFailed => PoolWrapper::Terminated(pool.into()),
                    PoolEvent::ConsensusReached => PoolWrapper::SyncCatchup(pool.into()),
                    _ => PoolWrapper::GettingCatchupTarget(pool)
                }
            }
            PoolWrapper::Terminated(pool) => {
                match pe {
                    PoolEvent::Close => PoolWrapper::Closed(pool.into()),
                    PoolEvent::Refresh => PoolWrapper::GettingCatchupTarget((CC::new(T::new()), pool).into())
                }
            }
        }
//        match (self, pe) {
//
//            (PoolWrapper::Active(pool), PoolEvent::Close) => PoolWrapper::Closed(pool.into()),
//            (PoolWrapper::Active(pool), PoolEvent::PoolOutdated) => PoolWrapper::Terminated(pool.into()),
//            (PoolWrapper::Active(pool), PoolEvent::Refresh) => PoolWrapper::GettingCatchupTarget(pool.into()),
//            (PoolWrapper::Active(pool), PoolEvent::ConsensusReached) => PoolWrapper::Active(pool),
//            (PoolWrapper::Active(pool), PoolEvent::ConsensusFailed) => PoolWrapper::Active(pool),
//
//            (PoolWrapper::SyncCatchup(pool), PoolEvent::Close) => PoolWrapper::Closed(pool.into()),
//            (PoolWrapper::SyncCatchup(pool), PoolEvent::NodesBlacklisted) => PoolWrapper::Terminated(pool.into()),
//            (PoolWrapper::SyncCatchup(pool), PoolEvent::Synced) => PoolWrapper::Active(pool.into()),
//
//            (PoolWrapper::Terminated(pool), ) => ,
//            (PoolWrapper::Terminated(pool), ) => ,
//
//            _ => unimplemented!()
//        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            &PoolWrapper::Initialization(ref pool) => pool.state.is_terminal(),
            _ => false
        }
    }
}

pub struct Pool <'pool, S: Networker, T: ConsensusCollector<'pool, S>>{
    pool_wrapper: PoolWrapper<'pool, S, T>,
    commander: &'pool Commander,
    consensus_collector: T,
    networker: S,
    worker: Option<JoinHandle<()>>
}

impl<'pool, S: Networker, T: ConsensusCollector<'pool, S>> Pool<'pool, S, T> {
    pub fn new(commander: &'pool Commander, networker: S, consensus_collector: T) -> Self {
        Pool {
            pool_wrapper: PoolWrapper::Initialization(PoolSM::new()),
            commander,
            consensus_collector,
            networker,
            worker: None,
        }
    }

    pub fn work(&self) {
        self.worker = Some(thread::spawn(move || {
            self._poll();
            while self._loop() {
                self._poll();
            }
        }));
    }

    fn _loop(&self) -> bool {
        let pe = self._get_event();
        match pe {
            Some(pe) => self._handle_event(pe),
            _ => ()
        }
        self.pool_wrapper.is_terminal()
    }

    fn _handle_event(&self, pe: PoolEvent) {
        self.pool_wrapper = self.pool_wrapper.handle_event(pe);
    }

    fn _get_event(&self) -> Option<PoolEvent> {
        let pe = self.commander.get_next_event();
        let cce: Option<Option<ConsensusCollectorEvent>> = pe.map(|pe| pe.into());
        let cce = match cce {
            Some(cce) => cce,
            _ => None
        };
        self.consensus_collector.process_event(cce).or(pe)
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
        Pool::new(&Commander::new(), networker, MockConsensusCollector::new(&networker));
    }
}