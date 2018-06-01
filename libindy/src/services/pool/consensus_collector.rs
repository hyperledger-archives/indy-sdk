use services::pool::pool::PoolEvent;
use services::pool::networker::{MockNetworker, Networker};
use services::pool::networker;

pub enum ConsensusCollectorEvent {
    SendRequest,
    NodeReply
}

impl From<PoolEvent> for Option<ConsensusCollectorEvent> {
    fn from(pe: PoolEvent) -> Self {
        match pe {
            PoolEvent::NodeReply => Some(ConsensusCollectorEvent::NodeReply),
            PoolEvent::SendRequest => Some(ConsensusCollectorEvent::SendRequest),
            _ => None
        }
    }
}

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

struct FinishState {}

impl ConsensusState for FinishState {
    fn is_terminal() -> bool {
        true
    }
}

struct ConsensusCollectorSM<T: ConsensusState> {
    state: T
}

impl From<ConsensusCollectorSM<CollectingConsensusState>> for ConsensusCollectorSM<FinishState> {
    fn from(ccsm: ConsensusCollectorSM<CollectingConsensusState>) -> Self {
        ConsensusCollectorSM {
            state: FinishState {}
        }
    }
}

enum ConsensusCollectorSMWrapper {
    Start(ConsensusCollectorSM<StartState>),
    CollectingConsensus(ConsensusCollectorSM<CollectingConsensusState>),
    Finish(ConsensusCollectorSM<FinishState>)
}

impl ConsensusCollectorSMWrapper {
    fn
}

pub trait ConsensusCollector<T: Networker> {
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent>;
}

pub struct ConsensusCollectorImpl<T: Networker, S: ConsensusState> {
    state: S,
    networker: T
}

impl<T: Networker, S: ConsensusState> ConsensusCollectorImpl<T, S> {
    fn _handle_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        match pe {
            Some(pe) => {
                match pe {
                    ConsensusCollectorEvent::SendRequest => {
                        None
                    }
                    ConsensusCollectorEvent::NodeReply => {
                        // TODO: check consensus
                        Some(PoolEvent::ConsensusReached)
                    }
                }
            }
            _ => None
        }
    }
}

impl <T: Networker> ConsensusCollectorImpl<T, CollectingConsensusState> {
    pub fn new(networker: T) -> Self {
        ConsensusCollectorImpl {
            networker,
            state: CollectingConsensusState {}
        }
    }
}

impl<T: Networker, S: ConsensusState> ConsensusCollector<T> for ConsensusCollectorImpl<T, S> {
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        self._handle_event(
            self.networker.process_event(pe.into()).or(pe)
        )
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