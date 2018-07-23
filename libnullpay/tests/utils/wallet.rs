use nullpay::ErrorCode;

use std::ffi::CString;
use std::os::raw::c_char;

pub const DEFAULT_WALLET_CREDENTIALS: &'static str = r#"{"key":"key"}"#;

pub fn create_wallet(config: &str) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(DEFAULT_WALLET_CREDENTIALS).unwrap();
    ;

    let err =
        unsafe {
            indy_create_wallet(command_handle,
                               config.as_ptr(),
                               credentials.as_ptr(),
                               cb)
        };

    super::results::result_to_empty(err, receiver)
}

pub fn open_wallet(config: &str) -> Result<i32, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(DEFAULT_WALLET_CREDENTIALS).unwrap();
    ;

    let err =
        unsafe {
            indy_open_wallet(command_handle,
                             config.as_ptr(),
                             credentials.as_ptr(),
                             cb)
        };

    super::results::result_to_int(err, receiver)
}

pub fn create_and_open_wallet() -> Result<i32, ErrorCode> {
    let wallet_name = format!("default-wallet-name-{}", super::sequence::get_next_id());
    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    create_wallet(&config)?;
    open_wallet(&config)
}

pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

    let err = unsafe {
        indy_close_wallet(command_handle, wallet_handle, cb)
    };

    super::results::result_to_empty(err, receiver)
}

extern {
    fn indy_create_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;

    fn indy_open_wallet(command_handle: i32,
                        config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             handle: i32)>) -> ErrorCode;

    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32,
                                              err: ErrorCode)>) -> ErrorCode;
}