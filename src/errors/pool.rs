extern crate zmq;
extern crate serde_json;

use std::error;
use std::io;
use std::fmt;
use std::error::Error;
use std::cell::{BorrowError, BorrowMutError};

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum PoolError {
    NotCreated(String),
    InvalidState(String),
    InvalidData(String),
    InvalidConfiguration(String),
    InvalidHandle(String),
    Io(io::Error)
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoolError::NotCreated(ref description) => write!(f, "Not created: {}", description),
            PoolError::InvalidState(ref description) => write!(f, "Internal error: {}", description),
            PoolError::InvalidHandle(ref description) => write!(f, "Invalid Handle: {}", description),
            PoolError::InvalidConfiguration(ref description) => write!(f, "Invalid configuration: {}", description),
            PoolError::InvalidData(ref description) => write!(f, "Invalid data: {}", description),
            PoolError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for PoolError {
    fn description(&self) -> &str {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::InvalidState(ref description) |
            PoolError::InvalidHandle(ref description) |
            PoolError::InvalidData(ref description) |
            PoolError::InvalidConfiguration(ref description) => description,
            PoolError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::InvalidState(ref description) |
            PoolError::InvalidHandle(ref description) |
            PoolError::InvalidData(ref description) |
            PoolError::InvalidConfiguration(ref description) => None,
            PoolError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for PoolError {
    fn from(err: io::Error) -> PoolError {
        PoolError::Io(err)
    }
}

impl From<zmq::Error> for PoolError {
    fn from(err: zmq::Error) -> PoolError {
        PoolError::Io(io::Error::from(err))
    }
}

impl From<serde_json::Error> for PoolError {
    fn from(err: serde_json::Error) -> PoolError {
        PoolError::InvalidConfiguration(err.description().to_string())
    }
}

impl From<BorrowError> for PoolError {
    fn from(err: BorrowError) -> Self {
        PoolError::InvalidState(err.description().to_string())
    }
}

impl From<BorrowMutError> for PoolError {
    fn from(err: BorrowMutError) -> Self {
        PoolError::InvalidState(err.description().to_string())
    }
}

impl ToErrorCode for PoolError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            PoolError::NotCreated(ref description) => ErrorCode::PoolLedgerNotCreatedError,
            PoolError::InvalidState(ref description) => ErrorCode::CommonInvalidState,
            PoolError::InvalidConfiguration(ref description) => ErrorCode::PoolLedgerInvalidConfiguration,
            PoolError::InvalidHandle(ref description) => ErrorCode::PoolLedgerInvalidPoolHandle,
            PoolError::InvalidData(ref description) => ErrorCode::PoolLedgerInvalidDataFormat,
            PoolError::Io(ref err) => ErrorCode::PoolLedgerIOError
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!!!

    //    #[test]
    //    fn sovrin_error_can_be_created() {
    //        let not_created_error = PoolError::NotCreated("NotCreated".to_string());
    //        let invalid_handle_error = PoolError::InvalidHandle("InvalidHandle".to_string());
    //        let no_consensus_error = PoolError::NoConsensus("NoConsensus".to_string());
    //        let invalid_data_error = PoolError::InvalidData("InvalidData".to_string());
    //        let io_error = PoolError::Io(io::Error());
    //    }
    //
    //    #[test]
    //    fn sovrin_error_can_be_formatted() {
    //        let not_created_error_formatted = format!("{}", PoolError::NotCreated("NotCreated".to_string()));
    //        let invalid_handle_error_formatted = format!("{}", PoolError::InvalidHandle("InvalidHandle".to_string()));
    //        let no_consensus_error_formatted = format!("{}", PoolError::NoConsensus("NoConsensus".to_string()));
    //        let invalid_data_error_formatted = format!("{}", PoolError::InvalidData("InvalidData".to_string()));
    //        let io_error_formatted = format!("{}", PoolError::Io(io::Error()));
    //
    //        assert_eq!("No consensus: NotCreated", not_created_error_formatted);
    //        assert_eq!("No consensus: InvalidHandle", invalid_handle_error_formatted);
    //        assert_eq!("No consensus: NoConsensus", no_consensus_error_formatted);
    //        assert_eq!("Invalid data: InvalidData", invalid_data_error_formatted);
    //        assert_eq!("IO", io_error_formatted);
    //    }
}