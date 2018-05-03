extern crate nullpaymentplugin;
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

use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::payments::PaymentsUtils;
use indy::api::ErrorCode;
use std::ffi::CString;

mod high {
    use nullpaymentplugin::payments::payment_callbacks::nullpayment_init;

    #[test]
    fn create_payment_address() {
        extern fn cb(cmd_handle: i32, _err: nullpaymentplugin::payments::ErrorCode) {}
        nullpayment_init(1, Some(cb));
        TestUtils::cleanup_storage();
        let wallet = WalletUtils::create_and_open_wallet("WALLET", None);

        let res = match wallet {
            Ok(handle) => PaymentsUtils::create_payment_address(handle),
            Err(e) => Err(e)
        };
        assert_match!(Ok(_), res);
        let res = res.unwrap();
        assert!(res.starts_with("pay:null"));
    }

    #[test]
    fn create
}