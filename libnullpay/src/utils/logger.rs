extern crate env_logger;
extern crate log;

use self::env_logger::Builder;
use self::log::LevelFilter;
use std::env;
use std::io::Write;

pub fn init() {
    Builder::new()
        .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
        .filter(None, LevelFilter::Off)
        .parse(env::var("RUST_LOG").as_ref().map(String::as_str).unwrap_or(""))
        .try_init()
        .ok();
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
