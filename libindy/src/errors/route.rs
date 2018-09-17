use api::ErrorCode;
use errors::ToErrorCode;

use std::error;
use std::fmt;

use errors::common::CommonError;

#[derive(Debug)]
pub enum RouteError {
    EncryptionError(String),
    EncodeError(String),
    DecodeError(String),
    UnpackError(String),
    PackError(String),
    MissingKeyError(String),
    SerializationError(String),
    TableError(String),
    CommonError(CommonError)
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RouteError::EncryptionError(ref description) => write!(f, "Did already exists: {}", description),
            RouteError::EncodeError(ref description) => write!(f, "Failed while encoding: {}", description),
            RouteError::DecodeError(ref description) => write!(f, "Failed while decoding: {}", description),
            RouteError::UnpackError(ref description) => write!(f, "Failed while unpacking message: {}", description),
            RouteError::PackError(ref description) => write!(f, "Failed while packing message: {}", description),
            RouteError::MissingKeyError(ref description) => write!(f, "Invalid key usage: {}", description),
            RouteError::SerializationError(ref description) => write!(f, "Failed to serialize: {}", description),
            RouteError::TableError(ref description) => write!(f, "Error with Route Table: {}", description),
            RouteError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for RouteError {
    fn description(&self) -> &str {
        match *self {
            RouteError::EncryptionError(ref description) => description,
            RouteError::EncodeError(ref description) => description,
            RouteError::DecodeError(ref description) => description,
            RouteError::UnpackError(ref description) => description,
            RouteError::PackError(ref description) => description,
            RouteError::MissingKeyError(ref description) => description,
            RouteError::SerializationError(ref description) => description,
            RouteError::TableError(ref description) => description,
            RouteError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RouteError::EncryptionError(_) => None,
            RouteError::EncodeError(_) => None,
            RouteError::DecodeError(_) => None,
            RouteError::UnpackError(_) => None,
            RouteError::PackError(_) => None,
            RouteError::MissingKeyError(_) => None,
            RouteError::SerializationError(_) => None,
            RouteError::TableError(_) => None,
            RouteError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for RouteError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            RouteError::EncryptionError(_) => ErrorCode::RouteEncryptionError,
            RouteError::EncodeError(_) => ErrorCode::RouteEncodeError,
            RouteError::DecodeError(_) => ErrorCode::RouteDecodeError,
            RouteError::UnpackError(_) => ErrorCode::RouteUnpackError,
            RouteError::PackError(_) => ErrorCode::RoutePackError,
            RouteError::MissingKeyError(_) => ErrorCode::RouteMissingKeyError,
            RouteError::SerializationError(_) => ErrorCode::RouteSerializationError,
            RouteError::TableError(_) => ErrorCode::RouteTableError,
            RouteError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for RouteError {
    fn from(err: CommonError) -> RouteError {
        RouteError::CommonError(err)
    }
}