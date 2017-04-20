extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config, sovrin_open_pool_ledger};
use std::ptr::null;

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

    let err = sovrin_create_pool_ledger_config(command_handle,
                                               pool_name.as_ptr(),
                                               pool_config.as_ptr(),
                                               callback);

    assert_eq!(ErrorCode::Success, err);

    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);
}

#[test]
fn sovrin_open_pool_ledger_can_be_called() {
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    let cb = Box::new(move |err, _| {
        sender.send(err).unwrap();
    });
    let cb2 = Box::new(move |err, _| {
        sender2.send(err).unwrap();
    });

    let (command_handle, callback) = CallbacksHelpers::closure_to_open_pool_ledger_cb(cb);

    let pool_name = CString::new("test1").unwrap();

    let err = sovrin_open_pool_ledger(command_handle,
                                      pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    //TODO separate test to check error after open the same pool multiply times
    let (command_handle, callback) = CallbacksHelpers::closure_to_open_pool_ledger_cb(cb2);
    let pool_name = CString::new("test1").unwrap();
    let err = sovrin_open_pool_ledger(command_handle,
                                      pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, err);
}
