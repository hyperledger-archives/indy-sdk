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