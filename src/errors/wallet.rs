use std::error;
use std::error::Error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum WalletError {
    WalletUnknownType(String),
    WalletTypeAlreadyRegistered(String),
    WalletNotFound(String),
    WalletInvalidDataFormat(String),
    WalletIncorrectPool(String),
    WalletInvalidConfig(String),
    WalletIOError(io::Error),
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::WalletUnknownType(ref description) => write!(f, "Unknown wallet type: {}", description),
            WalletError::WalletTypeAlreadyRegistered(ref description) => write!(f, "Wallet type already registered: {}", description),
            WalletError::WalletNotFound(ref description) => write!(f, "Wallet not found: {}", description),
            WalletError::WalletInvalidDataFormat(ref description) => write!(f, "Invalid format of wallet data: {}", description),
            WalletError::WalletIncorrectPool(ref description) => write!(f, "Wallet used with different pool: {}", description),
            WalletError::WalletInvalidConfig(ref description) => write!(f, "Invalid wallet config: {}", description),
            WalletError::WalletIOError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::WalletUnknownType(ref description) => description,
            WalletError::WalletTypeAlreadyRegistered(ref description) => description,
            WalletError::WalletNotFound(ref description) => description,
            WalletError::WalletInvalidDataFormat(ref description) => description,
            WalletError::WalletIncorrectPool(ref description) => description,
            WalletError::WalletInvalidConfig(ref description) => description,
            WalletError::WalletIOError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::WalletUnknownType(ref description) => None,
            WalletError::WalletTypeAlreadyRegistered(ref description) => None,
            WalletError::WalletNotFound(ref description) => None,
            WalletError::WalletInvalidDataFormat(ref description) => None,
            WalletError::WalletIncorrectPool(ref description) => None,
            WalletError::WalletInvalidConfig(ref description) => None,
            WalletError::WalletIOError(ref err) => Some(err)
        }
    }
}