use ErrorCode;
use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_sign_and_submit_request(command_handle: i32,
                                    pool_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    request_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_result_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_result_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_multi_sign_request(command_handle: i32,
                               wallet_handle: i32,
                               submitter_did: *const c_char,
                               request_json: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    signed_request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_nym_request(command_handle: i32,
                              submitter_did: *const c_char,
                              target_did: *const c_char,
                              verkey: *const c_char,
                              alias: *const c_char,
                              role: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_nym_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  target_did: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_attrib_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 target_did: *const c_char,
                                 hash: *const c_char,
                                 raw: *const c_char,
                                 enc: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_attrib_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     target_did: *const c_char,
                                     raw: *const c_char,
                                     hash: *const c_char,
                                     enc: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_schema_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_cred_def_request(command_handle: i32,
                                   submitter_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_cred_def_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_node_request(command_handle: i32,
                               submitter_did: *const c_char,
                               target_did: *const c_char,
                               data: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_config_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      writes: bool,
                                      force: bool,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_restart_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       action: *const c_char,
                                       datetime: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_upgrade_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       name: *const c_char,
                                       version: *const c_char,
                                       action: *const c_char,
                                       sha256: *const c_char,
                                       timeout: i32,
                                       schedule: *const c_char,
                                       justification: *const c_char,
                                       reinstall: bool,
                                       force: bool,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;
}
