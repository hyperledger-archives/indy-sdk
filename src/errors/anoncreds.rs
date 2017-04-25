use std::error;
use std::fmt;

use errors::crypto::CryptoError;
use errors::wallet::WalletError;
use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum AnoncredsError {
    NotIssuedError(String),
    MasterSecretDuplicateNameError(String),
    ProofRejected(String),
    CryptoError(CryptoError),
    WalletError(WalletError),
    CommonError(CommonError)
}

impl fmt::Display for AnoncredsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => write!(f, "Not issued: {}", description),
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => write!(f, "Dupplicated master secret: {}", description),
            AnoncredsError::ProofRejected(ref description) => write!(f, "Proof rejected: {}", description),
            AnoncredsError::CryptoError(ref err) => err.fmt(f),
            AnoncredsError::WalletError(ref err) => err.fmt(f),
            AnoncredsError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AnoncredsError {
    fn description(&self) -> &str {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => description,
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => description,
            AnoncredsError::ProofRejected(ref description) => description,
            AnoncredsError::CryptoError(ref err) => err.description(),
            AnoncredsError::WalletError(ref err) => err.description(),
            AnoncredsError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => None,
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => None,
            AnoncredsError::ProofRejected(ref description) => None,
            AnoncredsError::CryptoError(ref err) => Some(err),
            AnoncredsError::WalletError(ref err) => Some(err),
            AnoncredsError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for AnoncredsError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => ErrorCode::AnoncredsNotIssuedError,
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => ErrorCode::AnoncredsMasterSecretDuplicateNameError,
            AnoncredsError::ProofRejected(ref description) => ErrorCode::ProofRejected,
            AnoncredsError::CryptoError(ref err) => err.to_error_code(),
            AnoncredsError::WalletError(ref err) => err.to_error_code(),
            AnoncredsError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CryptoError> for AnoncredsError {
    fn from(err: CryptoError) -> AnoncredsError {
        AnoncredsError::CryptoError(err)
    }
}

impl From<WalletError> for AnoncredsError {
    fn from(err: WalletError) -> AnoncredsError {
        AnoncredsError::WalletError(err)
    }
}

impl From<CommonError> for AnoncredsError {
    fn from(err: CommonError) -> AnoncredsError {
        AnoncredsError::CommonError(err)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}