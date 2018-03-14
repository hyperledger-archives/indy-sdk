extern crate serde_json;

use std::error;
use std::error::Error;
use std::io;
use std::fmt;

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum WalletError {
    InvalidHandle(String),
    UnknownType(String),
    TypeAlreadyRegistered(String),
    AlreadyExists(String),
    NotFound(String),
    IncorrectPool(String),
    PluggedWallerError(ErrorCode),
    AlreadyOpened(String),
    CommonError(CommonError)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidHandle(ref description) => write!(f, "Invalid wallet handle was passed: {}", description),
            WalletError::UnknownType(ref description) => write!(f, "Unknown wallet type: {}", description),
            WalletError::TypeAlreadyRegistered(ref description) => write!(f, "Wallet type already registered: {}", description),
            WalletError::AlreadyExists(ref description) => write!(f, "Wallet with this name already exists: {}", description),
            WalletError::NotFound(ref description) => write!(f, "Key not found in wallet: {}", description),
            WalletError::IncorrectPool(ref description) => write!(f, "Wallet used with different pool: {}", description),
            WalletError::PluggedWallerError(err_code) => write!(f, "Plugged wallet error: {}", err_code as i32),
            WalletError::AlreadyOpened(ref description) => write!(f, "Wallet already opened: {}", description),
            WalletError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::InvalidHandle(ref description) => description,
            WalletError::UnknownType(ref description) => description,
            WalletError::TypeAlreadyRegistered(ref description) => description,
            WalletError::AlreadyExists(ref description) => description,
            WalletError::NotFound(ref description) => description,
            WalletError::IncorrectPool(ref description) => description,
            WalletError::PluggedWallerError(ref err_code) => "Plugged wallet error",
            WalletError::AlreadyOpened(ref description) => description,
            WalletError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::InvalidHandle(ref description) => None,
            WalletError::UnknownType(ref description) => None,
            WalletError::TypeAlreadyRegistered(ref description) => None,
            WalletError::AlreadyExists(ref description) => None,
            WalletError::NotFound(ref description) => None,
            WalletError::IncorrectPool(ref description) => None,
            WalletError::PluggedWallerError(ref err_code) => None,
            WalletError::AlreadyOpened(ref description) => None,
            WalletError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for WalletError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            WalletError::InvalidHandle(ref description) => ErrorCode::WalletInvalidHandle,
            WalletError::UnknownType(ref description) => ErrorCode::WalletUnknownTypeError,
            WalletError::TypeAlreadyRegistered(ref description) => ErrorCode::WalletTypeAlreadyRegisteredError,
            WalletError::AlreadyExists(ref description) => ErrorCode::WalletAlreadyExistsError,
            WalletError::NotFound(ref err) => ErrorCode::KeyNotFoundInWalletError,
            WalletError::IncorrectPool(ref err) => ErrorCode::WalletIncompatiblePoolError,
            WalletError::PluggedWallerError(err_code) => err_code,
            WalletError::AlreadyOpened(ref err) => ErrorCode::WalletAlreadyOpenedError,
            WalletError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::CommonError(CommonError::IOError((err)))
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(err: serde_json::Error) -> WalletError {
        WalletError::CommonError(CommonError::InvalidStructure(err.description().to_string()))
    }
}