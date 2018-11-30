extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;

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