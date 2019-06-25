extern crate futures;
extern crate indy_sys;

use indy::{IndyError, ErrorCode};
use indy::payments;
use self::futures::Future;
use self::indy_sys::{payments as payments_sys};

use std::collections::VecDeque;
use std::ffi::CString;
use super::libc::c_char;
use std::sync::{Once, ONCE_INIT, Mutex};

use utils::callback;

#[macro_export]
macro_rules! mocked_handler {
    ($first_param_name: ident: $first_param_type: ty $(, $param_name: ident: $param_type: ty)*) => (
        use super::*;

        lazy_static! {
          static ref INJECTIONS: Mutex<VecDeque<(i32, CString)>> = Default::default();
        }

        pub extern fn handle(cmd_handle: i32,
                                    $first_param_name: $first_param_type,
                                    $($param_name: $param_type,)*
                                    cb: Option<IndyPaymentCallback>) -> i32 {

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
                injections.push_back((err as i32, res))
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
                                     err: i32,
                                     payment_address: *const c_char) -> i32;

lazy_static! {
        static ref CREATE_PAYMENT_METHOD_INIT: Once = ONCE_INIT;
}

pub mod mock_method {
    use super::*;

    pub fn init() {
        CREATE_PAYMENT_METHOD_INIT.call_once(|| {
            let (receiver, cmd_handle, cb) = callback::_closure_to_cb_ec();
            let payment_method_name = CString::new("null").unwrap();
            unsafe {
                payments_sys::indy_register_payment_method(cmd_handle,
                                                       payment_method_name.as_ptr(),
                                                       Some(create_payment_address::handle),
                                                       Some(add_request_fees::handle),
                                                       Some(parse_response_with_fees::handle),
                                                       Some(build_get_payment_sources_request::handle),
                                                       Some(parse_get_payment_sources_response::handle),
                                                       Some(build_payment_req::handle),
                                                       Some(parse_payment_response::handle),
                                                       Some(build_mint_req::handle),
                                                       Some(build_set_txn_fees_req::handle),
                                                       Some(build_get_txn_fees_req::handle),
                                                       Some(parse_get_txn_fees_response::handle),
                                                       Some(build_verify_payment_req::handle),
                                                       Some(parse_verify_payment_response::handle),
                                                       cb,
                );
            }

            receiver.recv().unwrap();
        });
    }

    pub mod create_payment_address {
        mocked_handler!(_wallet_handle: i32, _config: *const c_char);
    }

    pub mod add_request_fees {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _req_json: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char, _extra: *const c_char);
    }

    pub mod parse_response_with_fees {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_get_payment_sources_request {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _payment_address: *const c_char);
    }

    pub mod parse_get_payment_sources_response {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_payment_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char, _extra: *const c_char);
    }

    pub mod parse_payment_response {
        mocked_handler!(_resp_json: *const c_char);
    }

    pub mod build_mint_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _outputs_json: *const c_char, _extra: *const c_char);
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

    pub mod build_verify_payment_req {
        mocked_handler!(_wallet_handle: i32, _submitter_did: *const c_char, _receipt: *const c_char);
    }

    pub mod parse_verify_payment_response {
        mocked_handler!(_resp_json: *const c_char);
    }
}

