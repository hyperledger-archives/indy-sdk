extern crate log;

use self::log::LevelFilter;
use libindy;
use libindy::logger::{EnabledCB, LogCB, FlushCB};

use std::ffi::CString;
use std::ptr;
use log::{Record, Metadata};
use libc::c_void;
use ErrorCode;

pub struct LibnullpayLogger {
    context: *const c_void,
    enabled: Option<EnabledCB>,
    log: LogCB,
    flush: Option<FlushCB>,
}

impl LibnullpayLogger {
    fn new(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Self {
        LibnullpayLogger { context, enabled, log, flush }
    }

    pub fn init() -> Result<(), ErrorCode> {
        // logging, as implemented, crashes with VCX for android and ios, so
        // for this hotfix (IS-1164) simply return OK
        if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            return Ok(())
        }

        let (context, enabled, log, flush) = libindy::logger::get_logger()?;

        let log = match log {
            Some(log) => log,
            None => return Err(ErrorCode::CommonInvalidState)
        };

        let logger = LibnullpayLogger::new(context, enabled, log, flush);

        log::set_boxed_logger(Box::new(logger)).ok();
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}

impl log::Log for LibnullpayLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(enabled_cb) = self.enabled {
            let level = metadata.level() as u32;
            let target = CString::new(metadata.target()).unwrap();

            enabled_cb(self.context,
                       level,
                       target.as_ptr(),
            )
        } else { true }
    }

    fn log(&self, record: &Record) {
        let log_cb = self.log;

        let level = record.level() as u32;
        let target = CString::new(record.target()).unwrap();
        let message = CString::new(record.args().to_string()).unwrap();

        let module_path = record.module_path().map(|a| CString::new(a).unwrap());
        let file = record.file().map(|a| CString::new(a).unwrap());
        let line = record.line().unwrap_or(0);

        log_cb(self.context,
               level,
               target.as_ptr(),
               message.as_ptr(),
               module_path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               file.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               line,
        )
    }

    fn flush(&self) {
        if let Some(flush_cb) = self.flush {
            flush_cb(self.context)
        }
    }
}

unsafe impl Sync for LibnullpayLogger {}

unsafe impl Send for LibnullpayLogger {}

#[macro_export]
macro_rules! try_log {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            error!("try_log! | {}", err);
            return Err(From::from(err))
        }
    })
}

macro_rules! _map_err {
    ($lvl:expr, $expr:expr) => (
        |err| {
            log!($lvl, "{} - {}", $expr, err);
            err
        }
    );
    ($lvl:expr) => (
        |err| {
            log!($lvl, "{}", err);
            err
        }
    )
}

#[macro_export]
macro_rules! map_err_err {
    () => ( _map_err!(::log::LogLevel::Error) );
    ($($arg:tt)*) => ( _map_err!(::log::LogLevel::Error, $($arg)*) )
}

#[macro_export]
macro_rules! map_err_trace {
    () => ( _map_err!(::log::LogLevel::Trace) );
    ($($arg:tt)*) => ( _map_err!(::log::LogLevel::Trace, $($arg)*) )
}