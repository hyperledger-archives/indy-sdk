use nullpay::ErrorCode;

use std::ffi::CString;
use std::ptr::null;
use std::os::raw::c_char;

pub const DEFAULT_WALLET_CREDENTIALS: &'static str = r#"{"key":"key"}"#;

pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

    let pool_name = CString::new(pool_name).unwrap();
    let wallet_name = CString::new(wallet_name).unwrap();
    let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let credentials = CString::new(credentials.unwrap_or(DEFAULT_WALLET_CREDENTIALS)).unwrap();

    let err =
        unsafe {
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               credentials.as_ptr(),
                               cb)
        };

    super::results::result_to_empty(err, receiver)
}

pub fn open_wallet(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<i32, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

    let wallet_name = CString::new(wallet_name).unwrap();
    let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let credentials = CString::new(credentials.unwrap_or(DEFAULT_WALLET_CREDENTIALS)).unwrap();

    let err =
        unsafe {
            indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             credentials.as_ptr(),
                             cb)
        };

    super::results::result_to_int(err, receiver)
}

pub fn create_and_open_wallet(pool_name: &str, xtype: Option<&str>) -> Result<i32, ErrorCode> {
    let wallet_name = format!("default-wallet-name-{}", super::sequence::get_next_id());

    create_wallet(pool_name, &wallet_name, xtype, None, None)?;
    open_wallet(&wallet_name, None, None)
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
                          pool_name: *const c_char,
                          name: *const c_char,
                          xtype: *const c_char,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;

    fn indy_open_wallet(command_handle: i32,
                        name: *const c_char,
                        runtime_config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             handle: i32)>) -> ErrorCode;

    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32,
                                              err: ErrorCode)>) -> ErrorCode;
}