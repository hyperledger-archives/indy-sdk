//extern crate env_logger;
extern crate log;
extern crate log4rs;

use settings;
use log::LevelFilter;
use std::sync::{Once, ONCE_INIT};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::env;

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

            // if RUST_LOG is set then use that for the default log level, otherwise
            // set the default to "error" -- for both libindy and libvcx

            let level = match env::var("RUST_LOG") {
                Err(_) => {
                    env::set_var("RUST_LOG", "error");
                    LevelFilter::Error
                },
                Ok(value) =>  match value.as_ref() {
                    "info" => LevelFilter::Info,
                    "warn" => LevelFilter::Warn,
                    "error" => LevelFilter::Error,
                    "debug" => LevelFilter::Debug,
                    "trace" => LevelFilter::Trace,
                    "off" => LevelFilter::Off,
                    _ => LevelFilter::Error,
                },
            };

            match settings::get_config_value(settings::CONFIG_LOG_CONFIG) {
                Err(e) => {
                    let stdout = ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new("{l:>5}|{d(%Y-%m-%dT%H:%M:%S%.3f%z):<30}|{f:>35}:{L:<4}| {m}{n}")))
                        .build();

                    let config = Config::builder()
                        .appender(Appender::builder().build("stdout", Box::new(stdout)))
                        .build(Root::builder()
                            .appender("stdout")
                            .build(level)).unwrap();

                    log4rs::init_config(config).unwrap();
                    info!("log_config not specified ({}), using default console logger", e);
                },
                Ok(x) => {
                    match log4rs::init_file(&x, Default::default()) {
                        Err(e) => println!("invalid log configuration: {}", e),
                        Ok(_) => {},
                    }
                }
            }

            info!("log level set to: {:?}", level);
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
