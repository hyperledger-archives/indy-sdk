use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum WalletError {
    Io(io::Error)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::Io(err)
    }
}