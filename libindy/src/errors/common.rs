extern crate zmq_pw as zmq;
extern crate indy_crypto;

use std::cell::{BorrowError, BorrowMutError};
use std::error::Error;
use std::{fmt, io};

use api::ErrorCode;
use errors::ToErrorCode;

use self::indy_crypto::errors::ToErrorCode as IndyCryptoToErrorCode;

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
            CommonError::InvalidStructure(ref description) => None,
            CommonError::IOError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for CommonError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            CommonError::InvalidParam1(ref description) => ErrorCode::CommonInvalidParam1,
            CommonError::InvalidParam2(ref description) => ErrorCode::CommonInvalidParam2,
            CommonError::InvalidParam3(ref description) => ErrorCode::CommonInvalidParam3,
            CommonError::InvalidParam4(ref description) => ErrorCode::CommonInvalidParam4,
            CommonError::InvalidParam5(ref description) => ErrorCode::CommonInvalidParam5,
            CommonError::InvalidParam6(ref description) => ErrorCode::CommonInvalidParam6,
            CommonError::InvalidParam7(ref description) => ErrorCode::CommonInvalidParam7,
            CommonError::InvalidParam8(ref description) => ErrorCode::CommonInvalidParam8,
            CommonError::InvalidParam9(ref description) => ErrorCode::CommonInvalidParam9,
            CommonError::InvalidState(ref description) => ErrorCode::CommonInvalidState,
            CommonError::InvalidStructure(ref description) => ErrorCode::CommonInvalidStructure,
            CommonError::IOError(ref description) => ErrorCode::CommonIOError
        }
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

impl From<indy_crypto::errors::IndyCryptoError> for CommonError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        match err.to_error_code() as i32 {
            code if code == ErrorCode::CommonInvalidParam1 as i32 => CommonError::InvalidParam1(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam2 as i32 => CommonError::InvalidParam2(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam3 as i32 => CommonError::InvalidParam3(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam4 as i32 => CommonError::InvalidParam4(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam5 as i32 => CommonError::InvalidParam5(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam6 as i32 => CommonError::InvalidParam6(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam7 as i32 => CommonError::InvalidParam7(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam8 as i32 => CommonError::InvalidParam8(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidParam9 as i32 => CommonError::InvalidParam9(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidState as i32 => CommonError::InvalidState(err.description().to_string()),
            code if code == ErrorCode::CommonInvalidStructure as i32 => CommonError::InvalidStructure(err.description().to_string()),
            _ => CommonError::InvalidStructure("Invalid error code".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}