use super::*;

use {ErrorCode, IndyHandle};

use std::os::raw::c_char;

pub type CreateWalletCB = extern fn(name: *const c_char,
                                    config: *const c_char,
                                    credentials: *const c_char) -> ErrorCode;
pub type OpenWalletCB = extern fn(name: *const c_char,
                                  config: *const c_char,
                                  runtime_config: *const c_char,
                                  credentials: *const c_char,
                                  handle: *mut i32) -> ErrorCode;
pub type SetWalletCB = extern fn(handle: IndyHandle,
                                 key: *const c_char,
                                 value: *const c_char) -> ErrorCode;
pub type GetWalletCB = extern fn(handle: IndyHandle,
                                 key: *const c_char,
                                 value_ptr: *mut *const c_char) -> ErrorCode;
pub type GetNotExpiredWalletCB = extern fn(handle: IndyHandle,
                                           key: *const c_char,
                                           value_ptr: *mut *const c_char) -> ErrorCode;
pub type ListWalletCB = extern fn(handle: IndyHandle,
                                  key_prefix: *const c_char,
                                  values_json_ptr: *mut *const c_char) -> ErrorCode;
pub type CloseWalletCB = extern fn(handle: IndyHandle) -> ErrorCode;
pub type DeleteWalletCB = extern fn(name: *const c_char,
                                    config: *const c_char,
                                    credentials: *const c_char) -> ErrorCode;
pub type FreeWalletCB = extern fn(wallet_handle: IndyHandle, value: *const c_char) -> ErrorCode;

extern {
    #[no_mangle]
    pub fn indy_register_wallet_type(command_handle: i32,
                                     xtype: *const c_char,
                                     create: Option<CreateWalletCB>,
                                     open: Option<OpenWalletCB>,
                                     set: Option<SetWalletCB>,
                                     get: Option<GetWalletCB>,
                                     get_not_expired: Option<GetNotExpiredWalletCB>,
                                     list: Option<ListWalletCB>,
                                     close: Option<CloseWalletCB>,
                                     delete: Option<DeleteWalletCB>,
                                     free: Option<FreeWalletCB>,
                                     cb: Option<ResponseEmptyCB>) -> ErrorCode;
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
