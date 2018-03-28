#ifndef __anoncreds__included__
#define __anoncreds__included__

#ifdef __cplusplus
extern "C" {
#endif
    
    extern indy_error_t indy_issuer_create_and_store_claim_def(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  issuer_did,
                                                               const char *  schema_json,
                                                               const char *  signature_type,
                                                               indy_bool_t   create_non_revoc,

                                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                                    indy_error_t  err,
                                                                                    const char*   claim_def_json)
                                                               );
    
    extern indy_error_t indy_issuer_create_and_store_revoc_reg(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  issuer_did,
                                                               const char *  schema_json,
                                                               indy_u32_t    max_claim_num,

                                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                                    indy_error_t  err,
                                                                                    const char*   revoc_reg_json)
                                                               );

    extern indy_error_t indy_issuer_create_claim_offer(indy_handle_t command_handle,
                                                       indy_handle_t wallet_handle,
                                                       const char *  schema_json,
                                                       const char *  issuer_did,
                                                       const char *  prover_did,

                                                       void           (*cb)(indy_handle_t xcommand_handle,
                                                                            indy_error_t  err,
                                                                            const char*   claim_offer_json)
                                                       );
    
    extern indy_error_t indy_issuer_create_claim(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  claim_req_json,
                                                 const char *  claim_json,
                                                 indy_i32_t    user_revoc_index, //option??

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   revoc_reg_update_json,
                                                                      const char*   xclaim_json   )
                                                 );
    
    
    extern indy_error_t indy_issuer_revoke_claim(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  issuer_did,
                                                 const char *  schema_json,
                                                 indy_u32_t    user_revoc_index,

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   revoc_reg_update_json)
                                                 );
    
    extern indy_error_t indy_prover_store_claim_offer(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      const char *  claim_offer_json,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err)
                                                      );
    
    
    extern indy_error_t indy_prover_get_claim_offers(indy_handle_t command_handle,
                                                     indy_handle_t wallet_handle,
                                                     const char *  filter_json,
                                                     void           (*cb)(indy_handle_t xcommand_handle,
                                                                          indy_error_t  err,
                                                                          const char*   claim_offers_json)
                                                     );
    
    
    extern indy_error_t indy_prover_create_master_secret(indy_handle_t command_handle,
                                                         indy_handle_t wallet_handle,
                                                         const char *  master_secret_name,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err)
                                                         );
    
    
    extern indy_error_t indy_prover_create_and_store_claim_req(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  prover_did,
                                                               const char *  claim_offer_json,
                                                               const char *  claim_def_json,
                                                               const char *  master_secret_name,

                                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                                    indy_error_t  err,
                                                                                    const char*   claim_req_json)
                                                               );
    
    
    
    extern indy_error_t indy_prover_store_claim(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  claims_json,
                                                const char *  rev_reg_json,

                                                void           (*cb)(indy_handle_t xcommand_handle,
                                                                     indy_error_t  err)
                                                );
    
    extern indy_error_t indy_prover_get_claims(indy_handle_t command_handle,
                                               indy_handle_t wallet_handle,
                                               const char *  filter_json,

                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                    indy_error_t  err,
                                                                    const char*   claims_json)
                                               );
    
    
    extern indy_error_t indy_prover_get_claims_for_proof_req(indy_handle_t command_handle,
                                                             indy_handle_t wallet_handle,
                                                             const char *  proof_request_json,

                                                             void           (*cb)(indy_handle_t xcommand_handle,
                                                                                  indy_error_t  err,
                                                                                  const char*   claims_json)
                                                             );
    
    
    extern indy_error_t indy_prover_create_proof(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  proof_req_json,
                                                 const char *  requested_claims_json,
                                                 const char *  schemas_json,
                                                 const char *  master_secret_name,
                                                 const char *  claim_defs_json,
                                                 const char *  revoc_regs_json,

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   proof_json)
                                                 );
    
    
    extern indy_error_t indy_verifier_verify_proof(indy_handle_t command_handle,
                                                   const char *  proof_request_json,
                                                   const char *  proof_json,
                                                   const char *  schemas_json,
                                                   const char *  claim_defs_jsons,
                                                   const char *  revoc_regs_json,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        indy_bool_t   valid )
                                                   );
    
#ifdef __cplusplus
}
#endif

#endif
