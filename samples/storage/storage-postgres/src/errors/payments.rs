use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use api::ErrorCode;
use errors::ToErrorCode;
use errors::prelude::*;

#[derive(Debug)]
pub enum PaymentsError {
    PluggedMethodError(ErrorCode),
    UnknownType(String),
    CommonError(CommonError),
    IncompatiblePaymentError(String),
}

impl Error for PaymentsError {
    fn description(&self) -> &str {
        match *self {
            PaymentsError::CommonError(ref err) => err.description(),
            PaymentsError::UnknownType(ref msg) => msg.as_str(),
            PaymentsError::PluggedMethodError(_error_code) => "Plugged method error. Consider the error code.",
            PaymentsError::IncompatiblePaymentError(ref msg) => msg.as_str(),
        }
    }
}

impl Display for PaymentsError {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        match *self {
            PaymentsError::CommonError(ref err) => err.fmt(_f),
            PaymentsError::PluggedMethodError(_err_code) => write!(_f, "Plugged method error. Consider the error code."),
            PaymentsError::UnknownType(ref msg) => write!(_f, "Unknown Type Error: {}", msg),
            PaymentsError::IncompatiblePaymentError(ref msg) => write!(_f, "Incompatible Payment Method Error: {}", msg),
        }
    }
}

impl ToErrorCode for PaymentsError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            PaymentsError::PluggedMethodError(e) => e,
            PaymentsError::CommonError(ref err) => err.to_error_code(),
            PaymentsError::UnknownType(ref _str) => ErrorCode::PaymentUnknownMethodError,
            PaymentsError::IncompatiblePaymentError(ref _str) => ErrorCode::PaymentIncompatibleMethodsError,
        }
    }
}