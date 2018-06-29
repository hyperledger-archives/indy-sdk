extern crate libc;

use self::libc::c_char;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvTimeoutError;
use utils::libindy::next_i32_command_handle;
use utils::libindy::callback;
use utils::libindy::error_codes::map_indy_error;
use utils::timeout::TimeoutUtils;
use utils::error;
use std::sync::mpsc::channel;
use std::fmt::Display;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ops::Deref;

fn log_error<T: Display>(e: T) {
    warn!("Unable to send through libindy callback in vcx: {}", e);
}

fn insert_closure<T>(closure: T, map: &Mutex<HashMap<i32, T>>) -> i32 {
    let command_handle = next_i32_command_handle();
    {
        let mut callbacks = map.lock().expect(callback::POISON_MSG);
        callbacks.insert(command_handle, closure);
    }
    command_handle
}

pub fn receive<T>(receiver: &Receiver<T>, timeout: Option<Duration>) -> Result<T, u32>{
    let timeout_val = timeout.unwrap_or(TimeoutUtils::medium_timeout());

    match receiver.recv_timeout(timeout_val) {
        Ok(t) => Ok(t),
        Err(e) => match e {
            RecvTimeoutError::Timeout => {
                warn!("Timed Out waiting for libindy to call back");
                Err(error::TIMEOUT_LIBINDY_ERROR.code_num)
            },
            RecvTimeoutError::Disconnected => {
                warn!("Channel to libindy was disconnected unexpectedly");
                Err(error::TIMEOUT_LIBINDY_ERROR.code_num)
            }
        }
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32 {
    pub command_handle: i32,
    pub receiver: Receiver<i32>,
}

impl Return_I32 {
    pub fn new() -> Result<Return_I32, u32> {
        let (sender, receiver) = channel();
        let closure: Box<FnMut(i32) + Send> = Box::new(move |err | {
            sender.send(err).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32.deref());

        Ok(Return_I32 {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn(command_handle: i32, arg1: i32) {
        callback::call_cb_i32
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<(), u32> {
        let err = receive(&self.receiver, timeout)?;
        map_indy_error((), err)
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_I32 {
    pub command_handle: i32,
    pub receiver: Receiver<(i32, i32)>,
}
impl Return_I32_I32 {
    pub fn new() -> Result<Return_I32_I32, u32> {
        let (sender, receiver) = channel();
        let closure: Box<FnMut(i32, i32) + Send> = Box::new(move |err, arg1 | {
            sender.send((err, arg1)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_I32.deref());

        Ok(Return_I32_I32 {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn (command_handle: i32, arg1: i32, arg2: i32) {
        callback::call_cb_i32_i32
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<i32, u32> {
        let (err, arg1) = receive(&self.receiver, timeout)?;

        map_indy_error(arg1, err)
    }
}


#[allow(non_camel_case_types)]
pub struct Return_I32_STR {
    pub command_handle: i32,
    pub receiver: Receiver<(i32, Option<String>)>,
}
impl Return_I32_STR {
    pub fn new() -> Result<Return_I32_STR, u32> {
        let (sender, receiver) = channel();
        let closure:Box<FnMut(i32, Option<String>) + Send> = Box::new(move |err, str | {
            sender.send((err, str)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_STR.deref());

        Ok(Return_I32_STR {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn(command_handle: i32, arg1: i32, arg2: *const c_char) {
        callback::call_cb_i32_str
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<Option<String>, u32> {
        let (err, str1) = receive(&self.receiver, timeout)?;
        map_indy_error(str1, err)
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_STR_STR {
    pub command_handle: i32,
    receiver: Receiver<(i32, Option<String>, Option<String>)>,
}
impl Return_I32_STR_STR {
    pub fn new() -> Result<Return_I32_STR_STR, u32> {
        let (sender, receiver) = channel();
        let closure:Box<FnMut(i32, Option<String>, Option<String>) + Send> = Box::new(move |err, str1, str2 | {
            sender.send((err, str1, str2)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_STR_STR.deref());

        Ok(Return_I32_STR_STR {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn(command_handle: i32,
                                            arg1: i32,
                                            arg2: *const c_char,
                                            arg3: *const c_char) {
        callback::call_cb_i32_str_str
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<(Option<String>, Option<String>), u32> {
        let (err, mut str1, mut str2) = receive(&self.receiver, timeout)?;

        str1 = map_indy_error(str1, err)?;
        str2 = map_indy_error(str2, err)?;
        Ok((str1, str2))
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_STR_STR_STR {
    pub command_handle: i32,
    receiver: Receiver<(i32, Option<String>, Option<String>, Option<String>)>,
}
impl Return_I32_STR_STR_STR {
    pub fn new() -> Result<Return_I32_STR_STR_STR , u32> {
        let (sender, receiver) = channel();
        let closure:Box<FnMut(i32, Option<String>, Option<String>, Option<String>) + Send> = Box::new(move |err, str1, str2, str3 | {
            sender.send((err, str1, str2, str3)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_STR_STR_STR.deref());

        Ok(Return_I32_STR_STR_STR {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn(command_handle: i32,
                                            arg1: i32,
                                            arg2: *const c_char,
                                            arg3: *const c_char,
                                            arg4: *const c_char) {
        callback::call_cb_i32_str_str_str
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<(Option<String>, Option<String>, Option<String>), u32> {
        let (err, mut str1, mut str2, mut str3) = receive(&self.receiver, timeout)?;

        str1 = map_indy_error(str1, err)?;
        str2 = map_indy_error(str2, err)?;
        str3 = map_indy_error(str3, err)?;
        Ok((str1, str2, str3))
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_BOOL {
    pub command_handle: i32,
    receiver: Receiver<(i32, bool)>,
}

impl Return_I32_BOOL {
    pub fn new() -> Result<Return_I32_BOOL, u32> {
        let (sender, receiver) = channel();
        let closure: Box<FnMut(i32, bool) + Send> = Box::new(move |err, arg1 | {
            sender.send((err, arg1)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_BOOL.deref());

        Ok(Return_I32_BOOL {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn (command_handle: i32, arg1: i32, arg2: bool) {
        callback::call_cb_i32_bool
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<bool, u32> {
        let (err, arg1) = receive(&self.receiver, timeout)?;

        map_indy_error(arg1, err)?;
        Ok(arg1)
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_BIN {
    pub command_handle: i32,
    receiver: Receiver<(i32, Vec<u8>)>,
}

impl Return_I32_BIN {
    pub fn new() -> Result<Return_I32_BIN, u32> {
        let (sender, receiver) = channel();
        let closure: Box<FnMut(i32, Vec<u8>) + Send> = Box::new(move |err, arg1| {
            sender.send((err, arg1)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_BIN.deref());

        Ok(Return_I32_BIN {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn (command_handle: i32, arg1: i32, *const u8, u32) {
        callback::call_cb_i32_bin
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<Vec<u8>, u32> {
        let (err, arg1) = receive(&self.receiver, timeout)?;

        map_indy_error(arg1, err)
    }
}

#[allow(non_camel_case_types)]
pub struct Return_I32_OPTSTR_BIN {
    pub command_handle: i32,
    receiver: Receiver<(i32, Option<String>, Vec<u8>)>,
}

impl Return_I32_OPTSTR_BIN {
    pub fn new() -> Result<Return_I32_OPTSTR_BIN, u32> {
        let (sender, receiver) = channel();
        let closure: Box<FnMut(i32, Option<String>, Vec<u8>) + Send> = Box::new(move |err, arg1, arg2| {
            sender.send((err, arg1, arg2)).unwrap_or_else(log_error);
        });

        let command_handle = insert_closure(closure, callback::CALLBACKS_I32_OPTSTR_BIN.deref());

        Ok(Return_I32_OPTSTR_BIN {
            command_handle,
            receiver,
        })
    }

    pub fn get_callback(&self) -> extern fn (command_handle: i32, arg1: i32, arg2: *const c_char, arg3: *const u8, arg4: u32) {
        callback::call_cb_i32_str_bin
    }

    pub fn receive(&self, timeout: Option<Duration>) -> Result<(Option<String>, Vec<u8>), u32> {
        let (err, arg1, mut arg2) = receive(&self.receiver, timeout)?;

        arg2 = map_indy_error(arg2, err)?;
        Ok((arg1, arg2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn cstring(str_val: &String) -> CString {
        CString::new(str_val.clone()).unwrap()
    }

    #[test]
    fn test_return_i32() {
        let rtn = Return_I32::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 0);
        let val = rtn.receive(None);
        assert!(val.is_ok());

        let rtn = Return_I32::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 123);
        let val = rtn.receive(None);
        assert!(val.is_err());
    }

    #[test]
    fn test_return_i32_i32() {
        let test_val = 23455;

        let rtn = Return_I32_I32::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 0, test_val);
        let val = rtn.receive(None);
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), test_val);

        let rtn = Return_I32_I32::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 123, test_val);
        let val = rtn.receive(None);
        assert!(val.is_err());
    }

    #[test]
    fn test_return_i32_str() {
        let test_str = "Journey before destination".to_string();

        let rtn = Return_I32_STR::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 0, cstring(&test_str).as_ptr());
        let val = rtn.receive(None);
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), Some(test_str.clone()));

        let rtn = Return_I32_STR::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 0, ptr::null());
        let val = rtn.receive(None);
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), None);

        let rtn = Return_I32_STR::new().unwrap();
        rtn.get_callback()(rtn.command_handle, 123, cstring(&test_str).as_ptr());
        let val = rtn.receive(None);
        assert!(val.is_err());
    }

}
