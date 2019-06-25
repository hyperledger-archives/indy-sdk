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

#[macro_use]
mod utils;

use self::indy::ErrorCode;
use utils::payments;
use utils::constants::*;

use api::INVALID_WALLET_HANDLE;

static EMPTY_OBJECT: &str = "{}";
static EMPTY_ARRAY: &str = "[]";
static PAYMENT_METHOD_NAME: &str = "null";
static WRONG_PAYMENT_METHOD_NAME: &str = "null_payment_handler";
static CORRECT_INPUTS: &str = r#"["pay:null:1", "pay:null:2"]"#;
static CORRECT_OUTPUTS: &str = r#"[{"recipient": "pay:null:1", "amount":1}, {"recipient": "pay:null:2", "amount":2}]"#;
static INPUTS_UNKNOWN_METHOD: &str = r#"["pay:unknown_payment_method:1"]"#;
static OUTPUTS_UNKNOWN_METHOD: &str = r#"[{"recipient": "pay:unknown_payment_method:1", "amount":1}]"#;
static INPUTS_INVALID_FORMAT: &str = r#"pay:null:1"#;
static OUTPUTS_INVALID_FORMAT: &str = r#"["pay:null:1",1]"#;
static INCOMPATIBLE_INPUTS: &str = r#"["pay:PAYMENT_METHOD_1:1", "pay:PAYMENT_METHOD_2:1"]"#;
static INCOMPATIBLE_OUTPUTS: &str = r#"[{"recipient": "pay:PAYMENT_METHOD_1:1", "amount":1}, {"recipient": "pay:PAYMENT_METHOD_2:1", "amount":1}]"#;
static EQUAL_INPUTS: &str = r#"["pay:null1:1", "pay:null1:1", "pay:null1:2"]"#;
static EQUAL_OUTPUTS: &str = r#"[{"paymentAddress": "pay:null:1", "amount":1}, {"paymentAddress": "pay:null:1", "amount":2}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#;
static CORRECT_FEES: &str = r#"{"txnType1":1, "txnType2":2}"#;
static PAYMENT_RESPONSE: &str = r#"{"reqId":1, "sources":[{"input": "pay:null:1", "amount":1}, {"input": "pay:null:2", "amount":2}]}"#;
static GET_TXN_FEES_RESPONSE: &str = r#"{"reqId":1, fees:{"txnType1":1, "txnType2":2}}"#;
static TEST_RES_STRING: &str = "test";
static CORRECT_PAYMENT_ADDRESS: &str = "pay:null:test";
static EXTRA: &str = "extra_1";

fn setup(name: &str) -> (i32, String) {
    let (wallet_handle, wallet_config) = utils::setup_with_wallet(name);
    payments::mock_method::init();
    (wallet_handle, wallet_config)
}

