extern crate libc;

use self::libc::{c_char, c_uchar};

#[no_mangle]
pub  extern fn sovrin_send_nym_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                  dest: *const c_char, verkey: *const c_char, xref: *const c_char,
                                  data: *const c_char, role: *const c_char,
                                  cb: extern fn(xcommand_id: i32, err: i32)) {}

#[no_mangle]
pub extern fn sovrin_send_attrib_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                    dest: *const c_char, hash: *const c_char, raw: *const c_char,
                                    enc: *const c_char,
                                    cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_get_att_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                     dest: *const c_char, data: *const c_char,
                                     cb: extern fn(xcommand_id: i32, err: i32, result: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_get_nym_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                     dest: *const c_char,
                                     cb: extern fn(xcommand_id: i32, err: i32, result: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_schema_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                    data: *const c_char,
                                    cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_get_schema_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                        data: *const c_char,
                                        cb: extern fn(xcommand_id: i32, err: i32, result: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_issuer_key_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                        xref: *const c_char, data: *const c_char,
                                        cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_get_issuer_key_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                            xref: *const c_char,
                                            cb: extern fn(xcommand_id: i32, err: i32, result: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn sovrin_send_node_tx(client_id: i32, command_id: i32, issuer: *const c_char,
                                  dest: *const c_char, data: *const c_char,
                                  cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}