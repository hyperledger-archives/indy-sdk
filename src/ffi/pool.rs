use super::*;

use {ErrorCode, IndyHandle};

use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_create_pool_ledger_config(command_handle: IndyHandle,
                                          config_name: *const c_char,
                                          config: *const c_char,
                                          cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_open_pool_ledger(command_handle: IndyHandle,
                                 config_name: *const c_char,
                                 config: *const c_char,
                                 cb: Option<ResponseI32CB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_refresh_pool_ledger(command_handle: IndyHandle,
                                    handle: IndyHandle,
                                    cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_pools(command_handle: IndyHandle,
                           cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_close_pool_ledger(command_handle: IndyHandle,
                                  handle: IndyHandle,
                                  cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_delete_pool_ledger_config(command_handle: IndyHandle,
                                          config_name: *const c_char,
                                          cb: Option<ResponseEmptyCB>) -> ErrorCode;
}
