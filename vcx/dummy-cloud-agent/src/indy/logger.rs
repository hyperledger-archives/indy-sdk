use indyrs::{logger, ErrorCode};

pub fn set_default_logger(pattern: Option<&str>) -> Result<(), ErrorCode> {
    logger::set_default_logger(pattern)
}