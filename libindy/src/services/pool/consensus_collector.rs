use services::pool::events::ConsensusCollectorEvent;
use services::pool::events::PoolEvent;
use services::pool::networker::{MockNetworker, Networker, NetworkerEvent};
use services::pool::networker;

trait ConsensusState {
    fn is_terminal() -> bool;
}

struct StartState {}

impl ConsensusState for StartState {
    fn is_terminal() -> bool {
        false
    }
}

struct CollectingConsensusState {}

impl ConsensusState for CollectingConsensusState {
    fn is_terminal() -> bool {
        false
    }
}

struct ConsensusCollectorSM<T: ConsensusState> {
    state: T
}

impl ConsensusCollectorSM<StartState> {
    fn new() -> Self {
        ConsensusCollectorSM {
            state: StartState {}
        }
    }
}

impl From<ConsensusCollectorSM<StartState>> for ConsensusCollectorSM<CollectingConsensusState> {
    fn from(_: ConsensusCollectorSM<StartState>) -> Self {
        ConsensusCollectorSM {
            state: CollectingConsensusState {}
        }
    }
}

impl From<ConsensusCollectorSM<CollectingConsensusState>> for ConsensusCollectorSM<StartState> {
    fn from(ccsm: ConsensusCollectorSM<CollectingConsensusState>) -> Self {
        ConsensusCollectorSM {
            state: StartState {}
        }
    }
}

enum ConsensusCollectorSMWrapper {
    Start(ConsensusCollectorSM<StartState>),
    CollectingConsensus(ConsensusCollectorSM<CollectingConsensusState>),
    Finish(ConsensusCollectorSM<FinishState>)
}

impl ConsensusCollectorSMWrapper {
    pub fn handle_event(self, pe: ConsensusCollectorEvent) -> (Self, Option<PoolEvent>) {
        match (self, pe) {
            (ConsensusCollectorSMWrapper::Start(consensus_collector), ConsensusCollectorEvent::StartConsensus) => {
                //TODO: check whether we need to get consensus
                //TODO: if we don't need
                //(ConsensusCollectorSMWrapper::Start(consensus_collector), None)
                //TODO: if we do
                (ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector.into()), None)
            }
            (ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector), ConsensusCollectorEvent::NodeReply) => {
                //TODO: check if consensus reached
                //TODO: if not
                //(ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector), None)
                //TODO: if failed
                //(ConsensusCollectorSMWrapper::Start(consensus_collector.into(), Some(PoolEvent::ConsensusFailed))
                //TODO: if success
                (ConsensusCollectorSMWrapper::Start(consensus_collector.into()), Some(PoolEvent::ConsensusReached))
            },
            _ => unimplemented!()
        }
    }
}

pub trait ConsensusCollector<T: Networker> {
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent>;
}

pub struct ConsensusCollectorImpl<'con, T: Networker> {
    consensus_collector_sm_wrapper: ConsensusCollectorSMWrapper,
    networker: &'con T
}

impl<'con, T: Networker> ConsensusCollectorImpl<'con, T> {
    fn _handle_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        match pe {
            Some(pe) => {
                let (wrapper, event) = self.consensus_collector_sm_wrapper.handle_event(pe);
                self.consensus_collector_sm_wrapper = wrapper;
                event
            }
            _ => None
        }
    }
}

impl <'con, T: Networker> ConsensusCollectorImpl<'con, T> {
    pub fn new(networker: &'con T) -> Self {
        ConsensusCollectorImpl {
            networker,
            consensus_collector_sm_wrapper: ConsensusCollectorSMWrapper::Start(ConsensusCollectorSM::new())
        }
    }
}

impl<'con, T: Networker> ConsensusCollector<T> for ConsensusCollectorImpl<'con, T> {
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        self._handle_event(
            self.networker.process_event(pe.into()).or(pe)
        )
    }
}

pub struct MockConsensusCollector<'mcon, T: Networker> {
    networker: &'mcon T
}

impl <'mcon, T: Networker> MockConsensusCollector<'mcon, T> {
    pub fn new(networker: &'mcon T) -> Self {
        MockConsensusCollector {
            networker
        }
    }
}

impl<'mcon, T: Networker> ConsensusCollector<T> for MockConsensusCollector<'mcon, T> {
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        unimplemented!()
    }
}

mod consensus_collector_tests {
    use super::*;

    #[test]
    pub fn consensus_collector_new_works() {
        let networker = MockNetworker::new();
        let a = ConsensusCollectorImpl::new(networker);
    }

    #[test]
    pub fn consensus_collector_process_event_works() {
        let networker = MockNetworker::new();
        let consensus_collector = ConsensusCollectorImpl::new(networker);
        consensus_collector.process_event(None);
    }
}