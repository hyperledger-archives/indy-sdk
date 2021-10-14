use super::*;

use {CString, Error, CommandHandle, WalletHandle};

extern {

    pub fn indy_is_pairwise_exists(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   their_did: CString,
                                   cb: Option<ResponseBoolCB>) -> Error;

    pub fn indy_create_pairwise(command_handle: CommandHandle,
                                wallet_handle: WalletHandle,
                                their_did: CString,
                                my_did: CString,
                                metadata: CString,
                                cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_list_pairwise(command_handle: CommandHandle,
                              wallet_handle: WalletHandle,
                              cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_get_pairwise(command_handle: CommandHandle,
                             wallet_handle: WalletHandle,
                             their_did: CString,
                             cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_set_pairwise_metadata(command_handle: CommandHandle,
                                      wallet_handle: WalletHandle,
                                      their_did: CString,
                                      metadata: CString,
                                      cb: Option<ResponseEmptyCB>) -> Error;
}

