extern crate serde_json;

use std::error;
use std::fmt;
use std::str;

use errors::prelude::*;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum DidError {
    AlreadyExistsError(String),
    CommonError(CommonError)
}

impl fmt::Display for DidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DidError::AlreadyExistsError(ref description) => write!(f, "Did already exists: {}", description),
            DidError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for DidError {
    fn description(&self) -> &str {
        match *self {
            DidError::AlreadyExistsError(ref description) => description,
            DidError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DidError::AlreadyExistsError(_) => None,
            DidError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for DidError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            DidError::AlreadyExistsError(_) => ErrorCode::DidAlreadyExistsError,
            DidError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for DidError {
    fn from(err: CommonError) -> DidError {
        DidError::CommonError(err)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}