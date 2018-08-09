use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum BaseError {
    WalletError(String),
    ConnectionError(),
    GeneralError(),
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BaseError::GeneralError() => write!(f, "General Error"),
            BaseError::WalletError(ref s) => write!(f, "Wallet Error: {}", s),
            BaseError::ConnectionError() => write!(f, "Connection Error"),
        }
    }
}

impl Error for BaseError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            BaseError::GeneralError() => None,
            BaseError::WalletError(ref s) => None,
            BaseError::ConnectionError() => None,
        }
    }
    // TODO: Either implement this correctly or remove.
    fn description(&self) -> &str {
        match *self {
            BaseError::WalletError(ref s) => "Wallet Error",
            BaseError::GeneralError() => "General Base Error",
            BaseError::ConnectionError() => "Connection Not Ready",
        }
    }
}
