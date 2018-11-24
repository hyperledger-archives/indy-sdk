use std::os::raw::c_char;
use std::ptr;
use super::IndyError;

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    let err = unsafe {
        let pattern = opt_c_str!(pattern);
        indy_set_default_logger(pattern.map(|x| x.as_ptr()).unwrap_or(ptr::null()))
    };

    if err == 0 {
        Ok(())
    } else {
        Err(IndyError::from_err_code(err))
    }
}

extern {
    pub fn indy_set_default_logger(pattern: *const c_char) -> i32;
}