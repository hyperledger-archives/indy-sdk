extern crate log;

use ErrorCode;

use libc::c_void;
use std::ptr;

pub fn get_indy_logger() -> Result<&'static log::Log, ErrorCode> {
    let mut logger_p: *const c_void = ptr::null();

    let res = unsafe {
        indy_get_logger(&mut logger_p)
    };

    match res {
        ErrorCode::Success => {
            let logger = unsafe { *(logger_p as *const &'static log::Log) };
            Ok(logger)
        }
        err @ _ => Err(err)
    }
}

extern {
    #[no_mangle]
    pub fn indy_get_logger(logger_p: *mut *const c_void) -> ErrorCode;
}