//extern crate env_logger;
extern crate log;
extern crate log4rs;

use settings;
use std::sync::{Once, ONCE_INIT};

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
    pub fn init() {
        LOGGER_INIT.call_once(|| {
            match settings::get_config_value(settings::CONFIG_LOG_CONFIG) {
                Err(_) => {/* NO-OP - no logging configured */},
                Ok(x) => {
                    match log4rs::init_file(&x, Default::default()) {
                        Err(e) => println!("invalid log configuration: {}", e),
                        Ok(_) => {},
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
