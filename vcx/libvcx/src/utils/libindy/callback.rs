extern crate  libc;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::Mutex;
use std::slice;
use std::ops::Deref;
use std::hash::Hash;

use utils::libindy::next_i32_command_handle;

pub const POISON_MSG: &str = "FAILED TO LOCK CALLBACK MAP!";

lazy_static! {
    pub static ref CALLBACKS_I32: Mutex<HashMap<i32, Box<FnMut(i32) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_I32: Mutex<HashMap<i32, Box<FnMut(i32, i32) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_STR: Mutex<HashMap<i32, Box<FnMut(i32, Option<String>) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_STR_STR: Mutex<HashMap <i32, Box<FnMut(i32, Option<String>, Option<String>) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_STR_STR_STR: Mutex<HashMap <i32, Box<FnMut(i32, Option<String>, Option<String>, Option<String>) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_BOOL: Mutex<HashMap<i32, Box<FnMut(i32, bool) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_BIN: Mutex<HashMap<i32, Box<FnMut(i32, Vec<u8>) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_OPTSTR_BIN: Mutex<HashMap<i32,Box<FnMut(i32, Option<String>, Vec<u8>) + Send>>> = Default::default();
    pub static ref CALLBACKS_I32_BIN_BIN: Mutex<HashMap<i32, Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>>> = Default::default();
}

pub extern "C" fn call_cb_i32(command_handle: i32, arg1: i32) {
    let cb = get_cb(command_handle, CALLBACKS_I32.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1)
    }
}

pub extern "C" fn call_cb_i32_i32(command_handle: i32, arg1: i32, arg2: i32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_I32.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, arg2)
    }
}

pub extern "C" fn call_cb_i32_str(command_handle: i32, arg1: i32, arg2: *const c_char) {
    let cb = get_cb(command_handle, CALLBACKS_I32_STR.deref());
    let str1 = build_string(arg2);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1)
    }
}

pub extern "C" fn call_cb_i32_str_str(command_handle: i32, arg1: i32, arg2: *const c_char, arg3: *const c_char) {
    let cb = get_cb(command_handle, CALLBACKS_I32_STR_STR.deref());
    let str1 = build_string(arg2);
    let str2 = build_string(arg3);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1, str2)
    }
}

pub extern "C" fn call_cb_i32_str_str_str(command_handle: i32,
                                          arg1: i32,
                                          arg2: *const c_char,
                                          arg3: *const c_char,
                                          arg4: *const c_char) {
    let cb = get_cb(command_handle, CALLBACKS_I32_STR_STR_STR.deref());
    let str1 = build_string(arg2);
    let str2 = build_string(arg3);
    let str3 = build_string(arg4);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1, str2, str3)
    }
}

pub extern "C" fn call_cb_i32_bool(command_handle: i32, arg1: i32, arg2: bool) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BOOL.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, arg2)
    }
}

pub extern "C" fn call_cb_i32_bin(command_handle: i32, arg1: i32, buf: *const u8, len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BIN.deref());
    let data = build_buf(buf, len);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, data)
    }
}

pub extern "C" fn call_cb_i32_str_bin(command_handle: i32, arg1: i32, arg2: *const c_char, buf: *const u8, len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_OPTSTR_BIN.deref());
    let data = build_buf(buf, len);

    let str1 = build_string(arg2);

    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1, data)
    }
}

pub extern "C" fn call_cb_i32_bin_bin(command_handle: i32, arg1: i32, buf1: *const u8, buf1_len: u32, buf2: *const u8, buf2_len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BIN_BIN.deref());
    let data1 = build_buf(buf1, buf1_len);
    let data2 = build_buf(buf2, buf2_len);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, data1, data2)
    }
}

pub fn build_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null(){
        return None;
    }

    let cstr: &CStr = unsafe {
        CStr::from_ptr(ptr)
    };

    match cstr.to_str() {
        Ok(s) => Some(s.to_string()),
        Err(e) => {
            warn!("String from libindy with malformed utf8: {}",e);
            None
        }
    }
}

