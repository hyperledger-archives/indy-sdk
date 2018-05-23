use ErrorCode;
use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_create_pool_ledger_config(command_handle: i32,
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
