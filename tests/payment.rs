extern crate rust_indy_sdk_wrapper as indy;
#[macro_use]
mod utils;

//use indy::ErrorCode;
//use indy::payments::Payment;
use indy::wallet::Wallet;

//use std::time::Duration;
//use std::sync::mpsc::channel;
//
//use utils::time_it_out;
use utils::constants::DEFAULT_CREDENTIALS;

mod low_tests {
    use super::*;

    #[test]
    fn create_payment_address_works() {
        let wallet_name = r#"{"id":"create_payment_address_works"}"#;
        safe_wallet_create!(wallet_name);
        let handle = Wallet::open(wallet_name, DEFAULT_CREDENTIALS).unwrap();

//        let payment_address = Payment::create_payment_address(handle, "sov", r#"{}"#).unwrap();
//
//        assert_eq!(payment_address.len(), 64);
//        assert!(payment_address.starts_with("pay:sov:"));

        wallet_cleanup!(handle, wallet_name);
    }
}
