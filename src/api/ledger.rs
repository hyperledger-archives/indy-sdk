extern crate libc;

use api::ErrorCode;

use self::libc::{c_char, c_uchar};

/// Signs and publishes transaction message to validator pool.
///
/// Adds submitter information to passed transaction json, signs it with submitter
/// sign key (see wallet_sign), and sends signed transaction message
/// to validator pool (see write_txn).
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// txn_json: Transaction data json.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub extern fn sign_and_publish_txn(session_handle: i32, command_handle: i32,
                                          submitter_did: *const c_char, txn_json: *const c_char,
                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        txn_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Publishes transaction message to validator pool (no signing, unlike sign_and_publish_txn).
///
/// The transaction is sent to the validator pool as is. It's assumed that it's already prepared.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// txn_json: Transaction data json.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn publish_txn(session_handle: i32, command_handle: i32,
                                 txn_json: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                               txn_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}


/// Builds a txn to get a DDO.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_get_ddo_txn(session_handle: i32, command_handle: i32,
                                       submitter_did: *const c_char, target_did: *const c_char,
                                       cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}


/// Builds a NYM transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// verkey: verification key
/// xref: id of a NYM record
/// data: alias
/// role: Role of a user NYM record
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_nym_txn(session_handle: i32, command_handle: i32,
                                   submitter_did: *const c_char,
                                   target_did: *const c_char,
                                   verkey: *const c_char, xref: *const c_char,
                                   data: *const c_char, role: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds an ATTRIB transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// hash: Hash of attribute data
/// raw: represented as json, where key is attribute name and value is it's value
/// enc: Encrypted attribute data
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_attrib_txn(session_handle: i32, command_handle: i32,
                                      submitter_did: *const c_char, target_did: *const c_char,
                                      hash: *const c_char, raw: *const c_char, enc: *const c_char,
                                      cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a GET_ATTRIB transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// data: name (attribute name)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
pub extern fn build_get_attrib_txn(session_handle: i32, command_handle: i32,
                                          submitter_did: *const c_char, target_did: *const c_char,
                                          data: *const c_char,
                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a GET_NYM transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_get_nym_txn(session_handle: i32, command_handle: i32,
                                       submitter_did: *const c_char, target_did: *const c_char,
                                       cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a SCHEMA transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// data: name, version, type, attr_names (ip, port, keys)
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_schema_txn(session_handle: i32, command_handle: i32,
                                      submitter_did: *const c_char, data: *const c_char,
                                      cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a GET_SCHEMA transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// data: name, version
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_get_schema_txn(session_handle: i32, command_handle: i32,
                                          submitter_did: *const c_char, data: *const c_char,
                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds an ISSUER_KEY transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// xref: Seq. number of schema
/// data: components of a key in json: N, R, S, Z
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_issuer_key_txn(session_handle: i32, command_handle: i32,
                                          submitter_did: *const c_char, xref: *const c_char,
                                          data: *const c_char,
                                          cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        txn_result_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a GET_ISSUER_KEY transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// xref: Seq. number of schema
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
pub extern fn build_get_issuer_key_txn(session_handle: i32, command_handle: i32,
                                              submitter_did: *const c_char, xref: *const c_char,
                                              cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                            txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Builds a NODE transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// data: id of a target NYM record
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn build_node_txn(session_handle: i32, command_handle: i32,
                                    submitter_did: *const c_char, target_did: *const c_char,
                                    data: *const c_char,
                                    cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                  txn_json: *const c_char)) -> ErrorCode {
    unimplemented!();
}