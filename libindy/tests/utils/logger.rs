use indy::api::logger::*;

extern crate libc;
extern crate time;
extern crate log;

use std::ptr::null;
use std::ffi::CString;

use self::libc::{c_void, c_char};

use utils::cstring::CStringUtils;

use self::log::Level;

pub struct LoggerUtils {}

impl LoggerUtils {
    pub extern fn log(_context: *const c_void,
                      level: u32,
                      target: *const c_char,
                      args: *const c_char,
                      _module_path: *const c_char,
                      file: *const c_char,
                      line: u32) {
        let target = CStringUtils::c_str_to_string(target).unwrap().unwrap();
        let args = CStringUtils::c_str_to_string(args).unwrap().unwrap();
        let file = CStringUtils::c_str_to_string(file).unwrap();

        let level = match level {
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            5 => Some(Level::Trace),
            _ => None,
        };

        println!(
            "{} {:>5}|{:<30}|{:>35}:{:<4}| {}",
            time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
            level.map(|l| l.to_string()).unwrap_or(String::new()),
            target.to_string(),
            file.unwrap_or(String::new()),
            line,
            args);
    }

    pub extern fn flush(_context: *const c_void) {}

    pub fn set_logger() {
        indy_set_logger(
            null(),
            None,
            Some(LoggerUtils::log),
            Some(LoggerUtils::flush),
        );
    }

    pub fn set_default_logger() {
        indy_set_default_logger(null());
    }
}