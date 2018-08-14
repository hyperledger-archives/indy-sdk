use indy::api::logger::*;

extern crate libc;
extern crate time;
extern crate log;

use std::ptr::null;
use std::str::FromStr;

use self::libc::{c_void, c_char};

use utils::cstring::CStringUtils;

use self::log::Level;


pub struct LoggerUtils {}

impl LoggerUtils {
    pub extern fn enable(_context: *const c_void,
                         _level: *const c_char,
                         _target: *const c_char) -> bool {
        true
    }

    pub extern fn log(_context: *const c_void,
                      level: *const c_char,
                      _target: *const c_char,
                      args: *const c_char,
                      _module_path: *const c_char,
                      file: *const c_char,
                      _line: i32) {
        let level = CStringUtils::c_str_to_string(level).unwrap().unwrap();
        let args = CStringUtils::c_str_to_string(args).unwrap().unwrap();
        let file = CStringUtils::c_str_to_string(file).unwrap();

        let level = Level::from_str(&level).unwrap();

        println!(
            "{} {:<5} [{}] {}",
            time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
            level.to_string(),
            file.unwrap_or(String::new()),
            args);
    }

    pub extern fn flush(_context: *const c_void) {}

    pub fn init_logger() {
        indy_init_logger(
            null(),
            Some(LoggerUtils::enable),
            Some(LoggerUtils::log),
            Some(LoggerUtils::flush),
        );
    }
}