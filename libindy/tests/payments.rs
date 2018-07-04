extern crate indy;

use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate named_type;
#[macro_use]
extern crate named_type_derive;

#[macro_use]
mod utils;

use indy::api::ErrorCode;
use utils::payments;
use utils::test::TestUtils;
use utils::constants::*;
use utils::wallet::WalletUtils;

static EMPTY_OBJECT: &str = "{}";
static EMPTY_ARRAY: &str = "[]";
static PAYMENT_METHOD_NAME: &str = "null";
static WRONG_PAYMENT_METHOD_NAME: &str = "null_payment_handler";
static CORRECT_INPUTS: &str = r#"["pay:null:1", "pay:null:2"]"#;
static CORRECT_OUTPUTS: &str = r#"[{"paymentAddress": "pay:null:1", "amount":1, "extra":"1"}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#;
static INPUTS_UNKNOWN_METHOD: &str = r#"["pay:unknown_payment_method:1"]"#;
static OUTPUTS_UNKNOWN_METHOD: &str = r#"[{"paymentAddress": "pay:unknown_payment_method:1", "amount":1, "extra":"1"}]"#;
static INPUTS_INVALID_FORMAT: &str = r#"pay:null:1"#;
static OUTPUTS_INVALID_FORMAT: &str = r#"["pay:null:1",1]"#;
static INCOMPATIBLE_INPUTS: &str = r#"["pay:PAYMENT_METHOD_1:1", "pay:PAYMENT_METHOD_2:1"]"#;
static INCOMPATIBLE_OUTPUTS: &str = r#"[{"paymentAddress": "pay:PAYMENT_METHOD_1:1", "amount":1}, {"paymentAddress": "pay:PAYMENT_METHOD_2:1", "amount":1}]"#;
static EQUAL_INPUTS: &str = r#"["pay:null1:1", "pay:null1:1", "pay:null1:2"]"#;
static EQUAL_OUTPUTS: &str = r#"[{"paymentAddress": "pay:null:1", "amount":1, "extra":"1"}, {"paymentAddress": "pay:null:1", "amount":2, "extra":"2"}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#;
static CORRECT_FEES: &str = r#"{"txnType1":1, "txnType2":2}"#;
static PAYMENT_RESPONSE: &str = r#"{"reqId":1, "utxos":[{"input": "pay:null:1", "amount":1, "extra":"1"}, {"input": "pay:null:2", "amount":2, "extra":"2"}]}"#;
static GET_TXN_FEES_RESPONSE: &str = r#"{"reqId":1, fees:{"txnType1":1, "txnType2":2}}"#;
static TEST_RES_STRING: &str = "test";
static CORRECT_PAYMENT_ADDRESS: &str = "pay:null:test";

mod high_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works() {
            TestUtils::cleanup_storage();

            let _res = payments::register_payment_method("register_payment_method_works",
                                                         Some(payments::mock_method::create_payment_address::handle),
                                                         Some(payments::mock_method::add_request_fees::handle),
                                                         Some(payments::mock_method::parse_response_with_fees::handle),
                                                         Some(payments::mock_method::build_get_utxo_request::handle),
                                                         Some(payments::mock_method::parse_get_utxo_response::handle),
                                                         Some(payments::mock_method::build_payment_req::handle),
                                                         Some(payments::mock_method::parse_payment_response::handle),
                                                         Some(payments::mock_method::build_mint_req::handle),
                                                         Some(payments::mock_method::build_set_txn_fees_req::handle),
                                                         Some(payments::mock_method::build_get_txn_fees_req::handle),
                                                         Some(payments::mock_method::parse_get_txn_fees_response::handle),
            ).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();

            let all_addresses = payments::list_payment_addresses(wallet_handle).unwrap();

