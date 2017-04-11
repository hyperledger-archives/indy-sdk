use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum CryptoError {
    CryptoInvalidStructure(String),
    CryptoUnknownType(String),
    CryptoRevocationRegistryFull(String),
    CryptoInvalidUserRevocIndex(String),
    CryptoBackendError(String)
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CryptoError::CryptoInvalidStructure(ref description) => write!(f, "Invalid crypto structure: {}", description),
            CryptoError::CryptoUnknownType(ref description) => write!(f, "Unknown crypto type: {}", description),
            CryptoError::CryptoRevocationRegistryFull(ref description) => write!(f, "Crypto revocation registry is full: {}", description),
            CryptoError::CryptoInvalidUserRevocIndex(ref description) => write!(f, "Crypto invalid revocation index: {}", description),
            CryptoError::CryptoBackendError(ref description) => write!(f, "Crypto backend error {}", description)
        }
    }
}

impl error::Error for CryptoError {
    fn description(&self) -> &str {
        match *self {
            CryptoError::CryptoInvalidStructure(ref description) => description,
            CryptoError::CryptoUnknownType(ref description) => description,
            CryptoError::CryptoRevocationRegistryFull(ref description) => description,
            CryptoError::CryptoInvalidUserRevocIndex(ref description) => description,
            CryptoError::CryptoBackendError(ref description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CryptoError::CryptoInvalidStructure(ref description) => None,
            CryptoError::CryptoUnknownType(ref description) => None,
            CryptoError::CryptoRevocationRegistryFull(ref description) => None,
            CryptoError::CryptoInvalidUserRevocIndex(ref description) => None,
            CryptoError::CryptoBackendError(ref err) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

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