pub fn build_buf(ptr: *const u8, len: u32) -> Vec<u8>{
    let data = unsafe {
        slice::from_raw_parts(ptr, len as usize)
    };

    data.to_vec()
}

pub fn get_cb<H: Eq + Hash,T>(command_handle: H, map: &Mutex<HashMap<H, T>>) -> Option<T> {
    //TODO Error case, what should we do if the static map can't be locked? Some what
    //TODO general question for all of our Mutexes.
    let mut locked_map = map.lock().expect(POISON_MSG);
    match locked_map.remove(&command_handle){
        Some(t) => Some(t),
        None => {
            warn!("Unable to find callback in map for libindy call");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn cstring(str_val: &String) -> CString {
        CString::new(str_val.clone()).unwrap()
    }

    #[test]
    fn test_build_string() {
        let test_str = "Journey before destination".to_string();

        let test = build_string(cstring(&test_str).as_ptr());
        assert!(test.is_some());
        assert_eq!(test_str, test.unwrap());
    }

    #[test]
    fn test_get_cb(){
        let mutex_map: Mutex<HashMap<i32, Box<FnMut(i32) + Send>>> = Default::default();
        assert!(get_cb(2123, &mutex_map).is_none());

        let closure: Box<FnMut(i32) + Send> = Box::new(move |err | {

        });

        mutex_map.lock().unwrap().insert(2123,closure);
        let cb = get_cb(2123, &mutex_map);
        assert!(cb.is_some());
    }


}


//**************************************
// FOR LEGACY
// Should be come not needed as the transition is complete
//**************************************

fn init_callback<T>(closure: T, map: &Mutex<HashMap<i32, T>>) -> (i32) {
    let command_handle = next_i32_command_handle();
    {
        let mut callbacks = map.lock().unwrap();
        callbacks.insert(command_handle, closure);
    }
    command_handle
}

pub fn closure_cb_i32(closure: Box<FnMut(i32) + Send>)
                      -> (i32, Option<extern fn(command_handle: i32, arg1: i32)>) {
    (init_callback(closure, CALLBACKS_I32.deref()), Some(call_cb_i32))
}

pub fn closure_cb_i32_i32(closure: Box<FnMut(i32, i32) + Send>)
                          -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: i32)>) {
    (init_callback(closure, CALLBACKS_I32_I32.deref()), Some(call_cb_i32_i32))
}

pub fn closure_cb_i32_str(closure: Box<FnMut(i32, Option<String>) + Send>)
                          -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char)>) {
    (init_callback(closure, CALLBACKS_I32_STR.deref()), Some(call_cb_i32_str))
}

pub fn closure_cb_i32_str_str(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
                              -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char, arg3: *const c_char)>) {
    (init_callback(closure, CALLBACKS_I32_STR_STR.deref()), Some(call_cb_i32_str_str))
}

pub fn closure_cb_i32_bool(closure: Box<FnMut(i32, bool) + Send>)
                           -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: bool)>) {
    (init_callback(closure, CALLBACKS_I32_BOOL.deref()), Some(call_cb_i32_bool))
}

pub fn closure_cb_i32_bin(closure: Box<FnMut(i32, Vec<u8>) + Send>)
                          -> (i32, Option<extern fn(command_handle: i32, arg1: i32, buf: *const u8, len: u32)>) {
    (init_callback(closure, CALLBACKS_I32_BIN.deref()), Some(call_cb_i32_bin))
}

pub fn closure_cb_i32_str_bin(closure: Box<FnMut(i32, Option<String>, Vec<u8>) + Send>)
                              -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char, buf: *const u8, len: u32)>){
    (init_callback(closure, CALLBACKS_I32_OPTSTR_BIN.deref()), Some(call_cb_i32_str_bin))
}

pub fn closure_cb_i32_bin_bin(closure: Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>)
                              -> (i32, Option<extern fn(command_handle: i32, arg1: i32, buf1: *const u8, buf1_len: u32, buf2: *const u8, buf2_len: u32)>){
    (init_callback(closure, CALLBACKS_I32_BIN_BIN.deref()), Some(call_cb_i32_bin_bin))
}