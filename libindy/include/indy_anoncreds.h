#ifndef __anoncreds__included__
#define __anoncreds__included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

    /// Create credential schema entity that describes credential attributes list and allows credentials
    /// interoperability.
    ///
    /// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
    /// to Indy distributed ledger.
    ///
    /// It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
    /// with correct seq_no to save compatibility with Ledger.
    /// After that can call indy_issuer_create_and_store_credential_def to build corresponding Credential Definition.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context
    /// issuer_did: DID of schema issuer
    /// name: a name the schema
    /// version: a version of the schema
    /// attrs: a list of schema attributes descriptions
    /// cb: Callback that takes command result as parameter
    ///
    /// #Returns
    /// schema_id: identifier of created schema
    /// schema_json: schema as json
    ///
    /// #Errors
    /// Common*
    /// Anoncreds*
    extern indy_error_t indy_issuer_create_schema(indy_handle_t command_handle,
                                                  const char *  issuer_did,
                                                  const char *  name,
                                                  const char *  version,
                                                  const char *  attr_names,
                                                  indy_str_str_cb cb
                                                  );

    /// Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
    /// and secrets used for credentials revocation.
    ///
    /// Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
    /// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
    /// to Indy distributed ledger.
    ///
    /// It is IMPORTANT for current version GET Schema from Ledger with correct seq_no to save compatibility with Ledger.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// issuer_did: a DID of the issuer signing cred_def transaction to the Ledger
    /// schema_json: credential schema as a json
    /// tag: allows to distinct between credential definitions for the same issuer and schema
    /// signature_type: credential definition type (optional, 'CL' by default) that defines credentials signature and revocation math. Supported types are:
    /// - 'CL': Camenisch-Lysyanskaya credential signature type
    /// config_json: (optional) type-specific configuration of credential definition as json:
    /// - 'CL':
    ///   - support_revocation: whether to request non-revocation credential (optional, default false)
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// cred_def_id: identifier of created credential definition
    /// cred_def_json: public part of created credential definition
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Anoncreds*
    extern indy_error_t indy_issuer_create_and_store_credential_def(indy_handle_t command_handle,
                                                                    indy_handle_t wallet_handle,
                                                                    const char *  issuer_did,
                                                                    const char *  schema_json,
                                                                    const char *  tag,
                                                                    const char *  signature_type,
                                                                    const char *  config_json,
                                                                    indy_str_str_cb cb
                                                                    );

    /// Create a new revocation registry for the given credential definition as tuple of entities
    /// - Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
    ///   secrets used for credentials revocation
    /// - Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
    ///   represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.
    ///
    /// Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
    /// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC_REG_DEF transaction
    /// to Indy distributed ledger.
    ///
    /// Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC_REG_ENTRY transactions.
    /// This call initializes the state in the wallet and returns the initial entry.
    ///
    /// Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked credentials in public
    /// revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
    /// This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// issuer_did: a DID of the issuer signing transaction to the Ledger
    /// revoc_def_type: revocation registry type (optional, default value depends on credential definition type). Supported types are:
    /// - 'CL_ACCUM': Type-3 pairing based accumulator. Default for 'CL' credential definition type
    /// tag: allows to distinct between revocation registries for the same issuer and credential definition
    /// cred_def_id: id of stored in ledger credential definition
    /// config_json: type-specific configuration of revocation registry as json:
    /// - 'CL_ACCUM': {
    ///     "issuance_type": (optional) type of issuance. Currently supported:
    ///         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
    ///            Revocation Registry is updated only during revocation.
    ///         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
    ///     "max_cred_num": maximum number of credentials the new registry can process (optional, default 100000)
    /// }
    /// tails_writer_handle: handle of blob storage to store tails
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// revoc_reg_id: identifier of created revocation registry definition
    /// revoc_reg_def_json: public part of revocation registry definition
    /// revoc_reg_entry_json: revocation registry entry that defines initial state of revocation registry
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Anoncreds*
    extern indy_error_t indy_issuer_create_and_store_revoc_reg(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  issuer_did,
                                                               const char *  revoc_def_type,
                                                               const char *  tag,
                                                               const char *  cred_def_id,
                                                               const char *  config_json,
                                                               indy_handle_t tails_writer_handle,
                                                               indy_str_str_str_cb cb
                                                               );

    /// Create credential offer that will be used by Prover for
    /// credential request creation. Offer includes nonce and key correctness proof
    /// for authentication between protocol steps and integrity checking.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context
    /// wallet_handle: wallet handler (created by open_wallet)
    /// cred_def_id: id of credential definition stored in the wallet
    /// cb: Callback that takes command result as parameter
    ///
    /// #Returns
    /// credential offer json:
    ///     {
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         // Fields below can depend on Cred Def type
    ///         "nonce": string,
    ///         "key_correctness_proof" : <key_correctness_proof>
    ///     }
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Anoncreds*
    extern indy_error_t indy_issuer_create_credential_offer(indy_handle_t command_handle,
                                                            indy_handle_t wallet_handle,
                                                            const char *  cred_def_id,
                                                            indy_str_cb cb
                                                            );

    /// Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.
    ///
    /// Cred Request must match Cred Offer. The credential definition and revocation registry definition
    /// referenced in Cred Offer and Cred Request must be already created and stored into the wallet.
    ///
    /// Information for this credential revocation will be store in the wallet as part of revocation registry under
    /// generated cred_revoc_id local for this wallet.
    ///
    /// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
    /// Note that it is possible to accumulate deltas to reduce ledger load.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// cred_offer_json: a cred offer created by indy_issuer_create_credential_offer
    /// cred_req_json: a credential request created by indy_prover_create_credential_req
    /// cred_values_json: a credential containing attribute values for each of requested attribute names.
    ///     Example:
    ///     {
    ///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
    ///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
    ///     }
    /// rev_reg_id: id of revocation registry stored in the wallet
    /// blob_storage_reader_handle: configuration of blob storage reader handle that will allow to read revocation tails
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// cred_json: Credential json containing signed credential values
    ///     {
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_def_id", Optional<string>,
    ///         "values": <see cred_values_json above>,
    ///         // Fields below can depend on Cred Def type
    ///         "signature": <signature>,
    ///         "signature_correctness_proof": <signature_correctness_proof>
    ///     }
    /// cred_revoc_id: local id for revocation info (Can be used for revocation of this credential)
    /// revoc_reg_delta_json: Revocation registry delta json with a newly issued credential
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_issuer_create_credential(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      const char *  cred_offer_json,
                                                      const char *  cred_req_json,
                                                      const char *  cred_values_json,
                                                      const char *  rev_reg_id,
                                                      indy_i32_t    blob_storage_reader_handle,
                                                      indy_str_str_str_cb cb
                                                      );

    /// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
    ///
    /// The corresponding credential definition and revocation registry must be already
    /// created an stored into the wallet.
    ///
    /// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
    /// Note that it is possible to accumulate deltas to reduce ledger load.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// blob_storage_reader_cfg_handle: configuration of blob storage reader handle that will allow to read revocation tails
    /// rev_reg_id: id of revocation registry stored in wallet
    /// cred_revoc_id: local id for revocation info
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// revoc_reg_delta_json: Revocation registry delta json with a revoked credential
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_issuer_revoke_credential(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      indy_i32_t    blob_storage_reader_handle,
                                                      const char *  rev_reg_id,
                                                      const char *  cred_revoc_id,
                                                      indy_str_cb cb
                                                      );

