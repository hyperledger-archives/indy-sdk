extern crate env_logger;
extern crate log;
extern crate chrono;

use self::env_logger::LogBuilder;
use self::log::{LogRecord, LogLevelFilter};
use self::chrono::prelude::Utc;
use std::env;
use std::sync::{Once, ONCE_INIT};

pub struct LoggerUtils {}

static LOGGER_INIT: Once = ONCE_INIT;

impl LoggerUtils {
    pub fn init() {
        LOGGER_INIT.call_once(|| {
            let format = |record: &LogRecord| {
                format!("{:>5}|{:<30}|{:>35}:{:<4}|{:<22}| {}",
                        record.level(),
                        record.target(),
                        record.location().file(),
                        record.location().line(),
                        Utc::now(),
                        record.args())
            };
            let mut builder = LogBuilder::new();
            builder.format(format).filter(None, LogLevelFilter::Info);

            if env::var("RUST_LOG").is_ok() {
                builder.parse(&env::var("RUST_LOG").unwrap());
            }

            match builder.init() {
                Ok(_) => info!("logging started"),
                Err(x) => println!("could not initialize logging: {}", x),
            };
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

/* commented out to avoid compiler warnings
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
*/

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
