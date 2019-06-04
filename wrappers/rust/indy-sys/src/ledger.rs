use super::*;

use {CString, Error, CommandHandle, WalletHandle, PoolHandle};

extern {
    #[no_mangle]
    pub fn indy_sign_and_submit_request(command_handle: CommandHandle,
                                        pool_handle: PoolHandle,
                                        wallet_handle: WalletHandle,
                                        submitter_did: CString,
                                        request_json: CString,
                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_submit_request(command_handle: CommandHandle,
                               pool_handle: PoolHandle,
                               request_json: CString,
                               cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_submit_action(command_handle: CommandHandle,
                              pool_handle: PoolHandle,
                              request_json: CString,
                              nodes: CString,
                              timeout: Timeout,
                              cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_sign_request(command_handle: CommandHandle,
                             wallet_handle: WalletHandle,
                             submitter_did: CString,
                             request_json: CString,
                             cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_multi_sign_request(command_handle: CommandHandle,
                                   wallet_handle: WalletHandle,
                                   submitter_did: CString,
                                   request_json: CString,
                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_ddo_request(command_handle: CommandHandle,
                                      submitter_did: CString,
                                      target_did: CString,
                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_nym_request(command_handle: CommandHandle,
                                  submitter_did: CString,
                                  target_did: CString,
                                  verkey: CString,
                                  alias: CString,
                                  role: CString,
                                  cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_nym_request(command_handle: CommandHandle,
                                      submitter_did: CString,
                                      target_did: CString,
                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_attrib_request(command_handle: CommandHandle,
                                     submitter_did: CString,
                                     target_did: CString,
                                     hash: CString,
                                     raw: CString,
                                     enc: CString,
                                     cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_attrib_request(command_handle: CommandHandle,
                                         submitter_did: CString,
                                         target_did: CString,
                                         raw: CString,
                                         hash: CString,
                                         enc: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_schema_request(command_handle: CommandHandle,
                                     submitter_did: CString,
                                     data: CString,
                                     cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_schema_request(command_handle: CommandHandle,
                                         submitter_did: CString,
                                         id: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_schema_response(command_handle: CommandHandle,
                                          get_schema_response: CString,
                                          cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_cred_def_request(command_handle: CommandHandle,
                                       submitter_did: CString,
                                       data: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_cred_def_request(command_handle: CommandHandle,
                                           submitter_did: CString,
                                           id: CString,
                                           cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_cred_def_response(command_handle: CommandHandle,
                                            get_cred_def_response: CString,
                                            cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_node_request(command_handle: CommandHandle,
                                   submitter_did: CString,
                                   target_did: CString,
                                   data: CString,
                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_validator_info_request(command_handle: CommandHandle,
                                                 submitter_did: CString,
                                                 cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_txn_request(command_handle: CommandHandle,
                                      submitter_did: CString,
                                      ledger_type: CString,
                                      seq_no: i32,
                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_pool_config_request(command_handle: CommandHandle,
                                          submitter_did: CString,
                                          writes: bool,
                                          force: bool,
                                          cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_pool_restart_request(command_handle: CommandHandle,
                                           submitter_did: CString,
                                           action: CString,
                                           datetime: CString,
                                           cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_pool_upgrade_request(command_handle: CommandHandle,
                                           submitter_did: CString,
                                           name: CString,
                                           version: CString,
                                           action: CString,
                                           sha256: CString,
                                           timeout: Timeout,
                                           schedule: CString,
                                           justification: CString,
                                           reinstall: bool,
                                           force: bool,
                                           package: CString,
                                           cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_revoc_reg_def_request(command_handle: CommandHandle,
                                            submitter_did: CString,
                                            data: CString,
                                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_revoc_reg_def_request(command_handle: CommandHandle,
                                                submitter_did: CString,
                                                id: CString,
                                                cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_def_response(command_handle: CommandHandle,
                                                 get_revoc_reg_def_response: CString,
                                                 cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_revoc_reg_entry_request(command_handle: CommandHandle,
                                              submitter_did: CString,
                                              revoc_reg_def_id: CString,
                                              rev_def_type: CString,
                                              value: CString,
                                              cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_revoc_reg_request(command_handle: CommandHandle,
                                            submitter_did: CString,
                                            revoc_reg_def_id: CString,
                                            timestamp: i64,
                                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_response(command_handle: CommandHandle,
                                             get_revoc_reg_response: CString,
                                             cb: Option<ResponseStringStringU64CB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_revoc_reg_delta_request(command_handle: CommandHandle,
                                                  submitter_did: CString,
                                                  revoc_reg_def_id: CString,
                                                  from: i64,
                                                  to: i64,
                                                  cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_revoc_reg_delta_response(command_handle: CommandHandle,
                                                   get_revoc_reg_delta_response: CString,
                                                   cb: Option<ResponseStringStringU64CB>) -> Error;

    #[no_mangle]
    pub fn indy_register_transaction_parser_for_sp(command_handle: CommandHandle,
                                                   txn_type: CString,
                                                   parser: Option<CustomTransactionParser>,
                                                   free: Option<CustomFree>,
                                                   cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_response_metadata(command_handle: CommandHandle,
                                      response: CString,
                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_auth_rule_request(command_handle: CommandHandle,
                                        submitter_did: CString,
                                        txn_type: CString,
                                        action: CString,
                                        field: CString,
                                        old_value: CString,
                                        new_value: CString,
                                        constraint: CString,
                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_auth_rules_request(command_handle: CommandHandle,
                                         submitter_did: CString,
                                         data: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_auth_rule_request(command_handle: CommandHandle,
                                            submitter_did: CString,
                                            txn_type: CString,
                                            action: CString,
                                            field: CString,
                                            old_value: CString,
                                            new_value: CString,
                                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_txn_author_agreement_request(command_handle: CommandHandle,
                                                   submitter_did: CString,
                                                   text: CString,
                                                   version: CString,
                                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_txn_author_agreement_request(command_handle: CommandHandle,
                                                       submitter_did: CString,
                                                       data: CString,
                                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_acceptance_mechanisms_request(command_handle: CommandHandle,
                                                    submitter_did: CString,
                                                    aml: CString,
                                                    version: CString,
                                                    aml_context: CString,
                                                    cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_acceptance_mechanisms_request(command_handle: CommandHandle,
                                                        submitter_did: CString,
                                                        timestamp: i64,
                                                        version: CString,
                                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_append_txn_author_agreement_acceptance_to_request(command_handle: CommandHandle,
                                                                  request_json: CString,
                                                                  text: CString,
                                                                  version: CString,
                                                                  hash: CString,
                                                                  acc_mech_type: CString,
                                                                  time_of_acceptance: u64,
                                                                  cb: Option<ResponseStringCB>) -> Error;
}

pub type CustomTransactionParser = extern fn(reply_from_node: CString, parsed_sp: *mut CString) -> Error;
pub type CustomFree = extern fn(data: CString) -> Error;