mod high_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works() {
            utils::setup("register_payment_method_works");

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
            ).unwrap();

            utils::tear_down("register_payment_method_works");
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works() {
            let (wallet_handle, wallet_config) = setup("create_payment_address_works");

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
            
            utils::tear_down_with_wallet(wallet_handle, "create_payment_address_works", &wallet_config);
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address_works() {
            let (wallet_handle, wallet_config) = setup("list_payment_address_works");

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            let all_addresses = payments::list_payment_addresses(wallet_handle).unwrap();

            let vec: Vec<String> = serde_json::from_str(&all_addresses).unwrap();
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&TEST_RES_STRING.to_string()));

            utils::tear_down_with_wallet(wallet_handle, "list_payment_address_works", &wallet_config);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works");

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(wallet_handle,
                                                                   Some(IDENTIFIER),
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_empty_outputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_empty_outputs");

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (txn, method) = payments::add_request_fees(wallet_handle,
                                                           Some(IDENTIFIER),
                                                           EMPTY_OBJECT,
                                                           CORRECT_INPUTS,
                                                           EMPTY_ARRAY,
                                                           None,
            ).unwrap();

            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_empty_outputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_extra() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_extra");

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(wallet_handle,
                                                                   Some(IDENTIFIER),
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_extra", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_empty_submitter_did");

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(wallet_handle,
                                                                   None,
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
                                                                   None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        fn parse_response_with_fees_works() {
            let (wallet_handle, wallet_config) = setup("parse_response_with_fees_works");

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "parse_response_with_fees_works", &wallet_config);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        fn build_get_payment_sources_request_works() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works");

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works", &wallet_config);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_empty_submitter_did");

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(wallet_handle, None, CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod parse_get_payment_sources_response {
        use super::*;

        #[test]
        fn parse_get_payment_sources_response_works() {
            let (wallet_handle, wallet_config) = setup("parse_get_payment_sources_response_works");

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "parse_get_payment_sources_response_works", &wallet_config);
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_req_works() {
            let (wallet_handle, wallet_config) = setup("build_payment_req_works");

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_req_works", &wallet_config);
        }

        #[test]
        fn build_payment_req_works_for_extra() {
            let (wallet_handle, wallet_config) = setup("build_payment_req_works_for_extra");

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_req_works_for_extra", &wallet_config);
        }

        #[test]
        fn build_payment_req_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_payment_req_works_for_empty_submitter_did");

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    None,
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_req_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works() {
            let (wallet_handle, wallet_config) = setup("parse_payment_response_works");

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "parse_payment_response_works", &wallet_config);
        }
    }

    mod author_agreement_acceptance_for_extra {
        use super::*;

        const TEXT: &str = "some agreement text";
        const VERSION: &str = "1.0.0";
        const HASH: &str = "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e";
        const ACCEPTANCE_MECH_TYPE: &str = "acceptance type 1";
        const TIME_OF_ACCEPTANCE: u64 = 123456789;

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
            let (wallet_handle, wallet_config) = setup("indy_prepare_payment_extra_with_acceptance_data_works_for_text_version");

            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     Some(TEXT),
                                                                     Some(VERSION),
                                                                     None,
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);

            utils::tear_down_with_wallet(wallet_handle, "indy_prepare_payment_extra_with_acceptance_data_works_for_text_version", &wallet_config);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_hash() {
            let (wallet_handle, wallet_config) = setup("indy_prepare_payment_extra_with_acceptance_data_works_for_hash");

            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     None,
                                                                     None,
                                                                     Some(HASH),
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);

            utils::tear_down_with_wallet(wallet_handle, "indy_prepare_payment_extra_with_acceptance_data_works_for_hash", &wallet_config);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_and_hash() {
            let (wallet_handle, wallet_config) = setup("indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_and_hash");

            let extra = payments::prepare_extra_with_acceptance_data(None,
                                                                     Some(TEXT),
                                                                     Some(VERSION),
                                                                     Some(HASH),
                                                                     ACCEPTANCE_MECH_TYPE,
                                                                     TIME_OF_ACCEPTANCE).unwrap();
            _check_request_meta(&extra);

            utils::tear_down_with_wallet(wallet_handle, "indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_and_hash", &wallet_config);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_not_correspond_to_hash() {
            let (wallet_handle, wallet_config) = setup("indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_not_correspond_to_hash");

            let res = payments::prepare_extra_with_acceptance_data(None,
                                                                   Some("other text"),
                                                                   Some("0.0.1"),
                                                                   Some(HASH),
                                                                   ACCEPTANCE_MECH_TYPE,
                                                                   TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_prepare_payment_extra_with_acceptance_data_works_for_text_version_not_correspond_to_hash", &wallet_config);
        }

        #[test]
        fn indy_prepare_payment_extra_with_acceptance_data_works_for_invalid_request() {
            let (wallet_handle, wallet_config) = setup("indy_prepare_payment_extra_with_acceptance_data_works_for_invalid_request");

            let res = payments::prepare_extra_with_acceptance_data(Some("Invalid extra string"),
                                                                   None,
                                                                   None,
                                                                   Some(HASH),
                                                                   ACCEPTANCE_MECH_TYPE,
                                                                   TIME_OF_ACCEPTANCE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_prepare_payment_extra_with_acceptance_data_works_for_invalid_request", &wallet_config);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_req_works() {
            let (wallet_handle, wallet_config) = setup("build_mint_req_works");

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_req_works", &wallet_config);
        }

        #[test]
        fn build_mint_req_works_for_extra() {
            let (wallet_handle, wallet_config) = setup("build_mint_req_works_for_extra");

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_req_works_for_extra", &wallet_config);
        }

        #[test]
        fn build_mint_req_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_mint_req_works_for_empty_submitter_did");

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 None,
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_req_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works");

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works", &wallet_config);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_empty_submitter_did");

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_for_generic_result() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_for_generic_result");

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_for_generic_result", &wallet_config);
        }

        #[test]
        fn build_get_txn_fees_request_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_for_empty_submitter_did");

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_for_empty_submitter_did", &wallet_config);
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works() {
            let (wallet_handle, wallet_config) = setup("parse_get_txn_fees_response_works");

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "parse_get_txn_fees_response_works", &wallet_config);
        }
    }


    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works");

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works", &wallet_config);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_empty_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_empty_submitter_did");

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(wallet_handle, None, "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_empty_submitter_did", &wallet_config);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        fn parse_verify_payment_response_works() {
            let (wallet_handle, wallet_config) = setup("parse_verify_payment_response_works");

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle, "parse_verify_payment_response_works", &wallet_config);
        }
    }
}

