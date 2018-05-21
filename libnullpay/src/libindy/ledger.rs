use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;
use utils::callbacks;

pub fn build_get_txn_request(
    submitter_did: &str,
    seq_no: i32,
    cb: Box<FnMut(ErrorCode, String) + Send>,
) -> ErrorCode {
    let (command_handle, cb) = callbacks::closure_to_cb_ec_string(cb);
    let submitter_did = CString::new(submitter_did).unwrap();

    unsafe {
        indy_build_get_txn_request(
            command_handle,
            submitter_did.as_ptr(),
            seq_no,
            cb,
        )
    }
}


extern {
    #[no_mangle]
    pub fn indy_build_get_txn_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      seq_no: i32,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           request_json: *const c_char)>) -> ErrorCode;
}