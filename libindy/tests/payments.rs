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

use nullpaymentplugin::payments::payment_callbacks::nullpayment_init;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use indy::api::ErrorCode;
use indy::api::payments::*;
use std::ffi::CString;

#[test]
fn create_payment_method() {
    extern fn cb (cmd_handle: i32, err: nullpaymentplugin::payments::ErrorCode) {

    }
    nullpayment_init(1, Some(cb));
    TestUtils::cleanup_storage();
    let wallet = WalletUtils::create_and_open_wallet("WALLET", None);




    assert_eq!(res.0, ErrorCode::Success);
    assert!(res.1.starts_with("pay:null"))
}