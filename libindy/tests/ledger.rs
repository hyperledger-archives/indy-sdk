#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate hex;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate openssl;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;
extern crate sodiumoxide;
extern crate rand;

#[macro_use]
mod utils;

use self::indy::ErrorCode;
#[cfg(feature = "local_nodes_pool")]
use utils::{pool, ledger, did, anoncreds};
use utils::types::*;
use utils::constants::*;

use openssl::hash::{MessageDigest, Hasher};
use sodiumoxide::crypto::secretbox;
use self::rand::distributions::Alphanumeric;

use utils::domain::ledger::constants;
use utils::domain::ledger::request::DEFAULT_LIBIDY_DID;
use utils::domain::anoncreds::schema::{Schema, SchemaV1};
use utils::domain::anoncreds::credential_definition::CredentialDefinitionV1;
use utils::domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinitionV1;
use utils::domain::anoncreds::revocation_registry::RevocationRegistryV1;
use utils::domain::anoncreds::revocation_registry_delta::RevocationRegistryDeltaV1;

use std::collections::HashMap;
use std::thread;

use api::INVALID_WALLET_HANDLE;

mod high_cases {
    use super::*;

    mod requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_request_works_for_invalid_pool_handle() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_send_request_works_for_invalid_pool_handle");

            let res = ledger::submit_request(pool_handle + 1, REQUEST);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_request_works_for_invalid_pool_handle", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_pool_handle() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_sign_and_submit_request_works_for_invalid_pool_handle");

            let res = ledger::sign_and_submit_request(pool_handle + 1, wallet_handle, &trustee_did, REQUEST);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_and_submit_request_works_for_invalid_pool_handle", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_sign_and_submit_request_works_for_invalid_wallet_handle");

            let res = ledger::sign_and_submit_request(pool_handle, INVALID_WALLET_HANDLE, &trustee_did, REQUEST);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_and_submit_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works() {
            let pool_handle = utils::setup_with_pool("indy_submit_request_works");

            let request = r#"{
                        "reqId":1491566332010860,
                         "identifier":"Th7MpTaRZVRYnPiabds81Y",
                         "operation":{
                            "type":"105",
                            "dest":"Th7MpTaRZVRYnPiabds81Y"
                         },
                         "protocolVersion":2,
                         "signature":"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV"}"#;

            let resp = ledger::submit_request(pool_handle, request);
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

            utils::tear_down_with_pool(pool_handle, "indy_submit_request_works");
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_sign_and_submit_request_works");
            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &did, None, None, None).unwrap();
            let nym_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&nym_response, ResponseType::REPLY);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_and_submit_request_works", &wallet_config);
        }
    }

    mod submit_action {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_pool_restart() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_pool_restart");

            let pool_request_request = ledger::build_pool_restart_request(DID_TRUSTEE, "start", None).unwrap();
            let pool_request_request = ledger::sign_request(wallet_handle, &trustee_did, &pool_request_request).unwrap();
            ledger::submit_action(pool_handle, &pool_request_request, None, None).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_pool_restart", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_validator_info() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_validator_info");

            let get_validator_info_request = ledger::build_get_validator_info_request(&trustee_did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &trustee_did, &get_validator_info_request).unwrap();
            ledger::submit_action(pool_handle, &get_validator_info_request, None, None).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_validator_info", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_not_supported_request_type() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_not_supported_request_type");

            let get_nym_request = ledger::build_get_nym_request(Some(&trustee_did), &trustee_did).unwrap();
            let res = ledger::submit_action(pool_handle, &get_nym_request, None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_not_supported_request_type", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_pool_restart_for_invalid_pool_handle() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_pool_restart_for_invalid_pool_handle");

            let get_validator_info_request = ledger::build_get_validator_info_request(&trustee_did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &trustee_did, &get_validator_info_request).unwrap();

            let res = ledger::submit_action(pool_handle + 1, &get_validator_info_request, None, None);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_pool_restart_for_invalid_pool_handle", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_list_nodes() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_list_nodes");

            let get_validator_info_request = ledger::build_get_validator_info_request(&trustee_did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &trustee_did, &get_validator_info_request).unwrap();

            let nodes = r#"["Node1", "Node2"]"#;
            let response = ledger::submit_action(pool_handle, &get_validator_info_request, Some(nodes), None).unwrap();
            let responses: HashMap<String, serde_json::Value> = serde_json::from_str(&response).unwrap();

            assert_eq!(2, responses.len());
            assert!(responses.contains_key("Node1"));
            assert!(responses.contains_key("Node2"));

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_list_nodes", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_timeout() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_timeout");

            let get_validator_info_request = ledger::build_get_validator_info_request(&trustee_did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &trustee_did, &get_validator_info_request).unwrap();
            ledger::submit_action(pool_handle, &get_validator_info_request, None, Some(100)).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_timeout", &wallet_config);
        }
    }

    mod sign_request {
        use super::*;

        #[test]
        fn indy_sign_request_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_sign_request_works");

            let request = ledger::sign_request(wallet_handle, &trustee_did, REQUEST).unwrap();
            let request: serde_json::Value = serde_json::from_str(&request).unwrap();
            assert_eq!(request["signature"].as_str().unwrap(), "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW");

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_request_works", &wallet_config);
        }

        #[test]
        fn indy_sign_works_for_unknown_signer() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_sign_works_for_unknown_signer");

            let res = ledger::sign_request(wallet_handle, DID, REQUEST);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_sign_works_for_unknown_signer", &wallet_config);
        }

        #[test]
        fn indy_sign_request_works_for_invalid_message_format() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_sign_request_works_for_invalid_message_format");

            let res = ledger::sign_request(wallet_handle, &my_did, "1495034346617224651");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_sign_request_works_for_invalid_message_format", &wallet_config);
        }

        #[test]
        fn indy_sign_request_works_for_invalid_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_sign_request_works_for_invalid_handle");

            let res = ledger::sign_request(INVALID_WALLET_HANDLE, &my_did, MESSAGE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_sign_request_works_for_invalid_handle", &wallet_config);
        }
    }

    mod multi_sign_request {
        use super::*;

        #[test]
        fn indy_multi_sign_request_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_multi_sign_request_works");

            let (did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did2, _) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let message = ledger::multi_sign_request(wallet_handle, &did, REQUEST).unwrap();
            let message = ledger::multi_sign_request(wallet_handle, &did2, &message).unwrap();

            let msg: serde_json::Value = serde_json::from_str(&message).unwrap();
            let signatures = msg["signatures"].as_object().unwrap();

            assert_eq!(signatures[DID_TRUSTEE], r#"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"#);
            assert_eq!(signatures[DID_MY1], r#"49aXkbrtTE3e522AefE76J51WzUiakw3ZbxxWzf44cv7RS21n8mMr4vJzi4TymuqDupzCz7wEtuGz6rA94Y73kKR"#);

            utils::tear_down_with_wallet(wallet_handle, "indy_multi_sign_request_works", &wallet_config);
        }

        #[test]
        fn indy_multi_sign_request_works_for_start_from_single_signature() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_multi_sign_request_works_for_start_from_single_signature");

            let (did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did2, _) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let message = ledger::sign_request(wallet_handle, &did, REQUEST_FROM_TRUSTEE).unwrap();
            let message = ledger::multi_sign_request(wallet_handle, &did2, &message).unwrap();

            let msg: serde_json::Value = serde_json::from_str(&message).unwrap();
            let signatures = msg["signatures"].as_object().unwrap();

            assert!(!msg.as_object().unwrap().contains_key("signature"));
            assert_eq!(signatures[DID_TRUSTEE], r#"3YnLxoUd4utFLzeXUkeGefAqAdHUD7rBprpSx2CJeH7gRYnyjkgJi7tCnFgUiMo62k6M2AyUDtJrkUSgHfcq3vua"#);
            assert_eq!(signatures[DID_MY1], r#"4EyvSFPoeQCJLziGVqjuMxrbuoWjAWUGPd6LdxeZuG9w3Bcbt7cSvhjrv8SX5e8mGf8jrf3K6xd9kEhXsQLqUg45"#);

            utils::tear_down_with_wallet(wallet_handle, "indy_multi_sign_request_works_for_start_from_single_signature", &wallet_config);
        }

        #[test]
        fn indy_multi_sign_request_works_for_unknown_signer() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_multi_sign_request_works_for_unknown_signer");

            let res = ledger::multi_sign_request(wallet_handle, DID, REQUEST);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_multi_sign_request_works_for_unknown_signer", &wallet_config);
        }

        #[test]
        fn indy_multi_sign_request_works_for_invalid_message_format() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_multi_sign_request_works_for_invalid_message_format");

            let res = ledger::multi_sign_request(wallet_handle, &my_did, "1495034346617224651");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_multi_sign_request_works_for_invalid_message_format", &wallet_config);
        }

        #[test]
        fn indy_multi_sign_request_works_for_twice_use_same_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_multi_sign_request_works_for_twice_use_same_did");

            let (did, _) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let message = ledger::multi_sign_request(wallet_handle, &did, REQUEST).unwrap();
            let message = ledger::multi_sign_request(wallet_handle, &did, &message).unwrap();
            let msg: serde_json::Value = serde_json::from_str(&message).unwrap();
            let signatures = msg["signatures"].as_object().unwrap();

            assert_eq!(1, signatures.len());
            assert_eq!(signatures[DID_MY1], r#"49aXkbrtTE3e522AefE76J51WzUiakw3ZbxxWzf44cv7RS21n8mMr4vJzi4TymuqDupzCz7wEtuGz6rA94Y73kKR"#);

            utils::tear_down_with_wallet(wallet_handle, "indy_multi_sign_request_works_for_twice_use_same_did", &wallet_config);
        }
    }


    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_only_required_fields() {
            let expected_result = json!({
                "type": constants::NYM,
                "dest": DEST,
            });

            let request = ledger::build_nym_request(&IDENTIFIER, &DEST, None, None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_with_option_fields() {
            let role = "STEWARD";
            let alias = "some_alias";

            let expected_result = json!({
                "alias": alias,
                "dest": DEST,
                "role": "2",
                "type": constants::NYM,
                "verkey": VERKEY_TRUSTEE
            });

            let request = ledger::build_nym_request(&IDENTIFIER, &DEST, Some(VERKEY_TRUSTEE), Some(alias), Some(role)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_empty_role() {
            let expected_result = json!({
                "dest": DEST,
                "role": serde_json::Value::Null,
                "type": constants::NYM
            });

            let request = ledger::build_nym_request(&IDENTIFIER, &DEST, None, None, Some("")).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_requests_works() {
            let expected_result = json!({
                "type": constants::GET_NYM,
                "dest": DEST
            });

            let request = ledger::build_get_nym_request(Some(IDENTIFIER), &DEST).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_requests_works_for_default_submitter_did() {
            let request = ledger::build_get_nym_request(None, &DEST).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_without_signature() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_nym_request_works_without_signature");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_request = ledger::build_nym_request(&trustee_did, &did, None, None, None).unwrap();
            let response = ledger::submit_request(pool_handle, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_nym_request_works_without_signature", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_get_nym_request_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_send_get_nym_request_works");

            let get_nym_request = ledger::build_get_nym_request(Some(&trustee_did), &trustee_did).unwrap();
            let get_nym_response = ledger::submit_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_get_nym_request_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_get_nym_request_works_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_send_get_nym_request_works_default_submitter_did");

            let get_nym_request = ledger::build_get_nym_request(None, DID_TRUSTEE).unwrap();
            let get_nym_response = ledger::submit_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_get_nym_request_works_default_submitter_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_nym_requests_works");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REPLY);

            let get_nym_request = ledger::build_get_nym_request(Some(&my_did), &my_did).unwrap();
            let get_nym_response = ledger::submit_request_with_retries(pool_handle, &get_nym_request, &nym_resp).unwrap();

            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_nym_requests_works", &wallet_config);
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_raw_value() {
            let expected_result = json!({
                "type": constants::ATTRIB,
                "dest": DEST,
                "raw": ATTRIB_RAW_DATA
            });

            let request = ledger::build_attrib_request(&IDENTIFIER, &DEST, None, Some(ATTRIB_RAW_DATA), None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_hash_value() {
            let expected_result = json!({
                "type": constants::ATTRIB,
                "dest": DEST,
                "hash": ATTRIB_HASH_DATA
            });

            let request = ledger::build_attrib_request(&IDENTIFIER, &DEST, Some(ATTRIB_HASH_DATA), None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_enc_value() {
            let expected_result = json!({
                "type": constants::ATTRIB,
                "dest": DEST,
                "enc": ATTRIB_ENC_DATA
            });

            let request = ledger::build_attrib_request(&IDENTIFIER, &DEST, None, None, Some(ATTRIB_ENC_DATA)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_requests_works_for_missed_attribute() {
            let res = ledger::build_attrib_request(&IDENTIFIER, &DEST, None, None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works_for_raw_value() {
            let raw = "endpoint";

            let expected_result = json!({
                "type": constants::GET_ATTR,
                "dest": DEST,
                "raw": raw
            });

            let request = ledger::build_get_attrib_request(Some(IDENTIFIER), &DEST, Some(raw), None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works_for_hash_value() {
            let expected_result = json!({
                "type": constants::GET_ATTR,
                "dest": DEST,
                "hash": ATTRIB_HASH_DATA
            });

            let request = ledger::build_get_attrib_request(Some(IDENTIFIER), &DEST, None, Some(ATTRIB_HASH_DATA), None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works_for_enc_value() {
            let expected_result = json!({
                "type": constants::GET_ATTR,
                "dest": DEST,
                "enc": ATTRIB_ENC_DATA
            });

            let request = ledger::build_get_attrib_request(Some(IDENTIFIER), &DEST, None, None, Some(ATTRIB_ENC_DATA)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_requests_works_for_default_submitter_did() {
            let request = ledger::build_get_attrib_request(None, &DEST, Some(ATTRIB_RAW_DATA), None, None).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_without_signature() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_attrib_request_works_without_signature");

            let attrib_request = ledger::build_attrib_request(&trustee_did, &trustee_did, None, Some(ATTRIB_RAW_DATA), None).unwrap();
            let response = ledger::submit_request(pool_handle, &attrib_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_attrib_request_works_without_signature", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_raw_value() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_attrib_requests_works_for_raw_value");

            let attrib_request = ledger::build_attrib_request(&did,
                                                              &did,
                                                              None,
                                                              Some(ATTRIB_RAW_DATA),
                                                              None).unwrap();
            let attrib_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &attrib_request).unwrap();
            pool::check_response_type(&attrib_req_resp, ResponseType::REPLY);

            let get_attrib_request = ledger::build_get_attrib_request(Some(&did), &did, Some("endpoint"), None, None).unwrap();
            let get_attrib_response = ledger::submit_request_with_retries(pool_handle, &get_attrib_request, &attrib_req_resp).unwrap();

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert_eq!(get_attrib_response.result.data.unwrap().as_str(), ATTRIB_RAW_DATA);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_attrib_requests_works_for_raw_value", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_hash_value() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_attrib_requests_works_for_hash_value");

            let mut ctx = Hasher::new(MessageDigest::sha256()).unwrap();
            ctx.update(&ATTRIB_RAW_DATA.as_bytes()).unwrap();
            let hashed_attr = hex::encode(ctx.finish().unwrap().as_ref());

            let attrib_request = ledger::build_attrib_request(&did,
                                                              &did,
                                                              Some(&hashed_attr),
                                                              None,
                                                              None).unwrap();
            let attrib_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &attrib_request).unwrap();
            pool::check_response_type(&attrib_req_resp, ResponseType::REPLY);

            let get_attrib_request = ledger::build_get_attrib_request(Some(&did), &did, None, Some(&hashed_attr), None).unwrap();
            let get_attrib_response = ledger::submit_request_with_retries(pool_handle, &get_attrib_request, &attrib_req_resp).unwrap();

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert_eq!(get_attrib_response.result.data.unwrap().as_str(), hashed_attr.as_str());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_attrib_requests_works_for_hash_value", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_requests_works_for_encrypted_value() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_attrib_requests_works_for_encrypted_value");

            let key = secretbox::gen_key();
            let nonce = secretbox::gen_nonce();
            let encryted_attr = hex::encode(secretbox::seal(&ATTRIB_RAW_DATA.as_bytes(), &nonce, &key));

            let attrib_request = ledger::build_attrib_request(&did,
                                                              &did,
                                                              None,
                                                              None,
                                                              Some(&encryted_attr)).unwrap();
            let attrib_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &attrib_request).unwrap();
            pool::check_response_type(&attrib_req_resp, ResponseType::REPLY);

            let get_attrib_request = ledger::build_get_attrib_request(Some(&did), &did, None, None, Some(&encryted_attr)).unwrap();
            let get_attrib_response = ledger::submit_request_with_retries(pool_handle, &get_attrib_request, &attrib_req_resp).unwrap();

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert_eq!(get_attrib_response.result.data.unwrap().as_str(), encryted_attr.as_str());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_attrib_requests_works_for_encrypted_value", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_requests_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_get_attrib_requests_works_for_default_submitter_did");

            let attrib_request = ledger::build_attrib_request(&did,
                                                              &did,
                                                              None,
                                                              Some(ATTRIB_RAW_DATA),
                                                              None).unwrap();
            let attrib_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &attrib_request).unwrap();
            pool::check_response_type(&attrib_req_resp, ResponseType::REPLY);

            let get_attrib_request = ledger::build_get_attrib_request(None, &did, Some("endpoint"), None, None).unwrap();
            let get_attrib_response = ledger::submit_request_with_retries(pool_handle, &get_attrib_request, &attrib_req_resp).unwrap();

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert_eq!(get_attrib_response.result.data.unwrap().as_str(), ATTRIB_RAW_DATA);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_attrib_requests_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod schema_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_correct_data_json() {
            let expected_result = json!({
                "type": constants::SCHEMA,
                "data": {
                    "name": GVT_SCHEMA_NAME,
                    "version": SCHEMA_VERSION,
                    "attr_names": ["name"]
                },
            });

            let request = ledger::build_schema_request(IDENTIFIER, SCHEMA_DATA).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_correct_data_json() {
            let expected_result = json!({
                "type": constants::GET_SCHEMA,
                "dest": ISSUER_DID,
                "data": {
                    "name": GVT_SCHEMA_NAME,
                    "version": SCHEMA_VERSION
                },
            });

            let request = ledger::build_get_schema_request(Some(IDENTIFIER), &anoncreds::gvt_schema_id()).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_default_submitter_did() {
            let request = ledger::build_get_schema_request(None, &anoncreds::gvt_schema_id()).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_without_signature() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_schema_request_works_without_signature");

            let schema_request = ledger::build_schema_request(&DID_TRUSTEE, SCHEMA_DATA).unwrap();
            let response = ledger::submit_request(pool_handle, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_schema_request_works_without_signature", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_requests_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_schema_requests_works");

            let (schema_id, _, _) = ledger::post_entities();

            let get_schema_request = ledger::build_get_schema_request(Some(DID_MY1), &schema_id).unwrap();
            let get_schema_response = ledger::submit_request(pool_handle, &get_schema_request).unwrap();
            let (_, schema_json) = ledger::parse_get_schema_response(&get_schema_response).unwrap();

            let _schema: SchemaV1 = serde_json::from_str(&schema_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_schema_requests_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_schema_requests_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_requests_works_for_default_submitter_did");

            let (schema_id, _, _) = ledger::post_entities();

            let get_schema_request = ledger::build_get_schema_request(None, &schema_id).unwrap();
            let get_schema_response = ledger::submit_request(pool_handle, &get_schema_request).unwrap();
            let (_, schema_json) = ledger::parse_get_schema_response(&get_schema_response).unwrap();

            let _schema: SchemaV1 = serde_json::from_str(&schema_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_requests_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod node_request {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_correct_data_json() {
            let expected_result = json!({
                "type": constants::NODE,
                "dest": DEST,
                "data": {
                    "node_ip": "10.0.0.100",
                    "node_port": 2,
                    "client_ip": "10.0.0.100",
                    "client_port": 1,
                    "alias": "Node5",
                    "services": ["VALIDATOR"],
                    "blskey": "4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba",
                    "blskey_pop": "RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1",
                },
            });

            let request = ledger::build_node_request(IDENTIFIER, DEST, NODE_DATA).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_without_signature() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_steward("indy_send_node_request_works_without_signature");

            let node_request = ledger::build_node_request(&did, &did, NODE_DATA).unwrap();
            let response = ledger::submit_request(pool_handle, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_node_request_works_without_signature", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        #[ignore] //FIXME currently unstable pool behaviour after new non-existing node was added
        fn indy_submit_node_request_works_for_new_steward() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_submit_node_request_works_for_new_steward");

            let (my_did, _) = did::create_store_and_publish_my_did_from_steward(wallet_handle, pool_handle).unwrap();

            let dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y"; // random(32) and base58

            let node_request = ledger::build_node_request(&my_did, dest, NODE_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_node_request_works_for_new_steward", &wallet_config);
        }
    }

    mod cred_def_requests {
        use super::*;

        #[test]
        fn indy_build_cred_def_request_works_for_correct_data_json() {
            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let cred_def_json = json!({
               "ver":"1.0",
               "id":"cred_def_id",
               "schemaId":"1",
               "type":"CL",
               "tag":"TAG_1",
               "value":{
                  "primary":{
                     "n":"1",
                     "s":"2",
                     "r":{"name":"1","master_secret":"3"},
                     "rctxt":"1",
                     "z":"1"
                  }
               }
            }).to_string();

            let expected_result = json!({
               "ref":1,
               "type":"102",
               "signature_type":"CL",
               "tag":"TAG_1",
               "data":{
                  "primary":{
                     "n":"1",
                     "s":"2",
                     "r":{"name":"1","master_secret":"3"},
                     "rctxt":"1",
                     "z":"1"
                  }
               }
            });

            let request = ledger::build_cred_def_txn(IDENTIFIER, &cred_def_json).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_cred_def_request_works() {
            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let expected_result = json!({
                "type": constants::GET_CRED_DEF,
                "ref": SEQ_NO,
                "signature_type": SIGNATURE_TYPE,
                "origin": IDENTIFIER,
                "tag": TAG_1
            });

            let id = anoncreds::cred_def_id(IDENTIFIER, &SEQ_NO.to_string(), SIGNATURE_TYPE, TAG_1);
            let request = ledger::build_get_cred_def_request(Some(IDENTIFIER), &id).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_cred_def_request_works_for_default_submitter_did() {
            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let id = anoncreds::cred_def_id(IDENTIFIER, &SEQ_NO.to_string(), SIGNATURE_TYPE, TAG_1);
            let request = ledger::build_get_cred_def_request(None, &id).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_cred_def_request_works_without_signature() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_cred_def_request_works_without_signature");

            let (did, _) = did::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let cred_def_request = ledger::build_cred_def_txn(&did, &anoncreds::credential_def_json()).unwrap();
            let response = ledger::submit_request(pool_handle, &cred_def_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_cred_def_request_works_without_signature", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_cred_def_requests_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_cred_def_requests_works");

            let (_, cred_def_id, _) = ledger::post_entities();

            let get_cred_def_request = ledger::build_get_cred_def_request(Some(DID_MY1), &cred_def_id).unwrap();
            let get_cred_def_response = ledger::submit_request(pool_handle, &get_cred_def_request).unwrap();
            let (_, cred_def_json) = ledger::parse_get_cred_def_response(&get_cred_def_response).unwrap();

            let _cred_def: CredentialDefinitionV1 = serde_json::from_str(&cred_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_cred_def_requests_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_cred_def_requests_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_requests_works_for_default_submitter_did");

            let (_, cred_def_id, _) = ledger::post_entities();

            let get_cred_def_request = ledger::build_get_cred_def_request(None, &cred_def_id).unwrap();
            let get_cred_def_response = ledger::submit_request(pool_handle, &get_cred_def_request).unwrap();
            let (_, cred_def_json) = ledger::parse_get_cred_def_response(&get_cred_def_response).unwrap();

            let _cred_def: CredentialDefinitionV1 = serde_json::from_str(&cred_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_requests_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod get_validator_info {
        use super::*;

        #[test]
        fn indy_build_get_validator_info_request() {
            let expected_result = json!({
                "type": constants::GET_VALIDATOR_INFO,
            });

            let request = ledger::build_get_validator_info_request(IDENTIFIER).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_validator_info_request_works() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_trustee("indy_get_validator_info_request_works");

            let get_validator_info_request = ledger::build_get_validator_info_request(&did).unwrap();
            let get_validator_info_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &get_validator_info_request).unwrap();

            let get_validator_info_response: HashMap<String, String> = serde_json::from_str(&get_validator_info_response).unwrap();
            for value in get_validator_info_response.values() {
                serde_json::from_str::<Reply<GetValidatorInfoResult>>(value).unwrap();
            }

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_validator_info_request_works", &wallet_config);
        }
    }

    mod get_txn_requests {
        use super::*;

        #[test]
        fn indy_build_get_txn_request() {
            let expected_result = json!({
                "type": constants::GET_TXN,
                "data": SEQ_NO,
                "ledgerId": 1
            });

            let request = ledger::build_get_txn_request(Some(IDENTIFIER), SEQ_NO, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_request_for_default_submitter_did() {
            let request = ledger::build_get_txn_request(None, SEQ_NO, None).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        fn indy_build_get_txn_request_for_ledger_type_as_number() {
            let expected_result = json!({
                "type": constants::GET_TXN,
                "data": SEQ_NO,
                "ledgerId": 10
            });

            let request = ledger::build_get_txn_request(Some(IDENTIFIER), SEQ_NO, Some("10")).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_request_for_ledger_type() {
            let expected_result = json!({
                "type": constants::GET_TXN,
                "data": SEQ_NO,
                "ledgerId": 0
            });

            let request = ledger::build_get_txn_request(Some(IDENTIFIER), SEQ_NO, Some("POOL")).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_get_txn_request_works");

            let schema_request = ledger::build_schema_request(&did, &anoncreds::gvt_schema_json()).unwrap();
            let schema_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            pool::check_response_type(&schema_response, ResponseType::REPLY);

            let seq_no = ledger::extract_seq_no_from_reply(&schema_response).unwrap() as i32;

            thread::sleep(std::time::Duration::from_secs(3));

            let get_txn_request = ledger::build_get_txn_request(Some(&did), seq_no, None).unwrap();
            let get_txn_response = ledger::submit_request(pool_handle, &get_txn_request).unwrap();

            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();
            let get_txn_schema_data: SchemaData = serde_json::from_value(
                serde_json::Value::Object(get_txn_response.result.data.unwrap()["txn"]["data"]["data"].as_object().unwrap().clone())
            ).unwrap();

            let expected_schema_data: SchemaData = serde_json::from_str(r#"{"name":"gvt","version":"1.0","attr_names":["name", "age", "sex", "height"]}"#).unwrap();
            assert_eq!(expected_schema_data, get_txn_schema_data);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_txn_request_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_txn_request_works_for_invalid_seq_no() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_get_txn_request_works_for_invalid_seq_no");

            let schema_request = ledger::build_schema_request(&did, &anoncreds::gvt_schema_json()).unwrap();
            let schema_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            pool::check_response_type(&schema_response, ResponseType::REPLY);

            let seq_no = ledger::extract_seq_no_from_reply(&schema_response).unwrap() as i32;
            let seq_no = seq_no + 1;

            thread::sleep(std::time::Duration::from_secs(3));

            let get_txn_request = ledger::build_get_txn_request(Some(&did), seq_no, None).unwrap();

            let get_txn_response = ledger::submit_request(pool_handle, &get_txn_request).unwrap();
            let get_txn_response: Reply<GetTxnResult> = serde_json::from_str(&get_txn_response).unwrap();
            assert!(get_txn_response.result.data.is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_txn_request_works_for_invalid_seq_no", &wallet_config);
        }
    }

    mod pool_config {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_config_request_works() {
            let expected_result = json!({
                "type": constants::POOL_CONFIG,
                "writes": true,
                "force": false
            });

            let request = ledger::build_pool_config_request(DID_TRUSTEE, true, false).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_config_request_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_pool_config_request_works");

            let request = ledger::build_pool_config_request(&trustee_did, true, false).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_pool_config_request_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_config_request_works_for_disabling_writing() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_pool_config_request_works_for_disabling_writing");

            // set Ledger as readonly
            let request = ledger::build_pool_config_request(&trustee_did, false, false).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            // try send schema request
            let schema_request = ledger::build_schema_request(&trustee_did, &anoncreds::gvt_schema_json()).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            // return Ledger to the previous state
            let request = ledger::build_pool_config_request(&trustee_did, true, false).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_pool_config_request_works_for_disabling_writing", &wallet_config);
        }
    }

    mod pool_restart {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_restart_request_works_for_start_action() {
            let expected_result = json!({
                "type": constants::POOL_RESTART,
                "action": "start",
                "datetime": "0"
            });

            let request = ledger::build_pool_restart_request(DID_TRUSTEE, "start", Some("0")).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_restart_request_works_for_cancel_action() {
            let expected_result = json!({
                "type": constants::POOL_RESTART,
                "action": "cancel"
            });

            let request = ledger::build_pool_restart_request(DID_TRUSTEE, "cancel", None).unwrap();
            check_request(&request, expected_result);
        }

        lazy_static! {
            static ref DATETIME: String = {
                let next_year = time::now().tm_year + 1900 + 1;
                format!("{}-01-25T12:49:05.258870+00:00", next_year)
            };
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_pool_restart_request_works_for_start_cancel_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_pool_restart_request_works_for_start_cancel_works");

            //start
            let request = ledger::build_pool_restart_request(&trustee_did, "start", Some(&DATETIME)).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            //cancel
            let request = ledger::build_pool_restart_request(&trustee_did, "cancel", None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_pool_restart_request_works_for_start_cancel_works", &wallet_config);
        }
    }

    mod pool_upgrade {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_upgrade_request_works_for_start_action() {
            let expected_result = json!({
                "type": constants::POOL_UPGRADE,
                "name": "upgrade-libindy",
                "version": "2.0.0",
                "action": "start",
                "sha256": "f284b",
                "schedule": {},
                "reinstall": false,
                "force": false
            });

            let request = ledger::build_pool_upgrade_request(DID_TRUSTEE,
                                                             "upgrade-libindy",
                                                             "2.0.0",
                                                             "start",
                                                             "f284b",
                                                             None,
                                                             Some("{}"),
                                                             None,
                                                             false,
                                                             false,
                                                             None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_upgrade_request_works_for_cancel_action() {
            let expected_result = json!({
                "type": constants::POOL_UPGRADE,
                "name": "upgrade-libindy",
                "version": "2.0.0",
                "action": "cancel",
                "sha256": "f284b",
                "reinstall": false,
                "force": false
            });

            let request = ledger::build_pool_upgrade_request(DID_TRUSTEE,
                                                             "upgrade-libindy",
                                                             "2.0.0",
                                                             "cancel",
                                                             "f284b",
                                                             None,
                                                             None,
                                                             None,
                                                             false,
                                                             false,
                                                             None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_pool_upgrade_request_works_for_package() {
            let expected_result = json!({
                "type": constants::POOL_UPGRADE,
                "name": "upgrade-libindy",
                "version": "2.0.0",
                "action": "start",
                "sha256": "f284b",
                "schedule": {},
                "reinstall": false,
                "force": false,
                "package": "some_package"
            });

            let request = ledger::build_pool_upgrade_request(DID_TRUSTEE,
                                                             "upgrade-libindy",
                                                             "2.0.0",
                                                             "start",
                                                             "f284b",
                                                             None,
                                                             Some("{}"),
                                                             None,
                                                             false,
                                                             false,
                                                             Some("some_package")).unwrap();
            check_request(&request, expected_result);
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
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_pool_upgrade_request_works_for_start_cancel_works");

            //start
            let request = ledger::build_pool_upgrade_request(&trustee_did,
                                                             "upgrade-libindy",
                                                             "2.0.0",
                                                             "start",
                                                             "f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398",
                                                             None,
                                                             Some(&SCHEDULE),
                                                             None,
                                                             false,
                                                             false,
                                                             None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            //cancel
            let request = ledger::build_pool_upgrade_request(&trustee_did,
                                                             "upgrade-libindy",
                                                             "2.0.0",
                                                             "cancel",
                                                             "ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398",
                                                             None,
                                                             None,
                                                             Some("Upgrade is not required"),
                                                             false,
                                                             false,
                                                             None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_pool_upgrade_request_works_for_start_cancel_works", &wallet_config);
        }
    }

    mod revoc_reg_def_requests {
        use super::*;

        #[test]
        #[cfg(all(feature = "local_nodes_pool", target_pointer_width = "64"))] //FIXME: fix AMCL hex serializing
        fn indy_build_revoc_reg_def_request() {
            let data = json!({
                "ver": "1.0",
                "id": "RevocRegID",
                "revocDefType": REVOC_REG_TYPE,
                "tag": TAG_1,
                "credDefId": "CredDefID",
                "value": json!({
                    "issuanceType":"ISSUANCE_ON_DEMAND",
                    "maxCredNum":5,
                    "tailsHash":"s",
                    "tailsLocation":"http://tails.location.com",
                    "publicKeys": json!({
                        "accumKey": json!({
                            "z": "1 0000000000000000000000000000000000000000000000000000000000001111 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"
                        })
                    })
                })
            }).to_string();

            let expected_result = r#""operation":{"type":"113","id":"RevocRegID","revocDefType":"CL_ACCUM","tag":"TAG_1","credDefId":"CredDefID","value":{"issuanceType":"ISSUANCE_ON_DEMAND","maxCredNum":5,"publicKeys":{"accumKey":{"z":"1 0000000000000000000000000000000000000000000000000000000000001111 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"}},"tailsHash":"s","tailsLocation":"http://tails.location.com"}}"#;

            let request = ledger::build_revoc_reg_def_request(DID, &data).unwrap();
            assert!(request.contains(expected_result));

            utils::tear_down("indy_build_revoc_reg_def_request");
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_def_request() {
            let expected_result = json!({
                "type": constants::GET_REVOC_REG_DEF,
                "id": "RevocRegID"
            });

            let request = ledger::build_get_revoc_reg_def_request(Some(DID), "RevocRegID").unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_def_request_for_default_submitter_did() {
            let request = ledger::build_get_revoc_reg_def_request(None, "RevocRegID").unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_revoc_reg_def_requests_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_revoc_reg_def_requests_works");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let get_rev_reg_def_request = ledger::build_get_revoc_reg_def_request(Some(DID_MY1), &rev_reg_id).unwrap();
            let get_rev_reg_def_response = ledger::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();

            let (_, revoc_reg_def_json) = ledger::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();
            let _revoc_reg_def: RevocationRegistryDefinitionV1 = serde_json::from_str(&revoc_reg_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_revoc_reg_def_requests_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_revoc_get_reg_def_requests_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_revoc_get_reg_def_requests_works_for_default_submitter_did");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let get_rev_reg_def_request = ledger::build_get_revoc_reg_def_request(None, &rev_reg_id).unwrap();
            let get_rev_reg_def_response = ledger::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();

            let (_, revoc_reg_def_json) = ledger::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();
            let _revoc_reg_def: RevocationRegistryDefinitionV1 = serde_json::from_str(&revoc_reg_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_revoc_get_reg_def_requests_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod revoc_reg_entry_request {
        use super::*;

        #[test]
        fn indy_build_revoc_reg_entry_request() {
            let expected_result = json!({
                "type": constants::REVOC_REG_ENTRY,
                "revocRegDefId": "RevocRegID",
                "revocDefType": "CL_ACCUM",
                "value": {
                    "accum": "1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"
                }
            });

            let rev_reg_entry_value = r#"{"value":{"accum":"1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"}, "ver":"1.0"}"#;

            let request = ledger::build_revoc_reg_entry_request(DID, "RevocRegID", REVOC_REG_TYPE, rev_reg_entry_value).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_revoc_reg_entry_requests_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_revoc_reg_entry_requests_works");

            ledger::post_entities();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_revoc_reg_entry_requests_works", &wallet_config);
        }
    }

    mod get_revoc_reg_request {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_request() {
            let expected_result = json!({
                "type": constants::GET_REVOC_REG,
                "revocRegDefId": "RevRegId",
                "timestamp": 100
            });

            let request = ledger::build_get_revoc_reg_request(Some(DID), "RevRegId", 100).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_request_for_default_submitter_did() {
            let request = ledger::build_get_revoc_reg_request(None, "RevRegId", 100).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_revoc_reg_request_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_revoc_reg_request_works");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let timestamp = time::get_time().sec as u64 + 1000;

            let get_rev_reg_req = ledger::build_get_revoc_reg_request(Some(DID_MY1), &rev_reg_id, timestamp).unwrap();
            let get_rev_reg_resp = ledger::submit_request(pool_handle, &get_rev_reg_req).unwrap();

            let (_, revoc_reg_json, _) = ledger::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();
            let _revoc_reg: RevocationRegistryV1 = serde_json::from_str(&revoc_reg_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_revoc_reg_request_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_revoc_reg_request_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_revoc_reg_request_works_for_default_submitter_did");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let timestamp = time::get_time().sec as u64 + 1000;

            let get_rev_reg_req = ledger::build_get_revoc_reg_request(None, &rev_reg_id, timestamp).unwrap();
            let get_rev_reg_resp = ledger::submit_request(pool_handle, &get_rev_reg_req).unwrap();

            let (_, revoc_reg_json, _) = ledger::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();
            let _revoc_reg: RevocationRegistryV1 = serde_json::from_str(&revoc_reg_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_revoc_reg_request_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod get_revoc_reg_delta_request {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_delta_request() {
            let expected_result = json!({
                "type": constants::GET_REVOC_REG_DELTA,
                "revocRegDefId": "RevRegId",
                "to": 100
            });

            let request = ledger::build_get_revoc_reg_delta_request(Some(DID), "RevRegId", None, 100).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_revoc_reg_delta_request_for_default_submitter_did() {
            let request = ledger::build_get_revoc_reg_delta_request(None, "RevRegId", None, 100).unwrap();
            check_default_identifier(&request);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_revoc_reg_delta_request_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_revoc_reg_delta_request_works");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let to = time::get_time().sec as u64 + 300;
            let get_rev_reg_delta_req = ledger::build_get_revoc_reg_delta_request(Some(DID_MY1), &rev_reg_id, None, to).unwrap();
            let get_rev_reg_delta_resp = ledger::submit_request(pool_handle, &get_rev_reg_delta_req).unwrap();

            let (_, revoc_reg_delta_json, _) = ledger::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_resp).unwrap();

            let _revoc_reg_delta: RevocationRegistryDeltaV1 = serde_json::from_str(&revoc_reg_delta_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_revoc_reg_delta_request_works", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_revoc_reg_delta_request_works_for_two_timestamps() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_revoc_reg_delta_request_works_for_two_timestamps");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let from = time::get_time().sec as u64;
            let to = time::get_time().sec as u64 + 300;
            let get_rev_reg_delta_req = ledger::build_get_revoc_reg_delta_request(Some(DID_MY1), &rev_reg_id, Some(from), to).unwrap();
            let get_rev_reg_delta_resp = ledger::submit_request(pool_handle, &get_rev_reg_delta_req).unwrap();

            let (_, revoc_reg_delta_json, _) = ledger::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_resp).unwrap();

            let _revoc_reg_delta: RevocationRegistryDeltaV1 = serde_json::from_str(&revoc_reg_delta_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_revoc_reg_delta_request_works_for_two_timestamps", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_revoc_reg_delta_request_works_for_default_submitter_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_revoc_reg_delta_request_works_for_default_submitter_did");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let to = time::get_time().sec as u64 + 1000;
            let get_rev_reg_delta_req = ledger::build_get_revoc_reg_delta_request(None, &rev_reg_id, None, to).unwrap();
            let get_rev_reg_delta_resp = ledger::submit_request(pool_handle, &get_rev_reg_delta_req).unwrap();

            let (_, revoc_reg_delta_json, _) = ledger::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_resp).unwrap();

            let _revoc_reg_delta: RevocationRegistryDeltaV1 = serde_json::from_str(&revoc_reg_delta_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_revoc_reg_delta_request_works_for_default_submitter_did", &wallet_config);
        }
    }

    mod indy_register_transaction_parser_for_sp {
        extern crate libc;

        use super::*;

        use self::libc::c_char;

        #[test]
        fn indy_register_transaction_parser_for_sp_works() {
            utils::setup("indy_register_transaction_parser_for_sp_works");

            extern fn parse(msg: *const c_char, parsed: *mut *const c_char) -> i32 {
                unsafe { *parsed = msg; }
                ErrorCode::Success as i32
            }
            extern fn free(_buf: *const c_char) -> i32 { ErrorCode::Success as i32 }

            ledger::register_transaction_parser_for_sp("my_txn_type", parse, free).unwrap();

            utils::tear_down("indy_register_transaction_parser_for_sp_works");
        }
    }

    mod get_response_metadata {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_nym_requests() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("get_response_metadata_works_for_nym_requests");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &did, None, None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REPLY);

            let response_metadata = ledger::get_response_metadata(&nym_resp).unwrap();
            _check_write_response_metadata(&response_metadata);

            let get_nym_request = ledger::build_get_nym_request(None, &did).unwrap();
            let get_nym_response = ledger::submit_request_with_retries(pool_handle, &get_nym_request, &nym_resp).unwrap();

            let response_metadata = ledger::get_response_metadata(&get_nym_response).unwrap();
            _check_read_response_metadata(&response_metadata);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "get_response_metadata_works_for_nym_requests", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_get_txn_request() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("get_response_metadata_works_for_get_txn_request");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &did, None, None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REPLY);

            let response_metadata = ledger::get_response_metadata(&nym_resp).unwrap();
            let response_metadata: serde_json::Value = serde_json::from_str(&response_metadata).unwrap();

            let seq_no = response_metadata["seqNo"].as_u64().unwrap() as i32;

            thread::sleep(std::time::Duration::from_secs(2));

            let get_txn_request = ledger::build_get_txn_request(None, seq_no, None).unwrap();
            let get_txn_response = ledger::submit_request(pool_handle, &get_txn_request).unwrap();

            let response_metadata = ledger::get_response_metadata(&get_txn_response).unwrap();
            let response_metadata: serde_json::Value = serde_json::from_str(&response_metadata).unwrap();
            assert!(response_metadata["seqNo"].as_u64().is_some());
            assert!(response_metadata["txnTime"].as_u64().is_none());
            assert!(response_metadata["lastTxnTime"].as_u64().is_none());
            assert!(response_metadata["lastSeqNo"].as_u64().is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "get_response_metadata_works_for_get_txn_request", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_pool_config_request() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("get_response_metadata_works_for_pool_config_request");

            let request = ledger::build_pool_config_request(&trustee_did, true, false).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            let response_metadata = ledger::get_response_metadata(&response).unwrap();
            _check_write_response_metadata(&response_metadata);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "get_response_metadata_works_for_pool_config_request", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_revocation_related_get_requests() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("get_response_metadata_works_for_revocation_related_get_requests");

            let (_, _, rev_reg_id) = ledger::post_entities();

            let timestamp = time::get_time().sec as u64 + 1000;

            let get_rev_reg_req = ledger::build_get_revoc_reg_request(Some(DID_MY1), &rev_reg_id, timestamp).unwrap();
            let get_rev_reg_resp = ledger::submit_request(pool_handle, &get_rev_reg_req).unwrap();

            let response_metadata = ledger::get_response_metadata(&get_rev_reg_resp).unwrap();
            _check_read_response_metadata(&response_metadata);

            let get_rev_reg_delta_req = ledger::build_get_revoc_reg_delta_request(Some(DID_MY1), &rev_reg_id, None, timestamp).unwrap();
            let get_rev_reg_delta_resp = ledger::submit_request(pool_handle, &get_rev_reg_delta_req).unwrap();

            let response_metadata = ledger::get_response_metadata(&get_rev_reg_delta_resp).unwrap();
            _check_read_response_metadata(&response_metadata);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "get_response_metadata_works_for_revocation_related_get_requests", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_invalid_response() {
            utils::setup("get_response_metadata_works_for_invalid_response");

            let res = ledger::get_response_metadata("{}");
            assert_code!(ErrorCode::LedgerInvalidTransaction, res);

            utils::tear_down("get_response_metadata_works_for_invalid_response");
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn get_response_metadata_works_for_not_found_response() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("get_response_metadata_works_for_not_found_response");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let get_nym_request = ledger::build_get_nym_request(Some(&did), &did).unwrap();
            let get_nym_response = ledger::submit_request(pool_handle, &get_nym_request).unwrap();

            let response_metadata = ledger::get_response_metadata(&get_nym_response).unwrap();
            let response_metadata: serde_json::Value = serde_json::from_str(&response_metadata).unwrap();

            assert!(response_metadata["lastTxnTime"].as_u64().is_some());
            assert!(response_metadata["seqNo"].as_u64().is_none());
            assert!(response_metadata["txnTime"].as_u64().is_none());
            assert!(response_metadata["lastSeqNo"].as_u64().is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "get_response_metadata_works_for_not_found_response", &wallet_config);
        }

        fn _check_write_response_metadata(response_metadata: &str) {
            let response_metadata: serde_json::Value = serde_json::from_str(response_metadata).unwrap();

            assert!(response_metadata["seqNo"].as_u64().is_some());
            assert!(response_metadata["txnTime"].as_u64().is_some());
            assert!(response_metadata["lastTxnTime"].as_u64().is_none());
            assert!(response_metadata["lastSeqNo"].as_u64().is_none());
        }

        fn _check_read_response_metadata(response_metadata: &str) {
            let response_metadata: serde_json::Value = serde_json::from_str(response_metadata).unwrap();

            assert!(response_metadata["seqNo"].as_u64().is_some());
            assert!(response_metadata["txnTime"].as_u64().is_some());
            assert!(response_metadata["lastTxnTime"].as_u64().is_some());
            assert!(response_metadata["lastSeqNo"].as_u64().is_none());
        }
    }

    mod auth_rule {
        use super::*;

        const ADD_AUTH_ACTION: &str = "ADD";
        const EDIT_AUTH_ACTION: &str = "EDIT";
        const FIELD: &str = "role";
        const VALUE: &str = "0";
        const NEW_VALUE: &str = "101";
        const ROLE_CONSTRAINT: &str = r#"{
            "sig_count": 1,
            "metadata": {},
            "role": "0",
            "constraint_id": "ROLE",
            "need_to_be_owner": false
        }"#;

        #[test]
        fn indy_build_auth_rule_requests_works_for_adding_new_trustee() {
            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": VALUE,
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &ADD_AUTH_ACTION,
                                                          FIELD,
                                                          None,
                                                          Some(VALUE),
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": VALUE,
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(constants::NYM),
                                                              Some(ADD_AUTH_ACTION),
                                                              Some(FIELD),
                                                              None,
                                                              Some(VALUE)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_requests_works_for_adding_new_identity_owner() {
            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": serde_json::Value::Null,
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &ADD_AUTH_ACTION,
                                                          FIELD,
                                                          None,
                                                          None,
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": serde_json::Value::Null,
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(constants::NYM),
                                                              Some(ADD_AUTH_ACTION),
                                                              Some(FIELD),
                                                              None,
                                                              None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_requests_works_for_demote_trustee() {
            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": VALUE,
                "new_value": serde_json::Value::Null,
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &EDIT_AUTH_ACTION,
                                                          FIELD,
                                                          Some(VALUE),
                                                          None,
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": VALUE,
                "new_value": serde_json::Value::Null,
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(constants::NYM),
                                                              Some(EDIT_AUTH_ACTION),
                                                              Some(FIELD),
                                                              Some(VALUE),
                                                              None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_requests_works_for_promote_role_to_trustee() {
            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": serde_json::Value::Null,
                "new_value": VALUE,
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &EDIT_AUTH_ACTION,
                                                          FIELD,
                                                          None,
                                                          Some(VALUE),
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": serde_json::Value::Null,
                "new_value": VALUE,
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(constants::NYM),
                                                              Some(EDIT_AUTH_ACTION),
                                                              Some(FIELD),
                                                              None,
                                                              Some(VALUE)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_requests_works_for_change_trustee_to_steward() {
            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": "0",
                "new_value": "2",
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &EDIT_AUTH_ACTION,
                                                          FIELD,
                                                          Some("0"),
                                                          Some("2"),
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": constants::NYM,
                "auth_action": EDIT_AUTH_ACTION,
                "field": FIELD,
                "old_value": "0",
                "new_value": "2",
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(constants::NYM),
                                                              Some(EDIT_AUTH_ACTION),
                                                              Some(FIELD),
                                                              Some("0"),
                                                              Some("2")).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_request_works_for_complex_constraint() {
            let constraint = r#"{
                "constraint_id": "AND",
                "auth_constraints": [
                    {
                        "constraint_id": "ROLE",
                        "role": "0",
                        "sig_count": 1,
                        "need_to_be_owner": false,
                        "metadata": {}
                    },
                    {
                        "constraint_id": "OR",
                        "auth_constraints": [
                            {
                                "constraint_id": "ROLE",
                                "role": "0",
                                "sig_count": 1,
                                "need_to_be_owner": false,
                                "metadata": {}
                            },
                            {
                                "constraint_id": "ROLE",
                                "role": "0",
                                "sig_count": 1
                            }
                        ]
                    }
                ]
            }"#;

            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": constants::NYM,
                "field": FIELD,
                "new_value": NEW_VALUE,
                "auth_action": ADD_AUTH_ACTION,
                "constraint": serde_json::from_str::<serde_json::Value>(constraint).unwrap(),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          constants::NYM,
                                                          &ADD_AUTH_ACTION,
                                                          FIELD,
                                                          None,
                                                          Some(NEW_VALUE),
                                                          constraint).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rule_request_works_for_invalid_constraint() {
            let res = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                      constants::NYM,
                                                      &ADD_AUTH_ACTION,
                                                      FIELD,
                                                      None,
                                                      Some(NEW_VALUE),
                                                      r#"{"field":"value"}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_build_auth_rule_requests_works_for_any_type() {
            let txn_type = "1000000000001";

            // write
            let expected_result = json!({
                "type": constants::AUTH_RULE,
                "auth_type": txn_type,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": serde_json::Value::Null,
                "constraint": json!({
                    "sig_count": 1,
                    "metadata": {},
                    "role": "0",
                    "constraint_id": "ROLE",
                    "need_to_be_owner": false
                }),
            });

            let request = ledger::build_auth_rule_request(DID_TRUSTEE,
                                                          txn_type,
                                                          &ADD_AUTH_ACTION,
                                                          FIELD,
                                                          None,
                                                          None,
                                                          ROLE_CONSTRAINT).unwrap();
            check_request(&request, expected_result);

            // read
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
                "auth_type": txn_type,
                "auth_action": ADD_AUTH_ACTION,
                "field": FIELD,
                "new_value": serde_json::Value::Null,
            });

            let request = ledger::build_get_auth_rule_request(None,
                                                              Some(txn_type),
                                                              Some(ADD_AUTH_ACTION),
                                                              Some(FIELD),
                                                              None,
                                                              None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_auth_rules_request_works() {
            let data = json!([
                {
                    "auth_type": constants::NYM,
                    "auth_action": ADD_AUTH_ACTION,
                    "field": FIELD,
                    "new_value": VALUE,
                    "constraint": json!({
                        "sig_count": 1,
                        "metadata": {},
                        "role": "0",
                        "constraint_id": "ROLE",
                        "need_to_be_owner": false
                    })
                },
                {
                    "auth_type": constants::NYM,
                    "auth_action": EDIT_AUTH_ACTION,
                    "field": FIELD,
                    "old_value": VALUE,
                    "new_value": NEW_VALUE,
                    "constraint": json!({
                        "sig_count": 1,
                        "metadata": {},
                        "role": "0",
                        "constraint_id": "ROLE",
                        "need_to_be_owner": false
                    })
                }
            ]);

            let expected_result = json!({
                "type": constants::AUTH_RULES,
                "rules": data.clone()
            });

            let request = ledger::build_auth_rules_request(DID_TRUSTEE, &data.to_string()).unwrap();
            check_request(&request, expected_result);
        }


        #[test]
        fn indy_build_get_auth_rule_request_works_for_get_all() {
            let expected_result = json!({
                "type": constants::GET_AUTH_RULE,
            });

            let request = ledger::build_get_auth_rule_request(Some(DID_TRUSTEE),
                                                              None,
                                                              None,
                                                              None,
                                                              None,
                                                              None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_auth_rule_request_works_for_some_fields_are_specified() {
            let res = ledger::build_get_auth_rule_request(Some(DID_TRUSTEE),
                                                          Some(constants::NYM),
                                                          None,
                                                          Some(FIELD),
                                                          None,
                                                          None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rule_requests_works_for_adding_new_trustee() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rule_requests_work");

            let (_, default_constraint_json) = _get_constraint(pool_handle, ADD_AUTH_ACTION,
                                                               constants::NYM, FIELD,
                                                               None, Some(VALUE));

            _change_constraint(pool_handle, wallet_handle, &trustee_did, ADD_AUTH_ACTION,
                               constants::NYM, FIELD,
                               None, Some(VALUE), ROLE_CONSTRAINT);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, ADD_AUTH_ACTION,
                                                         constants::NYM, FIELD,
                                                         None, Some(VALUE));

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();

            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, ADD_AUTH_ACTION,
                               constants::NYM, FIELD,
                               None, Some(VALUE), &default_constraint_json);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rule_requests_work", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rule_requests_works_for_adding_new_identity_owner() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rule_requests_works_for_adding_new_identity_owner");

            let (_, default_constraint_json) = _get_constraint(pool_handle,
                                                               &ADD_AUTH_ACTION,
                                                               constants::NYM,
                                                               FIELD,
                                                               None,
                                                               None);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, ADD_AUTH_ACTION,
                               constants::NYM, FIELD, None, None, ROLE_CONSTRAINT);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, ADD_AUTH_ACTION,
                                                         constants::NYM, FIELD, None, None);

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();

            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, ADD_AUTH_ACTION, constants::NYM,
                               FIELD, None, None, &default_constraint_json);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rule_requests_works_for_adding_new_identity_owner", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rule_requests_works_for_demote_trustee() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rule_requests_works_for_demote_trustee");

            let (_, default_constraint_json) = _get_constraint(pool_handle,
                                                               &EDIT_AUTH_ACTION,
                                                               constants::NYM,
                                                               FIELD,
                                                               Some(VALUE),
                                                               None);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION,
                               constants::NYM, FIELD, Some(VALUE), None, ROLE_CONSTRAINT);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, EDIT_AUTH_ACTION, constants::NYM,
                                                         FIELD, Some(VALUE), None);

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();

            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION, constants::NYM,
                               FIELD, Some(VALUE), None, &default_constraint_json);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rule_requests_works_for_demote_trustee", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rule_requests_works_for_promote_role_to_trustee() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rule_requests_works_for_promote_role_to_trustee");

            let (_, default_constraint_json) = _get_constraint(pool_handle,
                                                               &EDIT_AUTH_ACTION,
                                                               constants::NYM,
                                                               FIELD,
                                                               None,
                                                               Some(VALUE));

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION,
                               constants::NYM, FIELD, None, Some(VALUE), ROLE_CONSTRAINT);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, EDIT_AUTH_ACTION, constants::NYM,
                                                         FIELD, None, Some(VALUE));

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();

            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION, constants::NYM,
                               FIELD, None, Some(VALUE), &default_constraint_json);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rule_requests_works_for_promote_role_to_trustee", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rule_requests_works_for_change_trustee_to_steward() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rule_requests_works_for_change_trustee_to_steward");

            let (_, default_constraint_json) = _get_constraint(pool_handle,
                                                               &EDIT_AUTH_ACTION,
                                                               constants::NYM,
                                                               FIELD,
                                                               Some("0"),
                                                               Some("2"));

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION,
                               constants::NYM, FIELD, Some("0"),
                               Some("2"), ROLE_CONSTRAINT);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, EDIT_AUTH_ACTION,
                                                         constants::NYM, FIELD, Some("0"), Some("2"));

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();

            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, EDIT_AUTH_ACTION,
                               constants::NYM, FIELD, Some("0"), Some("2"), &default_constraint_json);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rule_requests_works_for_change_trustee_to_steward", &wallet_config);
        }

        fn _change_constraint(pool_handle: i32, wallet_handle: i32, trustee_did: &str, action: &str, txn_type: &str, field: &str,
                              old_value: Option<&str>, new_value: Option<&str>, constraint: &str) {
            let auth_rule_request = ledger::build_auth_rule_request(&trustee_did,
                                                                    txn_type,
                                                                    &action,
                                                                    field,
                                                                    old_value,
                                                                    new_value,
                                                                    constraint).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &auth_rule_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);
        }

        fn _get_constraint(pool_handle: i32, action: &str, txn_type: &str, field: &str,
                           old_value: Option<&str>, new_value: Option<&str>) -> (serde_json::Value, String) {
            let get_auth_rule_request = ledger::build_get_auth_rule_request(None,
                                                                            Some(txn_type),
                                                                            Some(action),
                                                                            Some(field),
                                                                            old_value,
                                                                            new_value).unwrap();
            let response = ledger::submit_request(pool_handle, &get_auth_rule_request).unwrap();
            let mut response: Reply<serde_json::Value> = serde_json::from_str(&response).unwrap();
            let auth_rules = response.result["data"].as_array_mut().unwrap();
            assert_eq!(auth_rules.len(), 1);

            let constraint = auth_rules.pop().unwrap();
            let constraint = constraint["constraint"].clone();
            let constraint_json = serde_json::to_string(&constraint).unwrap();
            (constraint, constraint_json)
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_auth_rule_request_works_for_getting_all() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_auth_rule_request_works_for_getting_all");

            let get_auth_rule_request = ledger::build_get_auth_rule_request(None,
                                                                            None,
                                                                            None,
                                                                            None,
                                                                            None,
                                                                            None).unwrap();

            let response = ledger::submit_request(pool_handle, &get_auth_rule_request).unwrap();

            let response: Reply<serde_json::Value> = serde_json::from_str(&response).unwrap();

            let constraints = response.result["data"].as_array().unwrap();
            assert!(constraints.len() > 0);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_auth_rule_request_works_for_getting_all", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_auth_rule_request_works_for_no_constraint() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_auth_rule_request_works_for_no_constraint");

            let get_auth_rule_request = ledger::build_get_auth_rule_request(None,
                                                                            Some(constants::NYM),
                                                                            Some(ADD_AUTH_ACTION),
                                                                            Some("wrong_filed"),
                                                                            None,
                                                                            Some("wrong_new_value")).unwrap();

            let response = ledger::submit_request(pool_handle, &get_auth_rule_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_auth_rule_request_works_for_no_constraint", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_auth_rules_request_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_auth_rules_request_works");

            let action1: (&str, &str, &str, Option<&str>, Option<&str>) = (ADD_AUTH_ACTION, constants::NYM, FIELD, None, Some(VALUE));
            let action2: (&str, &str, &str, Option<&str>, Option<&str>) = (EDIT_AUTH_ACTION, constants::NYM, FIELD, Some(VALUE), Some(NEW_VALUE));

            let (_, default_constraint_action_1) = _get_constraint(pool_handle, action1.0,
                                                                   action1.1, action1.2, action1.3, action1.4);

            let (_, default_constraint_action_2) = _get_constraint(pool_handle, action2.0,
                                                                   action2.1, action2.2, action2.3, action2.4);

            let data = json!([
                {
                    "auth_type": action1.1,
                    "auth_action": action1.0,
                    "field": action1.2,
                    "new_value": action1.4,
                    "constraint": json!({
                        "sig_count": 1,
                        "metadata": {},
                        "role": "0",
                        "constraint_id": "ROLE",
                        "need_to_be_owner": false
                    })
                },
                {
                    "auth_type": action2.1,
                    "auth_action": action2.0,
                    "field": action2.2,
                    "old_value": action2.3,
                    "new_value": action2.4,
                    "constraint": json!({
                        "sig_count": 1,
                        "metadata": {},
                        "role": "0",
                        "constraint_id": "ROLE",
                        "need_to_be_owner": false
                    })
                }
            ]);

            let auth_rule_request = ledger::build_auth_rules_request(&trustee_did, &data.to_string()).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &auth_rule_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            ::std::thread::sleep(::std::time::Duration::from_secs(1));

            let (actual_constraint, _) = _get_constraint(pool_handle, action1.0, action1.1, action1.2, action1.3, action1.4);

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();
            assert_eq!(expected_constraint, actual_constraint);

            let (actual_constraint, _) = _get_constraint(pool_handle, action2.0,
                                                         action2.1, action2.2, action2.3, action2.4);

            let expected_constraint: serde_json::Value = serde_json::from_str(ROLE_CONSTRAINT).unwrap();
            assert_eq!(expected_constraint, actual_constraint);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, action1.0,
                               action1.1, action1.2, action1.3, action1.4, &default_constraint_action_1);

            _change_constraint(pool_handle, wallet_handle, &trustee_did, action2.0,
                               action2.1, action2.2, action2.3, action2.4, &default_constraint_action_2);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_auth_rules_request_works", &wallet_config);
        }
    }

    mod author_agreement {
        use super::*;

        const TEXT: &str = "indy agreement";
        const VERSION: &str = "1.0.0";
        const TAA_DIGEST: &str = "83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3";

        #[test]
        fn indy_build_txn_author_agreement_request() {
            let expected_result = json!({
                "type": constants::TXN_AUTHR_AGRMT,
                "text": TEXT,
                "version": VERSION
            });

            let request = ledger::build_txn_author_agreement_request(DID_TRUSTEE,
                                                                     TEXT,
                                                                     VERSION).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_txn_author_agreement_request_works_for_empty() {
            let expected_result = json!({
                "type": constants::TXN_AUTHR_AGRMT,
                "text": "",
                "version": VERSION
            });

            let request = ledger::build_txn_author_agreement_request(DID_TRUSTEE,
                                                                     "",
                                                                     VERSION).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_author_agreement_request() {
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT,
            });

            let request = ledger::build_get_txn_author_agreement_request(None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_author_agreement_request_for_digest() {
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT,
                "digest": TAA_DIGEST,
            });

            let data = json!({
                "digest": TAA_DIGEST
            }).to_string();

            let request = ledger::build_get_txn_author_agreement_request(None, Some(&data)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_author_agreement_request_for_version() {
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT,
                "version": VERSION,
            });

            let data = json!({
                "version": VERSION
            }).to_string();

            let request = ledger::build_get_txn_author_agreement_request(None, Some(&data)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_txn_author_agreement_request_for_timestamp() {
            let timestamp = time::get_time().sec as u64;
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT,
                "timestamp": timestamp,
            });

            let data = json!({
                "timestamp": timestamp
            }).to_string();

            let request = ledger::build_get_txn_author_agreement_request(None, Some(&data)).unwrap();
            check_request(&request, expected_result);
        }
    }

    mod acceptance_mechanism {
        use super::*;

        const VERSION: &str = "1.0.0";

        #[test]
        fn indy_build_acceptance_mechanisms_request() {
            let aml = json!({
                "acceptance mechanism label 1": "some acceptance mechanism description 1"
            });

            let expected_result = json!({
                "type": constants::TXN_AUTHR_AGRMT_AML,
                "aml": aml.clone(),
                "version": VERSION
            });

            let request = ledger::build_acceptance_mechanisms_request(DID_TRUSTEE,
                                                                     &aml.to_string(),
                                                                     VERSION,
                                                                     None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_acceptance_mechanisms_request_with_context() {
            let aml = json!({
                "acceptance mechanism label 1": "some acceptance mechanism description 1"
            });
            let context = "Some aml context";

            let expected_result = json!({
                "type": constants::TXN_AUTHR_AGRMT_AML,
                "aml": aml.clone(),
                "version": VERSION,
                "amlContext": context,
            });

            let request = ledger::build_acceptance_mechanisms_request(DID_TRUSTEE,
                                                                     &aml.to_string(),
                                                                     VERSION,
                                                                     Some(context)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_acceptance_mechanisms_request() {
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT_AML,
            });

            let request = ledger::build_get_acceptance_mechanisms_request(None, None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_acceptance_mechanisms_request_for_timestamp() {
            let timestamp = time::get_time().sec as i64;

            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT_AML,
                "timestamp": timestamp
            });

            let request = ledger::build_get_acceptance_mechanisms_request(None, Some(timestamp), None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_acceptance_mechanisms_request_for_version() {
            let expected_result = json!({
                "type": constants::GET_TXN_AUTHR_AGRMT_AML,
                "version": VERSION,
            });

            let request = ledger::build_get_acceptance_mechanisms_request(None, None, Some(VERSION)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn indy_build_get_acceptance_mechanisms_request_for_timestamp_and_version() {
            let res = ledger::build_get_acceptance_mechanisms_request(None, Some(123456789), Some(VERSION));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod author_agreement_acceptance {
        use super::*;
        use rand::Rng;

        const TEXT: &str = "some agreement text";
        const VERSION: &str = "1.0.0";
        const HASH: &str = "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e";
        const ACCEPTANCE_MECH_TYPE: &str = "acceptance type 1";
        const TIME_OF_ACCEPTANCE: u64 = 123456789;

        fn _check_request_meta(request: &str) {
            let request: serde_json::Value = serde_json::from_str(&request).unwrap();

            let expected_meta = json!({
                "mechanism": ACCEPTANCE_MECH_TYPE,
                "taaDigest": HASH,
                "time": TIME_OF_ACCEPTANCE
            });

            assert_eq!(request["taaAcceptance"], expected_meta);
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version");

            let request = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                    Some(TEXT),
                                                                                    Some(VERSION),
                                                                                    None,
                                                                                    ACCEPTANCE_MECH_TYPE,
                                                                                    TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&request);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_hash() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_hash");

            let request = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                    None,
                                                                                    None,
                                                                                    Some(HASH),
                                                                                    ACCEPTANCE_MECH_TYPE,
                                                                                    TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&request);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_hash");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_and_hash() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_and_hash");

            let request = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                    Some(TEXT),
                                                                                    Some(VERSION),
                                                                                    Some(HASH),
                                                                                    ACCEPTANCE_MECH_TYPE,
                                                                                    TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&request);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_and_hash");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_not_correspond_to_hash() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_not_correspond_to_hash");

            let res = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                Some("other text"),
                                                                                Some("0.0.1"),
                                                                                Some(HASH),
                                                                                ACCEPTANCE_MECH_TYPE,
                                                                                TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_text_version_not_correspond_to_hash");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_invalid_request() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_invalid_request");

            let res = ledger::append_txn_author_agreement_acceptance_to_request("Invalid request string",
                                                                                None,
                                                                                None,
                                                                                Some(HASH),
                                                                                ACCEPTANCE_MECH_TYPE,
                                                                                TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_invalid_request");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_missed_text_version_hash() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_missed_text_version_hash");

            let res = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                None,
                                                                                None,
                                                                                None,
                                                                                ACCEPTANCE_MECH_TYPE,
                                                                                TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_missed_text_version_hash");
        }

        #[test]
        fn indy_append_txn_author_agreement_acceptance_to_request_works_for_partial_combination_of_text_version() {
            utils::setup("indy_append_txn_author_agreement_acceptance_to_request_works_for_partial_combination_of_text_version");

            let res = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                Some(TEXT),
                                                                                None,
                                                                                None,
                                                                                ACCEPTANCE_MECH_TYPE,
                                                                                TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            let res = ledger::append_txn_author_agreement_acceptance_to_request(REQUEST,
                                                                                None,
                                                                                Some(VERSION),
                                                                                None,
                                                                                ACCEPTANCE_MECH_TYPE,
                                                                                TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_append_txn_author_agreement_acceptance_to_request_works_for_partial_combination_of_text_version");
        }

        fn _rand_string() -> String {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .collect()
        }

        fn _rand_version() -> String {
            let version: u32 = rand::thread_rng().gen();
            version.to_string()
        }

        fn _gen_aml_data() -> (String, String, String, String) {
            let aml_label = _rand_string();
            let aml = json!({
                aml_label.clone(): _rand_string()
            }).to_string();
            let version: String = _rand_version();
            let aml_context: String = _rand_string();
            (aml, aml_label, version, aml_context)
        }

        fn _gen_taa_data() -> (String, String) {
            let text: String = _rand_string();
            let version: String = _rand_version();
            (text, version)
        }

        fn _send_taa(pool_handle: i32, wallet_handle: i32, trustee_did: &str, taa_text: &str, taa_version: &str) {
            let request = ledger::build_txn_author_agreement_request(&trustee_did, &taa_text, &taa_version).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);
        }

        fn _set_taa(pool_handle: i32, wallet_handle: i32, trustee_did: &str) -> (String, String) {
            let (taa_text, taa_version) = _gen_taa_data();
            _send_taa(pool_handle, wallet_handle, trustee_did, &taa_text, &taa_version);
            (taa_text, taa_version)
        }

        fn _reset_taa(pool_handle: i32, wallet_handle: i32, trustee_did: &str) {
            let taa_version = _rand_version();
            _send_taa(pool_handle, wallet_handle, trustee_did, "", &taa_version);
        }

        fn _set_aml(pool_handle: i32, wallet_handle: i32, trustee_did: &str) -> (String, String, String, String) {
            let (aml, aml_label, aml_version, aml_context) = _gen_aml_data();
            let request = ledger::build_acceptance_mechanisms_request(trustee_did, &aml, &aml_version, Some(&aml_context)).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did, &request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);
            (aml, aml_label, aml_version, aml_context)
        }

        #[test]
        fn indy_txn_author_agreement_requests_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_txn_author_agreement_requests_works");

            _set_aml(pool_handle, wallet_handle, &trustee_did);

            let (taa_text, taa_version) = _gen_taa_data();

            let txn_author_agreement_request = ledger::build_txn_author_agreement_request(&trustee_did, &taa_text, &taa_version).unwrap();
            let txn_author_agreement_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &txn_author_agreement_request).unwrap();
            pool::check_response_type(&txn_author_agreement_response, ResponseType::REPLY);

            let get_txn_author_agreement_request = ledger::build_get_txn_author_agreement_request(Some(&trustee_did), None).unwrap();
            let get_txn_author_agreement_response = ledger::submit_request_with_retries(pool_handle, &get_txn_author_agreement_request, &txn_author_agreement_response).unwrap();
            pool::check_response_type(&get_txn_author_agreement_response, ResponseType::REPLY);

            let response: serde_json::Value = serde_json::from_str(&get_txn_author_agreement_response).unwrap();
            let expected_data = json!({"text": taa_text, "version": taa_version});
            assert_eq!(response["result"]["data"], expected_data);

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_txn_author_agreement_requests_works", &wallet_config);
        }

        #[test]
        fn indy_acceptance_mechanism_requests_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_acceptance_mechanism_requests_works");

            let (aml, _, aml_version, aml_context) = _gen_aml_data();

            let acceptance_mechanisms_request = ledger::build_acceptance_mechanisms_request(&trustee_did, &aml, &aml_version, Some(&aml_context)).unwrap();
            let acceptance_mechanisms_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &acceptance_mechanisms_request).unwrap();
            pool::check_response_type(&acceptance_mechanisms_response, ResponseType::REPLY);

            //            {
            //                let request = ledger::build_get_acceptance_mechanisms_request(Some(&trustee_did), None, None).unwrap();
            //                let response = ledger::submit_request(pool_handle, &request).unwrap();
            //                pool::check_response_type(&response, ResponseType::REPLY);
            //
            //                let response: serde_json::Value = serde_json::from_str(&response).unwrap();
            //                let expected_data = json!({"aml": aml, "version": aml_version, "amlContext": aml_context});
            //                assert_eq!(response["result"]["data"], expected_data);
            //            }

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_acceptance_mechanism_requests_works", &wallet_config);
        }

        #[test]
        fn indy_author_agreement_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_author_agreement_works");

            let (_, aml_label, _, _) = _set_aml(pool_handle, wallet_handle, &trustee_did);
            let (taa_text, taa_version) = _set_taa(pool_handle, wallet_handle, &trustee_did);

            let (did_, verkey_) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REJECT);

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();
            let nym_req = ledger::append_txn_author_agreement_acceptance_to_request(&nym_req,
                                                                                    Some(&taa_text), Some(&taa_version),
                                                                                    None, &aml_label,
                                                                                    time::get_time().sec as u64).unwrap();

            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REPLY);

            let get_nym_req = ledger::build_get_nym_request(Some(&trustee_did), &did_).unwrap();
            let get_nym_resp = ledger::submit_request_with_retries(pool_handle, &get_nym_req, &nym_resp).unwrap();
            pool::check_response_type(&get_nym_resp, ResponseType::REPLY);

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_author_agreement_works", &wallet_config);
        }

        #[test]
        fn indy_reset_author_agreement_works() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_reset_author_agreement_works");

            _set_aml(pool_handle, wallet_handle, &trustee_did);
            _set_taa(pool_handle, wallet_handle, &trustee_did);

            let (did_, verkey_) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();

            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REJECT);

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
            pool::check_response_type(&nym_resp, ResponseType::REPLY);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_reset_author_agreement_works", &wallet_config);
        }

        #[test]
        fn indy_author_agreement_works_for_using_invalid_taa() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_author_agreement_works_for_using_invalid_taa");

            let (_, aml_label, _, _) = _set_aml(pool_handle, wallet_handle, &trustee_did);
            _set_taa(pool_handle, wallet_handle, &trustee_did);

            let (did_, verkey_) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            {
                let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();

                let nym_req = ledger::append_txn_author_agreement_acceptance_to_request(&nym_req,
                                                                                        Some("INVALID TAA TEXT"),
                                                                                        Some(&VERSION),
                                                                                        None, &aml_label,
                                                                                        time::get_time().sec as u64).unwrap();

                let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
                pool::check_response_type(&nym_resp, ResponseType::REJECT);
            }

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_author_agreement_works_for_using_invalid_taa", &wallet_config);
        }

        #[test]
        fn indy_author_agreement_works_for_using_invalid_aml() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_author_agreement_works_for_using_invalid_aml");

            _set_aml(pool_handle, wallet_handle, &trustee_did);
            let (taa_text, taa_version) = _set_taa(pool_handle, wallet_handle, &trustee_did);

            let (did_, verkey_) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            {
                let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();
                let nym_req = ledger::append_txn_author_agreement_acceptance_to_request(&nym_req,
                                                                                        Some(&taa_text),
                                                                                        Some(&taa_version),
                                                                                        None,
                                                                                        "INVALID AML LABEL",
                                                                                        time::get_time().sec as u64).unwrap();

                let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
                pool::check_response_type(&nym_resp, ResponseType::REJECT);
            }

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_author_agreement_works_for_using_invalid_aml", &wallet_config);
        }

        #[test]
        fn indy_author_agreement_works_for_using_not_last_taa() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_author_agreement_works_for_using_not_last_taa");

            let (_, aml_label, _, _) = _set_aml(pool_handle, wallet_handle, &trustee_did);
            let (taa_text, taa_version) = _set_taa(pool_handle, wallet_handle, &trustee_did);
            let (taa_text_2, taa_version_2) = _set_taa(pool_handle, wallet_handle, &trustee_did);

            let (did_, verkey_) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();

            {
                let nym_req = ledger::append_txn_author_agreement_acceptance_to_request(&nym_req,
                                                                                        Some(&taa_text), Some(&taa_version),
                                                                                        None, &aml_label,
                                                                                        time::get_time().sec as u64).unwrap();

                let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
                pool::check_response_type(&nym_resp, ResponseType::REJECT);
            }

            let nym_req = ledger::build_nym_request(&trustee_did, &did_, Some(&verkey_), None, None).unwrap();

            {
                let nym_req = ledger::append_txn_author_agreement_acceptance_to_request(&nym_req,
                                                                                        Some(&taa_text_2), Some(&taa_version_2),
                                                                                        None, &aml_label,
                                                                                        time::get_time().sec as u64).unwrap();

                let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
                pool::check_response_type(&nym_resp, ResponseType::REPLY);
            }

            _reset_taa(pool_handle, wallet_handle, &trustee_did);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_author_agreement_works_for_using_not_last_taa", &wallet_config);
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
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_sign_and_submit_request_works_for_not_found_signer");

            let res = ledger::sign_and_submit_request(pool_handle, wallet_handle, &DID, REQUEST);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_and_submit_request_works_for_not_found_signer", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_request_works_for_invalid_message() {
            let pool_handle = utils::setup_with_pool("indy_submit_request_works_for_invalid_message");

            let res = ledger::submit_request(pool_handle, "request");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_pool(pool_handle, "indy_submit_request_works_for_invalid_message");
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_sign_and_submit_request_works_for_invalid_json() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_trustee("indy_sign_and_submit_request_works_for_invalid_json");

            let res = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, "request");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_sign_and_submit_request_works_for_invalid_json", &wallet_config);
        }
    }

    mod submit_action {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_pool_restart_for_unknown_node_name() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_pool_restart_for_unknown_node_name");

            let get_validator_info_request = ledger::build_get_validator_info_request(&did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &did, &get_validator_info_request).unwrap();

            let nodes = r#"["Other Node"]"#;
            let res = ledger::submit_action(pool_handle, &get_validator_info_request, Some(nodes), None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_pool_restart_for_unknown_node_name", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_action_works_for_pool_restart_for_invalid_nodes_format() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_trustee("indy_submit_action_works_for_pool_restart_for_invalid_nodes_format");

            let get_validator_info_request = ledger::build_get_validator_info_request(&did).unwrap();
            let get_validator_info_request = ledger::sign_request(wallet_handle, &did, &get_validator_info_request).unwrap();

            let nodes = r#""Node1""#;
            let res = ledger::submit_action(pool_handle, &get_validator_info_request, Some(nodes), None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_action_works_for_pool_restart_for_invalid_nodes_format", &wallet_config);
        }
    }

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_only_required_fields() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_send_nym_request_works_for_only_required_fields");
            let (my_did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_nym_request_works_for_only_required_fields", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_with_option_fields() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_send_nym_request_works_with_option_fields");
            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), Some("some_alias"), Some("STEWARD")).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_nym_request_works_with_option_fields", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_nym_request_works_for_different_roles() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_send_nym_request_works_for_different_roles");

            for role in ["STEWARD", "TRUSTEE", "TRUST_ANCHOR", "ENDORSER", "NETWORK_MONITOR"].iter() {
                let (my_did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();
                let nym_request = ledger::build_nym_request(&trustee_did, &my_did, None, None, Some(role)).unwrap();
                let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
                pool::check_response_type(&response, ResponseType::REPLY);
            }

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_nym_request_works_for_different_roles", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_requests_works_for_wrong_role() {
            let res = ledger::build_nym_request(&IDENTIFIER, &DEST, None, None, Some("WRONG_ROLE"));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_wrong_signer_role() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_nym_request_works_for_wrong_signer_role");
            let (my_did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &my_did, None, None, None).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REPLY);

            let (my_did2, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_request = ledger::build_nym_request(&my_did, &my_did2, None, None, None).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_nym_request_works_for_wrong_signer_role", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_request_works_for_unknown_signer_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_nym_request_works_for_unknown_signer_did");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&did, DID, None, None, None).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &nym_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_nym_request_works_for_unknown_signer_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_nym_request_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_nym_request_works_for_unknown_did");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let get_nym_request = ledger::build_get_nym_request(Some(&did), &did).unwrap();
            let get_nym_response = ledger::submit_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_nym_request_works_for_unknown_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_request_works_for_invalid_submitter_identifier() {
            let res = ledger::build_nym_request(INVALID_IDENTIFIER, IDENTIFIER, None, None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_nym_request_works_for_invalid_target_identifier() {
            let res = ledger::build_nym_request(IDENTIFIER, INVALID_IDENTIFIER, None, None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_request_works_for_invalid_submitter_identifier() {
            let res = ledger::build_get_nym_request(Some(INVALID_IDENTIFIER), IDENTIFIER);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_nym_request_works_for_invalid_target_identifier() {
            let res = ledger::build_get_nym_request(Some(IDENTIFIER), INVALID_IDENTIFIER);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_nym_requests_works_for_reset_role() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_nym_requests_works_for_reset_role");
            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let mut nym_request = ledger::build_nym_request(&trustee_did, &my_did,
                                                            Some(&my_verkey), None, Some("TRUSTEE")).unwrap();
            let nym_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            pool::check_response_type(&nym_req_resp, ResponseType::REPLY);

            let mut get_nym_request = ledger::build_get_nym_request(Some(&my_did), &my_did).unwrap();
            let get_nym_response_with_role = ledger::submit_request_with_retries(pool_handle, &get_nym_request, &nym_req_resp).unwrap();

            let get_nym_response_with_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_with_role).unwrap();
            let get_nym_response_data_with_role: GetNymResultData = serde_json::from_str(&get_nym_response_with_role.result.data.unwrap()).unwrap();

            nym_request = ledger::build_nym_request(&my_did, &my_did,
                                                    Some(&my_verkey), None, Some("")).unwrap();
            let nym_req_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();
            pool::check_response_type(&nym_req_resp, ResponseType::REPLY);

            get_nym_request = ledger::build_get_nym_request(Some(&my_did), &my_did).unwrap();
            let get_nym_response_without_role = ledger::submit_request_with_retries(pool_handle, &get_nym_request, &nym_req_resp).unwrap();

            let get_nym_response_without_role: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response_without_role).unwrap();
            let get_nym_response_data_without_role: GetNymResultData = serde_json::from_str(&get_nym_response_without_role.result.data.unwrap()).unwrap();

            assert!(get_nym_response_data_without_role.role.is_none());
            assert_ne!(get_nym_response_data_without_role.role, get_nym_response_data_with_role.role);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_nym_requests_works_for_reset_role", &wallet_config);
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_attrib_request_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_attrib_request_works_for_unknown_did");

            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let attrib_request = ledger::build_attrib_request(&did, &did, None, Some(ATTRIB_RAW_DATA), None).unwrap();

            let response = ledger::submit_request(pool_handle, &attrib_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_attrib_request_works_for_unknown_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_attrib_request_works_for_unknown_did");
            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let get_attrib_request = ledger::build_get_attrib_request(Some(&did), &did, Some("endpoint"), None, None).unwrap();
            let get_attrib_response = ledger::submit_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_attrib_request_works_for_unknown_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_attrib_request_works_for_unknown_attribute() {
            let (wallet_handle, pool_handle, did, _my_vk, wallet_config) = utils::setup_new_identity("indy_get_attrib_request_works_for_unknown_attribute");

            let get_attrib_request = ledger::build_get_attrib_request(Some(&did), &did, Some("some_attribute"), None, None).unwrap();
            let get_attrib_response = ledger::submit_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_attrib_request_works_for_unknown_attribute", &wallet_config);
        }


        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_request_works_for_invalid_submitter_did() {
            let res = ledger::build_attrib_request(INVALID_IDENTIFIER, IDENTIFIER, None, Some(ATTRIB_RAW_DATA), None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_attrib_request_works_for_invalid_target_did() {
            let res = ledger::build_attrib_request(IDENTIFIER, INVALID_IDENTIFIER, None, Some(ATTRIB_RAW_DATA), None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_request_works_for_invalid_submitter_identifier() {
            let res = ledger::build_get_attrib_request(Some(INVALID_IDENTIFIER), IDENTIFIER, Some("endpoint"), None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_attrib_request_works_for_invalid_target_identifier() {
            let res = ledger::build_get_attrib_request(Some(IDENTIFIER), INVALID_IDENTIFIER, Some("endpoint"), None, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod schemas_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_missed_field_in_data_json() {
            let res = ledger::build_schema_request(IDENTIFIER, r#"{"name":"name"}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_invalid_data_json_format() {
            let res = ledger::build_schema_request(IDENTIFIER, r#"{"name":"name", "keys":"name"}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_schema_requests_works_for_invalid_submitter_identifier() {
            let res = ledger::build_schema_request(INVALID_IDENTIFIER, SCHEMA_DATA);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_id() {
            let res = ledger::build_get_schema_request(Some(IDENTIFIER), "invalid_schema_id");
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_build_get_schema_requests_works_for_invalid_submitter_identifier() {
            let res = ledger::build_get_schema_request(Some(INVALID_IDENTIFIER), &anoncreds::gvt_schema_id());
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_schema_request_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_schema_request_works_for_unknown_did");
            let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let schema_request = ledger::build_schema_request(&did, SCHEMA_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_schema_request_works_for_unknown_did", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_schema_request_works_for_unknown_schema() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_request_works_for_unknown_schema");

            let get_schema_request = ledger::build_get_schema_request(Some(DID_TRUSTEE), &Schema::schema_id(DID, "other_schema", "1.0")).unwrap();
            let get_schema_response = ledger::submit_request(pool_handle, &get_schema_request).unwrap();

            let res = ledger::parse_get_schema_response(&get_schema_response);
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_request_works_for_unknown_schema", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_parse_returns_error_for_wrong_type() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_parse_returns_error_for_wrong_type");

            let (schema_id, _, _) = ledger::post_entities();

            let get_schema_request = ledger::build_get_schema_request(Some(DID_MY1), &schema_id).unwrap();
            let get_schema_response = ledger::submit_request(pool_handle, &get_schema_request).unwrap();

            let res = ledger::parse_get_cred_def_response(&get_schema_response);
            assert_code!(ErrorCode::LedgerInvalidTransaction, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_parse_returns_error_for_wrong_type", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_get_parse_returns_error_for_wrong_type_and_unknown_schema() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_parse_returns_error_for_wrong_type_and_unknown_schema");

            let get_schema_request = ledger::build_get_schema_request(Some(DID_TRUSTEE), &Schema::schema_id(DID, "other_schema", "1.0")).unwrap();
            let get_schema_response = ledger::submit_request(pool_handle, &get_schema_request).unwrap();

            let res = ledger::parse_get_cred_def_response(&get_schema_response);
            assert_code!(ErrorCode::LedgerInvalidTransaction, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_parse_returns_error_for_wrong_type_and_unknown_schema", &wallet_config);
        }
    }

    mod node_requests {
        use super::*;

        #[test]
        fn indy_build_node_request_works_for_missed_fields_in_data_json() {
            let res = ledger::build_node_request(IDENTIFIER, DEST, r#"{ }"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_build_node_request_works_for_wrong_service() {
            let data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 1, "alias":"some", "services": ["SERVICE"], "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", "blskey_pop": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}"#;
            let res = ledger::build_node_request(IDENTIFIER, DEST, data);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_send_node_request_works_for_wrong_role() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_trustee("indy_send_node_request_works_for_wrong_role");

            let key = utils::crypto::create_key(wallet_handle, None).unwrap();
            let node_request = ledger::build_node_request(&did, &key, NODE_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REJECT);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_send_node_request_works_for_wrong_role", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_node_request_works_for_steward_already_has_node() {
            let (wallet_handle, pool_handle, did, wallet_config) = utils::setup_steward("indy_submit_node_request_works_for_steward_already_has_node");

            let key = utils::crypto::create_key(wallet_handle, None).unwrap();
            let node_request = ledger::build_node_request(&did, &key, NODE_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REJECT);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_node_request_works_for_steward_already_has_node", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_node_request_works_for_new_node_without_bls_pop() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_submit_node_request_works_for_new_node_without_bls_pop");

            let (my_did, _) = did::create_store_and_publish_my_did_from_steward(wallet_handle, pool_handle).unwrap();

            let node_data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 2, "alias":"some", "services": ["VALIDATOR"], "blskey": "4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba"}"#;
            let node_request = ledger::build_node_request(&my_did, DEST, node_data).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_node_request_works_for_new_node_without_bls_pop", &wallet_config);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_submit_node_request_works_for_pop_not_correspond_blskey() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_submit_node_request_works_for_pop_not_correspond_blskey");

            let (my_did, _) = did::create_store_and_publish_my_did_from_steward(wallet_handle, pool_handle).unwrap();

            let node_data = r#"{"node_ip":"10.0.0.100", "node_port": 1, "client_ip": "10.0.0.100", "client_port": 2, "alias":"some", "services": ["VALIDATOR"], "blskey": "4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba", "blskey_pop": "RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP"}"#;
            let node_request = ledger::build_node_request(&my_did, DEST, node_data).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_submit_node_request_works_for_pop_not_correspond_blskey", &wallet_config);
        }
    }

    mod cred_def_requests {
        use super::*;

        #[test]
        fn indy_build_cred_def_request_works_for_invalid_data_json() {
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"}}}"#;
            let res = ledger::build_cred_def_txn(IDENTIFIER, data);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_build_cred_def_request_works_for_invalid_submitter_did() {
            let res = ledger::build_cred_def_txn(INVALID_IDENTIFIER, &anoncreds::credential_def_json());
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_build_get_cred_def_request_works_for_invalid_submitter_did() {
            let res = ledger::build_get_cred_def_request(Some(INVALID_IDENTIFIER), &anoncreds::issuer_1_gvt_cred_def_id());
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_cred_def_requests_works_for_hash_field() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_cred_def_requests_works_for_hash_field");

            let (issuer_did, _) = did::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let (schema_id, schema_json) = anoncreds::issuer_create_schema(&issuer_did,
                                                                           GVT_SCHEMA_NAME,
                                                                           SCHEMA_VERSION,
                                                                           r#"["enc", "raw", "hash"]"#).unwrap();

            let schema_request = ledger::build_schema_request(&issuer_did, &schema_json).unwrap();
            let schema_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &schema_request).unwrap();
            pool::check_response_type(&schema_response, ::utils::types::ResponseType::REPLY);

            let get_schema_request = ledger::build_get_schema_request(Some(&issuer_did), &schema_id).unwrap();
            let get_schema_response = ledger::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
            let (_, schema_json) = ledger::parse_get_schema_response(&get_schema_response).unwrap();

            let (cred_def_id, cred_def_json) = anoncreds::issuer_create_credential_definition(wallet_handle,
                                                                                              &issuer_did,
                                                                                              &schema_json,
                                                                                              TAG_1,
                                                                                              None,
                                                                                              Some(&anoncreds::default_cred_def_config())).unwrap();
            let cred_def_request = ledger::build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
            let cred_def_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &cred_def_request).unwrap();
            pool::check_response_type(&cred_def_response, ::utils::types::ResponseType::REPLY);

            let get_cred_def_request = ledger::build_get_cred_def_request(Some(DID_MY1), &cred_def_id).unwrap();
            let get_cred_def_response = ledger::submit_request_with_retries(pool_handle, &get_cred_def_request, &cred_def_response).unwrap();
            let (_, cred_def_json) = ledger::parse_get_cred_def_response(&get_cred_def_response).unwrap();

            let _cred_def: CredentialDefinitionV1 = serde_json::from_str(&cred_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_cred_def_requests_works_for_hash_field", &wallet_config);
        }
    }
}

fn check_request(request: &str, expected_result: serde_json::Value) {
    let request: serde_json::Value = serde_json::from_str(request).unwrap();
    assert_eq!(request["operation"], expected_result);
}

fn check_default_identifier(request: &str) {
    let request: serde_json::Value = serde_json::from_str(&request).unwrap();
    assert_eq!(request["identifier"], DEFAULT_LIBIDY_DID);
}
