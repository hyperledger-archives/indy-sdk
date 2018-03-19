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

/// Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
/// and secrets used for credentials revocation.
///
/// Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
/// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
/// to Indy distributed ledger.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// issuer_did: a DID of the issuer signing cred_def transaction to the Ledger
/// schema_json: credential schema as a json
/// tag: allows to distinct between credential definitions for the same issuer and schema
/// type_: credential definition type (optional, 'CL' by default) that defines claims signature and revocation math. Supported types are:
/// - 'CL': Camenisch-Lysyanskaya credential signature type
/// config_json: type-specific configuration of credential definition as json:
/// - 'CL':
///   - revocationSupport: whether to request non-revocation credential (optional, default false)
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
#[no_mangle]
pub extern fn indy_issuer_create_and_store_credential_def(command_handle: i32,
                                                          wallet_handle: i32,
                                                          issuer_did: *const c_char,
                                                          schema_json: *const c_char,
                                                          tag: *const c_char,
                                                          type_: *const c_char,
                                                          config_json: *const c_char,
                                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                               cred_def_id: *const c_char,
                                                                               cred_def_json: *const c_char)>) -> ErrorCode {
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
                        let (err, cred_def_id, cred_def_json) = result_to_err_code_2!(result, String::new(), String::new());
                        let cred_def_id = CStringUtils::string_to_cstring(cred_def_id);
                        let cred_def_json = CStringUtils::string_to_cstring(cred_def_json);
                        cb(command_handle, err, cred_def_id.as_ptr(), cred_def_json.as_ptr())
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
/// cred_def_id: revoc_reg_def_id of stored in ledger credential definition
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
                                                                          revoc_reg_id: *const c_char,
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
                        let (err, revoc_reg_id, revoc_reg_def_json, revoc_reg_json) = result_to_err_code_3!(result, String::new(), String::new(), String::new());
                        let revoc_reg_id = CStringUtils::string_to_cstring(revoc_reg_id);
                        let revoc_reg_def_json = CStringUtils::string_to_cstring(revoc_reg_def_json);
                        let revoc_reg_json = CStringUtils::string_to_cstring(revoc_reg_json);
                        cb(command_handle, err, revoc_reg_id.as_ptr(), revoc_reg_def_json.as_ptr(), revoc_reg_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}

/// Create credential offer that will be used by Prover for
/// claim request creation. Offer includes nonce and key correctness proof
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
#[no_mangle]
pub extern fn indy_issuer_create_credential_offer(command_handle: i32,
                                                  wallet_handle: i32,
                                                  cred_def_id: *const c_char,
                                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                       cred_offer_json: *const c_char
                                                  )>) -> ErrorCode {
    check_useful_c_str!(cred_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Issuer(IssuerCommand::CreateCredentialOffer(
            wallet_handle,
            cred_def_id,
            Box::new(move |result| {
                let (err, cred_offer_json) = result_to_err_code_1!(result, String::new());
                let cred_offer_json = CStringUtils::string_to_cstring(cred_offer_json);
                cb(command_handle, err, cred_offer_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

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
/// cred_offer_json: a cred offer created by indy_issuer_create_cred_offer
/// cred_req_json: a credential request created by indy_prover_create_credential_request
/// cred_values_json: a credential containing attribute values for each of requested attribute names.
///     Example:
///     {
///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
///     }
/// rev_reg_id: id of revocation registry definition stored in the wallet
/// blob_storage_reader_handle: pre-configured blob storage reader instance handle that will allow to read revocation tails
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// cred_json: Credential json containing signed credential values
///     {
///         "cred_def_id": string,
///         "rev_reg_def_id", Optional<string>,
///         "values": <see credential_values_json above>,
///         // Fields below can depend on Cred Def type
///         "signature": <signature>,
///         "signature_correctness_proof": <signature_correctness_proof>,
///         "revoc_idx":                                                                TODO: FIXME: Think how to share it in a secure way
///     }
/// revoc_id: local id for revocation info (Can be used for revocation of this cred)
/// revoc_reg_delta_json: Revocation registry delta json with a newly issued credential
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_issuer_create_credential(command_handle: i32,
                                            wallet_handle: i32,
                                            cred_offer_json: *const c_char,
                                            cred_req_json: *const c_char,
                                            cred_values_json: *const c_char,
                                            rev_reg_id: *const c_char,
                                            blob_storage_reader_handle: i32,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 cred_json: *const c_char,
                                                                 revoc_id: *const c_char,
                                                                 revoc_reg_delta_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(cred_offer_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_req_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_values_json, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let blob_storage_reader_handle = if blob_storage_reader_handle != -1 { Some(blob_storage_reader_handle) } else { None };

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Issuer(IssuerCommand::CreateCredential(
            wallet_handle,
            cred_offer_json,
            cred_req_json,
            cred_values_json,
            rev_reg_id,
            blob_storage_reader_handle,
            Box::new(move |result| {
                let (err, cred_json, revoc_id, revoc_reg_delta_json) = result_to_err_code_3!(result, String::new(), None, None);
                let cred_json = CStringUtils::string_to_cstring(cred_json);
                let revoc_id = revoc_id.map(CStringUtils::string_to_cstring);
                let revoc_reg_delta_json = revoc_reg_delta_json.map(CStringUtils::string_to_cstring);
                cb(command_handle, err, cred_json.as_ptr(),
                   revoc_id.as_ref().map(|delta| delta.as_ptr()).unwrap_or(ptr::null()),
                   revoc_reg_delta_json.as_ref().map(|delta| delta.as_ptr()).unwrap_or(ptr::null()))
            })
        ))));

    result_to_err_code!(result)
}

/// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_cred).
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
/// blob_storage_reader_handle: pre-configured blob storage reader instance handle that will allow to read revocation tails
/// rev_reg_id: id of revocation registry stored in wallet
/// cred_revoc_id: local id for revocation info
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
                                            blob_storage_reader_handle: i32,
                                            rev_reg_id: *const c_char,
                                            cred_revoc_id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 revoc_reg_delta_json: *const c_char,
                                            )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_revoc_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::RevokeCredential(
                    wallet_handle,
                    blob_storage_reader_handle,
                    rev_reg_id,
                    cred_revoc_id,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}


/// Recover a credential identified by a cred_revoc_id (returned by indy_issuer_create_cred).
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
/// blob_storage_reader_handle: pre-configured blob storage reader instance handle that will allow to read revocation tails
/// rev_reg_id: id of revocation registry stored in wallet
/// cred_revoc_id: local id for revocation info
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
                                             blob_storage_reader_handle: i32,
                                             rev_reg_id: *const c_char,
                                             cred_revoc_id: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  revoc_reg_delta_json: *const c_char,
                                             )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_revoc_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::RecoverCredential(
                    wallet_handle,
                    blob_storage_reader_handle,
                    rev_reg_id,
                    cred_revoc_id,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
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
                                               master_secret_id: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode
                                               )>) -> ErrorCode {
    check_useful_c_str!(master_secret_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateMasterSecret(
            wallet_handle,
            master_secret_id,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        ))));

    result_to_err_code!(result)
}

/// Creates a clam request for the given credential offer.
///
/// The method creates a blinded master secret for a master secret identified by a provided name.
/// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
/// The blinded master secret is a part of the credential request.
///
/// #Params
/// command_handle: command handle to map callback to user context
/// wallet_handle: wallet handler (created by open_wallet)
/// prover_did: a DID of the prover
/// cred_offer_json: a cred offer created by indy_issuer_create_credential_offer
/// cred_def_json: credential definition json created by indy_issuer_create_and_store_credential_def
/// master_secret_id: the id of the master secret stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// cred_req_json: Credential request json for creation of credential by Issuer
///     {
///      "cred_def_id" : string,
///      "rev_reg_id" : Optional<string>,
///      "prover_did" : string,
///         // Fields below can depend on Cred Def type
///      "blinded_ms" : <blinded_master_secret>,
///      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
///      "nonce": string
///    }
/// cred_req_metadata_json: Credential request metadata json for processing of received from Issuer credential.

/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_create_credential_req(command_handle: i32,
                                                wallet_handle: i32,
                                                prover_did: *const c_char,
                                                cred_offer_json: *const c_char,
                                                cred_def_json: *const c_char,
                                                master_secret_id: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                     cred_req_json: *const c_char,
                                                                     cred_req_metadata_json: *const c_char
                                                )>) -> ErrorCode {
    check_useful_c_str!(prover_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_offer_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_def_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateAndStoreCredentialRequest(
            wallet_handle,
            prover_did,
            cred_offer_json,
            cred_def_json,
            master_secret_id,
            Box::new(move |result| {
                let (err, cred_req_json, cred_req_metadata_json) = result_to_err_code_2!(result, String::new(), String::new());
                let cred_req_json = CStringUtils::string_to_cstring(cred_req_json);
                let cred_req_metadata_json = CStringUtils::string_to_cstring(cred_req_metadata_json);
                cb(command_handle, err, cred_req_json.as_ptr(), cred_req_metadata_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

/// Check credential provided by Issuer for the given credential request,
/// updates the credential by a master secret and stores in a secure wallet.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// cred_id: (optional, default is a random one) identifier by which credential will be stored in the wallet
/// cred_req_json: a credential request created by indy_prover_create_cred_request
/// cred_req_metadata_json: a credential request metadata created by indy_prover_create_cred_request
/// cred_json: credential json created by indy_issuer_create_cred
/// cred_def_json: credential definition json created by indy_issuer_create_and_store_credential_def
/// rev_reg_def_json: revocation registry definition json created by indy_issuer_create_and_store_revoc_reg
/// rev_state_json: revocation state json
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// cred_id: identifier by which credential is stored in the wallet
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_prover_store_credential(command_handle: i32,
                                           wallet_handle: i32,
                                           cred_id: *const c_char,
                                           cred_req_json: *const c_char,
                                           cred_req_metadata_json: *const c_char,
                                           cred_json: *const c_char,
                                           cred_def_json: *const c_char,
                                           rev_reg_def_json: *const c_char,
                                           rev_state_json: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                out_cred_id: *const c_char)>) -> ErrorCode {
    check_useful_opt_c_str!(cred_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_req_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_req_metadata_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(cred_def_json, ErrorCode::CommonInvalidParam7);
    check_useful_opt_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam8);
    check_useful_opt_c_str!(rev_state_json, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam10);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::StoreCredential(
            wallet_handle,
            cred_id,
            cred_req_json,
            cred_req_metadata_json,
            cred_json,
            cred_def_json,
            rev_reg_def_json,
            rev_state_json,
            Box::new(move |result| {
                let (err, out_cred_id) = result_to_err_code_1!(result, String::new());
                let out_cred_id = CStringUtils::string_to_cstring(out_cred_id);
                cb(command_handle, err, out_cred_id.as_ptr())
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
    check_useful_opt_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
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
/// master_secret_id: the name of the master secret stored in the wallet
/// credential_def_jsons: all credential definition jsons participating in the proof request
///     {
///         "credential1_referent_in_wallet": <credential_def1>,
///         "credential2_referent_in_wallet": <credential_def2>,
///         "credential3_referent_in_wallet": <credential_def3>,
///     }
/// rev_states_json: all revocation registry jsons participating in the proof request
///     {
///         "credential1_referent_in_wallet": {
///             "freshness1": <rev_state1>,
///             "freshness2": <rev_state2>,
///         },
///         "credential2_referent_in_wallet": {
///             "freshness3": <rev_state3>
///         },
///         "credential3_referent_in_wallet": {
///             "freshness4": <rev_state4>
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
                                       master_secret_id: *const c_char,
                                       schemas_json: *const c_char,
                                       credential_defs_json: *const c_char,
                                       rev_states_json: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                            proof_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(proof_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(requested_credentials_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(master_secret_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(credential_defs_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_str!(rev_states_json, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateProof(
            wallet_handle,
            proof_req_json,
            requested_credentials_json,
            master_secret_id,
            schemas_json,
            credential_defs_json,
            rev_states_json,
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
pub extern fn indy_create_revocation_state(command_handle: i32,
                                           blob_storage_reader_handle: i32,
                                           rev_reg_def_json: *const c_char,
                                           rev_reg_delta_json: *const c_char,
                                           timestamp: u64,
                                           cred_rev_id: *const c_char,
                                           cb: Option<extern fn(
                                               xcommand_handle: i32, err: ErrorCode,
                                               rev_state_json: *const c_char
                                           )>) -> ErrorCode {
    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_rev_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateRevocationState(
            blob_storage_reader_handle,
            rev_reg_def_json,
            rev_reg_delta_json,
            timestamp,
            cred_rev_id,
            Box::new(move |result| {
                let (err, rev_state_json) = result_to_err_code_1!(result, String::new());
                let rev_state_json = CStringUtils::string_to_cstring(rev_state_json);
                cb(command_handle, err, rev_state_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_update_revocation_state(command_handle: i32,
                                           blob_storage_reader_handle: i32,
                                           rev_state_json: *const c_char,
                                           rev_reg_def_json: *const c_char,
                                           rev_reg_delta_json: *const c_char,
                                           timestamp: u64,
                                           cred_rev_id: *const c_char,
                                           cb: Option<extern fn(
                                               xcommand_handle: i32, err: ErrorCode,
                                               updated_rev_state_json: *const c_char
                                           )>) -> ErrorCode {
    check_useful_c_str!(rev_state_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_rev_id, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::UpdateRevocationState(
            blob_storage_reader_handle,
            rev_state_json,
            rev_reg_def_json,
            rev_reg_delta_json,
            timestamp,
            cred_rev_id,
            Box::new(move |result| {
                let (err, updated_rev_info_json) = result_to_err_code_1!(result, String::new());
                let updated_rev_info_json = CStringUtils::string_to_cstring(updated_rev_info_json);
                cb(command_handle, err, updated_rev_info_json.as_ptr())
            })
        ))));

    result_to_err_code!(result)
}