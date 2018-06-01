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

pub trait ConsensusCollector<T: Networker> {
    fn new(networker: T) -> Self;
    fn process_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent>;
}

pub struct ConsensusCollectorImpl<T: Networker> {
    networker: T
}

impl<T: Networker> ConsensusCollectorImpl<T> {
    fn _handle_event(&self, pe: Option<ConsensusCollectorEvent>) -> Option<PoolEvent> {
        match pe {
            Some(pe) => {
                match pe {
                    ConsensusCollectorEvent::SendRequest => {
                        self.networker.send_request();
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

impl<T: Networker> ConsensusCollector<T> for ConsensusCollectorImpl<T> {
    fn new(networker: T) -> Self {
        ConsensusCollectorImpl {
            networker
        }
    }

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
        ConsensusCollectorImpl::new(networker);
    }

    #[test]
    pub fn consensus_collector_process_event_works() {
        let networker = MockNetworker::new();
        let consensus_collector = ConsensusCollectorImpl::new(networker);
        consensus_collector.process_event(None);
    }
}