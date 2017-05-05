// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::ledger::sovrin_submit_request;

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::pool::PoolUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use std::fs;
use std::ffi::CString;
use std::sync::mpsc::channel;

#[test]
fn create_pool_ledger_config_works() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool_create");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
fn open_pool_ledger_works() {
    let name = "pool_open";
    let res = PoolUtils::create_pool_ledger_config(name);
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger(name);
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
fn open_pool_ledger_works_for_twice() {
    TestUtils::cleanup_storage();
    let pool_name = "pool_open_twice";

    let res = PoolUtils::create_pool_ledger_config(pool_name);
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger(pool_name);
    assert!(res.is_ok());
    let res = PoolUtils::open_pool_ledger(pool_name);
    assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

    TestUtils::cleanup_storage();
}

#[test]
#[ignore] //required nodes pool available from CI
fn sovrin_submit_request_works() {
    let pool_name = "test_submit_tx";
    #[allow(unused_must_use)]
    { fs::remove_dir_all(EnvironmentUtils::pool_path(pool_name)); }
    //create pool
    let res = PoolUtils::create_pool_ledger_config(pool_name);
    assert!(res.is_ok());
    //open pool
    let res = PoolUtils::open_pool_ledger(pool_name);
    assert!(res.is_ok());
    let pool_handle = res.unwrap();
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
    let (command_handle, callback) = CallbackUtils::closure_to_send_tx_cb(cb_send);

    let err = sovrin_submit_request(command_handle,
                                    pool_handle,
                                    req.as_ptr(),
                                    callback);

    assert_eq!(ErrorCode::Success, err);
    let resp = receiver.recv_timeout(TimeoutUtils::short_timeout());

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
    let act_reply: Reply = serde_json::from_str(resp.unwrap().1.as_str()).unwrap();
    assert_eq!(act_reply, exp_reply);
    fs::remove_dir_all(EnvironmentUtils::pool_path(pool_name)).unwrap();
}
