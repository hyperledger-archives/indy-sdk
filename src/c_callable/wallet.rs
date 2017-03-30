extern crate libc;

use self::libc::c_char;

pub extern fn wallet_set(client_id: i32, command_id: i32, collection: *const c_char,
                         key: *const c_char, sub_key: *const c_char, value: *const c_char,
                         cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}

pub extern fn wallet_get(client_id: i32, command_id: i32, collection: *const c_char,
                         key: *const c_char, sub_key: *const c_char,
                         cb: extern fn(xcommand_id: i32, err: i32, result: *const c_char)) {
    unimplemented!();
}