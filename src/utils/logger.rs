extern crate env_logger;
extern crate log;

use std::sync::{Once, ONCE_INIT};

pub struct LoggerUtils {}

struct SimpleLogger;

use self::log::{LogMetadata, LogRecord, LogLevelFilter};

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        true
    }
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{:>5}|{:<25}|{:>30}:{:<4}| {}", record.level(), record.target(), record.location().file(), record.location().line(), record.args());
        }
    }
}

impl LoggerUtils {
    pub fn init() {
        lazy_static! {
            static ref LOGGER_INIT: Once = ONCE_INIT;
        }

        LOGGER_INIT.call_once(|| {
            log::set_logger(|max_log_level| {
                max_log_level.set(LogLevelFilter::Info);
                Box::new(SimpleLogger)
            }).unwrap();
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
