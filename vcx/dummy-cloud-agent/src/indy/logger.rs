use super::IndyError;
use indyrs::logger;

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    logger::set_default_logger(pattern)
        .map_err(|err| IndyError::from_err_code(err as i32))
}