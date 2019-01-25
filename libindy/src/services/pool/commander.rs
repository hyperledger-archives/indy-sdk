extern crate byteorder;

use errors::prelude::*;
use services::pool::events::PoolEvent;

use super::zmq;

use self::byteorder::{ByteOrder, LittleEndian};

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
        let cmd_parts = self.cmd_socket.recv_multipart(zmq::DONTWAIT)
            .to_indy(IndyErrorKind::IOError, "ZMQ socket error on fetching pool events")
            .map_err(map_err_trace!())
            .ok()?;

        trace!("cmd_parts {:?}", cmd_parts);

        let cmd_s = String::from_utf8(cmd_parts[0].clone())
            .to_indy(IndyErrorKind::InvalidState, "Invalid utf8 sequence in command") // FIXME: review kind
            .map_err(map_err_trace!()).ok()?;

        let id = cmd_parts.get(1).map(|cmd: &Vec<u8>| LittleEndian::read_i32(cmd.as_slice()))
            .unwrap_or(-1);

        if "exit".eq(cmd_s.as_str()) {
            Some(PoolEvent::Close(id))
        } else if "refresh".eq(cmd_s.as_str()) {
            Some(PoolEvent::Refresh(id))
        } else if "connect".eq(cmd_s.as_str()) {
            Some(PoolEvent::CheckCache(id))
        } else {
            let timeout = LittleEndian::read_i32(cmd_parts[2].as_slice());
            let timeout = if timeout == -1 { None } else { Some(timeout) };

            let nodes = if let Some(nodes) = cmd_parts.get(3) {
                Some(String::from_utf8(nodes.clone())
                    .to_indy(IndyErrorKind::InvalidState, "Invalid utf8 sequence in command") // FIXME: review kind
                    .map_err(map_err_trace!()).ok()?)
            } else {
                None
            };

            Some(PoolEvent::SendRequest(id, cmd_s, timeout, nodes))
        }
    }

    pub fn get_poll_item(&self) -> zmq::PollItem {
        self.cmd_socket.as_poll_item(zmq::POLLIN)
    }
}

#[cfg(test)]
mod commander_tests {
    use utils::sequence;

    use super::*;

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

        let cmd_id: i32 = sequence::get_next_id();
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

        let cmd_id: i32 = sequence::get_next_id();
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

        let cmd_id: i32 = sequence::get_next_id();
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

        let cmd_id: i32 = sequence::get_next_id();
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let mut buf_to = [0u8; 4];
        LittleEndian::write_i32(&mut buf_to, -1);
        let msg = "test";
        send_cmd_sock.send_multipart(&[msg.as_bytes(), &buf, &buf_to], zmq::DONTWAIT).expect("FIXME");
        assert_match!(Some(PoolEvent::SendRequest(cmd_id_, msg_, None, None)), cmd.fetch_events(),
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