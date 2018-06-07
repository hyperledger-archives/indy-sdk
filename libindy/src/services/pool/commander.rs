use super::zmq;
use std::collections::VecDeque;
use services::pool::events::PoolEvent;

pub struct Commander {
    cmd_socket: zmq::Socket,
}

impl Commander {
    pub fn new(socket: zmq::Socket) -> Self {
        Commander {
            cmd_socket: socket,
        }
    }

    pub fn fetch_events(&mut self) -> Option<PoolEvent> {
        unimplemented!()
    }

    pub fn get_poll_item(&self) -> zmq::PollItem {
        self.cmd_socket.as_poll_item(zmq::POLLIN)
    }

    //TODO: push event -- formats of what will come to us?
}

mod commander_tests {
    use super::*;

    #[test]
    pub fn commander_new_works() {
        Commander::new();
    }
}