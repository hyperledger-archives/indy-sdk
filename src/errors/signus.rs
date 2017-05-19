extern crate serde_json;

use std::error;
use std::fmt;
use std::str;

use errors::crypto::CryptoError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum SignusError {
    CryptoError(CryptoError)
}

impl fmt::Display for SignusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignusError::CryptoError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SignusError {
    fn description(&self) -> &str {
        match *self {
            SignusError::CryptoError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignusError::CryptoError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for SignusError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            SignusError::CryptoError(ref err) => err.to_error_code()
        }
    }
}

impl From<serde_json::Error> for SignusError {
    fn from(err: serde_json::Error) -> SignusError {
        SignusError::CryptoError(CryptoError::InvalidStructure(err.to_string()))
    }
}

impl From<CryptoError> for SignusError {
    fn from(err: CryptoError) -> SignusError {
        SignusError::CryptoError(err)
    }
}

impl From<str::Utf8Error> for SignusError {
    fn from(err: str::Utf8Error) -> SignusError {
        SignusError::CryptoError(CryptoError::InvalidStructure(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}