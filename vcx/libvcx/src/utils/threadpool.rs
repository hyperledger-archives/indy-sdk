extern crate tokio_threadpool;
extern crate futures;

use self::tokio_threadpool::{Builder, ThreadPool};
use self::futures::Future;

use std::sync::{Once, ONCE_INIT};
use std::sync::Mutex;
use std::collections::HashMap;
use std::thread;
use std::ops::FnOnce;

lazy_static! {
    static ref THREADPOOL: Mutex<HashMap<u32, ThreadPool>> = Default::default();
}

static TP_INIT: Once = ONCE_INIT;

pub static mut TP_HANDLE: u32 = 0;

pub fn init() {
    let size = ::settings::get_threadpool_size();

    if size == 0 {
        info!("no threadpool created, threadpool_size is 0");
        return;
    } else {
        TP_INIT.call_once(|| {
            let pool = Builder::new().pool_size(size).build();

            THREADPOOL.lock().unwrap().insert(1, pool);

            unsafe { TP_HANDLE = 1; }
        });
    }
}

pub fn spawn<F>(future: F)
where
    F: FnOnce() -> Result<(), ()> + Send + 'static {
        let handle;
        unsafe { handle = TP_HANDLE; }
        if ::settings::get_threadpool_size() == 0 || handle == 0{
            thread::spawn(future);
        }
        else {
            spawn_thread_in_pool(futures::lazy(future));
        }

}

fn spawn_thread_in_pool<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static {
    let handle;
    unsafe { handle = TP_HANDLE; }
    match THREADPOOL.lock().unwrap().get(&handle) {
        Some(x) => {
            let n = x.spawn(future);
        },
        None => panic!("no threadpool!"),
    }
}
