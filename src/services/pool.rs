extern crate zmq;

use errors::pool::PoolError;
use self::zmq::Socket;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::thread;

pub struct PoolService {
    pools: HashMap<String, Socket>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            pools: HashMap::new(),
        }
    }

    fn run(cmd_sock: Socket) {
        let mut socks_to_poll: [zmq::PollItem; 1] = [
            cmd_sock.as_poll_item(zmq::POLLIN),
        ];
        loop {
            trace!("zmq poll loop >>");
            let r = zmq::poll(&mut socks_to_poll, -1);
            //FIXME implement
            trace!("zmq poll loop << ret {:?}, at cmd sock {:?}", r, cmd_sock.recv_string(0));
        }
    }

    pub fn create(&self, name: &str, config: &str) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn delete(&self, name: &str) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn open(&self, name: &str, config: &str) -> Result<i32, PoolError> {
        if self.pools.contains_key(&name.to_string()) {
            // TODO make methods of this service void and return error via ack command?
            //CommandExecutor::instance()
            // .send(super::super::commands::pool::PoolCommand::OpenAck())});
            // TODO change error
            return Err(PoolError::InvalidHandle("Already opened".to_string()));
        }

        let zmq_ctx = zmq::Context::new();
        //TODO ZMQ_PAIR may be unsupported on iOS
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let inproc_sock_name: String = format!("inproc://pool_{}", name);
        if recv_cmd_sock.bind(inproc_sock_name.as_str()).is_err() {
            return Err(PoolError::Io(Error::new(ErrorKind::ConnectionRefused, "Can't bind inproc socket")));
        }

        if send_cmd_sock.connect(inproc_sock_name.as_str()).is_err() {
            return Err(PoolError::Io(Error::new(ErrorKind::ConnectionRefused, "Can't connect to inproc socket")));
        }
        thread::spawn(move || {
            PoolService::run(recv_cmd_sock);
        });
        send_cmd_sock.send("test".as_bytes(), 0);
        // TODO mut ?
        // self.pools.insert(name.to_string(), send_cmd_sock);
        return Ok(0);
    }

    pub fn close(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn refresh(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod mocks {
    use super::*;

    use std::cell::RefCell;

    pub struct PoolService {
        create_results: RefCell<Vec<Result<(), PoolError>>>,
        delete_results: RefCell<Vec<Result<(), PoolError>>>,
        open_results: RefCell<Vec<Result<i32, PoolError>>>,
        close_results: RefCell<Vec<Result<(), PoolError>>>,
        refresh_results: RefCell<Vec<Result<(), PoolError>>>
    }

    impl PoolService {
        pub fn new() -> PoolService {
            PoolService {
                create_results: RefCell::new(Vec::new()),
                delete_results: RefCell::new(Vec::new()),
                open_results: RefCell::new(Vec::new()),
                close_results: RefCell::new(Vec::new()),
                refresh_results: RefCell::new(Vec::new())
            }
        }

        pub fn create(&self, name: &str, config: &str) -> Result<(), PoolError> {
            //self.create_results.pop().unwrap()
            unimplemented!()
        }

        pub fn delete(&self, name: &str) -> Result<(), PoolError> {
            //self.delete_results.pop().unwrap()
            unimplemented!()
        }

        pub fn open(&self, name: &str, config: &str) -> Result<i32, PoolError> {
            //self.open_results.pop().unwrap()
            unimplemented!()
        }

        pub fn close(&self, handle: i32) -> Result<(), PoolError> {
            //self.close_results.pop().unwrap()
            unimplemented!()
        }

        pub fn refresh(&self, handle: i32) -> Result<(), PoolError> {
            //self.refresh_results.pop().unwrap()
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_service_can_be_created() {
        let pool_service = PoolService::new();
        assert!(true, "No crashes on PoolService::new");
    }

    #[test]
    fn pool_service_can_be_dropped() {
        fn drop_test() {
            let pool_service = PoolService::new();
        }

        drop_test();
        assert!(true, "No crashes on PoolService::drop");
    }
}