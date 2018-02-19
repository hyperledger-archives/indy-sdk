extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;
use std::ffi::CString;
#[allow(dead_code)]
pub struct CStringUtils {}

#[allow(dead_code)]
impl CStringUtils {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn string_to_cstring(s: String) -> CString {
        CString::new(s).unwrap()
    }
}

//TODO DOCUMENT WHAT THIS DOES
#[macro_export]
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

/* commented out to avoid compiler warnings
macro_rules! check_useful_opt_c_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(opt_val) => opt_val,
            Err(_) => return $e
        };
    }
}
*/
