use std::error;
use std::io;
use std::fmt;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum WalletError {
    InvalidHandle(String),
    UnknownType(String),
    TypeAlreadyRegistered(String),
    AlreadyExists(String),
    NotFound(String),
    InvalidDataFormat(String),
    IncorrectPool(String),
    InvalidConfig(String),
    BackendError(String),
    IOError(io::Error)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidHandle(ref description) => write!(f, "Invalid wallet handle was passed: {}", description),
            WalletError::UnknownType(ref description) => write!(f, "Unknown wallet type: {}", description),
            WalletError::TypeAlreadyRegistered(ref description) => write!(f, "Wallet type already registered: {}", description),
            WalletError::AlreadyExists(ref description) => write!(f, "Wallet with this name already exists: {}", description),
            WalletError::NotFound(ref description) => write!(f, "Wallet not found: {}", description),
            WalletError::InvalidDataFormat(ref description) => write!(f, "Invalid format of wallet data: {}", description),
            WalletError::IncorrectPool(ref description) => write!(f, "Wallet used with different pool: {}", description),
            WalletError::InvalidConfig(ref description) => write!(f, "Invalid wallet config: {}", description),
            WalletError::BackendError(ref description) => write!(f, "Invalid wallet config: {}", description),
            WalletError::IOError(ref err) => err.fmt(f)
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
            WalletError::InvalidDataFormat(ref description) => description,
            WalletError::IncorrectPool(ref description) => description,
            WalletError::InvalidConfig(ref description) => description,
            WalletError::BackendError(ref description) => description,
            WalletError::IOError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::InvalidHandle(ref description) => None,
            WalletError::UnknownType(ref description) => None,
            WalletError::TypeAlreadyRegistered(ref description) => None,
            WalletError::AlreadyExists(ref description) => None,
            WalletError::NotFound(ref description) => None,
            WalletError::InvalidDataFormat(ref description) => None,
            WalletError::IncorrectPool(ref description) => None,
            WalletError::InvalidConfig(ref description) => None,
            WalletError::BackendError(ref description) => None,
            WalletError::IOError(ref err) => Some(err)
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
            WalletError::NotFound(ref err) => ErrorCode::WalletNotFoundError,
            WalletError::InvalidDataFormat(ref err) => ErrorCode::WalletInvalidDataFormat,
            WalletError::IncorrectPool(ref err) => ErrorCode::WalletIncompatiblePoolError,
            WalletError::InvalidConfig(ref err) => ErrorCode::WalletInvalidConfiguration,
            WalletError::BackendError(ref err) => ErrorCode::WalletBackendError,
            WalletError::IOError(ref err) => ErrorCode::WalletIOError
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::IOError(err)
    }
}