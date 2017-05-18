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

#[cfg(feature = "local_nodes_pool")]
use sovrin::api::ErrorCode;

use utils::pool::PoolUtils;
use utils::test::TestUtils;

#[test]
fn create_pool_ledger_config_works() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool_create");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
fn open_pool_ledger_works() {
    TestUtils::cleanup_storage();
    let name = "pool_open";
    let res = PoolUtils::create_pool_ledger_config(name);
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger(name);
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
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
#[cfg(feature = "local_nodes_pool")]
fn sovrin_submit_request_works() {
    TestUtils::cleanup_storage();
    let pool_name = "test_submit_tx";

    let res = PoolUtils::create_pool_ledger_config(pool_name);
    assert!(res.is_ok());
    let res = PoolUtils::open_pool_ledger(pool_name);
    assert!(res.is_ok());
    let pool_handle = res.unwrap();

    let request = "{\
            \"reqId\":1491566332010860,\
            \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\
            \"operation\":{\
                \"type\":\"105\",\
                \"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"\
            },\
            \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"\
        }\
        ";
    let resp = PoolUtils::send_request(pool_handle, request);

    let exp_reply = Reply {
        op: "REPLY".to_string(),
        result: ReplyResult {
            req_id: 1491566332010860,
            data: Some("{\"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\"role\":\"2\",\"verkey\":null}".to_string()),
            identifier: "Th7MpTaRZVRYnPiabds81Y".to_string(),
        }
    };
    let act_reply: Reply = serde_json::from_str(resp.unwrap().as_str()).unwrap();
    assert_eq!(act_reply, exp_reply);
    TestUtils::cleanup_storage();
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct Reply {
    op: String,
    result: ReplyResult,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct ReplyResult {
    identifier: String,
    req_id: u64,
    data: Option<String>
}
