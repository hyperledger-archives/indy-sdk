extern crate env_logger;
extern crate log;
extern crate log4rs;
extern crate log_panics;
#[cfg(target_os = "android")]
extern crate android_logger;

use settings;
use std::sync::{Once, ONCE_INIT};
use std::env;
#[allow(unused_imports)]
use self::log::{Level};
#[cfg(target_os = "android")]
use self::android_logger::Filter;

pub struct LoggerUtils {}

static LOGGER_INIT: Once = ONCE_INIT;

// From: https://www.tutorialspoint.com/log4j/log4j_logging_levels.htm
//
//DEBUG	Designates fine-grained informational events that are most useful to debug an application.
//ERROR	Designates error events that might still allow the application to continue running.
//FATAL	Designates very severe error events that will presumably lead the application to abort.
//INFO	Designates informational messages that highlight the progress of the application at coarse-grained level.
//OFF	The highest possible rank and is intended to turn off logging.
//TRACE	Designates finer-grained informational events than the DEBUG.
//WARN	Designates potentially harmful situations.


impl LoggerUtils {
    pub fn init_test_logging(level: &str) {
        // logger for testing purposes, sends to stdout (set env RUST_LOG to configure log level
        let level = match env::var("RUST_LOG") {
            Err(_) => level.to_string(),
            Ok(value) =>  value,
        };
        env::set_var("RUST_LOG", &level);
        LOGGER_INIT.call_once(|| {
            env_logger::init().unwrap();
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
                // TODO: Set logging to off when deploying production android app.
                #[cfg(target_os = "android")]
                android_logger::init_once(
                    Filter::default().with_min_level(Level::Trace)
                );
                trace!("logging enabled for android");
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_logger() {
        LoggerUtils::init();
    }
}
