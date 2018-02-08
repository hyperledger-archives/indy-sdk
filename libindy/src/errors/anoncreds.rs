extern crate serde_json;
extern crate indy_crypto;

use std::error;
use std::fmt;

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

use self::indy_crypto::errors::IndyCryptoError;

#[derive(Debug)]
pub enum AnoncredsError {
    NotIssuedError(String),
    MasterSecretDuplicateNameError(String),
    ProofRejected(String),
    RevocationRegistryFull(String),
    InvalidUserRevocIndex(String),
    AccumulatorIsFull(String),
    ClaimRevoked(String),
    ClaimDefAlreadyExists(String),
    CommonError(CommonError)
}

impl fmt::Display for AnoncredsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => write!(f, "Not issued: {}", description),
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => write!(f, "Dupplicated master secret: {}", description),
            AnoncredsError::ProofRejected(ref description) => write!(f, "Proof rejected: {}", description),
            AnoncredsError::RevocationRegistryFull(ref description) => write!(f, "Revocation registry is full: {}", description),
            AnoncredsError::InvalidUserRevocIndex(ref description) => write!(f, "Invalid revocation index: {}", description),
            AnoncredsError::AccumulatorIsFull(ref description) => write!(f, "Accumulator is full: {}", description),
            AnoncredsError::ClaimRevoked(ref description) => write!(f, "Claim revoked: {}", description),
            AnoncredsError::ClaimDefAlreadyExists(ref description) => write!(f, "Claim definition already exists: {}", description),
            AnoncredsError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AnoncredsError {
    fn description(&self) -> &str {
        match *self {
            AnoncredsError::NotIssuedError(ref description) |
            AnoncredsError::MasterSecretDuplicateNameError(ref description) |
            AnoncredsError::ProofRejected(ref description) |
            AnoncredsError::RevocationRegistryFull(ref description) |
            AnoncredsError::InvalidUserRevocIndex(ref description) => description,
            AnoncredsError::AccumulatorIsFull(ref description) => description,
            AnoncredsError::ClaimRevoked(ref description) => description,
            AnoncredsError::ClaimDefAlreadyExists(ref description) => description,
            AnoncredsError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AnoncredsError::NotIssuedError(ref description) |
            AnoncredsError::MasterSecretDuplicateNameError(ref description) |
            AnoncredsError::ProofRejected(ref description) |
            AnoncredsError::RevocationRegistryFull(ref description) |
            AnoncredsError::InvalidUserRevocIndex(ref description) => None,
            AnoncredsError::AccumulatorIsFull(ref description) => None,
            AnoncredsError::ClaimRevoked(ref description) => None,
            AnoncredsError::ClaimDefAlreadyExists(ref description) => None,
            AnoncredsError::CommonError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for AnoncredsError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            AnoncredsError::NotIssuedError(ref description) => ErrorCode::AnoncredsNotIssuedError,
            AnoncredsError::MasterSecretDuplicateNameError(ref description) => ErrorCode::AnoncredsMasterSecretDuplicateNameError,
            AnoncredsError::ProofRejected(ref description) => ErrorCode::AnoncredsProofRejected,
            AnoncredsError::RevocationRegistryFull(ref description) => ErrorCode::AnoncredsRevocationRegistryFullError,
            AnoncredsError::InvalidUserRevocIndex(ref description) => ErrorCode::AnoncredsInvalidUserRevocIndex,
            AnoncredsError::AccumulatorIsFull(ref description) => ErrorCode::AnoncredsAccumulatorIsFull,
            AnoncredsError::ClaimRevoked(ref description) => ErrorCode::AnoncredsClaimRevoked,
            AnoncredsError::ClaimDefAlreadyExists(ref description) => ErrorCode::AnoncredsClaimDefAlreadyExistsError,
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
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(err) => AnoncredsError::AccumulatorIsFull(err),
            IndyCryptoError::AnoncredsProofRejected(err) => AnoncredsError::ProofRejected(err),
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(err) => AnoncredsError::InvalidUserRevocIndex(err),
            IndyCryptoError::AnoncredsClaimRevoked(err) => AnoncredsError::ClaimRevoked(err),
            _ => AnoncredsError::CommonError(CommonError::InvalidStructure("Invalid error code".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!
}