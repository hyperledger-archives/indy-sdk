use std::error;
use std::io;
use std::fmt;
use std::num;

use errors::crypto::CryptoError;
use errors::pool::PoolError;
use errors::wallet::WalletError;

#[derive(Debug)]
pub enum LedgerError {
    CryptoError(CryptoError),
    PoolError(PoolError),
    WalletError(WalletError),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::CryptoError(ref err) => err.fmt(f),
            LedgerError::PoolError(ref err) => err.fmt(f),
            LedgerError::WalletError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::CryptoError(ref err) => err.description(),
            LedgerError::PoolError(ref err) => err.description(),
            LedgerError::WalletError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::CryptoError(ref err) => Some(err),
            LedgerError::PoolError(ref err) => Some(err),
            LedgerError::WalletError(ref err) => Some(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // TODO: FIXME: Provide tests!
}