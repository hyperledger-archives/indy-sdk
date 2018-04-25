extern crate zmq;
extern crate indy_crypto;

use std::cell::{BorrowError, BorrowMutError};
use std::error::Error;
use std::{fmt, io};

use api::ErrorCode;
use errors::ToErrorCode;

use self::indy_crypto::errors::IndyCryptoError;
use openssl::error::ErrorStack;


#[derive(Debug)]
pub enum CommonError {
    InvalidParam1(String),
    InvalidParam2(String),
    InvalidParam3(String),
    InvalidParam4(String),
    InvalidParam5(String),
    InvalidParam6(String),
    InvalidParam7(String),
    InvalidParam8(String),
    InvalidParam9(String),
    InvalidState(String),
    InvalidStructure(String),
    IOError(io::Error),
}

impl Clone for CommonError {
    fn clone(&self) -> CommonError {
        match self {
            &CommonError::InvalidParam1(ref err) => CommonError::InvalidParam1(err.to_string()),
            &CommonError::InvalidParam2(ref err) => CommonError::InvalidParam2(err.to_string()),
            &CommonError::InvalidParam3(ref err) => CommonError::InvalidParam3(err.to_string()),
            &CommonError::InvalidParam4(ref err) => CommonError::InvalidParam4(err.to_string()),
            &CommonError::InvalidParam5(ref err) => CommonError::InvalidParam5(err.to_string()),
            &CommonError::InvalidParam6(ref err) => CommonError::InvalidParam6(err.to_string()),
            &CommonError::InvalidParam7(ref err) => CommonError::InvalidParam7(err.to_string()),
            &CommonError::InvalidParam8(ref err) => CommonError::InvalidParam8(err.to_string()),
            &CommonError::InvalidParam9(ref err) => CommonError::InvalidParam9(err.to_string()),
            &CommonError::InvalidState(ref err) => CommonError::InvalidState(err.to_string()),
            &CommonError::InvalidStructure(ref err) => CommonError::InvalidStructure(err.to_string()),
            &CommonError::IOError(ref err) => CommonError::IOError(io::Error::new(err.kind(), err.description()))
        }
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommonError::InvalidParam1(ref description) => write!(f, "Invalid param 1: {}", description),
            CommonError::InvalidParam2(ref description) => write!(f, "Invalid param 2: {}", description),
            CommonError::InvalidParam3(ref description) => write!(f, "Invalid param 3: {}", description),
            CommonError::InvalidParam4(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam5(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam6(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam7(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam8(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam9(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidState(ref description) => write!(f, "Invalid library state: {}", description),
            CommonError::InvalidStructure(ref description) => write!(f, "Invalid structure: {}", description),
            CommonError::IOError(ref err) => err.fmt(f)
        }
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        match *self {
            CommonError::InvalidParam1(ref description) |
            CommonError::InvalidParam2(ref description) |
            CommonError::InvalidParam3(ref description) |
            CommonError::InvalidParam4(ref description) |
            CommonError::InvalidParam5(ref description) |
            CommonError::InvalidParam6(ref description) |
            CommonError::InvalidParam7(ref description) |
            CommonError::InvalidParam8(ref description) |
            CommonError::InvalidParam9(ref description) |
            CommonError::InvalidState(ref description) |
            CommonError::InvalidStructure(ref description) => description,
            CommonError::IOError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CommonError::InvalidParam1(_) |
            CommonError::InvalidParam2(_) |
            CommonError::InvalidParam3(_) |
            CommonError::InvalidParam4(_) |
            CommonError::InvalidParam5(_) |
            CommonError::InvalidParam6(_) |
            CommonError::InvalidParam7(_) |
            CommonError::InvalidParam8(_) |
            CommonError::InvalidParam9(_) |
            CommonError::InvalidState(_) |
            CommonError::InvalidStructure(_) => None,
            CommonError::IOError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for CommonError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            CommonError::InvalidParam1(_) => ErrorCode::CommonInvalidParam1,
            CommonError::InvalidParam2(_) => ErrorCode::CommonInvalidParam2,
            CommonError::InvalidParam3(_) => ErrorCode::CommonInvalidParam3,
            CommonError::InvalidParam4(_) => ErrorCode::CommonInvalidParam4,
            CommonError::InvalidParam5(_) => ErrorCode::CommonInvalidParam5,
            CommonError::InvalidParam6(_) => ErrorCode::CommonInvalidParam6,
            CommonError::InvalidParam7(_) => ErrorCode::CommonInvalidParam7,
            CommonError::InvalidParam8(_) => ErrorCode::CommonInvalidParam8,
            CommonError::InvalidParam9(_) => ErrorCode::CommonInvalidParam9,
            CommonError::InvalidState(_) => ErrorCode::CommonInvalidState,
            CommonError::InvalidStructure(_) => ErrorCode::CommonInvalidStructure,
            CommonError::IOError(_) => ErrorCode::CommonIOError
        }
    }
}

impl From<io::Error> for CommonError {
    fn from(err: io::Error) -> Self {
        CommonError::IOError(err)
    }
}

impl From<zmq::Error> for CommonError {
    fn from(err: zmq::Error) -> Self {
        CommonError::IOError(From::from(err))
    }
}

impl From<BorrowError> for CommonError {
    fn from(err: BorrowError) -> Self {
        CommonError::InvalidState(err.description().to_string())
    }
}

impl From<BorrowMutError> for CommonError {
    fn from(err: BorrowMutError) -> Self {
        CommonError::InvalidState(err.description().to_string())
    }
}

impl From<ErrorStack> for CommonError {
    fn from(err: ErrorStack) -> CommonError {
        // TODO: FIXME: Analyze ErrorStack and split invalid structure errors from other errors
        CommonError::InvalidStructure(err.description().to_string())
    }
}

impl From<indy_crypto::errors::IndyCryptoError> for CommonError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        match err {
            IndyCryptoError::InvalidParam1(err) => CommonError::InvalidParam1(err),
            IndyCryptoError::InvalidParam2(err) => CommonError::InvalidParam2(err),
            IndyCryptoError::InvalidParam3(err) => CommonError::InvalidParam3(err),
            IndyCryptoError::InvalidParam4(err) => CommonError::InvalidParam4(err),
            IndyCryptoError::InvalidParam5(err) => CommonError::InvalidParam5(err),
            IndyCryptoError::InvalidParam6(err) => CommonError::InvalidParam6(err),
            IndyCryptoError::InvalidParam7(err) => CommonError::InvalidParam7(err),
            IndyCryptoError::InvalidParam8(err) => CommonError::InvalidParam8(err),
            IndyCryptoError::InvalidParam9(err) => CommonError::InvalidParam9(err),
            IndyCryptoError::InvalidState(err) => CommonError::InvalidState(err),
            IndyCryptoError::InvalidStructure(err) => CommonError::InvalidStructure(err),
            IndyCryptoError::IOError(err) => CommonError::IOError(err),
            _ => CommonError::InvalidStructure("Invalid error code".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}
