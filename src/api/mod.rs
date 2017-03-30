extern crate libc;

pub mod anoncreds;
pub mod sovrin;
pub mod wallet;

use self::libc::{c_char, c_uchar};

#[no_mangle]
pub extern fn init_client(host_and_port: *const c_char) -> i32 {
    unimplemented!();
}

#[no_mangle]
pub extern fn release_client(client_id: i32) -> i32 {
    unimplemented!();
}

#[no_mangle]
pub extern fn free_str(c_ptr: *mut c_char) {
    unimplemented!();
}