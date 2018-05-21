use std::ffi::CString;
use nullpay::ErrorCode;
use std::os::raw::c_char;

pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

    let request_json = CString::new(request_json).unwrap();

    let err = unsafe {indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb)};

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
}