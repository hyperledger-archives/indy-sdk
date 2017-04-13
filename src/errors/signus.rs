use std::error;
use std::io;
use std::fmt;
use std::num;

use errors::crypto::CryptoError;
use errors::pool::PoolError;
use errors::wallet::WalletError;

#[derive(Debug)]
pub enum SignusError {
    CryptoError(CryptoError),
    PoolError(PoolError),
    WalletError(WalletError)
}

impl fmt::Display for SignusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignusError::CryptoError(ref err) => err.fmt(f),
            SignusError::PoolError(ref err) => err.fmt(f),
            SignusError::WalletError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SignusError {
    fn description(&self) -> &str {
        match *self {
            SignusError::CryptoError(ref err) => err.description(),
            SignusError::PoolError(ref err) => err.description(),
            SignusError::WalletError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignusError::CryptoError(ref err) => Some(err),
            SignusError::PoolError(ref err) => Some(err),
            SignusError::WalletError(ref err) => Some(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // TODO: FIXME: Provide tests!
}