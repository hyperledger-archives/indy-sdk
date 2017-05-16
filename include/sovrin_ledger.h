#ifndef __sovrin__ledger_included__
#define __sovrin__ledger_included__

#include "sovrin_mod.h"
#include "sovrin_types.h"

#ifdef __cplusplus
extern "C" {
#endif
    
    /// Signs and submits request message to validator pool.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see wallet_sign), and sends signed request message
    /// to validator pool (see write_request).
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// wallet_handle: wallet handle (created by open_wallet).
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// request_json: Request data json.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
    /// Crypto*
    
    extern sovrin_error_t sovrin_sign_and_submit_request(sovrin_handle_t command_handle,
                                                         sovrin_handle_t wallet_handle,
                                                         const char *    submitter_did,
                                                         const char *    request_json,
                                                     
                                                         void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                              sovrin_error_t  err,
                                                                              const char*     request_result_json)
                                                         );
    
    /// Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
    ///
    /// The request is sent to the validator pool as is. It's assumed that it's already prepared.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// pool_handle: pool handle (created by open_pool_ledger).
    /// request_json: Request data json.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    
    extern sovrin_error_t sovrin_submit_request(sovrin_handle_t command_handle,
                                                sovrin_handle_t pool_handle,
                                                const char *    request_json,
                                                         
                                                void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                     sovrin_error_t  err,
                                                                     const char*     request_result_json)
                                               );

    /// Builds a request to get a DDO.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_get_ddo_request(sovrin_handle_t command_handle,
                                                       const char *    submitter_did,
                                                       const char *    target_did,

                                                       void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                            sovrin_error_t  err,
                                                                            const char*     request_result_json)
                                                      );
    
    /// Builds a NYM request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// verkey: verification key
    /// xref: id of a NYM record
    /// data: alias
    /// role: Role of a user NYM record
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern sovrin_error_t sovrin_build_nym_request(sovrin_handle_t command_handle,
                                                   const char *    submitter_did,
                                                   const char *    target_did,
                                                   const char *    verkey,
                                                   const char *    xref,
                                                   const char *    data,
                                                   const char *    role,
                                                   
                                                   void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                        sovrin_error_t  err,
                                                                        const char*     request_json)
                                                  );

    /// Builds an ATTRIB request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// hash: Hash of attribute data
    /// raw: represented as json, where key is attribute name and value is it's value
    /// enc: Encrypted attribute data
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern sovrin_error_t sovrin_build_attrib_request(sovrin_handle_t command_handle,
                                                      const char *    submitter_did,
                                                      const char *    target_did,
                                                      const char *    hash,
                                                      const char *    raw,
                                                      const char *    enc,
                                                   
                                                      void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                           sovrin_error_t  err,
                                                                           const char*     request_json)
                                                      );

    /// Builds a GET_ATTRIB request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// data: name (attribute name)
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_attrib_request(sovrin_handle_t command_handle,
                                                      const char *    submitter_did,
                                                      const char *    target_did,
                                                      const char *    data,
                                                      
                                                      void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                           sovrin_error_t  err,
                                                                           const char*     request_json)
                                                      );
    /// Builds a GET_NYM request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern sovrin_error_t sovrin_build_get_nym_request(sovrin_handle_t command_handle,
                                                       const char *    submitter_did,
                                                       const char *    target_did,
                                                      
                                                       void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                            sovrin_error_t  err,
                                                                            const char*     request_json)
                                                      );

    /// Builds a SCHEMA request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// data: name, version, type, attr_names (ip, port, keys)
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern sovrin_error_t sovrin_build_schema_request(sovrin_handle_t command_handle,
                                                      const char *    submitter_did,
                                                      const char *    data,

                                                      void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                           sovrin_error_t  err,
                                                                           const char*     request_json)
                                                     );

    /// Builds a GET_SCHEMA request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// data: name, version
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_get_schema_request(sovrin_handle_t command_handle,
                                                          const char *    submitter_did,
                                                          const char *    data,
                                                      
                                                          void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                               sovrin_error_t  err,
                                                                               const char*     request_json)
                                                         );
    
    /// Builds an CLAIM_DEF request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// xref: Seq. number of schema
    /// data: components of a key in json: N, R, S, Z
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_claim_def_txn(sovrin_handle_t command_handle,
                                                     const char *    submitter_did,
                                                     const char *    xref,
                                                     const char *    data,
                                                          
                                                     void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                          sovrin_error_t  err,
                                                                          const char*     request_json)
                                                     );
    
    /// Builds a GET_CLAIM_DEF request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// xref: Seq. number of schema
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_get_claim_def_txn(sovrin_handle_t command_handle,
                                                         const char *    submitter_did,
                                                         const char *    xref,
                                                     
                                                         void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                              sovrin_error_t  err,
                                                                              const char*     request_json)
                                                        );

    /// Builds a NODE request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// data: id of a target NYM record
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern sovrin_error_t sovrin_build_node_request(sovrin_handle_t command_handle,
                                                    const char *    submitter_did,
                                                    const char *    target_did,
                                                    const char *    data,
                                                         
                                                    void           (*cb)(sovrin_handle_t xcommand_handle,
                                                                         sovrin_error_t  err,
                                                                         const char*     request_json)
                                                   );
    
#ifdef __cplusplus
}
#endif

#endif
