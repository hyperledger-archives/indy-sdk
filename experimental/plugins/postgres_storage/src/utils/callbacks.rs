use libindy::ErrorCode;
use utils::sequence;

use libc::c_char;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

type EcClosure = Box<FnMut(ErrorCode) + Send>;
type EcCallback = Option<extern fn(command_handle: i32, err: ErrorCode)>;
type EcStringClosure = Box<FnMut(ErrorCode, String) + Send>;
type EcStringCallback = Option<extern fn(command_handle: i32, err: ErrorCode, c_str: *const c_char)>;

pub fn closure_to_cb_ec(closure: EcClosure) -> (i32, EcCallback) {
    lazy_static! {
       static ref CALLBACKS: Mutex<HashMap<i32, EcClosure>> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let command_handle = sequence::SequenceUtils::get_next_id();
    let mut callbacks = CALLBACKS.lock().unwrap();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}

pub fn closure_to_cb_ec_string(closure: EcStringClosure) -> (i32, EcStringCallback) {
    lazy_static! {
       static ref CALLBACKS: Mutex<HashMap<i32, EcStringClosure>> = Default::default();
    }

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        cb(err, metadata)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = sequence::SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (command_handle, Some(_callback))
}