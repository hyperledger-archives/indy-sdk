extern crate env_logger;
extern crate log;
extern crate log_panics;
#[cfg(target_os = "android")]
extern crate android_logger;

use self::env_logger::Builder;
use self::log::LevelFilter;
use std::env;
use std::io::Write;
#[cfg(target_os = "android")]
use self::android_logger::Filter;

pub struct LoggerUtils {}

impl LoggerUtils {
    pub fn init() {
        log_panics::init(); //Logging of panics is essential for android. As android does not log to stdout for native code
        if cfg!(target_os = "android") {
            #[cfg(target_os = "android")]
            let log_filter = match env::var("RUST_LOG") {
                Ok(val) => match val.to_lowercase().as_ref(){
                    "error" => Filter::default().with_min_level(log::Level::Error),
                    "warn" => Filter::default().with_min_level(log::Level::Warn),
                    "info" => Filter::default().with_min_level(log::Level::Info),
                    "debug" => Filter::default().with_min_level(log::Level::Debug),
                    "trace" => Filter::default().with_min_level(log::Level::Trace),
                    _ => Filter::default().with_min_level(log::Level::Error),
                },
                Err(..) => Filter::default().with_min_level(log::Level::Error)
            };

            //Set logging to off when deploying production android app.
            #[cfg(target_os = "android")]
                android_logger::init_once(log_filter);
            info!("Logging for Android");
        } else {
            Builder::new()
                .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
                .filter(None, LevelFilter::Off)
                .parse(env::var("RUST_LOG").as_ref().map(String::as_str).unwrap_or(""))
                .try_init()
                .ok();
        }
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
