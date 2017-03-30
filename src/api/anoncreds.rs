extern crate libc;

use self::libc::{c_char, c_uchar};


#[no_mangle]
pub extern fn anoncreds_create_claim(client_id: i32, command_id: i32,
                                     schema_id: *const c_char, attributes: *const c_char,
                                     claim_request: *const c_char,
                                     cb: extern fn(xcommand_id: i32, err: i32, claim: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_claim_request(client_id: i32, command_id: i32,
                                             schema_id: *const c_char,
                                             request_non_revocation: bool,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           claim_req: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_proof(client_id: i32, command_id: i32,
                                     proof_input: *const c_char,
                                     nonce: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32,
                                                   proof: *const c_char, attrs: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_verify_proof(client_id: i32, command_id: i32,
                                     proof_input: *const c_char,
                                     proof: *const c_uchar,
                                     revealed_attributes: *const c_uchar,
                                     nonce: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}