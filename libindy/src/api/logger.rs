extern crate libc;

use self::libc::{c_void, c_char};
use std::ptr;

use api::ErrorCode;

extern crate time;
extern crate log;

use log::{Record, Metadata};
use self::log::LevelFilter;

use std::ffi::CString;

pub struct IndyLogger {
    context: Context,
    pub enabled_cb: EnabledCB,
    pub log_cb: LogCB,
    pub flush_cb: FlushCB,
}

#[repr(C)]
struct Context(*const c_void);
unsafe impl Sync for Context {}
unsafe impl Send for Context {}

impl log::Log for IndyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let enabled_cb = self.enabled_cb;

        let level = CString::new(metadata.level().to_string()).unwrap();
        let target = CString::new(metadata.target()).unwrap();

        enabled_cb(self.context.0, level.as_ptr(), target.as_ptr())
    }

    fn log(&self, record: &Record) {
        let log_cb = self.log_cb;

        let level = CString::new(record.level().to_string()).unwrap();
        let target = CString::new(record.target()).unwrap();
        let args = CString::new(record.args().to_string()).unwrap();

        let module_path = record.module_path().map(|a| CString::new(a).unwrap());
        let file = record.file().map(|a| CString::new(a).unwrap());
        let line = record.line().map(|line| line as i32).unwrap_or(-1);

        log_cb(self.context.0,
               level.as_ptr(),
               target.as_ptr(),
               args.as_ptr(),
               module_path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               file.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               line,
        )
    }

    fn flush(&self) {
        let flush_cb = self.flush_cb;
        flush_cb(self.context.0)
    }
}

pub type EnabledCB = extern fn(context: *const c_void,
                               level: *const c_char,
                               target: *const c_char) -> bool;

pub type LogCB = extern fn(context: *const c_void,
                           level: *const c_char,
                           target: *const c_char,
                           args: *const c_char,
                           module_path: *const c_char,
                           file: *const c_char,
                           line: i32);

pub type FlushCB = extern fn(context: *const c_void);

#[no_mangle]
pub extern fn indy_init_logger(context: *const c_void,
                               enabled_cb: Option<EnabledCB>,
                               log_cb: Option<LogCB>,
                               flush_cb: Option<FlushCB>) -> ErrorCode {
    trace!("indy_init_logger >>>");

    check_useful_c_callback!(enabled_cb, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(log_cb, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(flush_cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_init_logger: entities >>>");

    let logger = IndyLogger {
        context: Context(context),
        enabled_cb,
        log_cb,
        flush_cb,
    };

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Trace);

    let res = ErrorCode::Success;

    trace!("indy_init_logger: <<< res: {:?}", res);

    res
}