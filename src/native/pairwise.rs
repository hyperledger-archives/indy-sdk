use super::*;

use native::{Error, Handle, CString};

extern {
    #[no_mangle]
    pub fn indy_is_pairwise_exists(command_handle: Handle,
                                   wallet_handle: Handle,
                                   their_did: CString,
                                   cb: Option<ResponseBoolCB>) -> Error;

    #[no_mangle]
    pub fn indy_create_pairwise(command_handle: Handle,
                                wallet_handle: Handle,
                                their_did: CString,
                                my_did: CString,
                                metadata: CString,
                                cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_list_pairwise(command_handle: Handle,
                              wallet_handle: Handle,
                              cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_pairwise(command_handle: Handle,
                             wallet_handle: Handle,
                             their_did: CString,
                             cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_set_pairwise_metadata(command_handle: Handle,
                                      wallet_handle: Handle,
                                      their_did: CString,
                                      metadata: CString,
                                      cb: Option<ResponseEmptyCB>) -> Error;
}
