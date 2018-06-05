use services::pool::events::NetworkerEvent;
use services::pool::events::RequestEvent;

pub trait Networker {
    fn new() -> Self;
    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<RequestEvent>;
    fn get_timeout() -> u32;
    fn poll();
}

pub struct ZMQNetworker {}

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {}
    }

    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        match pe {
            Some(NetworkerEvent::SendAllRequest) => None,
            Some(NetworkerEvent::SendOneRequest) => None,
            None => None
        }
    }

    fn get_timeout() -> u32 {
        unimplemented!()
    }

    fn poll() {
        unimplemented!()
    }
}

pub struct MockNetworker {}

impl Networker for MockNetworker {
    fn new() -> Self {
        unimplemented!()
    }

    fn process_event(&self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        unimplemented!()
    }

    fn get_timeout() -> u32 {
        unimplemented!()
    }

    fn poll() {
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