//// TODO: FIXME: It must be removed after code layout stabilization!
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

use utils::pool::PoolUtils;
use utils::test::TestUtils;
use utils::signus::SignusUtils;
use utils::wallet::WalletUtils;


#[test]
fn create_pool_ledger_config_works() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool_create");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
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
            txn_id: "5511e5493c1d37dfa67b73269a392a7aca5b71e9d10ac106adc7f9e552aee560".to_string()
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
    txn_id: String,
    req_id: u64,
}


#[test]
#[ignore]
fn sovrin_attrib_request_works() {
    TestUtils::cleanup_storage();
    let pool_name = "test_submit_tx";
    let my_wallet_name = "my_wallet";
    let their_wallet_name = "their_wallet";
    let wallet_type = "default";

    let res = PoolUtils::create_pool_ledger_config(pool_name);
    assert!(res.is_ok());
    let res = PoolUtils::open_pool_ledger(pool_name);
    assert!(res.is_ok());
    let pool_handle = res.unwrap();

    let res = WalletUtils::create_wallet(pool_name, my_wallet_name, wallet_type);
    assert!(res.is_ok());
    let my_wallet_handle = res.unwrap();

    let res = WalletUtils::create_wallet(pool_name, their_wallet_name, wallet_type);
    assert!(res.is_ok());
    let their_wallet_handle = res.unwrap();

    let my_did_json = "{}";
    let res = SignusUtils::create_my_did(my_wallet_handle, my_did_json);
    assert!(res.is_ok());
    let (my_did, my_verkey, my_pk) = res.unwrap();

    let their_did_json = "{\"seed\":\"000000000000000000000000Trustee1\"}";
    let res = SignusUtils::create_my_did(their_wallet_handle, their_did_json);
    assert!(res.is_ok());
    let (their_did, their_verkey, their_pk) = res.unwrap();

    let nym_req_id = PoolUtils::get_req_id();
    let nym_operation = NymOperation::new(my_verkey.clone(), None, None, None, None);
    let mut nym_txn_req = Request::new(nym_req_id, their_verkey.clone(), nym_operation);

    let msg_for_sign = nym_txn_req.serialize_for_sign();
    let res = SignusUtils::sign(their_wallet_handle, &their_did.clone(), &msg_for_sign.clone());
    let signature = res.unwrap();
    nym_txn_req.signature = signature;

    let request = serde_json::to_string(&nym_txn_req).unwrap();
    let res = PoolUtils::send_request(pool_handle, &request);
    let resp = res.unwrap();
    let nym_resp: Reply = serde_json::from_str(&resp).unwrap();

    let get_nym_req_id = PoolUtils::get_req_id();
    let get_nym_operation = GetNymOperation::new(my_verkey.clone());
    let get_nym_txn_req = Request::new(get_nym_req_id, my_verkey.clone(), get_nym_operation);

    let request = serde_json::to_string(&get_nym_txn_req).unwrap();
    let res = PoolUtils::send_request(pool_handle, &request);
    let resp = res.unwrap();
    let get_nym_resp: Reply = serde_json::from_str(&resp).unwrap();

    let attrib_req_id = PoolUtils::get_req_id();

    use std::collections::HashMap;
    let mut attrs: HashMap<String, String> = HashMap::new();
    attrs.insert("ha".to_string(), "127.0.0.1:5555".to_string());
    let mut raw: HashMap<String, HashMap<String, String>> = HashMap::new();
    raw.insert("endpoint".to_string(), attrs);

    let attrib_operation = AttribOperation::new(my_verkey.clone(), None, Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}".to_string()), None);
    let mut attrib_txn_req = Request::new(attrib_req_id, their_verkey.clone(), attrib_operation);

    let msg_for_sign = attrib_txn_req.serialize_for_sign();
    println!("msg_for_sign{}", msg_for_sign.clone());
    let res = SignusUtils::sign(their_wallet_handle, &their_did.clone(), &msg_for_sign.clone());
    let signature = res.unwrap();
    attrib_txn_req.signature = signature;

    let request = serde_json::to_string(&attrib_txn_req).unwrap();
    println!("attrib_txn_req {}", request.clone());
    let res = PoolUtils::send_request(pool_handle, &request);
    let resp = res.unwrap();

    let get_attrib_req_id = PoolUtils::get_req_id();
    let get_attrib_operation = GetAttribOperation::new(my_verkey.clone(), "endpoint".to_string());
    let mut get_attrib_txn_req = Request::new(attrib_req_id, their_verkey.clone(), get_attrib_operation);

    let request = serde_json::to_string(&get_attrib_txn_req).unwrap();
    println!("get_attrib_txn_req {}", request.clone());
    let res = PoolUtils::send_request(pool_handle, &request);
    let resp = res.unwrap();
    //let nym_resp: Reply = serde_json::from_str(&resp).unwrap();

    TestUtils::cleanup_storage();
}

