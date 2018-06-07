extern crate zmq;

use self::zmq::PollItem;

use services::pool::events::*;

pub trait Networker {
    fn new() -> Self;
    fn fetch_events(&self) -> Vec<PoolEvent>;
    fn send_request(&self, pe: Option<NetworkerRequestType>) -> Option<RequestEvent>;
    fn get_timeout(&self) -> i64;
    fn get_poll_items(&self) -> Vec<PollItem>;
}

pub struct ZMQNetworker {}

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {}
    }

    fn fetch_events(&self) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn send_request(&self, pe: Option<NetworkerRequestType>) -> Option<RequestEvent> {
        match pe {
            Some(NetworkerRequestType::SendAllRequest) => None,
            Some(NetworkerRequestType::SendOneRequest) => None,
            None => None
        }
    }

    fn get_timeout(&self) -> i64 {
        unimplemented!()
    }

    fn get_poll_items(&self) -> Vec<PollItem>{
        unimplemented!()
    }
}

#[cfg(test)]
pub struct MockNetworker {}

#[cfg(test)]
impl Networker for MockNetworker {
    fn new() -> Self {
        unimplemented!()
    }

    fn fetch_events(&self) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn send_request(&self, pe: Option<NetworkerRequestType>) -> Option<RequestEvent> {
        unimplemented!()
    }

    fn get_timeout(&self) -> i64 {
        unimplemented!()
    }

    fn get_poll_items(&self) -> Vec<PollItem>{
        unimplemented!()
    }
}


#[cfg(test)]
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