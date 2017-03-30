extern crate libc;

use self::libc::{c_char, c_uchar};

pub extern fn crypto_sodium_create_key_pair(seed: *const c_char,
                                            cb: extern fn(xcommand_id: i32, err: i32, pub_key: *const c_uchar, priv_key: *const c_uchar)) {
    unimplemented!();
}

pub extern fn crypto_sodium_encrypt(public_key: *const c_uchar, doc: *const c_uchar,
                                    cb: extern fn(xcommand_id: i32, err: i32, result: *const c_uchar)) {
    unimplemented!();
}

pub extern fn crypto_sodium_decrypt(private_key: *const c_uchar, doc: *const c_uchar,
                                    cb: extern fn(xcommand_id: i32, err: i32, result: *const c_uchar)) {
    unimplemented!();
}

pub extern fn crypto_sodium_sign(private_key: *const c_uchar, doc: *const c_uchar,
                                 cb: extern fn(xcommand_id: i32, err: i32, result: *const c_uchar)) {
    unimplemented!();
}

pub extern fn crypto_sodium_verify(public_key: *const c_uchar, doc: *const c_uchar,
                                   cb: extern fn(xcommand_id: i32, err: i32, result: *const c_uchar)) {
    unimplemented!();
}

pub extern fn crypto_base58_encode(src: *const c_uchar,
                                   cb: extern fn(xcommand_id: i32, err: i32, result: *const c_cchar)) {
    unimplemented!();
}

pub extern fn crypto_base58_decode(str: *const c_char,
                                   cb: extern fn(xcommand_id: i32, err: i32, result: *const c_uchar)) {
    unimplemented!();
}