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


    /// Multi signs request message.
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

    extern indy_error_t indy_multi_sign_request(indy_handle_t command_handle,
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
    
    /// Builds a NYM request. Request to create a new NYM record for a specific user.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// verkey: Target identity verification key as base58-encoded string.
    /// alias: NYM's alias.
    /// role: Role of a user NYM record:
    ///                             null (common USER)
    ///                             TRUSTEE
    ///                             STEWARD
    ///                             TRUST_ANCHOR
    ///                             empty string to reset role
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

    /// Builds an ATTRIB request. Request to add attribute to a NYM record.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// hash: (Optional) Hash of attribute data.
    /// raw: (Optional) Json, where key is attribute name and value is attribute value.
    /// enc: (Optional) Encrypted value attribute data.
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

    /// Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// raw: (Optional) Requested attribute name.
    /// hash: (Optional) Requested attribute hash.
    /// enc: (Optional) Requested attribute encrypted value.
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

    /// Builds a GET_NYM request. Request to get information about a DID (NYM).
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// target_did: Target DID as base58-encoded string for 16 or 32 bit DID value.
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

    /// Builds a SCHEMA request. Request to add Credential's schema.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// data: Credential schema.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string,
    ///     ver: Version of the Schema json
    /// }
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

    /// Builds a GET_SCHEMA request. Request to get Credential's Schema.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// id: Schema ID in ledger
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern indy_error_t indy_build_get_schema_request(indy_handle_t command_handle,
                                                      const char *  submitter_did,
                                                      const char *  id,

                                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                                           indy_error_t  err,
                                                                           const char*   request_json)
                                                     );

    /// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// get_schema_response: response of GET_SCHEMA request.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Schema Id and Schema json.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string
    ///     ver: Version of the Schema json
    /// }
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_parse_get_schema_response(indy_handle_t command_handle,
                                                       const char *  get_schema_response,

                                                       void           (*cb)(indy_handle_t xcommand_handle,
                                                                            indy_error_t  err,
                                                                            const char*   schema_id,
                                                                            const char*   schema_json)
                                                       );
    
    /// Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// data: credential definition json
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the CredDef json
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*
    
    extern indy_error_t indy_build_cred_def_request(indy_handle_t command_handle,
                                                    const char *  submitter_did,
                                                    const char *  data,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   request_json)
                                                    );
    
    /// Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// id: Credential Definition ID in ledger.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

     extern indy_error_t indy_build_get_cred_def_request(indy_handle_t command_handle,
                                                         const char *  submitter_did,
                                                         const char *  id,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err,
                                                                              const char*   request_json)
                                                         );

    /// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// get_cred_def_response: response of GET_CRED_DEF request.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Credential Definition Id and Credential Definition json.
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the Credential Definition json
    /// }
    ///
    /// #Errors
    /// Common*

     extern indy_error_t indy_parse_get_cred_def_response(indy_handle_t command_handle,
                                                          const char *  get_cred_def_response,
                                                          void           (*cb)(indy_handle_t xcommand_handle,
                                                                               indy_error_t  err,
                                                                               const char*   cred_def_id,
                                                                               const char*   cred_def_json)
                                                          );

    /// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// target_did: Target Node's DID.  It differs from submitter_did field.
    /// data: Data associated with the Node: {
    ///     alias: string - Node's alias
    ///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
    ///     blskey_pop: string - (Optional) BLS key proof of possession as base58-encoded string.
    ///     client_ip: string - (Optional) Node's client listener IP address.
    ///     client_port: string - (Optional) Node's client listener port.
    ///     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
    ///     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
    ///     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
    /// }
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

        /// Builds a GET_VALIDATOR_INFO request.
        ///
        /// #Params
        /// command_handle: command handle to map callback to caller context.
        /// submitter_did: Id of Identity stored in secured Wallet.
        /// cb: Callback that takes command result as parameter.
        ///
        /// #Returns
        /// Request result as json.
        ///
        /// #Errors
        /// Common*

        extern indy_error_t indy_build_get_validator_info_request(indy_handle_t command_handle,
                                                       const char *  submitter_did,
                                                       void           (*cb)(indy_handle_t xcommand_handle,
                                                                            indy_error_t  err,
                                                                            const char*   request_json)
                                                       );


    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the request submitter.
    /// ledger_type: (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    ///     any number
    /// seq_no: seq_no of transaction in ledger.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_get_txn_request(indy_handle_t command_handle,
                                                   const char *  submitter_did,
                                                   const char *  ledger_type,
                                                   indy_i32_t    data,

                                                   void           (*cb)(indy_handle_t xcommand_handle,
                                                                        indy_error_t  err,
                                                                        const char*   request_json)
                                                   );

    /// Builds a POOL_CONFIG request. Request to change Pool's configuration.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// writes: Whether any write requests can be processed by the pool
    ///         (if false, then pool goes to read-only state). True by default.
    /// force: Whether we should apply transaction (for example, move pool to read-only state)
    ///        without waiting for consensus of this transaction.
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

    /// Builds a POOL_RESTART request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: Id of Identity stored in secured Wallet.
    /// action: Either start or cancel
    /// datetime:
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_pool_restart_request(indy_handle_t command_handle,
                                                        const char *  submitter_did,
                                                        const char *  action,
                                                        const char *  datetime,
                                                        void           (*cb)(indy_handle_t xcommand_handle,
                                                                             indy_error_t  err,
                                                                             const char*   request_json)
                                                        );

    /// Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    /// It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// name: Human-readable name for the upgrade.
    /// version: The version of indy-node package we perform upgrade to.
    ///          Must be greater than existing one (or equal if reinstall flag is True).
    /// action: Either start or cancel.
    /// sha256: sha256 hash of the package.
    /// timeout: (Optional) Limits upgrade time on each Node.
    /// schedule: (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
    /// justification: (Optional) justification string for this particular Upgrade.
    /// reinstall: Whether it's allowed to re-install the same version. False by default.
    /// force: Whether we should apply transaction (schedule Upgrade) without waiting
    ///        for consensus of this transaction.
    /// package: (Optional) Package to be upgraded.
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
                                                        const char *  package_,

                                                        void           (*cb)(indy_handle_t xcommand_handle,
                                                                             indy_error_t  err,
                                                                             const char*   request_json)
                                                        );

    /// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    /// to an exists credential definition.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// data: Revocation Registry data:
    ///     {
    ///         "id": string - ID of the Revocation Registry,
    ///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///         "tag": string - Unique descriptive ID of the Registry,
    ///         "credDefId": string - ID of the corresponding CredentialDefinition,
    ///         "value": Registry-specific data {
    ///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///             "tailsHash": string - Hash of tails.
    ///             "tailsLocation": string - Location of tails file.
    ///             "publicKeys": <public_keys> - Registry's public key.
    ///         },
    ///         "ver": string - version of revocation registry definition json.
    ///     }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_revoc_reg_def_request(indy_handle_t command_handle,
                                                         const char *  submitter_did,
                                                         const char *  data,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err,
                                                                              const char*   request_json)
                                                         );

    /// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    /// that Issuer creates for a particular Credential Definition.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// id:  ID of Revocation Registry Definition in ledger.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_get_revoc_reg_def_request(indy_handle_t command_handle,
                                                             const char *  submitter_did,
                                                             const char *  id,

                                                             void           (*cb)(indy_handle_t xcommand_handle,
                                                                                  indy_error_t  err,
                                                                                  const char*   request_json)
                                                            );

    /// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
    /// compatible with Anoncreds API.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// get_revoc_reg_def_response: response of GET_REVOC_REG_DEF request.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Revocation Registry Definition Id and Revocation Registry Definition json.
    /// {
    ///     "id": string - ID of the Revocation Registry,
    ///     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///     "tag": string - Unique descriptive ID of the Registry,
    ///     "credDefId": string - ID of the corresponding CredentialDefinition,
    ///     "value": Registry-specific data {
    ///         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///         "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///         "tailsHash": string - Hash of tails.
    ///         "tailsLocation": string - Location of tails file.
    ///         "publicKeys": <public_keys> - Registry's public key.
    ///     },
    ///     "ver": string - version of revocation registry definition json.
    /// }
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_parse_get_revoc_reg_def_response(indy_handle_t command_handle,
                                                              const char *  get_revoc_ref_def_response,

                                                              void           (*cb)(indy_handle_t xcommand_handle,
                                                                                   indy_error_t  err,
                                                                                   const char*   revoc_reg_def_id,
                                                                                   const char*   revoc_reg_def_json)
                                                             );

    /// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    /// the new accumulator value and issued/revoked indices.
    /// This is just a delta of indices, not the whole list.
    /// So, it can be sent each time a new credential is issued/revoked.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// revoc_reg_def_id: ID of the corresponding RevocRegDef.
    /// rev_def_type: Revocation Registry type (only CL_ACCUM is supported for now).
    /// value: Registry-specific data: {
    ///     value: {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     ver: string - version revocation registry entry json
    ///
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_revoc_reg_entry_request(indy_handle_t command_handle,
                                                           const char *  submitter_did,
                                                           const char *  revoc_reg_def_id,
                                                           const char *  rev_def_type,
                                                           const char *  value,

                                                           void           (*cb)(indy_handle_t xcommand_handle,
                                                                                indy_error_t  err,
                                                                                const char*   request_json)
                                                          );

    /// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    /// by ID. The state is defined by the given timestamp.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// revoc_reg_def_id:  ID of the corresponding Revocation Registry Definition in ledger.
    /// timestamp: Requested time represented as a total number of seconds from Unix Epoch
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_get_revoc_reg_request(indy_handle_t command_handle,
                                                         const char *  submitter_did,
                                                         const char *  revoc_reg_def_id,
                                                         long long    timestamp,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err,
                                                                              const char*   request_json)
                                                        );

    /// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// get_revoc_reg_response: response of GET_REVOC_REG request.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Revocation Registry Definition Id, Revocation Registry json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         "accum": string - current accumulator value
    ///     },
    ///     "ver": string - version revocation registry json
    /// }
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_parse_get_revoc_reg_response(indy_handle_t command_handle,
                                                          const char *  get_revoc_reg_response,

                                                          void           (*cb)(indy_handle_t xcommand_handle,
                                                                               indy_error_t  err,
                                                                               const char*   revoc_reg_def_id,
                                                                               const char*   revoc_reg_json,
                                                                               unsigned long long      timestamp)
                                                         );

    /// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    /// The Delta is defined by from and to timestamp fields.
    /// If from is not specified, then the whole state till to will be returned.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// submitter_did: DID of the read request sender.
    /// revoc_reg_def_id:  ID of the corresponding Revocation Registry Definition in ledger.
    /// from: Requested time represented as a total number of seconds from Unix Epoch
    /// to: Requested time represented as a total number of seconds from Unix Epoch
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_build_get_revoc_reg_delta_request(indy_handle_t command_handle,
                                                               const char *  submitter_did,
                                                               const char *  revoc_reg_def_id,
                                                               long long    from,
                                                               long long    to,

                                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                                    indy_error_t  err,
                                                                                    const char*   request_json)
                                                              );

    /// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// get_revoc_reg_response: response of GET_REVOC_REG_DELTA request.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     "ver": string - version revocation registry delta json
    /// }
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_parse_get_revoc_reg_delta_response(indy_handle_t command_handle,
                                                                const char *  get_revoc_reg_delta_response,

                                                                void           (*cb)(indy_handle_t xcommand_handle,
                                                                                     indy_error_t  err,
                                                                                     const char*   revoc_reg_def_id,
                                                                                     const char*   revoc_reg_delta_json,
                                                                                     unsigned long long      timestamp)
                                                               );

#ifdef __cplusplus
}
#endif

#endif
