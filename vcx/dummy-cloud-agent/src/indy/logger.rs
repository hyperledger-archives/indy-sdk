use indyrs::{logger, IndyError};

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    logger::set_default_logger(pattern)
}