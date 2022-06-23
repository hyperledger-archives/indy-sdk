use super::*;

use {CString, Error, CommandHandle, WalletHandle, PoolHandle};

extern {

    pub fn indy_get_schema(command_handle: CommandHandle,
                           pool_handle: PoolHandle,
                           wallet_handle: WalletHandle,
                           submitter_did: CString,
                           id: CString,
                           options_json: CString,
                           cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_get_cred_def(command_handle: CommandHandle,
                             pool_handle: PoolHandle,
                             wallet_handle: WalletHandle,
                             submitter_did: CString,
                             id: CString,
                             options_json: CString,
                             cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_purge_schema_cache(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   options_json: CString,
                                   cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_purge_cred_def_cache(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     options_json: CString,
                                     cb: Option<ResponseEmptyCB>) -> Error;
}
