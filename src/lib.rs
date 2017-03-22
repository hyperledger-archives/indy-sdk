#[macro_use]
extern crate log;
extern crate zmq;

mod commands;
mod services;

use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

type SovrinCallback = Box<Fn(String) + Send>;

pub struct SovrinClient {
    cb: Option<SovrinCallback>,
    worker: Option<thread::JoinHandle<()>>,
    cmd_socket: Option<zmq::Socket>
}

impl SovrinClient {
    pub fn new() -> SovrinClient {
        SovrinClient {
            cb: None,
            worker: None,
            cmd_socket: None
        }
    }

    pub fn init(&mut self, cb: SovrinCallback) -> i32 {
        let arc = Arc::new(Mutex::new(cb));
        self.worker = Some(thread::spawn(move || {
            let xarc = arc.clone();
            println!("thread");
            let xcb = xarc.lock();
            xcb.unwrap()(String::from("qwe"));
        }));
        return 0;
    }


    pub fn do_call(&mut self, params: &[&str]) -> i32 {
        return 0;
    }

    pub fn deinit(&mut self) -> i32 {
        let x = self.worker.take().unwrap().join();
        return 0;
    }
}