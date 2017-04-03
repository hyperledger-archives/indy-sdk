extern crate libc;

use self::libc::{c_char, c_uchar};

/// Create keys (both primary and revocation) for the given schema and stores the keys
/// in a secure wallet.
/// A signing type can be set (currently only CL signature type is supported).
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// schema_json: schema as a json
/// sign_type: signature type (optional). Currently only CL is supported.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Created public keys as a json.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_issuer_create_and_store_keys(client_handle: i32, command_handle: i32,
                                                     schema_json: *const c_char,
                                                     sign_type: *const c_char,
                                                     cb: extern fn(xcommand_handle: i32, err: i32,
                                                                   public_key_json: *const c_char
                                                     )) -> i32 {
    unimplemented!();
}

/// Create a new revocation registry for the given public key.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// public_key_id: ID of a public key
/// max_claim_num: maximum number of claims the new registry can process.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Created revocation registry as a json.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_issuer_create_and_store_revoc_reg(client_handle: i32, command_handle: i32,
                                                  public_key_id: *const c_char,
                                                  max_claim_num: i32,
                                                  cb: extern fn(xcommand_handle: i32, err: i32,
                                                                revoc_reg_json: *const c_char
                                                  )) -> i32 {
    unimplemented!();
}

/// Signs a given claim for the given user by a given key (that is create a credential).
/// The corresponding keys and revocation registry must be already created
/// an stored into the wallet.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// claim_req_json: a claim request with a blinded secret
///     from the user (returned by anoncreds_prover_create_and_store_claim_req)
/// claim_json: a claim containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : "value1",
///      "attr2" : "value2"
///     }
/// public_key_id: ID of a public key
/// revoc_reg_id: ID of a revocation registry to use
/// user_revoc_id: ID of a new user in the revocation
///     registry (optional; default one is used if not provided)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Credential json containing issued credential, and public_key_id and revoc_reg_id
/// used for issuance
///     {
///         "cred": string,
///         "schema_id": string,
///         "public_key_id", string,
///         "revoc_reg_id", string
///     }
/// Updated revoc_reg as json
///
/// #Errors
/// RevocRegistryFull
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_issuer_create_credential(client_handle: i32, command_handle: i32,
                                                 claim_req_json: *const c_char,
                                                 claim_json: *const c_char,
                                                 public_key_id: *const c_char,
                                                 revoc_reg_id: *const c_char,
                                                 user_revoc_id: *const c_char,
                                                 cb: extern fn(xcommand_handle: i32, err: i32,
                                                               credential_json: *const c_char,
                                                               revoc_reg_update_json: *const c_char
                                                 )) -> i32 {
    unimplemented!();
}

/// Revokes a user identified by a revoc_id in a given revoc-registry.
/// The corresponding keys and revocation registry must be already
/// created an stored into the wallet.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// revoc_reg_id: ID of a revocation registry to use
/// user_revoc_id: ID of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Updated revoc_reg as json
///
/// #Errors
/// NotIssued
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_issuer_revoke_claim(client_handle: i32, command_handle: i32,
                                             revoc_reg_id: *const c_char,
                                             user_revoc_id: *const c_char,
                                             cb: extern fn(xcommand_handle: i32, err: i32,
                                                       revoc_reg_update_json: *const c_char
                                             )) -> i32 {
    unimplemented!();
}

/// Creates a clam request json for the given claim offer and stores it in a secure wallet.
/// The claim offer contains the information about Issuer (DID, public_key_id),
/// and the schema (schema_id).
/// The method gets public key and schema from the ledger, stores them in a wallet,
/// and creates a blinded master secret which is a part of the claim request.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// claim_offer_json: claim offer as a json containing information about the issuer and a claim:
///        {
///            "issuer_did": string,
///            "schema_id": string,
///            "public_key_id": string
///        }
/// master_secret: (optional) If not provided, then a new one will be generated.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Claim request json.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_prover_create_and_store_claim_req(client_handle: i32, command_handle: i32,
                                             claim_offer_json: *const c_char,
                                             master_secret: *const c_char,
                                             cb: extern fn(xcommand_handle: i32, err: i32,
                                                           claim_req_json: *const c_char
                                             )) -> i32 {
    unimplemented!();
}

/// Updates the credential by a master secret and store in a secure wallet.
/// The credential contains the information about schema_id,
/// public_key_id revoc_reg_id (see anoncreds_issuer_sign_claim).
/// The method loads a blinded secret for this key from the wallet,
/// updates the credential and stores it in a wallet.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// credentials_json: credentials json:
///     {
///         "cred": string,
///         "schema_id": string,
///         "public_key_id", string,
///         "revoc_reg_id", string
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_prover_store_credential(client_handle: i32, command_handle: i32,
                                                credentials_json: *const c_char,
                                                cb: extern fn(
                                                    xcommand_handle: i32, err: i32
                                                )) -> i32 {
    unimplemented!();
}

/// Parses the given proof_request json and creates a proof.
/// A proof request may request multiple claims from different schemas and different issuers.
/// The method gets from the Ledger all required data (public keys, etc.),
/// and prepares a proof (of multiple claim).
/// The proof request also contains nonce.
/// The proof contains proofs for each schema with corresponding public_key_id, revoc_reg_id
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// proof_request_json: proof request as a json
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Proof json
/// {
///    "proofs":[
///        {
///            "proof": string,
///            "schema_id": string,
///            "public_key_id": string,
///            "revoc_reg_id": string,
///            "revealed_attr_values": array,
///        }],
///    "aggregated_proof": object
/// }
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_prover_create_proof(client_handle: i32, command_handle: i32,
                                             proof_request_json: *const c_char,
                                             cb: extern fn(xcommand_handle: i32, err: i32,
                                                proof_json: *const c_char)) -> i32 {
    unimplemented!();
}

/// The method gets all required data (public keys, etc.) from the Ledger,
/// and verifies a proof (of multiple claim).
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// proof_request_initial_json: initial proof request as sent by the verifier
/// proof_request_disclosed_json: an updated by the prover proof_request if the prover doesn't want
/// to disclose all the information from the initial proof_request.
/// Disclosed pool request contains information participating in the proof only.
/// proof_json: proof as a json
/// {
///    "proofs":[
///        {
///            "proof": string,
///            "schema_id": string,
///            "public_key_id": string,
///            "revoc_reg_id": string,
///            "revealed_attr_values": array,
///        }],
///    "aggregated_proof": object
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// VerificationError.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_verifier_verify_proof(client_handle: i32, command_handle: i32,
                                              proof_request_initial_json: *const c_char,
                                              proof_request_disclosed_json: *const c_char,
                                              proof_json: *const c_char,
                                              cb: extern fn(xcommand_handle: i32, err: i32
                                              )) -> i32 {
    unimplemented!();
}

