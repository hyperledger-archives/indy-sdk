extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;

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
}

