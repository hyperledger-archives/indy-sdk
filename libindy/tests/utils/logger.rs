extern crate libc;
extern crate byteorder;
extern crate serde_json;
extern crate rmp_serde;
extern crate time;
extern crate futures;
extern crate log;
extern crate indyrs as indy;

use self::indy::logger::Logger;

extern crate log as log_crate;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!(
            "{} {:>5}|{:<30}|{:>35}:{:<4?}| {}",
            time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
            record.level().to_string(),
            record.target().to_string(),
            record.file().unwrap_or(""),
            record.line(),
            record.args());
    }

    fn flush(&self) {}
}

pub fn set_logger(logger: &'static log::Log) {
    Logger::set_indy_logger(logger).unwrap()
}

pub fn set_default_logger() {
    Logger::set_default_logger("").unwrap()
}