extern crate libc;

use self::libc::{c_char, c_uchar};

#[no_mangle]
pub extern fn ledger_check_verkey(client_handle: i32, command_handle: i32,
                                   did: *const c_char, verkey: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: i32,
                                                 xverkey: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn ledger_read_ddo(client_handle: i32, command_handle: i32,
                               submitter_did: *const c_char, target_did: *const c_char,
                               cb: extern fn(xcommand_handle: i32, err: i32,
                                             txn_result_json: *const c_char)) {
    unimplemented!();
}

/// Sends transaction message to validator pool.
///
/// Adds issuer information to passed transaction json, signs it with issuer sign key
/// and sends signed transaction message to validator pool.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// submitter_did: Id of Identity stored in secured Wallet.
/// txn_json: Transaction data json.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn ledger_write_txn(client_handle: i32, command_handle: i32,
                                submitter_did: *const c_char, txn_json: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: i32,
                                              txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates NYM transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_write_nym_txn(client_handle: i32, command_handle: i32,
                                   submitter_did: *const c_char,
                                   target_did: *const c_char,
                                   verkey: *const c_char, xref: *const c_char,
                                   data: *const c_char, role: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: i32,
                                                 txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates ATTRIB transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_write_attrib_txn(client_handle: i32, command_handle: i32,
                                      submitter_did: *const c_char, target_did: *const c_char,
                                      hash: *const c_char, raw: *const c_char, enc: *const c_char,
                                      cb: extern fn(xcommand_handle: i32, err: i32,
                                                    txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_ATTRIB transaction message and sends it to validator pool.
pub extern fn ledger_read_attrib(client_handle: i32, command_handle: i32,
                                 submitter_did: *const c_char, target_did: *const c_char,
                                 data: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: i32,
                                               txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_NYM transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_read_nym(client_handle: i32, command_handle: i32,
                              submitter_did: *const c_char, target_did: *const c_char,
                              cb: extern fn(xcommand_handle: i32, err: i32,
                                            txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates SCHEMA transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_write_schema_txn(client_handle: i32, command_handle: i32,
                                      submitter_did: *const c_char, data: *const c_char,
                                      cb: extern fn(xcommand_handle: i32, err: i32,
                                                    txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_SCHEMA transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_read_schema(client_handle: i32, command_handle: i32,
                                 submitter_did: *const c_char, data: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: i32,
                                               txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates ISSUER_KEY transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_write_issuer_key_txn(client_handle: i32, command_handle: i32,
                                          submitter_did: *const c_char, xref: *const c_char,
                                          data: *const c_char,
                                          cb: extern fn(xcommand_handle: i32, err: i32,
                                                        txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_ISSUER_KEY transaction message and sends it to validator pool.
pub extern fn ledger_read_issuer_key(client_handle: i32, command_handle: i32,
                                     submitter_did: *const c_char, xref: *const c_char,
                                     cb: extern fn(xcommand_handle: i32, err: i32,
                                                   txn_result_json: *const c_char)) {
    unimplemented!();
}

// Creates NODE transaction message and sends it to validator pool.
#[no_mangle]
pub extern fn ledger_write_node_txn(client_handle: i32, command_handle: i32,
                                    submitter_did: *const c_char, target_did: *const c_char,
                                    data: *const c_char,
                                    cb: extern fn(xcommand_handle: i32, err: i32,
                                                  txn_result_json: *const c_char)) {
    unimplemented!();
}