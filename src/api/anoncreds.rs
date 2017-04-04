extern crate libc;

use api::ErrorCode;

use self::libc::{c_char, c_uchar};

/// Create keys (both primary and revocation) for the given schema and stores the keys
/// in a secure wallet.
/// Publishes the public key in the Ledger (so that a seq_no is associated with the key).
/// A signing type can be set (currently only CL signature type is supported).
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// issuer_did: a DID of the issuer signing public_key transaction to the Ledger
/// schema_json: schema as a json
/// signature_type: signature type (optional). Currently only 'CL' is supported.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Public key seq_no (sequence number of the Public Key transaction in Ledger).
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub extern fn anoncreds_issuer_create_and_store_keys(session_handle: i32, command_handle: i32,
                                                     issuer_did: *const c_char,
                                                     schema_json: *const c_char,
                                                     signature_type: *const c_char,
                                                     cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                   public_key_seq_no: i32
                                                     )) -> ErrorCode {
    unimplemented!();
}

/// Create a new revocation registry for the given public key. Stores it in a secure wallet.
/// Publishes the public key in the Ledger (so that a seq_no is associated with the registry).
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// issuer_did: a DID of the issuer signing revoc_reg transaction to the Ledger
/// public_key_seq_no: seq no of a public key transaction in Ledger
/// max_claim_num: maximum number of claims the new registry can process.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revoc registry seq_no (sequence number of the Revoc Reg transaction in Ledger).
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub extern fn anoncreds_issuer_create_and_store_revoc_reg(session_handle: i32, command_handle: i32,
                                                          issuer_did: *const c_char,
                                                          public_key_seq_no: i32,
                                                          max_claim_num: i32,
                                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                        revoc_reg_seq_no: *const c_char
                                                          )) -> ErrorCode {
    unimplemented!();
}

/// Signs a given claim for the given user by a given key (that is create a credential).
/// The corresponding keys and revocation registry must be already created
/// an stored into the wallet.
/// Updates the revocation registry in the Ledger for the newly issued claim
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// issuer_did: a DID of the issuer signing transactions to the Ledger
/// claim_req_json: a claim request with a blinded secret
///     from the user (returned by anoncreds_prover_create_and_store_claim_req)
/// claim_json: a claim containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : "value1",
///      "attr2" : "value2"
///     }
/// public_key_seq_no: seq no of a public key transaction in Ledger
/// revoc_reg_seq_no: seq no of a revocation registry transaction in Ledger
/// user_revoc_index: index of a new user in the revocation registry (optional; default one is used if not provided)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Credential json containing issued credential, and public_key_seq_no and revoc_reg_seq_no
/// used for issuance
///     {
///         "cred": string,
///         "schema_seq_no": string,
///         "public_key_seq_no", string,
///         "revoc_reg_seq_no", string
///     }
///
/// #Errors
/// RevocationRegistryFull
/// InvalidUserRevocIndex
/// PublicKeyNotFoundError
/// RevocRegNotFoundError
/// WalletError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn anoncreds_issuer_create_credential(session_handle: i32, command_handle: i32,
                                                 claim_req_json: *const c_char,
                                                 claim_json: *const c_char,
                                                 issuer_did: *const c_char,
                                                 public_key_seq_no: i32,
                                                 revoc_reg_seq_no: i32,
                                                 user_revoc_index: i32,
                                                 cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                               credential_json: *const c_char
                                                 )) -> ErrorCode {
    unimplemented!();
}

/// Revokes a user identified by a revoc_id in a given revoc-registry.
/// The corresponding keys and revocation registry must be already
/// created an stored into the wallet.
/// Updates the revocation registry in the Ledger for the revocated claim
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// issuer_did: a DID of the issuer signing transactions to the Ledger
/// revoc_reg_seq_no: seq no of a revocation registry transaction in Ledger
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// NotIssuedError
/// RevocRegNotFoundError
/// WalletError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn anoncreds_issuer_revoke_claim(session_handle: i32, command_handle: i32,
                                            issuer_did: *const c_char,
                                            revoc_reg_seq_no: *const c_char,
                                            user_revoc_index: *const c_char,
                                            cb: extern fn(xcommand_handle: i32, err: ErrorCode
                                            )) -> ErrorCode {
    unimplemented!();
}

/// Stores a claim offer from the given issuer in a secure storage.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// claim_offer_json: claim offer as a json containing information about the issuer and a claim:
///        {
///            "issuer_did": string,
///            "schema_seq_no": string,
///            "public_key_seq_no": string
///        }
///
/// #Returns
/// None.
///
/// #Errors
/// WalletError
/// IOError
#[no_mangle]
pub extern fn anoncreds_prover_store_claim_offer(session_handle: i32, command_handle: i32,
                                                 claim_offer_json: *const c_char,
                                                 cb: extern fn(xcommand_handle: i32, err: ErrorCode
                                                 )) -> ErrorCode {
    unimplemented!();
}

