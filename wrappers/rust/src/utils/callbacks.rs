use {ErrorCode, IndyHandle};

use utils::sequence::SequenceUtils;

use std::os::raw::c_char;

use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Display;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use futures::*;
use futures::sync::oneshot;

use ffi::{ResponseEmptyCB,
          ResponseI32CB,
          ResponseI32UsizeCB,
          ResponseStringCB,
          ResponseStringStringCB,
          ResponseStringStringStringCB,
          ResponseStringStringU64CB,
          ResponseStringSliceCB};

fn log_error<T: Display>(e: T) {
    warn!("Unable to send through libindy callback: {}", e);
}

lazy_static! {
    static ref CALLBACKS_SLICE: Mutex<HashMap<IndyHandle, oneshot::Sender<Result<Vec<u8>, ErrorCode>>>> = Default::default();
    static ref CALLBACKS_HANDLE: Mutex<HashMap<IndyHandle, oneshot::Sender<Result<IndyHandle, ErrorCode>>>> = Default::default();
    static ref CALLBACKS_BOOL: Mutex<HashMap<IndyHandle, oneshot::Sender<Result<bool, ErrorCode>>>> = Default::default();
}

macro_rules! cb_ec {
    ($name:ident($($cr:ident:$crt:ty),*)->$rrt:ty, $cbs:ident, $mapper:expr) => (
    pub fn $name() -> (sync::oneshot::Receiver<Result<$rrt, ErrorCode>>,
                          IndyHandle,
                          Option<extern fn(command_handle: IndyHandle, err: i32, $($crt),*)>) {
        extern fn callback(command_handle: IndyHandle, err: i32, $($cr:$crt),*) {
            let tx = {
                let mut callbacks = $cbs.lock().unwrap();
                callbacks.remove(&command_handle).unwrap()
            };

            let res = if err != 0 {
                Err(ErrorCode::from(err))
            } else {
                Ok($mapper($($cr),*))
            };

            tx.send(res).unwrap();
        }

        let (rx, command_handle) = {
            let (tx, rx) = oneshot::channel();
            let command_handle = ::utils::sequence::SequenceUtils::get_next_id();
            let mut callbacks = $cbs.lock().unwrap();
            callbacks.insert(command_handle, tx);
            (rx, command_handle)
        };
        (rx, command_handle, Some(callback))
    }
    )
}

pub struct ClosureHandler {}

