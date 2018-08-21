extern crate log;

use ErrorCode;

use libc::c_void;
use utils::logger::{EnabledCB, LogCB, FlushCB};

pub fn get_indy_logger() -> Result<(*const c_void, Option<EnabledCB>, Option<LogCB>, Option<FlushCB>), ErrorCode> {
    let mut context_p: *const c_void = ::std::ptr::null();
    let mut enabled_cb_p: Option<EnabledCB> = None;
    let mut log_cb_p: Option<LogCB> = None;
    let mut flush_cb_p: Option<FlushCB> = None;

    let res = unsafe {
        indy_get_logger(&mut context_p, &mut enabled_cb_p, &mut log_cb_p, &mut flush_cb_p)
    };

    match res {
        ErrorCode::Success => {
            Ok((context_p, enabled_cb_p, log_cb_p, flush_cb_p))
        }
        err @ _ => Err(err)
    }
}

extern {
    #[no_mangle]
    pub fn indy_get_logger(context: *mut *const c_void, enabled_cb_p: *mut Option<EnabledCB>, log_cb_p: *mut Option<LogCB>, flush_cb_p: *mut Option<FlushCB>) -> ErrorCode;
}