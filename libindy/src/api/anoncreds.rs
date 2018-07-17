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
#[no_mangle]
pub extern fn indy_issuer_create_schema(command_handle: i32,
                                        issuer_did: *const c_char,
                                        name: *const c_char,
                                        version: *const c_char,
                                        attrs: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             schema_id: *const c_char, schema_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_create_schema: >>> issuer_did: {:?}, name: {:?}, version: {:?}, attrs: {:?}", issuer_did, name, version, attrs);

    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(version, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(attrs, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_issuer_create_schema: entity >>> issuer_did: {:?}, name: {:?}, version: {:?}, attrs: {:?}", issuer_did, name, version, attrs);

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
                        trace!("indy_crypto_cl_credential_public_key_to_json: id: {:?}, schema_json: {:?}", id, schema_json);
                        let id = CStringUtils::string_to_cstring(id);
                        let schema_json = CStringUtils::string_to_cstring(schema_json);
                        cb(command_handle, err, id.as_ptr(), schema_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_create_schema: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_issuer_create_and_store_credential_def(command_handle: i32,
                                                          wallet_handle: i32,
                                                          issuer_did: *const c_char,
                                                          schema_json: *const c_char,
                                                          tag: *const c_char,
                                                          signature_type: *const c_char,
                                                          config_json: *const c_char,
                                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                               cred_def_id: *const c_char,
                                                                               cred_def_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_create_and_store_credential_def: >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, tag: {:?}, \
    signature_type: {:?}, config_json: {:?}", wallet_handle, issuer_did, schema_json, tag, signature_type, config_json);

    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schema_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(signature_type, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(config_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    trace!("indy_issuer_create_and_store_credential_def: entities >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, tag: {:?}, \
    signature_type: {:?}, config_json: {:?}", wallet_handle, issuer_did, schema_json, tag, signature_type, config_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateAndStoreCredentialDefinition(
                    wallet_handle,
                    issuer_did,
                    schema_json,
                    tag,
                    signature_type,
                    config_json,
                    Box::new(move |result| {
                        let (err, cred_def_id, cred_def_json) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_issuer_create_and_store_credential_def: cred_def_id: {:?}, cred_def_json: {:?}", cred_def_id, cred_def_json);
                        let cred_def_id = CStringUtils::string_to_cstring(cred_def_id);
                        let cred_def_json = CStringUtils::string_to_cstring(cred_def_json);
                        cb(command_handle, err, cred_def_id.as_ptr(), cred_def_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_create_and_store_credential_def: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_issuer_create_and_store_revoc_reg(command_handle: i32,
                                                     wallet_handle: i32,
                                                     issuer_did: *const c_char,
                                                     revoc_def_type: *const c_char,
                                                     tag: *const c_char,
                                                     cred_def_id: *const c_char,
                                                     config_json: *const c_char,
                                                     tails_writer_handle: i32,
                                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                          revoc_reg_id: *const c_char,
                                                                          revoc_reg_def_json: *const c_char,
                                                                          revoc_reg_entry_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_create_and_store_credential_def: >>> wallet_handle: {:?}, issuer_did: {:?}, revoc_def_type: {:?}, tag: {:?}, \
    cred_def_id: {:?}, config_json: {:?}, tails_writer_handle: {:?}", wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle);

    check_useful_c_str!(issuer_did, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(revoc_def_type, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_def_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    trace!("indy_issuer_create_and_store_credential_def: entities >>> wallet_handle: {:?}, issuer_did: {:?}, revoc_def_type: {:?}, tag: {:?}, \
    cred_def_id: {:?}, config_json: {:?}, tails_writer_handle: {:?}", wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateAndStoreRevocationRegistry(
                    wallet_handle,
                    issuer_did,
                    revoc_def_type,
                    tag,
                    cred_def_id,
                    config_json,
                    tails_writer_handle,
                    Box::new(move |result| {
                        let (err, revoc_reg_id, revoc_reg_def_json, revoc_reg_json) = result_to_err_code_3!(result, String::new(), String::new(), String::new());
                        trace!("indy_issuer_create_and_store_credential_def: revoc_reg_id: {:?}, revoc_reg_def_json: {:?}, revoc_reg_json: {:?}",
                               revoc_reg_id, revoc_reg_def_json, revoc_reg_json);
                        let revoc_reg_id = CStringUtils::string_to_cstring(revoc_reg_id);
                        let revoc_reg_def_json = CStringUtils::string_to_cstring(revoc_reg_def_json);
                        let revoc_reg_json = CStringUtils::string_to_cstring(revoc_reg_json);
                        cb(command_handle, err, revoc_reg_id.as_ptr(), revoc_reg_def_json.as_ptr(), revoc_reg_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_create_and_store_credential_def: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_issuer_create_credential_offer(command_handle: i32,
                                                  wallet_handle: i32,
                                                  cred_def_id: *const c_char,
                                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                       cred_offer_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_create_credential_offer: >>> wallet_handle: {:?}, cred_def_id: {:?}", wallet_handle, cred_def_id);

    check_useful_c_str!(cred_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_issuer_create_credential_offer: entities >>> wallet_handle: {:?}, cred_def_id: {:?}", wallet_handle, cred_def_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateCredentialOffer(
                    wallet_handle,
                    cred_def_id,
                    Box::new(move |result| {
                        let (err, cred_offer_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_issuer_create_credential_offer: cred_offer_json: {:?}", cred_offer_json);
                        let cred_offer_json = CStringUtils::string_to_cstring(cred_offer_json);
                        cb(command_handle, err, cred_offer_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_create_credential_offer: <<< res: {:?}", res);

    res
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
                                                                 cred_revoc_id: *const c_char,
                                                                 revoc_reg_delta_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_create_credential: >>> wallet_handle: {:?}, cred_offer_json: {:?}, cred_req_json: {:?}, cred_values_json: {:?}, rev_reg_id: {:?}, \
    blob_storage_reader_handle: {:?}", wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle);

    check_useful_c_str!(cred_offer_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_req_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_values_json, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let blob_storage_reader_handle = if blob_storage_reader_handle != -1 { Some(blob_storage_reader_handle) } else { None };

    trace!("indy_issuer_create_credential: entities >>> wallet_handle: {:?}, cred_offer_json: {:?}, cred_req_json: {:?}, cred_values_json: {:?}, rev_reg_id: {:?}, \
    blob_storage_reader_handle: {:?}", wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::CreateCredential(
                    wallet_handle,
                    cred_offer_json,
                    cred_req_json,
                    cred_values_json,
                    rev_reg_id,
                    blob_storage_reader_handle,
                    Box::new(move |result| {
                        let (err, cred_json, revoc_id, revoc_reg_delta_json) = result_to_err_code_3!(result, String::new(), None, None);
                        trace!("indy_issuer_create_credential: cred_json: {:?}, revoc_id: {:?}, revoc_reg_delta_json: {:?}", cred_json, revoc_id, revoc_reg_delta_json);
                        let cred_json = CStringUtils::string_to_cstring(cred_json);
                        let revoc_id = revoc_id.map(CStringUtils::string_to_cstring);
                        let revoc_reg_delta_json = revoc_reg_delta_json.map(CStringUtils::string_to_cstring);
                        cb(command_handle, err, cred_json.as_ptr(),
                           revoc_id.as_ref().map(|id| id.as_ptr()).unwrap_or(ptr::null()),
                           revoc_reg_delta_json.as_ref().map(|delta| delta.as_ptr()).unwrap_or(ptr::null()))
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_create_credential: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_issuer_revoke_credential(command_handle: i32,
                                            wallet_handle: i32,
                                            blob_storage_reader_cfg_handle: i32,
                                            rev_reg_id: *const c_char,
                                            cred_revoc_id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 revoc_reg_delta_json: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_revoke_credential: >>> wallet_handle: {:?}, blob_storage_reader_cfg_handle: {:?}, rev_reg_id: {:?}, cred_revoc_id: {:?}",
           wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id);

    check_useful_c_str!(rev_reg_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_revoc_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_issuer_revoke_credential: entities >>> wallet_handle: {:?}, blob_storage_reader_cfg_handle: {:?}, rev_reg_id: {:?}, cred_revoc_id: {:?}",
           wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::RevokeCredential(
                    wallet_handle,
                    blob_storage_reader_cfg_handle,
                    rev_reg_id,
                    cred_revoc_id,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_issuer_revoke_credential: revoc_reg_update_json: {:?}", revoc_reg_update_json);
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_revoke_credential: <<< res: {:?}", res);

    res
}

/*/// Recover a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
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
pub extern fn indy_issuer_recover_credential(command_handle: i32,
                                             wallet_handle: i32,
                                             blob_storage_reader_cfg_handle: i32,
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
                    blob_storage_reader_cfg_handle,
                    rev_reg_id,
                    cred_revoc_id,
                    Box::new(move |result| {
                        let (err, revoc_reg_update_json) = result_to_err_code_1!(result, String::new());
                        let revoc_reg_update_json = CStringUtils::string_to_cstring(revoc_reg_update_json);
                        cb(command_handle, err, revoc_reg_update_json.as_ptr())
                    })
                ))));

    result_to_err_code!(result)
}*/

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
#[no_mangle]
pub extern fn indy_issuer_merge_revocation_registry_deltas(command_handle: i32,
                                                           rev_reg_delta_json: *const c_char,
                                                           other_rev_reg_delta_json: *const c_char,
                                                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                                merged_rev_reg_delta: *const c_char)>) -> ErrorCode {
    trace!("indy_issuer_merge_revocation_registry_deltas: >>> rev_reg_delta_json: {:?}, other_rev_reg_delta_json: {:?}",
           rev_reg_delta_json, other_rev_reg_delta_json);

    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(other_rev_reg_delta_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_issuer_merge_revocation_registry_deltas: entities >>> rev_reg_delta_json: {:?}, other_rev_reg_delta_json: {:?}",
           rev_reg_delta_json, other_rev_reg_delta_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Issuer(
                IssuerCommand::MergeRevocationRegistryDeltas(
                    rev_reg_delta_json,
                    other_rev_reg_delta_json,
                    Box::new(move |result| {
                        let (err, merged_rev_reg_delta) = result_to_err_code_1!(result, String::new());
                        trace!("indy_issuer_merge_revocation_registry_deltas: merged_rev_reg_delta: {:?}", merged_rev_reg_delta);
                        let merged_rev_reg_delta = CStringUtils::string_to_cstring(merged_rev_reg_delta);
                        cb(command_handle, err, merged_rev_reg_delta.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_issuer_merge_revocation_registry_deltas: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_create_master_secret(command_handle: i32,
                                               wallet_handle: i32,
                                               master_secret_id: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                    out_master_secret_id: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_create_master_secret: >>> wallet_handle: {:?}, master_secret_id: {:?}", wallet_handle, master_secret_id);

    check_useful_opt_c_str!(master_secret_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_prover_create_master_secret: entities >>> wallet_handle: {:?}, master_secret_id: {:?}", wallet_handle, master_secret_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::CreateMasterSecret(
                    wallet_handle,
                    master_secret_id,
                    Box::new(move |result| {
                        let (err, out_master_secret_id) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_create_master_secret: out_master_secret_id: {:?}", out_master_secret_id);
                        let out_master_secret_id = CStringUtils::string_to_cstring(out_master_secret_id);
                        cb(command_handle, err, out_master_secret_id.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_create_master_secret: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_create_credential_req(command_handle: i32,
                                                wallet_handle: i32,
                                                prover_did: *const c_char,
                                                cred_offer_json: *const c_char,
                                                cred_def_json: *const c_char,
                                                master_secret_id: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                     cred_req_json: *const c_char,
                                                                     cred_req_metadata_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_create_credential_req: >>> wallet_handle: {:?}, prover_did: {:?}, cred_offer_json: {:?}, cred_def_json: {:?}, master_secret_id: {:?}",
           wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id);

    check_useful_c_str!(prover_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_offer_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_def_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(master_secret_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_prover_create_credential_req: entities >>> wallet_handle: {:?}, prover_did: {:?}, cred_offer_json: {:?}, cred_def_json: {:?}, master_secret_id: {:?}",
           wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::CreateCredentialRequest(
                    wallet_handle,
                    prover_did,
                    cred_offer_json,
                    cred_def_json,
                    master_secret_id,
                    Box::new(move |result| {
                        let (err, cred_req_json, cred_req_metadata_json) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_prover_create_credential_req: cred_req_json: {:?}, cred_req_metadata_json: {:?}", cred_req_json, cred_req_metadata_json);
                        let cred_req_json = CStringUtils::string_to_cstring(cred_req_json);
                        let cred_req_metadata_json = CStringUtils::string_to_cstring(cred_req_metadata_json);
                        cb(command_handle, err, cred_req_json.as_ptr(), cred_req_metadata_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_create_credential_req: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_store_credential(command_handle: i32,
                                           wallet_handle: i32,
                                           cred_id: *const c_char,
                                           cred_req_metadata_json: *const c_char,
                                           cred_json: *const c_char,
                                           cred_def_json: *const c_char,
                                           rev_reg_def_json: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                out_cred_id: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_store_credential: >>> wallet_handle: {:?}, cred_id: {:?}, cred_req_metadata_json: {:?}, cred_json: {:?}, cred_def_json: {:?}, \
    cred_def_json: {:?}", wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json);

    check_useful_opt_c_str!(cred_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cred_req_metadata_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_def_json, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    trace!("indy_prover_store_credential: entities >>> wallet_handle: {:?}, cred_id: {:?}, cred_req_metadata_json: {:?}, cred_json: {:?}, cred_def_json: {:?}, \
    cred_def_json: {:?}", wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::StoreCredential(
                    wallet_handle,
                    cred_id,
                    cred_req_metadata_json,
                    cred_json,
                    cred_def_json,
                    rev_reg_def_json,
                    Box::new(move |result| {
                        let (err, out_cred_id) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_store_credential: out_cred_id: {:?}", out_cred_id);
                        let out_cred_id = CStringUtils::string_to_cstring(out_cred_id);
                        cb(command_handle, err, out_cred_id.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_store_credential: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_get_credential(command_handle: i32,
                                         wallet_handle: i32,
                                         cred_id: *const c_char,
                                         cb: Option<extern fn(
                                             xcommand_handle: i32, err: ErrorCode,
                                             credential_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_get_credential: >>> wallet_handle: {:?}, cred_id: {:?}", wallet_handle, cred_id);

    check_useful_c_str!(cred_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_prover_get_credential: entities >>> wallet_handle: {:?}, cred_id: {:?}", cred_id, cred_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::GetCredential(
                    wallet_handle,
                    cred_id,
                    Box::new(move |result| {
                        let (err, credential_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_get_credential: credential_json: {:?}", credential_json);
                        let credential_json = CStringUtils::string_to_cstring(credential_json);
                        cb(command_handle, err, credential_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_get_credential: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_get_credentials(command_handle: i32,
                                          wallet_handle: i32,
                                          filter_json: *const c_char,
                                          cb: Option<extern fn(
                                              xcommand_handle: i32, err: ErrorCode,
                                              matched_credentials_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_get_credentials: >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

    check_useful_opt_c_str!(filter_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_prover_get_credentials: entities >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::GetCredentials(
                    wallet_handle,
                    filter_json,
                    Box::new(move |result| {
                        let (err, matched_credentials_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_get_credentials: matched_credentials_json: {:?}", matched_credentials_json);
                        let matched_credentials_json = CStringUtils::string_to_cstring(matched_credentials_json);
                        cb(command_handle, err, matched_credentials_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_get_credentials: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_search_credentials(command_handle: i32,
                                             wallet_handle: i32,
                                             query_json: *const c_char,
                                             cb: Option<extern fn(
                                                 xcommand_handle: i32, err: ErrorCode,
                                                 search_handle: i32,
                                                 total_count: usize)>) -> ErrorCode {
    trace!("indy_prover_search_credentials: >>> wallet_handle: {:?}, query_json: {:?}", wallet_handle, query_json);

    check_useful_opt_c_str!(query_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_prover_search_credentials: entities >>> wallet_handle: {:?}, query_json: {:?}", wallet_handle, query_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::SearchCredentials(
                    wallet_handle,
                    query_json,
                    Box::new(move |result| {
                        let (err, handle, total_count) = result_to_err_code_2!(result, 0, 0);
                        cb(command_handle, err, handle, total_count)
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_search_credentials: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub  extern fn indy_prover_fetch_credentials(command_handle: i32,
                                             search_handle: i32,
                                             count: usize,
                                             cb: Option<extern fn(command_handle_: i32, err: ErrorCode,
                                                                  credentials_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_fetch_credentials: >>> search_handle: {:?}, count: {:?}", search_handle, count);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_prover_fetch_credentials: entities >>> search_handle: {:?}, count: {:?}", search_handle, count);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::FetchCredentials(
                    search_handle,
                    count,
                    Box::new(move |result| {
                        let (err, credentials_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_fetch_credentials: credentials_json: {:?}", credentials_json);
                        let credentials_json = CStringUtils::string_to_cstring(credentials_json);
                        cb(command_handle, err, credentials_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_fetch_credentials: <<< res: {:?}", res);

    res
}

/// Close credentials search (make search handle invalid)
///
/// #Params
/// search_handle: Search handle (created by indy_prover_search_credentials)
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_prover_close_credentials_search(command_handle: i32,
                                                    search_handle: i32,
                                                    cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_prover_close_credentials_search: >>> search_handle: {:?}", search_handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_prover_close_credentials_search: entities >>> search_handle: {:?}", search_handle);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::CloseCredentialsSearch(
                    search_handle,
                    Box::new(move |result| {
                        let err = result_to_err_code!(result);
                        trace!("indy_prover_close_credentials_search:");
                        cb(command_handle, err)
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_close_credentials_search: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_get_credentials_for_proof_req(command_handle: i32,
                                                        wallet_handle: i32,
                                                        proof_request_json: *const c_char,
                                                        cb: Option<extern fn(
                                                            xcommand_handle: i32, err: ErrorCode,
                                                            credentials_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_get_credentials_for_proof_req: >>> wallet_handle: {:?}, proof_request_json: {:?}", wallet_handle, proof_request_json);

    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_prover_get_credentials_for_proof_req: entities >>> wallet_handle: {:?}, proof_request_json: {:?}",
           wallet_handle, proof_request_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::GetCredentialsForProofReq(
                    wallet_handle,
                    proof_request_json,
                    Box::new(move |result| {
                        let (err, credentials_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_get_credentials_for_proof_req: credentials_json: {:?}", credentials_json);
                        let credentials_json = CStringUtils::string_to_cstring(credentials_json);
                        cb(command_handle, err, credentials_json.as_ptr())
                    })
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_get_credentials_for_proof_req: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_prover_search_credentials_for_proof_req(command_handle: i32,
                                                           wallet_handle: i32,
                                                           proof_request_json: *const c_char,
                                                           extra_query_json: *const c_char,
                                                           cb: Option<extern fn(
                                                               xcommand_handle: i32, err: ErrorCode,
                                                               search_handle: i32)>) -> ErrorCode {
    trace!("indy_prover_search_credentials_for_proof_req: >>> wallet_handle: {:?}, proof_request_json: {:?}, extra_query_json: {:?}", wallet_handle, proof_request_json, extra_query_json);

    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(extra_query_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_prover_search_credentials_for_proof_req: entities >>> wallet_handle: {:?}, proof_request_json: {:?}, extra_query_json: {:?}",
           wallet_handle, proof_request_json, extra_query_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::SearchCredentialsForProofReq(
                    wallet_handle,
                    proof_request_json,
                    extra_query_json,
                    Box::new(move |result| {
                        let (err, search_handle) = result_to_err_code_1!(result, 0);
                        trace!("indy_prover_search_credentials_for_proof_req: search_handle: {:?}", search_handle);
                        cb(command_handle, err, search_handle)
                    }),
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_search_credentials_for_proof_req: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub  extern fn indy_prover_fetch_credentials_for_proof_req(command_handle: i32,
                                                           search_handle: i32,
                                                           item_referent: *const c_char,
                                                           count: usize,
                                                           cb: Option<extern fn(command_handle_: i32, err: ErrorCode,
                                                                                    credentials_json: *const c_char)>) -> ErrorCode {
    trace!("indy_prover_fetch_credentials_for_proof_req: >>> search_handle: {:?}, count: {:?}", search_handle, count);

    check_useful_c_str!(item_referent, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_prover_fetch_credentials_for_proof_req: entities >>> search_handle: {:?}, count: {:?}", search_handle, count);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::FetchCredentialForProofReq(
                    search_handle,
                    item_referent,
                    count,
                    Box::new(move |result| {
                        let (err, credentials_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_prover_fetch_credentials_for_proof_request: credentials_json: {:?}", credentials_json);
                        let credentials_json = CStringUtils::string_to_cstring(credentials_json);
                        cb(command_handle, err, credentials_json.as_ptr())
                    }),
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_fetch_credentials_for_proof_req: <<< res: {:?}", res);

    res
}

/// Close credentials search for proof request (make search handle invalid)
///
/// #Params
/// search_handle: Search handle (created by indy_prover_search_credentials_for_proof_req)
///
/// #Errors
/// Annoncreds*
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_prover_close_credentials_search_for_proof_req(command_handle: i32,
                                                                  search_handle: i32,
                                                                  cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_prover_close_credentials_search_for_proof_req: >>> search_handle: {:?}", search_handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_prover_close_credentials_search_for_proof_req: entities >>> search_handle: {:?}", search_handle);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(
            AnoncredsCommand::Prover(
                ProverCommand::CloseCredentialsSearchForProofReq(
                    search_handle,
                    Box::new(move |result| {
                        let err = result_to_err_code!(result);
                        trace!("indy_prover_close_credentials_search:");
                        cb(command_handle, err)
                    }),
                ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_close_credentials_search_for_proof_req: <<< res: {:?}", res);

    res
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
    trace!("indy_prover_create_proof: >>> wallet_handle: {:?}, proof_req_json: {:?}, requested_credentials_json: {:?}, master_secret_id: {:?}, \
    schemas_json: {:?}, credential_defs_json: {:?}, rev_states_json: {:?}",
           wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json);

    check_useful_c_str!(proof_req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(requested_credentials_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(master_secret_id, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(credential_defs_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_str!(rev_states_json, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    trace!("indy_prover_create_proof: entities >>> wallet_handle: {:?}, proof_req_json: {:?}, requested_credentials_json: {:?}, master_secret_id: {:?}, \
    schemas_json: {:?}, credential_defs_json: {:?}, rev_states_json: {:?}",
           wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json);

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
                trace!("indy_prover_create_proof: proof_json: {:?}", proof_json);
                let proof_json = CStringUtils::string_to_cstring(proof_json);
                cb(command_handle, err, proof_json.as_ptr())
            })
        ))));

    let res = result_to_err_code!(result);

    trace!("indy_prover_create_proof: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_verifier_verify_proof(command_handle: i32,
                                         proof_request_json: *const c_char,
                                         proof_json: *const c_char,
                                         schemas_json: *const c_char,
                                         credential_defs_json: *const c_char,
                                         rev_reg_defs_json: *const c_char,
                                         rev_regs_json: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              valid: bool)>) -> ErrorCode {
    trace!("indy_verifier_verify_proof: >>> proof_request_json: {:?}, proof_json: {:?}, schemas_json: {:?}, credential_defs_json: {:?}, \
    rev_reg_defs_json: {:?}, rev_regs_json: {:?}", proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json);

    check_useful_c_str!(proof_request_json, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(proof_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(schemas_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(credential_defs_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(rev_reg_defs_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_str!(rev_regs_json, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    trace!("indy_verifier_verify_proof: entities >>> proof_request_json: {:?}, proof_json: {:?}, schemas_json: {:?}, credential_defs_json: {:?}, \
    rev_reg_defs_json: {:?}, rev_regs_json: {:?}", proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Verifier(VerifierCommand::VerifyProof(
            proof_request_json,
            proof_json,
            schemas_json,
            credential_defs_json,
            rev_reg_defs_json,
            rev_regs_json,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                trace!("indy_verifier_verify_proof: valid: {:?}", valid);

                cb(command_handle, err, valid)
            })
        ))));

    let res = result_to_err_code!(result);

    trace!("indy_verifier_verify_proof: <<< res: {:?}", res);

    res
}

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
#[no_mangle]
pub extern fn indy_create_revocation_state(command_handle: i32,
                                           blob_storage_reader_handle: i32,
                                           rev_reg_def_json: *const c_char,
                                           rev_reg_delta_json: *const c_char,
                                           timestamp: u64,
                                           cred_rev_id: *const c_char,
                                           cb: Option<extern fn(
                                               xcommand_handle: i32, err: ErrorCode,
                                               rev_state_json: *const c_char)>) -> ErrorCode {
    trace!("indy_create_revocation_state: >>> blob_storage_reader_handle: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, \
    cred_rev_id: {:?}", blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id);

    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(cred_rev_id, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_create_revocation_state: entities >>> blob_storage_reader_handle: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, \
    cred_rev_id: {:?}", blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id);

    let result = CommandExecutor::instance()
        .send(Command::Anoncreds(AnoncredsCommand::Prover(ProverCommand::CreateRevocationState(
            blob_storage_reader_handle,
            rev_reg_def_json,
            rev_reg_delta_json,
            timestamp,
            cred_rev_id,
            Box::new(move |result| {
                let (err, rev_state_json) = result_to_err_code_1!(result, String::new());
                trace!("indy_create_revocation_state: rev_state_json: {:?}", rev_state_json);
                let rev_state_json = CStringUtils::string_to_cstring(rev_state_json);
                cb(command_handle, err, rev_state_json.as_ptr())
            })
        ))));

    let res = result_to_err_code!(result);

    trace!("indy_create_revocation_state: <<< res: {:?}", res);

    res
}

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
                                               updated_rev_state_json: *const c_char)>) -> ErrorCode {
    trace!("indy_update_revocation_state: >>> blob_storage_reader_handle: {:?}, rev_state_json: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, \
    timestamp: {:?}, cred_rev_id: {:?}", blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id);

    check_useful_c_str!(rev_state_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_reg_def_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(rev_reg_delta_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(cred_rev_id, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    trace!("indy_update_revocation_state: entities >>> blob_storage_reader_handle: {:?}, rev_state_json: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, \
    timestamp: {:?}, cred_rev_id: {:?}", blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id);

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
                trace!("indy_update_revocation_state: updated_rev_info_json: {:?}", updated_rev_info_json);
                let updated_rev_info_json = CStringUtils::string_to_cstring(updated_rev_info_json);
                cb(command_handle, err, updated_rev_info_json.as_ptr())
            })
        ))));

    let res = result_to_err_code!(result);

    trace!("indy_update_revocation_state: <<< res: {:?}", res);

    res
}