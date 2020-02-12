extern crate futures;
extern crate log;
extern crate time;

use indy::logger;

pub struct SimpleLogger;

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

pub fn set_logger(logger: &'static dyn log::Log) {
    logger::set_logger(logger).ok();
}

pub fn set_default_logger() {
    logger::set_default_logger(None).ok();
}