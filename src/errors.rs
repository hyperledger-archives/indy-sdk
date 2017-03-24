use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SovrinClientError {
    WalletError(WalletError)
}

#[derive(Debug)]
pub enum  WalletError {
    NotFoundError,
    UnknownError(Box<Error>)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::NotFoundError => write!(f, "Not Found Error"),
            WalletError::UnknownError(ref e) => e.fmt(f),
        }
    }
}

impl PartialEq for WalletError {
    fn eq(&self, other: &WalletError) -> bool {
        self.eq(other)
    }
}