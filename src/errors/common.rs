extern crate zmq;

use std::error;
use std::fmt;
use std::io;

use api::ErrorCode;
use errors::ToErrorCode;

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

impl error::Error for CommonError {
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

    fn cause(&self) -> Option<&error::Error> {
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

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}