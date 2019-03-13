extern crate log;

use indy::logger;
use utils::libindy::error_codes::map_rust_indy_sdk_error;
use error::prelude::*;

pub fn set_logger(logger: &'static log::Log) -> VcxResult<()> {
    logger::set_logger(logger)
       .map_err(map_rust_indy_sdk_error)
}

pub fn set_default_logger(patter: Option<&str>) -> VcxResult<()> {
    logger::set_default_logger(patter)
       .map_err(map_rust_indy_sdk_error)
}