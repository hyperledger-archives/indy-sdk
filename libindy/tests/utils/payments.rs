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

    pub fn init_nullpay_plugin() {
        CREATE_PAYMENT_METHOD_INIT.call_once(|| {
            TestUtils::cleanup_storage();
            nullpay_init();
        });
    }

//    pub fn inject_create_address_error(error: ErrorCode) {
//        nullpay::create_payment_address::nullpay_injmock_create_payment_address(error, CString::new("").unwrap().as_ptr());
//    }

    pub fn create_payment_address(wallet_handle: i32, config: CString, payment_method: CString) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let err = indy_create_payment_address(
            cmd_handle,
            wallet_handle,
            payment_method.as_ptr(),
            config.as_ptr(),
            cb
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let err = indy_list_payment_addresses(
            cmd_handle,
            wallet_handle,
            cb
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn add_request_fees(wallet_handle: i32, req_json: CString, inputs_json: CString, outputs_json: CString) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();
        let err = indy_add_request_fees(
            cmd_handle,
            wallet_handle,
            req_json.as_ptr(),
            inputs_json.as_ptr(),
            outputs_json.as_ptr(),
            cb
        );
        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_get_utxo_request(wallet_handle: i32, payment_address: CString) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();
        let err = indy_build_get_utxo_request(
            cmd_handle,
            wallet_handle,
            payment_address.as_ptr(),
            cb
        );
        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_payment_req(wallet_handle: i32, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let inputs_json = CString::new(inputs_json).unwrap();
        let outputs_json = CString::new(outputs_json).unwrap();

        let err = indy_build_payment_req(cmd_handle,
                                         wallet_handle,
                                         inputs_json.as_ptr(),
                                         outputs_json.as_ptr(),
                                         cb
        );
        super::results::result_to_string_string(err, receiver)
    }
}