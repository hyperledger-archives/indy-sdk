extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config, sovrin_open_pool_ledger};
use sovrin::api::ledger::sovrin_submit_request;

#[path = "../src/utils/environment.rs"]
mod environment;

use environment::EnvironmentUtils;

use std::fs;

use std::ptr::null;

use std::time::Duration;

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

    let pool_name = CString::new("test_open").unwrap();
    let pool_config = CString::new("{\"genesis_txn\": \"./test_open_src.txn\"}").unwrap();
    std::fs::File::create("./test_open_src.txn").unwrap();

    let err = sovrin_create_pool_ledger_config(command_handle,
                                               pool_name.as_ptr(),
                                               pool_config.as_ptr(),
                                               callback);

    assert_eq!(ErrorCode::Success, err);

    let err = receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    std::fs::remove_file("./test_open_src.txn").unwrap();
    std::fs::remove_dir_all(EnvironmentUtils::pool_path("test_open")).unwrap();
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

    let pool_name: String = "test1".to_string();
    let c_pool_name = CString::new(pool_name.as_str()).unwrap();
    let mut path = environment::EnvironmentUtils::pool_path(pool_name.as_str());
    fs::create_dir_all(path.as_path()).unwrap();
    path.push(pool_name);
    path.set_extension("txn");
    fs::File::create(path.as_path()).unwrap();

    let err = sovrin_open_pool_ledger(command_handle,
                                      c_pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(ErrorCode::Success, err);

    //TODO separate test to check error after open the same pool multiply times
    let (command_handle, callback) = CallbacksHelpers::closure_to_open_pool_ledger_cb(cb2);
    let err = sovrin_open_pool_ledger(command_handle,
                                      c_pool_name.as_ptr(),
                                      null(),
                                      callback);

    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, err);
}

#[test]
#[ignore] //required nodes pool available from CI
fn sovrin_submit_request_can_be_called() {
    //TODO call create pool ledger before open
    let (sender, receiver) = channel();
    let cb = Box::new(move |err, handle| {
        sender.send((err, handle)).unwrap();
    });
    let (command_handle, callback) = CallbacksHelpers::closure_to_open_pool_ledger_cb(cb);
    let pool_name = CString::new("test").unwrap();
    let err = sovrin_open_pool_ledger(command_handle,
                                      pool_name.as_ptr(),
                                      null(),
                                      callback);
    assert_eq!(ErrorCode::Success, err);
    let (err, pool_handle) = receiver.recv_timeout(std::time::Duration::from_secs(1)).unwrap();
    assert_eq!(ErrorCode::Success, err);
    std::thread::sleep(std::time::Duration::from_secs(1)); //TODO

    let (sender, receiver) = channel();
    let cb_send = Box::new(move |err, resp| {
        sender.send((err, resp)).unwrap();
    });
    let json = "{\
            \"reqId\":1491566332010860,\
            \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\
            \"operation\":{\
                \"type\":\"105\",\
                \"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\
            },\
            \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"\
        }\
        ";
    let req = CString::new(json).unwrap();
    let (command_handle, callback) = CallbacksHelpers::closure_to_send_tx_cb(cb_send);

    let err = sovrin_submit_request(command_handle,
                                    pool_handle,
                                    req.as_ptr(),
                                    callback);

    assert_eq!(ErrorCode::Success, err);
    let recv = receiver.recv_timeout(std::time::Duration::from_secs(1));
    recv.unwrap(); //TODO check data in ok
}
