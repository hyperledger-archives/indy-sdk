extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

use indy::api::ErrorCode;
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
use utils::types::{
    ClaimDefinition,
    ClaimDefinitionData,
    GetAttribReplyResult,
    GetClaimDefReplyResult,
    GetNymReplyResult,
    GetSchemaReplyResult,
    Reply,
    SchemaResult,
    GetTxnResult,
    SchemaData,
    GetNymResultData
};
use std::collections::HashSet;
// TODO: FIXME: create_my_did doesn't support CID creation, but this trustee has CID as DID. So it is rough workaround for this issue.
// See: https://github.com/hyperledger/indy-sdk/issues/25


mod high_cases {
    use super::*;

    mod requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_request_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_send_request_works_for_invalid_pool_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = PoolUtils::send_request(invalid_pool_handle, &get_nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_sign_and_submit_request_works_for_invalid_pool_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = LedgerUtils::sign_and_submit_request(invalid_pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_sign_and_submit_request_works_for_invalid_wallet_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = LedgerUtils::sign_and_submit_request(pool_handle, invalid_wallet_handle, &trustee_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_incompatible_wallet_and_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger("pool1").unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("pool2", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works() {
            TestUtils::cleanup_storage();
            let pool_name = "test_submit_tx";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            let request = r#"{
                        "reqId":1491566332010860,
                         "identifier":"Th7MpTaRZVRYnPiabds81Y",
                         "operation":{
                            "type":"105",
                            "dest":"Th7MpTaRZVRYnPiabds81Y"
                         },
                         "signature":"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV"}"#;

            let resp = PoolUtils::send_request(pool_handle, request);
            let reply: serde_json::Value = serde_json::from_str(resp.unwrap().as_str()).unwrap();

            assert_eq!(reply["op"].as_str().unwrap(), "REPLY");
            assert_eq!(reply["result"]["type"].as_str().unwrap(), "105");
            assert_eq!(reply["result"]["reqId"].as_u64().unwrap(), 1491566332010860);

            let data: serde_json::Value = serde_json::from_str(reply["result"]["data"].as_str().unwrap()).unwrap();
            assert_eq!(data["dest"].as_str().unwrap(), "Th7MpTaRZVRYnPiabds81Y");
            assert_eq!(data["identifier"].as_str().unwrap(), "V4SGRU86Z58d6TV7PBUe6f");
            assert_eq!(data["role"].as_str().unwrap(), "2");
            assert_eq!(data["verkey"].as_str().unwrap(), "~7TYfekw4GUagBnBVCqPjiC");

            assert_eq!(reply["result"]["identifier"].as_str().unwrap(), "Th7MpTaRZVRYnPiabds81Y");
            assert_eq!(reply["result"]["dest"].as_str().unwrap(), "Th7MpTaRZVRYnPiabds81Y");

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_sign_and_submit_request_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sign_request {
        use super::*;

        #[test]
        fn indy_sign_request_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let message = r#"{
                "reqId":1496822211362017764,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"VsKV7grR1BUE29mG2Fm2kX",
                    "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                }
            }"#;

            let expected_signature = r#""signature":"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW""#;

            let msg = LedgerUtils::sign_request(wallet_handle, &my_did, message).unwrap();
            assert!(msg.contains(expected_signature));

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_unknow_signer() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let message = r#"{"reqId":1495034346617224651}"#;

            let res = LedgerUtils::sign_request(wallet_handle, "did", message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_request_works_for_invalid_message_format() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let message = r#"1495034346617224651"#;

            let res = LedgerUtils::sign_request(wallet_handle, &my_did, message);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_request_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let message = r#"{"reqId":1495034346617224651}"#;

            let res = LedgerUtils::sign_request(wallet_handle + 1, &my_did, message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_only_required_fields() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"dest\":\"{}\",\
                    \"type\":\"1\"\
                }}", identifier, dest);

            let nym_request = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), None, None, None).unwrap();
            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_with_option_fields() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            let verkey = "Anfh2rjAcxkE249DcdsaQl";
            let role = "STEWARD";
            let alias = "some_alias";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"alias\":\"{}\",\
                    \"dest\":\"{}\",\
                    \"role\":\"2\",\
                    \"type\":\"1\",\
                    \"verkey\":\"{}\"\
                }}",  identifier, alias, dest, verkey );

            let nym_request = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), Some(verkey), Some(alias), Some(role)).unwrap();

            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_empty_role() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"dest\":\"{}\",\
                    \"role\":null,\
                    \"type\":\"1\"\
                }}", identifier, dest);

            let nym_request = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), None, None, Some("")).unwrap();
            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_requests_works() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"105\",\
                    \"dest\":\"{}\"\
                }}", identifier, dest);

            let get_nym_request = LedgerUtils::build_get_nym_request(&identifier.clone(), &dest.clone()).unwrap();

            assert!(get_nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_without_signature() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_nym_request_works_without_signature";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&my_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_get_nym_request_works() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_send_get_nym_request_works";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_nym_requests_works";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();

            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            info!("nym_response {:?}", nym_response);

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            info!("get_nym_response {:?}", get_nym_response);

            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_raw_data() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";
            let raw = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"100\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{{\\\"endpoint\\\":{{\\\"ha\\\":\\\"127.0.0.1:5555\\\"}}}}\"\
                }}", identifier, dest);

            let attrib_request = LedgerUtils::build_attrib_request(&identifier, &dest, None, Some(raw), None).unwrap();

            assert!(attrib_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_missed_attribute() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";

            let res = LedgerUtils::build_attrib_request(&identifier, &dest, None, None, None);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";
            let raw = "endpoint";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"104\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{}\"\
                }}", identifier, dest, raw);

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&identifier, &dest, raw).unwrap();

            assert!(get_attrib_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_attrib_request_works_without_signature";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &attrib_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_attrib_requests_works";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let attrib_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &attrib_request).unwrap();
            info!("attrib_response {}", attrib_response);

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "endpoint").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            info!("get_attrib_response {}", get_attrib_response);

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod schema_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_correct_data_json() {
            let identifier = "identifier";
            let data = r#"{"name":"name", "version":"1.0", "attr_names":["name","male"]}"#;

            let expected_result = r#""operation":{"type":"101","data":{"name":"name","version":"1.0","attr_names":["name","male"]"#;

            let schema_request = LedgerUtils::build_schema_request(identifier, data).unwrap();

            assert!(schema_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_correct_data_json() {
            let identifier = "identifier";
            let data = r#"{"name":"name","version":"1.0"}"#;

            let expected_result = r#""identifier":"identifier","operation":{"type":"107","dest":"identifier","data":{"name":"name","version":"1.0"}}"#;

            let get_schema_request = LedgerUtils::build_get_schema_request(identifier, identifier, data).unwrap();
            assert!(get_schema_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_schema_request_works_without_signature";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"attr_names\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let res = PoolUtils::send_request(pool_handle, &schema_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_requests_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_schema_requests_works";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            info!("nym_response {}", nym_response.clone());

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"attr_names\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            info!("schema_response {}", schema_response);

            let get_schema_data = "{\"name\":\"gvt2\",\
                                    \"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();

            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();
            info!("get_schema_response {}", get_schema_response.clone());

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            assert!(get_schema_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod node_request {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_correct_data_json() {
            let identifier = "identifier";
            let dest = "dest";
            let data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 1, "alias":"some", "services": ["VALIDATOR"], "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}"#;

            let expected_result = r#""identifier":"identifier","operation":{"type":"0","dest":"dest","data":{"node_ip":"10.0.0.100","node_port":1,"client_ip":"10.0.0.100","client_port":1,"alias":"some","services":["VALIDATOR"],"blskey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;

            let node_request = LedgerUtils::build_node_request(identifier, dest, data).unwrap();
            assert!(node_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_send_node_request_works_without_signature";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Steward1")).unwrap();

            let node_data = "{\"node_ip\":\"10.0.0.100\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"10.0.0.100\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"],\
                              \"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res = PoolUtils::send_request(pool_handle, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        #[ignore] //FIXME currently unstable pool behaviour after new non-existing node was added
        fn indy_submit_node_request_works_for_new_steward() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_submit_node_request_works_for_new_steward";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let role = "STEWARD";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), None, Some(role)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let node_data = "{\"node_ip\":\"10.0.0.100\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"10.0.0.100\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"],\
                              \"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            let dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y"; // random(32) and base58

            let node_request = LedgerUtils::build_node_request(&my_did.clone(), dest, node_data).unwrap();

            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        fn indy_build_claim_def_request_works_for_correct_data_json() {
            let identifier = "identifier";
            let signature_type = "CL";
            let schema_seq_no = 1;
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"},"rctxt":"1","z":"1"}}"#;

            let expected_result = r#""identifier":"identifier","operation":{"ref":1,"data":{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"},"rctxt":"1","z":"1"},"revocation":{}},"type":"102","signature_type":"CL""#;

            let claim_def_request = LedgerUtils::build_claim_def_txn(identifier, schema_seq_no, signature_type, data).unwrap();
            assert!(claim_def_request.contains(expected_result));
        }

        #[test]
        fn indy_build_get_claim_def_request_works() {
            let identifier = "identifier";
            let _ref = 1;
            let signature_type = "signature_type";
            let origin = "origin";

            let expected_result = r#""identifier":"identifier","operation":{"type":"108","ref":1,"signature_type":"signature_type","origin":"origin"}"#;

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(identifier, _ref, signature_type, origin).unwrap();
            assert!(get_claim_def_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_claim_def_request_works_without_signature() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_claim_def_request_works_without_signature";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"attr_names\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();

            let get_schema_data = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();
            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();

            let claim_def_json = AnoncredsUtils::get_gvt_claim_def();

            let claim_def: ClaimDefinition = serde_json::from_str(&claim_def_json).unwrap();
            let claim_def_data = ClaimDefinitionData {
                public_key: claim_def.data.public_key,
                public_key_revocation: claim_def.data.public_key_revocation
            };
            let claim_def_data_json = serde_json::to_string(&claim_def_data).unwrap();

            let claim_def_request = LedgerUtils::build_claim_def_txn(&my_did.clone(), get_schema_response.result.seq_no.unwrap(),
                                                                     &claim_def.signature_type, &claim_def_data_json).unwrap();

            let res = PoolUtils::send_request(pool_handle, &claim_def_request);
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_claim_def_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_claim_def_requests_works";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"attr_names\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();

            let get_schema_data = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();
            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();

            let claim_def_json = AnoncredsUtils::get_gvt_claim_def();

            let claim_def: ClaimDefinition = serde_json::from_str(&claim_def_json).unwrap();
            let claim_def_data = ClaimDefinitionData {
                public_key: claim_def.data.public_key,
                public_key_revocation: claim_def.data.public_key_revocation
            };
            let claim_def_data_json = serde_json::to_string(&claim_def_data).unwrap();

            let claim_def_request = LedgerUtils::build_claim_def_txn(&my_did.clone(), get_schema_response.result.seq_no.unwrap(),
                                                                     &claim_def.signature_type, &claim_def_data_json).unwrap();

            let claim_def_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &claim_def_request).unwrap();
            info!("claim_def_response {}", claim_def_response);

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(&my_did.clone(),
                                                                             get_schema_response.result.seq_no.unwrap(),
                                                                             &claim_def.signature_type,
                                                                             &get_schema_response.result.dest.unwrap()).unwrap();

            let get_claim_def_response = PoolUtils::send_request(pool_handle, &get_claim_def_request).unwrap();
            info!("get_claim_def_response {}", get_claim_def_response);
            let _: Reply<GetClaimDefReplyResult> = serde_json::from_str(&get_claim_def_response).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_requests {
        use super::*;

        #[test]
        fn indy_build_get_txn_request() {
            let identifier = "identifier";
            let data = 1;

            let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1}"#;

            let get_txn_request = LedgerUtils::build_get_txn_request(identifier, data).unwrap();
            assert!(get_txn_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_txn_request_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let mut keys: HashSet<String> = HashSet::new();
            keys.insert("name".to_string());

            let schema_data = SchemaData {
                name: "gvt3".to_string(),
                version: "3.0".to_string(),
                attr_names: keys
            };
            let schema_data_json = serde_json::to_string(&schema_data).unwrap();
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), &schema_data_json).unwrap();
            let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            let schema_response: Reply<SchemaResult> = serde_json::from_str(&schema_response).unwrap();

            let get_schema_data = "{\"name\":\"gvt3\",\"version\":\"3.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();
            PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();

            let seq_no = schema_response.result.seq_no;

            let get_txn_request = LedgerUtils::build_get_txn_request(&my_did, seq_no).unwrap();
            let get_txn_response = LedgerUtils::submit_request(pool_handle, &get_txn_request).unwrap();

            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();

            let get_txn_schema_result: SchemaResult = serde_json::from_value(get_txn_response.result.data.unwrap()).unwrap();

            assert_eq!(schema_data, get_txn_schema_result.data.unwrap());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works_for_invalid_seq_no() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_txn_request_works_for_invalid_seq_no";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let mut keys: HashSet<String> = HashSet::new();
            keys.insert("name".to_string());

            let schema_data = SchemaData {
                name: "gvt3".to_string(),
                version: "3.0".to_string(),
                attr_names: keys
            };
            let schema_data_json = serde_json::to_string(&schema_data).unwrap();
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), &schema_data_json).unwrap();
            let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            let schema_response: Reply<SchemaResult> = serde_json::from_str(&schema_response).unwrap();

            let seq_no = schema_response.result.seq_no + 1;

            let get_txn_request = LedgerUtils::build_get_txn_request(&my_did, seq_no).unwrap();

            let get_txn_response = LedgerUtils::submit_request(pool_handle, &get_txn_request).unwrap();
            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();
            assert!(get_txn_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_not_found_signer() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_sign_and_submit_request_works_for_not_found_signer";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let signer_did = "Fjc4fCrwGxBq2BkqGs5wEu";
            let nym_request = LedgerUtils::build_nym_request(&signer_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &signer_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works_for_invalid_json() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_submit_request_works_for_invalid_json";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            let request = r#"request"#;

            let res = PoolUtils::send_request(pool_handle, request);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_json() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_sign_and_submit_request_works_for_invalid_json";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let request = r#"request"#;

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_only_required_fields() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_send_nym_request_works_for_only_required_fields";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_with_option_fields() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_send_nym_request_works_with_option_fields";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let role = "STEWARD";
            let alias = "some_alias";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), Some(alias), Some(role)).unwrap();

            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            info!("nym_response {:?}", nym_response);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_different_roles() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_send_nym_request_works_for_different_roles";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let role = "STEWARD";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, Some(role)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let (my_did2, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let role = "TRUSTEE";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did2.clone(), None, None, Some(role)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let (my_did3, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let role = "TRUST_ANCHOR";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did3.clone(), None, None, Some(role)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_wrong_role() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            let role = "WRONG_ROLE";

            let res = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), None, None, Some(role));
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_wrong_signer_role() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_nym_request_works_for_wrong_signer_role";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{\"cid\":true}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let (my_did2, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();
            let nym_request = LedgerUtils::build_nym_request(&my_did.clone(), &my_did2.clone(), None, None, None).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_unknown_signer_did() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_nym_request_works_for_unknown_signer_did";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"000000000000000000000000Trustee9","cid":true}"#).unwrap();
            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_nym_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_nym_request_works_for_unknown_did";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"00000000000000000000000000000My3"}"#).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_request_works_for_invalid_identifier() {
            let identifier = "invalid_base58_identifier";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let res = LedgerUtils::build_nym_request(identifier, dest, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_request_works_for_invalid_identifier() {
            let identifier = "invalid_base58_identifier";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let res = LedgerUtils::build_get_nym_request(identifier, dest);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works_for_reset_role() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_nym_requests_works_for_reset_role";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let mut nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(),
                                                                 Some(&my_verkey.clone()), None, Some("TRUSTEE")).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let mut get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();
            let get_nym_response_with_role = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            info!("get_nym_response_with_role {:?}", get_nym_response_with_role);

            let get_nym_response_with_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_with_role).unwrap();
            let get_nym_response_data_with_role: GetNymResultData = serde_json::from_str(&get_nym_response_with_role.result.data.unwrap()).unwrap();

            nym_request = LedgerUtils::build_nym_request(&my_did.clone(), &my_did.clone(),
                                                         Some(&my_verkey.clone()), None, Some("")).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();

            get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();
            let get_nym_response_without_role = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            info!("get_nym_response_without_role {:?}", get_nym_response_without_role);

            let get_nym_response_without_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_without_role).unwrap();
            let get_nym_response_data_without_role: GetNymResultData = serde_json::from_str(&get_nym_response_without_role.result.data.unwrap()).unwrap();

            assert!(get_nym_response_data_without_role.role.is_none());
            assert_ne!(get_nym_response_data_without_role.role, get_nym_response_data_with_role.role);

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_attrib_request_works_for_unknown_did";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &attrib_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_attrib_request_works_for_unknown_did";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"00000000000000000000000000000My2"}"#).unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "endpoint").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_attribute() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_attrib_request_works_for_unknown_attribute";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "some_attribute").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_request_works_for_invalid_identifier() {
            let identifier = "invalid_base58_identifier";

            let res = LedgerUtils::build_attrib_request(identifier, identifier, None, Some(r#"{"endpoint":{"ha":"127.0.0.1:5555"}}"#), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_request_works_for_invalid_identifier() {
            let identifier = "invalid_base58_identifier";

            let res = LedgerUtils::build_get_attrib_request(identifier, identifier, "endpoint");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod schemas_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_missed_field_in_data_json() {
            let identifier = "identifier";
            let data = r#"{"name":"name"}"#;

            let res = LedgerUtils::build_schema_request(identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_invalid_data_json_format() {
            let identifier = "identifier";
            let data = r#"{"name":"name", "keys":"name"}"#;

            let res = LedgerUtils::build_schema_request(identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_data_json() {
            let identifier = "identifier";
            let data = r#"{"name":"name"}"#;

            let res = LedgerUtils::build_get_schema_request(identifier, identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_schema_request_works_for_unknown_did";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"00000000000000000000000000000My3"}"#).unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"attr_names\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_schema_request_works_for_unknown_name() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_get_schema_request_works_for_unknown_name";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let get_schema_data = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did.clone(), get_schema_data).unwrap();

            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();
            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            assert!(get_schema_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }
    }

    mod node_requests {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_missed_field_in_data_json() {
            let identifier = "identifier";
            let dest = "dest";
            let data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 1}"#;

            let res = LedgerUtils::build_node_request(identifier, dest, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn indy_build_node_request_works_for_wrong_service() {
            let identifier = "identifier";
            let dest = "dest";
            let data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 1, "alias":"some", "services": ["SERVICE"], "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}"#;

            let res = LedgerUtils::build_node_request(identifier, dest, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_for_wrong_role() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_send_node_request_works_for_wrong_role";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

            let node_data = "{\"node_ip\":\"10.0.0.100\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"10.0.0.100\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"],\
                              \"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res: Result<String, ErrorCode> = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_node_request_works_for_already_has_node() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_submit_node_request_works_for_already_has_node";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Steward1")).unwrap();

            let node_data = "{\"node_ip\":\"10.0.0.100\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"10.0.0.100\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"],\
                              \"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res: Result<String, ErrorCode> = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        fn indy_build_claim_def_request_works_for_invalid_data_json() {
            TestUtils::cleanup_storage();

            let identifier = "identifier";
            let signature_type = "CL";
            let schema_seq_no = 1;
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"}}}"#;

            let res = LedgerUtils::build_claim_def_txn(identifier, schema_seq_no, signature_type, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }
}
