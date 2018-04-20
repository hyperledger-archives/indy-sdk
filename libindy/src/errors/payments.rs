use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use api::ErrorCode;
use errors::ToErrorCode;
use errors::common::CommonError;

#[derive(Debug)]
pub enum PaymentsError {
    PluggedMethodError(ErrorCode),
    UnknownType(String),
    CommonError(CommonError),
}

impl Error for PaymentsError {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl Display for PaymentsError {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl ToErrorCode for PaymentsError {
    fn to_error_code(&self) -> ErrorCode {
        unimplemented!()
    }
}