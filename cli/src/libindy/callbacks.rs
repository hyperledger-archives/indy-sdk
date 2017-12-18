use super::ErrorCode;

use utils::sequence::SequenceUtils;

use libc::c_char;

use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

pub fn _closure_to_cb_ec(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                    Option<extern fn(command_handle: i32,
                                                                                     err: ErrorCode)>) {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_i32(closure: Box<FnMut(ErrorCode, i32) + Send>)
                             -> (i32,
                                 Option<extern fn(command_handle: i32, err: ErrorCode,
                                                  c_i32: i32)>) {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_i32: i32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err, c_i32)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_string(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                   Option<extern fn(command_handle: i32,
                                                                                                    err: ErrorCode,
                                                                                                    c_str: *const c_char)>) {
    lazy_static! {
        static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        cb(err, metadata)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_string_string(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                  Option<extern fn(command_handle: i32,
                                                                                                                   err: ErrorCode,
                                                                                                                   str1: *const c_char,
                                                                                                                   str2: *const c_char)>) {
    lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
        }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
        let str2 = unsafe { CStr::from_ptr(str2).to_str().unwrap().to_string() };
        cb(err, str1, str2)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}
