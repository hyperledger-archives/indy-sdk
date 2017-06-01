extern crate env_logger;

use std::sync::{Once, ONCE_INIT};

pub struct LoggerUtils {}

impl LoggerUtils {
    pub fn init() {
        lazy_static! {
            static ref LOGGER_INIT: Once = ONCE_INIT;
        }

        LOGGER_INIT.call_once(|| {
            env_logger::init().unwrap();
        });
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
    () => ( _map_err!(::log::LogLevel::Error) );
    ($($arg:tt)*) => ( _map_err!(::log::LogLevel::Error, $($arg)*) )
}

#[macro_export]
macro_rules! map_err_trace {
    () => ( _map_err!(::log::LogLevel::Trace) );
    ($($arg:tt)*) => ( _map_err!(::log::LogLevel::Trace, $($arg)*) )
}
