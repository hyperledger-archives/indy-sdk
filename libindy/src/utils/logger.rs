extern crate env_logger;
extern crate log_panics;
extern crate log;
#[cfg(target_os = "android")]
extern crate android_logger;

use self::env_logger::Builder as EnvLoggerBuilder;
use self::log::{LevelFilter, Level};
use std::env;
use std::io::Write;
#[cfg(target_os = "android")]
use self::android_logger::Filter;
use log::{Record, Metadata};

use libc::{c_void, c_char};
use std::ffi::CString;
use std::ptr;

use errors::prelude::*;
use utils::ctypes;

pub static mut LOGGER_STATE: LoggerState = LoggerState::Default;

pub enum LoggerState {
    Default,
    Custom
}

impl LoggerState {
    pub fn get(&self) -> (*const c_void, Option<EnabledCB>, Option<LogCB>, Option<FlushCB>) {
        match self {
            LoggerState::Default => (ptr::null(), Some(LibindyDefaultLogger::enabled), Some(LibindyDefaultLogger::log), Some(LibindyDefaultLogger::flush)),
            LoggerState::Custom => unsafe { (CONTEXT, ENABLED_CB, LOG_CB, FLUSH_CB) },
        }
    }
}


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

static mut CONTEXT: *const c_void = ptr::null();
static mut ENABLED_CB: Option<EnabledCB> = None;
static mut LOG_CB: Option<LogCB> = None;
static mut FLUSH_CB: Option<FlushCB> = None;

pub struct LibindyLogger {
    context: *const c_void,
    enabled: Option<EnabledCB>,
    log: LogCB,
    flush: Option<FlushCB>,
}

impl LibindyLogger {
    fn new(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Self {
        LibindyLogger { context, enabled, log, flush }
    }
}

impl log::Log for LibindyLogger {
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

unsafe impl Sync for LibindyLogger {}

unsafe impl Send for LibindyLogger {}

impl LibindyLogger {
    pub fn init(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Result<(), IndyError> {
        let logger = LibindyLogger::new(context, enabled, log, flush);

        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(LevelFilter::Trace);

        unsafe {
            LOGGER_STATE = LoggerState::Custom;
            CONTEXT = context;
            ENABLED_CB = enabled;
            LOG_CB = Some(log);
            FLUSH_CB = flush
        };

        Ok(())
    }
}

pub struct LibindyDefaultLogger;

impl LibindyDefaultLogger {
    pub fn init(pattern: Option<String>) -> Result<(), IndyError> {
        let pattern = pattern.or_else(|| env::var("RUST_LOG").ok());

        log_panics::init(); //Logging of panics is essential for android. As android does not log to stdout for native code

        if cfg!(target_os = "android") {
            #[cfg(target_os = "android")]
            let log_filter = match pattern {
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
        } else {
            EnvLoggerBuilder::new()
                .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
                .filter(None, LevelFilter::Off)
                .parse_filters(pattern.as_ref().map(String::as_str).unwrap_or(""))
                .try_init()?;
        }
        unsafe { LOGGER_STATE = LoggerState::Default };
        Ok(())
    }

    extern fn enabled(_context: *const c_void,
                          level: u32,
                          target: *const c_char) -> bool {
        let level = get_level(level);
        let target = ctypes::c_str_to_string(target).unwrap().unwrap();

        let metadata: Metadata = Metadata::builder()
            .level(level)
            .target(&target)
            .build();

        log::logger().enabled(&metadata)
    }

    extern fn log(_context: *const c_void,
                      level: u32,
                      target: *const c_char,
                      args: *const c_char,
                      module_path: *const c_char,
                      file: *const c_char,
                      line: u32) {
        let target = ctypes::c_str_to_string(target).unwrap().unwrap();
        let args = ctypes::c_str_to_string(args).unwrap().unwrap();
        let module_path = ctypes::c_str_to_string(module_path).unwrap();
        let file = ctypes::c_str_to_string(file).unwrap();

        let level = get_level(level);

        log::logger().log(
            &Record::builder()
                .args(format_args!("{}", args))
                .level(level)
                .target(&target)
                .module_path(module_path)
                .file(file)
                .line(Some(line))
                .build(),
        );
    }

    extern fn flush(_context: *const c_void) {
        log::logger().flush()
    }
}

fn get_level(level: u32) -> Level {
    match level {
        1 => Level::Error,
        2 => Level::Warn,
        3 => Level::Info,
        4 => Level::Debug,
        5 => Level::Trace,
        _ => unreachable!(),
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