extern crate log4rs;
extern crate log;
extern crate libc;

use utils::cstring::CStringUtils;

use self::log::{Record, Metadata, Level};
use self::libc::{c_void, c_char};
use std::error::Error;

use libindy::logger;

pub struct IndyCliLogger;

impl IndyCliLogger {
    pub fn init(path: &str) -> Result<(), String> {
        log4rs::init_file(path, Default::default())
            .map_err(|err| format!("Cannot init Indy CLI logger: {}", err.description()))?;

        logger::set_indy_logger(Some(IndyCliLogger::enabled_cb), Some(IndyCliLogger::log_cb), Some(IndyCliLogger::flush_cb))
            .map_err(|_| format!("Cannot init Libindy logger"))?;

        Ok(())
    }

    pub extern fn enabled_cb(_context: *const c_void,
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

    extern fn log_cb(_context: *const c_void,
                     level: u32,
                     target: *const c_char,
                     args: *const c_char,
                     module_path: *const c_char,
                     file: *const c_char,
                     line: u32) {
        let target = CStringUtils::c_str_to_string(target).unwrap().unwrap();
        let args = CStringUtils::c_str_to_string(args).unwrap().unwrap();
        let module_path = CStringUtils::c_str_to_string(module_path).unwrap();
        let file = CStringUtils::c_str_to_string(file).unwrap();
        let level = get_level(level);

        log::logger().log(
            &Record::builder()
                .args(format_args!("{}", args))
                .level(level)
                .target(&target)
                .module_path(module_path.as_ref().map(String::as_str))
                .file(file.as_ref().map(String::as_str))
                .line(Some(line))
                .build(),
        );
    }

    pub extern fn flush_cb(_context: *const c_void) {
        log::logger().flush()
    }
}

pub fn get_level(level: u32) -> Level {
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

macro_rules! _log_err {
    ($lvl:expr, $expr:expr) => (
        |err| {
            log!($lvl, "{} - {:?}", $expr, err);
            err
        }
    );
    ($lvl:expr) => (
        |err| {
            log!($lvl, "{:?}", err);
            err
        }
    )
}

#[macro_export]
macro_rules! error_err {
    () => ( _log_err!(::log::Level::Error) );
    ($($arg:tt)*) => ( _log_err!(::log::Level::Error, $($arg)*) )
}

#[macro_export]
macro_rules! trace_err {
    () => ( _log_err!(::log::Level::Trace) );
    ($($arg:tt)*) => ( _log_err!(::log::Level::Trace, $($arg)*) )
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