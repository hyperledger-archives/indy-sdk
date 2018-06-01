use super::*;

use {Error, Handle, CString};

pub type CreateWalletCB = extern fn(name: CString,
                                    config: CString,
                                    credentials: CString) -> Error;
pub type OpenWalletCB = extern fn(name: CString,
                                  config: CString,
                                  runtime_config: CString,
                                  credentials: CString,
                                  handle: *mut i32) -> Error;
pub type SetWalletCB = extern fn(handle: Handle,
                                 key: CString,
                                 value: CString) -> Error;
pub type GetWalletCB = extern fn(handle: Handle,
                                 key: CString,
                                 value_ptr: *mut CString) -> Error;
pub type GetNotExpiredWalletCB = extern fn(handle: Handle,
                                           key: CString,
                                           value_ptr: *mut CString) -> Error;
pub type ListWalletCB = extern fn(handle: Handle,
                                  key_prefix: CString,
                                  values_json_ptr: *mut CString) -> Error;
pub type CloseWalletCB = extern fn(handle: Handle) -> Error;
pub type DeleteWalletCB = extern fn(name: CString,
                                    config: CString,
                                    credentials: CString) -> Error;
pub type FreeWalletCB = extern fn(wallet_handle: Handle, value: CString) -> Error;

extern {
    #[no_mangle]
    pub fn indy_register_wallet_type(command_handle: i32,
                                     xtype: CString,
                                     create: Option<CreateWalletCB>,
                                     open: Option<OpenWalletCB>,
                                     set: Option<SetWalletCB>,
                                     get: Option<GetWalletCB>,
                                     get_not_expired: Option<GetNotExpiredWalletCB>,
                                     list: Option<ListWalletCB>,
                                     close: Option<CloseWalletCB>,
                                     delete: Option<DeleteWalletCB>,
                                     free: Option<FreeWalletCB>,
                                     cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_create_wallet(command_handle: Handle,
                              pool_name: CString,
                              name: CString,
                              xtype: CString,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_open_wallet(command_handle: Handle,
                            name: CString,
                            runtime_config: CString,
                            credentials: CString,
                            cb: Option<ResponseI32CB>) -> Error;

    #[no_mangle]
    pub fn indy_list_wallets(command_handle: Handle,
                             cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_close_wallet(command_handle: Handle,
                             handle: Handle,
                             cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_delete_wallet(command_handle: Handle,
                              name: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
}
