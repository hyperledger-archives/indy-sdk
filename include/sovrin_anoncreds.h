#ifndef __anoncreds__included__
#define __anoncreds__included__

extern "C"
{
    
    extern sovrin_error_t sovrin_issuer_create_and_store_claim_def(sovrin_handle_t command_handle,
                                                                   sovrin_handle_t wallet_handle,
                                                                   const char *    schema_json,
                                                                   const char *    signature_type,
                                                                   sovrin_bool_t   create_non_revoc,
                                                                   
                                                                   sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                                        sovrin_error_t  err,
                                                                                        const char*     clain_def_json,
                                                                                        const char*     claim_def_uuid)
                                                                   );
    
    extern sovrin_error_t sovrin_issuer_create_and_store_revoc_reg(sovrin_handle_t command_handle,
                                                                   sovrin_handle_t wallet_handle,
                                                                   sovrin_i32_t    claim_def_seq_no,
                                                                   sovrin_i32_t    max_claim_num,
                                                                   
                                                                   sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                                        sovrin_error_t  err,
                                                                                        const char*     revoc_reg_json,
                                                                                        const char*     revoc_reg_uuid   )
                                                                   );
    
    extern sovrin_error_t sovrin_issuer_create_claim(sovrin_handle_t command_handle,
                                                     sovrin_handle_t wallet_handle,
                                                     const char *    claim_req_json,
                                                     const char *    claim_json,
                                                     sovrin_i32_t    revoc_reg_seq_no, //option??
                                                     sovrin_i32_t    user_revoc_index, //option??
                                                     
                                                     sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                          sovrin_error_t  err,
                                                                          const char*     revoc_reg_update_json,
                                                                          const char*     xclaim_json   )
                                                     );
    
    
    extern sovrin_error_t sovrin_issuer_revoke_claim(sovrin_handle_t command_handle,
                                                     sovrin_handle_t wallet_handle,
                                                     sovrin_i32_t    claim_def_seq_no,
                                                     sovrin_i32_t    revoc_reg_seq_no,
                                                     sovrin_i32_t    user_revoc_index,
                                                     
                                                     sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                          sovrin_error_t  err,
                                                                          const char*     revoc_reg_update_json)
                                                     );
    
    extern sovrin_error_t sovrin_prover_store_claim_offer(sovrin_handle_t command_handle,
                                                          sovrin_handle_t wallet_handle,
                                                          const char *    claim_offer_json,
                                                          
                                                          sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                               sovrin_error_t  err)
                                                          );
    
    
    extern sovrin_error_t sovrin_prover_get_claim_offers(sovrin_handle_t command_handle,
                                                         sovrin_handle_t wallet_handle,
                                                         const char *filter_json,
                                                         sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                              sovrin_error_t  err,
                                                                              const char*    claim_offers_json)
                                                         );
    
    
    extern sovrin_error_t sovrin_prover_create_master_secret(sovrin_handle_t command_handle,
                                                             sovrin_handle_t wallet_handle,
                                                             const char *    master_secret_name,
                                                             
                                                             sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                                  sovrin_error_t  err)
                                                             );
    
    
    extern sovrin_error_t sovrin_prover_create_and_store_claim_req(sovrin_handle_t command_handle,
                                                                   sovrin_handle_t wallet_handle,
                                                                   const char *    prover_did,
                                                                   const char *    claim_offer_json,
                                                                   const char *    claim_def_json,
                                                                   const char *    master_secret_name,
                                                                   
                                                                   sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                                        sovrin_error_t  err,
                                                                                        const char*    claim_req_json)
                                                                   );
    
    
    
    extern sovrin_error_t sovrin_prover_store_claim(sovrin_handle_t command_handle,
                                                    sovrin_handle_t wallet_handle,
                                                    const char *    claims_json,
                                                    
                                                    sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                         sovrin_error_t  err)
                                                    );
    
    extern sovrin_error_t sovrin_prover_get_claims(sovrin_handle_t command_handle,
                                                   sovrin_handle_t wallet_handle,
                                                   const char *    filter_json,
                                                   
                                                   sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                        sovrin_error_t  err,
                                                                        const char*     claims_json)
                                                   );
    
    
    extern sovrin_error_t sovrin_prover_get_claims_for_proof_req(sovrin_handle_t command_handle,
                                                                 sovrin_handle_t wallet_handle,
                                                                 const char *    proof_request_json,
                                                                 
                                                                 sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                                      sovrin_error_t  err,
                                                                                      const char*     claims_json)
                                                                 );
    
    
    extern sovrin_error_t sovrin_prover_create_proof(sovrin_handle_t command_handle,
                                                     sovrin_handle_t wallet_handle,
                                                     const char *    proof_req_json,
                                                     const char *    requested_claims_json,
                                                     const char *    schemas_json,
                                                     const char *    master_secret_name,
                                                     const char *    claim_defs_json,
                                                     const char *    revoc_regs_json,
                                                     
                                                     sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                          sovrin_error_t  err,
                                                                          const char*     proof_json)
                                                     );
    
    
    extern sovrin_error_t sovrin_verifier_verify_proof(sovrin_handle_t command_handle,
                                                       sovrin_handle_t wallet_handle,
                                                       const char *    proof_request_json,
                                                       const char *    proof_json,
                                                       const char *    schemas_json,
                                                       const char *    claim_defs_jsons,
                                                       const char *    revoc_regs_json,
                                                       
                                                       sovrin_error_t (*cb)(sovrin_handle_t xcommand_handle,
                                                                            sovrin_error_t  err,
                                                                            sovrin_bool_t   valid )
                                                       );
    
}
#endif
