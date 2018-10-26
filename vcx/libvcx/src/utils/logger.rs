extern crate env_logger;
extern crate log;
extern crate log4rs;
extern crate log_panics;
extern crate libc;

#[cfg(target_os = "android")]
extern crate android_logger;

use std::io::Write;
use utils::logger;
use self::env_logger::Builder as EnvLoggerBuilder;
use self::log::{Level, LevelFilter, Metadata, Record};
use settings;
use std::sync::{Once, ONCE_INIT};
use self::libc::{c_char, c_void};
use std::env;
use std::ptr;
use api::logger::vcx_set_default_logger;

#[allow(unused_imports)]
#[cfg(target_os = "android")]
use self::android_logger::Filter;
use utils::cstring::CStringUtils;

use utils::error::SUCCESS;
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
    pub fn init(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> u32 {
        let logger = LibvcxLogger::new(context, enabled, log, flush);
        log::set_boxed_logger(Box::new(logger));
        log::set_max_level(LevelFilter::Trace);
        SUCCESS.code_num
    }
}

unsafe impl Sync for LibvcxLogger {}
unsafe impl Send for LibvcxLogger {}

impl log::Log for LibvcxLogger {

    fn enabled(&self, metadata: &Metadata) -> bool { true }
    fn log(&self, record: &Record) {
        use std::ptr::null;
        let log_cb = self.log;
        let message = CStringUtils::string_to_cstring(record.args().to_string());
        log_cb(self.context, 1, null(), message.as_ptr(), null(), null(), 1)
    }
    fn flush(&self) {}
}

// going away
pub struct LoggerUtils {}


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
    pub fn init(pattern: Option<String>) -> u32 {
        let pattern = pattern.or(env::var("RUST_LOG").ok());
        // This calls
        // log::set_max_level(logger.filter());
        // log::set_boxed_logger(Box::new(logger))
        match EnvLoggerBuilder::new()
            .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
            .filter(None, LevelFilter::Off)
            .parse(pattern.as_ref().map(String::as_str).unwrap_or("warn"))
            .try_init() {
            Ok(_) => 0,
            Err(e) => {
                println!("Error in logging init: {:?}", e);
                1
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

impl LoggerUtils {
    pub fn init_test_logging(level: &str) {
        // logger for testing purposes, sends to stdout (set env RUST_LOG to configure log level
        let level = match env::var("RUST_LOG") {
            Err(_) => level.to_string(),
            Ok(value) =>  value,
        };
        env::set_var("RUST_LOG", &level);
        LOGGER_INIT.call_once(|| {
            env_logger::init();
        });
    }

    pub fn init() {

        match settings::get_config_value(settings::CONFIG_ENABLE_TEST_MODE) {
            Ok(_) => return LoggerUtils::init_test_logging("off"),
            Err(x) => (),
        };

        // turn libindy logging off if RUST_LOG is not specified

        match env::var("RUST_LOG") {
            Err(_) => {
                env::set_var("RUST_LOG", "off");
            },
            Ok(value) =>  (),
        };

        LOGGER_INIT.call_once(|| {
            // Logging of panics is essential for android. As android does not log to stdout for native code
            log_panics::init();
            if cfg!(target_os = "android") {
                #[cfg(target_os = "android")]
                let log_filter = match env::var("RUST_LOG") {
                    Ok(val) => match val.to_lowercase().as_ref(){
                        "error" => Filter::default().with_min_level(log::Level::Error),
                        "warn" => Filter::default().with_min_level(log::Level::Warn),
                        "info" => Filter::default().with_min_level(log::Level::Info),
                        "debug" => Filter::default().with_min_level(log::Level::Debug),
                        "trace" => Filter::default().with_min_level(log::Level::Trace),
                        _ => Filter::default().with_min_level(log::Level::Error)
                    }
                    Err(..) => Filter::default().with_min_level(log::Level::Error)
                };

                #[cfg(target_os = "android")]
                android_logger::init_once(log_filter);
                info!("Logging for Android");
            } else if cfg!(target_os = "ios") {
                #[cfg(target_os = "ios")]
                env_logger::Builder::new()
                    .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
                    .filter(None, LevelFilter::Off)
                    .parse(env::var("RUST_LOG").as_ref().map(String::as_str).unwrap_or(""))
                    .try_init()
                    .ok();
                info!("Logging for iOS");
            } else {
                match settings::get_config_value(settings::CONFIG_LOG_CONFIG) {
                    Err(_) => {/* NO-OP - no logging configured */},
                    Ok(x) => {
                        match log4rs::init_file(&x, Default::default()) {
                            Err(e) => println!("invalid log configuration: {}", e),
                            Ok(_) => {},
                        }
                    }
                }
            }
        });
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
    fn test_logger() {
        LoggerUtils::init();
    }

    #[test]
    fn test_logging_get_logger() {
        assert_eq!(LibvcxDefaultLogger::init(Some("debug".to_string())), SUCCESS.code_num);
        unsafe {
            let (context, enabled_cb, log_cb, flush_cb) = LOGGER_STATE.get();
            assert_eq!(context, ptr::null());
            let target = CStringUtils::string_to_cstring("target".to_string());
            let level = 1;
            let b = LibvcxDefaultLogger::enabled(ptr::null(), 1, target.as_ptr());

            assert_eq!(enabled_cb.unwrap()(ptr::null(), level, target.as_ptr()), b);
        }
    }

    #[test]
    fn test_customer_logger() {
        LibvcxLogger::init(get_custom_context(),
                           Some(custom_enabled),
                           custom_log,
                           Some(custom_flush));
        error!("error level message");
        unsafe {
            assert_eq!(COUNT, 1)
        }

    }
}
