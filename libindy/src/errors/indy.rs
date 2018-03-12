extern crate indy_crypto;
extern crate serde_json;

use self::indy_crypto::errors::IndyCryptoError;
use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use errors::ledger::LedgerError;
use errors::pool::PoolError;
use errors::signus::SignusError;
use errors::wallet::WalletError;

use errors::authz::AuthzError;

use api::ErrorCode;
use errors::ToErrorCode;

use std::error;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum IndyError {
    IndyCryptoError(IndyCryptoError),
    AnoncredsError(AnoncredsError),
    CommonError(CommonError),
    LedgerError(LedgerError),
    PoolError(PoolError),
    SignusError(SignusError),
    WalletError(WalletError),
    AuthzError(AuthzError),
}

impl fmt::Display for IndyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndyError::IndyCryptoError(ref err) => err.fmt(f),
            IndyError::AnoncredsError(ref err) => err.fmt(f),
            IndyError::CommonError(ref err) => err.fmt(f),
            IndyError::LedgerError(ref err) => err.fmt(f),
            IndyError::PoolError(ref err) => err.fmt(f),
            IndyError::SignusError(ref err) => err.fmt(f),
            IndyError::WalletError(ref err) => err.fmt(f),
            IndyError::AuthzError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for IndyError {
    fn description(&self) -> &str {
        match *self {
            IndyError::IndyCryptoError(ref err) => err.description(),
            IndyError::AnoncredsError(ref err) => err.description(),
            IndyError::CommonError(ref err) => err.description(),
            IndyError::LedgerError(ref err) => err.description(),
            IndyError::PoolError(ref err) => err.description(),
            IndyError::SignusError(ref err) => err.description(),
            IndyError::WalletError(ref err) => err.description(),
            IndyError::AuthzError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IndyError::IndyCryptoError(ref err) => Some(err),
            IndyError::AnoncredsError(ref err) => Some(err),
            IndyError::CommonError(ref err) => Some(err),
            IndyError::LedgerError(ref err) => Some(err),
            IndyError::PoolError(ref err) => Some(err),
            IndyError::SignusError(ref err) => Some(err),
            IndyError::WalletError(ref err) => Some(err),
            IndyError::AuthzError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for IndyError {
    fn to_error_code(&self) -> ErrorCode {
        error!("Casting error to ErrorCode: {}", self);
        match *self {
            IndyError::IndyCryptoError(ref err) => ErrorCode::CommonIOError,
            IndyError::AnoncredsError(ref err) => err.to_error_code(),
            IndyError::CommonError(ref err) => err.to_error_code(),
            IndyError::LedgerError(ref err) => err.to_error_code(),
            IndyError::PoolError(ref err) => err.to_error_code(),
            IndyError::SignusError(ref err) => err.to_error_code(),
            IndyError::WalletError(ref err) => err.to_error_code(),
            IndyError::AuthzError(ref err) => err.to_error_code()
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

impl From<SignusError> for IndyError {
    fn from(err: SignusError) -> IndyError {
        IndyError::SignusError(err)
    }
}

impl From<AuthzError> for IndyError {
    fn from(err: AuthzError) -> IndyError {
        IndyError::AuthzError(err)
    }
}

impl From<IndyCryptoError> for IndyError {
    fn from(err: IndyCryptoError) -> IndyError {
        IndyError::IndyCryptoError(err)
    }
}

impl From<serde_json::Error> for IndyError {
    fn from(err: serde_json::Error) -> Self {
        match err {
            _ => IndyError::CommonError(CommonError::InvalidStructure("Invalid json error".to_string()))
        }
    }
}

impl From<str::Utf8Error> for IndyError {
    fn from(err: str::Utf8Error) -> Self {
        match err {
            _ => IndyError::CommonError(CommonError::InvalidStructure("Invalid utf-8 string error".to_string()))
        }
    }
}
