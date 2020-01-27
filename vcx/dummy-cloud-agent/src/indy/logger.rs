use indyrs::{IndyError, logger};

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), IndyError> {
    logger::set_default_logger(pattern)
}