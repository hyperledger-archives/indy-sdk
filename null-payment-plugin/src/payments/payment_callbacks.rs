extern crate libc;
extern crate rand;

use self::rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ffi::CStr;
use payments::ErrorCode;
use payments::libindy::indy_build_get_txn_request;
use payments::libindy::indy_register_payment_method;
use std::sync::mpsc::channel;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::ATOMIC_USIZE_INIT;
use std::sync::mpsc::Receiver;

type CommonResponseCallback = extern fn(command_handle_: i32,
                                        err: ErrorCode,
                                        res1: *const c_char) -> ErrorCode;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

#[no_mangle]
pub extern fn init() -> ErrorCode {
    let (receiver, command_handle, cb) = _closure_to_cb_ec();

    _init(command_handle, cb);

    let err = receiver.recv().unwrap();

    err
}

fn _init(cmd_handle:i32, cb: Option<extern fn(_cmd_handle: i32, err: ErrorCode)>) {
    let payment_method_name = CString::new("null_payment").unwrap();

    unsafe {
        indy_register_payment_method(
            cmd_handle,
            payment_method_name.as_ptr(),
            Some(_create_payment_address_handler),
            Some(_add_request_fees_handler),
            Some(_parse_response_with_fees_handler),
            Some(_build_get_utxo_request_handler),
            Some(_parse_get_utxo_response_handler),
            Some(_build_payment_req_handler),
            Some(_parse_payment_response_handler),
            Some(_build_mint_req_handler),
            Some(_build_set_txn_fees_req_handler),
            Some(_build_get_txn_fees_req_handler),
            Some(_parse_get_txn_fees_response_handler),
            cb
        );
    }
}

