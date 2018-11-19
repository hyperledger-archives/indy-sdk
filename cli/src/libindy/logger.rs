extern crate log;
extern crate libc;

use super::ErrorCode;

use self::libc::{c_void, c_char};
use std::ptr::null;

pub type EnabledCB = extern fn(context: *const c_void,
                               level: u32,
                               target: *const c_char) -> bool;

pub type LogCB = extern fn(context: *const c_void,
                           level: u32,
                           target: *const c_char,
                           message: *const c_char,
                           module_path: *const c_char,
                           file: *const c_char,
                           line: u32);

pub type FlushCB = extern fn(context: *const c_void);

pub fn set_indy_logger(enabled: Option<EnabledCB>, log: Option<LogCB>, flush: Option<FlushCB>) -> Result<(), ErrorCode> {
    let res = unsafe {
        indy_set_logger(
            null(),
            enabled,
            log,
            flush,
        )
    };

    match res {
        ErrorCode::Success => Ok(()),
        err => Err(err)
    }
}

extern {
    #[no_mangle]
    pub fn indy_set_logger(context: *const c_void,
                           enabled: Option<EnabledCB>,
                           log: Option<LogCB>,
                           flush: Option<FlushCB>) -> ErrorCode;
}