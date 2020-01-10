extern crate log4rs;
extern crate log;
extern crate libc;

use std::error::Error;
use indy;

pub struct  IndyCliLogger;

impl IndyCliLogger {
    pub fn init(path: &str) -> Result<(), String> {
        log4rs::init_file(path, Default::default())
            .map_err(|err| format!("Cannot init Indy CLI logger: {}", err.description()))?;

        indy::logger::set_logger(log::logger())
            .map_err(|_| "Cannot init Libindy logger".to_string())
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