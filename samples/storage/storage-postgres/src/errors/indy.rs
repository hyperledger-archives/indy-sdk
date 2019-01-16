use errors::anoncreds::AnoncredsError;
use errors::prelude::*;
use errors::ledger::LedgerError;

use errors::crypto::CryptoError;
use errors::wallet::WalletError;
use errors::did::DidError;
use errors::payments::PaymentsError;

use api::ErrorCode;
use errors::ToErrorCode;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum IndyError {
    AnoncredsError(AnoncredsError),
    CommonError(CommonError),
    LedgerError(LedgerError),
    PoolError(PoolError),
    CryptoError(CryptoError),
    WalletError(WalletError),
    DidError(DidError),
    PaymentsError(PaymentsError),
}

impl fmt::Display for IndyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndyError::AnoncredsError(ref err) => err.fmt(f),
            IndyError::CommonError(ref err) => err.fmt(f),
            IndyError::LedgerError(ref err) => err.fmt(f),
            IndyError::PoolError(ref err) => err.fmt(f),
            IndyError::CryptoError(ref err) => err.fmt(f),
            IndyError::WalletError(ref err) => err.fmt(f),
            IndyError::DidError(ref err) => err.fmt(f),
            IndyError::PaymentsError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for IndyError {
    fn description(&self) -> &str {
        match *self {
            IndyError::AnoncredsError(ref err) => err.description(),
            IndyError::CommonError(ref err) => err.description(),
            IndyError::LedgerError(ref err) => err.description(),
            IndyError::PoolError(ref err) => err.description(),
            IndyError::CryptoError(ref err) => err.description(),
            IndyError::WalletError(ref err) => err.description(),
            IndyError::DidError(ref err) => err.description(),
            IndyError::PaymentsError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IndyError::AnoncredsError(ref err) => Some(err),
            IndyError::CommonError(ref err) => Some(err),
            IndyError::LedgerError(ref err) => Some(err),
            IndyError::PoolError(ref err) => Some(err),
            IndyError::CryptoError(ref err) => Some(err),
            IndyError::WalletError(ref err) => Some(err),
            IndyError::DidError(ref err) => Some(err),
            IndyError::PaymentsError(ref err) => Some(err),
        }
    }
}

impl ToErrorCode for IndyError {
    fn to_error_code(&self) -> ErrorCode {
        error!("Casting error to ErrorCode: {}", self);
        match *self {
            IndyError::AnoncredsError(ref err) => err.to_error_code(),
            IndyError::CommonError(ref err) => err.to_error_code(),
            IndyError::LedgerError(ref err) => err.to_error_code(),
            IndyError::PoolError(ref err) => err.to_error_code(),
            IndyError::CryptoError(ref err) => err.to_error_code(),
            IndyError::WalletError(ref err) => err.to_error_code(),
            IndyError::DidError(ref err) => err.to_error_code(),
            IndyError::PaymentsError(ref err) => err.to_error_code(),
        }
    }
}

impl From<AnoncredsError> for IndyError {
    fn from(err: AnoncredsError) -> IndyError {
        IndyError::AnoncredsError(err)
    }
}

impl From<CommonError> for IndyError {
    fn from(err: CommonError) -> IndyError {
        IndyError::CommonError(err)
    }
}

impl From<PoolError> for IndyError {
    fn from(err: PoolError) -> IndyError {
        IndyError::PoolError(err)
    }
}

impl From<WalletError> for IndyError {
    fn from(err: WalletError) -> IndyError {
        IndyError::WalletError(err)
    }
}

impl From<LedgerError> for IndyError {
    fn from(err: LedgerError) -> IndyError {
        IndyError::LedgerError(err)
    }
}

impl From<CryptoError> for IndyError {
    fn from(err: CryptoError) -> IndyError {
        IndyError::CryptoError(err)
    }
}

impl From<DidError> for IndyError {
    fn from(err: DidError) -> IndyError {
        IndyError::DidError(err)
    }
}

impl From<PaymentsError> for IndyError {
    fn from(err: PaymentsError) -> IndyError {
        IndyError::PaymentsError(err)
    }
}