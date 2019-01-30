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
extern crate indy_crypto;
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

fn setup() -> i32 {
    let wallet_handle = utils::setup_with_wallet();
    payments::mock_method::init();
    wallet_handle
}

mod high_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works() {
            utils::setup();

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

            utils::tear_down();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works() {
            let wallet_handle = setup();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);
            
            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address_works() {
            let wallet_handle = setup();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            let all_addresses = payments::list_payment_addresses(wallet_handle).unwrap();

            let vec: Vec<String> = serde_json::from_str(&all_addresses).unwrap();
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&TEST_RES_STRING.to_string()));

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works() {
            let wallet_handle = setup();

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

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_empty_outputs() {
            let wallet_handle = setup();

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

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_extra() {
            let wallet_handle = setup();

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

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_empty_submitter_did() {
            let wallet_handle = setup();

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

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        fn parse_response_with_fees_works() {
            let wallet_handle = setup();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        fn build_get_payment_sources_request_works() {
            let wallet_handle = setup();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_payment_sources_request(wallet_handle, None, CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_get_payment_sources_response {
        use super::*;

        #[test]
        fn parse_get_payment_sources_response_works() {
            let wallet_handle = setup();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_req_works() {
            let wallet_handle = setup();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_req_works_for_extra() {
            let wallet_handle = setup();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    Some(IDENTIFIER),
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_req_works_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    None,
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
                                                                    None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works() {
            let wallet_handle = setup();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_req_works() {
            let wallet_handle = setup();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_req_works_for_extra() {
            let wallet_handle = setup();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 Some(IDENTIFIER),
                                                                 CORRECT_OUTPUTS,
                                                                 Some(EXTRA),
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_req_works_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 None,
                                                                 CORRECT_OUTPUTS,
                                                                 None,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works() {
            let wallet_handle = setup();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_for_generic_result() {
            let wallet_handle = setup();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_txn_fees_request_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(wallet_handle,
                                                       None,
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works() {
            let wallet_handle = setup();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }


    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works() {
            let wallet_handle = setup();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_empty_submitter_did() {
            let wallet_handle = setup();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, pm) = payments::build_verify_payment_req(wallet_handle, None, "pay:null:test").unwrap();

            assert_eq!(req, TEST_RES_STRING);
            assert_eq!(pm, PAYMENT_METHOD_NAME);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        fn parse_verify_payment_response_works() {
            let wallet_handle = setup();

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }
}

mod medium_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works_for_no_first_method() {
            utils::setup();

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

            utils::tear_down();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works_for_non_existent_plugin() {
            let wallet_handle = setup();

            let res = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn create_payment_address_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let res = payments::create_payment_address(wallet_handle + 1, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn create_payment_address_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_works_for_nonexistent_wallet() {
            let wallet_handle = setup();

            let err = payments::list_payment_addresses(wallet_handle + 1);

            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_non_existent_plugin_name() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INPUTS_UNKNOWN_METHOD,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_empty_inputs() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EMPTY_ARRAY,
                                                 CORRECT_OUTPUTS,
                                                 None,
            );

            assert_code!( ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_no_method() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_outputs() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 INCOMPATIBLE_OUTPUTS,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_inputs() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 INCOMPATIBLE_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_several_methods_with_inputs_and_outputs() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1"]"#,
                                                 r#"[{"recipient": "pay:null2:1", "amount":1, "extra":"1"}]"#,
                                                 None,
            );

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_malformed_input() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1", 1]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_incorrect_payment_address() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1"]"#,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle + 1,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(INVALID_IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }


        #[test]
        fn add_request_fees_works_for_several_equal_inputs() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 EQUAL_INPUTS,
                                                 EMPTY_ARRAY,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_several_equal_outputs() {
            let wallet_handle = setup();
            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EQUAL_OUTPUTS,
                                                 None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn add_request_fees_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::add_request_fees(wallet_handle,
                                                 Some(IDENTIFIER),
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 CORRECT_OUTPUTS,
                                                 None);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_nonexistent_plugin() {
            utils::setup();
            payments::mock_method::init();

            let err = payments::parse_response_with_fees(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down();
        }

        #[test]
        fn parse_response_with_fees_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod build_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn build_get_payment_sources_request_works_for_nonexistent_plugin() {
            let wallet_handle = setup();

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), "pay:null1:test");

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_malformed_payment_address() {
            let wallet_handle = setup();

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(IDENTIFIER), "pay:null1");

            assert_code!(ErrorCode::IncompatiblePaymentError, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let err = payments::build_get_payment_sources_request(wallet_handle + 1, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS);
            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_get_payment_sources_request_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let err = payments::build_get_payment_sources_request(wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_payment_sources_request_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::build_get_payment_sources_request::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_payment_sources_request(wallet_handle,
                                                                  Some(IDENTIFIER),
                                                                  CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_get_payment_sources_request {
        use super::*;

        #[test]
        pub fn parse_get_payment_sources_response_works_for_nonexistent_plugin() {
            utils::setup();
            payments::mock_method::init();

            let err = payments::parse_get_payment_sources_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down();
        }

        #[test]
        fn parse_get_payment_sources_response_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::parse_get_payment_sources_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_payment_sources_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_payment_req(invalid_wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(INVALID_IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_empty_inputs() {
            let wallet_handle = setup();

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EMPTY_ARRAY,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_empty_outputs() {
            let wallet_handle = setup();

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EMPTY_ARRAY,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_unknown_payment_method() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_UNKNOWN_METHOD,
                                                  OUTPUTS_UNKNOWN_METHOD,
                                                  None);
            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_invalid_input_payment_address() {
            let wallet_handle = setup();

            let inputs = r#"["pay:null"]"#;
            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_payment_methods() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INCOMPATIBLE_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_output_payment_methods() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  INCOMPATIBLE_OUTPUTS,
                                                  None);
            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_output_payment_methods() {
            let wallet_handle = setup();
            let inputs = r#"["pay:PAYMENT_METHOD_1:1"]"#;
            let outputs = r#"[{"recipient": "pay:PAYMENT_METHOD_2:1", "amount": 1}]"#;

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  inputs,
                                                  outputs,
                                                  None);

            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_invalid_inputs_format() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  INPUTS_INVALID_FORMAT,
                                                  CORRECT_OUTPUTS,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_invalid_outputs_format() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  OUTPUTS_INVALID_FORMAT,
                                                  None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_several_equal_inputs() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  EQUAL_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_several_equal_outputs() {
            let wallet_handle = setup();

            let res = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  EQUAL_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_payment_request_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_payment_req(wallet_handle,
                                                  Some(IDENTIFIER),
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
                                                  None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            utils::setup();
            payments::mock_method::init();

            let res = payments::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                       PAYMENT_RESPONSE);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down();
        }

        #[test]
        fn parse_payment_response_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works_for_empty_outputs() {
            let wallet_handle = setup();

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               EMPTY_ARRAY,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_unknown_payment_method() {
            let wallet_handle = setup();

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_UNKNOWN_METHOD,
                                               None);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = payments::build_mint_req(invalid_wallet_handle,
                                               Some(IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let res = payments::build_mint_req(wallet_handle,
                                               Some(INVALID_IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_invalid_outputs_format() {
            let wallet_handle = setup();

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               OUTPUTS_INVALID_FORMAT,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_invalid_output_payment_address() {
            let wallet_handle = setup();
            let outputs = r#"[{"recipient": "pay:null", "amount":1, "extra":"1"}]"#;

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               outputs,
                                               None);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_several_equal_outputs() {
            let wallet_handle = setup();

            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               EQUAL_OUTPUTS,
                                               None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_incompatible_output_payment_methods() {
            let wallet_handle = setup();
            let res = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               INCOMPATIBLE_OUTPUTS,
                                               None);

            assert_code!(ErrorCode::IncompatiblePaymentError, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_mint_request_works_for_generic_error() {
            let wallet_handle = setup();
            payments::mock_method::build_mint_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_mint_req(wallet_handle,
                                               Some(IDENTIFIER),
                                               CORRECT_OUTPUTS,
                                               None,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works_for_unknown_payment_method() {
            let wallet_handle = setup();

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_set_txn_fees_req(invalid_wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_fees_format() {
            let wallet_handle = setup();
            let fees = r#"[txnType1:1, txnType2:2]"#;

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       fees);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_set_txn_fees_request_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_set_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_get_txn_fees_req(invalid_wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(INVALID_IDENTIFIER),
                                                       PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_unknown_payment_method() {
            let wallet_handle = setup();

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       WRONG_PAYMENT_METHOD_NAME);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_get_txn_fees_request_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_txn_fees_req(wallet_handle,
                                                       Some(IDENTIFIER),
                                                       PAYMENT_METHOD_NAME,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works_for_unknown_payment_method() {
            utils::setup();
            payments::mock_method::init();

            let res = payments::parse_get_txn_fees_response(WRONG_PAYMENT_METHOD_NAME,
                                                            GET_TXN_FEES_RESPONSE);

            assert_code!(ErrorCode::UnknownPaymentMethod, res);

            utils::tear_down();
        }

        #[test]
        fn parse_get_txn_fees_response_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod build_verify_payment_req {
        use super::*;

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_method() {
            let wallet_handle = setup();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null1:test");

            assert_code!(ErrorCode::UnknownPaymentMethod, ec);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_incorrect_payment_address() {
            let wallet_handle = setup();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let ec = payments::build_verify_payment_req(wallet_handle, Some(IDENTIFIER), "pay:null1");

            assert_code!(ErrorCode::IncompatiblePaymentError, ec);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let err = payments::build_verify_payment_req(wallet_handle + 1, Some(IDENTIFIER), CORRECT_PAYMENT_ADDRESS);
            assert_code!(ErrorCode::WalletInvalidHandle, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        pub fn build_verify_payment_req_works_for_invalid_submitter_did() {
            let wallet_handle = setup();

            let err = payments::build_verify_payment_req(wallet_handle, Some(INVALID_IDENTIFIER), CORRECT_PAYMENT_ADDRESS);

            assert_code!(ErrorCode::CommonInvalidStructure, err);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn build_verify_payment_req_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::build_verify_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_verify_payment_req(wallet_handle,
                                                         Some(IDENTIFIER),
                                                         CORRECT_PAYMENT_ADDRESS,
            );

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod parse_verify_payment_response {
        use super::*;

        #[test]
        pub fn parse_verify_payment_response_works_for_nonexistent_plugin() {
            utils::setup();
            payments::mock_method::init();

            let err = payments::parse_verify_payment_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS);

            assert_code!(ErrorCode::UnknownPaymentMethod, err);

            utils::tear_down();
        }

        #[test]
        fn parse_verify_payment_response_works_for_generic_error() {
            let wallet_handle = setup();

            payments::mock_method::parse_verify_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_verify_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT);

            assert_code!(ErrorCode::WalletAccessFailed, err);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }
}
