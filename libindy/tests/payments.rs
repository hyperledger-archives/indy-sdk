extern crate indy;

use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

use indy::api::ErrorCode;
use utils::payment_method::*;
use utils::payments::PaymentsUtils;
use utils::test::TestUtils;
use utils::constants::*;
use utils::wallet::WalletUtils;
use utils::types::Utxo;

use serde_json::{from_str, Value};

use std::collections::HashMap;

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
static INCOMPATIBLE_OUTPUTS: &str = r#"[{"paymentAddress": "pay:PAYMENT_METHOD_1:1"}, {"paymentAddress": "pay:PAYMENT_METHOD_2:1"}]"#;
static CORRECT_FEES: &str = r#"{"txnType1":1, "txnType2":2}"#;
static PAYMENT_RESPONSE: &str = r#"{"reqId":1, "utxos":[{"input": "pay:null:1", "amount":1, "extra":"1"}, {"input": "pay:null:2", "amount":2, "extra":"2"}]}"#;
static GET_TXN_FEES_RESPONSE: &str = r#"{"reqId":1, fees:{"txnType1":1, "txnType2":2}}"#;

mod high_cases {
    use super::*;

    mod register_payment_method {
        use super::*;

        #[test]
        fn register_payment_method_works() {
            TestUtils::cleanup_storage();

            let _res = PaymentsUtils::register_payment_method("register_payment_method_works",
                                                             Some(create_payment_address_stub),
                                                             Some(add_request_fees_stub),
                                                             Some(parse_response_with_fees),
                                                             Some(build_get_utxo_request_stub),
                                                             Some(parse_get_utxo_response_stub),
                                                             Some(build_payment_req_stub),
                                                             Some(parse_payment_response_stub),
                                                             Some(build_mint_req_stub),
                                                             Some(build_set_txn_fees_request_stub),
                                                             Some(build_get_txn_fees_request_stub),
                                                             Some(parse_get_txn_fees_response_stub),
            ).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_works() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let res = PaymentsUtils::create_payment_address(wallet_handle, EMPTY_OBJECT, PAYMENT_METHOD_NAME).unwrap();
            assert!(res.starts_with("pay:null"));
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address_works() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let cfg = EMPTY_OBJECT;
            let payment_method = PAYMENT_METHOD_NAME;
            let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap();

            let all_addresses = PaymentsUtils::list_payment_addresses(wallet_handle).unwrap();
            let vec: Vec<Value> = from_str(all_addresses.as_str()).unwrap();
            let size_before = vec.len();

            let vec: Vec<String> = vec.iter().filter_map(|val| val.as_str()).map(|s| s.to_string()).collect();

            assert_eq!(vec.len(), size_before);
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&res));
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_method_works_for_inputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(wallet_handle,
                                                                EMPTY_OBJECT,
                                                                CORRECT_INPUTS,
                                                                EMPTY_ARRAY,
            ).unwrap();
            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, EMPTY_OBJECT);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_method_works_for_outputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(wallet_handle,
                                                                EMPTY_OBJECT,
                                                                EMPTY_ARRAY,
                                                                CORRECT_OUTPUTS,
            ).unwrap();
            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, EMPTY_OBJECT);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_method_works_for_inputs_and_outputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(wallet_handle,
                                                                EMPTY_OBJECT,
                                                                CORRECT_INPUTS,
                                                                CORRECT_OUTPUTS,
            ).unwrap();
            assert_eq!(method, PAYMENT_METHOD_NAME);
            assert_eq!(txn, EMPTY_OBJECT);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        fn parse_response_with_fees_works() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let utxo = PaymentsUtils::parse_response_with_fees(PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap();

            assert_eq!(utxo, CORRECT_OUTPUTS);

            TestUtils::cleanup_storage();
        }
    }

    mod build_get_utxo_request {
        use super::*;

        #[test]
        fn build_get_utxo_request_works() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let payment_address = "pay:null:test";
            let (_, method) = PaymentsUtils::build_get_utxo_request(wallet_handle, payment_address).unwrap();
            assert_eq!("null".to_string(), method);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_utxo_response {
        use super::*;

        #[test]
        fn parse_get_utxo_response_works() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let utxo = PaymentsUtils::parse_get_utxo_response(PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap();
            let _res: Vec<Utxo> = serde_json::from_str(utxo.as_str()).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let (payment_req_json, payment_method) = PaymentsUtils::build_payment_req(wallet_handle,
                                                                                      CORRECT_INPUTS,
                                                                                      CORRECT_OUTPUTS).unwrap();

            let payment_req = serde_json::from_str::<serde_json::Value>(&payment_req_json).unwrap();
            assert!(payment_req.is_object());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_empty_inputs() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let (payment_req_json, payment_method) = PaymentsUtils::build_payment_req(wallet_handle,
                                                                                      EMPTY_ARRAY,
                                                                                      CORRECT_OUTPUTS).unwrap();

            let payment_req = serde_json::from_str::<serde_json::Value>(&payment_req_json).unwrap();
            assert!(payment_req.is_object());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let (payment_req_json, payment_method) = PaymentsUtils::build_payment_req(wallet_handle,
                                                                                      CORRECT_INPUTS,
                                                                                      EMPTY_ARRAY).unwrap();

            let payment_req = serde_json::from_str::<serde_json::Value>(&payment_req_json).unwrap();
            assert!(payment_req.is_object());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let res = PaymentsUtils::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                            PAYMENT_RESPONSE);
            assert_eq!(ErrorCode::UnknownPaymentMethod, res.unwrap_err());

            TestUtils::cleanup_storage();
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let (mint_req_json, payment_method) = PaymentsUtils::build_mint_req(wallet_handle,
                                                                                CORRECT_OUTPUTS).unwrap();

            let mint_req = serde_json::from_str::<serde_json::Value>(&mint_req_json).unwrap();
            assert!(mint_req.is_object());
            assert_eq!(payment_method, PAYMENT_METHOD_NAME);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let set_txn_fees_json = PaymentsUtils::build_set_txn_fees_req(wallet_handle,
                                                                          PAYMENT_METHOD_NAME,
                                                                          CORRECT_FEES).unwrap();

            let set_txn_fees = serde_json::from_str::<serde_json::Value>(&set_txn_fees_json).unwrap();
            assert!(set_txn_fees.is_object());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let set_txn_fees_json = PaymentsUtils::build_get_txn_fees_req(wallet_handle,
                                                                          PAYMENT_METHOD_NAME).unwrap();

            let set_txn_fees: serde_json::Value = serde_json::from_str(&set_txn_fees_json).unwrap();
            assert!(set_txn_fees.is_object());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let fees_json = PaymentsUtils::parse_get_txn_fees_response(PAYMENT_METHOD_NAME,
                                                                       GET_TXN_FEES_RESPONSE).unwrap();

            let _fees: HashMap<String, i64> = serde_json::from_str(&fees_json).unwrap();

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

            let err = PaymentsUtils::register_payment_method(PAYMENT_METHOD_NAME,
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
        fn create_payment_address_works_for_non_existant_plugin() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let res = PaymentsUtils::create_payment_address(wallet_handle, EMPTY_OBJECT, WRONG_PAYMENT_METHOD_NAME).unwrap_err();
            assert_eq!(res, ErrorCode::UnknownPaymentMethod);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_works_for_nonexistant_wallet() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::list_payment_addresses(wallet_handle + 1).unwrap_err();
            assert_eq!(err, ErrorCode::WalletInvalidHandle);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_works_for_non_existant_plugin_name() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      r#"["pay:null1:1"]"#,
                                                      EMPTY_ARRAY,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::UnknownPaymentMethod);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_no_method() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      EMPTY_ARRAY,
                                                      EMPTY_ARRAY,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_outputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      EMPTY_ARRAY,
                                                      r#"[{"paymentAddress": "pay:null1:1", "amount":1, "extra":"1"}, {"paymentAddress": "pay:null2:2", "amount":2, "extra":"2"}]"#,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_in_inputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      r#"["pay:null1:1", "pay:null2:2"]"#,
                                                      EMPTY_ARRAY,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_several_methods_with_inputs_and_outputs() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      r#"["pay:null1:1"]"#,
                                                      r#"[{"paymentAddress": "pay:null2:1", "amount":1, "extra":"1"}]"#,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn add_request_fees_works_for_malformed_input() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
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

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let err = PaymentsUtils::add_request_fees(wallet_handle,
                                                      EMPTY_OBJECT,
                                                      r#"["pay:null1"]"#,
                                                      EMPTY_ARRAY,
            ).unwrap_err();
            assert_eq!(err, ErrorCode::CommonInvalidStructure);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod parse_response_with_fees {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_nonexistant_plugin () {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let err = PaymentsUtils::parse_response_with_fees(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap_err();

            assert_eq!(err, ErrorCode::UnknownPaymentMethod);

            TestUtils::cleanup_storage();
        }
    }

    mod build_get_utxo_request {
        use super::*;

        #[test]
        pub fn build_get_utxo_request_works_for_nonexistant_plugin() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let err = PaymentsUtils::build_get_utxo_request(wallet_handle, "pay:null1:test").unwrap_err();
            assert_eq!(err, ErrorCode::UnknownPaymentMethod);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn build_get_utxo_request_works_for_malformed_payment_address() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let err = PaymentsUtils::build_get_utxo_request(wallet_handle, "pay:null1").unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_utxo_request {
        use super::*;

        #[test]
        pub fn parse_get_utxo_response_works_for_nonexistant_plugin() {
            TestUtils::cleanup_storage();

            PaymentsUtils::init_nullpay_plugin();
            let err = PaymentsUtils::parse_get_utxo_response(WRONG_PAYMENT_METHOD_NAME, CORRECT_OUTPUTS).unwrap_err();
            assert_eq!(err, ErrorCode::UnknownPaymentMethod);

            TestUtils::cleanup_storage();
        }
    }
    mod payment_request {
        use super::*;

        #[test]
        fn build_payment_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = PaymentsUtils::build_payment_req(invalid_wallet_handle,
                                                       CORRECT_INPUTS,
                                                       CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_unknown_input_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       INPUTS_UNKNOWN_METHOD,
                                                       EMPTY_ARRAY);
            assert_eq!(ErrorCode::UnknownPaymentMethod, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_unknown_output_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       EMPTY_ARRAY,
                                                       OUTPUTS_UNKNOWN_METHOD);
            assert_eq!(ErrorCode::UnknownPaymentMethod, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_input_payment_address() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let inputs = r#"["pay:null"]"#;
            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       inputs,
                                                       CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_payment_methods() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       INCOMPATIBLE_INPUTS,
                                                       EMPTY_ARRAY);
            assert_eq!(ErrorCode::IncompatiblePaymentError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_output_payment_methods() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       EMPTY_ARRAY,
                                                       INCOMPATIBLE_OUTPUTS);
            assert_eq!(ErrorCode::IncompatiblePaymentError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_incompatible_input_output_payment_methods() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let inputs = r#"["pay:PAYMENT_METHOD_1:1"]"#;
            let outputs = r#"[{"paymentAddress": "pay:PAYMENT_METHOD_2:1"}]"#;

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       inputs,
                                                       outputs);
            assert_eq!(ErrorCode::IncompatiblePaymentError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_inputs_format() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       INPUTS_INVALID_FORMAT,
                                                       CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_payment_request_works_for_invalid_outputs_format() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_payment_req(wallet_handle,
                                                       CORRECT_INPUTS,
                                                       OUTPUTS_INVALID_FORMAT);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        fn parse_payment_response_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let res = PaymentsUtils::parse_payment_response(WRONG_PAYMENT_METHOD_NAME,
                                                            PAYMENT_RESPONSE);
            assert_eq!(ErrorCode::UnknownPaymentMethod, res.unwrap_err());

            TestUtils::cleanup_storage();
        }
    }

    mod mint_request {
        use super::*;

        #[test]
        fn build_mint_request_works_for_empty_outputs() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_mint_req(wallet_handle,
                                                    EMPTY_ARRAY);
            assert_eq!(ErrorCode::IncompatiblePaymentError, res.unwrap_err()); // TODO: CommonInvalidStructure looks more appropriate here

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_mint_req(wallet_handle,
                                                    OUTPUTS_UNKNOWN_METHOD);
            assert_eq!(ErrorCode::UnknownPaymentMethod, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = PaymentsUtils::build_mint_req(invalid_wallet_handle,
                                                    CORRECT_OUTPUTS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_outputs_format() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_mint_req(wallet_handle,
                                                    OUTPUTS_INVALID_FORMAT);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_invalid_output_payment_address() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let outputs = r#"[{"paymentAddress": "pay:null", "amount":1, "extra":"1"}]"#;

            let res = PaymentsUtils::build_mint_req(wallet_handle,
                                                    outputs);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_mint_request_works_for_incompatible_output_payment_methods() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_mint_req(wallet_handle,
                                                    INCOMPATIBLE_OUTPUTS);
            assert_eq!(ErrorCode::IncompatiblePaymentError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod set_txn_fees_request {
        use super::*;

        #[test]
        fn build_set_txn_fees_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_set_txn_fees_req(wallet_handle,
                                                            WRONG_PAYMENT_METHOD_NAME,
                                                            CORRECT_FEES);
            assert_eq!(res.unwrap_err(), ErrorCode::UnknownPaymentMethod);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = PaymentsUtils::build_set_txn_fees_req(invalid_wallet_handle,
                                                            PAYMENT_METHOD_NAME,
                                                            CORRECT_FEES);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_set_txn_fees_request_works_for_invalid_fees_format() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let fees = r#"[txnType1:1, txnType2:2]"#;
            let res = PaymentsUtils::build_set_txn_fees_req(wallet_handle,
                                                            PAYMENT_METHOD_NAME,
                                                            fees);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod get_txn_fees_request {
        use super::*;

        #[test]
        fn build_get_txn_fees_request_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = PaymentsUtils::build_get_txn_fees_req(invalid_wallet_handle,
                                                            PAYMENT_METHOD_NAME);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn build_get_txn_fees_request_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            let res = PaymentsUtils::build_get_txn_fees_req(wallet_handle,
                                                            WRONG_PAYMENT_METHOD_NAME);
            assert_eq!(res.unwrap_err(), ErrorCode::UnknownPaymentMethod);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod parse_get_txn_fees_response {
        use super::*;

        #[test]
        fn parse_get_txn_fees_response_works_for_unknown_payment_method() {
            TestUtils::cleanup_storage();
            PaymentsUtils::init_nullpay_plugin();

            let res = PaymentsUtils::parse_get_txn_fees_response(WRONG_PAYMENT_METHOD_NAME,
                                                                 GET_TXN_FEES_RESPONSE);
            assert_eq!(res.unwrap_err(), ErrorCode::UnknownPaymentMethod);

            TestUtils::cleanup_storage();
        }
    }


    //    #[test]
    //    fn create_payment_address_plugworks_for_error() {
    //        PaymentsUtils::create_payment_method();
    //        PaymentsUtils::inject_create_address_error(701);
    //        let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
    //        let cfg = CString::new ("{}").unwrap();
    //        let payment_method = CString::new("null").unwrap();
    //        let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap_err();
    //        assert_eq!(res as i32, 701);
    //
    //    }
}
