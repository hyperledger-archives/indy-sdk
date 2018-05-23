use {ErrorCode, IndyHandle};
use std::os::raw::c_char;
use super::*;

extern {
    #[no_mangle]
    pub fn indy_create_and_store_my_did(command_handle: IndyHandle,
                                        wallet_handle: IndyHandle,
                                        did_json: *const c_char,
                                        cb: Option<ResponseStringStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_replace_keys_start(command_handle: IndyHandle,
                                   wallet_handle: IndyHandle,
                                   did: *const c_char,
                                   identity_json: *const c_char,
                                   cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_replace_keys_apply(command_handle: IndyHandle,
                                   wallet_handle: IndyHandle,
                                   did: *const c_char,
                                   cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_store_their_did(command_handle: IndyHandle,
                                wallet_handle: IndyHandle,
                                identity_json: *const c_char,
                                cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_key_for_did(command_handle: IndyHandle,
                            pool_handle: IndyHandle,
                            wallet_handle: IndyHandle,
                            did: *const c_char,
                            cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_key_for_local_did(command_handle: IndyHandle,
                                 wallet_handle: IndyHandle,
                                 did: *const c_char,
                                 cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_endpoint_for_did(command_handle: IndyHandle,
                                     wallet_handle: IndyHandle,
                                     did: *const c_char,
                                     address: *const c_char,
                                     transport_key: *const c_char,
                                     cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_get_endpoint_for_did(command_handle: IndyHandle,
                                     wallet_handle: IndyHandle,
                                     pool_handle: IndyHandle,
                                     did: *const c_char,
                                     cb: Option<ResponseStringStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_did_metadata(command_handle: IndyHandle,
                             wallet_handle: IndyHandle,
                             did: *const c_char,
                             metadata: *const c_char,
                             cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_get_did_metadata(command_handle: IndyHandle,
                                 wallet_handle: IndyHandle,
                                 did: *const c_char,
                                 cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_get_my_did_with_meta(command_handle: IndyHandle,
                                     wallet_handle: IndyHandle,
                                     my_did: *const c_char,
                                     cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_my_dids_with_meta(command_handle: IndyHandle,
                                   wallet_handle: IndyHandle,
                                   cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_abbreviate_verkey(command_handle: IndyHandle,
                              did: *const c_char,
                              full_verkey: *const c_char,
                              cb: Option<ResponseStringCB>) -> ErrorCode;
}
