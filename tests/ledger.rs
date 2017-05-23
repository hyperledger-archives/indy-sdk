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

#[cfg(feature = "local_nodes_pool")]
use utils::logger::LoggerUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::wallet::WalletUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::ledger::LedgerUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::signus::SignusUtils;


#[test]
#[cfg(feature = "local_nodes_pool")]
fn sovrin_nym_requests_works() {
    TestUtils::cleanup_storage();
    LoggerUtils::init();

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

    let res = LedgerUtils::build_nym_request(&their_verkey.clone(), &my_verkey.clone(), None, None, None, None);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    let res = SignusUtils::sign(their_wallet_handle, &their_did, &nym_request);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    println!("{}", nym_request.clone());
    let res = PoolUtils::send_request(pool_handle, &nym_request);
    assert!(res.is_ok());
    let nym_response = res.unwrap();

    let res = LedgerUtils::build_get_nym_request(&my_verkey.clone(), &my_did.clone());
    assert!(res.is_ok());
    let get_nym_request = res.unwrap();

    println!("{}", get_nym_request.clone());
    let res = PoolUtils::send_request(pool_handle, &get_nym_request);
    assert!(res.is_ok());
    let nym_response = res.unwrap();

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
#[ignore]
fn sovrin_attrib_requests_works() {
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

    let res = LedgerUtils::build_nym_request(&their_verkey.clone(), &my_did.clone(), None, None, None, None);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    let res = SignusUtils::sign(their_wallet_handle, &their_did, &nym_request);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    println!("nym_request {}", nym_request.clone());
    let res = PoolUtils::send_request(pool_handle, &nym_request);
    assert!(res.is_ok());
    let nym_response = res.unwrap();

    let res = LedgerUtils::build_attrib_request(&their_verkey.clone(), &my_did.clone(), None, Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"), None);
    assert!(res.is_ok());
    let attrib_request = res.unwrap();

    let res = SignusUtils::sign(their_wallet_handle, &their_did, &attrib_request);
    assert!(res.is_ok());
    let attrib_request = res.unwrap();

    println!("attrib_request {}", attrib_request.clone());
    let res = PoolUtils::send_request(pool_handle, &attrib_request);
    assert!(res.is_ok());
    let attrib_response = res.unwrap();

    let res = LedgerUtils::build_get_attrib_request(&their_verkey.clone(), &my_did.clone(), "endpoint");
    assert!(res.is_ok());
    let get_attrib_request = res.unwrap();

    println!("get_attrib_request {}", get_attrib_request.clone());
    let res = PoolUtils::send_request(pool_handle, &get_attrib_request);
    assert!(res.is_ok());
    let get_attrib_response = res.unwrap();

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
#[ignore]
fn sovrin_schema_requests_works() {
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

    let res = LedgerUtils::build_nym_request(&their_verkey.clone(), &my_did.clone(), None, None, None, None);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    let res = SignusUtils::sign(their_wallet_handle, &their_did, &nym_request);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    println!("nym_request {}", nym_request.clone());
    let res = PoolUtils::send_request(pool_handle, &nym_request);
    assert!(res.is_ok());
    let nym_response = res.unwrap();

    let schema_data = "{\"name\":\"gvt\",\
                        \"version\":\"1.0\",\
                        \"keys\": [\"name\", \"male\"]}";
    let res = LedgerUtils::build_schema_request(&my_verkey.clone(), schema_data);
    assert!(res.is_ok());
    let schema_request = res.unwrap();

    let res = SignusUtils::sign(their_wallet_handle, &their_did, &schema_request);
    assert!(res.is_ok());
    let schema_request = res.unwrap();

    println!("schema_request {}", schema_request.clone());
    let res = PoolUtils::send_request(pool_handle, &schema_request);
    assert!(res.is_ok());
    let schema_response = res.unwrap();

    let get_schema_data = "{\"name\":\"gvt\",\"version\":\"1.0\"}";
    let res = LedgerUtils::build_get_schema_request(&my_verkey.clone(), get_schema_data);
    assert!(res.is_ok());
    let get_schema_request = res.unwrap();

    println!("get_schema_request {}", get_schema_request.clone());
    let res = PoolUtils::send_request(pool_handle, &get_schema_request);
    assert!(res.is_ok());
    let get_schema_response = res.unwrap();

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
#[ignore]
fn sovrin_node_request_works() {
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

    let my_did_json = "{\"seed\":\"000000000000000000000000Steward1\"}";
    let res = SignusUtils::create_my_did(my_wallet_handle, my_did_json);
    assert!(res.is_ok());
    let (my_did, my_verkey, my_pk) = res.unwrap();

    let their_did_json = "{\"seed\":\"000000000000000000000000Trustee1\"}";
    let res = SignusUtils::create_my_did(their_wallet_handle, their_did_json);
    assert!(res.is_ok());
    let (their_did, their_verkey, their_pk) = res.unwrap();

    let res = LedgerUtils::build_nym_request(&their_verkey.clone(), &my_did.clone(), None, None, None, None);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    let res = SignusUtils::sign(my_wallet_handle, &their_did, &nym_request);
    assert!(res.is_ok());
    let nym_request = res.unwrap();

    println!("nym_request {}", nym_request.clone());
    let res = PoolUtils::send_request(pool_handle, &nym_request);
    assert!(res.is_ok());
    let nym_response = res.unwrap();


    let node_data = "{\"node_ip\":\"192.168.53.148\",\
                      \"node_port\":9710, \
                      \"client_ip\":\"192.168.53.148\",\
                      \"client_port\":9709, \
                      \"alias\":Node5, \
                      \"services\": [\"VALIDATOR\"]}";
    let res = LedgerUtils::build_node_request(&their_verkey.clone(), &my_did.clone(), node_data);
    assert!(res.is_ok());
    let node_request = res.unwrap();

    let res = SignusUtils::sign(my_wallet_handle, &their_did, &node_request);
    assert!(res.is_ok());
    let node_request = res.unwrap();

    println!("node_request {}", node_request.clone());
    let res = PoolUtils::send_request(pool_handle, &node_request);
    assert!(res.is_ok());
    let node_response = res.unwrap();

    TestUtils::cleanup_storage();
}

//#[test]
//#[cfg(feature = "local_nodes_pool")]
//#[ignore]
//fn sovrin_claim_def_requests_works() {
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
//    let schema_data = "{\"name\":\"gvt\",\
//                        \"version\":\"1.0\",\
//                        \"keys\": [\"name\", \"male\"]}";
//    let res = LedgerUtils::build_schema_request(&my_verkey.clone(), schema_data);
//    assert!(res.is_ok());
//    let schema_request = res.unwrap();
//
//    let res = SignusUtils::sign(my_wallet_handle, &their_did, &schema_request);
//    assert!(res.is_ok());
//    let schema_request = res.unwrap();
//
//    println!("schema_request {}", schema_request.clone());
//    let res = PoolUtils::send_request(pool_handle, &schema_request);
//    assert!(res.is_ok());
//    let schema_response = res.unwrap();
//    let schema_response: Reply = serde_json::from_str(&schema_response).unwrap();
//    let schema = schema_response.result.data;
//
//    let schema: Schema = serde_json::from_str(&schema).unwrap();
//
//    let res = AnoncredsUtils::issuer_create_claim_definition(my_wallet_handle, &schema);
//    assert!(res.is_ok());
//    let (claim_def_json, claim_def_uuid) = res.unwrap();
//
//    //TODO Claim_def_json cast and change json
//    let signature_type = "CL".to_string();
//    let res = LedgerUtils::build_claim_def_txn(&my_verkey.clone(), schema.seq_no, signature_type, claim_def_json);
//    assert!(res.is_ok());
//    let claim_def_request = res.unwrap();
//
//    let res = SignusUtils::sign(my_wallet_handle, &their_did, &claim_def_request);
//    assert!(res.is_ok());
//    let claim_def_request = res.unwrap();
//
//    println!("claim_def_request {}", claim_def_request.clone());
//    let res = PoolUtils::send_request(pool_handle, &claim_def_request);
//    assert!(res.is_ok());
//    let claim_def_response = res.unwrap();
//
//    TestUtils::cleanup_storage();
//}

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

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSchemaResultData {
    pub attr_names: Vec<String>,
    pub name: String,
    pub origin: String,
    pub seq_no: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub version: String
}