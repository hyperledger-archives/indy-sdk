use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;

pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_create_wallet(command_handle,
                               config.as_ptr(),
                               credentials.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_open_wallet(command_handle,
                             config.as_ptr(),
                             credentials.as_ptr(),
                             cb)
        };

        super::results::result_to_int(err, receiver)
    }

    pub fn delete_wallet(wallet_name: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let wallet_name = CString::new(wallet_name).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               credentials.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();


        let err = unsafe { indy_close_wallet(command_handle, wallet_handle, cb) };

        super::results::result_to_empty(err, receiver)
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let export_config_json = CString::new(export_config_json).unwrap();

        let err = unsafe {
            indy_export_wallet(command_handle,
                               wallet_handle,
                               export_config_json.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();
        let import_config_json = CString::new(import_config_json).unwrap();

        let err = unsafe {
            indy_import_wallet(command_handle,
                               config.as_ptr(),
                               credentials.as_ptr(),
                               import_config_json.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }
}

extern {
    #[no_mangle]
    fn indy_create_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_open_wallet(command_handle: i32,
                        config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>) -> ErrorCode;

    #[no_mangle]
    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_delete_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_export_wallet(command_handle: i32,
                          wallet_handle: i32,
                          export_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_import_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          import_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;
}