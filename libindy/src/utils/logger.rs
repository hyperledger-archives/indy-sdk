extern crate env_logger;
extern crate log;

use self::env_logger::LogBuilder;
use self::log::{LogRecord, LogLevelFilter};
use std::env;
use std::sync::{Once, ONCE_INIT};

pub struct LoggerUtils {}

static LOGGER_INIT: Once = ONCE_INIT;

impl LoggerUtils {
    pub fn init() {
        LOGGER_INIT.call_once(|| {
            let format = |record: &LogRecord| {
                format!("{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.location().file(), record.location().line(), record.args())
            };
            let mut builder = LogBuilder::new();
            builder.format(format).filter(None, LogLevelFilter::Info);

            if env::var("RUST_LOG").is_ok() {
                builder.parse(&env::var("RUST_LOG").unwrap());
            }

            builder.init().unwrap();
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
