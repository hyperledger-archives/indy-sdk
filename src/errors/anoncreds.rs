use std::error;
use std::io;
use std::fmt;
use std::num;

use errors::crypto::CryptoError;
use errors::wallet::WalletError;

#[derive(Debug)]
pub enum AnoncredsError {
    AnoncredsNotIssuedError(String),
    AnoncredsMasterSecretDuplicateNameError(String),
    AnoncredsProofRejected(String),
    CryptoError(CryptoError),
    WalletError(WalletError),
}

impl fmt::Display for AnoncredsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnoncredsError::AnoncredsNotIssuedError(ref description) => write!(f, "Not issued: {}", description),
            AnoncredsError::AnoncredsMasterSecretDuplicateNameError(ref description) => write!(f, "Dupplicated master secret: {}", description),
            AnoncredsError::AnoncredsProofRejected(ref description) => write!(f, "Proof rejected: {}", description),
            AnoncredsError::CryptoError(ref err) => err.fmt(f),
            AnoncredsError::WalletError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AnoncredsError {
    fn description(&self) -> &str {
        match *self {
            AnoncredsError::AnoncredsNotIssuedError(ref description) => description,
            AnoncredsError::AnoncredsMasterSecretDuplicateNameError(ref description) => description,
            AnoncredsError::AnoncredsProofRejected(ref description) => description,
            AnoncredsError::CryptoError(ref err) => err.description(),
            AnoncredsError::WalletError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AnoncredsError::AnoncredsNotIssuedError(ref description) => None,
            AnoncredsError::AnoncredsMasterSecretDuplicateNameError(ref description) => None,
            AnoncredsError::AnoncredsProofRejected(ref description) => None,
            AnoncredsError::CryptoError(ref err) => Some(err),
            AnoncredsError::WalletError(ref err) => Some(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // TODO: FIXME: Provide tests!
}