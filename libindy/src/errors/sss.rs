extern crate serde_json;
extern crate indy_crypto;


use std::error;
use std::fmt;
use std::str;

use errors::common::CommonError;
use errors::signus::SignusError;
use errors::wallet::WalletError;
use errors::indy::IndyError;

use self::indy_crypto::errors::IndyCryptoError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum SSSError {
    MGreaterThanN(String),
    CommonError(CommonError),
    SignusError(SignusError)
}

impl fmt::Display for SSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SSSError::MGreaterThanN(ref description) => write!(f, "m greater than n: {}", description),
            SSSError::CommonError(ref err) => err.fmt(f),
            SSSError::SignusError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SSSError {
    fn description(&self) -> &str {
        match *self {
            SSSError::MGreaterThanN(ref description) => description,
            SSSError::CommonError(ref err) => err.description(),
            SSSError::SignusError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SSSError::MGreaterThanN(ref description) => None,
            SSSError::CommonError(ref err) => Some(err),
            SSSError::SignusError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for SSSError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            SSSError::MGreaterThanN(ref description) => ErrorCode::PolicyAlreadyExistsError,
            SSSError::CommonError(ref err) => err.to_error_code(),
            SSSError::SignusError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for SSSError {
    fn from(err: CommonError) -> SSSError {
        SSSError::CommonError(err)
    }
}

impl From<SignusError> for SSSError {
    fn from(err: SignusError) -> SSSError {
        SSSError::SignusError(err)
    }
}


impl From<indy_crypto::errors::IndyCryptoError> for SSSError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        match err {
            _ => SSSError::CommonError(CommonError::InvalidState("Invalid crypto error".to_string()))
        }
    }
}


impl From<WalletError> for SSSError {
    fn from(err: WalletError) -> Self {
        match err {
            _ => SSSError::CommonError(CommonError::InvalidStructure("Invalid wallet error".to_string()))
        }
    }
}

impl From<IndyError> for SSSError {
    fn from(err: IndyError) -> Self {
        match err {
            _ => SSSError::CommonError(CommonError::InvalidStructure("Invalid generic indy error".to_string()))
        }
    }
}