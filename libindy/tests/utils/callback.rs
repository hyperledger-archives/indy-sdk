extern crate libc;

use indy::api::ErrorCode;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::slice;
use std::sync::mpsc::{channel, Receiver};

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

lazy_static! {
    static ref CLOSURE_CB_MAP: Mutex<HashMap<i32, i32>> = Default::default();
}

pub struct CallbackUtils {}

impl CallbackUtils {
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
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
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
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_bool() -> (Receiver<(ErrorCode, bool)>, i32,
                                        Option<extern fn(command_handle: i32, err: ErrorCode,
                                                         valid: bool)>) {
        let (sender, receiver) = channel();

        lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, bool) + Send>>> = Default::default();
    }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, valid: bool) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, valid)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
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
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
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
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_string_string_string() -> (Receiver<(ErrorCode, String, String, String)>, i32,
                                                        Option<extern fn(command_handle: i32,
                                                                         err: ErrorCode,
                                                                         str1: *const c_char,
                                                                         str2: *const c_char,
                                                                         str3: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String, String) + Send > >> = Default::default();
    }

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char, str3: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = unsafe { CStr::from_ptr(str2).to_str().unwrap().to_string() };
            let str3 = unsafe { CStr::from_ptr(str3).to_str().unwrap().to_string() };
            cb(err, str1, str2, str3)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_opt_string() -> (Receiver<(ErrorCode, Option<String>)>, i32,
                                              Option<extern fn(command_handle: i32,
                                                               err: ErrorCode,
                                                               str1: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Option<String>) + Send > >> = Default::default();
    }

        let closure = Box::new(move |err, val1| {
            sender.send((err, val1)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = if !str1.is_null() {
                unsafe { Some(CStr::from_ptr(str1).to_str().unwrap().to_string()) }
            } else { None };
            cb(err, str1)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_string_opt_string() -> (Receiver<(ErrorCode, String, Option<String>)>, i32,
                                                     Option<extern fn(command_handle: i32,
                                                                      err: ErrorCode,
                                                                      str1: *const c_char,
                                                                      str2: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Option<String>) + Send > >> = Default::default();
    }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = if !str2.is_null() {
                unsafe { Some(CStr::from_ptr(str2).to_str().unwrap().to_string()) }
            } else { None };
            cb(err, str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_string_opt_string_opt_string() -> (Receiver<(ErrorCode, String, Option<String>, Option<String>)>, i32,
                                                                Option<extern fn(command_handle: i32,
                                                                                 err: ErrorCode,
                                                                                 str1: *const c_char,
                                                                                 str2: *const c_char,
                                                                                 str3: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Option<String>, Option<String>) + Send > >> = Default::default();
    }

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char, str3: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = if !str2.is_null() {
                unsafe { Some(CStr::from_ptr(str2).to_str().unwrap().to_string()) }
            } else { None };
            let str3 = if !str3.is_null() {
                unsafe { Some(CStr::from_ptr(str3).to_str().unwrap().to_string()) }
            } else { None };
            cb(err, str1, str2, str3)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_vec_u8() -> (Receiver<(ErrorCode, Vec<u8>)>, i32,
                                          Option<extern fn(command_handle: i32,
                                                           err: ErrorCode,
                                                           raw: *const u8,
                                                           len: u32)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val1| {
            sender.send((err, val1)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let vec = unsafe { slice::from_raw_parts(raw, len as usize) };
            cb(err, vec.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_string_vec_u8() -> (Receiver<(ErrorCode, String, Vec<u8>)>, i32,
                                                 Option<extern fn(command_handle: i32,
                                                                  err: ErrorCode,
                                                                  str: *const c_char,
                                                                  raw: *const u8,
                                                                  len: u32)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Vec<u8>) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str: *const c_char, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str = unsafe { CStr::from_ptr(str).to_str().unwrap().to_string() };
            let vec = unsafe { slice::from_raw_parts(raw, len as usize) };
            cb(err, str, vec.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn _closure_to_cb_ec_string_string_u64() -> (Receiver<(ErrorCode, String, String, u64)>, i32,
                                                     Option<extern fn(command_handle: i32,
                                                                      err: ErrorCode,
                                                                      str1: *const c_char,
                                                                      str2: *const c_char,
                                                                      val: u64)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String, u64) + Send > >> = Default::default();
    }

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char, val: u64) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = unsafe { CStr::from_ptr(str2).to_str().unwrap().to_string() };
            cb(err, str1, str2, val)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }
}
