use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();
        let pool_config_str = CString::new(pool_config).unwrap();

        let err = unsafe {
            indy_create_pool_ledger_config(command_handle,
                                           pool_name.as_ptr(),
                                           pool_config_str.as_ptr(),
                                           cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

        let pool_name = CString::new(pool_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_open_pool_ledger(command_handle,
                                  pool_name.as_ptr(),
                                  if config.is_some() { config_str.as_ptr() } else { null() },
                                  cb)
        };

        super::results::result_to_int(err, receiver)
    }

    #[allow(dead_code)] //TODO add refresh pool command or remove this code
    pub fn refresh(pool_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let err = unsafe { indy_refresh_pool_ledger(command_handle, pool_handle, cb) };

        super::results::result_to_empty(err, receiver)
    }

    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let err = unsafe { indy_list_pools(command_handle, cb) };

        super::results::result_to_string(err, receiver)
    }

    pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let err = unsafe { indy_close_pool_ledger(command_handle, pool_handle, cb) };

        super::results::result_to_empty(err, receiver)
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (receiver, cmd_id, cb) = super::callbacks::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();

        let err = unsafe { indy_delete_pool_ledger_config(cmd_id, pool_name.as_ptr(), cb) };

        super::results::result_to_empty(err, receiver)
    }
}

extern {
    #[no_mangle]
    fn indy_create_pool_ledger_config(command_handle: i32,
                                      config_name: *const c_char,
                                      config: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_open_pool_ledger(command_handle: i32,
                                 config_name: *const c_char,
                                 config: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, pool_handle: i32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_refresh_pool_ledger(command_handle: i32,
                                    handle: i32,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_pools(command_handle: i32,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                pools: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_close_pool_ledger(command_handle: i32,
                                  handle: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_delete_pool_ledger_config(command_handle: i32,
                                          config_name: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;
}