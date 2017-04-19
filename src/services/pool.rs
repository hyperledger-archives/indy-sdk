extern crate zmq;

use commands::Command;
use commands::CommandExecutor;
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use self::zmq::Socket;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::thread;

pub struct PoolService {
    pools: RefCell<HashMap<i32, Socket>>,
    pools_names: RefCell<HashMap<String, i32>>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            pools: RefCell::new(HashMap::new()),
            pools_names: RefCell::new(HashMap::new()),
        }
    }

    fn run(cmd_sock: Socket, pool_id: i32, cmd_id: i32) {
        let mut socks_to_poll: [zmq::PollItem; 1] = [
            cmd_sock.as_poll_item(zmq::POLLIN),
        ];
        CommandExecutor::instance().send(Command::Pool(
            PoolCommand::OpenAck(cmd_id, Ok(pool_id)))); //TODO send only after catch-up?
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
        if self.pools_names.borrow().contains_key(&name.to_string()) {
            // TODO change error
            return Err(PoolError::InvalidHandle("Already opened".to_string()));
        }

        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let inproc_sock_name: String = format!("inproc://pool_{}", name);
        if recv_cmd_sock.bind(inproc_sock_name.as_str()).is_err() {
            return Err(PoolError::Io(Error::new(ErrorKind::ConnectionRefused, "Can't bind inproc socket")));
        }

        if send_cmd_sock.connect(inproc_sock_name.as_str()).is_err() {
            return Err(PoolError::Io(Error::new(ErrorKind::ConnectionRefused, "Can't connect to inproc socket")));
        }

        let pool_id: i32 = CommandExecutor::get_new_id();
        let cmd_id: i32 = CommandExecutor::get_new_id();
        thread::spawn(move || { PoolService::run(recv_cmd_sock, pool_id, cmd_id); });
        self.pools.borrow_mut().insert(pool_id, send_cmd_sock);
        self.pools_names.borrow_mut().insert(name.to_string(), pool_id);
        return Ok(cmd_id);
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