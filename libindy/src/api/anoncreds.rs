extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::anoncreds::AnoncredsCommand;
use commands::anoncreds::issuer::IssuerCommand;
use commands::anoncreds::prover::ProverCommand;
use commands::anoncreds::verifier::VerifierCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;
use std::ptr;

/// Create credential schema entity that describes credential attributes list and allows credentials
/// interoperability.
///
/// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
/// to Indy distributed ledger.
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
#[no_mangle]
pub extern fn indy_issuer_create_schema(command_handle: i32,
                                        issuer_did: *const c_char,
                                        name: *const c_char,
                                        version: *const c_char,
                                        attrs: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             id: *const c_char, schema_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(version, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(attrs, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateSchema(
                    issuer_did,
                    name,
                    version,
                    attrs,
                    Box::new(move |result| {
                        let (err, id, schema_json) = result_to_err_code_2!(result, String::new(), String::new());
                        let id = CStringUtils::string_to_cstring(id);
                        let schema_json = CStringUtils::string_to_cstring(schema_json);
                        cb(command_handle, err, id.as_ptr(), schema_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}

/// Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
/// Store the keys together with signature type and schema in a secure wallet as a credential definition.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing credential_def transaction to the Ledger
/// schema_json: schema as a json
/// tag:
/// type_: (optional) signature type. Currently only 'CL' is supported.
/// config_json: config json.
///     {
///         "support_revocation": boolean
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// credential definition json containing information about signature type, schema and issuer's public key.
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_and_store_credential_def(command_handle: i32,
                                                          wallet_handle: i32,
                                                          issuer_did: *const c_char,
                                                          schema_json: *const c_char,
                                                          tag: *const c_char,
                                                          type_: *const c_char,
                                                          config_json: *const c_char,
                                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                               id: *const c_char,
                                                                               credential_def_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schema_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(type_, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateAndStoreCredentialDefinition(
                    wallet_handle,
                    issuer_did,
                    schema_json,
                    tag,
                    type_,
                    config_json,
                    Box::new(move |result| {
                        let (err, id, credential_def_json) = result_to_err_code_2!(result, String::new(), String::new());
                        let id = CStringUtils::string_to_cstring(id);
                        let credential_def_json = CStringUtils::string_to_cstring(credential_def_json);
                        cb(command_handle, err, id.as_ptr(), credential_def_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}

/// Create a new revocation registry for the given credential definition.
/// Stores it in a secure wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing transaction to the Ledger
/// type_: (optional) registry type. Currently only 'CL_ACCUM' is supported.
/// tag:
/// cred_def_id: id of stored in ledger credential definition
/// config_json: {
///     "issuance_type": (optional) type of issuance. Currently supported:
///         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
///                                 Revocation Registry is updated only during revocation.
///         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
///     "max_cred_num": maximum number of credentials the new registry can process.
/// }
/// tails_writer_type:
/// tails_writer_config:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry definition json and revocation registry entry json
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_and_store_revoc_reg(command_handle: i32,
                                                     wallet_handle: i32,
                                                     issuer_did: *const c_char,
                                                     type_: *const c_char,
                                                     tag: *const c_char,
                                                     cred_def_id: *const c_char,
                                                     config_json: *const c_char,
                                                     tails_writer_type: *const c_char,
                                                     tails_writer_config: *const c_char,
                                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                          id: *const c_char,
                                                                          revoc_reg_def_json: *const c_char,
                                                                          revoc_reg_entry_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(type_, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_def_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam7);
    check_useful_opt_c_str!(tails_writer_type, ErrorCode::CommonInvalidParam8);
    check_useful_c_str!(tails_writer_config, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam10);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateAndStoreRevocationRegistry(
                    wallet_handle,
                    issuer_did,
                    type_,
                    tag,
                    cred_def_id,
                    config_json,
                    tails_writer_type,
                    tails_writer_config,
                    Box::new(move |result| {
                        let (err, id, revoc_reg_def_json, revoc_reg_json) = result_to_err_code_3!(result, String::new(), String::new(), String::new());
                        let id = CStringUtils::string_to_cstring(id);
                        let revoc_reg_def_json = CStringUtils::string_to_cstring(revoc_reg_def_json);
                        let revoc_reg_json = CStringUtils::string_to_cstring(revoc_reg_json);
                        cb(command_handle, err, id.as_ptr(), revoc_reg_def_json.as_ptr(), revoc_reg_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}

/// Create credential offer in Wallet
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// cred_def_id: id of stored in ledger credential definition
/// issuer_did: a DID of the issuer of credential
/// prover_did: a DID of the target user
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// credential offer json:
///        {
///            "cred_def_id": string,
///            "issuer_did" : string,
///            "nonce": string,
///            "key_correctness_proof" : <key_correctness_proof>
///        }
///
/// #Errors
/// Common*
/// Wallet*
/// Anoncreds*
#[no_mangle]
pub extern fn indy_issuer_create_credential_offer(command_handle: i32,
                                                  wallet_handle: i32,
                                                  cred_def_id: *const c_char,
                                                  issuer_did: *const c_char,
                                                  prover_did: *const c_char,
                                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                       credential_offer_json: *const c_char
                                                  )>) -> ErrorCode {
    check_useful_c_str!(cred_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(prover_did, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Issuer(IssuerCommand::CreateCredentialOffer(
            wallet_handle,
            cred_def_id,
            issuer_did,
            prover_did,
            Box::new(move |result| {
                let (err, credential_offer_json) = result_to_err_code_1!(result, String::new());
                let credential_offer_json = CStringUtils::string_to_cstring(credential_offer_json);
                cb(command_handle, err, credential_offer_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Signs a given credential values for the given user by a given key (credential def).
/// The corresponding credential definition and revocation registry must be already created
/// an stored into the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// credential_req_json: a credential request with a blinded secret from the user (returned by prover_create_and_store_credential_req).
///     Example:
///     {
///      "blinded_ms" : <blinded_master_secret>,
///      "cred_def_id" : string,
///      "issuer_did" : string,
///      "prover_did" : string,
///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
///      "nonce": string
///    }
/// credential_values_json: a credential containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
///     }
/// rev_reg_id: (Optional) id of stored in ledger revocation registry definition
/// tails_reader_handle:
/// user_revoc_index: index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry update json with a newly issued credential
/// Credential json containing signed credential values, issuer_did, schema_key, and revoc_reg_seq_no
/// used for issuance
///     {
///         "values": <see credential_values_json above>,
///         "signature": <signature>,
///         "issuer_did": string,
///         "cred_def_id": string,
///         "rev_reg_id", Optional<string>,
///         "signature_correctness_proof": <signature_correctness_proof>
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_create_credential(command_handle: i32,
                                            wallet_handle: i32,
                                            credential_req_json: *const c_char,
                                            credential_values_json: *const c_char,
                                            rev_reg_id: *const c_char,
                                            tails_reader_handle: i32,
                                            user_revoc_index: i32,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 revoc_reg_delta_json: *const c_char,
                                                                 credential_json: *const c_char
                                            )>) -> ErrorCode {
    check_useful_c_str!(credential_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(credential_values_json, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let tails_reader_handle = if tails_reader_handle != -1 { Some(tails_reader_handle) } else { None };
    let user_revoc_index = if user_revoc_index != -1 { Some(user_revoc_index as u32) } else { None };

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Issuer(IssuerCommand::CreateCredential(
            wallet_handle,
            credential_req_json,
            credential_values_json,
            rev_reg_id,
            tails_reader_handle,
            user_revoc_index,
            Box::new(move |result| {
                let (err, revoc_reg_delta_json, credential_json) = result_to_err_code_2!(result, None, String::new());
                let revoc_reg_delta_json = revoc_reg_delta_json.map(CStringUtils::string_to_cstring);
                let credential_json = CStringUtils::string_to_cstring(credential_json);
                cb(command_handle, err, revoc_reg_delta_json.as_ref().map(|delta| delta.as_ptr()).unwrap_or(ptr::null()), credential_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Revokes a user identified by a user_revoc_index in a given revoc-registry.
/// The corresponding credential definition and revocation registry must be already
/// created an stored into the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// rev_reg_id: id of revocation registry stored in wallet
/// tails_reader_handle:
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry delta json with a revoked credential
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_revoke_credential(command_handle: i32,
                                            wallet_handle: i32,
                                            tails_reader_handle: i32,
                                            rev_reg_id: *const c_char,
                                            user_revoc_index: u32,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 revoc_reg_delta_json: *const c_char,
                                            )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::RevokeCredential(
                    wallet_handle,
                    tails_reader_handle,
                    rev_reg_id,
                    user_revoc_index,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}


/// Recover a user identified by a user_revoc_index in a given revoc-registry.
/// The corresponding credential definition and revocation registry must be already
/// created an stored into the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// rev_reg_id: id of revocation registry stored in wallet
/// tails_reader_handle:
/// user_revoc_index: index of the user in the revocation registry
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Revocation registry delta json with a revoked credential
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_recover_credential(command_handle: i32,
                                             wallet_handle: i32,
                                             tails_reader_handle: i32,
                                             rev_reg_id: *const c_char,
                                             user_revoc_index: u32,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  revoc_reg_delta_json: *const c_char,
                                             )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::RecoverCredential(
                    wallet_handle,
                    tails_reader_handle,
                    rev_reg_id,
                    user_revoc_index,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}

/// Stores a credential offer from the given issuer in a secure storage.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// credential_offer_json: credential offer as a json containing information about the issuer and a credential:
///        {
///            "cred_def_id": string,
///            "rev_reg_id" : Optional<string>,
///            "nonce": string,
///            "key_correctness_proof" : <key_correctness_proof>
///        }
/// #Returns
/// None.
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_store_credential_offer(command_handle: i32,
                                                 wallet_handle: i32,
                                                 credential_offer_json: *const c_char,
                                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode
                                                 )>) -> ErrorCode {
    check_useful_c_str!(credential_offer_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::StoreCredentialOffer(
            wallet_handle,
            credential_offer_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        ))));

    result_to_err_code!(result)
}

/// Gets all stored credential offers (see prover_store_credential_offer).
/// A filter can be specified to get credential offers for specific Issuer, credential_def or schema only.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// filter_json: optional filter to get credential offers for specific Issuer, credential_def or schema only only
///     Each of the filters is optional and can be combines
///        {
///            "schema_id": string, (Optional)
///            "schema_did": string, (Optional)
///            "schema_name": string, (Optional)
///            "schema_version": string, (Optional)
///            "issuer_did": string, (Optional)
///            "issuer_did": string, (Optional)
///            "cred_def_id": string, (Optional)
///        }
///
/// #Returns
/// A json with a list of credential offers for the filter.
///        {
///            [{
///                 "cred_def_id": string,
///                 "issuer_did": string,
///                 "nonce": string,
///                 "key_correctness_proof" : <key_correctness_proof>
///            }]
///        }
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_get_credential_offers(command_handle: i32,
                                                wallet_handle: i32,
                                                filter_json: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                     credential_offers_json: *const c_char
                                                )>) -> ErrorCode {
    check_useful_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::GetCredentialOffers(
            wallet_handle,
            filter_json,
            Box::new(move |result| {
                let (err, credential_offers_json) = result_to_err_code_1!(result, String::new());
                let credential_offers_json = CStringUtils::string_to_cstring(credential_offers_json);
                cb(command_handle, err, credential_offers_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}


/// Creates a master secret with a given name and stores it in the wallet.
/// The name must be unique.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// master_secret_name: a new master secret name
///
/// #Returns
/// None.
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_master_secret(command_handle: i32,
                                               wallet_handle: i32,
                                               master_secret_name: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode
                                               )>) -> ErrorCode {
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateMasterSecret(
            wallet_handle,
            master_secret_name,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        ))));

    result_to_err_code!(result)
}


/// Creates a clam request json for the given credential offer and stores it in a secure wallet.
/// The credential offer contains the information about Issuer (DID, schema_seq_no),
/// and the schema (schema_key).
/// The method creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
/// The blinded master secret is a part of the credential request.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// prover_did: a DID of the prover
/// credential_offer_json: credential offer as a json containing information about the issuer and a credential:
///        {
///            "cred_def_id": string,
///            "rev_reg_id" : Optional<string>,
///            "nonce": string,
///            "key_correctness_proof" : <key_correctness_proof>
///        }
/// credential_def_json: credential definition json associated with issuer_did and schema_seq_no in the credential_offer
/// master_secret_name: the name of the master secret stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Credential request json.
///     {
///      "blinded_ms" : <blinded_master_secret>,
///      "cred_def_id" : string,
///      "issuer_did" : string,
///      "prover_did" : string,
///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
///      "nonce": string
///    }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_and_store_credential_req(command_handle: i32,
                                                          wallet_handle: i32,
                                                          prover_did: *const c_char,
                                                          credential_offer_json: *const c_char,
                                                          credential_def_json: *const c_char,
                                                          master_secret_name: *const c_char,
                                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                               credential_req_json: *const c_char
                                                          )>) -> ErrorCode {
    check_useful_c_str!(prover_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(credential_offer_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(credential_def_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateAndStoreCredentialRequest(
            wallet_handle,
            prover_did,
            credential_offer_json,
            credential_def_json,
            master_secret_name,
            Box::new(move |result| {
                let (err, credential_req_json) = result_to_err_code_1!(result, String::new());
                let credential_req_json = CStringUtils::string_to_cstring(credential_req_json);
                cb(command_handle, err, credential_req_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Updates the credential by a master secret and stores in a secure wallet.
/// The credential contains the information about
/// schema_key, issuer_did, revoc_reg_seq_no (see issuer_create_credential).
/// Seq_no is a sequence number of the corresponding transaction in the ledger.
/// The method loads a blinded secret for this key from the wallet,
/// updates the credential and stores it in a wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// id: identifier by which credential will be stored in wallet
/// credentials_json: credential json:
///     {
///         "values": <see credential_values_json above>,
///         "signature": <signature>,
///         "cred_def_id": string,
///         "rev_reg_id", Optional<string>,
///         "signature_correctness_proof": <signature_correctness_proof>
///     }
/// rev_reg_def_json: revocation registry definition json
/// rev_reg_json: revocation registry json
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_store_credential(command_handle: i32,
                                           wallet_handle: i32,
                                           id: *const c_char,
                                           credentials_json: *const c_char,
                                           rev_reg_def_json: *const c_char,
                                           cb: Option<extern fn(
                                               xcommand_handle: i32, err: ErrorCode
                                           )>) -> ErrorCode {
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(credentials_json, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::StoreCredential(
            wallet_handle,
            id,
            credentials_json,
            rev_reg_def_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        ))));

    result_to_err_code!(result)
}


/// Gets human readable credentials according to the filter.
/// If filter is NULL, then all credentials are returned.
/// Credentials can be filtered by Issuer, credential_def and/or Schema.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// filter_json: filter for credentials
///        {
///            "schema_id": string, (Optional)
///            "schema_did": string, (Optional)
///            "schema_name": string, (Optional)
///            "schema_version": string, (Optional)
///            "issuer_did": string, (Optional)
///            "issuer_did": string, (Optional)
///            "cred_def_id": string, (Optional)
///        }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// credentials json
///     [{
///         "referent": string,
///         "values": <see credential_values_json above>,
///         "issuer_did": string,
///         "cred_def_id": string,
///         "rev_reg_id", Optional<string>
///     }]
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_get_credentials(command_handle: i32,
                                          wallet_handle: i32,
                                          filter_json: *const c_char,
                                          cb: Option<extern fn(
                                              xcommand_handle: i32, err: ErrorCode,
                                              credentials_json: *const c_char
                                          )>) -> ErrorCode {
    check_useful_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::GetCredentials(
            wallet_handle,
            filter_json,
            Box::new(move |result| {
                let (err, credentials_json) = result_to_err_code_1!(result, String::new());
                let credentials_json = CStringUtils::string_to_cstring(credentials_json);
                cb(command_handle, err, credentials_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Gets human readable credentials matching the given proof request.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// proof_request_json: proof request json
///     {
///         "name": string,
///         "version": string,
///         "nonce": string,
///         "requested_attrs": {
///             "requested_attr1_referent": <attr_info>,
///             "requested_attr2_referent": <attr_info>,
///             "requested_attr3_referent": <attr_info>,
///         },
///         "requested_predicates": {
///             "requested_predicate_1_referent": <predicate_info>,
///             "requested_predicate_2_referent": <predicate_info>,
///         },
///         "freshness": Optional<number>
///     }
/// cb: Callback that takes command result as parameter.
///
/// where attr_info:
///     {
///         "name": attribute name, (case insensitive and ignore spaces)
///         "freshness": (Optional)
///         "restrictions": [
///             <see filter json above>
///         ]  (Optional) - if specified, credential must satisfy to one of the given restriction.
///     }
/// predicate_info:
///     {
///         "attr_name": attribute name, (case insensitive and ignore spaces)
///         "p_type": predicate type (Currently >= only)
///         "value": requested value of attribute
///         "freshness": (Optional)
///         "restrictions": [
///             <see filter json above>
///         ]  (Optional) - if specified, credential must satisfy to one of the given restriction.
///     }
/// #Returns
/// json with credentials for the given pool request.
///     {
///         "attrs": {
///             "requested_attr1_referent": [(credential1, Optional<freshness>), (credential2, Optional<freshness>)],
///             "requested_attr2_referent": [],
///             "requested_attr3_referent": [(credential3, Optional<freshness>)]
///         },
///         "predicates": {
///             "requested_predicate_1_referent": [(credential1, Optional<freshness>), (credential3, Optional<freshness>)],
///             "requested_predicate_2_referent": [(credential2, Optional<freshness>)]
///         }
///     }, where credential is
///     {
///         "referent": <string>,
///         "attrs": [{"attr_name" : "attr_raw_value"}],
///         "issuer_did": string,
///         "cred_def_id": string,
///         "rev_reg_id": Optional<int>
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_get_credentials_for_proof_req(command_handle: i32,
                                                        wallet_handle: i32,
                                                        proof_request_json: *const c_char,
                                                        cb: Option<extern fn(
                                                            xcommand_handle: i32, err: ErrorCode,
                                                            credentials_json: *const c_char
                                                        )>) -> ErrorCode {
    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::GetCredentialsForProofReq(
            wallet_handle,
            proof_request_json,
            Box::new(move |result| {
                let (err, credentials_json) = result_to_err_code_1!(result, String::new());
                let credentials_json = CStringUtils::string_to_cstring(credentials_json);
                cb(command_handle, err, credentials_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

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
/// proof_req_json: proof request json as come from the verifier
///     {
///         "name": string,
///         "version": string,
///         "nonce": string,
///         "requested_attrs": {
///             "requested_attr1_referent": <attr_info>,
///             "requested_attr2_referent": <attr_info>,
///             "requested_attr3_referent": <attr_info>,
///         },
///         "requested_predicates": {
///             "requested_predicate_1_referent": <predicate_info>,
///             "requested_predicate_2_referent": <predicate_info>,
///         },
///         "freshness": Optional<number>
///     }
/// requested_credentials_json: either a credential or self-attested attribute for each requested attribute
///     {
///         "requested_attr1_referent": [{"cred_id": string, "freshness": Optional<number>}, true <reveal_attr>],
///         "requested_attr2_referent": [self_attested_attribute],
///         "requested_attr3_referent": [{"cred_id": string, "freshness": Optional<number>}, false]
///         "requested_attr4_referent": [{"cred_id": string, "freshness": Optional<number>}, true]
///         "requested_predicate_1_referent": [{"cred_id": string, "freshness": Optional<number>}],
///         "requested_predicate_2_referent": [{"cred_id": string, "freshness": Optional<number>}],
///     }
/// schemas_jsons: all schema jsons participating in the proof request
///     {
///         "credential1_referent_in_wallet": <schema1>,
///         "credential2_referent_in_wallet": <schema2>,
///         "credential3_referent_in_wallet": <schema3>,
///     }
/// master_secret_name: the name of the master secret stored in the wallet
/// credential_def_jsons: all credential definition jsons participating in the proof request
///     {
///         "credential1_referent_in_wallet": <credential_def1>,
///         "credential2_referent_in_wallet": <credential_def2>,
///         "credential3_referent_in_wallet": <credential_def3>,
///     }
/// revoc_infos_jsons: all revocation registry jsons participating in the proof request
///     {
///         "credential1_referent_in_wallet": {
///             "freshness1": <revoc_info1>,
///             "freshness2": <revoc_info2>,
///         },
///         "credential2_referent_in_wallet": {
///             "freshness3": <revoc_info3>
///         },
///         "credential3_referent_in_wallet": {
///             "freshness4": <revoc_info4>
///         },
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Proof json
/// For each requested attribute either a proof (with optionally revealed attribute value) or
/// self-attested attribute value is provided.
/// Each proof is associated with a credential and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
/// There ais also aggregated proof part common for all credential proofs.
///     {
///         "requested": {
///             "revealed_attrs": {
///                 "requested_attr1_id": {referent: string, raw: string, encoded: string},
///                 "requested_attr4_id": {referent: string, raw: string, encoded: string},
///             },
///             "unrevealed_attrs": {
///                 "requested_attr3_id": referent
///             },
///             "self_attested_attrs": {
///                 "requested_attr2_id": self_attested_value,
///             },
///             "requested_predicates": {
///                 "requested_predicate_1_referent": [credential_proof2_referent],
///                 "requested_predicate_2_referent": [credential_proof3_referent],
///             }
///         }
///         "proof": {
///             "proofs": {
///                 "credential_proof1_referent": <credential_proof>,
///                 "credential_proof2_referent": <credential_proof>,
///                 "credential_proof3_referent": <credential_proof>
///             },
///             "aggregated_proof": <aggregated_proof>
///         }
///         "identifiers": {"credential_proof1_referent":{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}}
///     }
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_proof(command_handle: i32,
                                       wallet_handle: i32,
                                       proof_req_json: *const c_char,
                                       requested_credentials_json: *const c_char,
                                       schemas_json: *const c_char,
                                       master_secret_name: *const c_char,
                                       credential_defs_json: *const c_char,
                                       rev_infos_json: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                            proof_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(proof_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(requested_credentials_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_name, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(credential_defs_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_str!(rev_infos_json, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateProof(
            wallet_handle,
            proof_req_json,
            requested_credentials_json,
            schemas_json,
            master_secret_name,
            credential_defs_json,
            rev_infos_json,
            Box::new(move |result| {
                let (err, proof_json) = result_to_err_code_1!(result, String::new());
                let proof_json = CStringUtils::string_to_cstring(proof_json);
                cb(command_handle, err, proof_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Verifies a proof (of multiple credential).
/// All required schemas, public keys and revocation registries must be provided.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// proof_request_json: initial proof request as sent by the verifier
///     {
///         "name": string,
///         "version": string,
///         "nonce": string,
///         "requested_attrs": {
///             "requested_attr1_referent": <attr_info>,
///             "requested_attr2_referent": <attr_info>,
///             "requested_attr3_referent": <attr_info>,
///         },
///         "requested_predicates": {
///             "requested_predicate_1_referent": <predicate_info>,
///             "requested_predicate_2_referent": <predicate_info>,
///         },
///         "freshness": Optional<number>
///     }
/// proof_json: proof json
/// For each requested attribute either a proof (with optionally revealed attribute value) or
/// self-attested attribute value is provided.
/// Each proof is associated with a credential and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
/// There ais also aggregated proof part common for all credential proofs.
///     {
///         "requested": {
///             "requested_attr1_id": [credential_proof1_referent, revealed_attr1, revealed_attr1_as_int],
///             "requested_attr2_id": [self_attested_attribute],
///             "requested_attr3_id": [credential_proof2_referent]
///             "requested_attr4_id": [credential_proof2_referent, revealed_attr4, revealed_attr4_as_int],
///             "requested_predicate_1_referent": [credential_proof2_referent],
///             "requested_predicate_2_referent": [credential_proof3_referent],
///         }
///         "proof": {
///             "proofs": {
///                 "credential_proof1_referent": <credential_proof>,
///                 "credential_proof2_referent": <credential_proof>,
///                 "credential_proof3_referent": <credential_proof>
///             },
///             "aggregated_proof": <aggregated_proof>
///         }
///         "identifiers": {"credential_proof1_referent":{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}}
///     }
/// schemas_jsons: all schemas json participating in the proof
///         {
///             "credential_proof1_referent": <schema>,
///             "credential_proof2_referent": <schema>,
///             "credential_proof3_referent": <schema>
///         }
/// credential_defs_jsons: all credential definitions json participating in the proof
///         {
///             "credential_proof1_referent": <credential_def>,
///             "credential_proof2_referent": <credential_def>,
///             "credential_proof3_referent": <credential_def>
///         }
/// rev_reg_defs_json: all revocation registry definitions json participating in the proof
///         {
///             "credential_proof1_referent": <rev_reg_def>,
///             "credential_proof2_referent": <rev_reg_def>,
///             "credential_proof3_referent": <rev_reg_def>
///         }
/// rev_regs_json: all revocation registry definitions json participating in the proof
///     {
///         "credential1_referent_in_wallet": {
///             "freshness1": <revoc_reg1>,
///             "freshness2": <revoc_reg2>,
///         },
///         "credential2_referent_in_wallet": {
///             "freshness3": <revoc_reg3>
///         },
///         "credential3_referent_in_wallet": {
///             "freshness4": <revoc_reg4>
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
#[no_mangle]
pub extern fn indy_verifier_verify_proof(command_handle: i32,
                                         proof_request_json: *const c_char,
                                         proof_json: *const c_char,
                                         schemas_json: *const c_char,
                                         credential_defs_jsons: *const c_char,
                                         rev_reg_defs_json: *const c_char,
                                         rev_regs_json: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              valid: bool)>) -> ErrorCode {
    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(proof_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(credential_defs_jsons, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(rev_reg_defs_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(rev_regs_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Verifier(VerifierCommand::VerifyProof(
            proof_request_json,
            proof_json,
            schemas_json,
            credential_defs_jsons,
            rev_reg_defs_json,
            rev_regs_json,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                cb(command_handle, err, valid)
            })
        ))));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_create_revocation_info(command_handle: i32,
                                          tails_reader_handle: i32,
                                          rev_reg_def_json: *const c_char,
                                          rev_reg_delta_json: *const c_char,
                                          timestamp: u64,
                                          rev_idx: u32,
                                          cb: Option<extern fn(
                                              xcommand_handle: i32, err: ErrorCode,
                                              rev_info_json: *const c_char
                                          )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateRevocationInfo(
            tails_reader_handle,
            rev_reg_def_json,
            rev_reg_delta_json,
            timestamp,
            rev_idx,
            Box::new(move |result| {
                let (err, rev_info_json) = result_to_err_code_1!(result, String::new());
                let rev_info_json = CStringUtils::string_to_cstring(rev_info_json);
                cb(command_handle, err, rev_info_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_update_revocation_info(command_handle: i32,
                                          tails_reader_handle: i32,
                                          rev_info_json: *const c_char,
                                          rev_reg_def_json: *const c_char,
                                          rev_reg_delta_json: *const c_char,
                                          timestamp: u64,
                                          rev_idx: u32,
                                          cb: Option<extern fn(
                                              xcommand_handle: i32, err: ErrorCode,
                                              updated_rev_info_json: *const c_char
                                          )>) -> ErrorCode {
    check_useful_c_str!(rev_info_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::UpdateRevocationInfo(
            tails_reader_handle,
            rev_info_json,
            rev_reg_def_json,
            rev_reg_delta_json,
            timestamp,
            rev_idx,
            Box::new(move |result| {
                let (err, updated_rev_info_json) = result_to_err_code_1!(result, String::new());
                let updated_rev_info_json = CStringUtils::string_to_cstring(updated_rev_info_json);
                cb(command_handle, err, updated_rev_info_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_store_revocation_info(command_handle: i32,
                                         wallet_handle: i32,
                                         id: *const c_char,
                                         rev_info_json: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_info_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::StoreRevocationInfo(
            wallet_handle,
            id,
            rev_info_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        ))));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_get_revocation_info(command_handle: i32,
                                       wallet_handle: i32,
                                       id: *const c_char,
                                       timestamp: i64,
                                       cb: Option<extern fn(
                                           xcommand_handle: i32, err: ErrorCode,
                                           rev_info_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let timestamp = if timestamp != -1 { Some(timestamp as u64) } else { None };

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::GetRevocationInfo(
            wallet_handle,
            id,
            timestamp,
            Box::new(move |result| {
                let (err, rev_info_json) = result_to_err_code_1!(result, String::new());
                let rev_info_json = CStringUtils::string_to_cstring(rev_info_json);
                cb(command_handle, err, rev_info_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}
