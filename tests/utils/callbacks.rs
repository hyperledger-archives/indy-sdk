use sovrin::api::ErrorCode;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

pub struct CallbacksHelpers {}

impl CallbacksHelpers {
    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref CREATE_POOL_LEDGER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn create_pool_ledger_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_pool_ledger_callback))
    }

    pub fn closure_to_open_pool_ledger_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                          -> (i32,
                                              Option<extern fn(command_handle: i32, err: ErrorCode,
                                                               pool_handle: i32)>) {
        lazy_static! {
            static ref OPEN_POOL_LEDGER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        extern "C" fn open_pool_ledger_callback(command_handle: i32, err: ErrorCode, pool_handle: i32) {
            let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, pool_handle)
        }

        let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(open_pool_ledger_callback))
    }
}
