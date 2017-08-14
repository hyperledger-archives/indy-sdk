extern crate libc;

use std::mem;

macro_rules! get_byte_array {
    ($x:ident, $l:expr) => {
        let $x =  unsafe { Vec::from_raw_parts($x as *mut u8, $l as usize, $l as usize)};
    }
}

pub fn vec_to_pointer(v: Vec<u8>) -> (*const libc::c_void, usize) {
    let len = v.len();
    let res = (v.as_ptr() as *const libc::c_void, len);
    mem::forget(v);
    res
}