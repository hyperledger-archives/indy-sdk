extern crate serde_json;
extern crate indy_crypto;

use std::error;
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
    AccessFailed(String),
    CommonError(CommonError)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidHandle(ref description) => write!(f, "Invalid wallet handle was passed: {}", description),
            WalletError::UnknownType(ref description) => write!(f, "Unknown wallet type: {}", description),
            WalletError::TypeAlreadyRegistered(ref description) => write!(f, "Wallet type already registered: {}", description),
            WalletError::AlreadyExists(ref description) => write!(f, "Wallet with this name already exists: {}", description),
            WalletError::NotFound(ref description) => write!(f, "Wallet not found: {}", description),
            WalletError::IncorrectPool(ref description) => write!(f, "Wallet used with different pool: {}", description),
            WalletError::PluggedWallerError(err_code) => write!(f, "Plugged wallet error: {}", err_code as i32),
            WalletError::AlreadyOpened(ref description) => write!(f, "Wallet already opened: {}", description),
            WalletError::AccessFailed(ref description) => write!(f, "Wallet security error: {}", description),
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
            WalletError::PluggedWallerError(_) => "Plugged wallet error",
            WalletError::AlreadyOpened(ref description) => description,
            WalletError::AccessFailed(ref description) => description,
            WalletError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::InvalidHandle(_) => None,
            WalletError::UnknownType(_) => None,
            WalletError::TypeAlreadyRegistered(_) => None,
            WalletError::AlreadyExists(_) => None,
            WalletError::NotFound(_) => None,
            WalletError::IncorrectPool(_) => None,
            WalletError::PluggedWallerError(_) => None,
            WalletError::AlreadyOpened(_) => None,
            WalletError::AccessFailed(_) => None,
            WalletError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for WalletError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            WalletError::InvalidHandle(_) => ErrorCode::WalletInvalidHandle,
            WalletError::UnknownType(_) => ErrorCode::WalletUnknownTypeError,
            WalletError::TypeAlreadyRegistered(_) => ErrorCode::WalletTypeAlreadyRegisteredError,
            WalletError::AlreadyExists(_) => ErrorCode::WalletAlreadyExistsError,
            WalletError::NotFound(_) => ErrorCode::WalletNotFoundError,
            WalletError::IncorrectPool(_) => ErrorCode::WalletIncompatiblePoolError,
            WalletError::PluggedWallerError(err_code) => err_code,
            WalletError::AlreadyOpened(_) => ErrorCode::WalletAlreadyOpenedError,
            WalletError::AccessFailed(_) => ErrorCode::WalletAccessFailed,
            WalletError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::CommonError(CommonError::IOError((err)))
    }
}

impl From<indy_crypto::errors::IndyCryptoError> for WalletError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        WalletError::CommonError(CommonError::from(err))
    }
}