mod medium_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works_for_no_first_method() {
            utils::setup("register_payment_method_works_for_no_first_method");

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
            ).unwrap_err();

            assert_eq!(ErrorCode::CommonInvalidParam3, err);

            utils::tear_down("register_payment_method_works_for_no_first_method");
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works_for_non_existent_plugin() {
            let (wallet_handle, wallet_config) = setup("create_payment_address_works_for_non_existent_plugin");

            let res = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle, "create_payment_address_works_for_non_existent_plugin", &wallet_config);
        }

        #[test]
        fn create_payment_address_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("create_payment_address_works_for_invalid_wallet_handle");

            let res = payments::create_payment_address(INVALID_WALLET_HANDLE, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "create_payment_address_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn create_payment_address_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("create_payment_address_works_for_generic_error");

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "create_payment_address_works_for_generic_error", &wallet_config);
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_works_for_nonexistent_wallet() {
            let (wallet_handle, wallet_config) = setup("list_payment_addresses_works_for_nonexistent_wallet");

            let err = payments::list_payment_addresses(INVALID_WALLET_HANDLE);

            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle, "list_payment_addresses_works_for_nonexistent_wallet", &wallet_config);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_non_existent_plugin_name() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_non_existent_plugin_name");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INPUTS_UNKNOWN_METHOD,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_non_existent_plugin_name", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_empty_inputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_empty_inputs");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EMPTY_ARRAY,
                                                 CORRECT_OUTPUTS,
                                                 None,
            );

            assert_code!( ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_empty_inputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_no_method() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_no_method");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_no_method", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_outputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_several_methods_in_outputs");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 INCOMPATIBLE_OUTPUTS,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_several_methods_in_outputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_inputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_several_methods_in_inputs");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INCOMPATIBLE_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_several_methods_in_inputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_with_inputs_and_outputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_several_methods_with_inputs_and_outputs");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1"]"#,
                                                 r#"[{"recipient": "pay:null2:1", "amount":1, "extra":"1"}]"#,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_several_methods_with_inputs_and_outputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_malformed_input() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_malformed_input");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1", 1]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_malformed_input", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_incorrect_payment_address() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_incorrect_payment_address");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_incorrect_payment_address", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_invalid_wallet_handle");

            let err = payments::add_request_fees(INVALID_WALLET_HANDLE,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_invalid_submitter_did");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(INVALID_IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_invalid_submitter_did", &wallet_config);
        }


        #[test]
        fn add_request_fees_works_for_several_equal_inputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_several_equal_inputs");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EQUAL_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_several_equal_inputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_several_equal_outputs() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_several_equal_outputs");
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EQUAL_OUTPUTS,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_several_equal_outputs", &wallet_config);
        }

        #[test]
        fn add_request_fees_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("add_request_fees_works_for_generic_error");

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 CORRECT_OUTPUTS,
                                                 None);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "add_request_fees_works_for_generic_error", &wallet_config);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_nonexistent_plugin() {
            utils::setup("parse_response_with_fees_works_for_nonexistent_plugin");
            payments::mock_method::init();

            let err = payments::parse_response_with_fees(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down("parse_response_with_fees_works_for_nonexistent_plugin");
        }

        #[test]
        fn parse_response_with_fees_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("parse_response_with_fees_works_for_generic_error");

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "parse_response_with_fees_works_for_generic_error", &wallet_config);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn build_get_payment_sources_request_works_for_nonexistent_plugin() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_nonexistent_plugin");

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), "pay:null1:test");

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_nonexistent_plugin", &wallet_config);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_malformed_payment_address() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_malformed_payment_address");

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), "pay:null1");

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_malformed_payment_address", &wallet_config);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_invalid_wallet_handle");

            let err = payments::build_get_payment_sources_request(INVALID_WALLET_HANDLE, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS);
            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_invalid_submitter_did");

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_get_payment_sources_request_works_for_generic_error");

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_payment_sources_request(wallet_handle,
                                                                  Some(IDENTIFIER),
                                                                  CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_payment_sources_request_works_for_generic_error", &wallet_config);
        }
    }

    mod parse_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn parse_get_payment_sources_response_works_for_nonexistent_plugin() {
            utils::setup("parse_get_payment_sources_response_works_for_nonexistent_plugin");
            payments::mock_method::init();

            let err = payments::parse_get_payment_sources_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down("parse_get_payment_sources_response_works_for_nonexistent_plugin");
        }

        #[test]
        fn parse_get_payment_sources_response_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("parse_get_payment_sources_response_works_for_generic_error");

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "parse_get_payment_sources_response_works_for_generic_error", &wallet_config);
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_invalid_wallet_handle");

            let res = payments::build_payment_req(INVALID_WALLET_HANDLE,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_invalid_submitter_did");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(INVALID_IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_empty_inputs() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_empty_inputs");

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EMPTY_ARRAY,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_empty_inputs", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_empty_outputs() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_empty_outputs");

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EMPTY_ARRAY,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_empty_outputs", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_unknown_payment_method() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_unknown_payment_method");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_UNKNOWN_METHOD,
                                                  OUTPUTS_UNKNOWN_METHOD,
                                                  None);
            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_unknown_payment_method", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_invalid_input_payment_address() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_invalid_input_payment_address");

            let inputs = r#"["pay:null"]"#;
            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_invalid_input_payment_address", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_payment_methods() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_incompatible_input_payment_methods");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INCOMPATIBLE_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_incompatible_input_payment_methods", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_output_payment_methods() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_incompatible_output_payment_methods");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  INCOMPATIBLE_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_incompatible_output_payment_methods", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_output_payment_methods() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_incompatible_input_output_payment_methods");
            let inputs = r#"["pay:PAYMENT_METHOD_1:1"]"#;
            let outputs = r#"[{"recipient": "pay:PAYMENT_METHOD_2:1", "amount": 1}]"#;

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  outputs,
                                                  None);

            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_incompatible_input_output_payment_methods", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_invalid_inputs_format() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_invalid_inputs_format");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_INVALID_FORMAT,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_invalid_inputs_format", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_invalid_outputs_format() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_invalid_outputs_format");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  OUTPUTS_INVALID_FORMAT,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_invalid_outputs_format", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_several_equal_inputs() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_several_equal_inputs");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EQUAL_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_several_equal_inputs", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_several_equal_outputs() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_several_equal_outputs");

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EQUAL_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_several_equal_outputs", &wallet_config);
        }

        #[test]
        fn build_payment_request_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_payment_request_works_for_generic_error");

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_payment_request_works_for_generic_error", &wallet_config);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            utils::setup("parse_payment_response_works_for_unknown_payment_method");
            payments::mock_method::init();

            let res = payments::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                       PAYMENT_RESPONSE);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down("parse_payment_response_works_for_unknown_payment_method");
        }

        #[test]
        fn parse_payment_response_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("parse_payment_response_works_for_generic_error");

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "parse_payment_response_works_for_generic_error", &wallet_config);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works_for_empty_outputs() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_empty_outputs");

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               EMPTY_ARRAY,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_empty_outputs", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_unknown_payment_method() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_unknown_payment_method");

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_UNKNOWN_METHOD,
                                               None);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_unknown_payment_method", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_invalid_wallet_handle");

            let res = payments::build_mint_req(INVALID_WALLET_HANDLE,
                                               Some(IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_invalid_submitter_did");

            let res = payments::build_mint_req(wallet_handle,
                                               Some(INVALID_IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_invalid_outputs_format() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_invalid_outputs_format");

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_INVALID_FORMAT,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_invalid_outputs_format", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_invalid_output_payment_address() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_invalid_output_payment_address");
            let outputs = r#"[{"recipient": "pay:null", "amount":1, "extra":"1"}]"#;

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               outputs,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_invalid_output_payment_address", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_several_equal_outputs() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_several_equal_outputs");

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               EQUAL_OUTPUTS,
                                               None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_several_equal_outputs", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_incompatible_output_payment_methods() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_incompatible_output_payment_methods");
            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               INCOMPATIBLE_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_incompatible_output_payment_methods", &wallet_config);
        }

        #[test]
        fn build_mint_request_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_mint_request_works_for_generic_error");
            payments::mock_method::build_mint_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_mint_request_works_for_generic_error", &wallet_config);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works_for_unknown_payment_method() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_unknown_payment_method");

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_unknown_payment_method", &wallet_config);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_invalid_wallet_handle");

            let res = payments::build_set_txn_fees_req(INVALID_WALLET_HANDLE,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_invalid_submitter_did");

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_fees_format() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_invalid_fees_format");
            let fees = r#"[txnType1:1, txnType2:2]"#;

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       fees);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_invalid_fees_format", &wallet_config);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_set_txn_fees_request_works_for_generic_error");

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_set_txn_fees_request_works_for_generic_error", &wallet_config);
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_works_for_invalid_wallet_handle");

            let res = payments::build_get_txn_fees_req(INVALID_WALLET_HANDLE,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_works_for_invalid_submitter_did");

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_unknown_payment_method() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_works_for_unknown_payment_method");

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_works_for_unknown_payment_method", &wallet_config);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_get_txn_fees_request_works_for_generic_error");

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_get_txn_fees_request_works_for_generic_error", &wallet_config);
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works_for_unknown_payment_method() {
            utils::setup("parse_get_txn_fees_response_works_for_unknown_payment_method");
            payments::mock_method::init();

            let res = payments::parse_get_txn_fees_response(WRONG_PAYMENT_METHOD_NAME,
                                                            GET_TXN_FEES_RESPONSE);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down("parse_get_txn_fees_response_works_for_unknown_payment_method");
        }

        #[test]
        fn parse_get_txn_fees_response_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("parse_get_txn_fees_response_works_for_generic_error");

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "parse_get_txn_fees_response_works_for_generic_error", &wallet_config);
        }
    }

    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_method() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_incorrect_payment_method");

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null1:test");

            assert_code!(ErrorCode::UnknownPaymentMethod, ec);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_incorrect_payment_method", &wallet_config);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_address() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_incorrect_payment_address");

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null1");

            assert_code!(ErrorCode::IncompatiblePaymentError, ec);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_incorrect_payment_address", &wallet_config);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_invalid_wallet_handle");

            let err = payments::build_verify_payment_req(INVALID_WALLET_HANDLE, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS);
            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_invalid_submitter_did() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_invalid_submitter_did");

            let err = payments::build_verify_payment_req(wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_invalid_submitter_did", &wallet_config);
        }

        #[test]
        fn build_verify_payment_req_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("build_verify_payment_req_works_for_generic_error");

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_verify_payment_req(wallet_handle,
                                                         Some(IDENTIFIER),
                                                         CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "build_verify_payment_req_works_for_generic_error", &wallet_config);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        pub fn parse_verify_payment_response_works_for_nonexistent_plugin() {
            utils::setup("parse_verify_payment_response_works_for_nonexistent_plugin");
            payments::mock_method::init();

            let err = payments::parse_verify_payment_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down("parse_verify_payment_response_works_for_nonexistent_plugin");
        }

        #[test]
        fn parse_verify_payment_response_works_for_generic_error() {
            let (wallet_handle, wallet_config) = setup("parse_verify_payment_response_works_for_generic_error");

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle, "parse_verify_payment_response_works_for_generic_error", &wallet_config);
        }
    }
}
