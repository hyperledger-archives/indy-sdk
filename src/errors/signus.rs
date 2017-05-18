extern crate serde_json;

use std::error;
use std::fmt;
use std::str;

use errors::crypto::CryptoError;
use errors::ledger::LedgerError;
use errors::pool::PoolError;
use errors::wallet::WalletError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum SignusError {
    CryptoError(CryptoError),
    PoolError(PoolError),
    WalletError(WalletError),
    LedgerError(LedgerError),
}

impl fmt::Display for SignusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignusError::CryptoError(ref err) => err.fmt(f),
            SignusError::PoolError(ref err) => err.fmt(f),
            SignusError::WalletError(ref err) => err.fmt(f),
            SignusError::LedgerError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SignusError {
    fn description(&self) -> &str {
        match *self {
            SignusError::CryptoError(ref err) => err.description(),
            SignusError::PoolError(ref err) => err.description(),
            SignusError::WalletError(ref err) => err.description(),
            SignusError::LedgerError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SignusError::CryptoError(ref err) => Some(err),
            SignusError::PoolError(ref err) => Some(err),
            SignusError::WalletError(ref err) => Some(err),
            SignusError::LedgerError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for SignusError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            SignusError::CryptoError(ref err) => err.to_error_code(),
            SignusError::PoolError(ref err) => err.to_error_code(),
            SignusError::WalletError(ref err) => err.to_error_code(),
            SignusError::LedgerError(ref err) => err.to_error_code()
        }
    }
}

impl From<serde_json::Error> for SignusError {
    fn from(err: serde_json::Error) -> SignusError {
        SignusError::CryptoError(CryptoError::InvalidStructure(err.to_string()))
    }
}

impl From<WalletError> for SignusError {
    fn from(err: WalletError) -> SignusError {
        SignusError::WalletError(err)
    }
}

impl From<PoolError> for SignusError {
    fn from(err: PoolError) -> SignusError {
        SignusError::PoolError(err)
    }
}

impl From<LedgerError> for SignusError {
    fn from(err: LedgerError) -> SignusError {
        SignusError::LedgerError(err)
    }
}

impl From<CryptoError> for SignusError {
    fn from(err: CryptoError) -> SignusError {
        SignusError::CryptoError(err)
    }
}

impl From<str::Utf8Error> for SignusError {
    fn from(err: str::Utf8Error) -> SignusError {
        SignusError::CryptoError(CryptoError::InvalidStructure(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}