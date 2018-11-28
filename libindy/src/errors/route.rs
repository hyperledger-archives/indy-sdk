use api::ErrorCode;
use errors::ToErrorCode;

use std::error;
use std::fmt;

use errors::common::CommonError;

#[derive(Debug)]
pub enum AgentError {
    EncryptionError(String),

    DecodeError(String),
    UnpackError(String),
    PackError(String),
    SerializationError(String),
    CommonError(CommonError)
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AgentError::UnpackError(ref description) => write!(f, "Failed while unpacking message: {}", description),
            AgentError::PackError(ref description) => write!(f, "Failed while packing message: {}", description),
            AgentError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AgentError {
    fn description(&self) -> &str {
        match *self {
            AgentError::UnpackError(ref description) => description,
            AgentError::PackError(ref description) => description,
            AgentError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AgentError::UnpackError(_) => None,
            AgentError::PackError(_) => None,
            AgentError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for AgentError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            AgentError::UnpackError(_) => ErrorCode::RouteUnpackError,
            AgentError::PackError(_) => ErrorCode::RoutePackError,
            AgentError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for AgentError {
    fn from(err: CommonError) -> AgentError {
        AgentError::CommonError(err)
    }
}