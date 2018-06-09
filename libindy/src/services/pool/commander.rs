extern crate byteorder;

use self::byteorder::{ByteOrder, LittleEndian, WriteBytesExt, ReadBytesExt};

use super::zmq;
use std::collections::VecDeque;
use errors::common::CommonError;
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

    pub fn fetch_events(&self) -> Option<PoolEvent> {
        let cmd = self.cmd_socket.recv_multipart(zmq::DONTWAIT).expect("FIXME");
        trace!("cmd {:?}", cmd);
        let cmd_s = String::from_utf8(cmd[0].clone())
            .map_err(|err|
                CommonError::InvalidState(format!("Invalid command received: {:?}", err)))
            .expect("FIXME");
        let id = cmd.get(1).map(|cmd: &Vec<u8>| LittleEndian::read_i32(cmd.as_slice()))
            .unwrap_or(-1);
        if "exit".eq(cmd_s.as_str()) {
            Some(PoolEvent::Close)  // FIXME pass id
        } else if "refresh".eq(cmd_s.as_str()) {
            Some(PoolEvent::Refresh) // FIXME pass id
        } else if "connect".eq(cmd_s.as_str()){
            Some(PoolEvent::CheckCache(id))
        } else {
            Some(PoolEvent::SendRequest(id, cmd_s))
        }
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