#[cfg(any(feature = "bn_openssl", feature = "hash_openssl"))]
extern crate openssl;

extern crate serde_json;

use std::error;
use std::fmt;
use std::error::Error;
use api::ErrorCode;
use errors::ToErrorCode;
use std::num::ParseIntError;

#[cfg(any(feature = "bn_openssl", feature = "hash_openssl"))]
use self::openssl::error::ErrorStack;

#[derive(Debug)]
pub enum CryptoError {
    InvalidStructure(String),
    UnknownType(String),
    RevocationRegistryFull(String),
    InvalidUserRevocIndex(String),
    BackendError(String)
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CryptoError::InvalidStructure(ref description) => write!(f, "Invalid crypto structure: {}", description),
            CryptoError::UnknownType(ref description) => write!(f, "Unknown crypto type: {}", description),
            CryptoError::RevocationRegistryFull(ref description) => write!(f, "Crypto revocation registry is full: {}", description),
            CryptoError::InvalidUserRevocIndex(ref description) => write!(f, "Crypto invalid revocation index: {}", description),
            CryptoError::BackendError(ref description) => write!(f, "Crypto backend error {}", description)
        }
    }
}

impl error::Error for CryptoError {
    fn description(&self) -> &str {
        match *self {
            CryptoError::InvalidStructure(ref description) => description,
            CryptoError::UnknownType(ref description) => description,
            CryptoError::RevocationRegistryFull(ref description) => description,
            CryptoError::InvalidUserRevocIndex(ref description) => description,
            CryptoError::BackendError(ref description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CryptoError::InvalidStructure(ref description) => None,
            CryptoError::UnknownType(ref description) => None,
            CryptoError::RevocationRegistryFull(ref description) => None,
            CryptoError::InvalidUserRevocIndex(ref description) => None,
            CryptoError::BackendError(ref err) => None
        }
    }
}

impl ToErrorCode for CryptoError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            CryptoError::InvalidStructure(ref description) => ErrorCode::CryptoInvalidStructure,
            CryptoError::UnknownType(ref description) => ErrorCode::CryptoUnknownTypeError,
            CryptoError::RevocationRegistryFull(ref description) => ErrorCode::CryptoRevocationRegistryFullError,
            CryptoError::InvalidUserRevocIndex(ref err) => ErrorCode::CryptoInvalidUserRevocIndex,
            CryptoError::BackendError(ref err) => ErrorCode::CryptoBackendError,
        }
    }
}

#[cfg(any(feature = "bn_openssl", feature = "hash_openssl"))]
impl From<ErrorStack> for CryptoError {
    fn from(err: ErrorStack) -> CryptoError {
        CryptoError::BackendError(err.description().to_string())
    }
}

impl From<ParseIntError> for CryptoError {
    fn from(err: ParseIntError) -> CryptoError {
        CryptoError::BackendError(err.description().to_string())
    }
}

impl From<serde_json::Error> for CryptoError {
    fn from(err: serde_json::Error) -> CryptoError {
        CryptoError::BackendError(err.description().to_string())
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    //    #[test]
    //    fn crypto_error_can_be_created() {
    //        let error = CryptoError::InvalidData("TEST".to_string());
    //    }
    //
    //    #[test]
    //    fn crypto_error_can_be_formatted() {
    //        let error_formatted = format!("{}", CryptoError::InvalidData("TEST".to_string()));
    //        assert_eq!("Invalid data: TEST", error_formatted);
    //    }
}