use nullpay::ErrorCode;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string_string();

    let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
    let my_did_json = CString::new(my_did_json).unwrap();

    let err = unsafe {
        indy_create_and_store_my_did(command_handle, wallet_handle, my_did_json.as_ptr(), cb)
    };

    super::results::result_to_string_string(err, receiver)
}

extern {
    #[no_mangle]
    fn indy_create_and_store_my_did(command_handle: i32,
                                    wallet_handle: i32,
                                    did_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32,
                                                         err: ErrorCode,
                                                         did: *const c_char,
                                                         verkey: *const c_char)>) -> ErrorCode;
}