/// Gets all claim offers stored for the given issuer DID (see anoncreds_prover_store_claim_offer).
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// isseur_did: isser DID find claim offers for
///
/// #Returns
/// A json with a ist of claim offers for this issuer.
///
/// #Errors
/// WalletError
/// IOError
#[no_mangle]
pub extern fn anoncreds_prover_get_claim_offers(session_handle: i32, command_handle: i32,
                                                isseur_did: *const c_char,
                                                cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              claim_offers_json: *const c_char
                                                )) -> ErrorCode {
    unimplemented!();
}


/// Creates a master secret with a given name and stores it in the wallet.
/// The name must be unique.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// master_secret_name: a new master secret name
///
/// #Returns
/// None.
///
/// #Errors
/// DuplicateNameError
/// WalletError
/// IOError
#[no_mangle]
pub extern fn anoncreds_prover_create_master_secret(session_handle: i32, command_handle: i32,
                                                    master_secret_name: *const c_char,
                                                    cb: extern fn(xcommand_handle: i32, err: ErrorCode
                                                    )) -> ErrorCode {
    unimplemented!();
}

/// Creates a clam request json for the given claim offer and stores it in a secure wallet.
/// The claim offer contains the information about Issuer (DID, public_key_seq_no),
/// and the schema (schema_seq_no).
/// The method gets public key and schema from the ledger, stores them in a wallet,
/// and creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see anoncreds_prover_create_master_secret)
/// The blinded master secret is a part of the claim request.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// claim_offer_json: claim offer as a json containing information about the issuer and a claim:
///        {
///            "issuer_did": string,
///            "schema_seq_no": string,
///            "public_key_seq_no": string
///        }
/// master_secret_name: the name of the master secret stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Claim request json.
///
/// #Errors
/// WalletError
/// PublicKeyNotFoundError
/// SchemaNotFoundError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn anoncreds_prover_create_and_store_claim_req(session_handle: i32, command_handle: i32,
                                                          claim_offer_json: *const c_char,
                                                          master_secret_name: *const c_char,
                                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                        claim_req_json: *const c_char
                                                          )) -> ErrorCode {
    unimplemented!();
}

/// Updates the credential by a master secret and stores in a secure wallet.
/// The credential contains the information about schema_seq_no,
/// public_key_seq_no revoc_reg_seq_no (see anoncreds_issuer_create_credential).
/// Seq_no is a sequence number of the corresponding transaction in the ledger.
/// The method loads a blinded secret for this key from the wallet,
/// updates the credential and stores it in a wallet.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// credentials_json: credentials json:
///     {
///         "cred": string,
///         "schema_seq_no": string,
///         "public_key_seq_no", string,
///         "revoc_reg_seq_no", string
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// WalletError
/// IOError
#[no_mangle]
pub extern fn anoncreds_prover_store_credential(session_handle: i32, command_handle: i32,
                                                credentials_json: *const c_char,
                                                cb: extern fn(
                                                    xcommand_handle: i32, err: ErrorCode
                                                )) -> ErrorCode {
    unimplemented!();
}

/// Parses the given proof_request json and creates a proof.
/// A proof request may request multiple claims from different schemas and different issuers.
/// The method gets from the Ledger all required data (public keys, etc.),
/// and prepares a proof (of multiple claim).
/// The proof request also contains nonce.
/// The proof contains proofs for each schema with corresponding seq_no of public_key and revoc_reg transactions in Ledger.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// proof_request_json: proof request as a json
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Proof json
/// {
///    "proofs":[
///        {
///            "proof": string,
///            "schema_seq_no": string,
///            "public_key_seq_no": string,
///            "revoc_reg_seq_no": string,
///            "revealed_attr_values": array,
///        }],
///    "aggregated_proof": object
/// }
///
/// #Errors
/// ClaimNotFoundError
/// WalletError
/// PublicKeyNotFoundError
/// SchemaNotFoundError
/// RevocRegNotFoundError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn anoncreds_prover_create_proof(session_handle: i32, command_handle: i32,
                                            proof_request_json: *const c_char,
                                            cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                          proof_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// The method gets all required data (public keys, etc.) from the Ledger,
/// and verifies a proof (of multiple claim).
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// proof_request_initial_json: initial proof request as sent by the verifier
/// proof_request_disclosed_json: an updated by the prover proof_request if the prover doesn't want
/// to disclose all the information from the initial proof_request.
/// Disclosed pool request contains information participating in the proof only.
/// proof_json: proof as a json
/// {
///    "proofs":[
///        {
///            "proof": string,
///            "schema_seq_no": string,
///            "public_key_seq_no": string,
///            "revoc_reg_seq_no": string,
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
/// ProofRejected.
/// WalletError
/// PublicKeyNotFoundError
/// SchemaNotFoundError
/// RevocRegNotFoundError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn anoncreds_verifier_verify_proof(session_handle: i32, command_handle: i32,
                                              proof_request_initial_json: *const c_char,
                                              proof_request_disclosed_json: *const c_char,
                                              proof_json: *const c_char,
                                              cb: extern fn(xcommand_handle: i32, err: ErrorCode
                                              )) -> ErrorCode {
    unimplemented!();
}

