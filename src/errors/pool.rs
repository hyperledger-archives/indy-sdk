extern crate zmq;
extern crate serde_json;

use std::error;
use std::io;
use std::fmt;
use std::error::Error;
use std::cell::{BorrowError, BorrowMutError};

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum PoolError {
    NotCreated(String),
    InvalidHandle(String),
    CommonError(CommonError)
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoolError::NotCreated(ref description) => write!(f, "Not created: {}", description),
            PoolError::InvalidHandle(ref description) => write!(f, "Invalid Handle: {}", description),
            PoolError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for PoolError {
    fn description(&self) -> &str {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::InvalidHandle(ref description) => description,
            PoolError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::InvalidHandle(ref description) => None,
            PoolError::CommonError(ref err) => Some(err)
        }
    }
}

impl From<CommonError> for PoolError {
    fn from(err: CommonError) -> PoolError {
        PoolError::CommonError(err)
    }
}

impl From<zmq::Error> for PoolError {
    fn from(err: zmq::Error) -> PoolError {
        PoolError::CommonError(CommonError::IOError(io::Error::from(err)))
    }
}

impl From<BorrowError> for PoolError {
    fn from(err: BorrowError) -> Self {
        PoolError::CommonError(CommonError::InvalidState(err.description().to_string()))
    }
}

impl From<BorrowMutError> for PoolError {
    fn from(err: BorrowMutError) -> Self {
        PoolError::CommonError(CommonError::InvalidState(err.description().to_string()))
    }
}

impl From<io::Error> for PoolError {
    fn from(err: io::Error) -> PoolError {
        PoolError::CommonError(CommonError::IOError((err)))
    }
}

impl ToErrorCode for PoolError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            PoolError::NotCreated(ref description) => ErrorCode::PoolLedgerNotCreatedError,
            PoolError::InvalidHandle(ref description) => ErrorCode::PoolLedgerInvalidPoolHandle,
            PoolError::CommonError(ref err) => err.to_error_code()
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