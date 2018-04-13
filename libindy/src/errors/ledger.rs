extern crate serde_json;

use std::error;
use std::fmt;

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum LedgerError {
    #[allow(dead_code)]
    NoConsensus(String),
    InvalidTransaction(String),
    CommonError(CommonError)
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::NoConsensus(ref description) => write!(f, "No consensus: {}", description),
            LedgerError::InvalidTransaction(ref description) => write!(f, "Invalid transaction: {}", description),
            LedgerError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::NoConsensus(ref description) => description,
            LedgerError::InvalidTransaction(ref description) => description,
            LedgerError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::NoConsensus(_) => None,
            LedgerError::InvalidTransaction(_) => None,
            LedgerError::CommonError(ref err) => Some(err)
        }
    }
}

impl From<CommonError> for LedgerError {
    fn from(err: CommonError) -> Self {
        LedgerError::CommonError(err)
    }
}

impl ToErrorCode for LedgerError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            LedgerError::NoConsensus(_) => ErrorCode::LedgerNoConsensusError,
            LedgerError::InvalidTransaction(_) => ErrorCode::LedgerInvalidTransaction,
            LedgerError::CommonError(ref err) => err.to_error_code()
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO: FIXME: Provide tests!
}