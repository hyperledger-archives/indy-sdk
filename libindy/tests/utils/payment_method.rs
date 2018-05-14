use indy::api::ErrorCode;
use indy::api::ledger::indy_build_get_txn_request;
use super::rand_utils;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::ffi::CString;
use std::ffi::CStr;
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

type IndyPaymentCallback = extern fn(command_handle_: i32,
                                     err: ErrorCode,
                                     payment_address: *const c_char) -> ErrorCode;

type LedgerCallback = extern fn(command_handle_: i32,
                                     err: ErrorCode,
                                     payment_address: *const c_char);

pub mod create_payment_address {
    use super::*;

    mocked_handler!(wallet_handle: i32, config: *const c_char);

    fn handle(cmd_handle: i32, _wallet_handle: i32, _config: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let res = CString::new(format!("pay:null:{}", rand_utils::get_rand_string(15))).unwrap();
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
        indy_build_get_txn_request(cmd_handle, submitter_did, 1, Some(make_ledger_callback(cb, cmd_handle)))
    }
}

pub mod parse_get_utxo_response {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, _resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let utxo_example =
            format!(
                r#"[{{"input":"pov:null:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null:2", "amount":2, "extra":"{}"}}]"#,
                rand_utils::get_rand_string(15),
                rand_utils::get_rand_string(15)
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
        indy_build_get_txn_request(cmd_handle, submitter_did, 1, Some(make_ledger_callback(cb, cmd_handle)))
    }
}

pub mod parse_payment_response {
    use super::*;

    mocked_handler!(resp_json: *const c_char);

    fn handle(cmd_handle: i32, _resp_json: *const c_char, cb: IndyPaymentCallback) -> ErrorCode {
        let payment_response_example =
            format!(
                r#"[{{"input":"pov:null_payment:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null_payment:2", "amount":2, "extra":"{}"}}]"#,
                rand_utils::get_rand_string(15),
                rand_utils::get_rand_string(15)
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
        indy_build_get_txn_request(cmd_handle, submitter_did, 1, Some(make_ledger_callback(cb, cmd_handle)))
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
        indy_build_get_txn_request(cmd_handle, submitter_did, 1, Some(make_ledger_callback(cb, cmd_handle)))
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

fn make_ledger_callback(closure: IndyPaymentCallback, cmd_handle: i32) -> LedgerCallback {
    lazy_static! {
       static ref CALLBACKS: Mutex<HashMap<i32, Box<IndyPaymentCallback>>> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, res: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let cb = callbacks.remove(&command_handle).unwrap();
        cb(command_handle, err, res);
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    callbacks.insert(cmd_handle, Box::new(closure));

    _callback
}