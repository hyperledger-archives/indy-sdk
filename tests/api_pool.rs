extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
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
use std::io::Write;

fn _create_default_pool_transaction_file() {
    let mut f = fs::File::create("./pool_transactions_sandbox").unwrap();
    let data = format!("{}\n{}\n{}\n{}\n",
                       "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                       "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.3\",\"client_port\":9704,\"node_ip\":\"10.0.0.3\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                       "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.4\",\"client_port\":9706,\"node_ip\":\"10.0.0.4\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                       "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"10.0.0.5\",\"client_port\":9708,\"node_ip\":\"10.0.0.5\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
    f.write_all(data.as_bytes()).unwrap();
    f.flush().unwrap();
}

#[test]
fn sovrin_create_pool_ledger_can_be_called() {
    let (sender, receiver) = channel();

    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (command_handle, callback) = CallbacksHelpers::closure_to_create_pool_ledger_cb(cb);

    let pool_name = CString::new("test_open").unwrap();
    let pool_config = CString::new("{\"genesis_txn\": \"./pool_transactions_sandbox\"}").unwrap();
    _create_default_pool_transaction_file();

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
    std::fs::remove_dir_all(EnvironmentUtils::pool_path("test")).unwrap();
    //create pool
    let (sender, receiver) = channel();
    let cb = Box::new(move |err| {
        sender.send(err).unwrap();
    });
    let (command_handle, callback) = CallbacksHelpers::closure_to_create_pool_ledger_cb(cb);
    let pool_name = CString::new("test").unwrap();
    let pool_config = CString::new("{\"genesis_txn\": \"./pool_transactions_sandbox\"}").unwrap();
    _create_default_pool_transaction_file();
    let err = sovrin_create_pool_ledger_config(command_handle,
                                               pool_name.as_ptr(),
                                               pool_config.as_ptr(),
                                               callback);
    assert_eq!(ErrorCode::Success, err);
    let err = receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(ErrorCode::Success, err);
    //open pool
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
    //test communication
    let (sender, receiver) = channel();
    let cb_send = Box::new(move |err, resp| {
        sender.send((err, resp)).unwrap();
    });
    let json = "{\
            \"reqId\":1491566332010860,\
            \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\
            \"operation\":{\
                \"type\":\"105\",\
                \"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"\
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

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    struct Reply {
        op: String,
        result: ReplyResult,
    }
    #[derive(Deserialize, Eq, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ReplyResult {
        txn_id: String,
        req_id: u64,
    }
    let exp_reply = Reply {
        op: "REPLY".to_string(),
        result: ReplyResult {
            req_id: 1491566332010860,
            txn_id: "5511e5493c1d37dfa67b73269a392a7aca5b71e9d10ac106adc7f9e552aee560".to_string(),
        }
    };
    let act_reply: Reply = serde_json::from_str(recv.unwrap().1.as_str()).unwrap();
    assert_eq!(act_reply, exp_reply);
    //    std::fs::remove_dir_all(EnvironmentUtils::pool_path("test")).unwrap();
}
