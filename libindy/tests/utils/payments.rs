use indy::api::ErrorCode;
use indy::api::payments::*;
use utils::callback::CallbackUtils;

use std::collections::VecDeque;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::{Once, ONCE_INIT, Mutex};

pub struct PaymentsUtils {}

#[macro_export]
macro_rules! mocked_handler {
    ($first_param_name: ident: $first_param_type: ty $(, $param_name: ident: $param_type: ty)*) => (
        use super::*;

        lazy_static! {
          static ref INJECTIONS: Mutex<VecDeque<(ErrorCode, CString)>> = Default::default();
        }

        pub extern fn handle(cmd_handle: i32,
                                    $first_param_name: $first_param_type,
                                    $($param_name: $param_type,)*
                                    cb: Option<IndyPaymentCallback>) -> ErrorCode {

            let cb = cb.unwrap_or_else(|| {
                panic!("Null passed as callback!")
            });

            if let Ok(mut injections) = INJECTIONS.lock() {
                if let Some((err, res)) = injections.pop_front() {
                    return (cb)(cmd_handle, err, res.as_ptr());
                }
            } else {
                panic!("Can't lock injections mutex");
            }

            panic!("No injections left!");
        }

        pub fn inject_mock(err: ErrorCode, res: &str) {
            if let Ok(mut injections) = INJECTIONS.lock() {
                let res = CString::new(res).unwrap();
                injections.push_back((err, res))
            } else {
                panic!("Can't lock injections mutex");
            }
        }

        pub fn clear_mocks() {
            if let Ok(mut injections) = INJECTIONS.lock() {
                injections.clear();
            } else {
                panic!("Can't lock injections mutex");
            }
        }
    )
}

type IndyPaymentCallback = extern fn(command_handle_: i32,
                                     err: ErrorCode,
                                     payment_address: *const c_char) -> ErrorCode;

lazy_static! {
        static ref CREATE_PAYMENT_METHOD_INIT: Once = ONCE_INIT;
}

pub mod mock_method {
    use super::*;

    pub fn init() {
        CREATE_PAYMENT_METHOD_INIT.call_once(|| {
            let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec();
            let payment_method_name = CString::new("null").unwrap();
            indy_register_payment_method(cmd_handle,
                                         payment_method_name.as_ptr(),
                                         Some(create_payment_address::handle),
                                         Some(add_request_fees::handle),
                                         Some(parse_response_with_fees::handle),
                                         Some(build_get_utxo_request::handle),
                                         Some(parse_get_utxo_response::handle),
                                         Some(build_payment_req::handle),
                                         Some(parse_payment_response::handle),
                                         Some(build_mint_req::handle),
                                         Some(build_set_txn_fees_req::handle),
                                         Some(build_get_txn_fees_req::handle),
                                         Some(parse_get_txn_fees_response::handle),
                                         cb,
            );

            receiver.recv().unwrap();
        });
    }

    pub mod create_payment_address {
        mocked_handler!(_wallet_handle: i32, _config: *const c_char);
    }

    pub mod add_request_fees {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _req_json: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char);
    }

    pub mod parse_response_with_fees {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_get_utxo_request {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _payment_address: *const c_char);
    }

    pub mod parse_get_utxo_response {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_payment_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char);
    }

    pub mod parse_payment_response {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_mint_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _outputs_json: *const c_char);
    }

    pub mod build_set_txn_fees_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _fees_json: *const c_char);
    }

    pub mod build_get_txn_fees_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char);
    }

    pub mod parse_get_txn_fees_response {
        mocked_handler!(_resp_json: *const c_char);
    }
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

    let err = indy_register_payment_method(cmd_handle,
                                           payment_method_name.as_ptr(),
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

pub fn add_request_fees(wallet_handle: i32, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

    let req_json = CString::new(req_json).unwrap();
    let inputs_json = CString::new(inputs_json).unwrap();
    let outputs_json = CString::new(outputs_json).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_add_request_fees(
        cmd_handle,
        wallet_handle,
        submitter_did.as_ptr(),
        req_json.as_ptr(),
        inputs_json.as_ptr(),
        outputs_json.as_ptr(),
        cb,
    );

    super::results::result_to_string_string(err, receiver)
}

pub fn build_get_utxo_request(wallet_handle: i32, submitter_did: &str, payment_address: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

    let payment_address = CString::new(payment_address).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_get_utxo_request(cmd_handle,
                                          wallet_handle,
                                          submitter_did.as_ptr(),
                                          payment_address.as_ptr(),
                                          cb,
    );

    super::results::result_to_string_string(err, receiver)
}

pub fn build_payment_req(wallet_handle: i32, submitter_did: &str, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

    let inputs_json = CString::new(inputs_json).unwrap();
    let outputs_json = CString::new(outputs_json).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_payment_req(cmd_handle,
                                     wallet_handle,
                                     submitter_did.as_ptr(),
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

pub fn build_mint_req(wallet_handle: i32, submitter_did: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

    let outputs_json = CString::new(outputs_json).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_mint_req(cmd_handle,
                                  wallet_handle,
                                  submitter_did.as_ptr(),
                                  outputs_json.as_ptr(),
                                  cb,
    );

    super::results::result_to_string_string(err, receiver)
}

pub fn build_set_txn_fees_req(wallet_handle: i32, submitter_did: &str, payment_method: &str, fees_json: &str) -> Result<String, ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

    let payment_method = CString::new(payment_method).unwrap();
    let fees_json = CString::new(fees_json).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_set_txn_fees_req(cmd_handle,
                                          wallet_handle,
                                          submitter_did.as_ptr(),
                                          payment_method.as_ptr(),
                                          fees_json.as_ptr(),
                                          cb,
    );

    super::results::result_to_string(err, receiver)
}

pub fn build_get_txn_fees_req(wallet_handle: i32, submitter_did: &str, payment_method: &str) -> Result<String, ErrorCode> {
    let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

    let payment_method = CString::new(payment_method).unwrap();
    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_get_txn_fees_req(cmd_handle,
                                          wallet_handle,
                                          submitter_did.as_ptr(),
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
