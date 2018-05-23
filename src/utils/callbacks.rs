use {ErrorCode, IndyHandle};

use utils::sequence::SequenceUtils;

use std::os::raw::c_char;

use std::collections::HashMap;
use std::slice;
use std::ffi::CStr;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

use ffi::{ResponseEmptyCB,
          ResponseI32CB,
          ResponseStringCB,
          ResponseStringStringCB,
          ResponseStringStringU64CB,
          ResponseSliceCB,
          ResponseStringSliceCB,
          ResponseBoolCB};

pub struct ClosureHandler {}

impl ClosureHandler {
    pub fn cb_ec() -> (Receiver<ErrorCode>, IndyHandle, Option<ResponseEmptyCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_i32() -> (Receiver<(ErrorCode, IndyHandle)>, IndyHandle, Option<ResponseI32CB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, IndyHandle) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, c_i32: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, c_i32)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_string() -> (Receiver<(ErrorCode, String)>, IndyHandle, Option<ResponseStringCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, c_str: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = rust_str!(c_str);
            cb(err, metadata)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_string_string() -> (Receiver<(ErrorCode, String, String)>, IndyHandle, Option<ResponseStringStringCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, String) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = rust_str!(str2);
            cb(err, str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_string_string_u64() -> (Receiver<(ErrorCode, String, String, u64)>, IndyHandle, Option<ResponseStringStringU64CB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex <HashMap<i32, Box<FnMut(ErrorCode, String, String, u64) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, str1: *const c_char, str2: *const c_char, arg1: u64) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = rust_str!(str2);
            cb(err, str1, str2, arg1)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_slice() -> (Receiver<(ErrorCode, Vec<u8>)>, IndyHandle, Option<ResponseSliceCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, Vec<u8>) + Send> >> = Default::default();
        }

        let closure = Box::new(move |err, sig| {
            sender.send((err, sig)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let sig = rust_slice!(raw, len);
            cb(err, sig.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_string_slice() -> (Receiver<(ErrorCode, String, Vec<u8>)>, IndyHandle, Option<ResponseStringSliceCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, Vec<u8>) + Send> >> = Default::default();
        }

        let closure = Box::new(move |err, key, msg| {
            sender.send((err, key, msg)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, vk: *const c_char, d_msg_raw: *const u8, d_msg_len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let key = rust_str!(vk);
            let decrypted = rust_slice!(d_msg_raw, d_msg_len);
            cb(err, key, decrypted.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn cb_ec_bool() -> (Receiver<(ErrorCode, bool)>, IndyHandle, Option<ResponseBoolCB>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, bool) + Send> >> = Default::default();
        }

        let closure = Box::new(move |err, v| {
            sender.send((err, v)).unwrap();
        });

        extern "C" fn _callback(command_handle: IndyHandle, err: ErrorCode, valid: u8) {
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
}
