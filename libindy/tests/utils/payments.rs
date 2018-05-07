extern crate nullpay;

use indy::api::ErrorCode;
use indy::api::payments::*;
use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use utils::callback::CallbackUtils;
use utils::test::TestUtils;
use self::nullpay::nullpay_init;

pub struct PaymentsUtils {}

lazy_static! {
        static ref CREATE_PAYMENT_METHOD_INIT: Once = ONCE_INIT;
}

impl PaymentsUtils {

    pub fn create_payment_method() {
        CREATE_PAYMENT_METHOD_INIT.call_once(|| {
            TestUtils::cleanup_storage();
            nullpay_init();
        });
    }

    pub fn create_payment_address(wallet_handle: i32, config: CString, payment_method: CString) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let errc = indy_create_payment_address(
            cmd_handle,
            wallet_handle,
            payment_method.as_ptr(),
            config.as_ptr(),
            cb
        );
        super::results::result_to_string(errc, receiver)
    }

    pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let ec = indy_list_payment_addresses(
            cmd_handle,
            wallet_handle,
            cb
        );
        super::results::result_to_string(ec, receiver)
    }

//    pub fn
}