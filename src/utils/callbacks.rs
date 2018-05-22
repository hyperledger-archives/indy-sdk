use ErrorCode;

use utils::sequence::SequenceUtils;

use std::os::raw::c_char;

use std::collections::HashMap;
use std::slice;
use std::ffi::CStr;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

pub fn _closure_to_cb_ec() -> (Receiver<ErrorCode>, i32,
                               Option<extern fn(command_handle: i32,
                                                err: ErrorCode)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
    }

    let closure = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_i32() -> (Receiver<(ErrorCode, i32)>, i32,
                                   Option<extern fn(command_handle: i32, err: ErrorCode,
                                                    c_i32: i32)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
    }

    let closure = Box::new(move |err, val| {
        sender.send((err, val)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_i32: i32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err, c_i32)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_string() -> (Receiver<(ErrorCode, String)>, i32,
                                      Option<extern fn(command_handle: i32,
                                                       err: ErrorCode,
                                                       c_str: *const c_char)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
    }

    let closure = Box::new(move |err, val| {
        sender.send((err, val)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        cb(err, metadata)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_string_string() -> (Receiver<(ErrorCode, String, String)>, i32,
                                             Option<extern fn(command_handle: i32,
                                                              err: ErrorCode,
                                                              str1: *const c_char,
                                                              str2: *const c_char)>) {
    let (sender, receiver) = channel();

    lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
    }

    let closure = Box::new(move |err, val1, val2| {
        sender.send((err, val1, val2)).unwrap();
    });

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

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_u8() -> (Receiver<(ErrorCode, Vec<u8>)>, i32,
                                  Option<extern fn(command_handle: i32,
                                                   err: ErrorCode,
                                                   signature_raw: *const u8,
                                                   signature_len: u32)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, Vec<u8>) + Send> >> = Default::default();
    }

    let closure = Box::new(move |err, sig| {
        sender.send((err, sig)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, signature_raw: *const u8, signature_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let sig = unsafe { slice::from_raw_parts(signature_raw, signature_len as usize) };
        cb(err, sig.to_vec())
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_string_u8() -> (Receiver<(ErrorCode, String, Vec<u8>)>, i32,
                                                  Option<extern fn(command_handle: i32,
                                                                   err: ErrorCode,
                                                                   sender_vk: *const c_char,
                                                                   decrypted_msg_raw: *const u8,
                                                                   decrypted_msg_len: u32)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, Vec<u8>) + Send> >> = Default::default();
    }

    let closure = Box::new(move |err, key, msg| {
        sender.send((err, key, msg)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, vk: *const c_char, d_msg_raw: *const u8, d_msg_len: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let key = unsafe { CStr::from_ptr(vk).to_str().unwrap().to_string() };
        let decrypted = unsafe { slice::from_raw_parts(d_msg_raw, d_msg_len as usize) };
        cb(err, key, decrypted.to_vec())
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub fn _closure_to_cb_ec_bool() -> (Receiver<(ErrorCode, bool)>, i32,
                                    Option<extern fn(command_handle: i32,
                                                     err: ErrorCode,
                                                     valid: u8)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, bool) + Send> >> = Default::default();
    }

    let closure = Box::new(move |err, v| {
        sender.send((err, v)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, valid: u8) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let v = valid > 0;
        cb(err, v)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}
    