/*
    /// Recover a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
    ///
    /// The corresponding credential definition and revocation registry must be already
    /// created an stored into the wallet.
    ///
    /// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
    /// Note that it is possible to accumulate deltas to reduce ledger load.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// blob_storage_reader_cfg_handle: configuration of blob storage reader handle that will allow to read revocation tails
    /// rev_reg_id: id of revocation registry stored in wallet
    /// cred_revoc_id: local id for revocation info
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// revoc_reg_delta_json: Revocation registry delta json with a recovered credential
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    #[no_mangle]
    extern indy_error_t indy_issuer_recover_credential(indy_handle_t command_handle,
                                                       indy_handle_t wallet_handle,
                                                       indy_i32_t    blob_storage_reader_handle,
                                                       const char *  rev_reg_id,
                                                       const char *  cred_revoc_id,
                                                       indy_str_cb cb
                                                       );*/

    /// Merge two revocation registry deltas (returned by indy_issuer_create_credential or indy_issuer_revoke_credential) to accumulate common delta.
    /// Send common delta to ledger to reduce the load.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// rev_reg_delta_json: revocation registry delta.
    /// other_rev_reg_delta_json: revocation registry delta for which PrevAccum value  is equal to current accum value of rev_reg_delta_json.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// merged_rev_reg_delta: Merged revocation registry delta
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_issuer_merge_revocation_registry_deltas(indy_handle_t command_handle,
                                                                     const char *  rev_reg_delta_json,
                                                                     const char *  other_rev_reg_delta_json,
                                                                     indy_str_cb cb
                                                                     );

    /// Creates a master secret with a given id and stores it in the wallet.
    /// The id must be unique.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// master_secret_id: (optional, if not present random one will be generated) new master id
    ///
    /// #Returns
    /// out_master_secret_id: Id of generated master secret
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_create_master_secret(indy_handle_t command_handle,
                                                         indy_handle_t wallet_handle,
                                                         const char *  master_secret_id,
                                                         indy_str_cb cb
                                                         );

    /// Creates a credential request for the given credential offer.
    ///
    /// The method creates a blinded master secret for a master secret identified by a provided name.
    /// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
    /// The blinded master secret is a part of the credential request.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context
    /// wallet_handle: wallet handler (created by open_wallet)
    /// prover_did: a DID of the prover
    /// cred_offer_json: credential offer as a json containing information about the issuer and a credential
    /// cred_def_json: credential definition json related to <cred_def_id> in <cred_offer_json>
    /// master_secret_id: the id of the master secret stored in the wallet
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// cred_req_json: Credential request json for creation of credential by Issuer
    ///     {
    ///      "prover_did" : string,
    ///      "cred_def_id" : string,
    ///         // Fields below can depend on Cred Def type
    ///      "blinded_ms" : <blinded_master_secret>,
    ///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
    ///      "nonce": string
    ///    }
    /// cred_req_metadata_json: Credential request metadata json for further processing of received form Issuer credential.
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_create_credential_req(indy_handle_t command_handle,
                                                          indy_handle_t wallet_handle,
                                                          const char *  prover_did,
                                                          const char *  cred_offer_json,
                                                          const char *  cred_def_json,
                                                          const char *  master_secret_id,
                                                          indy_str_str_cb cb
                                                          );

    /// Check credential provided by Issuer for the given credential request,
    /// updates the credential by a master secret and stores in a secure wallet.
    ///
    /// To support efficient and flexible search the following tags will be created for stored credential:
    ///     {
    ///         "schema_id": <credential schema id>,
    ///         "schema_issuer_did": <credential schema issuer did>,
    ///         "schema_name": <credential schema name>,
    ///         "schema_version": <credential schema version>,
    ///         "issuer_did": <credential issuer did>,
    ///         "cred_def_id": <credential definition id>,
    ///         "rev_reg_id": <credential revocation registry id>, // "None" as string if not present
    ///         // for every attribute in <credential values>
    ///         "attr::<attribute name>::marker": "1",
    ///         "attr::<attribute name>::value": <attribute raw value>,
    ///     }
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// cred_id: (optional, default is a random one) identifier by which credential will be stored in the wallet
    /// cred_req_metadata_json: a credential request metadata created by indy_prover_create_credential_req
    /// cred_json: credential json received from issuer
    /// cred_def_json: credential definition json related to <cred_def_id> in <cred_json>
    /// rev_reg_def_json: revocation registry definition json related to <rev_reg_def_id> in <cred_json>
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// out_cred_id: identifier by which credential is stored in the wallet
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_store_credential(indy_handle_t command_handle,
                                                     indy_handle_t wallet_handle,
                                                     const char *  cred_id,
                                                     const char *  cred_req_metadata_json,
                                                     const char *  cred_json,
                                                     const char *  cred_def_json,
                                                     const char *  rev_reg_def_json,
                                                     indy_str_cb cb
                                                     );

    /// Gets human readable credential by the given id.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// cred_id: Identifier by which requested credential is stored in the wallet
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// credential json:
    ///     {
    ///         "referent": string, // cred_id in the wallet
    ///         "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_id": Optional<string>,
    ///         "cred_rev_id": Optional<string>
    ///     }
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_get_credentials(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  filter_json,
                                                    indy_str_cb cb
                                                    );

    /// Gets human readable credentials according to the filter.
    /// If filter is NULL, then all credentials are returned.
    /// Credentials can be filtered by Issuer, credential_def and/or Schema.
    ///
    /// NOTE: This method is deprecated because immediately returns all fetched credentials.
    /// Use <indy_prover_search_credentials> to fetch records by small batches.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// filter_json: filter for credentials
    ///        {
    ///            "schema_id": string, (Optional)
    ///            "schema_issuer_did": string, (Optional)
    ///            "schema_name": string, (Optional)
    ///            "schema_version": string, (Optional)
    ///            "issuer_did": string, (Optional)
    ///            "cred_def_id": string, (Optional)
    ///        }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// credentials json
    ///     [{
    ///         "referent": string, // cred_id in the wallet
    ///         "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_id": Optional<string>,
    ///         "cred_rev_id": Optional<string>
    ///     }]
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_get_credential(indy_handle_t command_handle,
                                                   indy_handle_t wallet_handle,
                                                   const char *  cred_id,
                                                   indy_str_cb cb
                                                   );

    /// Search for credentials stored in wallet.
    /// Credentials can be filtered by tags created during saving of credential.
    ///
    /// Instead of immediately returning of fetched credentials
    /// this call returns search_handle that can be used later
    /// to fetch records by small batches (with indy_prover_fetch_credentials).
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// query_json: Wql query filter for credentials searching based on tags.
    /// where query: indy-sdk/doc/design/011-wallet-query-language/README.md
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// search_handle: Search handle that can be used later to fetch records by small batches (with indy_prover_fetch_credentials)
    /// total_count: Total count of records
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_search_credentials(indy_handle_t command_handle,
                                                       indy_handle_t wallet_handle,
                                                       const char *  query_json,
                                                       indy_handle_u32_cb cb
                                                       );

    /// Fetch next credentials for search.
    ///
    /// #Params
    /// search_handle: Search handle (created by indy_prover_search_credentials)
    /// count: Count of credentials to fetch
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// credentials_json: List of human readable credentials:
    ///     [{
    ///         "referent": string, // cred_id in the wallet
    ///         "attrs": {"key1":"raw_value1", "key2":"raw_value2"},
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_id": Optional<string>,
    ///         "cred_rev_id": Optional<string>
    ///     }]
    /// NOTE: The list of length less than the requested count means credentials search iterator is completed.
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_fetch_credentials(indy_handle_t command_handle,
                                                      indy_handle_t search_handle,
                                                      indy_u32_t    count,
                                                      indy_str_cb cb
                                                      );

    /// Close credentials search (make search handle invalid)
    ///
    /// #Params
    /// search_handle: Search handle (created by indy_prover_search_credentials)
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_close_credentials_search(indy_handle_t command_handle,
                                                             indy_handle_t search_handle,
                                                             indy_empty_cb cb
                                                             );

    /// Gets human readable credentials matching the given proof request.
    ///
    /// NOTE: This method is deprecated because immediately returns all fetched credentials.
    /// Use <indy_prover_search_credentials_for_proof_req> to fetch records by small batches.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// proof_request_json: proof request json
    ///     {
    ///         "name": string,
    ///         "version": string,
    ///         "nonce": string,
    ///         "requested_attributes": { // set of requested attributes
    ///              "<attr_referent>": <attr_info>, // see below
    ///              ...,
    ///         },
    ///         "requested_predicates": { // set of requested predicates
    ///              "<predicate_referent>": <predicate_info>, // see below
    ///              ...,
    ///          },
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval for each attribute
    ///                        // (can be overridden on attribute level)
    ///     }
    /// cb: Callback that takes command result as parameter.
    ///
    /// where
    /// attr_referent: Proof-request local identifier of requested attribute
    /// attr_info: Describes requested attribute
    ///     {
    ///         "name": string, // attribute name, (case insensitive and ignore spaces)
    ///         "restrictions": Optional<filter_json>, // see above
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval this attribute
    ///                        // (overrides proof level interval)
    ///     }
    /// predicate_referent: Proof-request local identifier of requested attribute predicate
    /// predicate_info: Describes requested attribute predicate
    ///     {
    ///         "name": attribute name, (case insensitive and ignore spaces)
    ///         "p_type": predicate type (Currently ">=" only)
    ///         "p_value": int predicate value
    ///         "restrictions": Optional<filter_json>, // see above
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval this attribute
    ///                        // (overrides proof level interval)
    ///     }
    /// non_revoc_interval: Defines non-revocation interval
    ///     {
    ///         "from": Optional<int>, // timestamp of interval beginning
    ///         "to": Optional<int>, // timestamp of interval ending
    ///     }
    ///
    /// #Returns
    /// credentials_json: json with credentials for the given proof request.
    ///     {
    ///         "requested_attrs": {
    ///             "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
    ///             ...,
    ///         },
    ///         "requested_predicates": {
    ///             "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
    ///             "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
    ///         }
    ///     }, where credential is
    ///     {
    ///         "referent": <string>,
    ///         "attrs": {"attr_name" : "attr_raw_value"},
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_id": Optional<int>,
    ///         "cred_rev_id": Optional<int>,
    ///     }
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_get_credentials_for_proof_req(indy_handle_t command_handle,
                                                                  indy_handle_t wallet_handle,
                                                                  const char *  proof_request_json,
                                                                  indy_str_cb cb
                                                                  );


    /// Search for credentials matching the given proof request.
    ///
    /// Instead of immediately returning of fetched credentials
    /// this call returns search_handle that can be used later
    /// to fetch records by small batches (with indy_prover_fetch_credentials_for_proof_req).
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// proof_request_json: proof request json
    ///     {
    ///         "name": string,
    ///         "version": string,
    ///         "nonce": string,
    ///         "requested_attributes": { // set of requested attributes
    ///              "<attr_referent>": <attr_info>, // see below
    ///              ...,
    ///         },
    ///         "requested_predicates": { // set of requested predicates
    ///              "<predicate_referent>": <predicate_info>, // see below
    ///              ...,
    ///          },
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval for each attribute
    ///                        // (can be overridden on attribute level)
    ///     }
    /// extra_query_json:(Optional) List of extra queries that will be applied to correspondent attribute/predicate:
    ///     {
    ///         "<attr_referent>": <wql query>,
    ///         "<predicate_referent>": <wql query>,
    ///     }
    /// where wql query: indy-sdk/doc/design/011-wallet-query-language/README.md
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// search_handle: Search handle that can be used later to fetch records by small batches (with indy_prover_fetch_credentials_for_proof_req)
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_search_credentials_for_proof_req(indy_handle_t command_handle,
                                                                     indy_handle_t wallet_handle,
                                                                     const char *  proof_request_json,
                                                                     const char *  extra_query_json,
                                                                     indy_handle_cb cb
                                                                     );

    /// Fetch next credentials for the requested item using proof request search
    /// handle (created by indy_prover_search_credentials_for_proof_req).
    ///
    /// #Params
    /// search_handle: Search handle (created by indy_prover_search_credentials_for_proof_req)
    /// item_referent: Referent of attribute/predicate in the proof request
    /// count: Count of credentials to fetch
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// credentials_json: List of credentials for the given proof request.
    ///     [{
    ///         cred_info: <credential_info>,
    ///         interval: Optional<non_revoc_interval>
    ///     }]
    /// where
    /// credential_info:
    ///     {
    ///         "referent": <string>,
    ///         "attrs": {"attr_name" : "attr_raw_value"},
    ///         "schema_id": string,
    ///         "cred_def_id": string,
    ///         "rev_reg_id": Optional<int>,
    ///         "cred_rev_id": Optional<int>,
    ///     }
    /// non_revoc_interval:
    ///     {
    ///         "from": Optional<int>, // timestamp of interval beginning
    ///         "to": Optional<int>, // timestamp of interval ending
    ///     }
    /// NOTE: The list of length less than the requested count means that search iterator
    /// correspondent to the requested <item_referent> is completed.
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_fetch_credentials_for_proof_req(indy_handle_t command_handle,
                                                                    indy_handle_t search_handle,
                                                                    const char*   item_referent,
                                                                    indy_u32_t    count,
                                                                    indy_str_cb cb
                                                                    );

    /// Close credentials search for proof request (make search handle invalid)
    ///
    /// #Params
    /// search_handle: Search handle (created by indy_prover_search_credentials_for_proof_req)
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_close_credentials_search_for_proof_req(indy_handle_t command_handle,
                                                                           indy_handle_t search_handle,
                                                                           indy_empty_cb cb
                                                                           );

    /// Creates a proof according to the given proof request
    /// Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
    /// for each requested attribute (see indy_prover_get_credentials_for_pool_req).
    /// A proof request may request multiple credentials from different schemas and different issuers.
    /// All required schemas, public keys and revocation registries must be provided.
    /// The proof request also contains nonce.
    /// The proof contains either proof or self-attested attribute value for each requested attribute.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// proof_request_json: proof request json
    ///     {
    ///         "name": string,
    ///         "version": string,
    ///         "nonce": string,
    ///         "requested_attributes": { // set of requested attributes
    ///              "<attr_referent>": <attr_info>, // see below
    ///              ...,
    ///         },
    ///         "requested_predicates": { // set of requested predicates
    ///              "<predicate_referent>": <predicate_info>, // see below
    ///              ...,
    ///          },
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval for each attribute
    ///                        // (can be overridden on attribute level)
    ///     }
    /// requested_credentials_json: either a credential or self-attested attribute for each requested attribute
    ///     {
    ///         "self_attested_attributes": {
    ///             "self_attested_attribute_referent": string
    ///         },
    ///         "requested_attributes": {
    ///             "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }},
    ///             "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }}
    ///         },
    ///         "requested_predicates": {
    ///             "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional<number> }},
    ///         }
    ///     }
    /// master_secret_id: the id of the master secret stored in the wallet
    /// schemas_json: all schemas json participating in the proof request
    ///     {
    ///         <schema1_id>: <schema1_json>,
    ///         <schema2_id>: <schema2_json>,
    ///         <schema3_id>: <schema3_json>,
    ///     }
    /// credential_defs_json: all credential definitions json participating in the proof request
    ///     {
    ///         "cred_def1_id": <credential_def1_json>,
    ///         "cred_def2_id": <credential_def2_json>,
    ///         "cred_def3_id": <credential_def3_json>,
    ///     }
    /// rev_states_json: all revocation states json participating in the proof request
    ///     {
    ///         "rev_reg_def1_id": {
    ///             "timestamp1": <rev_state1>,
    ///             "timestamp2": <rev_state2>,
    ///         },
    ///         "rev_reg_def2_id": {
    ///             "timestamp3": <rev_state3>
    ///         },
    ///         "rev_reg_def3_id": {
    ///             "timestamp4": <rev_state4>
    ///         },
    ///     }
    /// cb: Callback that takes command result as parameter.
    ///
    /// where
    /// where wql query: indy-sdk/doc/design/011-wallet-query-language/README.md
    /// attr_referent: Proof-request local identifier of requested attribute
    /// attr_info: Describes requested attribute
    ///     {
    ///         "name": string, // attribute name, (case insensitive and ignore spaces)
    ///         "restrictions": Optional<wql query>,
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval this attribute
    ///                        // (overrides proof level interval)
    ///     }
    /// predicate_referent: Proof-request local identifier of requested attribute predicate
    /// predicate_info: Describes requested attribute predicate
    ///     {
    ///         "name": attribute name, (case insensitive and ignore spaces)
    ///         "p_type": predicate type (Currently >= only)
    ///         "p_value": predicate value
    ///         "restrictions": Optional<wql query>,
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval this attribute
    ///                        // (overrides proof level interval)
    ///     }
    /// non_revoc_interval: Defines non-revocation interval
    ///     {
    ///         "from": Optional<int>, // timestamp of interval beginning
    ///         "to": Optional<int>, // timestamp of interval ending
    ///     }
    ///
    /// #Returns
    /// Proof json
    /// For each requested attribute either a proof (with optionally revealed attribute value) or
    /// self-attested attribute value is provided.
    /// Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
    /// There is also aggregated proof part common for all credential proofs.
    ///     {
    ///         "requested": {
    ///             "revealed_attrs": {
    ///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
    ///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
    ///             },
    ///             "unrevealed_attrs": {
    ///                 "requested_attr3_id": {sub_proof_index: number}
    ///             },
    ///             "self_attested_attrs": {
    ///                 "requested_attr2_id": self_attested_value,
    ///             },
    ///             "requested_predicates": {
    ///                 "requested_predicate_1_referent": {sub_proof_index: int},
    ///                 "requested_predicate_2_referent": {sub_proof_index: int},
    ///             }
    ///         }
    ///         "proof": {
    ///             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
    ///             "aggregated_proof": <aggregated_proof>
    ///         }
    ///         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
    ///     }
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_prover_create_proof(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  proof_req_json,
                                                 const char *  requested_credentials_json,
                                                 const char *  master_secret_name,
                                                 const char *  schemas_json,
                                                 const char *  credential_defs_json,
                                                 const char *  rev_states_json,
                                                 indy_str_cb cb
                                                 );


    /// Verifies a proof (of multiple credential).
    /// All required schemas, public keys and revocation registries must be provided.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// proof_request_json: proof request json
    ///     {
    ///         "name": string,
    ///         "version": string,
    ///         "nonce": string,
    ///         "requested_attributes": { // set of requested attributes
    ///              "<attr_referent>": <attr_info>, // see below
    ///              ...,
    ///         },
    ///         "requested_predicates": { // set of requested predicates
    ///              "<predicate_referent>": <predicate_info>, // see below
    ///              ...,
    ///          },
    ///         "non_revoked": Optional<<non_revoc_interval>>, // see below,
    ///                        // If specified prover must proof non-revocation
    ///                        // for date in this interval for each attribute
    ///                        // (can be overridden on attribute level)
    ///     }
    /// proof_json: created for request proof json
    ///     {
    ///         "requested": {
    ///             "revealed_attrs": {
    ///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
    ///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
    ///             },
    ///             "unrevealed_attrs": {
    ///                 "requested_attr3_id": {sub_proof_index: number}
    ///             },
    ///             "self_attested_attrs": {
    ///                 "requested_attr2_id": self_attested_value,
    ///             },
    ///             "requested_predicates": {
    ///                 "requested_predicate_1_referent": {sub_proof_index: int},
    ///                 "requested_predicate_2_referent": {sub_proof_index: int},
    ///             }
    ///         }
    ///         "proof": {
    ///             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
    ///             "aggregated_proof": <aggregated_proof>
    ///         }
    ///         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
    ///     }
    /// schemas_json: all schema jsons participating in the proof
    ///     {
    ///         <schema1_id>: <schema1_json>,
    ///         <schema2_id>: <schema2_json>,
    ///         <schema3_id>: <schema3_json>,
    ///     }
    /// credential_defs_json: all credential definitions json participating in the proof
    ///     {
    ///         "cred_def1_id": <credential_def1_json>,
    ///         "cred_def2_id": <credential_def2_json>,
    ///         "cred_def3_id": <credential_def3_json>,
    ///     }
    /// rev_reg_defs_json: all revocation registry definitions json participating in the proof
    ///     {
    ///         "rev_reg_def1_id": <rev_reg_def1_json>,
    ///         "rev_reg_def2_id": <rev_reg_def2_json>,
    ///         "rev_reg_def3_id": <rev_reg_def3_json>,
    ///     }
    /// rev_regs_json: all revocation registries json participating in the proof
    ///     {
    ///         "rev_reg_def1_id": {
    ///             "timestamp1": <rev_reg1>,
    ///             "timestamp2": <rev_reg2>,
    ///         },
    ///         "rev_reg_def2_id": {
    ///             "timestamp3": <rev_reg3>
    ///         },
    ///         "rev_reg_def3_id": {
    ///             "timestamp4": <rev_reg4>
    ///         },
    ///     }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// valid: true - if signature is valid, false - otherwise
    ///
    /// #Errors
    /// Annoncreds*
    /// Common*
    /// Wallet*
    extern indy_error_t indy_verifier_verify_proof(indy_handle_t command_handle,
                                                   const char *  proof_request_json,
                                                   const char *  proof_json,
                                                   const char *  schemas_json,
                                                   const char *  credential_defs_jsons,
                                                   const char *  rev_reg_defs_json,
                                                   const char *  rev_regs_json,
                                                   indy_bool_cb cb
                                                   );


    /// Create revocation state for a credential in the particular time moment.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context
    /// blob_storage_reader_handle: configuration of blob storage reader handle that will allow to read revocation tails
    /// rev_reg_def_json: revocation registry definition json
    /// rev_reg_delta_json: revocation registry definition delta json
    /// timestamp: time represented as a total number of seconds from Unix Epoch
    /// cred_rev_id: user credential revocation id in revocation registry
    /// cb: Callback that takes command result as parameter
    ///
    /// #Returns
    /// revocation state json:
    ///     {
    ///         "rev_reg": <revocation registry>,
    ///         "witness": <witness>,
    ///         "timestamp" : integer
    ///     }
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Anoncreds*
    extern indy_error_t indy_create_revocation_state(indy_handle_t command_handle,
                                                     indy_i32_t    blob_storage_reader_handle,
                                                     const char *  rev_reg_def_json,
                                                     const char *  rev_reg_delta_json,
                                                     indy_u64_t    timestamp,
                                                     const char *  cred_rev_id,
                                                     indy_str_cb cb
                                                     );

    /// Create new revocation state for a credential based on existed state
    /// at the particular time moment (to reduce calculation time).
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context
    /// blob_storage_reader_handle: configuration of blob storage reader handle that will allow to read revocation tails
    /// rev_state_json: revocation registry state json
    /// rev_reg_def_json: revocation registry definition json
    /// rev_reg_delta_json: revocation registry definition delta json
    /// timestamp: time represented as a total number of seconds from Unix Epoch
    /// cred_rev_id: user credential revocation id in revocation registry
    /// cb: Callback that takes command result as parameter
    ///
    /// #Returns
    /// revocation state json:
    ///     {
    ///         "rev_reg": <revocation registry>,
    ///         "witness": <witness>,
    ///         "timestamp" : integer
    ///     }
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Anoncreds*
    extern indy_error_t indy_update_revocation_state(indy_handle_t command_handle,
                                                     indy_i32_t    blob_storage_reader_handle,
                                                     const char *  rev_state_json,
                                                     const char *  rev_reg_def_json,
                                                     const char *  rev_reg_delta_json,
                                                     indy_u64_t    timestamp,
                                                     const char *  cred_rev_id,
                                                     indy_str_cb cb
                                                     );

#ifdef __cplusplus
}
#endif

#endif
