extern crate libc;
extern crate log;

use self::libc::c_char;
use nullpay::ErrorCode;

pub fn set_default_indy_logger() {
    unsafe { indy_set_default_logger(::std::ptr::null()); }
}

extern {
    #[no_mangle]
    fn indy_set_default_logger(level: *const c_char) -> ErrorCode;
}