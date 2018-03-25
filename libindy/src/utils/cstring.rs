extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;
use std::ffi::CString;

pub struct CStringUtils {}

impl CStringUtils {
    pub fn c_str_to_string(cstr: *const c_char) -> Result<Option<String>, Utf8Error> {
        if cstr.is_null() {
            return Ok(None);
        }

        unsafe {
            match CStr::from_ptr(cstr).to_str() {
                Ok(str) => Ok(Some(str.to_string())),
                Err(err) => Err(err)
            }
        }
    }

    pub fn string_to_cstring(s: String) -> CString {
        CString::new(s).unwrap()
    }
}

macro_rules! check_useful_c_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => return $e,
        };

        if $x.is_empty() {
            return $e
        }
    }
}

macro_rules! check_useful_c_str_empty_accepted {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => return $e,
        };
    }
}

macro_rules! check_useful_opt_c_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(opt_val) => opt_val,
            Err(_) => return $e
        };
    }
}

// Macro for optional integer type that converts any negative integer to None.
macro_rules! check_useful_opt_c_int {
    ($x:ident) => {
        let $x = if $x >= 0 { Some($x as u32) }  else { None };
    }
}


// Macro for usize integer type.
macro_rules! check_usize_c_int {
    ($x:ident, $e:expr) => {
        let $x = if $x > 0 { $x as usize }  else { return $e };
    }
}
