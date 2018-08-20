extern crate libc;

use self::libc::c_char;

use nullpay::ErrorCode;

extern crate log;

use std::ffi::CString;

pub fn set_default_indy_logger() {
    let level = CString::new("TRACE").unwrap();
    unsafe { indy_set_default_logger(level.as_ptr()); }
}

extern {
    #[no_mangle]
    fn indy_set_default_logger(level: *const c_char) -> ErrorCode;
}