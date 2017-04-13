extern crate libsovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use libsovrin::api::ErrorCode;
use libsovrin::api::pool::sovrin_create_pool_ledger;

use utils::callbacks::CallbacksHelpers;

use std::sync::mpsc::{channel};
use std::ffi::{CString};

#[test]
fn sovrin_create_pool_ledger_can_be_called() {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (command_handle, callback) = CallbacksHelpers::closure_to_create_pool_ledger_cb(cb);

    let pool_name = CString::new("test1").unwrap();
    let pool_config = CString::new("{\"genesis_txn\": \"test1.txn\"}").unwrap();

    let err = sovrin_create_pool_ledger(command_handle,
                                        pool_name.as_ptr(),
                                        pool_config.as_ptr(),
                                        callback);

    assert_eq!(ErrorCode::Success, err);

    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);
}