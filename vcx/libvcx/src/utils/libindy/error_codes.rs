use utils::error;
use std::ffi::NulError;

pub fn map_indy_error<T>(rtn: T, error_code: i32) -> Result<T, u32> {
    if error_code == 0 {
        return Ok(rtn);
    }

    Err(map_indy_error_code(error_code))
}

pub fn map_indy_error_code(error_code: i32) -> u32 {
    match error_code {
        306 =>  error::CREATE_POOL_CONFIG.code_num,
        _ =>    error::UNKNOWN_LIBINDY_ERROR.code_num
    }
}

pub fn map_string_error(err: NulError) -> u32 {
    error!("Invalid String: {:?}", err);
    error::UNKNOWN_LIBINDY_ERROR.code_num
}