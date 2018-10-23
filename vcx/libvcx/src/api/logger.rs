extern crate env_logger;
extern crate log;
use self::env_logger::Builder as EnvLoggerBuilder;
use self::log::LevelFilter;
use std::io::Write;
use std::env;

#[derive(Debug, PartialEq)]
pub enum LoggerState {
    Default,
    Custom
}
pub static mut LOGGER_STATE: LoggerState = LoggerState::Default;

#[no_mangle]
pub extern fn vcx_set_default_logger(pattern: Option<String>) -> u32 {

    let pattern = pattern.or(env::var("RUST_LOG").ok());
//    let pattern = pattern.or(var("RUST_LOG").ok_or("debug")).unwrap_or("debug");
    println!("logger set");
    match EnvLoggerBuilder::new()
        .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
        .filter(None, LevelFilter::Off)
        .parse(pattern.as_ref().map(String::as_str).unwrap_or("warn"))
        .try_init() {
        Ok(_) => 0,
        Err(e) => { println!("Error in logging init: {:?}", e); 1},
    }
}