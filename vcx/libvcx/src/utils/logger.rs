extern crate env_logger;
extern crate log;
extern crate log4rs;
extern crate log_panics;
extern crate libc;

#[cfg(target_os = "android")]
extern crate android_logger;

use std::io::Write;
#[allow(unused_imports)] use utils::logger;
use self::env_logger::Builder as EnvLoggerBuilder;
use self::log::{Level, LevelFilter, Metadata, Record};
use std::sync::{Once, ONCE_INIT};
use self::libc::{c_char, c_void};
use std::env;
use std::ptr;
#[allow(unused_imports)] use api::logger::vcx_set_default_logger;

#[allow(unused_imports)]
#[cfg(target_os = "android")]
use self::android_logger::Filter;
use utils::cstring::CStringUtils;

use utils::error::LOGGING_ERROR;
pub static mut LOGGER_STATE: LoggerState = LoggerState::Default;
static LOGGER_INIT: Once = ONCE_INIT;
static mut CONTEXT: *const c_void = ptr::null();
static mut ENABLED_CB: Option<EnabledCB> = None;
static mut LOG_CB: Option<LogCB> = None;
static mut FLUSH_CB: Option<FlushCB> = None;

#[derive(Debug, PartialEq)]
pub enum LoggerState {
    Default,
    Custom
}

impl LoggerState {
    pub fn get(&self) -> (*const c_void, Option<EnabledCB>, Option<LogCB>, Option<FlushCB>) {
        match self {
            LoggerState::Default => (ptr::null(), Some(LibvcxDefaultLogger::enabled), Some(LibvcxDefaultLogger::log), Some(LibvcxDefaultLogger::flush)),
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

pub struct LibvcxLogger {
    context: *const c_void,
    enabled: Option<EnabledCB>,
    log: LogCB,
    flush: Option<FlushCB>,
}

impl LibvcxLogger {
    fn new(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Self {
        LibvcxLogger { context, enabled, log, flush }
    }
    pub fn init(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Result<(), u32> {
        let logger = LibvcxLogger::new(context, enabled, log, flush);
        log::set_boxed_logger(Box::new(logger)).map_err(|_| LOGGING_ERROR.code_num)?;
        log::set_max_level(LevelFilter::Trace);
        unsafe {
            LOGGER_STATE = LoggerState::Custom;
            CONTEXT = context;
            ENABLED_CB = enabled;
            LOG_CB = Some(log);
            FLUSH_CB = flush
        }
        Ok(())
    }
}

unsafe impl Sync for LibvcxLogger {}
unsafe impl Send for LibvcxLogger {}

impl log::Log for LibvcxLogger {

    fn enabled(&self, metadata: &Metadata) -> bool { true }
    fn log(&self, record: &Record) {
        let log_cb = self.log;
        let level: u32 = record.level() as u32;
        let target = CStringUtils::string_to_cstring(record.target().to_string());
        let message = CStringUtils::string_to_cstring(record.args().to_string());
        let module_path = CStringUtils::string_to_cstring(record.module_path().unwrap_or("").to_string());
        let file = CStringUtils::string_to_cstring(record.file().unwrap_or("").to_string());
        let line = record.line().unwrap_or(0);
        log_cb(self.context, level, target.as_ptr(), message.as_ptr(), module_path.as_ptr() , file.as_ptr(), line)
    }
    fn flush(&self) {}
}

// From: https://www.tutorialspoint.com/log4j/log4j_logging_levels.htm
//
//DEBUG	Designates fine-grained informational events that are most useful to debug an application.
//ERROR	Designates error events that might still allow the application to continue running.
//FATAL	Designates very severe error events that will presumably lead the application to abort.
//INFO	Designates informational messages that highlight the progress of the application at coarse-grained level.
//OFF	The highest possible rank and is intended to turn off logging.
//TRACE	Designates finer-grained informational events than the DEBUG.
//WARN	Designates potentially harmful situations.
pub struct LibvcxDefaultLogger;

impl LibvcxDefaultLogger {
    pub fn init(pattern: Option<String>) -> Result<(), u32> {
        let pattern = pattern.or(env::var("RUST_LOG").ok());
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
            Ok(())
        } else {
            // This calls
            // log::set_max_level(logger.filter());
            // log::set_boxed_logger(Box::new(logger))
            // which are what set the logger.
            match EnvLoggerBuilder::new()
                .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
                .filter(None, LevelFilter::Off)
                .parse(pattern.as_ref().map(String::as_str).unwrap_or("warn"))
                .try_init() {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("Error in logging init: {:?}", e);
                    Err(LOGGING_ERROR.code_num)
                }
            }
        }
    }

    extern fn enabled(_context: *const c_void,
                      level: u32,
                      target: *const c_char) -> bool {
        let level = get_level(level);
        let target = CStringUtils::c_str_to_string(target).unwrap().unwrap();

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
        let target = CStringUtils::c_str_to_string(target).unwrap().unwrap();
        let args = CStringUtils::c_str_to_string(args).unwrap().unwrap();
        let module_path = CStringUtils::c_str_to_str(module_path).unwrap();
        let file = CStringUtils::c_str_to_str(file).unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;
    fn get_custom_context() -> *const c_void {
        ptr::null()
    }
    static mut CHANGED: Option<String> = None;
    static mut COUNT: u32 = 0;

    extern fn custom_enabled(context: *const c_void, level: u32, target: *const c_char) -> bool { true }
    extern fn custom_flush(context: *const c_void){}
    extern fn custom_log(context: *const c_void,
                  level: u32,
                  target: *const c_char,
                  message: *const c_char,
                  module_path: *const c_char,
                  file: *const c_char,
                  line: u32) {
        let message = CStringUtils::c_str_to_string(message).unwrap();
        unsafe { COUNT = COUNT + 1 }
    }

    #[test]
    fn test_logging_get_logger() {
        LibvcxDefaultLogger::init(Some("debug".to_string())).unwrap();
        unsafe {
            let (context, enabled_cb, log_cb, flush_cb) = LOGGER_STATE.get();
            assert_eq!(context, ptr::null());
            let target = CStringUtils::string_to_cstring("target".to_string());
            let level = 1;
            let b = LibvcxDefaultLogger::enabled(ptr::null(), 1, target.as_ptr());

            assert_eq!(enabled_cb.unwrap()(ptr::null(), level, target.as_ptr()), b);
        }
    }

    // Can only have one test that initializes logging.
    #[ignore]
    #[test]
    fn test_custom_logger() {
        LibvcxLogger::init(get_custom_context(),
                           Some(custom_enabled),
                           custom_log,
                           Some(custom_flush)).unwrap();
        error!("error level message");
        unsafe {
            assert_eq!(COUNT, 1)
        }

    }
}
