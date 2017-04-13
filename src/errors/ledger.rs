use std::error;
use std::io;
use std::fmt;

use errors::crypto::CryptoError;
use errors::pool::PoolError;
use errors::wallet::WalletError;

#[derive(Debug)]
pub enum LedgerError {
    NoConsensus(String),
    Io(io::Error),
    CryptoError(CryptoError),
    PoolError(PoolError),
    WalletError(WalletError),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::NoConsensus(ref description) => write!(f, "No consensus: {}", description),
            LedgerError::Io(ref err) => err.fmt(f),
            LedgerError::CryptoError(ref err) => err.fmt(f),
            LedgerError::PoolError(ref err) => err.fmt(f),
            LedgerError::WalletError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::NoConsensus(ref description) => description,
            LedgerError::Io(ref err) => err.description(),
            LedgerError::CryptoError(ref err) => err.description(),
            LedgerError::PoolError(ref err) => err.description(),
            LedgerError::WalletError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::NoConsensus(ref description) => None,
            LedgerError::Io(ref err) => Some(err),
            LedgerError::CryptoError(ref err) => Some(err),
            LedgerError::PoolError(ref err) => Some(err),
            LedgerError::WalletError(ref err) => Some(err)
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO: FIXME: Provide tests!
}