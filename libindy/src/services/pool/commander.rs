extern crate byteorder;

use errors::common::CommonError;
use self::byteorder::{ByteOrder, LittleEndian};
use services::pool::events::PoolEvent;
use super::zmq;

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
        let cmd = match self.cmd_socket.recv_multipart(zmq::DONTWAIT).map_err(map_err_trace!()) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        };
        trace!("cmd {:?}", cmd);
        let cmd_s = match String::from_utf8(cmd[0].clone())
            .map_err(|err|
                CommonError::InvalidState(format!("Invalid command received: {:?}", err)))
            .map_err(map_err_trace!()) {
            Ok(cmd) => cmd,
            Err(_) => { return None; }
        };
        let id = cmd.get(1).map(|cmd: &Vec<u8>| LittleEndian::read_i32(cmd.as_slice()))
            .unwrap_or(-1);
        if "exit".eq(cmd_s.as_str()) {
            Some(PoolEvent::Close(id))
        } else if "refresh".eq(cmd_s.as_str()) {
            Some(PoolEvent::Refresh(id))
        } else if "connect".eq(cmd_s.as_str()) {
            Some(PoolEvent::CheckCache(id))
        } else {
            Some(PoolEvent::SendRequest(id, cmd_s))
        }
    }

    pub fn get_poll_item(&self) -> zmq::PollItem {
        self.cmd_socket.as_poll_item(zmq::POLLIN)
    }
}

#[cfg(test)]
mod commander_tests {
    use super::*;
    use utils::sequence::SequenceUtils;

    #[test]
    pub fn commander_new_works() {
        let zmq_ctx = zmq::Context::new();
        let cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        Commander::new(cmd_sock);
    }

    #[test]
    pub fn commander_get_poll_item_works() {
        let zmq_ctx = zmq::Context::new();
        let cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let cmd = Commander::new(cmd_sock);
        cmd.get_poll_item();
    }

    #[test]
    pub fn commander_fetch_works_when_socket_error() {
        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();

        let cmd = Commander::new(recv_cmd_sock);

        assert_match!(None, cmd.fetch_events());
    }

    #[test]
    pub fn commander_fetch_works_for_invalid_utf8() {
        let (send_cmd_sock, recv_cmd_sock) = _create_pair_of_sockets("invalid_utf8");

        let cmd = Commander::new(recv_cmd_sock);

        let buf: &[u8] = &vec![225][0..];
        send_cmd_sock.send_multipart(&[buf], zmq::DONTWAIT).expect("FIXME");
        assert_match!(None, cmd.fetch_events());
    }

    #[test]
    pub fn commander_fetch_close_event_works() {
        let (send_cmd_sock, recv_cmd_sock) = _create_pair_of_sockets("close");

        let cmd = Commander::new(recv_cmd_sock);

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let msg = "exit";
        send_cmd_sock.send_multipart(&[msg.as_bytes(), &buf], zmq::DONTWAIT).expect("FIXME");
        assert_match!(Some(PoolEvent::Close(cmd_id_)), cmd.fetch_events(), cmd_id_, cmd_id);
    }

    #[test]
    pub fn commander_fetch_refresh_event_works() {
        let (send_cmd_sock, recv_cmd_sock) = _create_pair_of_sockets("refresh");

        let cmd = Commander::new(recv_cmd_sock);

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let msg = "refresh";
        send_cmd_sock.send_multipart(&[msg.as_bytes(), &buf], zmq::DONTWAIT).expect("FIXME");
        assert_match!(Some(PoolEvent::Refresh(cmd_id_)), cmd.fetch_events(), cmd_id_, cmd_id);
    }

    #[test]
    pub fn commander_fetch_check_cache_event_works() {
        let (send_cmd_sock, recv_cmd_sock) = _create_pair_of_sockets("check_cache");

        let cmd = Commander::new(recv_cmd_sock);

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let msg = "connect";
        send_cmd_sock.send_multipart(&[msg.as_bytes(), &buf], zmq::DONTWAIT).expect("FIXME");
        assert_match!(Some(PoolEvent::CheckCache(cmd_id_)), cmd.fetch_events(), cmd_id_, cmd_id);
    }

    #[test]
    pub fn commander_fetch_send_request_event_works() {
        let (send_cmd_sock, recv_cmd_sock) = _create_pair_of_sockets("send_request");

        let cmd = Commander::new(recv_cmd_sock);

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let msg = "test";
        send_cmd_sock.send_multipart(&[msg.as_bytes(), &buf], zmq::DONTWAIT).expect("FIXME");
        assert_match!(Some(PoolEvent::SendRequest(cmd_id_, msg_)), cmd.fetch_events(),
                      cmd_id_, cmd_id,
                      msg_, msg);
    }

    fn _create_pair_of_sockets(addr: &str) -> (zmq::Socket, zmq::Socket) {
        let zmq_ctx = zmq::Context::new();
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();

        let inproc_sock_name: String = format!("inproc://{}", addr);
        recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
        send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
        (send_cmd_sock, recv_cmd_sock)
    }
}