use libindy::ErrorCode;
use libindy::ledger;
use libindy::payments::IndyPaymentCallback;
use utils::rand;

use std::collections::VecDeque;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;

#[macro_export]
macro_rules! mocked_handler {
    ($first_param_name: ident: $first_param_type: ty $(, $param_name: ident: $param_type: ty)*) => (
        lazy_static! {
          static ref INJECTIONS: Mutex<VecDeque<(ErrorCode, CString)>> = Default::default();
        }

        pub extern fn handle_mocked(cmd_handle: i32,
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

            handle(cmd_handle, $first_param_name, $($param_name,)* cb)
        }

        pub fn inject_mock(err: ErrorCode, res: *const c_char) {
            if let Ok(mut injections) = INJECTIONS.lock() {
                injections.push_back((err, CString::from(unsafe { CStr::from_ptr(res) })))
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

pub static PAYMENT_METHOD_NAME: &str = "null";

pub mod create_payment_address {
    use super::*;

    mocked_handler!(wallet_handle: i32, config: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, _config: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = CString::new(format!("pay:null:{}", rand::get_rand_string(15))).unwrap();
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res.as_ptr())
    }
}

pub mod add_request_fees {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char, req_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, _submitter_did: *const c_char, req_json: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = req_json;
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res)
    }
}

pub mod parse_response_with_fees {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = resp_json;
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res)
    }
}

pub mod build_get_utxo_request {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char, payment_address: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, _payment_address: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let res = CString::new(res).unwrap();
                cb(cmd_handle, ec, res.as_ptr());
            })
        )
    }
}

pub mod parse_get_utxo_response {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, _resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let utxo_example =
            format!(
                r#"[{{"input":"pov:null:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null:2", "amount":2, "extra":"{}"}}]"#,
                rand::get_rand_string(15),
                rand::get_rand_string(15)
            );
        let utxo_json = CString::new(utxo_example).unwrap();
        let ec = ErrorCode::Success;
        (cb)(cmd_handle, ec, utxo_json.as_ptr())
    }
}

pub mod build_payment_req {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, _inputs_json: *const c_char, _outputs_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let res = CString::new(res).unwrap();
                cb(cmd_handle, ec, res.as_ptr());
            })
        )
    }
}

pub mod parse_payment_response {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, _resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let payment_response_example =
            format!(
                r#"[{{"input":"pov:null_payment:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null_payment:2", "amount":2, "extra":"{}"}}]"#,
                rand::get_rand_string(15),
                rand::get_rand_string(15)
            );
        let res = CString::new(payment_response_example).unwrap();
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res.as_ptr())
    }
}

pub mod build_mint_req {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char, outputs_json: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, _outputs_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let res = CString::new(res).unwrap();
                cb(cmd_handle, ec, res.as_ptr());
            })
        )
    }
}

pub mod build_set_txn_fees_req {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char, fees_json: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, _submitter_did: *const c_char, fees_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = fees_json;
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res)
    }
}

pub mod build_get_txn_fees_req {
    use super::*;

    mocked_handler!(wallet_handle: i32, submitter_did: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let res = CString::new(res).unwrap();
                cb(cmd_handle, ec, res.as_ptr());
            })
        )
    }
}

pub mod parse_get_txn_fees_response {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, _resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = CString::new(
            r#"{"txnType1":1, "txnType2":2, "txnType3":3}"#
        ).unwrap();
        let err = ErrorCode::Success;
        (cb)(cmd_handle, err, res.as_ptr())
    }
}