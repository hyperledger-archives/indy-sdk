use std::ffi::CString;
use nullpay::ErrorCode;
use std::os::raw::c_char;

pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

    let request_json = CString::new(request_json).unwrap();

    let err = unsafe { indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb) };

    super::results::result_to_string(err, receiver)
}

pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let request_json = CString::new(request_json).unwrap();

    let err = unsafe {
        indy_sign_and_submit_request(command_handle,
                                     pool_handle,
                                     wallet_handle,
                                     submitter_did.as_ptr(),
                                     request_json.as_ptr(),
                                     cb)
    };

    super::results::result_to_string(err, receiver)
}

pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: &str, alias: &str, role: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let target_did = CString::new(target_did).unwrap();
    let verkey = CString::new(verkey).unwrap();
    let alias = CString::new(alias).unwrap();
    let role = CString::new(role).unwrap();

    let err = unsafe {
        indy_build_nym_request(command_handle,
                               submitter_did.as_ptr(),
                               target_did.as_ptr(),
                               verkey.as_ptr(),
                               alias.as_ptr(),
                               role.as_ptr(),
                               cb)
    };

    super::results::result_to_string(err, receiver)
}

extern {
    #[no_mangle]
    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32,
                                                err: ErrorCode,
                                                request_result_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_nym_request(command_handle: i32,
                              submitter_did: *const c_char,
                              target_did: *const c_char,
                              verkey: *const c_char,
                              alias: *const c_char,
                              role: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32,
                                                   err: ErrorCode,
                                                   request_json: *const c_char)>) -> ErrorCode;
    #[no_mangle]
    fn indy_sign_and_submit_request(command_handle: i32,
                                               pool_handle: i32,
                                               wallet_handle: i32,
                                               submitter_did: *const c_char,
                                               request_json: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32,
                                                                    err: ErrorCode,
                                                                    request_result_json: *const c_char)>) -> ErrorCode;

}