// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config, sovrin_open_pool_ledger};

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::test::TestUtils;

use std::fs;
use std::ffi::{CString};
use std::ptr::null;
use std::sync::mpsc::{channel};

#[test]
fn create_pool_ledger_works() {
    TestUtils::cleanup_storage();

    let err = _create_pool_ledger("pool1");
    assert_eq!(ErrorCode::Success, err);

    TestUtils::cleanup_storage();
}

#[test]
fn open_pool_ledger_works() {
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    let cb = Box::new(move |err, _| {
        sender.send(err).unwrap();
    });
    let cb2 = Box::new(move |err, _| {
        sender2.send(err).unwrap();
    });

    let (command_handle, callback) = CallbackUtils::closure_to_open_pool_ledger_cb(cb);

    let pool_name = CString::new("test1").unwrap();

    let err = sovrin_open_pool_ledger(command_handle,
                                      pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    //TODO separate test to check error after open the same pool multiply times
    let (command_handle, callback) = CallbackUtils::closure_to_open_pool_ledger_cb(cb2);
    let pool_name = CString::new("test1").unwrap();
    let err = sovrin_open_pool_ledger(command_handle,
                                      pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv().unwrap();
    assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, err);
}

fn _create_pool_ledger(pool_name: &str) -> ErrorCode {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (command_handle, callback) = CallbackUtils::closure_to_create_pool_ledger_cb(cb);

    let txn_file_name = EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name));
    let pool_name = CString::new(pool_name);
    let pool_config = CString::new(format!("{{\"genesis_txn\": \"{}\"}}", txn_file_name)).unwrap();
    fs::File::create(txn_file_name).unwrap();

    let err = sovrin_create_pool_ledger_config(command_handle,
                                               pool_name.as_ptr(),
                                               pool_config.as_ptr(),
                                               callback);

    if (err != ErrorCode::Success) {
        return err;
    }

    receiver.recv_timeout(Duration::from_secs(10)).unwrap();
}
