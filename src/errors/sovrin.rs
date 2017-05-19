use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use errors::ledger::LedgerError;
use errors::pool::PoolError;
use errors::signus::SignusError;
use errors::wallet::WalletError;

use api::ErrorCode;
use errors::ToErrorCode;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum SovrinError {
    AnoncredsError(AnoncredsError),
    CommonError(CommonError),
    LedgerError(LedgerError),
    PoolError(PoolError),
    SignusError(SignusError),
    WalletError(WalletError),
}

impl fmt::Display for SovrinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SovrinError::AnoncredsError(ref err) => err.fmt(f),
            SovrinError::CommonError(ref err) => err.fmt(f),
            SovrinError::LedgerError(ref err) => err.fmt(f),
            SovrinError::PoolError(ref err) => err.fmt(f),
            SovrinError::SignusError(ref err) => err.fmt(f),
            SovrinError::WalletError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SovrinError {
    fn description(&self) -> &str {
        match *self {
            SovrinError::AnoncredsError(ref err) => err.description(),
            SovrinError::CommonError(ref err) => err.description(),
            SovrinError::LedgerError(ref err) => err.description(),
            SovrinError::PoolError(ref err) => err.description(),
            SovrinError::SignusError(ref err) => err.description(),
            SovrinError::WalletError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SovrinError::AnoncredsError(ref err) => Some(err),
            SovrinError::CommonError(ref err) => Some(err),
            SovrinError::LedgerError(ref err) => Some(err),
            SovrinError::PoolError(ref err) => Some(err),
            SovrinError::SignusError(ref err) => Some(err),
            SovrinError::WalletError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for SovrinError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            SovrinError::AnoncredsError(ref err) => err.to_error_code(),
            SovrinError::CommonError(ref err) => err.to_error_code(),
            SovrinError::LedgerError(ref err) => err.to_error_code(),
            SovrinError::PoolError(ref err) => err.to_error_code(),
            SovrinError::SignusError(ref err) => err.to_error_code(),
            SovrinError::WalletError(ref err) => err.to_error_code()
        }
    }
}

impl From<AnoncredsError> for SovrinError {
    fn from(err: AnoncredsError) -> SovrinError {
        SovrinError::AnoncredsError(err)
    }
}

impl From<CommonError> for SovrinError {
    fn from(err: CommonError) -> SovrinError {
        SovrinError::CommonError(err)
    }
}

impl From<PoolError> for SovrinError {
    fn from(err: PoolError) -> SovrinError {
        SovrinError::PoolError(err)
    }
}

impl From<WalletError> for SovrinError {
    fn from(err: WalletError) -> SovrinError {
        SovrinError::WalletError(err)
    }
}

impl From<LedgerError> for SovrinError {
    fn from(err: LedgerError) -> SovrinError {
        SovrinError::LedgerError(err)
    }
}

impl From<SignusError> for SovrinError {
    fn from(err: SignusError) -> SovrinError {
        SovrinError::SignusError(err)
    }
}