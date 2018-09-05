extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;
use std::ffi::CString;

pub struct CStringUtils {}

impl CStringUtils {
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
}

macro_rules! check_useful_c_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => val.to_string(),
            _ => return $e,
        };

        if $x.is_empty() {
            return $e
        }
    }
}

macro_rules! check_useful_opt_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match CStringUtils::c_str_to_string($x) {
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

macro_rules! check_request {
    ($x:ident) => {
        let $x: Value = match serde_json::from_str(&$x) {
            Ok(Some(val)) => val,
            _ => {
                trace!("indy_sign_and_submit_request: could not parse request as valid json: {:?}", $x);
                return ErrorCode::CommonInvalidParam4;
            }
        };
        if !$x.is_object() {
        trace!("indy_sign_and_submit_request: request json is not an object: {:?}", $x);
        return ErrorCode::CommonInvalidParam4;
        }
        if !$x["req_id"].is_null() {
            trace!("indy_sign_and_submit_request: request json is not an object: {:?}", $x);
            return ErrorCode::CommonInvalidParam4;
        }
    }
}


macro_rules! check_useful_json {
    ($x:ident, $e:expr, $t:ty) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => return $e,
        };

        parse_json!($x, $e, $t);
    }
}

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

macro_rules! check_useful_c_str_empty_accepted {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => val.to_string(),
            _ => return $e,
        };
    }
}

macro_rules! check_useful_opt_c_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(opt_val) => opt_val.map(String::from),
            Err(_) => return $e
        };
    }
}