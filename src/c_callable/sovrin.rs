extern crate libc;

use self::libc::{c_char, c_uchar};

/// Creates json for generic Ledger transaction.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// issuer: Id of Identity stored in secured Wallet.
/// tx_json: Transaction data json.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Signed transaction json.
///
/// #Errors
/// No method specific errors.
/// See `SovrinError` docs for common errors description.
#[no_mangle]
pub  extern fn sovrin_create_tx(client_id: i32, command_id: i32,
                                issuer: *const c_char, tx_json: *const c_char,
                                cb: extern fn(xcommand_id: i32, err: i32,
                                              signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_nym_tx(client_id: i32, command_id: i32,
                                   issuer: *const c_char,
                                   dest: *const c_char, verkey: *const c_char, xref: *const c_char,
                                   data: *const c_char, role: *const c_char,
                                   cb: extern fn(xcommand_id: i32, err: i32,
                                                 signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_attrib_tx(client_id: i32, command_id: i32,
                                      issuer: *const c_char, dest: *const c_char,
                                      hash: *const c_char, raw: *const c_char, enc: *const c_char,
                                      cb: extern fn(xcommand_id: i32, err: i32,
                                                    signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_get_attrib_tx(client_id: i32, command_id: i32,
                                          issuer: *const c_char, dest: *const c_char,
                                          data: *const c_char,
                                          cb: extern fn(xcommand_id: i32, err: i32,
                                                        signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_get_nym_tx(client_id: i32, command_id: i32,
                                       issuer: *const c_char, dest: *const c_char,
                                       cb: extern fn(xcommand_id: i32, err: i32,
                                                     signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_schema_tx(client_id: i32, command_id: i32,
                                      issuer: *const c_char, data: *const c_char,
                                      cb: extern fn(xcommand_id: i32, err: i32,
                                                    signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_get_schema_tx(client_id: i32, command_id: i32,
                                          issuer: *const c_char, data: *const c_char,
                                          cb: extern fn(xcommand_id: i32, err: i32,
                                                        signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_issuer_key_tx(client_id: i32, command_id: i32,
                                          issuer: *const c_char, xref: *const c_char,
                                          data: *const c_char,
                                          cb: extern fn(xcommand_id: i32, err: i32,
                                                        signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_get_issuer_key_tx(client_id: i32, command_id: i32,
                                              issuer: *const c_char, xref: *const c_char,
                                              cb: extern fn(xcommand_id: i32, err: i32,
                                                            signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_create_node_tx(client_id: i32, command_id: i32,
                                    issuer: *const c_char, dest: *const c_char, data: *const c_char,
                                    cb: extern fn(xcommand_id: i32, err: i32,
                                                  signed_tx_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_identity_tx(client_id: i32, command_id: i32,
                                      signed_tx_json: *const c_char,
                                      cb: extern fn(xcommand_id: i32, err: i32,
                                                    tx_result_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_pool_tx(client_id: i32, command_id: i32,
                                  signed_tx_json: *const c_char,
                                  cb: extern fn(xcommand_id: i32, err: i32,
                                                tx_result_json: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_configuration_tx(client_id: i32, command_id: i32,
                                           signed_tx_json: *const c_char,
                                           cb: extern fn(xcommand_id: i32, err: i32,
                                                         tx_result_json: *const c_char)) {
    unimplemented!();
}