fn _closure_to_cb_ec() -> (Receiver<ErrorCode>, i32,
                           Option<extern fn(command_handle: i32,
                                            err: ErrorCode)>) {
    let (sender, receiver) = channel();

    lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

    let closure = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

lazy_static! {
      static ref CREATE_ADDRESS_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref ADD_REQUEST_FEES_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref PARSE_RESPONSE_WITH_FEES_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref BUILD_GET_UTXO_REQUEST_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref PARSE_GET_UTXO_RESPONSE_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref BUILD_PAYMENT_REQ_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref PARSE_PAYMENT_RESPONSE_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref BUILD_MINT_REQ_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref BUILD_SET_TXN_FEES_REQ_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref BUILD_GET_TXN_FEES_REQ_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
      static ref PARSE_GET_TXN_FEES_RESPONSE_RESULT_INJECTIONS: Mutex<Vec<(ErrorCode, CString)>> = Default::default();
}

extern fn _create_payment_address_handler(
    cmd_handle:i32,
    wallet_handle: i32,
    _config: *const c_char,
    cb: Option<CommonResponseCallback>) -> ErrorCode {

    let addr = CString::new(format!("pay:null_payment:{}", _get_rand_string(15))).unwrap();
    let mut results = CREATE_ADDRESS_RESULT_INJECTIONS.lock().unwrap();
    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, addr)
    };
    _execute_cb(cb, cmd_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_create_payment_address_result(err: ErrorCode, res: *const c_char) {
    let mut results = CREATE_ADDRESS_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_create_payment_address_injections() {
    let mut vec = CREATE_ADDRESS_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _add_request_fees_handler(
    command_handle: i32,
    _wallet_handle: i32,
    req_json: *const c_char,
    _inputs_json: *const c_char,
    _outputs_json: *const c_char,
    cb: Option<CommonResponseCallback>) -> ErrorCode {
    let mut results = ADD_REQUEST_FEES_RESULT_INJECTIONS.lock().unwrap();
    let req_json = CString::from(unsafe { CStr::from_ptr(req_json) });
    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, req_json)
    };
    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_add_request_fees_result(err: ErrorCode, res: *const c_char) {
    let mut results = ADD_REQUEST_FEES_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_add_request_fees_injections() {
    let mut vec = ADD_REQUEST_FEES_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _parse_response_with_fees_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: Option<CommonResponseCallback>) -> ErrorCode {
    let mut results = PARSE_RESPONSE_WITH_FEES_RESULT_INJECTIONS.lock().unwrap();
    let resp_json = CString::from(unsafe { CStr::from_ptr(resp_json) });
    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, resp_json)
    };
    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_parse_response_with_fees_result(err: ErrorCode, res: *const c_char) {
    let mut results = PARSE_RESPONSE_WITH_FEES_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_parse_response_with_fees_injections() {
    let mut vec = PARSE_RESPONSE_WITH_FEES_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _build_get_utxo_request_handler(
    command_handle: i32,
    _wallet_handle: i32,
    _payment_address: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let mut results = BUILD_GET_UTXO_REQUEST_RESULT_INJECTIONS.lock().unwrap();
    let res = match results.is_empty() {
        false => Some(results.remove(0)),
        true => None
    };
    _build_get_txn_request(command_handle, cb, res)
}

#[no_mangle]
pub extern fn nullpayment_inject_build_get_utxo_request_result(err: ErrorCode, res: *const c_char) {
    let mut results = BUILD_GET_UTXO_REQUEST_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_build_get_utxo_request_injections() {
    let mut vec = BUILD_GET_UTXO_REQUEST_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _parse_get_utxo_response_handler(
    command_handle: i32,
    _resp_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let utxo_example =
        format!(
            r#"[{{"input":"pov:null_payment:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null_payment:2", "amount":2, "extra":"{}"}}]"#,
            _get_rand_string(15),
            _get_rand_string(15)
        );
    let utxo_json = CString::new(utxo_example).unwrap();
    let mut results = PARSE_GET_UTXO_RESPONSE_RESULT_INJECTIONS.lock().unwrap();

    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, utxo_json)
    };

    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_parse_get_utxo_response_result(err: ErrorCode, res: *const c_char) {
    let mut results = PARSE_GET_UTXO_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_parse_get_utxo_response_injections() {
    let mut vec = PARSE_GET_UTXO_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _build_payment_req_handler(
    command_handle: i32,
    _wallet_handle: i32,
    _inputs_json: *const c_char,
    outputs_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let mut results = BUILD_PAYMENT_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = match results.is_empty() {
        false => Some(results.remove(0)),
        true => None
    };
    _build_get_txn_request(command_handle, cb, res)
}

#[no_mangle]
pub extern fn nullpayment_inject_build_payment_req_result(err: ErrorCode, res: *const c_char) {
    let mut results = BUILD_PAYMENT_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_build_payment_req_injections() {
    let mut vec = BUILD_PAYMENT_REQ_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _parse_payment_response_handler(
    command_handle: i32,
    resp_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let payment_response_example =
        format!(
            r#"[{{"input":"pov:null_payment:1", "amount":1, "extra":"{}"}}, {{"input":"pov:null_payment:2", "amount":2, "extra":"{}"}}]"#,
            _get_rand_string(15),
            _get_rand_string(15)
        );
    let mut results = PARSE_PAYMENT_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
    let resp_json = CString::new(payment_response_example).unwrap();
    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, resp_json)
    };
    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_parse_payment_response_result(err: ErrorCode, res: *const c_char){
    let mut results = PARSE_PAYMENT_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_parse_payment_response_injections() {
    let mut vec = PARSE_PAYMENT_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _build_mint_req_handler(
    command_handle: i32,
    _wallet_handle: i32,
    _outputs_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let mut results = BUILD_MINT_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = match results.is_empty() {
        false => Some(results.remove(0)),
        true => None
    };
    _build_get_txn_request(command_handle, cb, res)
}

#[no_mangle]
pub extern fn nullpayment_inject_build_mint_req_result(err: ErrorCode, res: *const c_char) {
    let mut results = BUILD_MINT_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_build_mint_req_injections() {
    let mut vec = BUILD_MINT_REQ_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _build_set_txn_fees_req_handler(
    command_handle: i32,
    _wallet_handle: i32,
    fees_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let mut results = BUILD_SET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
    let outputs_json = CString::from(unsafe { CStr::from_ptr( fees_json )});
    let (err, res) = match results.is_empty() {
        false => results.remove(0),
        true => (ErrorCode::Success, outputs_json)
    };
    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_build_set_txn_fees_req_result(err: ErrorCode, res: *const c_char) {
    let mut results = BUILD_SET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_build_set_txn_fees_req_injections() {
    let mut vec = BUILD_SET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _build_get_txn_fees_req_handler(
    command_handle: i32,
    _wallet_handle: i32,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let mut results = BUILD_GET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = match results.is_empty() {
        false => Some(results.remove(0)),
        true => None
    };
    _build_get_txn_request(command_handle, cb, res)
}

#[no_mangle]
pub extern fn nullpayment_inject_build_get_txn_fees_req_result(err: ErrorCode, res: *const c_char) {
    let mut results = BUILD_GET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_build_get_txn_fees_req_injections() {
    let mut vec = BUILD_GET_TXN_FEES_REQ_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

extern fn _parse_get_txn_fees_response_handler(
    command_handle: i32,
    _resp_json: *const c_char,
    cb: Option<CommonResponseCallback>
) -> ErrorCode {
    let parsed_response = CString::new(
        r#"{"txnType1":1, "txnType2":2, "txnType3":3}"#
    ).unwrap();
    let mut results = PARSE_GET_TXN_FEES_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
    let (err, res) = match results.is_empty() {
        true => (ErrorCode::Success, parsed_response),
        false => results.remove(0)
    };
    _execute_cb(cb, command_handle, res, err)
}

#[no_mangle]
pub extern fn nullpayment_inject_parse_get_txn_fees_response_result(err: ErrorCode, res: *const c_char) {
    let mut results = PARSE_GET_TXN_FEES_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
    let res = CString::from(unsafe { CStr::from_ptr(res) });
    results.push((err, res));
}

#[no_mangle]
pub extern fn nullpayment_clear_parse_get_txn_fees_response_injections() {
    let mut vec = PARSE_GET_TXN_FEES_RESPONSE_RESULT_INJECTIONS.lock().unwrap();
	vec.clear();
}

fn _get_rand_string(len: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(len).collect()
}

fn _execute_cb(
    cb: Option<CommonResponseCallback>,
    command_handle: i32,
    res: CString,
    err: ErrorCode
) -> ErrorCode {
    cb.unwrap_or_else(|| {
        extern fn cb (_a: i32, _b: ErrorCode, _c: *const c_char) -> ErrorCode { ErrorCode::UnknownPaymentMethod }
        cb
    })(command_handle, err, res.as_ptr())
}

fn _build_get_txn_request(
    command_handle: i32,
    cb: Option<CommonResponseCallback>,
    res: Option<(ErrorCode, CString)>
) -> ErrorCode {
    match res {
        Some((err, res)) => _execute_cb(cb, command_handle, res, err),
        None => {
            lazy_static!{
                static ref CALLBACKS : Mutex<HashMap<i32, Box<CommonResponseCallback>>> = Default::default();
            }

            extern "C" fn _callback(command_handle:i32, err: ErrorCode, req_json: *const c_char) {
                let mut map = CALLBACKS.lock().unwrap();
                let _cb = map.remove(&command_handle);
                //WARN: maybe dereferencing is needed
                _cb.unwrap()(command_handle, err, req_json);
            }

            let mut map = CALLBACKS.lock().unwrap();
            map.insert(command_handle, Box::new(cb.unwrap()));

            let submitter_did = CString::new("null_payment").unwrap();

            unsafe {
                indy_build_get_txn_request(
                    command_handle,
                    submitter_did.as_ptr(),
                    1,
                    Some(_callback)
                );
            }
            ErrorCode::Success
        }
    }
}