use super::*;

use native::{CString, Error, Handle};

extern {
    #[no_mangle]
    pub fn indy_sign_and_submit_request(command_handle: Handle,
                                        pool_handle: Handle,
                                        wallet_handle: Handle,
                                        submitter_did: CString,
                                        request_json: CString,
                                        cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_submit_request(command_handle: Handle,
                               pool_handle: Handle,
                               request_json: CString,
                               cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_submit_action(command_handle: Handle,
                              pool_handle: Handle,
                              request_json: CString,
                              nodes: CString,
                              timeout: Handle,
                              cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_sign_request(command_handle: Handle,
                             wallet_handle: Handle,
                             submitter_did: CString,
                             request_json: CString,
                             cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_multi_sign_request(command_handle: Handle,
                                   wallet_handle: Handle,
                                   submitter_did: CString,
                                   request_json: CString,
                                   cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_ddo_request(command_handle: Handle,
                                      submitter_did: CString,
                                      target_did: CString,
                                      cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_nym_request(command_handle: Handle,
                                  submitter_did: CString,
                                  target_did: CString,
                                  verkey: CString,
                                  alias: CString,
                                  role: CString,
                                  cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_nym_request(command_handle: Handle,
                                      submitter_did: CString,
                                      target_did: CString,
                                      cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_attrib_request(command_handle: Handle,
                                     submitter_did: CString,
                                     target_did: CString,
                                     hash: CString,
                                     raw: CString,
                                     enc: CString,
                                     cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_attrib_request(command_handle: Handle,
                                         submitter_did: CString,
                                         target_did: CString,
                                         raw: CString,
                                         hash: CString,
                                         enc: CString,
                                         cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_schema_request(command_handle: Handle,
                                     submitter_did: CString,
                                     data: CString,
                                     cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_schema_request(command_handle: Handle,
                                         submitter_did: CString,
                                         id: CString,
                                         cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_schema_response(command_handle: Handle,
                                          get_schema_response: CString,
                                          cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_cred_def_request(command_handle: Handle,
                                       submitter_did: CString,
                                       data: CString,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_cred_def_request(command_handle: Handle,
                                           submitter_did: CString,
                                           id: CString,
                                           cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_cred_def_response(command_handle: Handle,
                                            get_cred_def_response: CString,
                                            cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_node_request(command_handle: Handle,
                                   submitter_did: CString,
                                   target_did: CString,
                                   data: CString,
                                   cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_validator_info_request(command_handle: Handle,
                                                 submitter_did: CString,
                                                 cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_txn_request(command_handle: Handle,
                                      submitter_did: CString,
                                      ledger_type: CString,
                                      seq_no: Handle,
                                      cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_pool_config_request(command_handle: Handle,
                                          submitter_did: CString,
                                          writes: bool,
                                          force: bool,
                                          cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_pool_restart_request(command_handle: Handle,
                                           submitter_did: CString,
                                           action: CString,
                                           datetime: CString,
                                           cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_pool_upgrade_request(command_handle: Handle,
                                           submitter_did: CString,
                                           name: CString,
                                           version: CString,
                                           action: CString,
                                           sha256: CString,
                                           timeout: Handle,
                                           schedule: CString,
                                           justification: CString,
                                           reinstall: bool,
                                           force: bool,
                                           package: CString,
                                           cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_revoc_reg_def_request(command_handle: Handle,
                                            submitter_did: CString,
                                            data: CString,
                                            cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_revoc_reg_def_request(command_handle: Handle,
                                                submitter_did: CString,
                                                id: CString,
                                                cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_def_response(command_handle: Handle,
                                                 get_revoc_reg_def_response: CString,
                                                 cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_revoc_reg_entry_request(command_handle: Handle,
                                              submitter_did: CString,
                                              revoc_reg_def_id: CString,
                                              rev_def_type: CString,
                                              value: CString,
                                              cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_revoc_reg_request(command_handle: Handle,
                                            submitter_did: CString,
                                            revoc_reg_def_id: CString,
                                            timestamp: i64,
                                            cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_response(command_handle: Handle,
                                             get_revoc_reg_response: CString,
                                             cb: Option<ResponseStringStringU64CB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_revoc_reg_delta_request(command_handle: Handle,
                                                  submitter_did: CString,
                                                  revoc_reg_def_id: CString,
                                                  from: i64,
                                                  to: i64,
                                                  cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_delta_response(command_handle: Handle,
                                                   get_revoc_reg_delta_response: CString,
                                                   cb: Option<ResponseStringStringU64CB>) -> Error;
    #[no_mangle]
    pub fn indy_register_transaction_parser_for_sp(command_handle: Handle,
                                                   txn_type: CString,
                                                   parser: Option<CustomTransactionParser>,
                                                   free: Option<CustomFree>,
                                                   cb: Option<ResponseEmptyCB>) -> Error;
}

pub type CustomTransactionParser = extern fn(reply_from_node: CString, parsed_sp: *mut CString) -> Error;
pub type CustomFree = extern fn(data: CString) -> Error;
