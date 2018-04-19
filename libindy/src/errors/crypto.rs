extern crate serde_json;

use std::error;
use std::fmt;
use std::str;

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum CryptoError {
    UnknownCryptoError(String),
    CommonError(CommonError)
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CryptoError::UnknownCryptoError(ref description) => write!(f, "Unknown crypto: {}", description),
            CryptoError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for CryptoError {
    fn description(&self) -> &str {
        match *self {
            CryptoError::UnknownCryptoError(ref description) => description,
            CryptoError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CryptoError::UnknownCryptoError(_) => None,
            CryptoError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for CryptoError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            CryptoError::UnknownCryptoError(_) => ErrorCode::UnknownCryptoTypeError,
            CryptoError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for CryptoError {
    fn from(err: CommonError) -> CryptoError {
        CryptoError::CommonError(err)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}