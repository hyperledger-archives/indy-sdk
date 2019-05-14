use super::*;

use {CString, Error, CommandHandle, WalletHandle, PoolHandle};

extern {

    #[no_mangle]
    pub fn indy_create_and_store_my_did(command_handle: CommandHandle,
                                        wallet_handle: WalletHandle,
                                        did_info: CString,
                                        cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_replace_keys_start(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   did: CString,
                                   key_info: CString,
                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_replace_keys_apply(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   did: CString,
                                   cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_store_their_did(command_handle: CommandHandle,
                                wallet_handle: WalletHandle,
                                identity_json: CString,
                                cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_key_for_did(command_handle: CommandHandle,
                            pool_handle: PoolHandle,
                            wallet_handle: WalletHandle,
                            did: CString,
                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_key_for_local_did(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  did: CString,
                                  cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_set_endpoint_for_did(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     did: CString,
                                     address: CString,
                                     transport_key: CString,
                                     cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_endpoint_for_did(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     pool_handle: PoolHandle,
                                     did: CString,
                                     cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_set_did_metadata(command_handle: CommandHandle,
                                 wallet_handle: WalletHandle,
                                 did: CString,
                                 metadata: CString,
                                 cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_did_metadata(command_handle: CommandHandle,
                                 wallet_handle: WalletHandle,
                                 did: CString,
                                 cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_my_did_with_meta(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     my_did: CString,
                                     cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_list_my_dids_with_meta(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_abbreviate_verkey(command_handle: CommandHandle,
                                  did: CString,
                                  full_verkey: CString,
                                  cb: Option<ResponseStringCB>) -> Error;
}