impl ClosureHandler {
    pub fn cb_ec() -> (Receiver<ErrorCode>, IndyHandle, Option<ResponseEmptyCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err| {
            sender.send(err).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec(closure: Box<FnMut(ErrorCode) + Send>) -> (IndyHandle, Option<ResponseEmptyCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }
        extern "C" fn _callback(command_handle: IndyHandle, err: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(ErrorCode::from(err))
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    cb_ec!(cb_ec_handle(handle:IndyHandle)->IndyHandle, CALLBACKS_HANDLE, |a| a);


    pub fn cb_ec_i32() -> (Receiver<(ErrorCode, IndyHandle)>, IndyHandle, Option<ResponseI32CB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_i32(closure: Box<FnMut(ErrorCode, IndyHandle) + Send>) -> (IndyHandle, Option<ResponseI32CB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, IndyHandle) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, val: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(ErrorCode::from(err), val)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_i32_usize() -> (Receiver<(ErrorCode, IndyHandle, usize)>, IndyHandle, Option<ResponseI32UsizeCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32_usize(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_i32_usize(closure: Box<FnMut(ErrorCode, IndyHandle, usize) + Send>) -> (IndyHandle, Option<ResponseI32UsizeCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, IndyHandle, usize) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, val1: i32, val2: usize) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(ErrorCode::from(err), val1, val2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string() -> (Receiver<(ErrorCode, String)>, IndyHandle, Option<ResponseStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string(closure: Box<FnMut(ErrorCode, String) + Send>) -> (IndyHandle, Option<ResponseStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, c_str: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = rust_str!(c_str);
            cb(ErrorCode::from(err), metadata)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string_string() -> (Receiver<(ErrorCode, String, String)>, IndyHandle, Option<ResponseStringStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_string(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (IndyHandle, Option<ResponseStringStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, String) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = rust_str!(str2);
            cb(ErrorCode::from(err), str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string_opt_string() -> (Receiver<(ErrorCode, String, Option<String>)>, IndyHandle, Option<ResponseStringStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_opt_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_opt_string(closure: Box<FnMut(ErrorCode, String, Option<String>) + Send>) -> (IndyHandle, Option<ResponseStringStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, Option<String>) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = opt_rust_str!(str2);
            cb(ErrorCode::from(err), str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string_string_string() -> (Receiver<(ErrorCode, String, String, String)>, IndyHandle, Option<ResponseStringStringStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap_or_else(log_error);
        });
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_string_string(closure: Box<FnMut(ErrorCode, String, String, String) + Send>) -> (IndyHandle, Option<ResponseStringStringStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, String, String) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, str1: *const c_char, str2: *const c_char, str3: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = rust_str!(str2);
            let str3 = rust_str!(str3);
            cb(ErrorCode::from(err), str1, str2, str3)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string_opt_string_opt_string() -> (Receiver<(ErrorCode, String, Option<String>, Option<String>)>, IndyHandle, Option<ResponseStringStringStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap_or_else(log_error);
        });
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_opt_string_opt_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_opt_string_opt_string(closure: Box<FnMut(ErrorCode, String, Option<String>, Option<String>) + Send>) -> (IndyHandle, Option<ResponseStringStringStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, Option<String>, Option<String>) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, str1: *const c_char, str2: *const c_char, str3: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = opt_rust_str!(str2);
            let str3 = opt_rust_str!(str3);
            cb(ErrorCode::from(err), str1, str2, str3)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string_string_u64() -> (Receiver<(ErrorCode, String, String, u64)>, IndyHandle, Option<ResponseStringStringU64CB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val1, val2, val3| {
            sender.send((err, val1, val2, val3)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string_u64(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_string_u64(closure: Box<FnMut(ErrorCode, String, String, u64) + Send>) -> (IndyHandle, Option<ResponseStringStringU64CB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex <HashMap<i32, Box<FnMut(ErrorCode, String, String, u64) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, str1: *const c_char, str2: *const c_char, arg1: u64) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = rust_str!(str1);
            let str2 = rust_str!(str2);
            cb(ErrorCode::from(err), str1, str2, arg1)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    cb_ec!(cb_ec_slice(data:*const u8, len:u32)->Vec<u8>, CALLBACKS_SLICE, |data, len| rust_slice!(data, len).to_owned());

    pub fn cb_ec_string_slice() -> (Receiver<(ErrorCode, String, Vec<u8>)>, IndyHandle, Option<ResponseStringSliceCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, key, msg| {
            sender.send((err, key, msg)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_slice(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string_slice(closure: Box<FnMut(ErrorCode, String, Vec<u8>) + Send>) -> (IndyHandle, Option<ResponseStringSliceCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String, Vec<u8>) + Send> >> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, vk: *const c_char, msg_raw: *const u8, msg_len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let key = rust_str!(vk);
            let msg = rust_slice!(msg_raw, msg_len);
            cb(ErrorCode::from(err), key, msg.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    cb_ec!(cb_ec_bool(b: u8)->bool, CALLBACKS_BOOL, |b| b > 0);
/*    pub fn cb_ec_bool() -> (Receiver<(ErrorCode, bool)>, IndyHandle, Option<ResponseBoolCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, v| {
            sender.send((err, v)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_bool(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_bool(closure: Box<FnMut(ErrorCode, bool) + Send>) -> (IndyHandle, Option<ResponseBoolCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, bool) + Send> >> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, valid: u8) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let v = valid > 0;
            cb(ErrorCode::from(err), v)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }*/
}

macro_rules! result_handler {
    ($name:ident($res_type:ty), $map:ident) => (
    pub fn $name(command_handle: IndyHandle,
                 err: ErrorCode,
                 rx: sync::oneshot::Receiver<Result<$res_type, ErrorCode>>) -> Box<Future<Item=$res_type, Error= ErrorCode>> {
        if err != ErrorCode::Success {
            let mut callbacks = $map.lock().unwrap();
            callbacks.remove(&command_handle).unwrap();
            Box::new(future::err(ErrorCode::from(err)))
        } else {
            Box::new(rx
                .map_err(|_| panic!("channel error!"))
                .and_then(|res| res))
        }
    }
    )
}

pub struct ResultHandler {}

impl ResultHandler {
    result_handler!(ec_slice(Vec<u8>), CALLBACKS_SLICE);
    result_handler!(ec_handle(IndyHandle), CALLBACKS_HANDLE);
    result_handler!(ec_bool(bool), CALLBACKS_BOOL);

    pub fn ec_test(command_handle: IndyHandle,
                   err: ErrorCode,
                   rx: sync::oneshot::Receiver<Result<Vec<u8>, ErrorCode>>) -> Box<Future<Item=Vec<u8>, Error=ErrorCode>> {
        if err != ErrorCode::Success {
            let mut callbacks = CALLBACKS_HANDLE.lock().unwrap();
            callbacks.remove(&command_handle).unwrap();
            Box::new(future::err(ErrorCode::from(err)))
        } else {
            Box::new(rx
                .map_err(|_| panic!("channel error!"))
                .and_then(|res| res))
        }
    }

    pub fn empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
        err.try_err()?;
        match receiver.recv() {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn empty_timeout(err: ErrorCode, receiver: Receiver<ErrorCode>, timeout: Duration) -> Result<(), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        err.try_err()?;

        let (err, val) = receiver.recv()?;

        err.try_err()?;

        Ok(val)
    }

    pub fn one_timeout<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>, timeout: Duration) -> Result<T, ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val)) => {
                err.try_err()?;
                Ok(val)
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;

        let (err, val, val2) = receiver.recv()?;

        err.try_err()?;

        Ok((val, val2))
    }

    pub fn two_timeout<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>, timeout: Duration) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val1, val2)) => {
                err.try_err()?;
                Ok((val1, val2))
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn three<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;

        let (err, val, val2, val3) = receiver.recv()?;

        err.try_err()?;

        Ok((val, val2, val3))
    }

    pub fn three_timeout<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>, timeout: Duration) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val1, val2, val3)) => {
                err.try_err()?;
                Ok((val1, val2, val3))
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::ffi::CString;
    use std::ptr::null;

    #[test]
    fn cb_ec_slice() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let test_vec: Vec<u8> = vec![250, 251, 252, 253, 254, 255];
        let callback = cb.unwrap();
        callback(command_handle, 0, test_vec.as_ptr(), test_vec.len() as u32);

        let (err, slice1) = receiver.recv().unwrap();
        assert_eq!(err, ErrorCode::Success);
        assert_eq!(test_vec, slice1);
    }

    #[test]
    fn ec_string_opt_string_null() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let callback = cb.unwrap();
        callback(command_handle, 0, CString::new("This is a test").unwrap().as_ptr(), null());

        let (err, str1, str2) = receiver.recv().unwrap();
        assert_eq!(err, ErrorCode::Success);
        assert_eq!(str1, "This is a test".to_string());
        assert_eq!(str2, None);
    }

    #[test]
    fn ec_string_opt_string_some() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let callback = cb.unwrap();
        callback(command_handle, 0, CString::new("This is a test").unwrap().as_ptr(), CString::new("The second string has something").unwrap().as_ptr());

        let (err, str1, str2) = receiver.recv().unwrap();
        assert_eq!(err, ErrorCode::Success);
        assert_eq!(str1, "This is a test".to_string());
        assert_eq!(str2, Some("The second string has something".to_string()));
    }
}
