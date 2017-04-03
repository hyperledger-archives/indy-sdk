extern crate libc;

use self::libc::{c_char, c_uchar};

/// Signs and sends transaction message to validator pool.
///
/// Adds submitter information to passed transaction json, signs it with submitter
/// sign key (see wallet_sign_by_my_did), and sends signed transaction message
/// to validator pool (see ledger_write_txn).
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
pub extern fn ledger_sign_and_send_txn(client_handle: i32, command_handle: i32,
                                       submitter_did: *const c_char, txn_json: *const c_char,
                                       cb: extern fn(xcommand_handle: i32, err: i32,
                                                     txn_result_json: *const c_char)) {
    unimplemented!();
}

/// Sends transaction message to validator pool (no signing, unlike ledger_sign_and_write_txn).
///
/// The transaction is sent to the validator pool as is. It's assumed that it's already prepared.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
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
pub extern fn ledger_send_txn(client_handle: i32, command_handle: i32,
                              txn_json: *const c_char,
                              cb: extern fn(xcommand_handle: i32, err: i32,
                                            txn_result_json: *const c_char)) {
    unimplemented!();
}


/// Creates and optionally signs a txn to get a DDO.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_get_ddo_txn(client_handle: i32, command_handle: i32, sign: i32,
                                 submitter_did: *const c_char, target_did: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: i32,
                                               txn_json: *const c_char)) {
    unimplemented!();
}



/// Creates and optionally signs NYM transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// verkey:
/// xref:
/// data:
/// role:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_nym_txn(client_handle: i32, command_handle: i32, sign: i32,
                             submitter_did: *const c_char,
                             target_did: *const c_char,
                             verkey: *const c_char, xref: *const c_char,
                             data: *const c_char, role: *const c_char,
                             cb: extern fn(xcommand_handle: i32, err: i32,
                                           txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs ATTRIB transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// hash:
/// raw:
/// enc:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_attrib_txn(client_handle: i32, command_handle: i32, sign: i32,
                                submitter_did: *const c_char, target_did: *const c_char,
                                hash: *const c_char, raw: *const c_char, enc: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: i32,
                                              txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs GET_ATTRIB transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// data:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
pub extern fn create_get_attrib_txn(client_handle: i32, command_handle: i32, sign: i32,
                                    submitter_did: *const c_char, target_did: *const c_char,
                                    data: *const c_char,
                                    cb: extern fn(xcommand_handle: i32, err: i32,
                                                  txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs GET_NYM transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_get_nym_txn(client_handle: i32, command_handle: i32, sign: i32,
                                 submitter_did: *const c_char, target_did: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: i32,
                                               txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs SCHEMA transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// data:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_schema_txn(client_handle: i32, command_handle: i32, sign: i32,
                                submitter_did: *const c_char, data: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: i32,
                                              txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs GET_SCHEMA transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// data:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_get_schema_txn(client_handle: i32, command_handle: i32, sign: i32,
                                    submitter_did: *const c_char, data: *const c_char,
                                    cb: extern fn(xcommand_handle: i32, err: i32,
                                                  txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs ISSUER_KEY transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// xref:
/// data:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_issuer_key_txn(client_handle: i32, command_handle: i32, sign: i32,
                                    submitter_did: *const c_char, xref: *const c_char,
                                    data: *const c_char,
                                    cb: extern fn(xcommand_handle: i32, err: i32,
                                                  txn_result_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs GET_ISSUER_KEY transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// xref:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
pub extern fn crate_get_issuer_key_txn(client_handle: i32, command_handle: i32, sign: i32,
                                       submitter_did: *const c_char, xref: *const c_char,
                                       cb: extern fn(xcommand_handle: i32, err: i32,
                                                     txn_json: *const c_char)) {
    unimplemented!();
}

/// Creates and optionally signs NODE transaction.
///
/// #Params
/// client_handle: id of Ledger client instance.
/// command_handle: command id to map of callback to user context.
/// sign:
/// submitter_did: Id of Identity stored in secured Wallet.
/// target_did: Id of Identity stored in secured Wallet.
/// data:
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `LedgerError` docs for common errors description.
#[no_mangle]
pub extern fn create_node_txn(client_handle: i32, command_handle: i32, sign: i32,
                              submitter_did: *const c_char, target_did: *const c_char,
                              data: *const c_char,
                              cb: extern fn(xcommand_handle: i32, err: i32,
                                            txn_json: *const c_char)) {
    unimplemented!();
}