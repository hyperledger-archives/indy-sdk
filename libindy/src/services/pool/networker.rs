use services::pool::pool::PoolEvent;
use services::pool::consensus_collector::ConsensusCollectorEvent;

enum NetworkerEvent {
    SendRequest
}

impl From<Option<ConsensusCollectorEvent>> for Option<NetworkerEvent> {
    fn from(cce: ConsensusCollectorEvent) -> Self {
        match cce {
            Some(ConsensusCollectorEvent::SendRequest) => Some(NetworkerEvent::SendRequest),
            _ => None
        }
    }
}

pub trait Networker {
    fn new() -> Self;
    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<ConsensusCollectorEvent>;
}

pub struct ZMQNetworker {}

impl Networker for ZMQNetworker {
    fn new() -> Self {
        Networker {}
    }

    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<ConsensusCollectorEvent> {
        unimplemented!();
    }
}

pub struct MockNetworker {}

impl Networker for MockNetworker {
    fn new() -> Self {
        unimplemented!()
    }

    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<ConsensusCollectorEvent> {
        unimplemented!()
    }
}



mod networker_tests {
    use super::*;
    
    #[test]
    pub fn networker_new_works() {
        Networker::new();
    }

    #[test]
    pub fn networker_process_event_works() {
        let networker = Networker::new();
        networker.process_event(None);
    }
}