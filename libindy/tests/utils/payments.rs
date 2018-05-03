use indy::api::ErrorCode;
use indy::api::payments::*;
use utils::callback::CallbackUtils;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;

pub struct PaymentsUtils {}

impl PaymentsUtils {
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
}