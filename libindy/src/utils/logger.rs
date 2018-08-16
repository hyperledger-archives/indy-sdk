extern crate env_logger;
extern crate log_panics;
extern crate log;
#[cfg(target_os = "android")]
extern crate android_logger;
extern crate libc;

use self::env_logger::Builder;
use self::log::LevelFilter;
use std::env;
use std::io::Write;
#[cfg(target_os = "android")]
use self::android_logger::Filter;
use log::{Record, Metadata};

use self::libc::{c_void, c_char};
use std::ffi::CString;
use std::ptr;

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

struct IndyLogger {
    context: *const c_void,
    #[allow(dead_code)]
    enabled_cb: Option<EnabledCB>,
    log_cb: LogCB,
    flush_cb: Option<FlushCB>,
}

impl IndyLogger {
    fn new(context: *const c_void, log_cb: LogCB, flush_cb: Option<FlushCB>) -> Self {
        IndyLogger {
            context,
            enabled_cb: None,
            log_cb,
            flush_cb,
        }
    }
}

impl log::Log for IndyLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let log_cb = self.log_cb;

        let level = record.level() as u32;
        let target = CString::new(record.target()).unwrap();
        let args = CString::new(record.args().to_string()).unwrap();

        let module_path = record.module_path().map(|a| CString::new(a).unwrap());
        let file = record.file().map(|a| CString::new(a).unwrap());
        let line = record.line().unwrap_or(0);

        log_cb(self.context,
               level,
               target.as_ptr(),
               args.as_ptr(),
               module_path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               file.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               line,
        )
    }

    fn flush(&self) {
        if let Some(flush_cb) = self.flush_cb {
            flush_cb(self.context)
        }
    }
}

unsafe impl Sync for IndyLogger {}

unsafe impl Send for IndyLogger {}

pub fn init_indy_logger(context: *const c_void, log_cb: LogCB, flush_cb: Option<FlushCB>) -> Result<(), log::SetLoggerError> {
    let logger = IndyLogger::new(context, log_cb, flush_cb);

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Trace);
    Ok(())
}

pub fn init_default_logger(level: Option<String>) -> Result<(), log::SetLoggerError> {
    let level = level.or(env::var("RUST_LOG").ok());

    log_panics::init(); //Logging of panics is essential for android. As android does not log to stdout for native code

    if cfg!(target_os = "android") {
        #[cfg(target_os = "android")]
        let log_filter = match level {
            Some(val) => match val.to_lowercase().as_ref() {
                "error" => Filter::default().with_min_level(log::Level::Error),
                "warn" => Filter::default().with_min_level(log::Level::Warn),
                "info" => Filter::default().with_min_level(log::Level::Info),
                "debug" => Filter::default().with_min_level(log::Level::Debug),
                "trace" => Filter::default().with_min_level(log::Level::Trace),
                _ => Filter::default().with_min_level(log::Level::Error),
            },
            None => Filter::default().with_min_level(log::Level::Error)
        };

        //Set logging to off when deploying production android app.
        #[cfg(target_os = "android")]
        android_logger::init_once(log_filter);
        info!("Logging for Android");
        Ok(())
    } else {
        Builder::new()
            .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
            .filter(None, LevelFilter::Off)
            .parse(level.as_ref().map(String::as_str).unwrap_or(""))
            .try_init()
    }
}

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
    () => ( _map_err!(::log::Level::Error) );
    ($($arg:tt)*) => ( _map_err!(::log::Level::Error, $($arg)*) )
}

#[macro_export]
macro_rules! map_err_trace {
    () => ( _map_err!(::log::Level::Trace) );
    ($($arg:tt)*) => ( _map_err!(::log::Level::Trace, $($arg)*) )
}

#[macro_export]
macro_rules! map_err_info {
    () => ( _map_err!(::log::Level::Info) );
    ($($arg:tt)*) => ( _map_err!(::log::Level::Info, $($arg)*) )
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ $val }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ "_" }};
}