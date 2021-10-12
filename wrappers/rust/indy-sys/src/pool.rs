use super::*;

use {CString, Error, CommandHandle, PoolHandle};

extern {

    pub fn indy_create_pool_ledger_config(command_handle: CommandHandle,
                                          config_name: CString,
                                          config: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_open_pool_ledger(command_handle: CommandHandle,
                                 config_name: CString,
                                 config: CString,
                                 cb: Option<ResponseI32CB>) -> Error;

    pub fn indy_refresh_pool_ledger(command_handle: CommandHandle,
                                    handle: PoolHandle,
                                    cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_list_pools(command_handle: CommandHandle,
                           cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_close_pool_ledger(command_handle: CommandHandle,
                                  handle: PoolHandle,
                                  cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_delete_pool_ledger_config(command_handle: CommandHandle,
                                          config_name: CString,
                                          cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_set_protocol_version(command_handle: CommandHandle,
                                     protocol_version: usize,
                                     cb: Option<ResponseEmptyCB>) -> Error;
}

