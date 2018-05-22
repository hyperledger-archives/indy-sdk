use ErrorCode;
use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_create_and_store_my_did(command_handle: i32,
                                        wallet_handle: i32,
                                        did_json: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             did: *const c_char,
                                                             verkey: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_replace_keys_start(command_handle: i32,
                                   wallet_handle: i32,
                                   did: *const c_char,
                                   identity_json: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        verkey: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_replace_keys_apply(command_handle: i32,
                                   wallet_handle: i32,
                                   did: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32,
                                                        err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_did_metadata(command_handle: i32,
                             wallet_handle: i32,
                             did: *const c_char,
                             metadata: *const c_char,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_get_my_did_with_meta(command_handle: i32,
                                     wallet_handle: i32,
                                     my_did: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                          did_with_meta: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_my_dids_with_meta(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        dids: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_abbreviate_verkey(command_handle: i32,
                              did: *const c_char,
                              full_verkey: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                   verkey: *const c_char)>) -> ErrorCode;
}
