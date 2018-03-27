#ifndef __indy__ledger_included__
#define __indy__ledger_included__

#include "indy_mod.h"
#include "indy_types.h"

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
    /// pool_handle: pool handle (created by open_pool_ledger).
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
    
    extern indy_error_t indy_sign_and_submit_request(indy_handle_t command_handle,
                                                     indy_handle_t pool_handle,
                                                     indy_handle_t wallet_handle,
                                                     const char *  submitter_did,
                                                     const char *  request_json,

                                                     void           (*cb)(indy_handle_t xcommand_handle,
                                                                          indy_error_t  err,
                                                                          const char*   request_result_json)
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
    
    extern indy_error_t indy_submit_request(indy_handle_t command_handle,
                                            indy_handle_t pool_handle,
                                            const char *  request_json,

                                            void           (*cb)(indy_handle_t xcommand_handle,
                                                                 indy_error_t  err,
                                                                 const char*   request_result_json)
                                           );


    /// Signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see wallet_sign).
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// wallet_handle: wallet handle (created by open_wallet).
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// request_json: Request data json.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Signed request json.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
    /// Crypto*

    extern indy_error_t indy_sign_request(indy_handle_t command_handle,
                                         indy_handle_t  wallet_handle,
                                         const char *   submitter_did,
                                         const char *   request_json,

                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                              indy_error_t  err,
                                                              const char*   signed_request_json)
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
    
    extern indy_error_t indy_build_get_ddo_request(indy_handle_t command_handle,
                                                   const char *  submitter_did,
                                                   const char *  target_did,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        const char*   request_result_json)
                                                  );
    
    /// Builds a NYM request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// target_did: Id of Identity stored in secured Wallet.
    /// verkey: verification key
    /// alias
    /// role: Role of a user NYM record
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_nym_request(indy_handle_t command_handle,
                                               const char *  submitter_did,
                                               const char *  target_did,
                                               const char *  verkey,
                                               const char *  alias,
                                               const char *  role,

                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                    indy_error_t  err,
                                                                    const char*   request_json)
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

    extern indy_error_t indy_build_attrib_request(indy_handle_t command_handle,
                                                  const char *  submitter_did,
                                                  const char *  target_did,
                                                  const char *  hash,
                                                  const char *  raw,
                                                  const char *  enc,

                                                  void           (*cb)(indy_handle_t xcommand_handle,
                                                                       indy_error_t  err,
                                                                       const char*   request_json)
                                                  );

    /// Builds a GET_ATTRIB request.
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
    
    extern indy_error_t indy_build_get_attrib_request(indy_handle_t command_handle,
                                                      const char *  submitter_did,
                                                      const char *  target_did,
                                                      const char *  hash,
                                                      const char *  raw,
                                                      const char *  enc,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   request_json)
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

    extern indy_error_t indy_build_get_nym_request(indy_handle_t command_handle,
                                                   const char *  submitter_did,
                                                   const char *  target_did,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        const char*   request_json)
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

    extern indy_error_t indy_build_schema_request(indy_handle_t command_handle,
                                                  const char *  submitter_did,
                                                  const char *  data,

                                                  void           (*cb)(indy_handle_t xcommand_handle,
                                                                       indy_error_t  err,
                                                                       const char*   request_json)
                                                 );

    /// Builds a GET_SCHEMA request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// dest: Id of Identity stored in secured Wallet.
    /// data: name, version
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern indy_error_t indy_build_get_schema_request(indy_handle_t command_handle,
                                                      const char *  submitter_did,
                                                      const char *  dest,
                                                      const char *  data,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   request_json)
                                                     );
    
    /// Builds an CLAIM_DEF request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// xref: Seq. number of schema
    /// signature_type
    /// data: components of a key in json: N, R, S, Z
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern indy_error_t indy_build_claim_def_txn(indy_handle_t command_handle,
                                                 const char *  submitter_did,
                                                 indy_i32_t  xref,
                                                 const char *  signature_type,
                                                 const char *  data,

                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err,
                                                                      const char*   request_json)
                                                 );
    
    /// Builds a GET_CLAIM_DEF request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// xref: Seq. number of schema
    /// signature_type: signature type (only CL supported now)
    /// origin: issuer did
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

     extern indy_error_t indy_build_get_claim_def_txn(indy_handle_t command_handle,
                                                      const char *  submitter_did,
                                                      indy_i32_t  xref,
                                                      const char *  signature_type,
                                                      const char *  origin,
                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   request_json)
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
    
    extern indy_error_t indy_build_node_request(indy_handle_t command_handle,
                                                const char *  submitter_did,
                                                const char *  target_did,
                                                const char *  data,

                                                void           (*cb)(indy_handle_t xcommand_handle,
                                                                     indy_error_t  err,
                                                                     const char*   request_json)
                                               );

    /// Builds a GET_TXN request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// data: seq_no of transaction in ledger
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_get_txn_request(indy_handle_t command_handle,
                                                   const char *  submitter_did,
                                                   indy_i32_t    data,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        const char*   request_json)
                                                   );

    /// Builds a POOL_CONFIG request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// writes:
    /// force:
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_pool_config_request(indy_handle_t command_handle,
                                                       const char *  submitter_did,
                                                       indy_bool_t    writes,
                                                       indy_bool_t    force,

                                                       void           (*cb)(indy_handle_t xcommand_handle,
                                                                            indy_error_t  err,
                                                                            const char*   request_json)
                                                       );

    /// Builds a POOL_UPGRADE request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// name:
    /// action: Either start or cancel
    /// sha256:
    /// timeout:
    /// schedule:
    /// justification:
    /// reinstall:
    /// force:
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_pool_upgrade_request(indy_handle_t command_handle,
                                                        const char *  submitter_did,
                                                        const char *  name,
                                                        const char *  version,
                                                        const char *  action,
                                                        const char *  sha256,
                                                        indy_i32_t    timeout,
                                                        const char *  schedule,
                                                        const char *  justification,
                                                        indy_bool_t   reinstall,
                                                        indy_bool_t   force,

                                                        void           (*cb)(indy_handle_t xcommand_handle,
                                                                             indy_error_t  err,
                                                                             const char*   request_json)
                                                        );
    
#ifdef __cplusplus
}
#endif

#endif
