extern crate libsovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use libsovrin::api::ErrorCode;
use libsovrin::api::pool::sovrin_create_pool_ledger;

use utils::callbacks::CallbacksHelpers;

use std::sync::mpsc::{channel};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[test]
fn sovrin_create_pool_ledger_can_be_called() {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err);
    });

    let (command_handle, callback) = CallbacksHelpers::closure_to_create_pool_ledger_cb(cb);

    let pool_name = CString::new("sandbox").unwrap();
    let pool_config = CString::new("{\"genesis_txn\": \"sandbox.txn\"}").unwrap();

    let err = sovrin_create_pool_ledger(command_handle,
                                        pool_name.as_ptr(),
                                        pool_config.as_ptr(),
                                        callback);

    //let err = receiver.recv().unwrap();

    assert_eq!(ErrorCode::Success, err);
}