pub fn register_payment_method(payment_method_name: &str,
                               create_payment_address: Option<payments_sys::CreatePaymentAddressCB>,
                               add_request_fees: Option<payments_sys::AddRequestFeesCB>,
                               parse_response_with_fees: Option<payments_sys::ParseResponseWithFeesCB>,
                               build_get_payment_sources_request: Option<payments_sys::BuildGetPaymentSourcesRequestCB>,
                               parse_get_payment_sources_response: Option<payments_sys::ParseGetPaymentSourcesResponseCB>,
                               build_payment_req: Option<payments_sys::BuildPaymentReqCB>,
                               parse_payment_response: Option<payments_sys::ParsePaymentResponseCB>,
                               build_mint_req: Option<payments_sys::BuildMintReqCB>,
                               build_set_txn_fees_req: Option<payments_sys::BuildSetTxnFeesReqCB>,
                               build_get_txn_fees_req: Option<payments_sys::BuildGetTxnFeesReqCB>,
                               parse_get_txn_fees_response: Option<payments_sys::ParseGetTxnFeesResponseCB>,
                               build_verify_payment_req: Option<payments_sys::BuildVerifyPaymentReqCB>,
                               parse_verify_payment_response: Option<payments_sys::ParseVerifyPaymentResponseCB>,
) -> Result<(), ErrorCode> {
    let (receiver, cmd_handle, cb) = callback::_closure_to_cb_ec();

    let payment_method_name = CString::new(payment_method_name).unwrap();

    let err = unsafe {
        payments_sys::indy_register_payment_method(cmd_handle,
                                               payment_method_name.as_ptr(),
                                               create_payment_address,
                                               add_request_fees,
                                               parse_response_with_fees,
                                               build_get_payment_sources_request,
                                               parse_get_payment_sources_response,
                                               build_payment_req,
                                               parse_payment_response,
                                               build_mint_req,
                                               build_set_txn_fees_req,
                                               build_get_txn_fees_req,
                                               parse_get_txn_fees_response,
                                               build_verify_payment_req,
                                               parse_verify_payment_response,
                                               cb,
        )
    };

    super::results::result_to_empty(err, receiver)
}

pub fn create_payment_address(wallet_handle: i32, config: &str, payment_method: &str) -> Result<String, IndyError> {
    payments::create_payment_address(wallet_handle, payment_method, config).wait()
}

pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, IndyError> {
    payments::list_payment_addresses(wallet_handle).wait()
}

pub fn add_request_fees(wallet_handle: i32, submitter_did: Option<&str>, req_json: &str, inputs_json: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::add_request_fees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra).wait()
}

pub fn build_get_payment_sources_request(wallet_handle: i32, submitter_did: Option<&str>, payment_address: &str) -> Result<(String, String), IndyError> {
    payments::build_get_payment_sources_request(wallet_handle, submitter_did, payment_address).wait()
}

pub fn build_payment_req(wallet_handle: i32, submitter_did: Option<&str>, inputs_json: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::build_payment_req(wallet_handle, submitter_did, inputs_json, outputs_json, extra).wait()
}

pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_response_with_fees(payment_method, resp_json).wait()
}

pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_get_payment_sources_response(payment_method, resp_json).wait()
}

pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_payment_response(payment_method, resp_json).wait()
}

pub fn prepare_extra_with_acceptance_data(extra: Option<&str>,
                                          text: Option<&str>,
                                          version: Option<&str>,
                                          taa_digest: Option<&str>,
                                          acc_mech_type: &str,
                                          time_of_acceptance: u64) -> Result<String, IndyError> {
    payments::prepare_extra_with_acceptance_data(extra, text, version, taa_digest, acc_mech_type, time_of_acceptance).wait()
}

pub fn build_mint_req(wallet_handle: i32, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::build_mint_req(wallet_handle, submitter_did, outputs_json, extra).wait()
}

pub fn build_set_txn_fees_req(wallet_handle: i32, submitter_did: Option<&str>, payment_method: &str, fees_json: &str) -> Result<String, IndyError> {
    payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, fees_json).wait()
}

pub fn build_get_txn_fees_req(wallet_handle: i32, submitter_did: Option<&str>, payment_method: &str) -> Result<String, IndyError> {
    payments::build_get_txn_fees_req(wallet_handle, submitter_did, payment_method).wait()
}

pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_get_txn_fees_response(payment_method, resp_json).wait()
}

pub fn build_verify_payment_req(wallet_handle: i32, submitter_did: Option<&str>, receipt: &str) -> Result<(String, String), IndyError> {
    payments::build_verify_payment_req(wallet_handle, submitter_did, receipt).wait()
}

pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_verify_payment_response(payment_method, resp_json).wait()
}
