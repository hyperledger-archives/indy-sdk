extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;

macro_rules! check_useful_c_callback {
    ($x:ident, $e:expr) => {
        let $x = match $x {
            Some($x) => $x,
            None => return $e
        };
    }
}