use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum CryptoError {
    InvalidData(String)
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CryptoError::InvalidData(ref description) => write!(f, "Invalid data: {}", description)
        }
    }
}

impl error::Error for CryptoError {
    fn description(&self) -> &str {
        match *self {
            CryptoError::InvalidData(ref description) => &description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CryptoError::InvalidData(ref description) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn crypto_error_can_be_created() {
        let error = CryptoError::InvalidData("TEST".to_string());
    }

    #[test]
    fn crypto_error_can_be_formatted() {
        let error_formatted = format!("{}", CryptoError::InvalidData("TEST".to_string()));
        assert_eq!("Invalid data: TEST", error_formatted);
    }
}