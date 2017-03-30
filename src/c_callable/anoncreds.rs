extern crate libc;

use self::libc::{c_char, c_uchar};

#[no_mangle]
pub extern fn anoncreds_create_master_secret(client_id: i32, command_id: i32,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           master_secret: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_key_pair(client_id: i32, command_id: i32, schema: *const c_uchar,
                                        cb: extern fn(xcommand_id: i32, err: i32,
                                                      pk: *const c_char, sk: *const c_char,
                                                      pnrk: *const c_char, snrk: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_issue_accumulator(client_id: i32, command_id: i32, schema: *const c_uchar,
                                          accumulator_id: *const c_uchar,
                                          max_claims: *const c_uchar,
                                          public_key_non_revocation: *const c_uchar,
                                          cb: extern fn(xcommand_id: i32, err: i32,
                                                        acc: *const c_char, tails: *const c_char,
                                                        acc_pk: *const c_char, acc_sk: *const c_char
                                          )) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_issue_claim(client_id: i32, command_id: i32, attributes: *const c_uchar,
                                    accumulator: *const c_uchar, sequence_number: *const c_uchar,
                                    claim_request: *const c_uchar, public_key: *const c_uchar,
                                    secret_key: *const c_uchar,
                                    public_key_non_revocation: *const c_uchar,
                                    secret_key_non_revocation: *const c_uchar,
                                    tails: *const c_uchar, secret_key_accumulator: *const c_uchar,
                                    cb: extern fn(xcommand_id: i32, err: i32, claim: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_claim_request(client_id: i32, command_id: i32, master_secret: *const c_uchar,
                                             public_key: *const c_uchar,
                                             public_key_non_revocation: *const c_uchar,
                                             request_non_revocation: *const c_uchar,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           claim_req: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_proof(proof_input: *const c_uchar, nonce: *const c_uchar,
                                     claims: *const c_uchar,
                                     public_key_non_revocation: *const c_uchar,
                                     accumulator: *const c_uchar, public_key: *const c_uchar,
                                     master_secret: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32,
                                                   proof: *const c_char, attrs: *const c_char)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_verify_proof(proof_input: *const c_uchar, proof: *const c_uchar,
                                     revealed_attributes: *const c_uchar, nonce: *const c_uchar,
                                     public_key_non_revocation: *const c_uchar,
                                     accumulator: *const c_uchar,
                                     public_key_accumulator: *const c_uchar,
                                     public_key: *const c_uchar, attributes: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}

#[no_mangle]
pub extern fn anoncreds_create_nonce(cb: extern fn(client_id: i32, command_id: i32, err: i32, nonce: *const c_char)) {
    unimplemented!();
}