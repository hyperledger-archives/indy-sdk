extern crate indy;

use utils::payment_callbacks::PaymentCallbacks;
use std::ffi::CString;
use indy::api::ErrorCode;
use std::os::raw::c_char;

#[macro_use]
mod utils;

#[test]
fn test_cbs() {
    let cb_creator = PaymentCallbacks::new();
    let (handle, cb) = cb_creator.get_create_payment_address_cb();
    cb(1, CString::new("aaa")?.as_ptr(), Some(test));
    let cmd_handle: Option<i32> = cb_creator.get(format!("{}_cmd_handle", handle));
    let cfg: Option<*const char> = cb_creator.get(format!("{}_config", handle));
}

extern "C" fn test(command_handle: i32,
                             err: ErrorCode,
                             c_str: *const c_char) -> ErrorCode {
    unimplemented!();
}