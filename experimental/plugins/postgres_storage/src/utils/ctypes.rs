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
            _ => return $e,
        };

        if $x.is_empty() {
            return $e
        }
    }
}

#[allow(unused_macros)]
macro_rules! check_useful_opt_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => Some(val),
            Ok(None) => None,
            _ => return $e,
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

#[allow(unused_macros)]
macro_rules! check_useful_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => return $e,
        };

        parse_json!($x, $e, $t);
    }
}

#[allow(unused_macros)]
macro_rules! parse_json {
    ($x:ident, $e:expr, $t:ty) => {
        if $x.is_empty() {
            return $e
        }

        let $x: $t = match
            serde_json::from_str::<$t>($x)
                .map_err(map_err_trace!())
                .map_err(|err|
                    CommonError::InvalidStructure(
                        format!("Invalid $t json: {:?}", err)))
            {
                Ok(ok) => ok,
                Err(err) => return err.to_error_code(),
            };
    }
}

#[allow(unused_macros)]
macro_rules! check_useful_c_str_empty_accepted {
    ($x:ident, $e:expr) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(Some(val)) => val.to_string(),
            _ => return $e,
        };
    }
}

#[allow(unused_macros)]
macro_rules! check_useful_opt_c_str {
    ($x:ident, $e:expr) => {
        let $x = match ctypes::c_str_to_string($x) {
            Ok(opt_val) => opt_val.map(String::from),
            Err(_) => return $e
        };
    }
}

/// Vector helpers
#[allow(unused_macros)]
macro_rules! check_useful_c_byte_array {
    ($ptr:ident, $len:expr, $err1:expr, $err2:expr) => {
        if $ptr.is_null() {
            return $err1
        }

        if $len <= 0 {
            return $err2
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