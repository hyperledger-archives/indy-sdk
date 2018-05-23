use super::*;

use {ErrorCode, IndyHandle};

use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_create_wallet(command_handle: IndyHandle,
                          pool_name: *const c_char,
                          name: *const c_char,
                          xtype: *const c_char,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_open_wallet(command_handle: IndyHandle,
                        name: *const c_char,
                        runtime_config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<ResponseI32CB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_wallets(command_handle: IndyHandle,
                         cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_close_wallet(command_handle: IndyHandle,
                         handle: IndyHandle,
                         cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_delete_wallet(command_handle: IndyHandle,
                          name: *const c_char,
                          credentials: *const c_char,
                          cb: Option<ResponseEmptyCB>) -> ErrorCode;
}
