extern crate libc;

use self::libc::{c_char, c_uchar};

/// Sends transaction message to Sovrin network.
///
/// Adds issuer information to passed transaction json, signs it with issuer sign key
/// and sends signed transaction message to Sovring network.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// issuer: Id of Identity stored in secured Wallet.
/// tx_json: Transaction data json.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Transaction result as json.
///
/// #Errors
/// No method specific errors.
/// See `SovrinError` docs for common errors description.
#[no_mangle]
pub  extern fn sovrin_send_tx(client_id: i32, command_id: i32,
                              issuer: *const c_char, tx_json: *const c_char,
                              cb: extern fn(xcommand_id: i32, err: i32,
                                            tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates NYM transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_nym_tx(client_id: i32, command_id: i32,
                                 issuer: *const c_char,
                                 dest: *const c_char, verkey: *const c_char, xref: *const c_char,
                                 data: *const c_char, role: *const c_char,
                                 cb: extern fn(xcommand_id: i32, err: i32,
                                               tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates ATTRIB transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_attrib_tx(client_id: i32, command_id: i32,
                                    issuer: *const c_char, dest: *const c_char,
                                    hash: *const c_char, raw: *const c_char, enc: *const c_char,
                                    cb: extern fn(xcommand_id: i32, err: i32,
                                                  tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_ATTRIB transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_get_attrib_tx(client_id: i32, command_id: i32,
                                        issuer: *const c_char, dest: *const c_char,
                                        data: *const c_char,
                                        cb: extern fn(xcommand_id: i32, err: i32,
                                                      tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_NYM transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_get_nym_tx(client_id: i32, command_id: i32,
                                     issuer: *const c_char, dest: *const c_char,
                                     cb: extern fn(xcommand_id: i32, err: i32,
                                                   tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates SCHEMA transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_schema_tx(client_id: i32, command_id: i32,
                                    issuer: *const c_char, data: *const c_char,
                                    cb: extern fn(xcommand_id: i32, err: i32,
                                                  tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_SCHEMA transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_get_schema_tx(client_id: i32, command_id: i32,
                                        issuer: *const c_char, data: *const c_char,
                                        cb: extern fn(xcommand_id: i32, err: i32,
                                                      tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates ISSUER_KEY transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_issuer_key_tx(client_id: i32, command_id: i32,
                                        issuer: *const c_char, xref: *const c_char,
                                        data: *const c_char,
                                        cb: extern fn(xcommand_id: i32, err: i32,
                                                      tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates GET_ISSUER_KEY transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_get_issuer_key_tx(client_id: i32, command_id: i32,
                                            issuer: *const c_char, xref: *const c_char,
                                            cb: extern fn(xcommand_id: i32, err: i32,
                                                          tx_result_json: *const c_char)) {
    unimplemented!();
}

// Creates NODE transaction message and sends it to Sovrin network.
#[no_mangle]
pub extern fn sovrin_send_node_tx(client_id: i32, command_id: i32,
                                  issuer: *const c_char, dest: *const c_char, data: *const c_char,
                                  cb: extern fn(xcommand_id: i32, err: i32,
                                                tx_result_json: *const c_char)) {
    unimplemented!();
}