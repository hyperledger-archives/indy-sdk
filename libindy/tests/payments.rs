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
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;
extern crate sha2;

#[macro_use]
mod utils;

use self::indy::ErrorCode;
use utils::payments;
use utils::constants::*;
use utils::Setup;

static EMPTY_OBJECT: &str = "{}";
static EMPTY_ARRAY: &str = "[]";
static PAYMENT_METHOD_NAME: &str = "null";
static CORRECT_INPUTS: &str = r#"["pay:null:1", "pay:null:2"]"#;
static CORRECT_OUTPUTS: &str = r#"[{"recipient": "pay:null:1", "amount":1}, {"recipient": "pay:null:2", "amount":2}]"#;
static CORRECT_FEES: &str = r#"{"txnType1":1, "txnType2":2}"#;
static TEST_RES_STRING: &str = "test";
static CORRECT_PAYMENT_ADDRESS: &str = "pay:null:test";
static EXTRA: &str = "extra_1";

mod high_cases {
    use super::*;


    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works() {
            Setup::empty();

            let _res = payments::register_payment_method("register_payment_method_works",
                                                         Some(payments::mock_method::create_payment_address::handle),
                                                         Some(payments::mock_method::add_request_fees::handle),
                                                         Some(payments::mock_method::parse_response_with_fees::handle),
                                                         Some(payments::mock_method::build_get_payment_sources_request::handle),
                                                         Some(payments::mock_method::parse_get_payment_sources_response::handle),
                                                         Some(payments::mock_method::build_payment_req::handle),
                                                         Some(payments::mock_method::parse_payment_response::handle),
                                                         Some(payments::mock_method::build_mint_req::handle),
                                                         Some(payments::mock_method::build_set_txn_fees_req::handle),
                                                         Some(payments::mock_method::build_get_txn_fees_req::handle),
                                                         Some(payments::mock_method::parse_get_txn_fees_response::handle),
                                                         Some(payments::mock_method::build_verify_payment_req::handle),
                                                         Some(payments::mock_method::parse_verify_payment_response::handle),
                                                         Some(payments::mock_method::sign_with_address::handle),
                                                         Some(payments::mock_method::verify_with_address::handle)
            ).unwrap();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::create_payment_address(setup.wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();
            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            payments::create_payment_address(setup.wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            let all_addresses = payments::list_payment_addresses(setup.wallet_handle).unwrap();

            let vec: Vec<String> = serde_json::from_str(&all_addresses).unwrap();
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&TEST_RES_STRING.to_string()));
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(setup.wallet_handle,
                                                                   Some(IDENTIFIER),
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);
        }

