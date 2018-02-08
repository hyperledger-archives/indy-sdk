extern crate indy;
extern crate time;
extern crate openssl;
extern crate hex;
extern crate sodiumoxide;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
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
use utils::did::DidUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::anoncreds::AnoncredsUtils;
use utils::types::*;
use utils::constants::*;

use self::openssl::hash::{MessageDigest, Hasher};
use self::hex::ToHex;
use self::sodiumoxide::crypto::secretbox;

mod high_cases {
    use super::*;

    mod requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_request_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did, &my_did).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = LedgerUtils::submit_request(invalid_pool_handle, &get_nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did, &my_did).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = LedgerUtils::sign_and_submit_request(invalid_pool_handle, wallet_handle, &my_did, &get_nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = LedgerUtils::sign_and_submit_request(pool_handle, invalid_wallet_handle, &trustee_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_incompatible_wallet_and_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool", None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let request = r#"{
                        "reqId":1491566332010860,
                         "identifier":"Th7MpTaRZVRYnPiabds81Y",
                         "operation":{
                            "type":"105",
                            "dest":"Th7MpTaRZVRYnPiabds81Y"
                         },
                         "signature":"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV"}"#;

            let resp = LedgerUtils::submit_request(pool_handle, request);
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

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sign_request {
        use super::*;

        #[test]
        fn indy_sign_request_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = LedgerUtils::sign_request(wallet_handle, DID, MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_request_works_for_invalid_message_format() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let res = LedgerUtils::sign_request(wallet_handle, &my_did, "1495034346617224651");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_request_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = LedgerUtils::sign_request(invalid_wallet_handle, &my_did, MESSAGE);
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
            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"dest\":\"{}\",\
                    \"type\":\"1\"\
                }},\
                \"protocolVersion\":1", IDENTIFIER, DEST);

            let nym_request = LedgerUtils::build_nym_request(&IDENTIFIER, &DEST, None, None, None).unwrap();
            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_with_option_fields() {
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
                }},\
                \"protocolVersion\":1", IDENTIFIER, alias, DEST, VERKEY_TRUSTEE);

            let nym_request = LedgerUtils::build_nym_request(&IDENTIFIER, &DEST, Some(VERKEY_TRUSTEE), Some(alias), Some(role)).unwrap();
            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_empty_role() {
            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"dest\":\"{}\",\
                    \"role\":null,\
                    \"type\":\"1\"\
                }},\
                \"protocolVersion\":1", IDENTIFIER, DEST);

            let nym_request = LedgerUtils::build_nym_request(&IDENTIFIER, &DEST, None, None, Some("")).unwrap();
            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_requests_works() {
            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"105\",\
                    \"dest\":\"{}\"\
                }},\
                \"protocolVersion\":1", IDENTIFIER, DEST);

            let get_nym_request = LedgerUtils::build_get_nym_request(&IDENTIFIER, &DEST).unwrap();
            assert!(get_nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&did, &did, None, None, None).unwrap();

            let response = LedgerUtils::submit_request(pool_handle, &nym_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_get_nym_request_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&did, &did).unwrap();

            let get_nym_response = LedgerUtils::submit_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            let nym_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did, &my_did).unwrap();
            let get_nym_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_nym_request, &nym_resp).unwrap();

            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_raw_data() {
            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"100\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{{\\\"endpoint\\\":{{\\\"ha\\\":\\\"127.0.0.1:5555\\\"}}}}\"\
                }},\
                \"protocolVersion\":1", IDENTIFIER, DEST);

            let attrib_request = LedgerUtils::build_attrib_request(&IDENTIFIER, &DEST, None, Some(ATTRIB_RAW_DATA), None).unwrap();
            assert!(attrib_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_missed_attribute() {
            let res = LedgerUtils::build_attrib_request(&IDENTIFIER, &DEST, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works() {
            let raw = "endpoint";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"104\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{}\"\
                }},\
                \"protocolVersion\":1", IDENTIFIER, DEST, raw);

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&IDENTIFIER, &DEST, raw).unwrap();
            assert!(get_attrib_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did, &my_did, None, Some(ATTRIB_RAW_DATA), None).unwrap();

            let response = LedgerUtils::submit_request(pool_handle, &attrib_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_raw() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&trustee_did,
                                                                   &trustee_did,
                                                                   None,
                                                                   Some(ATTRIB_RAW_DATA),
                                                                   None).unwrap();
            let attrib_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&trustee_did, &trustee_did, "endpoint").unwrap();
            let get_attrib_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_attrib_request, &attrib_req_resp).unwrap();

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_some());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //TODO Delete after Indy-Node fixed
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_hash_value() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let mut ctx = Hasher::new(MessageDigest::sha256()).unwrap();
            ctx.update(&ATTRIB_RAW_DATA.as_bytes()).unwrap();
            let hashed_attr = ctx.finish2().unwrap().as_ref().to_hex();

            let attrib_request = LedgerUtils::build_attrib_request(&trustee_did,
                                                                   &trustee_did,
                                                                   Some(&hashed_attr),
                                                                   None,
                                                                   None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //TODO Delete after Indy-Node fixed
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_encrypted_value() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let key = secretbox::gen_key();
            let nonce = secretbox::gen_nonce();
            let encryted_attr = secretbox::seal(&ATTRIB_RAW_DATA.as_bytes(), &nonce, &key).to_hex();

            let attrib_request = LedgerUtils::build_attrib_request(&trustee_did,
                                                                   &trustee_did,
                                                                   None,
                                                                   None,
                                                                   Some(&encryted_attr)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod schema_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_correct_data_json() {
            let expected_result = r#""operation":{"type":"101","data":{"name":"name","version":"1.0","attr_names":["name","male"]}},"protocolVersion":1"#;

            let schema_request = LedgerUtils::build_schema_request(IDENTIFIER, SCHEMA_DATA).unwrap();
            assert!(schema_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_correct_data_json() {
            let expected_result = format!(r#""identifier":"{}","operation":{{"type":"107","dest":"{}","data":{{"name":"name","version":"1.0"}}}},"protocolVersion":1"#,
                                          IDENTIFIER, DEST);

            let get_schema_request = LedgerUtils::build_get_schema_request(IDENTIFIER, DEST, GET_SCHEMA_DATA).unwrap();
            assert!(get_schema_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, SCHEMA_DATA).unwrap();

            let response = LedgerUtils::submit_request(pool_handle, &schema_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_requests_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, SCHEMA_DATA).unwrap();
            let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();

            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, GET_SCHEMA_DATA).unwrap();
            let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_req_resp).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            assert!(get_schema_response.result.data.is_some());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod node_request {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_correct_data_json() {
            let expected_result = format!(r#""identifier":"{}","operation":{{"type":"0","dest":"{}","data":{{"node_ip":"10.0.0.100","node_port":1,"client_ip":"10.0.0.100","client_port":1,"alias":"some","services":["VALIDATOR"],"blskey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}}},"protocolVersion":1"#,
                                          IDENTIFIER, DEST);

            let node_request = LedgerUtils::build_node_request(IDENTIFIER, DEST, NODE_DATA).unwrap();
            assert!(node_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(STEWARD_SEED)).unwrap();

            let node_request = LedgerUtils::build_node_request(&did, &did, NODE_DATA).unwrap();

            let response = LedgerUtils::submit_request(pool_handle, &node_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        #[ignore] //FIXME currently unstable pool behaviour after new non-existing node was added
        fn indy_submit_node_request_works_for_new_steward() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let role = "STEWARD";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, Some(role)).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y"; // random(32) and base58

            let node_request = LedgerUtils::build_node_request(&my_did, dest, NODE_DATA).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        fn indy_build_claim_def_request_works_for_correct_data_json() {
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"},"rctxt":"1","z":"1"}}"#;

            let expected_result = format!(r#""identifier":"{}","operation":{{"ref":{},"data":{{"primary":{{"n":"1","s":"2","rms":"3","r":{{"name":"1"}},"rctxt":"1","z":"1"}}}},"type":"102","signature_type":"{}"}},"protocolVersion":1"#,
                                          IDENTIFIER, SEQ_NO, SIGNATURE_TYPE);

            let claim_def_request = LedgerUtils::build_claim_def_txn(IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, &data).unwrap();
            assert!(claim_def_request.contains(&expected_result));
        }

        #[test]
        fn indy_build_get_claim_def_request_works() {
            let expected_result = format!(r#""identifier":"{}","operation":{{"type":"108","ref":{},"signature_type":"{}","origin":"{}"}},"protocolVersion":1"#,
                                          IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, IDENTIFIER);

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, IDENTIFIER).unwrap();
            assert!(get_claim_def_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_claim_def_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, SCHEMA_DATA).unwrap();
            let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();

            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, GET_SCHEMA_DATA).unwrap();
            let get_schema_response_str = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_req_resp).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response_str).unwrap();

            let claim_def_data_json = AnoncredsUtils::gvt_claim_def_data_json();

            let claim_def_request = LedgerUtils::build_claim_def_txn(&did, get_schema_response.result.seq_no.unwrap(),
                                                                     SIGNATURE_TYPE, &claim_def_data_json).unwrap();

            let response = LedgerUtils::submit_request_with_retries(pool_handle, &claim_def_request, &get_schema_response_str).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_claim_def_requests_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, SCHEMA_DATA).unwrap();
            let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();

            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, GET_SCHEMA_DATA).unwrap();
            let get_schema_response_str = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_req_resp).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response_str).unwrap();
            let schema_seq_no = get_schema_response.result.seq_no.unwrap();

            let claim_def_data_json = AnoncredsUtils::gvt_claim_def_data_json();

            let claim_def_request = LedgerUtils::build_claim_def_txn(&did, schema_seq_no,
                                                                     SIGNATURE_TYPE, &claim_def_data_json).unwrap();

            let claim_def_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &claim_def_request).unwrap();

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(&did, schema_seq_no,
                                                                             &SIGNATURE_TYPE, &did).unwrap();

            let get_claim_def_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_claim_def_request, &claim_def_req_resp).unwrap();
            let _: Reply<GetClaimDefReplyResult> = serde_json::from_str(&get_claim_def_response).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_requests {
        use super::*;

        #[test]
        fn indy_build_get_txn_request() {
            let expected_result = format!(r#""identifier":"{}","operation":{{"type":"3","data":{}}},"protocolVersion":1"#, IDENTIFIER, SEQ_NO);

            let get_txn_request = LedgerUtils::build_get_txn_request(IDENTIFIER, SEQ_NO).unwrap();
            assert!(get_txn_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, &SCHEMA_DATA).unwrap();
            let schema_response_str = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            let schema_response: Reply<SchemaResult> = serde_json::from_str(&schema_response_str).unwrap();
            let seq_no = schema_response.result.seq_no;

            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, GET_SCHEMA_DATA).unwrap();
            LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response_str).unwrap();

            let get_txn_request = LedgerUtils::build_get_txn_request(&did, seq_no).unwrap();
            let get_txn_response = LedgerUtils::submit_request(pool_handle, &get_txn_request).unwrap();

            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();

            let get_txn_schema_result: SchemaResult = serde_json::from_value(get_txn_response.result.data.unwrap()).unwrap();

            let expected_schema_data: SchemaData = serde_json::from_str(SCHEMA_DATA).unwrap();
            assert_eq!(expected_schema_data, get_txn_schema_result.data.unwrap());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works_for_invalid_seq_no() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, &SCHEMA_DATA).unwrap();
            let schema_response_str = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            let schema_response: Reply<SchemaResult> = serde_json::from_str(&schema_response_str).unwrap();

            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, GET_SCHEMA_DATA).unwrap();
            LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response_str).unwrap();

            let seq_no = schema_response.result.seq_no + 1;

            let get_txn_request = LedgerUtils::build_get_txn_request(&did, seq_no).unwrap();

            let get_txn_response = LedgerUtils::submit_request(pool_handle, &get_txn_request).unwrap();
            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();
            assert!(get_txn_response.result.data.is_none());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod pool_config {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_config_request_works() {
            TestUtils::cleanup_storage();

            let expected_result = r#""operation":{"type":"111","writes":true,"force":false"#;
            let request = LedgerUtils::build_pool_config_request(DID_TRUSTEE, true, false).unwrap();
            assert!(request.contains(expected_result));


            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_config_request_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let request = LedgerUtils::build_pool_config_request(&trustee_did, true, false).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_config_request_works_for_disabling_writing() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            // set Ledger as readonly
            let request = LedgerUtils::build_pool_config_request(&trustee_did, false, false).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            // try send schema request
            let schema_request = LedgerUtils::build_schema_request(&trustee_did, SCHEMA_DATA).unwrap();
            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &schema_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            // return Ledger to the previous state
            let request = LedgerUtils::build_pool_config_request(&trustee_did, true, false).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod pool_upgrade {
        use super::*;


        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_upgrade_request_works_for_start_action() {
            TestUtils::cleanup_storage();

            let expected_result = r#""operation":{"type":"109","name":"upgrade-libindy","version":"2.0.0","action":"start","sha256":"f284b","schedule":{},"reinstall":false,"force":false"#;
            let request = LedgerUtils::build_pool_upgrade_request(DID_TRUSTEE,
                                                                  "upgrade-libindy",
                                                                  "2.0.0",
                                                                  "start",
                                                                  "f284b",
                                                                  None,
                                                                  Some("{}"),
                                                                  None,
                                                                  false,
                                                                  false).unwrap();
            assert!(request.contains(expected_result));

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_upgrade_request_works_for_cancel_action() {
            TestUtils::cleanup_storage();

            let expected_result = r#""type":"109","name":"upgrade-libindy","version":"2.0.0","action":"cancel","sha256":"f284b","reinstall":false,"force":false"#;
            let request = LedgerUtils::build_pool_upgrade_request(DID_TRUSTEE,
                                                                  "upgrade-libindy",
                                                                  "2.0.0",
                                                                  "cancel",
                                                                  "f284b",
                                                                  None,
                                                                  None,
                                                                  None,
                                                                  false,
                                                                  false).unwrap();
            assert!(request.contains(expected_result));

            TestUtils::cleanup_storage();
        }

        lazy_static! {
            static ref SCHEDULE: String = {
                let next_year = time::now().tm_year + 1900 + 1;
                format!(r#"{{"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv":"{}-01-25T12:49:05.258870+00:00",
                             "8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb":"{}-01-25T13:49:05.258870+00:00",
                             "DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya":"{}-01-25T14:49:05.258870+00:00",
                             "4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA":"{}-01-25T15:49:05.258870+00:00"}}"#,
                             next_year, next_year, next_year, next_year)
            };
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_upgrade_request_works_for_start_cancel_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            //start
            let request = LedgerUtils::build_pool_upgrade_request(&trustee_did,
                                                                  "upgrade-libindy",
                                                                  "2.0.0",
                                                                  "start",
                                                                  "f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398",
                                                                  None,
                                                                  Some(&SCHEDULE),
                                                                  None,
                                                                  false,
                                                                  false).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            //cancel
            let request = LedgerUtils::build_pool_upgrade_request(&trustee_did,
                                                                  "upgrade-libindy",
                                                                  "2.0.0",
                                                                  "cancel",
                                                                  "ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398",
                                                                  None,
                                                                  None,
                                                                  Some("Upgrade is not required"),
                                                                  false,
                                                                  false).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

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

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&DID, &DID, None, None, None).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &DID, &nym_request);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works_for_invalid_message() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let res = LedgerUtils::submit_request(pool_handle, "request");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, "request");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_only_required_fields() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();

            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_with_option_fields() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let role = "STEWARD";
            let alias = "some_alias";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), Some(alias), Some(role)).unwrap();

            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_different_roles() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            for role in ["STEWARD", "TRUSTEE", "TRUST_ANCHOR"].iter() {
                let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();
                let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, Some(role)).unwrap();
                LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            }

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_wrong_role() {
            let role = "WRONG_ROLE";
            let res = LedgerUtils::build_nym_request(&IDENTIFIER, &DEST, None, None, Some(role));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_wrong_signer_role() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();
            let nym_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&trustee_did, &my_did).unwrap();
            LedgerUtils::submit_request_with_retries(pool_handle, &get_nym_request, &nym_req_resp).unwrap();

            let (my_did2, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();
            let nym_request = LedgerUtils::build_nym_request(&my_did, &my_did2, None, None, None).unwrap();
            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_unknown_signer_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_my_did(wallet_handle, r#"{"seed":"000000000000000000000000Trustee9"}"#).unwrap();
            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();
            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_nym_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_my_did(wallet_handle, r#"{"seed":"00000000000000000000000000000My3"}"#).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&did, &did).unwrap();

            let get_nym_response = LedgerUtils::submit_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_none());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_request_works_for_invalid_submitter_identifier() {
            let res = LedgerUtils::build_nym_request(INVALID_IDENTIFIER, IDENTIFIER, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_request_works_for_invalid_target_identifier() {
            let res = LedgerUtils::build_nym_request(IDENTIFIER, INVALID_IDENTIFIER, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_request_works_for_invalid_submitter_identifier() {
            let res = LedgerUtils::build_get_nym_request(INVALID_IDENTIFIER, IDENTIFIER);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_request_works_for_invalid_target_identifier() {
            let res = LedgerUtils::build_get_nym_request(IDENTIFIER, INVALID_IDENTIFIER);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works_for_reset_role() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let mut nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did,
                                                                 Some(&my_verkey), None, Some("TRUSTEE")).unwrap();
            let nym_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let mut get_nym_request = LedgerUtils::build_get_nym_request(&my_did, &my_did).unwrap();
            let get_nym_response_with_role = LedgerUtils::submit_request_with_retries(pool_handle, &get_nym_request, &nym_req_resp).unwrap();

            let get_nym_response_with_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_with_role).unwrap();
            let get_nym_response_data_with_role: GetNymResultData = serde_json::from_str(&get_nym_response_with_role.result.data.unwrap()).unwrap();

            nym_request = LedgerUtils::build_nym_request(&my_did, &my_did,
                                                         Some(&my_verkey), None, Some("")).unwrap();
            let nym_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();

            get_nym_request = LedgerUtils::build_get_nym_request(&my_did, &my_did).unwrap();
            let get_nym_response_without_role = LedgerUtils::submit_request_with_retries(pool_handle, &get_nym_request, &nym_req_resp).unwrap();

            let get_nym_response_without_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_without_role).unwrap();
            let get_nym_response_data_without_role: GetNymResultData = serde_json::from_str(&get_nym_response_without_role.result.data.unwrap()).unwrap();

            assert!(get_nym_response_data_without_role.role.is_none());
            assert_ne!(get_nym_response_data_without_role.role, get_nym_response_data_with_role.role);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&did, &did, None, Some(ATTRIB_RAW_DATA), None).unwrap();

            let response = LedgerUtils::submit_request(pool_handle, &attrib_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&did, &did, "endpoint").unwrap();
            let get_attrib_response = LedgerUtils::submit_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_attribute() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&did, &did, "some_attribute").unwrap();
            let get_attrib_response = LedgerUtils::submit_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_request_works_for_invalid_submitter_did() {
            let res = LedgerUtils::build_attrib_request(INVALID_IDENTIFIER, IDENTIFIER, None,
                                                        Some(ATTRIB_RAW_DATA), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_request_works_for_invalid_target_did() {
            let res = LedgerUtils::build_attrib_request(IDENTIFIER, INVALID_IDENTIFIER, None,
                                                        Some(ATTRIB_RAW_DATA), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_request_works_for_invalid_submitter_identifier() {
            let res = LedgerUtils::build_get_attrib_request(INVALID_IDENTIFIER, IDENTIFIER, "endpoint");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_request_works_for_invalid_target_identifier() {
            let res = LedgerUtils::build_get_attrib_request(IDENTIFIER, INVALID_IDENTIFIER, "endpoint");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod schemas_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_missed_field_in_data_json() {
            let data = r#"{"name":"name"}"#;

            let res = LedgerUtils::build_schema_request(IDENTIFIER, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_invalid_data_json_format() {
            let data = r#"{"name":"name", "keys":"name"}"#;

            let res = LedgerUtils::build_schema_request(IDENTIFIER, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_invalid_submitter_identifier() {
            let res = LedgerUtils::build_schema_request(INVALID_IDENTIFIER, SCHEMA_DATA);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_data_json() {
            let data = r#"{"name":"name"}"#;
            let res = LedgerUtils::build_get_schema_request(IDENTIFIER, IDENTIFIER, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_submitter_identifier() {
            let res = LedgerUtils::build_get_schema_request(INVALID_IDENTIFIER, DEST, GET_SCHEMA_DATA);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_dest() {
            let res = LedgerUtils::build_get_schema_request(IDENTIFIER, INVALID_IDENTIFIER, GET_SCHEMA_DATA);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let schema_request = LedgerUtils::build_schema_request(&did, SCHEMA_DATA).unwrap();

            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_schema_request_works_for_unknown_schema() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let get_schema_data = r#"{"name":"unknown_schema_name","version":"2.0"}"#;
            let get_schema_request = LedgerUtils::build_get_schema_request(&did, &did, get_schema_data).unwrap();

            let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
            // TODO FIXME restore after INDY-699 will be fixed
            // let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            // assert!(get_schema_response.result.data.is_none());
            assert!(serde_json::from_str::<Reply<GetSchemaReplyResult>>(&get_schema_response).unwrap_err().to_string().contains("missing field `attr_names`"));

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod node_requests {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_missed_fields_in_data_json() {
            let data = r#"{ }"#;
            let res = LedgerUtils::build_node_request(IDENTIFIER, DEST, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn indy_build_node_request_works_for_wrong_service() {
            let data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 1, "alias":"some", "services": ["SERVICE"], "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}"#;
            let res = LedgerUtils::build_node_request(IDENTIFIER, DEST, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_for_wrong_role() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let node_request = LedgerUtils::build_node_request(&did, &did, NODE_DATA).unwrap();

            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &node_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REJECT);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_node_request_works_for_steward_already_has_node() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(STEWARD_SEED)).unwrap();

            let node_request = LedgerUtils::build_node_request(&did, &did, NODE_DATA).unwrap();

            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &node_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REJECT);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        fn indy_build_claim_def_request_works_for_invalid_data_json() {
            TestUtils::cleanup_storage();

            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"}}}"#;

            let res = LedgerUtils::build_claim_def_txn(IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn indy_build_claim_def_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();

            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"},"rctxt":"1","z":"1"}}"#;

            let res = LedgerUtils::build_claim_def_txn(INVALID_IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, data);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn indy_build_get_claim_def_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();

            let res = LedgerUtils::build_get_claim_def_txn(INVALID_IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, IDENTIFIER);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn indy_build_get_claim_def_request_works_for_invalid_origin() {
            TestUtils::cleanup_storage();

            let res = LedgerUtils::build_get_claim_def_txn(IDENTIFIER, SEQ_NO, SIGNATURE_TYPE, INVALID_IDENTIFIER);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }
}
