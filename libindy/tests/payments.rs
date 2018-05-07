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

    #[test]
    fn create_payment_address() {
        PaymentsUtils::create_payment_method();
        let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
        let cfg = CString::new ("{}").unwrap();
        let payment_method = CString::new("null").unwrap();
        let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap();
        assert!(res.starts_with("pay:null"));
        let all_addresses = PaymentsUtils::list_payment_addresses(wallet_handle).unwrap();
        let vec : Vec<Value> = from_str(all_addresses.as_str()).unwrap();
        let size_before = vec.len();

        let vec: Vec<String> = vec.iter().filter_map(|val| val.as_str()).map(|s| s.to_string()).collect();

        assert_eq!(vec.len(), size_before);
        assert_eq!(vec.len(), 1);
        assert!(vec.contains(&res));
    }

}

mod medium_cases {
    use super::*;

    #[test]
    fn create_payment_address_in_non_existant_plugin() {
        PaymentsUtils::create_payment_method();
        let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
        let cfg = CString::new ("{}").unwrap();
        let payment_method = CString::new("null_payment_handler").unwrap();
        let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap_err();
        assert_eq!(res, ErrorCode::UnknownPaymentMethod);
    }
}