extern crate libc;

use api::ErrorCode;

use self::libc::c_char;

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
#[no_mangle]
pub extern fn sign_and_submit_request(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char, request_json: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 request_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
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
pub extern fn submit_request(command_handle: i32,
                          pool_handle: i32,
                          request_json: *const c_char,
                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                        request_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
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
pub extern fn build_get_ddo_request(command_handle: i32,
                                submitter_did: *const c_char, target_did: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                              request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}


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
#[no_mangle]
pub extern fn build_nym_request(command_handle: i32,
                            submitter_did: *const c_char,
                            target_did: *const c_char,
                            verkey: *const c_char, xref: *const c_char,
                            data: *const c_char, role: *const c_char,
                            cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                          request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
#[no_mangle]
pub extern fn build_attrib_request(command_handle: i32,
                               submitter_did: *const c_char, target_did: *const c_char,
                               hash: *const c_char, raw: *const c_char, enc: *const c_char,
                               cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                             request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
pub extern fn build_get_attrib_request(command_handle: i32,
                                   submitter_did: *const c_char, target_did: *const c_char,
                                   data: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
#[no_mangle]
pub extern fn build_get_nym_request(command_handle: i32,
                                submitter_did: *const c_char, target_did: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                              request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
#[no_mangle]
pub extern fn build_schema_request(command_handle: i32,
                               submitter_did: *const c_char, data: *const c_char,
                               cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                             request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
#[no_mangle]
pub extern fn build_get_schema_request(command_handle: i32,
                                   submitter_did: *const c_char, data: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds an ISSUER_KEY request.
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
#[no_mangle]
pub extern fn build_issuer_key_request(command_handle: i32,
                                   submitter_did: *const c_char, xref: *const c_char,
                                   data: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 request_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a GET_ISSUER_KEY request.
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
pub extern fn build_get_issuer_key_request(command_handle: i32,
                                       submitter_did: *const c_char, xref: *const c_char,
                                       cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

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
#[no_mangle]
pub extern fn build_node_request(command_handle: i32,
                             submitter_did: *const c_char, target_did: *const c_char,
                             data: *const c_char,
                             cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                           request_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}