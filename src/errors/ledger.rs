use std::error;
use std::io;
use std::fmt;

use errors::crypto::CryptoError;
use errors::pool::PoolError;
use errors::signus::SignusError;
use errors::wallet::WalletError;

use api::ErrorCode;
use errors::ToErrorCode;

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

impl From<WalletError> for LedgerError {
    fn from(err: WalletError) -> Self {
        LedgerError::WalletError(err)
    }
}

impl From<CryptoError> for LedgerError {
    fn from(err: CryptoError) -> Self {
        LedgerError::CryptoError(err)
    }
}

impl From<SignusError> for LedgerError {
    fn from(se: SignusError) -> Self {
        match se {
            SignusError::LedgerError(err) => err,
            SignusError::PoolError(err) => LedgerError::PoolError(err),
            SignusError::WalletError(err) => LedgerError::WalletError(err),
            SignusError::CryptoError(err) => LedgerError::CryptoError(err),
        }
    }
}

impl ToErrorCode for LedgerError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            LedgerError::NoConsensus(ref description) => ErrorCode::LedgerNoConsensusError,
            LedgerError::Io(ref err) => ErrorCode::PoolLedgerIOError,
            LedgerError::CryptoError(ref err) => err.to_error_code(),
            LedgerError::PoolError(ref err) => err.to_error_code(),
            LedgerError::WalletError(ref err) => err.to_error_code(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO: FIXME: Provide tests!
}