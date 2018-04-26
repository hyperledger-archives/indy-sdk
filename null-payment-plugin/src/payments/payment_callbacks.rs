extern crate libc;

use indy::api::payments::indy_register_payment_method;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::ATOMIC_USIZE_INIT;
use std::ffi::CString;
use std::os::raw::c_char;


type CommonResponseCallback = extern fn(command_handle_: i32,
                                        err: i32,
                                        res1: *const c_char) -> i32;

pub extern fn init() {
    let _cmd_handle = get_next_id();
    let payment_method_name = CString::new("null_payment_plugin").unwrap();
    indy_register_payment_method(
        _cmd_handle,
        payment_method_name.as_ptr(),
        Some(create_payment_address),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    );
}

extern fn create_payment_address(
    cmd_handle:i32,
    config: *const c_char,
    cb: Option<extern fn(command_handle_: i32,
                         err: i32,
                         payment_address: *const c_char) -> i32>
) -> i32 {
    let addr = CString::new("pay:null_payment_plugin:null").unwrap();
    cb.unwrap()(cmd_handle, 0, addr.as_ptr())
}

extern fn

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT; //TODO use AtomicI32
}

fn get_next_id() -> i32 {
    (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}