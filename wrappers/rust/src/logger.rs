use {ErrorCode, IndyError};

use std::ffi::CString;

use ffi::{logger, CVoid, CString as IndyCString};

use log::{Log, Record, Metadata, Level};

use std::ptr::null;

use utils::ctypes::c_str_to_string;

static mut LOGGER: Option<Box<(&'static dyn Log)>> = None;

/// Set default logger implementation.
///
/// Allows library user use `env_logger` logger as default implementation.
/// More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger
///
/// # Arguments
/// * `pattern` - (optional) pattern that corresponds with the log messages to show.
pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    let pattern_str = opt_c_str!(pattern);

    let res = ErrorCode::from(unsafe {
        logger::indy_set_default_logger(opt_c_ptr!(pattern, pattern_str))
    });

    match res {
        ErrorCode::Success => Ok(()),
        err => Err(IndyError::new(err))
    }
}

/// Set application logger implementation to Libindy.
///
/// # Arguments
/// * `logger` - reference to logger used by application.
pub fn set_logger(logger: &'static dyn Log) -> Result<(), IndyError> {
    {
        unsafe {
            if LOGGER.is_some() {
                return Err(IndyError {
                    error_code: ErrorCode::CommonInvalidState,
                    message: "Logger is already set".to_string(),
                    indy_backtrace: None,
                });
            }
            LOGGER = Some(Box::new(logger));
        }
    }

    let res = ErrorCode::from(unsafe {
        logger::indy_set_logger(
            null(),
            Some(IndyLogger::enabled_cb),
            Some(IndyLogger::log_cb),
            Some(IndyLogger::flush_cb),
        )
    });

    match res {
        ErrorCode::Success => Ok(()),
        err => Err(IndyError::new(err))
    }
}

pub struct IndyLogger;

impl IndyLogger {
    pub extern fn enabled_cb(_context: *const CVoid,
                             level: u32,
                             target: IndyCString) -> bool {
        unsafe {
            match LOGGER {
                Some(ref logger) => {
                    let level = Self::get_level(level);
                    let target = c_str_to_string(target).unwrap().unwrap();

                    let metadata: Metadata = Metadata::builder()
                        .level(level)
                        .target(&target)
                        .build();

                    logger.enabled(&metadata)
                }
                None => true
            }
        }
    }

    extern fn log_cb(_context: *const CVoid,
                     level: u32,
                     target: IndyCString,
                     args: IndyCString,
                     module_path: IndyCString,
                     file: IndyCString,
                     line: u32) {
        unsafe {
            match LOGGER {
                Some(ref logger) => {
                    let target = c_str_to_string(target).unwrap().unwrap();
                    let args = c_str_to_string(args).unwrap().unwrap();
                    let module_path = c_str_to_string(module_path).unwrap();
                    let file = c_str_to_string(file).unwrap();
                    let level = Self::get_level(level);

                    logger.log(
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
                None => {}
            }
        }
    }

    pub extern fn flush_cb(_context: *const CVoid) {
        unsafe {
            match LOGGER {
                Some(ref logger) => { logger.flush() }
                None => {}
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{set_boxed_logger, logger};

    #[test]
    fn test_logger() {
        set_boxed_logger(Box::new(SimpleLogger {})).unwrap();
        set_logger(logger()).unwrap();
    }

    struct SimpleLogger;

    impl Log for SimpleLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            println!(
                "{:>5}|{:<20}|{:>25}:{:?}| {:?}",
                record.level(),
                record.target(),
                record.file().unwrap_or(""),
                record.line(),
                record.args());
        }

        fn flush(&self) {}
    }
}