            let vec: Vec<String> = serde_json::from_str(&all_addresses).unwrap();
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&TEST_RES_STRING.to_string()));

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::add_request_fees(wallet_handle,
                                                                   IDENTIFIER,
                                                                   EMPTY_OBJECT,
                                                                   CORRECT_INPUTS,
                                                                   CORRECT_OUTPUTS,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (txn, method) = payments::add_request_fees(wallet_handle,
                                                           IDENTIFIER,
                                                           EMPTY_OBJECT,
                                                           CORRECT_INPUTS,
                                                           EMPTY_ARRAY,
            ).unwrap();

            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        fn parse_response_with_fees_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod build_get_utxo_request {
        use super::*;

        #[test]
        fn build_get_utxo_request_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_get_utxo_request::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_get_utxo_request(wallet_handle, IDENTIFIER, CORRECT_PAYMENT_ADDRESS).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_utxo_response {
        use super::*;

        #[test]
        fn parse_get_utxo_response_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_get_utxo_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_utxo_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_req_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_payment_req(wallet_handle,
                                                                    IDENTIFIER,
                                                                    CORRECT_INPUTS,
                                                                    CORRECT_OUTPUTS,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_req_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_mint_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let (req, payment_method) = payments::build_mint_req(wallet_handle,
                                                                 IDENTIFIER,
                                                                 CORRECT_OUTPUTS,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());
            assert_eq!(PAYMENT_METHOD_NAME, payment_method);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_set_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_for_generic_result() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let req = payments::build_get_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
            ).unwrap();

            assert_eq!(req, TEST_RES_STRING.to_string());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::Success, TEST_RES_STRING);

            let res_plugin = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            assert_eq!(res_plugin, TEST_RES_STRING);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works_for_no_first_method() {
            TestUtils::cleanup_storage();

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
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidParam3);

            TestUtils::cleanup_storage();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works_for_non_existent_plugin() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME).unwrap_err();

            assert_eq!(res, ErrorCode::PaymentUnknownMethodError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_payment_address_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::create_payment_address(wallet_handle + 1, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME).unwrap_err();

            assert_eq!(res, ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_payment_address_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::create_payment_address::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_works_for_nonexistent_wallet() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::list_payment_addresses(wallet_handle + 1).unwrap_err();

            assert_eq!(err, ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_non_existent_plugin_name() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 INPUTS_UNKNOWN_METHOD,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentUnknownMethodError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_empty_inputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 EMPTY_ARRAY,
                                                 CORRECT_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_no_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 r#"["pay"]"#,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 INCOMPATIBLE_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentIncompatibleMethodsError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_inputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 INCOMPATIBLE_INPUTS,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentIncompatibleMethodsError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_with_inputs_and_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1"]"#,
                                                 r#"[{"paymentAddress": "pay:null2:1", "amount":1, "extra":"1"}]"#,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentIncompatibleMethodsError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_malformed_input() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1:1", 1]"#,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_incorrect_payment_address() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 r#"["pay:null1"]"#,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle + 1,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::add_request_fees(wallet_handle,
                                                 INVALID_IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }


        #[test]
        fn add_request_fees_works_for_several_equal_inputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 EQUAL_INPUTS,
                                                 EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_equal_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let err = payments::add_request_fees(wallet_handle,
                                                 IDENTIFIER,
                                                 EMPTY_OBJECT,
                                                 CORRECT_INPUTS,
                                                 EQUAL_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::add_request_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::add_request_fees(wallet_handle, IDENTIFIER, EMPTY_OBJECT, CORRECT_INPUTS, CORRECT_OUTPUTS).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_nonexistent_plugin() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let err = payments::parse_response_with_fees(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentUnknownMethodError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn parse_response_with_fees_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_response_with_fees::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod build_get_utxo_request {
        use super::*;

        #[test]
        pub fn build_get_utxo_request_works_for_nonexistent_plugin() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_get_utxo_request(wallet_handle, IDENTIFIER, "pay:null1:test").unwrap_err();

            assert_eq!(err, ErrorCode::PaymentUnknownMethodError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn build_get_utxo_request_works_for_malformed_payment_address() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_get_utxo_request(wallet_handle, IDENTIFIER, "pay:null1").unwrap_err();

            assert_eq!(err, ErrorCode::PaymentIncompatibleMethodsError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn build_get_utxo_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_get_utxo_request(wallet_handle + 1, IDENTIFIER, CORRECT_PAYMENT_ADDRESS).unwrap_err();
            assert_eq!(err, ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn build_get_utxo_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_get_utxo_request(wallet_handle, INVALID_IDENTIFIER, CORRECT_PAYMENT_ADDRESS).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_get_utxo_request_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_get_utxo_request::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_utxo_request(wallet_handle,
                                                       IDENTIFIER,
                                                       CORRECT_PAYMENT_ADDRESS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_utxo_request {
        use super::*;

        #[test]
        pub fn parse_get_utxo_response_works_for_nonexistent_plugin() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let err = payments::parse_get_utxo_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap_err();

            assert_eq!(err, ErrorCode::PaymentUnknownMethodError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn parse_get_utxo_response_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_get_utxo_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_utxo_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_payment_req(invalid_wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS);

            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  INVALID_IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_empty_inputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  EMPTY_ARRAY,
                                                  CORRECT_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let err = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  EMPTY_ARRAY,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  INPUTS_UNKNOWN_METHOD,
                                                  OUTPUTS_UNKNOWN_METHOD);
            assert_eq!(ErrorCode::PaymentUnknownMethodError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_input_payment_address() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let inputs = r#"["pay:null"]"#;
            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  inputs,
                                                  CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_payment_methods() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  INCOMPATIBLE_INPUTS,
                                                  CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::PaymentIncompatibleMethodsError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_output_payment_methods() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  INCOMPATIBLE_OUTPUTS);
            assert_eq!(ErrorCode::PaymentIncompatibleMethodsError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_output_payment_methods() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let inputs = r#"["pay:PAYMENT_METHOD_1:1"]"#;
            let outputs = r#"[{"paymentAddress": "pay:PAYMENT_METHOD_2:1", "amount": 1}]"#;

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  inputs,
                                                  outputs);

            assert_eq!(ErrorCode::PaymentIncompatibleMethodsError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_inputs_format() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  INPUTS_INVALID_FORMAT,
                                                  CORRECT_OUTPUTS);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_outputs_format() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  OUTPUTS_INVALID_FORMAT);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_several_equal_inputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  EQUAL_INPUTS,
                                                  CORRECT_OUTPUTS,
            );

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_several_equal_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  EQUAL_OUTPUTS,
            );

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_payment_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_payment_req(wallet_handle,
                                                  IDENTIFIER,
                                                  CORRECT_INPUTS,
                                                  CORRECT_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let res = payments::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                       PAYMENT_RESPONSE);

            assert_eq!(ErrorCode::PaymentUnknownMethodError, res.unwrap_err());

            TestUtils::cleanup_storage();
        }

        #[test]
        fn parse_payment_response_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_payment_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_payment_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               EMPTY_ARRAY);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               OUTPUTS_UNKNOWN_METHOD);

            assert_eq!(ErrorCode::PaymentUnknownMethodError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = payments::build_mint_req(invalid_wallet_handle,
                                               IDENTIFIER,
                                               CORRECT_OUTPUTS);

            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_mint_req(wallet_handle,
                                               INVALID_IDENTIFIER,
                                               CORRECT_OUTPUTS);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_outputs_format() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               OUTPUTS_INVALID_FORMAT);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_output_payment_address() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let outputs = r#"[{"paymentAddress": "pay:null", "amount":1, "extra":"1"}]"#;

            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               outputs);

            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_several_equal_outputs() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               EQUAL_OUTPUTS);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_incompatible_output_payment_methods() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let res = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               INCOMPATIBLE_OUTPUTS);

            assert_eq!(ErrorCode::PaymentIncompatibleMethodsError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            payments::mock_method::build_mint_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_mint_req(wallet_handle,
                                               IDENTIFIER,
                                               CORRECT_OUTPUTS,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       WRONG_PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_eq!(res.unwrap_err(), ErrorCode::PaymentUnknownMethodError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_set_txn_fees_req(invalid_wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       INVALID_IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES);

            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_fees_format() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let fees = r#"[txnType1:1, txnType2:2]"#;

            let res = payments::build_set_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
                                                       fees);

            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_set_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_set_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
                                                       CORRECT_FEES,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let invalid_wallet_handle = wallet_handle + 1;

            let res = payments::build_get_txn_fees_req(invalid_wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME);

            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_submitter_did() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       INVALID_IDENTIFIER,
                                                       PAYMENT_METHOD_NAME);

            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_get_txn_fees_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = payments::build_get_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       WRONG_PAYMENT_METHOD_NAME);

            assert_eq!(res.unwrap_err(), ErrorCode::PaymentUnknownMethodError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_get_txn_fees_request_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::build_get_txn_fees_req::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::build_get_txn_fees_req(wallet_handle,
                                                       IDENTIFIER,
                                                       PAYMENT_METHOD_NAME,
            ).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();

            let res = payments::parse_get_txn_fees_response(WRONG_PAYMENT_METHOD_NAME,
                                                            GET_TXN_FEES_RESPONSE);

            assert_eq!(res.unwrap_err(), ErrorCode::PaymentUnknownMethodError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn parse_get_txn_fees_response_works_for_generic_error() {
            TestUtils::cleanup_storage();
            payments::mock_method::init();
            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            payments::mock_method::parse_get_txn_fees_response::inject_mock(ErrorCode::WalletAccessFailed, "");

            let err = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap_err();

            assert_eq!(err, ErrorCode::WalletAccessFailed);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }
}
