use std::error;
use std::io;
use std::fmt;

use errors::crypto::CryptoError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum LedgerError {
    NoConsensus(String),
    Io(io::Error),
    CryptoError(CryptoError)
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::NoConsensus(ref description) => write!(f, "No consensus: {}", description),
            LedgerError::Io(ref err) => err.fmt(f),
            LedgerError::CryptoError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::NoConsensus(ref description) => description,
            LedgerError::Io(ref err) => err.description(),
            LedgerError::CryptoError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::NoConsensus(ref description) => None,
            LedgerError::Io(ref err) => Some(err),
            LedgerError::CryptoError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for LedgerError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            LedgerError::NoConsensus(ref description) => ErrorCode::LedgerNoConsensusError,
            LedgerError::Io(ref err) => ErrorCode::PoolLedgerIOError,
            LedgerError::CryptoError(ref err) => err.to_error_code()
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO: FIXME: Provide tests!
}