extern crate serde_json;

use std::error;
use std::fmt;
use std::str;

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum SignusError {
    UnknownCryptoError(String),
    CommonError(CommonError)
}

impl fmt::Display for SignusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignusError::UnknownCryptoError(ref description) => write!(f, "Unknown crypto: {}", description),
            SignusError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SignusError {
    fn description(&self) -> &str {
        match *self {
            SignusError::UnknownCryptoError(ref description) => description,
            SignusError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignusError::UnknownCryptoError(ref description) => None,
            SignusError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for SignusError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            SignusError::UnknownCryptoError(ref description) => ErrorCode::SignusUnknownCryptoError,
            SignusError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for SignusError {
    fn from(err: CommonError) -> SignusError {
        SignusError::CommonError(err)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}