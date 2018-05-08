extern crate nullpay;

use indy::api::ErrorCode;
use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use utils::callback::CallbackUtils;
use self::nullpay::nullpay_init;
use indy::api::payments::*;

pub struct PaymentsUtils {}

lazy_static! {
        static ref CREATE_PAYMENT_METHOD_INIT: Once = ONCE_INIT;
}

impl PaymentsUtils {
    pub fn init_nullpay_plugin() {
        CREATE_PAYMENT_METHOD_INIT.call_once(|| {
            nullpay_init();
        });
    }

    pub fn register_payment_method(payment_method_name: &str,
                                   create_payment_address: Option<CreatePaymentAddressCB>,
                                   add_request_fees: Option<AddRequestFeesCB>,
                                   parse_response_with_fees: Option<ParseResponseWithFeesCB>,
                                   build_get_utxo_request: Option<BuildGetUTXORequestCB>,
                                   parse_get_utxo_response: Option<ParseGetUTXOResponseCB>,
                                   build_payment_req: Option<BuildPaymentReqCB>,
                                   parse_payment_response: Option<ParsePaymentResponseCB>,
                                   build_mint_req: Option<BuildMintReqCB>,
                                   build_set_txn_fees_req: Option<BuildSetTxnFeesReqCB>,
                                   build_get_txn_fees_req: Option<BuildGetTxnFeesReqCB>,
                                   parse_get_txn_fees_response: Option<ParseGetTxnFeesResponseCB>,
    ) -> Result<(), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec();
        let payment_method_name = CString::new(payment_method_name).unwrap();
        let payment_method_name = payment_method_name.as_ptr();
        let err = indy_register_payment_method(cmd_handle,
                                               payment_method_name,
                                               create_payment_address,
                                               add_request_fees,
                                               parse_response_with_fees,
                                               build_get_utxo_request,
                                               parse_get_utxo_response,
                                               build_payment_req,
                                               parse_payment_response,
                                               build_mint_req,
                                               build_set_txn_fees_req,
                                               build_get_txn_fees_req,
                                               parse_get_txn_fees_response,
                                               cb,
        );

        super::results::result_to_empty(err, receiver)
    }

    pub fn create_payment_address(wallet_handle: i32, config: &str, payment_method: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let config = CString::new(config).unwrap();
        let payment_method = CString::new(payment_method).unwrap();
        let err = indy_create_payment_address(
            cmd_handle,
            wallet_handle,
            payment_method.as_ptr(),
            config.as_ptr(),
            cb,
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let err = indy_list_payment_addresses(
            cmd_handle,
            wallet_handle,
            cb,
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn add_request_fees(wallet_handle: i32, req_json: &str, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();
        let req_json = CString::new(req_json).unwrap();
        let inputs_json = CString::new(inputs_json).unwrap();
        let outputs_json = CString::new(outputs_json).unwrap();
        let err = indy_add_request_fees(
            cmd_handle,
            wallet_handle,
            req_json.as_ptr(),
            inputs_json.as_ptr(),
            outputs_json.as_ptr(),
            cb,
        );
        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_get_utxo_request(wallet_handle: i32, payment_address: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();
        let payment_address = CString::new(payment_address).unwrap();
        let err = indy_build_get_utxo_request(
            cmd_handle,
            wallet_handle,
            payment_address.as_ptr(),
            cb,
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
                                         cb,
        );
        super::results::result_to_string_string(err, receiver)
    }

    pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = indy_parse_response_with_fees(cmd_handle,
                                                payment_method.as_ptr(),
                                                resp_json.as_ptr(),
                                                cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn parse_get_utxo_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = indy_parse_get_utxo_response(cmd_handle,
                                               payment_method.as_ptr(),
                                               resp_json.as_ptr(),
                                               cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = indy_parse_payment_response(cmd_handle,
                                              payment_method.as_ptr(),
                                              resp_json.as_ptr(),
                                              cb,
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn build_mint_req(wallet_handle: i32, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let outputs_json = CString::new(outputs_json).unwrap();

        let err = indy_build_mint_req(cmd_handle,
                                      wallet_handle,
                                      outputs_json.as_ptr(),
                                      cb,
        );
        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_set_txn_fees_req(wallet_handle: i32, payment_method: &str, fees_json: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let fees_json = CString::new(fees_json).unwrap();

        let err = indy_build_set_txn_fees_req(cmd_handle,
                                              wallet_handle,
                                              payment_method.as_ptr(),
                                              fees_json.as_ptr(),
                                              cb,
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_txn_fees_req(wallet_handle: i32, payment_method: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();

        let err = indy_build_get_txn_fees_req(cmd_handle,
                                              wallet_handle,
                                              payment_method.as_ptr(),
                                              cb,
        );
        super::results::result_to_string(err, receiver)
    }

    pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = indy_parse_get_txn_fees_response(cmd_handle,
                                                   payment_method.as_ptr(),
                                                   resp_json.as_ptr(),
                                                   cb,
        );
        super::results::result_to_string(err, receiver)
    }
}