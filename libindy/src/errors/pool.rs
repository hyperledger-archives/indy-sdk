extern crate zmq;
extern crate serde_json;
extern crate indy_crypto;

use std::{error, fmt, io};

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug, Clone)]
pub enum PoolError {
    NotCreated(String),
    InvalidHandle(String),
    Terminate,
    Timeout,
    AlreadyExists(String),
    CommonError(CommonError)
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoolError::NotCreated(ref description) => write!(f, "Not created: {}", description),
            PoolError::InvalidHandle(ref description) => write!(f, "Invalid Handle: {}", description),
            PoolError::Terminate => write!(f, "Pool work terminated"),
            PoolError::Timeout => write!(f, "Timeout"),
            PoolError::AlreadyExists(ref description) => write!(f, "Pool ledger config already exists {}", description),
            PoolError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for PoolError {
    fn description(&self) -> &str {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::InvalidHandle(ref description) => description,
            PoolError::Terminate => "Pool work terminated",
            PoolError::Timeout => "Timeout",
            PoolError::AlreadyExists(ref description) => description,
            PoolError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PoolError::NotCreated(_) | PoolError::InvalidHandle(_) => None,
            PoolError::Terminate | PoolError::Timeout => None,
            PoolError::AlreadyExists(_) => None,
            PoolError::CommonError(ref err) => Some(err)
        }
    }
}

impl From<CommonError> for PoolError {
    fn from(err: CommonError) -> PoolError {
        PoolError::CommonError(err)
    }
}


impl From<io::Error> for PoolError {
    fn from(err: io::Error) -> PoolError {
        PoolError::CommonError(CommonError::IOError(err))
    }
}

impl From<zmq::Error> for PoolError {
    fn from(err: zmq::Error) -> PoolError {
        PoolError::CommonError(From::from(err))
    }
}

impl ToErrorCode for PoolError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            PoolError::NotCreated(_) => ErrorCode::PoolLedgerNotCreatedError,
            PoolError::InvalidHandle(_) => ErrorCode::PoolLedgerInvalidPoolHandle,
            PoolError::Terminate => ErrorCode::PoolLedgerTerminated,
            PoolError::Timeout => ErrorCode::PoolLedgerTimeout,
            PoolError::AlreadyExists(_) => ErrorCode::PoolLedgerConfigAlreadyExistsError,
            PoolError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<indy_crypto::errors::IndyCryptoError> for PoolError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        PoolError::CommonError(CommonError::from(err))
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!!!

    //    #[test]
    //    fn indy_error_can_be_created() {
    //        let not_created_error = PoolError::NotCreated("NotCreated".to_string());
    //        let invalid_handle_error = PoolError::InvalidHandle("InvalidHandle".to_string());
    //        let no_consensus_error = PoolError::NoConsensus("NoConsensus".to_string());
    //        let invalid_data_error = PoolError::InvalidData("InvalidData".to_string());
    //        let io_error = PoolError::Io(io::Error());
    //    }
    //
    //    #[test]
    //    fn indy_error_can_be_formatted() {
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
