extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;
use std::ffi::CString;

/// String helpers
pub fn c_str_to_string<'a>(cstr: *const c_char) -> Result<Option<&'a str>, Utf8Error> {
    if cstr.is_null() {
        return Ok(None);
    }

    unsafe {
        match CStr::from_ptr(cstr).to_str() {
            Ok(str) => Ok(Some(str)),
            Err(err) => Err(err)
        }
    }
}

pub fn string_to_cstring(s: String) -> CString {
    CString::new(s).unwrap()
}

pub fn str_to_cstring(s: &str) -> CString { CString::new(s).unwrap() }


macro_rules! check_useful_c_str {
    ($x:ident, $e:expr) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => val.to_string(),
            _ => {
                return err_msg($e.into(), "Invalid pointer has been passed").into()
            }
        };

        if $x.is_empty() {
            return err_msg($e.into(), "Empty string has been passed").into()
        }
    }
}

macro_rules! check_useful_opt_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => Some(val),
            Ok(None) => None,
            _ => {
                return err_msg($e.into(), "Invalid pointer has been passed").into()
            },
        };

        let $x: Option<$t>  = match $x {
            Some(val) => {
                parse_json!(val, $e, $t);
                Some(val)
            },
            None => None
        };
    }
}

macro_rules! check_useful_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => {
                return err_msg($e.into(), "Invalid pointer has been passed").into()
            },
        };

        parse_json!($x, $e, $t);
    }
}

macro_rules! parse_json {
    ($x:ident, $e:expr, $t:ty) => {
        if $x.is_empty() {
           return err_msg($e.into(), "Empty string has been passed").into()
        }

        let r = serde_json::from_str::<$t>($x)
                    .to_indy(::errors::IndyErrorKind::InvalidStructure, format!("Invalid {} json has been passed", stringify!($t)));

        let $x: $t = match r {
            Ok(ok) => ok,
            Err(err) => {
                return err.into()
            }
        };
    }
}

macro_rules! check_useful_c_str_empty_accepted {
    ($x:ident, $e:expr) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => val.to_string(),
            _ => {
                return err_msg($e.into(), "Invalid pointer has been passed").into()
            }
        };
    }
}

macro_rules! check_useful_opt_c_str {
    //TODO This no longer returns None options, only Strings are returned
    ($x:ident, $e:expr) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(opt_val) => opt_val.map(String::from),
            Err(_) => {
                return err_msg($e.into(), "Invalid pointer has been passed").into()
            }
        };
    }
}

/// Vector helpers
macro_rules! check_useful_c_byte_array {
    ($ptr:ident, $len:expr, $err1:expr, $err2:expr) => {
        if $ptr.is_null() {
            return err_msg($err1.into(), "Invalid pointer has been passed").into();
        }

        if $len <= 0 {
            return err_msg($err2.into(), "Array length must be greater than 0").into();
        }

        let $ptr = unsafe { $crate::std::slice::from_raw_parts($ptr, $len as usize) };
        let $ptr = $ptr.to_vec();
    }
}

//Returnable pointer is valid only before first vector modification
pub fn vec_to_pointer(v: &Vec<u8>) -> (*const u8, u32) {
    let len = v.len() as u32;
    (v.as_ptr() as *const u8, len)
}