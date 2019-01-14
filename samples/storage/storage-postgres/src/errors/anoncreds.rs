extern crate serde_json;
extern crate indy_crypto;

use std::error;
use std::fmt;

use errors::prelude::*;

use api::ErrorCode;
use errors::ToErrorCode;

use self::indy_crypto::errors::IndyCryptoError;

#[derive(Debug)]
pub enum AnoncredsError {
    MasterSecretDuplicateNameError(String),
    ProofRejected(String),
    RevocationRegistryFull(String),
    InvalidUserRevocId(String),
    CredentialRevoked(String),
    CredDefAlreadyExists(String),
    CommonError(CommonError)
}

impl fmt::Display for AnoncredsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => write!(f, "Duplicated master secret: {}", description),
            AnoncredsError::ProofRejected(ref description) => write!(f, "Proof rejected: {}", description),
            AnoncredsError::RevocationRegistryFull(ref description) => write!(f, "Revocation registry is full: {}", description),
            AnoncredsError::InvalidUserRevocId(ref description) => write!(f, "Invalid revocation id: {}", description),
            AnoncredsError::CredentialRevoked(ref description) => write!(f, "Credential revoked: {}", description),
            AnoncredsError::CredDefAlreadyExists(ref description) => write!(f, "Credential definition already exists: {}", description),
            AnoncredsError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AnoncredsError {
    fn description(&self) -> &str {
        match *self {
            AnoncredsError::MasterSecretDuplicateNameError(ref description) |
            AnoncredsError::ProofRejected(ref description) |
            AnoncredsError::RevocationRegistryFull(ref description) |
            AnoncredsError::InvalidUserRevocId(ref description) => description,
            AnoncredsError::CredentialRevoked(ref description) => description,
            AnoncredsError::CredDefAlreadyExists(ref description) => description,
            AnoncredsError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AnoncredsError::MasterSecretDuplicateNameError(_) |
            AnoncredsError::ProofRejected(_) |
            AnoncredsError::RevocationRegistryFull(_) |
            AnoncredsError::InvalidUserRevocId(_) => None,
            AnoncredsError::CredentialRevoked(_) => None,
            AnoncredsError::CredDefAlreadyExists(_) => None,
            AnoncredsError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for AnoncredsError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            AnoncredsError::MasterSecretDuplicateNameError(_) => ErrorCode::AnoncredsMasterSecretDuplicateNameError,
            AnoncredsError::ProofRejected(_) => ErrorCode::AnoncredsProofRejected,
            AnoncredsError::RevocationRegistryFull(_) => ErrorCode::AnoncredsRevocationRegistryFullError,
            AnoncredsError::InvalidUserRevocId(_) => ErrorCode::AnoncredsInvalidUserRevocId,
            AnoncredsError::CredentialRevoked(_) => ErrorCode::AnoncredsCredentialRevoked,
            AnoncredsError::CredDefAlreadyExists(_) => ErrorCode::AnoncredsCredDefAlreadyExistsError,
            AnoncredsError::CommonError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for AnoncredsError {
    fn from(err: CommonError) -> AnoncredsError {
        AnoncredsError::CommonError(err)
    }
}

impl From<indy_crypto::errors::IndyCryptoError> for AnoncredsError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        match err {
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(err) => AnoncredsError::RevocationRegistryFull(err),
            IndyCryptoError::AnoncredsProofRejected(err) => AnoncredsError::ProofRejected(err),
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(err) => AnoncredsError::InvalidUserRevocId(err),
            IndyCryptoError::AnoncredsCredentialRevoked(err) => AnoncredsError::CredentialRevoked(err),
            err => AnoncredsError::CommonError(CommonError::from(err))
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}