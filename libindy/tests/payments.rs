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

mod high_cases {
    use serde_json::from_str;
    use std::ffi::CString;
    use utils::wallet::WalletUtils;
    use utils::payments::PaymentsUtils;

    #[test]
    fn create_payment_address() {
        PaymentsUtils::create_payment_method();
        let wallet_handle = WalletUtils::create_and_open_wallet("WALLET", None).unwrap();
        let cfg = CString::new ("{}").unwrap();
        let payment_method = CString::new("null_payment_plugin").unwrap();
        let res = PaymentsUtils::create_payment_address(wallet_handle, cfg, payment_method).unwrap();
        assert!(res.starts_with("pay:null"));
        let all_addresses = PaymentsUtils::list_payment_addresses(wallet_handle).unwrap();
//        let vec = from_str(all_addresses.to_str()).unwrap();
//        let size_before = vec.len();
//
//        assert_eq!(vec.len(), 1);
//        assert!(vec.contains(res));
    }

}

mod medium_cases {
    use indy::api::ErrorCode;
    use std::ffi::CString;
    use utils::wallet::WalletUtils;
    use utils::payments::PaymentsUtils;

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