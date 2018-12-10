extern crate tokio_threadpool;
extern crate futures;

use self::tokio_threadpool::{Builder, ThreadPool};
use self::futures::Future;

use std::sync::{Once, ONCE_INIT};
use std::sync::Mutex;
use std::thread;
use std::ops::FnOnce;

lazy_static! {
    static ref THREADPOOL: Mutex<Option<ThreadPool>> = Default::default();
}

static TP_INIT: Once = ONCE_INIT;

pub fn init() {
    let size = ::settings::get_threadpool_size();

    _init_cleaner();

    if size == 0 {
        info!("no threadpool created, threadpool_size is 0");
        return;
    } else {
        TP_INIT.call_once(|| {
            let pool = Builder::new().pool_size(size).build();
            let mut threadpool = THREADPOOL.lock().unwrap();
            *threadpool = Some(pool);
        });
    }
}

pub fn spawn<F>(future: F)
    where
        F: FnOnce() -> Result<(), ()> + Send + 'static {
    if ::settings::get_threadpool_size() == 0 || THREADPOOL.lock().unwrap().as_ref().is_none() {
        thread::spawn(future);
    } else {
        spawn_thread_in_pool(futures::lazy(future));
    }
}

fn spawn_thread_in_pool<F>(future: F)
    where
        F: Future<Item=(), Error=()> + Send + 'static {
    match THREADPOOL.lock().unwrap().as_ref() {
        Some(x) => {
            let n = x.spawn(future);
        }
        None => panic!("no threadpool!"),
    }
}

#[derive(Debug)]
struct ThreadPoolCleaner { name: &'static str }

impl Drop for ThreadPoolCleaner {
    fn drop(&mut self) {
        THREADPOOL.lock().unwrap().take();
    }
}

const CLEANER: ThreadPoolCleaner = ThreadPoolCleaner { name: "" };

fn _init_cleaner() {
    /// Dirty hack to force cleaning of lazy_static THREADPOOL when the library will be unloaded.
    /// The otherwise possible situation when the thread of application dies earlier then ThreadPool. In this case, SigAbort will be raised.
    println!(r#"{}"#, CLEANER.name);
}
