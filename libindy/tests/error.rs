extern crate libc;
extern crate indyrs as indy;

use std::ptr;
use std::ffi::CStr;
use std::str::Utf8Error;

use self::libc::c_char;

#[test]
fn get_last_error_works_for_no_error() {
    let mut error_json_p: *const c_char = ptr::null();

    unsafe { indy_get_last_error(0, &mut error_json_p); }
    assert_eq!(None, c_str_to_string(error_json_p).unwrap());
}

#[test]
fn get_last_error_works_for_error_occurred() {
    let mut error_json_p: *const c_char = ptr::null();

    unsafe { indy_set_runtime_config(ptr::null()) };

    unsafe { indy_get_last_error(0, &mut error_json_p); }
    assert!(c_str_to_string(error_json_p).unwrap().is_some());
}

extern {
    #[no_mangle]
    pub fn indy_set_runtime_config(config: *const c_char) -> i32;

    #[no_mangle]
    pub fn indy_get_last_error(command_handle: i32,
                               error_json: *mut *const c_char) -> i32;
}

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