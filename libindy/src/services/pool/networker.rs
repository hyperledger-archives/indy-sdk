extern crate zmq;

use self::zmq::PollItem;

use services::pool::events::*;

pub trait Networker {
    fn new() -> Self;
    fn fetch_events(&self) -> Vec<PoolEvent>;
    fn send_request(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent>;
    fn get_timeout(&self) -> i64;
    fn get_poll_items(&self) -> Vec<PollItem>;
}

pub struct ZMQNetworker {
    pool_connections: Vec<PoolConnection>,
}

impl Networker for ZMQNetworker {
    fn new() -> Self {
        ZMQNetworker {
            pool_connections: Vec::new(),
        }
    }

    fn fetch_events(&self) -> Vec<PoolEvent> {
        self.pool_connections.iter()
            .flat_map(PoolConnection::fetch_events).collect()
    }

    fn send_request(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
        match pe {
            Some(NetworkerEvent::SendAllRequest) | Some(NetworkerEvent::SendOneRequest) => {
                if self.pool_connections.last().map(PoolConnection::is_full).unwrap_or(true) {
                    self.pool_connections.push(PoolConnection {});
                }
                self.pool_connections.last().unwrap().send_request(pe);
                None
            },
            _ => unimplemented!()
        }
    }

    fn get_timeout(&self) -> i64 {
        self.pool_connections.iter()
            .fold(::std::i64::MAX, |acc, cur| ::std::cmp::min(acc, PoolConnection::get_timeout(cur)))
    }

    fn get_poll_items(&self) -> Vec<PollItem>{
        self.pool_connections.iter()
            .flat_map(PoolConnection::get_poll_items).collect()
    }
}

pub struct PoolConnection {}

impl PoolConnection {
    fn fetch_events(&self) -> Vec<PoolEvent> {
        unimplemented!()
    }

    fn get_poll_items(&self) -> Vec<PollItem> {
        unimplemented!()
    }

    fn get_timeout(&self) -> i64 {
        unimplemented!()
    }

    fn is_full(&self) -> bool {
        unimplemented!()
    }

    fn send_request(&self, pe: Option<NetworkerEvent>) {
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

    fn send_request(&mut self, pe: Option<NetworkerEvent>) -> Option<RequestEvent> {
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