#![warn(dead_code)]

use ::{ErrorCode, IndyError};
use ffi::CommandHandle;

use libc::c_char;

use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

use futures::*;
use futures::sync::oneshot;

lazy_static! {
    static ref CALLBACKS_EMPTY: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(), IndyError>>>> = Default::default();
    static ref CALLBACKS_SLICE: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<Vec<u8>, IndyError>>>> = Default::default();
    static ref CALLBACKS_HANDLE: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<CommandHandle, IndyError>>>> = Default::default();
    static ref CALLBACKS_BOOL: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<bool, IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_SLICE: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, Vec<u8>), IndyError>>>> = Default::default();
    static ref CALLBACKS_HANDLE_USIZE: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(CommandHandle, usize), IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_STR_U64: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, String, u64), IndyError>>>> = Default::default();
    static ref CALLBACKS_STR: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<String, IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_STR: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, String), IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_OPTSTR: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, Option<String>), IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_STR_STR: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, String, String), IndyError>>>> = Default::default();
    static ref CALLBACKS_STR_OPTSTR_OPTSTR: Mutex<HashMap<CommandHandle, oneshot::Sender<Result<(String, Option<String>, Option<String>), IndyError>>>> = Default::default();
}

macro_rules! cb_ec {
    ($name:ident($($cr:ident:$crt:ty),*)->$rrt:ty, $cbs:ident, $res:expr) => (
    pub fn $name() -> (sync::oneshot::Receiver<Result<$rrt, IndyError>>,
                          CommandHandle,
                          Option<extern fn(command_handle: CommandHandle, err: i32, $($crt),*)>) {
        extern fn callback(command_handle: CommandHandle, err: i32, $($cr:$crt),*) {
            let tx = {
                let mut callbacks = $cbs.lock().unwrap();
                callbacks.remove(&command_handle).unwrap()
            };

            let res = if err != 0 {
                Err(IndyError::new(ErrorCode::from(err)))
            } else {
                Ok($res)
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
    cb_ec!(cb_ec()->(), CALLBACKS_EMPTY, ());

    cb_ec!(cb_ec_handle(handle:CommandHandle)->CommandHandle, CALLBACKS_HANDLE, handle);

    cb_ec!(cb_ec_handle_usize(handle:CommandHandle, u: usize)->(CommandHandle, usize), CALLBACKS_HANDLE_USIZE, (handle, u));

    cb_ec!(cb_ec_string(str1:*const c_char)->String,
           CALLBACKS_STR,
           rust_str!(str1));

    cb_ec!(cb_ec_string_string(str1:*const c_char, str2:*const c_char)->(String, String),
           CALLBACKS_STR_STR,
           (rust_str!(str1), rust_str!(str2)));

    cb_ec!(cb_ec_string_opt_string(str1:*const c_char, str2:*const c_char)->(String, Option<String>),
           CALLBACKS_STR_OPTSTR,
           (rust_str!(str1), opt_rust_str!(str2)));

    cb_ec!(cb_ec_string_string_string(str1: *const c_char, str2: *const c_char, str3: *const c_char)->(String, String, String),
           CALLBACKS_STR_STR_STR,
           (rust_str!(str1), rust_str!(str2), rust_str!(str3)));

    cb_ec!(cb_ec_string_opt_string_opt_string(str1: *const c_char, str2: *const c_char, str3: *const c_char)->(String, Option<String>, Option<String>),
           CALLBACKS_STR_OPTSTR_OPTSTR,
           (rust_str!(str1), opt_rust_str!(str2), opt_rust_str!(str3)));

    cb_ec!(cb_ec_string_string_u64(str1:*const c_char, str2:*const c_char, u: u64)->(String, String, u64),
           CALLBACKS_STR_STR_U64,
           (rust_str!(str1), rust_str!(str2), u));

    cb_ec!(cb_ec_slice(data:*const u8, len:u32)->Vec<u8>, CALLBACKS_SLICE, rust_slice!(data, len).to_owned());

    cb_ec!(cb_ec_string_slice(str: *const c_char, data:*const u8, len:u32)->(String, Vec<u8>),
           CALLBACKS_STR_SLICE,
           (rust_str!(str), rust_slice!(data, len).to_owned()));

    cb_ec!(cb_ec_bool(b: bool)->bool, CALLBACKS_BOOL, b);
}

macro_rules! result_handler {
    ($name:ident($res_type:ty), $map:ident) => (
    pub fn $name(command_handle: CommandHandle,
                 err: ErrorCode,
                 rx: sync::oneshot::Receiver<Result<$res_type, IndyError>>) -> Box<Future<Item=$res_type, Error= IndyError>> {
        if err != ErrorCode::Success {
            let mut callbacks = $map.lock().unwrap();
            callbacks.remove(&command_handle).unwrap();
            Box::new(future::err(IndyError::new(err)))
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
    result_handler!(empty(()), CALLBACKS_EMPTY);
    result_handler!(handle(CommandHandle), CALLBACKS_HANDLE);
    result_handler!(slice(Vec<u8>), CALLBACKS_SLICE);
    result_handler!(bool(bool), CALLBACKS_BOOL);
    result_handler!(str(String), CALLBACKS_STR);
    result_handler!(handle_usize((CommandHandle, usize)), CALLBACKS_HANDLE_USIZE);
    result_handler!(str_slice((String, Vec<u8>)), CALLBACKS_STR_SLICE);
    result_handler!(str_str((String, String)), CALLBACKS_STR_STR);
    result_handler!(str_optstr((String, Option<String>)), CALLBACKS_STR_OPTSTR);
    result_handler!(str_optstr_optstr((String, Option<String>, Option<String>)), CALLBACKS_STR_OPTSTR_OPTSTR);
    result_handler!(str_str_str((String, String, String)), CALLBACKS_STR_STR_STR);
    result_handler!(str_str_u64((String, String, u64)), CALLBACKS_STR_STR_U64);
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

        let slice1 = receiver.wait().unwrap().unwrap();
        assert_eq!(test_vec, slice1);
    }

    #[test]
    fn ec_string_opt_string_null() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let callback = cb.unwrap();
        callback(command_handle, 0, CString::new("This is a test").unwrap().as_ptr(), null());

        let (str1, str2) = receiver.wait().unwrap().unwrap();
        assert_eq!(str1, "This is a test".to_string());
        assert_eq!(str2, None);
    }

    #[test]
    fn ec_string_opt_string_some() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let callback = cb.unwrap();
        callback(command_handle, 0, CString::new("This is a test").unwrap().as_ptr(), CString::new("The second string has something").unwrap().as_ptr());

        let (str1, str2) = receiver.wait().unwrap().unwrap();
        assert_eq!(str1, "This is a test".to_string());
        assert_eq!(str2, Some("The second string has something".to_string()));
    }
}
