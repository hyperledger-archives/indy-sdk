use std::error;
use std::error::Error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum WalletError {
    BackendError(Box<Error>)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::BackendError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::BackendError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::BackendError(ref err) => Some(err.as_ref())
        }
    }
}