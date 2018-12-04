extern crate log;

use indy::logger;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;

pub fn set_logger(logger: &'static log::Log) -> Result<(), u32> {
    logger::set_logger(logger)
       .map_err(map_rust_indy_sdk_error_code)
}

pub fn set_default_logger(patter: Option<&str>) -> Result<(), u32> {
    logger::set_default_logger(patter)
       .map_err(map_rust_indy_sdk_error_code)
}