use super::*;

use {ErrorCode, IndyHandle};

use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_sign_and_submit_request(command_handle: IndyHandle,
                                        pool_handle: IndyHandle,
                                        wallet_handle: IndyHandle,
                                        submitter_did: *const c_char,
                                        request_json: *const c_char,
                                        cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_submit_request(command_handle: IndyHandle,
                               pool_handle: IndyHandle,
                               request_json: *const c_char,
                               cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_sign_request(command_handle: IndyHandle,
                             wallet_handle: IndyHandle,
                             submitter_did: *const c_char,
                             request_json: *const c_char,
                             cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_multi_sign_request(command_handle: IndyHandle,
                                   wallet_handle: IndyHandle,
                                   submitter_did: *const c_char,
                                   request_json: *const c_char,
                                   cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_ddo_request(command_handle: IndyHandle,
                                      submitter_did: *const c_char,
                                      target_did: *const c_char,
                                      cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_nym_request(command_handle: IndyHandle,
                                  submitter_did: *const c_char,
                                  target_did: *const c_char,
                                  verkey: *const c_char,
                                  alias: *const c_char,
                                  role: *const c_char,
                                  cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_nym_request(command_handle: IndyHandle,
                                      submitter_did: *const c_char,
                                      target_did: *const c_char,
                                      cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_attrib_request(command_handle: IndyHandle,
                                     submitter_did: *const c_char,
                                     target_did: *const c_char,
                                     hash: *const c_char,
                                     raw: *const c_char,
                                     enc: *const c_char,
                                     cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_attrib_request(command_handle: IndyHandle,
                                         submitter_did: *const c_char,
                                         target_did: *const c_char,
                                         raw: *const c_char,
                                         hash: *const c_char,
                                         enc: *const c_char,
                                         cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_schema_request(command_handle: IndyHandle,
                                     submitter_did: *const c_char,
                                     data: *const c_char,
                                     cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_schema_request(command_handle: IndyHandle,
                                         submitter_did: *const c_char,
                                         id: *const c_char,
                                         cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_schema_response(command_handle: IndyHandle,
                                          get_schema_response: *const c_char,
                                          cb: Option<ResponseStringStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_cred_def_request(command_handle: IndyHandle,
                                       submitter_did: *const c_char,
                                       data: *const c_char,
                                       cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_cred_def_request(command_handle: IndyHandle,
                                           submitter_did: *const c_char,
                                           id: *const c_char,
                                           cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_cred_def_response(command_handle: IndyHandle,
                                            get_cred_def_response: *const c_char,
                                            cb: Option<ResponseStringStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_node_request(command_handle: IndyHandle,
                                   submitter_did: *const c_char,
                                   target_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_txn_request(command_handle: IndyHandle,
                                      submitter_did: *const c_char,
                                      seq_no: IndyHandle,
                                      cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_config_request(command_handle: IndyHandle,
                                          submitter_did: *const c_char,
                                          writes: bool,
                                          force: bool,
                                          cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_restart_request(command_handle: IndyHandle,
                                           submitter_did: *const c_char,
                                           action: *const c_char,
                                           datetime: *const c_char,
                                           cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_pool_upgrade_request(command_handle: IndyHandle,
                                           submitter_did: *const c_char,
                                           name: *const c_char,
                                           version: *const c_char,
                                           action: *const c_char,
                                           sha256: *const c_char,
                                           timeout: IndyHandle,
                                           schedule: *const c_char,
                                           justification: *const c_char,
                                           reinstall: bool,
                                           force: bool,
                                           cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_revoc_reg_def_request(command_handle: IndyHandle,
                                            submitter_did: *const c_char,
                                            data: *const c_char,
                                            cb: Option<ResponseStringCB>) -> ErrorCode;


    #[no_mangle]
    pub fn indy_build_get_revoc_reg_def_request(command_handle: IndyHandle,
                                                submitter_did: *const c_char,
                                                id: *const c_char,
                                                cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_def_response(command_handle: IndyHandle,
                                                 get_revoc_reg_def_response: *const c_char,
                                                 cb: Option<ResponseStringStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_revoc_reg_entry_request(command_handle: IndyHandle,
                                              submitter_did: *const c_char,
                                              revoc_reg_def_id: *const c_char,
                                              rev_def_type: *const c_char,
                                              value: *const c_char,
                                              cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_revoc_reg_request(command_handle: IndyHandle,
                                            submitter_did: *const c_char,
                                            revoc_reg_def_id: *const c_char,
                                            timestamp: i64,
                                            cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_response(command_handle: IndyHandle,
                                             get_revoc_reg_response: *const c_char,
                                             cb: Option<ResponseStringStringU64CB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_revoc_reg_delta_request(command_handle: IndyHandle,
                                                  submitter_did: *const c_char,
                                                  revoc_reg_def_id: *const c_char,
                                                  from: i64,
                                                  to: i64,
                                                  cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_delta_response(command_handle: IndyHandle,
                                                   get_revoc_reg_delta_response: *const c_char,
                                                   cb: Option<ResponseStringStringU64CB>) -> ErrorCode;
}
