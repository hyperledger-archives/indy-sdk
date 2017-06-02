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
#[cfg(feature = "local_nodes_pool")]
use utils::test::TestUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::pool::PoolUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::wallet::WalletUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::ledger::LedgerUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::signus::SignusUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::anoncreds::AnoncredsUtils;

use std::collections::{HashMap, HashSet};


// TODO: FIXME: create_my_did doesn't support CID creation, but this trustee has CID as DID. So it is rough workaround for this issue.
// See: https://github.com/hyperledger/indy-sdk/issues/25
#[cfg(feature = "local_nodes_pool")]
fn get_trustee_keys(wallet_handle: i32) -> (String, String, String) {
    // workaround start >>>
    let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\"}");
    assert!(res.is_ok());
    let (trustee_did, trustee_verkey, trustee_pk) = res.unwrap();

    let res = SignusUtils::create_my_did(wallet_handle, &format!("{{\"did\":\"{}\", \"seed\":\"000000000000000000000000Trustee1\"}}", trustee_verkey));
    assert!(res.is_ok());
    res.unwrap()
    // workaround end <<<
}

mod high_cases {
    use super::*;

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_nym_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool1";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            let res = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default");
            assert!(res.is_ok());
            let wallet_handle = res.unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = get_trustee_keys(wallet_handle);

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}");
            assert!(res.is_ok());
            let (my_did, my_verkey, my_pk) = res.unwrap();

            let res = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None, None);
            assert!(res.is_ok());
            let nym_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_ok());
            let nym_response = res.unwrap();
            println!("nym_response {:?}", nym_response);

            let res = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone());
            assert!(res.is_ok());
            let get_nym_request = res.unwrap();

            let res = PoolUtils::send_request(pool_handle, &get_nym_request);
            assert!(res.is_ok());
            let get_nym_response = res.unwrap();
            println!("get_nym_response {:?}", get_nym_response);

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_attrib_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool2";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            let res = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default");
            assert!(res.is_ok());
            let wallet_handle = res.unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = get_trustee_keys(wallet_handle);

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}");
            assert!(res.is_ok());
            let (my_did, my_verkey, my_pk) = res.unwrap();

            let res = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None, None);
            assert!(res.is_ok());
            let nym_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_ok());
            let nym_response = res.unwrap();

            let res = LedgerUtils::build_attrib_request(&my_did.clone(), &my_did.clone(), None, Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"), None);
            assert!(res.is_ok());
            let attrib_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &attrib_request);
            assert!(res.is_ok());
            let attrib_response = res.unwrap();
            println!("attrib_response {}", attrib_response);

            let res = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "endpoint");
            assert!(res.is_ok());
            let get_attrib_request = res.unwrap();

            println!("get_attrib_request {}", get_attrib_request);
            let res = PoolUtils::send_request(pool_handle, &get_attrib_request);
            assert!(res.is_ok());
            let get_attrib_response = res.unwrap();
            println!("get_attrib_response {}", get_attrib_response);

            TestUtils::cleanup_storage();
        }
    }

    mod schema_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_schema_requests_works() {
            TestUtils::cleanup_storage();
            // TODO: FIXME: Understand why we use verkey insted of did as submitter id in NYM transaction
            let pool_name = "pool3";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            let res = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default");
            assert!(res.is_ok());
            let wallet_handle = res.unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = get_trustee_keys(wallet_handle);

            let res = SignusUtils::create_my_did(wallet_handle, "{}");
            assert!(res.is_ok());
            let (my_did, my_verkey, my_pk) = res.unwrap();

            let res = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None, None);
            assert!(res.is_ok());
            let nym_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_ok());
            let nym_response = res.unwrap();
            println!("nym_response {}", nym_response.clone());

            let schema_data = "{\"name\":\"gvt2\",\
                        \"version\":\"2.0\",\
                        \"keys\": [\"name\", \"male\"]}";
            let res = LedgerUtils::build_schema_request(&my_did.clone(), schema_data);
            assert!(res.is_ok());
            let schema_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert!(res.is_ok());
            let schema_response = res.unwrap();
            println!("schema_response {}", schema_response.clone());

            let get_schema_data = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            let res = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data);
            assert!(res.is_ok());
            let get_schema_request = res.unwrap();

            let res = PoolUtils::send_request(pool_handle, &get_schema_request);
            assert!(res.is_ok());
            let get_schema_response = res.unwrap();
            println!("get_schema_response {}", get_schema_response.clone());

            TestUtils::cleanup_storage();
        }
    }

    mod node_request {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_node_request_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool4";
            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            let res = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default");
            assert!(res.is_ok());
            let wallet_handle = res.unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\"}");
            assert!(res.is_ok());
            let (trustee_did, trustee_verkey, trustee_pk) = res.unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Steward1\"}");
            assert!(res.is_ok());
            let (my_did, my_verkey, my_pk) = res.unwrap();

            // TODO: FIXME: Understand why we use verkey insted of did as submitter id in NYM transaction
            let res = LedgerUtils::build_nym_request(&trustee_verkey.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None, None);
            assert!(res.is_ok());
            let nym_request = res.unwrap();

            println!("nym_request {}", nym_request.clone());
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_ok());
            let nym_response = res.unwrap();

            let node_data = "{\"node_ip\":\"192.168.53.148\",\
                      \"node_port\":9710, \
                      \"client_ip\":\"192.168.53.148\",\
                      \"client_port\":9709, \
                      \"alias\":\"Node5\", \
                      \"services\": [\"VALIDATOR\"]}";
            let res = LedgerUtils::build_node_request(&my_verkey.clone(), &my_did.clone(), node_data);
            let node_request = res.unwrap();


            println!("node_request {}", node_request.clone());
            let res: Result<String, ErrorCode> = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request);
            //TODO correct handling of Reject
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_claim_def_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool5";
            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            let res = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default");
            assert!(res.is_ok());
            let wallet_handle = res.unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = get_trustee_keys(wallet_handle);

            let res = SignusUtils::create_my_did(wallet_handle, "{}");
            assert!(res.is_ok());
            let (my_did, my_verkey, my_pk) = res.unwrap();

            let res = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None, None);
            assert!(res.is_ok());
            let nym_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_ok());
            let nym_response = res.unwrap();
            println!("nym_response {}", nym_response.clone());

            let schema_data = "{\"name\":\"gvt2\",\
                        \"version\":\"2.0\",\
                        \"keys\": [\"name\", \"male\"]}";
            let res = LedgerUtils::build_schema_request(&my_did.clone(), schema_data);
            assert!(res.is_ok());
            let schema_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert!(res.is_ok());
            let schema_response = res.unwrap();
            println!("schema_response {}", schema_response.clone());

            let get_schema_data = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            let res = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data);
            assert!(res.is_ok());
            let get_schema_request = res.unwrap();

            let res = PoolUtils::send_request(pool_handle, &get_schema_request);
            assert!(res.is_ok());
            let get_schema_response = res.unwrap();
            println!("get_schema_response {}", get_schema_response);
            let get_schema_response: Reply = serde_json::from_str(&get_schema_response).unwrap();
            //    let schema_result_data: GetSchemaResultData = serde_json::from_str(&get_schema_response.result.data.unwrap()).unwrap();

            let schema = Schema {
                name: get_schema_response.result.data.name,
                keys: get_schema_response.result.data.keys,
                version: get_schema_response.result.data.version,
                seq_no: get_schema_response.result.seq_no
            };

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &serde_json::to_string(&schema).unwrap());
            assert!(res.is_ok());
            let (claim_def_json, claim_def_uuid) = res.unwrap();
            println!("claim_def_json {:}", claim_def_json);

            let claim_def: ClaimDefinition = serde_json::from_str(&claim_def_json).unwrap();
            let claim_def_data = ClaimDefinitionData {
                primary: claim_def.public_key,
                revocation: claim_def.public_key_revocation
            };
            let claim_def_data_json = serde_json::to_string(&claim_def_data).unwrap();

            let res = LedgerUtils::build_claim_def_txn(&my_did.clone(), schema.seq_no, &claim_def.signature_type, &claim_def_data_json);
            assert!(res.is_ok());
            let claim_def_request = res.unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &claim_def_request);
            assert!(res.is_ok());
            let claim_def_response = res.unwrap();
            println!("claim_def_response {}", claim_def_response);

            let res = LedgerUtils::build_get_claim_def_txn(&my_did.clone(), schema.seq_no, &claim_def.signature_type, &get_schema_response.result.data.origin);
            assert!(res.is_ok());
            let get_claim_def_request = res.unwrap();

            let res = PoolUtils::send_request(pool_handle, &get_claim_def_request);
            assert!(res.is_ok());
            let get_claim_def_response = res.unwrap();
            println!("get_claim_def_response {}", get_claim_def_response);

            TestUtils::cleanup_storage();
        }
    }
}


#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    op: String,
    reason: String,
    req_id: u64,
    identifier: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct Reply {
    op: String,
    result: GetSchemaReplyResult,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct GetSchemaReplyResult {
    identifier: String,
    req_id: u64,
    seq_no: i32,
    #[serde(rename = "type")]
    _type: String,
    data: GetSchemaResultData,
    dest: Option<String>
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GetSchemaResultData {
    pub keys: HashSet<String>,
    pub name: String,
    pub origin: String,
    pub version: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub keys: HashSet<String>,
    pub seq_no: i32
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinition {
    pub public_key: PublicKey,
    pub public_key_revocation: Option<String>,
    pub schema_seq_no: i32,
    pub signature_type: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    pub n: String,
    pub s: String,
    pub rms: String,
    pub r: HashMap<String, String>,
    pub rctxt: String,
    pub z: String
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinitionData {
    pub primary: PublicKey,
    pub revocation: Option<String>
}


