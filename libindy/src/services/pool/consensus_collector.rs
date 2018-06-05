use services::pool::events::*;
use services::pool::networker::{MockNetworker, Networker};
use services::pool::networker;
use services::pool::events::NetworkerEvent;
use services::pool::events::RequestEvent;

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
    CollectingConsensus(ConsensusCollectorSM<CollectingConsensusState>)
}

impl ConsensusCollectorSMWrapper {
    pub fn handle_event(self, pe: ConsensusCollectorEvent) -> (Self, Option<RequestEvent>) {
        match (self, pe) {
            (ConsensusCollectorSMWrapper::Start(consensus_collector), ConsensusCollectorEvent::StartConsensus) => {
                //TODO: send request from event to all nodes
                (ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector.into()), None)
            }
            (ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector), ConsensusCollectorEvent::NodeReply) => {
                //TODO: check if consensus reached
                //TODO: if not
                //(ConsensusCollectorSMWrapper::CollectingConsensus(consensus_collector), None)
                //TODO: if failed
                //(ConsensusCollectorSMWrapper::Start(consensus_collector.into(), Some(PoolEvent::ConsensusFailed))
                //TODO: if success
                (ConsensusCollectorSMWrapper::Start(consensus_collector.into()), Some(RequestEvent::ConsensusReached))
            },
            _ => unimplemented!()
        }
    }
}

pub trait ConsensusCollector<'con, T: Networker> {
    fn new(networker: &'con T) -> Self;
    fn process_event(&mut self, pe: Option<ConsensusCollectorEvent>) -> Option<RequestEvent>;
}

pub struct ConsensusCollectorImpl<'con, T: Networker + 'con> {
    consensus_collector_sm_wrapper: Option<ConsensusCollectorSMWrapper>,
    networker: &'con T
}

impl<'con, T: Networker> ConsensusCollectorImpl<'con, T> {

    fn _handle_event(&mut self, pe: Option<ConsensusCollectorEvent>) -> Option<RequestEvent> {
        match pe {
            Some(pe) => {
                if let Some((wrapper, event)) = self.consensus_collector_sm_wrapper.take().map(|w| w.handle_event(pe)) {
                    self.consensus_collector_sm_wrapper = Some(wrapper);
                    event
                } else {
                    self.consensus_collector_sm_wrapper = None;
                    None
                }
            }
            _ => None
        }
    }
}

impl<'con, T: Networker> ConsensusCollector<'con, T> for ConsensusCollectorImpl<'con, T> {
    fn new(networker: &'con T) -> Self {
        ConsensusCollectorImpl {
            networker,
            consensus_collector_sm_wrapper: Some(ConsensusCollectorSMWrapper::Start(ConsensusCollectorSM::new()))
        }
    }

    fn process_event(&mut self, pe: Option<ConsensusCollectorEvent>) -> Option<RequestEvent> {
        let ne: Option<Option<NetworkerEvent>> = pe.clone().map(|pe| pe.into());
        let ne = match ne {
            Some(ne) => ne,
            _ => None
        };
        self._handle_event(
            self.networker.process_event(ne).or(pe)
        )
    }
}

pub struct MockConsensusCollector<'mcon, T: Networker + 'mcon> {
    networker: &'mcon T
}

impl<'mcon, T: Networker> ConsensusCollector<'mcon, T> for MockConsensusCollector<'mcon, T> {
    fn new(networker: &'mcon T) -> Self {
        MockConsensusCollector {
            networker
        }
    }

    fn process_event(&mut self, pe: Option<ConsensusCollectorEvent>) -> Option<RequestEvent> {
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