use super::IndyError;
use indyrs::logger::Logger as logger;

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    logger::set_default_logger(pattern)
        .map_err(|err| IndyError::from_err_code(err as i32))
}