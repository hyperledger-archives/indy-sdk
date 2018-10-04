use super::*;

use native::{CString, Error, Handle};

extern {
    #[no_mangle]
    pub fn indy_create_pool_ledger_config(command_handle: Handle,
                                          config_name: CString,
                                          config: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_open_pool_ledger(command_handle: Handle,
                                 config_name: CString,
                                 config: CString,
                                 cb: Option<ResponseI32CB>) -> Error;
    #[no_mangle]
    pub fn indy_refresh_pool_ledger(command_handle: Handle,
                                    handle: Handle,
                                    cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_list_pools(command_handle: Handle,
                           cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_close_pool_ledger(command_handle: Handle,
                                  handle: Handle,
                                  cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_delete_pool_ledger_config(command_handle: Handle,
                                          config_name: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_set_protocol_version(command_handle: Handle,
                                     protocol_version: usize,
                                     cb: Option<ResponseEmptyCB>) -> Error;
}

