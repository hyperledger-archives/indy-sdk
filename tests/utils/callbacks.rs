use std::sync::Mutex;

use sovrin::api::ErrorCode;

pub struct CallbacksHelpers {}

impl CallbacksHelpers {
    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref CREATE_POOL_LEDGER_CALLBACKS: Mutex<Vec<Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn create_pool_ledger_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(command_handle as usize);
            cb(err)
        }

        let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
        callbacks.push(closure);
        ((callbacks.len() - 1) as i32, Some(create_pool_ledger_callback))
    }
}