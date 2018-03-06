#ifndef __anoncreds__included__
#define __anoncreds__included__

#ifdef __cplusplus
extern "C" {
#endif
    
    extern indy_error_t indy_issuer_create_schema(indy_handle_t command_handle,
                                                  indy_handle_t wallet_handle,
                                                  const char *  issuer_did,
                                                  const char *  name,
                                                  const char *  version,
                                                  const char *  attr_names,

                                                  void           (*cb)(indy_handle_t xcommand_handle,
                                                                       indy_error_t  err,
                                                                       const char*   id,
                                                                       const char*   schema_json)
                                                  );

    extern indy_error_t indy_issuer_create_and_store_credential_def(indy_handle_t command_handle,
                                                                    indy_handle_t wallet_handle,
                                                                    const char *  issuer_did,
                                                                    const char *  schema_json,
                                                                    const char *  tag,
                                                                    const char *  type_,
                                                                    const char *  config_json,

                                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                                         indy_error_t  err,
                                                                                         const char*   id,
                                                                                         const char*   credential_def_json)
                                                                    );
    
    extern indy_error_t indy_issuer_create_and_store_revoc_reg(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  issuer_did,
                                                               const char *  type_,
                                                               const char *  tag,
                                                               const char *  cred_def_id,
                                                               const char *  config_json,
                                                               const char *  tails_writer_type,
                                                               const char *  tails_writer_config,

                                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                                    indy_error_t  err,
                                                                                    const char*   id,
                                                                                    const char*   revoc_reg_def_json,
                                                                                    const char*   revoc_reg_entry_json)
                                                               );

    extern indy_error_t indy_issuer_create_credential_offer(indy_handle_t command_handle,
                                                            indy_handle_t wallet_handle,
                                                            const char *  cred_def_id,
                                                            const char *  rev_reg_id,
                                                            const char *  prover_did,

                                                            void           (*cb)(indy_handle_t xcommand_handle,
                                                                                 indy_error_t  err,
                                                                                 const char*   credential_offer_json)
                                                            );
    
    extern indy_error_t indy_issuer_create_credential(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      const char *  credential_req_json,
                                                      const char *  credential_values_json,
                                                      indy_handle_t    tails_reader_handle,
                                                      indy_i32_t    user_revoc_index,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   revoc_reg_delta_json,
                                                                           const char*   credential_json)
                                                      );
    
    extern indy_error_t indy_issuer_revoke_credential(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      indy_i32_t tails_reader_handle,
                                                      const char *  rev_reg_id,
                                                      indy_u32_t    user_revoc_index,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   revoc_reg_delta_json)
                                                      );

    extern indy_error_t indy_issuer_recover_credential(indy_handle_t command_handle,
                                                       indy_handle_t wallet_handle,
                                                       indy_i32_t tails_reader_handle,
                                                       const char *  rev_reg_id,
                                                       indy_u32_t    user_revoc_index,

                                                       void           (*cb)(indy_handle_t xcommand_handle,
                                                                            indy_error_t  err,
                                                                            const char*   revoc_reg_delta_json)
                                                       );
    
    extern indy_error_t indy_prover_store_credential_offer(indy_handle_t command_handle,
                                                           indy_handle_t wallet_handle,
                                                           const char *  claim_offer_json,

                                                           void           (*cb)(indy_handle_t xcommand_handle,
                                                                                indy_error_t  err)
                                                           );
    
    
    extern indy_error_t indy_prover_get_credential_offers(indy_handle_t command_handle,
                                                          indy_handle_t wallet_handle,
                                                          const char *  filter_json,
                                                          void           (*cb)(indy_handle_t xcommand_handle,
                                                                               indy_error_t  err,
                                                                               const char*   credential_offers_json)
                                                          );
    
    
    extern indy_error_t indy_prover_create_master_secret(indy_handle_t command_handle,
                                                         indy_handle_t wallet_handle,
                                                         const char *  master_secret_name,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err)
                                                         );
    
    
    extern indy_error_t indy_prover_create_and_store_credential_req(indy_handle_t command_handle,
                                                                    indy_handle_t wallet_handle,
                                                                    const char *  prover_did,
                                                                    const char *  credential_offer_json,
                                                                    const char *  credential_def_json,
                                                                    const char *  master_secret_name,

                                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                                         indy_error_t  err,
                                                                                         const char*   credential_req_json)
                                                                    );

    extern indy_error_t indy_prover_store_credential(indy_handle_t command_handle,
                                                     indy_handle_t wallet_handle,
                                                     const char *  credentials_json,
                                                     const char *  rev_reg_def_json,

                                                     void           (*cb)(indy_handle_t xcommand_handle,
                                                                          indy_error_t  err)
                                                     );
    
    extern indy_error_t indy_prover_get_credentials(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  filter_json,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   credentials_json)
                                                    );
    
    
    extern indy_error_t indy_prover_get_credentials_for_proof_req(indy_handle_t command_handle,
                                                                  indy_handle_t wallet_handle,
                                                                  const char *  proof_request_json,

                                                                  void           (*cb)(indy_handle_t xcommand_handle,
                                                                                       indy_error_t  err,
                                                                                       const char*   credentials_json)
                                                                  );
    
    
    extern indy_error_t indy_prover_create_proof(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  proof_req_json,
                                                 const char *  requested_credentials_json,
                                                 const char *  schemas_json,
                                                 const char *  master_secret_name,
                                                 const char *  credential_defs_json,
                                                 const char *  rev_infos_json,

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   proof_json)
                                                 );


    extern indy_error_t indy_verifier_verify_proof(indy_handle_t command_handle,
                                                   const char *  proof_request_json,
                                                   const char *  proof_json,
                                                   const char *  credential_defs_jsons,
                                                   const char *  rev_reg_defs_json,
                                                   const char *  rev_regs_json,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        indy_bool_t   valid )
                                                   );


    extern indy_error_t indy_create_revocation_info(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    indy_i32_t tails_reader_handle,
                                                    const char *  rev_reg_def_json,
                                                    const char *  rev_reg_delta_json,
                                                    indy_u64_t  timestamp,
                                                    indy_u32_t  rev_idx,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   rev_info_json)
                                                    );


    extern indy_error_t indy_update_revocation_info(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    indy_i32_t tails_reader_handle,
                                                    const char *  rev_info_json,
                                                    const char *  rev_reg_def_json,
                                                    const char *  rev_reg_delta_json,
                                                    indy_u64_t  timestamp,
                                                    indy_u32_t  rev_idx,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   updated_rev_info_json)
                                                    );


    extern indy_error_t indy_store_revocation_info(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  id,
                                                    const char *  rev_info_json,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   updated_rev_info_json)
                                                    );


    extern indy_error_t indy_get_revocation_info(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  id,
                                                 indy_i64_t timestamp,

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   rev_info_json)
                                                 );
    
#ifdef __cplusplus
}
#endif

#endif
