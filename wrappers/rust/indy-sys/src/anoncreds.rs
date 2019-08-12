use super::*;

use {CString, Error, CommandHandle, WalletHandle};

extern {

    #[no_mangle]
    pub fn indy_issuer_create_schema(command_handle: CommandHandle,
                                     issuer_did: CString,
                                     name: CString,
                                     version: CString,
                                     attrs: CString,
                                     cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_create_and_store_credential_def(command_handle: CommandHandle,
                                                       wallet_handle: WalletHandle,
                                                       issuer_did: CString,
                                                       schema_json: CString,
                                                       tag: CString,
                                                       signature_type: CString,
                                                       config_json: CString,
                                                       cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_rotate_credential_def_start(command_handle: CommandHandle,
                                                   wallet_handle: WalletHandle,
                                                   cred_def_id: CString,
                                                   config_json: CString,
                                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_rotate_credential_def_apply(command_handle: CommandHandle,
                                                   wallet_handle: WalletHandle,
                                                   cred_def_id: CString,
                                                   cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_create_and_store_revoc_reg(command_handle: CommandHandle,
                                                  wallet_handle: WalletHandle,
                                                  issuer_did: CString,
                                                  revoc_def_type: CString,
                                                  tag: CString,
                                                  cred_def_id: CString,
                                                  config_json: CString,
                                                  tails_writer_handle: TailWriterHandle,
                                                  cb: Option<ResponseStringStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_create_credential_offer(command_handle: CommandHandle,
                                               wallet_handle: WalletHandle,
                                               cred_def_id: CString,
                                               cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_create_credential(command_handle: CommandHandle,
                                         wallet_handle: WalletHandle,
                                         cred_offer_json: CString,
                                         cred_req_json: CString,
                                         cred_values_json: CString,
                                         rev_reg_id: CString,
                                         blob_storage_reader_handle: BlobStorageReaderHandle,
                                         cb: Option<ResponseStringStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_revoke_credential(command_handle: CommandHandle,
                                         wallet_handle: WalletHandle,
                                         blob_storage_reader_cfg_handle: BlobStorageReaderCfgHandle,
                                         rev_reg_id: CString,
                                         cred_revoc_id: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_issuer_merge_revocation_registry_deltas(command_handle: CommandHandle,
                                                        rev_reg_delta_json: CString,
                                                        other_rev_reg_delta_json: CString,
                                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_create_master_secret(command_handle: CommandHandle,
                                            wallet_handle: WalletHandle,
                                            master_secret_id: CString,
                                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_create_credential_req(command_handle: CommandHandle,
                                             wallet_handle: WalletHandle,
                                             prover_did: CString,
                                             cred_offer_json: CString,
                                             cred_def_json: CString,
                                             master_secret_id: CString,
                                             cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_set_credential_attr_tag_policy(command_handle: CommandHandle,
                                                      wallet_handle: WalletHandle,
                                                      cred_def_id: CString,
                                                      taggable_json: CString,
                                                      retroactive: bool,
                                                      cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_get_credential_attr_tag_policy(command_handle: CommandHandle,
                                                      wallet_handle: WalletHandle,
                                                      cred_def_id: CString,
                                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_store_credential(command_handle: CommandHandle,
                                        wallet_handle: WalletHandle,
                                        cred_id: CString,
                                        cred_req_metadata_json: CString,
                                        cred_json: CString,
                                        cred_def_json: CString,
                                        rev_reg_def_json: CString,
                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_get_credential(command_handle: CommandHandle,
                                      wallet_handle: WalletHandle,
                                      cred_id: CString,
                                      cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_delete_credential(command_handle: CommandHandle,
                                         wallet_handle: WalletHandle,
                                         cred_id: CString,
                                         cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_get_credentials(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       filter_json: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_search_credentials(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          query_json: CString,
                                          cb: Option<ResponseI32UsizeCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_fetch_credentials(command_handle: CommandHandle,
                                         search_handle: SearchHandle,
                                         count: usize,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_close_credentials_search(command_handle: CommandHandle,
                                                search_handle: SearchHandle,
                                                cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_get_credentials_for_proof_req(command_handle: CommandHandle,
                                                     wallet_handle: WalletHandle,
                                                     proof_request_json: CString,
                                                     cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_search_credentials_for_proof_req(command_handle: CommandHandle,
                                                        wallet_handle: WalletHandle,
                                                        proof_request_json: CString,
                                                        extra_query_json: CString,
                                                        cb: Option<ResponseI32CB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_fetch_credentials_for_proof_req(command_handle: CommandHandle,
                                                       search_handle: SearchHandle,
                                                       item_referent: CString,
                                                       count: usize,
                                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_close_credentials_search_for_proof_req(command_handle: CommandHandle,
                                                              search_handle: SearchHandle,
                                                              cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_prover_create_proof(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    proof_req_json: CString,
                                    requested_credentials_json: CString,
                                    master_secret_id: CString,
                                    schemas_json: CString,
                                    credential_defs_json: CString,
                                    rev_states_json: CString,
                                    cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_verifier_verify_proof(command_handle: CommandHandle,
                                      proof_request_json: CString,
                                      proof_json: CString,
                                      schemas_json: CString,
                                      credential_defs_json: CString,
                                      rev_reg_defs_json: CString,
                                      rev_regs_json: CString,
                                      cb: Option<ResponseBoolCB>) -> Error;

    #[no_mangle]
    pub fn indy_create_revocation_state(command_handle: CommandHandle,
                                        blob_storage_reader_handle: BlobStorageReaderHandle,
                                        rev_reg_def_json: CString,
                                        rev_reg_delta_json: CString,
                                        timestamp: u64,
                                        cred_rev_id: CString,
                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_update_revocation_state(command_handle: CommandHandle,
                                        blob_storage_reader_handle: BlobStorageReaderHandle,
                                        rev_state_json: CString,
                                        rev_reg_def_json: CString,
                                        rev_reg_delta_json: CString,
                                        timestamp: u64,
                                        cred_rev_id: CString,
                                        cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_generate_nonce(command_handle: CommandHandle,
                               cb: Option<ResponseStringCB>) -> Error;
}