        #[test]
        fn add_request_fees_works_for_empty_outputs() {
            let setup = Setup::payment_wallet();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (txn, method) = payments::add_request_fees(setup.wallet_handle,
                                                           Some(IDENTIFIER),
                                                           EMPTY_OBJECT,
                                                           CORRECT_INPUTS,
                                                           EMPTY_ARRAY,
                                                           None,
            ).unwrap();

            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, TEST_RES_STRING);
        }

        #[test]
        fn add_request_fees_works_for_extra() {
            let setup = Setup::payment_wallet();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(setup.wallet_handle,
                                                                   Some(IDENTIFIER),
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);
        }

        #[test]
        fn add_request_fees_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(setup.wallet_handle,
                                                                   None,
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        fn parse_response_with_fees_works() {
            Setup::payment();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        fn build_get_payment_sources_request_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(setup.wallet_handle, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(setup.wallet_handle, None, CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }
    }

    mod build_get_payment_sources_with_from_request {
        use super::*;

        #[test]
        fn build_get_payment_sources_with_from_request_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS, Some(1)).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_get_payment_sources_with_from_request_works_for_no_from() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS, None).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }


        #[test]
        fn build_get_payment_sources_request_works_with_from_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, None, CORRECT_PAYMENT_ADDRESS, Some(1)).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }
    }

    mod parse_get_payment_sources_response {
        use super::*;

        #[test]
        fn parse_get_payment_sources_response_works() {
            Setup::payment();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::Success, TEST_RES_STRING, -1);

            let res_plugin = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod parse_get_payment_sources_with_from_response {
        use super::*;

        #[test]
        fn parse_get_payment_sources_with_from_response_works() {
            Setup::payment();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::Success, TEST_RES_STRING, -1);

            let (res_plugin, num) = payments::parse_get_payment_sources_with_from_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
            assert_eq!(num, None);
        }


        #[test]
        fn parse_get_payment_sources_with_from_response_works_has_next() {
            Setup::payment();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::Success, TEST_RES_STRING, 1);

            let (res_plugin, num) = payments::parse_get_payment_sources_with_from_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
            assert_eq!(num, Some(1));
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_req_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(setup.wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_payment_req_works_for_extra() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(setup.wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_payment_req_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(setup.wallet_handle,
                                                                    None,
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works() {
            Setup::payment();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod author_agreement_acceptance_for_extra {
        use super::*;

        const TEXT: &str = "some agreement text";
        const VERSION: &str = "1.0.0";
        const HASH: &str = "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e";
        const ACCEPTANCE_MECH_TYPE: &str = "acceptance type 1";
        const TIME_OF_ACCEPTANCE: u64 = 123379200;

        fn _check_request_meta(extra: &str) {
            let extra: serde_json::Value = serde_json::from_str(&extra).unwrap();

            let expected_acceptance = json!({
                "mechanism": ACCEPTANCE_MECH_TYPE,
                "taaDigest": HASH,
                "time": TIME_OF_ACCEPTANCE
            });

            assert_eq!(extra["taaAcceptance"], expected_acceptance);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_text_version() {
            Setup::payment();

            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     Some(TEXT),
                                                                     Some(VERSION),
                                                                     None,
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_hash() {
            Setup::payment();

            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     None,
                                                                     None,
                                                                     Some(HASH),
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_and_hash() {
            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     Some(TEXT),
                                                                     Some(VERSION),
                                                                     Some(HASH),
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_not_correspond_to_hash() {
            Setup::payment();

            let res = payments::prepare_extra_with_acceptance_data(None,
                                                                   Some("other text"),
                                                                   Some("0.0.1"),
                                                                   Some(HASH),
                                                                   ACCEPTANCE_MECH_TYPE,
                                                                   TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_invalid_request() {
            Setup::payment();

            let res = payments::prepare_extra_with_acceptance_data(Some("Invalid extra string"),
                                                                   None,
                                                                   None,
                                                                   Some(HASH),
                                                                   ACCEPTANCE_MECH_TYPE,
                                                                   TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_req_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(setup.wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_mint_req_works_for_extra() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(setup.wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }

        #[test]
        fn build_mint_req_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(setup.wallet_handle,
                                                                 None,
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
        }

        #[test]
        fn build_set_txn_fees_request_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_for_generic_result() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
        }

        #[test]
        fn build_get_txn_fees_request_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(setup.wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works() {
            Setup::payment();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(setup.wallet_handle, Some(IDENTIFIER), "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_empty_submitter_did() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(setup.wallet_handle, None, "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        fn parse_verify_payment_response_works() {
            Setup::payment();

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
        }
    }

    mod indy_get_request_info {
        use super::*;

        fn _fees() -> String {
            json!({
                "1": 100
            }).to_string()
        }

        fn _auth_rule() -> String {
            json!({
                "result":{
                    "data":[{
                        "new_value":"0",
                        "constraint":{
                            "need_to_be_owner":false,
                            "sig_count":1,
                            "metadata":{
                                "fees": "1"
                            },
                            "role":"0",
                            "constraint_id":"ROLE"
                        },
                        "field":"role",
                        "auth_type":"1",
                        "auth_action":"ADD"
                    }],
                    "identifier":"LibindyDid111111111111",
                    "auth_action":"ADD",
                    "new_value":"0",
                    "reqId":15616,
                    "auth_type":"1",
                    "type":"121",
                    "field":"role"
                },
                "op":"REPLY"
            }).to_string()
        }

        fn _requester_info() -> String {
            json!({
                "role": "0",
                "need_to_be_owner":false,
                "sig_count":1,
            }).to_string()
        }

        #[test]
        fn indy_get_request_info_for_requester_match_to_constraint() {
            Setup::empty();

            let req_info = payments::get_request_info(&_auth_rule(), &_requester_info(), &_fees()).unwrap();
            let req_info: serde_json::Value = serde_json::from_str(&req_info).unwrap();

            let expected_req_info = json!({
                "price": 100,
                "requirements": [{
                    "role": "0",
                    "need_to_be_owner":false,
                    "sig_count":1,
                }]
            });

            assert_eq!(expected_req_info, req_info);
        }

        #[test]
        fn indy_get_request_info_for_requester_not_match_to_constraint() {
            Setup::empty();

            let requester_info = json!({
                "role": "101",
                "need_to_be_owner":false,
                "sig_count":1,
            }).to_string();

            let res = payments::get_request_info(&_auth_rule(), &requester_info, &_fees());
            assert_code!(ErrorCode::TransactionNotAllowed, res);
        }

        #[test]
        fn indy_get_request_info_for_no_fee() {
            Setup::empty();

            let req_info = payments::get_request_info(&_auth_rule(), &_requester_info(), "{}").unwrap();
            let req_info: serde_json::Value = serde_json::from_str(&req_info).unwrap();

            let expected_req_info = json!({
                "price": 0,
                "requirements": [{
                    "role": "0",
                    "need_to_be_owner":false,
                    "sig_count":1,
                }]
            });

            assert_eq!(expected_req_info, req_info);
        }
    }
    
    mod sign_with_address {
        use super::*;
        use sha2::Digest;

        #[test]
        fn sign_with_address_works() {
            let setup = Setup::payment_wallet();

            let test_vec = vec![0u8; 32];

            let test_sig = sha2::Sha256::digest(test_vec.as_slice()).as_slice().to_vec();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, PAYMENT_METHOD_NAME);
            payments::create_payment_address(setup.wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();
            payments::mock_method::sign_with_address::inject_mock(ErrorCode::Success, test_sig);

            let res = payments::sign_with_address(setup.wallet_handle, CORRECT_PAYMENT_ADDRESS, test_vec.as_slice()).unwrap();

            assert_eq!(res, vec![ 102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37 ]);
        }
    }
}

#[cfg(not(feature="only_high_cases"))]
mod medium_cases {
    use super::*;
    use api::INVALID_WALLET_HANDLE;
    static WRONG_PAYMENT_METHOD_NAME: &str = "null_payment_handler";
    static INPUTS_UNKNOWN_METHOD: &str = r#"["pay:unknown_payment_method:1"]"#;
    static OUTPUTS_UNKNOWN_METHOD: &str = r#"[{"recipient": "pay:unknown_payment_method:1", "amount":1}]"#;
    static INPUTS_INVALID_FORMAT: &str = r#"pay:null:1"#;
    static OUTPUTS_INVALID_FORMAT: &str = r#"["pay:null:1",1]"#;
    static INCOMPATIBLE_INPUTS: &str = r#"["pay:PAYMENT_METHOD_1:1", "pay:PAYMENT_METHOD_2:1"]"#;
    static INCOMPATIBLE_OUTPUTS: &str = r#"[{"recipient": "pay:PAYMENT_METHOD_1:1", "amount":1}, {"recipient": "pay:PAYMENT_METHOD_2:1", "amount":1}]"#;
    static EQUAL_INPUTS: &str = r#"["pay:null1:1", "pay:null1:1", "pay:null1:2"]"#;
    static EQUAL_OUTPUTS: &str = r#"[{"paymentAddress": "pay:null:1", "amount":1}, {"paymentAddress": "pay:null:1", "amount":2}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#;
    static PAYMENT_RESPONSE: &str = r#"{"reqId":1, "sources":[{"input": "pay:null:1", "amount":1}, {"input": "pay:null:2", "amount":2}]}"#;
    static GET_TXN_FEES_RESPONSE: &str = r#"{"reqId":1, fees:{"txnType1":1, "txnType2":2}}"#;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works_for_no_first_method() {
            Setup::empty();

            let err = payments::register_payment_method(PAYMENT_METHOD_NAME,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None,
                                                        None
            ).unwrap_err();

            assert_eq!(ErrorCode::CommonInvalidParam3, err);
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works_for_non_existent_plugin() {
            let setup = Setup::payment_wallet();

            let res = payments::create_payment_address(setup.wallet_handle, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME);
            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn create_payment_address_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::create_payment_address(setup.wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME);
            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_works_for_nonexistent_wallet() {
            Setup::payment();

            let err = payments::list_payment_addresses(INVALID_WALLET_HANDLE);
            assert_code!(ErrorCode::WalletInvalidHandle, err);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_non_existent_plugin_name() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INPUTS_UNKNOWN_METHOD,
                                                 EMPTY_ARRAY,
                                                 None,
            );
            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        fn add_request_fees_works_for_empty_inputs() {
            let setup = Setup::payment_wallet();

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EMPTY_ARRAY,
                                                 CORRECT_OUTPUTS,
                                                 None,
            );
            assert_code!( ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_no_method() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_outputs() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 INCOMPATIBLE_OUTPUTS,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_inputs() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INCOMPATIBLE_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_with_inputs_and_outputs() {
            let setup = Setup::payment_wallet();

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1"]"#,
                                                 r#"[{"recipient": "pay:null2:1", "amount":1, "extra":"1"}]"#,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        fn add_request_fees_works_for_malformed_input() {
            let setup = Setup::payment_wallet();

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1", 1]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_incorrect_payment_address() {
            let setup = Setup::payment_wallet();

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(INVALID_IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }


        #[test]
        fn add_request_fees_works_for_several_equal_inputs() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EQUAL_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_several_equal_outputs() {
            let setup = Setup::payment_wallet();
            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EQUAL_OUTPUTS,
                                                 None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn add_request_fees_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::add_request_fees(setup.wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 CORRECT_OUTPUTS,
                                                 None);

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_nonexistent_plugin() {
            Setup::payment();

            let err = payments::parse_response_with_fees(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);
            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        fn parse_response_with_fees_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT);
            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn build_get_payment_sources_request_works_for_nonexistent_plugin() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_request(setup.wallet_handle, Some(IDENTIFIER), "pay:null1:test");
            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_malformed_payment_address() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_request(setup.wallet_handle, Some(IDENTIFIER), "pay:null1");
            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_request(setup.wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_payment_sources_request(setup.wallet_handle,
                                                                  Some(IDENTIFIER),
                                                                  CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod build_get_payment_sources_with_from_request {
        use super::*;

        #[test]
        pub fn build_get_payment_sources_with_from_request_works_for_nonexistent_plugin() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, Some(IDENTIFIER), "pay:null1:test", Some(1));
            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        pub fn build_get_payment_sources_with_from_request_works_for_malformed_payment_address() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, Some(IDENTIFIER), "pay:null1", Some(1));

            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        pub fn build_get_payment_sources_with_from_request_works_for_invalid_wallet_handle() {
            Setup::payment();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::WalletInvalidHandle, "");

            let err = payments::build_get_payment_sources_with_from_request(INVALID_WALLET_HANDLE, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS, Some(1));
            assert_code!(ErrorCode::WalletInvalidHandle, err);
        }

        #[test]
        pub fn build_get_payment_sources_with_from_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let err = payments::build_get_payment_sources_with_from_request(setup.wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS, Some(1));
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn build_get_payment_sources_with_from_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_payment_sources_with_from_request(setup.wallet_handle,
                                                                            Some(IDENTIFIER),
                                                                            CORRECT_PAYMENT_ADDRESS,
                                                                            Some(1),
            );
            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn parse_get_payment_sources_response_works_for_nonexistent_plugin() {
            Setup::payment();

            let err = payments::parse_get_payment_sources_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);
            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        fn parse_get_payment_sources_response_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::WalletAccessFailed, "", -1);

            let err = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);
            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_get_payment_sources_with_from_response {
        use super::*;

        #[test]
        pub fn parse_get_payment_sources_with_from_response_works_for_nonexistent_plugin() {
            Setup::payment();

            let err = payments::parse_get_payment_sources_with_from_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        fn parse_get_payment_sources_with_from_response_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::WalletAccessFailed, "", -1);

            let err = payments::parse_get_payment_sources_with_from_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(INVALID_IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_payment_request_works_for_empty_inputs() {
            let setup = Setup::payment_wallet();

            let err = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EMPTY_ARRAY,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn build_payment_request_works_for_empty_outputs() {
            let setup = Setup::payment_wallet();

            let err = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EMPTY_ARRAY,
                                                  None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn build_payment_request_works_for_unknown_payment_method() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_UNKNOWN_METHOD,
                                                  OUTPUTS_UNKNOWN_METHOD,
                                                  None);
            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn build_payment_request_works_for_invalid_input_payment_address() {
            let setup = Setup::payment_wallet();

            let inputs = r#"["pay:null"]"#;
            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_payment_methods() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INCOMPATIBLE_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_output_payment_methods() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  INCOMPATIBLE_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_output_payment_methods() {
            let setup = Setup::payment_wallet();
            let inputs = r#"["pay:PAYMENT_METHOD_1:1"]"#;
            let outputs = r#"[{"recipient": "pay:PAYMENT_METHOD_2:1", "amount": 1}]"#;

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  outputs,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);
        }

        #[test]
        fn build_payment_request_works_for_invalid_inputs_format() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_INVALID_FORMAT,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_payment_request_works_for_invalid_outputs_format() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  OUTPUTS_INVALID_FORMAT,
                                                  None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_payment_request_works_for_several_equal_inputs() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EQUAL_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, res);

        }

        #[test]
        fn build_payment_request_works_for_several_equal_outputs() {
            let setup = Setup::payment_wallet();

            let res = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EQUAL_OUTPUTS,
                                                  None,
            );
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_payment_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_payment_req(setup.wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            Setup::payment();

            let res = payments::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                       PAYMENT_RESPONSE);
            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn parse_payment_response_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);
            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works_for_empty_outputs() {
            let setup = Setup::payment_wallet();

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               EMPTY_ARRAY,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_mint_request_works_for_unknown_payment_method() {
            let setup = Setup::payment_wallet();

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_UNKNOWN_METHOD,
                                               None);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn build_mint_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(INVALID_IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_mint_request_works_for_invalid_outputs_format() {
            let setup = Setup::payment_wallet();

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_INVALID_FORMAT,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_mint_request_works_for_invalid_output_payment_address() {
            let setup = Setup::payment_wallet();
            let outputs = r#"[{"recipient": "pay:null", "amount":1, "extra":"1"}]"#;

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               outputs,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_mint_request_works_for_several_equal_outputs() {
            let setup = Setup::payment_wallet();

            let res = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               EQUAL_OUTPUTS,
                                               None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_mint_request_works_for_incompatible_output_payment_methods() {
            let setup = Setup::payment_wallet();
            let err = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               INCOMPATIBLE_OUTPUTS,
                                               None);
            assert_code!(ErrorCode::IncompatiblePaymentError, err);
        }

        #[test]
        fn build_mint_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();
            payments::mock_method::build_mint_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_mint_req(setup.wallet_handle,
                                               Some(IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works_for_unknown_payment_method() {
            let setup = Setup::payment_wallet();

            let res = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let res = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_fees_format() {
            let setup = Setup::payment_wallet();
            let fees = r#"[txnType1:1, txnType2:2]"#;

            let res = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       fees);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_set_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let res = payments::build_get_txn_fees_req(setup.wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_unknown_payment_method() {
            let setup = Setup::payment_wallet();

            let res = payments::build_get_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_generic_error() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_txn_fees_req(setup.wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works_for_unknown_payment_method() {
            Setup::payment();

            let res = payments::parse_get_txn_fees_response(WRONG_PAYMENT_METHOD_NAME,
                                                            GET_TXN_FEES_RESPONSE);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);
        }

        #[test]
        fn parse_get_txn_fees_response_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_method() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(setup.wallet_handle, Some(IDENTIFIER), "pay:null1:test");

            assert_code!(ErrorCode::UnknownPaymentMethod, ec);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_address() {
            let setup = Setup::payment_wallet();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(setup.wallet_handle, Some(IDENTIFIER), "pay:null1");

            assert_code!(ErrorCode::IncompatiblePaymentError, ec);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_invalid_submitter_did() {
            let setup = Setup::payment_wallet();

            let err = payments::build_verify_payment_req(setup.wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);

            assert_code!(ErrorCode::CommonInvalidStructure, err);
        }

        #[test]
        fn build_verify_payment_req_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_verify_payment_req(INVALID_WALLET_HANDLE,
                                                         Some(IDENTIFIER),
                                                         CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        pub fn parse_verify_payment_response_works_for_nonexistent_plugin() {
            Setup::payment();

            let err = payments::parse_verify_payment_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        fn parse_verify_payment_response_works_for_generic_error() {
            Setup::payment();

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);
        }
    }

    mod sign_with_address {
        use super::*;

        #[test]
        pub fn sign_with_address_fails_for_nonexistent_plugin() {
            let setup = Setup::payment_wallet();

            let err = payments::sign_with_address(setup.wallet_handle, "pay:123:kill", vec![33].as_slice());

            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        pub fn sign_with_address_fails_for_no_payment_address() {
            let setup = Setup::payment_wallet();

            let err = payments::sign_with_address(setup.wallet_handle, "", vec![33].as_slice());

            assert_code!(ErrorCode::CommonInvalidParam3, err);
        }

        #[test]
        pub fn sign_with_address_fails_for_no_message() {
            let setup = Setup::payment_wallet();

            let err = payments::sign_with_address(setup.wallet_handle, CORRECT_PAYMENT_ADDRESS, vec![].as_slice());

            assert_code!(ErrorCode::CommonInvalidParam5, err);
        }
        
        #[test]
        pub fn sign_with_address_for_incorrect_payment_address() {
            let setup = Setup::payment_wallet();

            let ec = payments::sign_with_address(setup.wallet_handle, "pay:null1", vec![33].as_slice());

            assert_code!(ErrorCode::IncompatiblePaymentError, ec);
        }
    }

    mod verify_with_address {
        use super::*;

        #[test]
        pub fn verify_with_address_fails_for_nonexistent_plugin() {
            Setup::payment();

            let err = payments::verify_with_address("pay:123:kill", vec![33].as_slice(), vec![33].as_slice());

            assert_code!(ErrorCode::UnknownPaymentMethod, err);
        }

        #[test]
        pub fn verify_with_address_for_incorrect_payment_address() {
            Setup::payment();

            payments::mock_method::verify_with_address::inject_mock(ErrorCode::Success, true);

            let ec = payments::verify_with_address("pay:null1", vec![33].as_slice(), vec![33].as_slice());

            assert_code!(ErrorCode::IncompatiblePaymentError, ec);
        }

        #[test]
        pub fn verify_with_address_for_no_payment_address() {
            Setup::payment();

            payments::mock_method::verify_with_address::inject_mock(ErrorCode::Success, true);

            let ec = payments::verify_with_address("", vec![33].as_slice(), vec![33].as_slice());

            assert_code!(ErrorCode::CommonInvalidParam2, ec);
        }

        #[test]
        pub fn verify_with_address_for_no_message() {
            Setup::payment();

            payments::mock_method::verify_with_address::inject_mock(ErrorCode::Success, true);

            let ec = payments::verify_with_address(CORRECT_PAYMENT_ADDRESS, vec![].as_slice(), vec![33].as_slice());

            assert_code!(ErrorCode::CommonInvalidParam4, ec);
        }

        #[test]
        pub fn verify_with_address_for_no_signature() {
            Setup::payment();

            payments::mock_method::verify_with_address::inject_mock(ErrorCode::Success, true);

            let ec = payments::verify_with_address(CORRECT_PAYMENT_ADDRESS, vec![33].as_slice(), vec![].as_slice());

            assert_code!(ErrorCode::CommonInvalidParam6, ec);
        }
    }
}
