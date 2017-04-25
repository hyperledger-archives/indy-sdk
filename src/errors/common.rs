use std::error;
use std::fmt;

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
    InvalidStructure(String)
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
            CommonError::InvalidStructure(ref description) => write!(f, "Invalid object: {}", description)
        }
    }
}

impl error::Error for CommonError {
    fn description(&self) -> &str {
        match *self {
            CommonError::InvalidParam1(ref description) => description,
            CommonError::InvalidParam2(ref description) => description,
            CommonError::InvalidParam3(ref description) => description,
            CommonError::InvalidParam4(ref description) => description,
            CommonError::InvalidParam5(ref description) => description,
            CommonError::InvalidParam6(ref description) => description,
            CommonError::InvalidParam7(ref description) => description,
            CommonError::InvalidParam8(ref description) => description,
            CommonError::InvalidParam9(ref description) => description,
            CommonError::InvalidState(ref description) => description,
            CommonError::InvalidStructure(ref description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CommonError::InvalidParam1(ref description) => None,
            CommonError::InvalidParam2(ref description) => None,
            CommonError::InvalidParam3(ref description) => None,
            CommonError::InvalidParam4(ref description) => None,
            CommonError::InvalidParam5(ref description) => None,
            CommonError::InvalidParam6(ref description) => None,
            CommonError::InvalidParam7(ref description) => None,
            CommonError::InvalidParam8(ref description) => None,
            CommonError::InvalidParam9(ref description) => None,
            CommonError::InvalidState(ref description) => None,
            CommonError::InvalidStructure(ref description) => None
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
            CommonError::InvalidStructure(ref description) => ErrorCode::InvalidStructure
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}