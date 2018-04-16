extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

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
#[no_mangle]
pub extern fn indy_sign_and_submit_request(command_handle: i32,
                                           pool_handle: i32,
                                           wallet_handle: i32,
                                           submitter_did: *const c_char,
                                           request_json: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                request_result_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(request_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::SignAndSubmitRequest(
            pool_handle,
            wallet_handle,
            submitter_did,
            request_json,
            Box::new(move |result| {
                let (err, request_result_json) = result_to_err_code_1!(result, String::new());
                let request_result_json = CStringUtils::string_to_cstring(request_result_json);
                cb(command_handle, err, request_result_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_submit_request(command_handle: i32,
                                  pool_handle: i32,
                                  request_json: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                       request_result_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(request_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::SubmitRequest(
            pool_handle,
            request_json,
            Box::new(move |result| {
                let (err, request_result_json) = result_to_err_code_1!(result, String::new());
                let request_result_json = CStringUtils::string_to_cstring(request_result_json);
                cb(command_handle, err, request_result_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_sign_request(command_handle: i32,
                                wallet_handle: i32,
                                submitter_did: *const c_char,
                                request_json: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     signed_request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(request_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::SignRequest(
            wallet_handle,
            submitter_did,
            request_json,
            Box::new(move |result| {
                let (err, signed_request_json) = result_to_err_code_1!(result, String::new());
                let signed_request_json = CStringUtils::string_to_cstring(signed_request_json);
                cb(command_handle, err, signed_request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


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
#[no_mangle]
pub extern fn indy_build_get_ddo_request(command_handle: i32,
                                         submitter_did: *const c_char,
                                         target_did: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetDdoRequest(
            submitter_did,
            target_did,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


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
#[no_mangle]
pub extern fn indy_build_nym_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     target_did: *const c_char,
                                     verkey: *const c_char,
                                     alias: *const c_char,
                                     role: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                          request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(verkey, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(alias, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(role, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildNymRequest(
            submitter_did,
            target_did,
            verkey,
            alias,
            role,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_attrib_request(command_handle: i32,
                                        submitter_did: *const c_char,
                                        target_did: *const c_char,
                                        hash: *const c_char,
                                        raw: *const c_char,
                                        enc: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(hash, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(raw, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(enc, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildAttribRequest(
            submitter_did,
            target_did,
            hash,
            raw,
            enc,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_attrib_request(command_handle: i32,
                                            submitter_did: *const c_char,
                                            target_did: *const c_char,
                                            raw: *const c_char,
                                            hash: *const c_char,
                                            enc: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(raw, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(hash, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(enc, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetAttribRequest(
            submitter_did,
            target_did,
            raw,
            hash,
            enc,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_nym_request(command_handle: i32,
                                         submitter_did: *const c_char,
                                         target_did: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetNymRequest(
            submitter_did,
            target_did,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_schema_request(command_handle: i32,
                                        submitter_did: *const c_char,
                                        data: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(data, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildSchemaRequest(
            submitter_did,
            data,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_schema_request(command_handle: i32,
                                            submitter_did: *const c_char,
                                            id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetSchemaRequest(
            submitter_did,
            id,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_parse_get_schema_response(command_handle: i32,
                                             get_schema_response: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  schema_id: *const c_char,
                                                                  schema_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(get_schema_response, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::ParseGetSchemaResponse(
            get_schema_response,
            Box::new(move |result| {
                let (err, schema_id, schema_json) = result_to_err_code_2!(result, String::new(), String::new());
                let schema_id = CStringUtils::string_to_cstring(schema_id);
                let schema_json = CStringUtils::string_to_cstring(schema_json);
                cb(command_handle, err, schema_id.as_ptr(), schema_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_cred_def_request(command_handle: i32,
                                          submitter_did: *const c_char,
                                          data: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                               request_result_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(data, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildCredDefRequest(
            submitter_did,
            data,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_cred_def_request(command_handle: i32,
                                              submitter_did: *const c_char,
                                              id: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                   request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetCredDefRequest(
            submitter_did,
            id,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_parse_get_cred_def_response(command_handle: i32,
                                               get_cred_def_response: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                    cred_def_id: *const c_char,
                                                                    cred_def_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(get_cred_def_response, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::ParseGetCredDefResponse(
            get_cred_def_response,
            Box::new(move |result| {
                let (err, cred_def_id, cred_def_json) = result_to_err_code_2!(result, String::new(), String::new());
                let cred_def_id = CStringUtils::string_to_cstring(cred_def_id);
                let cred_def_json = CStringUtils::string_to_cstring(cred_def_json);
                cb(command_handle, err, cred_def_id.as_ptr(), cred_def_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// submitter_did: DID of the submitter stored in secured Wallet.
/// target_did: Target Node's DID.  It differs from submitter_did field.
/// data: Data associated with the Node: {
///     alias: string - Node's alias
///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
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
#[no_mangle]
pub extern fn indy_build_node_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      target_did: *const c_char,
                                      data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(target_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(data, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildNodeRequest(
            submitter_did,
            target_did,
            data,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Builds a GET_TXN request. Request to get any transaction by its seq_no.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// submitter_did: DID of the request submitter.
/// seq_no: seq_no of transaction in ledger.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Request result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_build_get_txn_request(command_handle: i32,
                                         submitter_did: *const c_char,
                                         seq_no: i32,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetTxnRequest(
            submitter_did,
            seq_no,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_pool_config_request(command_handle: i32,
                                             submitter_did: *const c_char,
                                             writes: bool,
                                             force: bool,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildPoolConfigRequest(
            submitter_did,
            writes,
            force,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Builds a POOL_RESTART request.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// submitter_did: Id of Identity stored in secured Wallet.
/// action:
/// datetime:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Request result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_build_pool_restart_request(command_handle: i32,
                                              submitter_did: *const c_char,
                                              action: *const c_char,
                                              datetime: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                  request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(action, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(datetime, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildPoolRestartRequest(submitter_did,
                                                                     action,
                                                                     datetime,
                                                                     Box::new(move |result| {
                                                                         let (err, request_json) = result_to_err_code_1!(result, String::new());
                                                                         let request_json = CStringUtils::string_to_cstring(request_json);
                                                                         cb(command_handle, err, request_json.as_ptr())
                                                                     }))));

    result_to_err_code!(result)
}


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
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Request result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_build_pool_upgrade_request(command_handle: i32,
                                              submitter_did: *const c_char,
                                              name: *const c_char,
                                              version: *const c_char,
                                              action: *const c_char,
                                              sha256: *const c_char,
                                              timeout: i32,
                                              schedule: *const c_char,
                                              justification: *const c_char,
                                              reinstall: bool,
                                              force: bool,
                                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                   request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(version, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(action, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(sha256, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(schedule, ErrorCode::CommonInvalidParam8);
    check_useful_opt_c_str!(justification, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam12);

    let timeout = if timeout != -1 { Some(timeout as u32) } else { None };


    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildPoolUpgradeRequest(
            submitter_did,
            name,
            version,
            action,
            sha256,
            timeout,
            schedule,
            justification,
            reinstall,
            force,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_revoc_reg_def_request(command_handle: i32,
                                               submitter_did: *const c_char,
                                               data: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                    rev_reg_def_req: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(data, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildRevocRegDefRequest(
            submitter_did,
            data,
            Box::new(move |result| {
                let (err, rev_reg_def_req) = result_to_err_code_1!(result, String::new());
                let rev_reg_def_req = CStringUtils::string_to_cstring(rev_reg_def_req);
                cb(command_handle, err, rev_reg_def_req.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_revoc_reg_def_request(command_handle: i32,
                                                   submitter_did: *const c_char,
                                                   id: *const c_char,
                                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                        request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetRevocRegDefRequest(
            submitter_did,
            id,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_parse_get_revoc_reg_def_response(command_handle: i32,
                                                    get_revoc_reg_def_response: *const c_char,
                                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                         revoc_reg_def_id: *const c_char,
                                                                         revoc_reg_def_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(get_revoc_reg_def_response, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::ParseGetRevocRegDefResponse(
            get_revoc_reg_def_response,
            Box::new(move |result| {
                let (err, revoc_reg_def_id, revoc_reg_def_json) = result_to_err_code_2!(result, String::new(), String::new());
                let revoc_reg_def_id = CStringUtils::string_to_cstring(revoc_reg_def_id);
                let revoc_reg_def_json = CStringUtils::string_to_cstring(revoc_reg_def_json);
                cb(command_handle, err, revoc_reg_def_id.as_ptr(), revoc_reg_def_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_revoc_reg_entry_request(command_handle: i32,
                                                 submitter_did: *const c_char,
                                                 revoc_reg_def_id: *const c_char,
                                                 rev_def_type: *const c_char,
                                                 value: *const c_char,
                                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                      request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(revoc_reg_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(rev_def_type, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(value, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildRevocRegEntryRequest(
            submitter_did,
            revoc_reg_def_id,
            rev_def_type,
            value,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_revoc_reg_request(command_handle: i32,
                                               submitter_did: *const c_char,
                                               revoc_reg_def_id: *const c_char,
                                               timestamp: i64,
                                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                    request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(revoc_reg_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetRevocRegRequest(
            submitter_did,
            revoc_reg_def_id,
            timestamp,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
///         "accum": string - current accumulator value.
///     },
///     "ver": string - version revocation registry json
/// }
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_parse_get_revoc_reg_response(command_handle: i32,
                                                get_revoc_reg_response: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                     revoc_reg_def_id: *const c_char,
                                                                     revoc_reg_json: *const c_char,
                                                                     timestamp: u64)>) -> ErrorCode {
    check_useful_c_str!(get_revoc_reg_response, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::ParseGetRevocRegResponse(
            get_revoc_reg_response,
            Box::new(move |result| {
                let (err, revoc_reg_def_id, revoc_reg_json, timestamp) = result_to_err_code_3!(result, String::new(), String::new(), 0);
                let revoc_reg_def_id = CStringUtils::string_to_cstring(revoc_reg_def_id);
                let revoc_reg_json = CStringUtils::string_to_cstring(revoc_reg_json);
                cb(command_handle, err, revoc_reg_def_id.as_ptr(), revoc_reg_json.as_ptr(), timestamp)
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_build_get_revoc_reg_delta_request(command_handle: i32,
                                                     submitter_did: *const c_char,
                                                     revoc_reg_def_id: *const c_char,
                                                     from: i64,
                                                     to: i64,
                                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                          request_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(revoc_reg_def_id, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let from = if from != -1 { Some(from) } else { None };

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::BuildGetRevocRegDeltaRequest(
            submitter_did,
            revoc_reg_def_id,
            from,
            to,
            Box::new(move |result| {
                let (err, request_json) = result_to_err_code_1!(result, String::new());
                let request_json = CStringUtils::string_to_cstring(request_json);
                cb(command_handle, err, request_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_parse_get_revoc_reg_delta_response(command_handle: i32,
                                                      get_revoc_reg_delta_response: *const c_char,
                                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                           revoc_reg_def_id: *const c_char,
                                                                           revoc_reg_delta_json: *const c_char,
                                                                           timestamp: u64)>) -> ErrorCode {
    check_useful_c_str!(get_revoc_reg_delta_response, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Ledger(LedgerCommand::ParseGetRevocRegDeltaResponse(
            get_revoc_reg_delta_response,
            Box::new(move |result| {
                let (err, revoc_reg_def_id, revoc_reg_delta_json, timestamp) = result_to_err_code_3!(result, String::new(), String::new(), 0);
                let revoc_reg_def_id = CStringUtils::string_to_cstring(revoc_reg_def_id);
                let revoc_reg_delta_json = CStringUtils::string_to_cstring(revoc_reg_delta_json);
                cb(command_handle, err, revoc_reg_def_id.as_ptr(), revoc_reg_delta_json.as_ptr(), timestamp)
            })
        )));

    result_to_err_code!(result)
}