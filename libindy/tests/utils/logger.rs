use indy::api::logger::*;

extern crate libc;
extern crate time;
extern crate log as log_crate;

use std::ptr::null;

use self::libc::{c_void, c_char};

use utils::ctypes;

use self::log_crate::Level;


pub extern fn log(_context: *const c_void,
                  level: u32,
                  target: *const c_char,
                  args: *const c_char,
                  _module_path: *const c_char,
                  file: *const c_char,
                  line: u32) {
    let target = ctypes::c_str_to_string(target).unwrap().unwrap();
    let args = ctypes::c_str_to_string(args).unwrap().unwrap();
    let file = ctypes::c_str_to_string(file).unwrap();

    let level = match level {
        1 => Level::Error,
        2 => Level::Warn,
        3 => Level::Info,
        4 => Level::Debug,
        5 => Level::Trace,
        _ => unreachable!(),
    };

    println!(
        "{} {:>5}|{:<30}|{:>35}:{:<4}| {}",
        time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
        level.to_string(),
        target.to_string(),
        file.unwrap_or(""),
        line,
        args);
}

pub extern fn flush(_context: *const c_void) {}

pub fn set_logger() {
    indy_set_logger(
        null(),
        None,
        Some(log),
        Some(flush),
    );
}

pub fn set_default_logger() {
    indy_set_default_logger(null());
}