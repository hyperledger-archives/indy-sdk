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
extern crate log;

#[macro_use]
mod utils;

use indy::api::ErrorCode;
use utils::wallet::WalletUtils;
use utils::payments::PaymentsUtils;

use serde_json::{from_str, Value};
use std::ffi::CString;

mod high_cases {
    use super::*;

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let cfg = CString::new ("{}").unwrap();
            let payment_method = CString::new("null").unwrap();
            let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap();
            assert!(res.starts_with("pay:null"));
        }
    }

    mod list_payment_addresses {
        use super::*;

        #[test]
        fn list_payment_address() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let cfg = CString::new ("{}").unwrap();
            let payment_method = CString::new("null").unwrap();
            let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap();

            let all_addresses = PaymentsUtils::list_payment_addresses(wallet_handle).unwrap();
            let vec : Vec<Value> = from_str(all_addresses.as_str()).unwrap();
            let size_before = vec.len();

            let vec: Vec<String> = vec.iter().filter_map(|val| val.as_str()).map(|s| s.to_string()).collect();

            assert_eq!(vec.len(), size_before);
            assert_eq!(vec.len(), 1);
            assert!(vec.contains(&res));
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_with_method_in_inputs() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(
                wallet_handle,
                CString::new("{}").unwrap(),
                CString::new("[\"pay:null:1\", \"pay:null:2\"]").unwrap(),
                CString::new("[]").unwrap()
            ).unwrap();
            assert_eq!(method, "null");
            assert_eq!(txn, "{}");
        }

        #[test]
        fn add_request_fees_with_method_in_outputs() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(
                wallet_handle,
                CString::new("{}").unwrap(),
                CString::new("[]").unwrap(),
                CString::new(r#"[{"paymentAddress": "pay:null:1", "amount":1, "extra":"1"}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#).unwrap()
            ).unwrap();
            assert_eq!(method, "null");
            assert_eq!(txn, "{}");
        }

        #[test]
        fn add_request_fees_with_method_in_inputs_and_outputs() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let (txn, method) = PaymentsUtils::add_request_fees(
                wallet_handle,
                CString::new("{}").unwrap(),
                CString::new("[\"pay:null:1\", \"pay:null:2\"]").unwrap(),
                CString::new(r#"[{"paymentAddress": "pay:null:1", "amount":1, "extra":"1"}, {"paymentAddress": "pay:null:2", "amount":2, "extra":"2"}]"#).unwrap()
            ).unwrap();
            assert_eq!(method, "null");
            assert_eq!(txn, "{}");
        }
    }



    #[test]
    fn build_get_utxo_request() {
        PaymentsUtils::init_nullpay_plugin();
        let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
        let payment_address = CString::new("pay:null:test").unwrap();
        let (txn, method) = PaymentsUtils::build_get_utxo_request(wallet_handle, payment_address).unwrap();
        assert_eq!("null".to_string(), method);
    }
}

mod medium_cases {
    use super::*;

    mod create_payment_address {
        use super::*;

        #[test]
        fn create_payment_address_in_non_existant_plugin() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let cfg = CString::new ("{}").unwrap();
            let payment_method = CString::new("null_payment_handler").unwrap();
            let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap_err();
            assert_eq!(res, ErrorCode::UnknownPaymentMethod);
        }
    }

    mod list_payment_address {
        use super::*;

        #[test]
        fn list_payment_addresses_in_nonexistant_wallet() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap() + 1;
            let err = PaymentsUtils::list_payment_addresses(wallet_handle).unwrap_err();
            assert_eq!(err, ErrorCode::WalletInvalidHandle);
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        fn add_request_fees_with_no_method() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let err = PaymentsUtils::add_request_fees(
                wallet_handle,
                CString::new("{}").unwrap(),
                CString::new("[]").unwrap(),
                CString::new("[]").unwrap()
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
        }

        #[test]
        fn add_request_fees_with_several_methods_in_inputs() {
            PaymentsUtils::init_nullpay_plugin();
            let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
            let err = PaymentsUtils::add_request_fees(
                wallet_handle,
                CString::new("{}").unwrap(),
                CString::new("[\"pay:null1:1\", \"pay:null2:2\"]").unwrap(),
                CString::new("[]").unwrap()
            ).unwrap_err();
            assert_eq!(err, ErrorCode::IncompatiblePaymentError);
        }
    }

//    #[test]
//    fn create_payment_address_plugin_error() {
//        PaymentsUtils::create_payment_method();
//        PaymentsUtils::inject_create_address_error(701);
//        let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
//        let cfg = CString::new ("{}").unwrap();
//        let payment_method = CString::new("null").unwrap();
//        let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap_err();
//        assert_eq!(res as i32, 701);
//
//    }
}