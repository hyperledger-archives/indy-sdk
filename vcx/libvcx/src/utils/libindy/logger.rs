extern crate log;

use indy::logger;
use error::prelude::*;

pub fn set_logger(logger: &'static dyn log::Log) -> VcxResult<()> {
    logger::set_logger(logger)
        .map_err(VcxError::from)
}

pub fn set_default_logger(patter: Option<&str>) -> VcxResult<()> {
    logger::set_default_logger(patter)
        .map_err(VcxError::from)
}