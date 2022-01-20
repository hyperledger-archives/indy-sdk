use ErrorCode;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;
use utils::callbacks;

pub fn build_get_txn_request(
    submitter_did: Option<&str>,
    ledger_type: Option<&str>,
    seq_no: i32,
    cb: Box<dyn FnMut(ErrorCode, String) + Send>,
) -> ErrorCode {
    let (command_handle, cb) = callbacks::closure_to_cb_ec_string(cb);
    let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let ledger_type_str = ledger_type.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

    unsafe {
        indy_build_get_txn_request(
            command_handle,
            if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
            if ledger_type.is_some() { ledger_type_str.as_ptr() } else { null() },
            seq_no,
            cb,
        )
    }
}


extern {
    pub fn indy_build_get_txn_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      ledger_type: *const c_char,
                                      seq_no: i32,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           request_json: *const c_char)>) -> ErrorCode;
}