//#[test]
//#[ignore]
//fn sovrin_schema_request_works() {
//    TestUtils::cleanup_storage();
//    let pool_name = "test_submit_tx";
//    let my_wallet_name = "my_wallet";
//    let their_wallet_name = "their_wallet";
//    let wallet_type = "default";
//
//    let res = PoolUtils::create_pool_ledger_config(pool_name);
//    assert!(res.is_ok());
//    let res = PoolUtils::open_pool_ledger(pool_name);
//    assert!(res.is_ok());
//    let pool_handle = res.unwrap();
//
//    let res = WalletUtils::create_wallet(pool_name, my_wallet_name, wallet_type);
//    assert!(res.is_ok());
//    let my_wallet_handle = res.unwrap();
//
//    let res = WalletUtils::create_wallet(pool_name, their_wallet_name, wallet_type);
//    assert!(res.is_ok());
//    let their_wallet_handle = res.unwrap();
//
//    let my_did_json = "{}";
//    let res = SignusUtils::create_my_did(my_wallet_handle, my_did_json);
//    assert!(res.is_ok());
//    let (my_did, my_verkey, my_pk) = res.unwrap();
//
//    let their_did_json = "{\"seed\":\"000000000000000000000000Trustee1\"}";
//    let res = SignusUtils::create_my_did(their_wallet_handle, their_did_json);
//    assert!(res.is_ok());
//    let (their_did, their_verkey, their_pk) = res.unwrap();
//
//    let nym_req_id = PoolUtils::get_req_id();
//    let nym_operation = NymOperation::new(my_verkey.clone(), None, None, None, None);
//    let mut nym_txn_req = Request::new(nym_req_id, their_verkey.clone(), nym_operation);
//
//    let msg_for_sign = nym_txn_req.serialize_for_sign();
//    let res = SignusUtils::sign(their_wallet_handle, &their_did.clone(), &msg_for_sign.clone());
//    let signature = res.unwrap();
//    nym_txn_req.signature = signature;
//
//    let request = serde_json::to_string(&nym_txn_req).unwrap();
//    let res = PoolUtils::send_request(pool_handle, &request);
//    let resp = res.unwrap();
//    let nym_resp: Reply = serde_json::from_str(&resp).unwrap();
//
//    let get_nym_req_id = PoolUtils::get_req_id();
//    let get_nym_operation = GetNymOperation::new(my_verkey.clone());
//    let get_nym_txn_req = Request::new(get_nym_req_id, my_verkey.clone(), get_nym_operation);
//
//    let request = serde_json::to_string(&get_nym_txn_req).unwrap();
//    let res = PoolUtils::send_request(pool_handle, &request);
//    let resp = res.unwrap();
//    let get_nym_resp: Reply = serde_json::from_str(&resp).unwrap();
//
//    let schema_req_id = PoolUtils::get_req_id();
//    let schema_operation_data = SchemaOperationData::new("gvt".to_string(), "1.0".to_string(), "name,male".to_string());
//    let schema_operation = SchemaOperation::new(schema_operation_data);
//    let mut schema_txn_req = Request::new(schema_req_id, their_verkey.clone(), schema_operation);
//
//    let msg_for_sign = schema_txn_req.serialize_for_sign();
//    println!("msg_for_sign{}", msg_for_sign.clone());
//    let res = SignusUtils::sign(their_wallet_handle, &their_did.clone(), &msg_for_sign.clone());
//    let signature = res.unwrap();
//    schema_txn_req.signature = signature;
//
//    let request = serde_json::to_string(&schema_txn_req).unwrap();
//    println!("schema_txn_req {}", request.clone());
//    let res = PoolUtils::send_request(pool_handle, &request);
//    let resp = res.unwrap();
//
//    let get_schema_req_id = PoolUtils::get_req_id();
//    let get_schema_operation_data = GetSchemaOperationData::new("gvt".to_string(), "1.0".to_string());
//    let get_schema_operation = GetSchemaOperation::new(my_verkey.clone(), get_schema_operation_data);
//    let mut get_schema_txn_req = Request::new(get_schema_req_id, their_verkey.clone(), get_schema_operation);
//
//    let request = serde_json::to_string(&get_schema_txn_req).unwrap();
//    println!("get_schema_txn_req {}", request.clone());
//    let res = PoolUtils::send_request(pool_handle, &request);
//    let resp = res.unwrap();
//
//    TestUtils::cleanup_storage();
//}
//
////#[test]
////#[ignore]
////fn sovrin_node_request_works() {
////    TestUtils::cleanup_storage();
////    let pool_name = "test_submit_tx";
////    let my_wallet_name = "my_wallet";
////    let their_wallet_name = "their_wallet";
////    let wallet_type = "default";
////
////    let res = PoolUtils::create_pool_ledger_config(pool_name);
////    assert!(res.is_ok());
////    let res = PoolUtils::open_pool_ledger(pool_name);
////    assert!(res.is_ok());
////    let pool_handle = res.unwrap();
////
////    let res = WalletUtils::create_wallet(pool_name, my_wallet_name, wallet_type);
////    assert!(res.is_ok());
////    let my_wallet_handle = res.unwrap();
////
////    let res = WalletUtils::create_wallet(pool_name, their_wallet_name, wallet_type);
////    assert!(res.is_ok());
////    let their_wallet_handle = res.unwrap();
////
////    let my_did_json = "{\"seed\":\"000000000000000000000000Steward1\"}";
////    let res = SignusUtils::create_my_did(my_wallet_handle, my_did_json);
////    assert!(res.is_ok());
////    let (my_did, my_verkey, my_pk) = res.unwrap();
////
////    let their_did_json = "{\"seed\":\"000000000000000000000000Trustee1\"}";
////    let res = SignusUtils::create_my_did(their_wallet_handle, their_did_json);
////    assert!(res.is_ok());
////    let (their_did, their_verkey, their_pk) = res.unwrap();
////
////    let nym_req_id = PoolUtils::get_req_id();
////    let nym_operation = NymOperation::new(my_verkey.clone(), None, None, None, None);
////    let mut nym_txn_req = Request::new(nym_req_id, their_verkey.clone(), nym_operation);
////
////    let msg_for_sign = nym_txn_req.serialize_for_sign();
////    let res = SignusUtils::sign(their_wallet_handle, &their_did.clone(), &msg_for_sign.clone());
////    let signature = res.unwrap();
////    nym_txn_req.signature = signature;
////
////    let request = serde_json::to_string(&nym_txn_req).unwrap();
////    let res = PoolUtils::send_request(pool_handle, &request);
////    let resp = res.unwrap();
////    let nym_resp: Reply = serde_json::from_str(&resp).unwrap();
////
////
////    let req_id = PoolUtils::get_req_id();
////    let node_operation_data = NodeOperationData::new("192.168.53.148".to_string(),
////                                                     9710,
////                                                     "192.168.53.148".to_string(),
////                                                     9709,
////                                                     "Node5".to_string(),
////                                                     vec!["VALIDATOR".to_string()]
////    );
////
////    let node_operation = NodeOperation::new(my_verkey.clone(), node_operation_data);
////    let mut node_txn_req = Request::new(req_id, my_verkey.clone(), node_operation);
////
////    let msg_for_sign = node_txn_req.serialize_for_sign();
////    println!("{}", msg_for_sign);
////    let res = SignusUtils::sign(my_wallet_handle, &my_did.clone(), &msg_for_sign.clone());
////    let signature = res.unwrap();
////    node_txn_req.signature = signature;
////
////    let request = serde_json::to_string(&node_txn_req).unwrap();
////    println!("{}", request.clone());
////    let res = PoolUtils::send_request(pool_handle, &request);
////    let resp = res.unwrap();
////    let nym_resp: Reply = serde_json::from_str(&resp).unwrap();
////
////
////    TestUtils::cleanup_storage();
////}