extern crate serde_json;
extern crate indy_crypto;


use std::error;
use std::fmt;
use std::str;

use errors::common::CommonError;
use errors::crypto::CryptoError;
use self::indy_crypto::errors::IndyCryptoError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum AuthzError {
    PolicyAlreadyExistsError(String),
    PolicyDoesNotExistError(String),
    AgentDoesNotExistError(String),
    AgentAlreadyExistsError(String),
    AgentHasNoSecretError(String),
    CommonError(CommonError),
    CryptoError(CryptoError)
}


impl fmt::Display for AuthzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthzError::PolicyAlreadyExistsError(ref description) => write!(f, "Policy address does not exists: {}", description),
            AuthzError::PolicyDoesNotExistError(ref description) => write!(f, "Policy address already exists: {}", description),
            AuthzError::AgentDoesNotExistError(ref description) => write!(f, "Agent does not exist: {}", description),
            AuthzError::AgentAlreadyExistsError(ref description) => write!(f, "Agent does already exist: {}", description),
            AuthzError::AgentHasNoSecretError(ref description) => write!(f, "Agent does have secret: {}", description),
            AuthzError::CommonError(ref err) => err.fmt(f),
            AuthzError::CryptoError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for AuthzError {
    fn description(&self) -> &str {
        match *self {
            AuthzError::PolicyAlreadyExistsError(ref description) => description,
            AuthzError::PolicyDoesNotExistError(ref description) => description,
            AuthzError::AgentDoesNotExistError(ref description) => description,
            AuthzError::AgentAlreadyExistsError(ref description) => description,
            AuthzError::AgentHasNoSecretError(ref description) => description,
            AuthzError::CommonError(ref err) => err.description(),
            AuthzError::CryptoError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AuthzError::PolicyAlreadyExistsError(ref description) => None,
            AuthzError::PolicyDoesNotExistError(ref description) => None,
            AuthzError::AgentDoesNotExistError(ref description) => None,
            AuthzError::AgentAlreadyExistsError(ref description) => None,
            AuthzError::AgentHasNoSecretError(ref description) => None,
            AuthzError::CommonError(ref err) => Some(err),
            AuthzError::CryptoError(ref err) => Some(err)
        }
    }
}

impl ToErrorCode for AuthzError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            AuthzError::PolicyAlreadyExistsError(ref description) => ErrorCode::PolicyAlreadyExistsError,
            AuthzError::PolicyDoesNotExistError(ref description) => ErrorCode::PolicyAlreadyExistsError,
            AuthzError::AgentDoesNotExistError(ref description) => ErrorCode::AgentDoesNotExistError,
            AuthzError::AgentAlreadyExistsError(ref description) => ErrorCode::AgentAlreadyExistsError,
            AuthzError::AgentHasNoSecretError(ref description) => ErrorCode::AgentHasNoSecretError,
            AuthzError::CommonError(ref err) => err.to_error_code(),
            AuthzError::CryptoError(ref err) => err.to_error_code()
        }
    }
}

impl From<CommonError> for AuthzError {
    fn from(err: CommonError) -> AuthzError {
        AuthzError::CommonError(err)
    }
}

impl From<CryptoError> for AuthzError {
    fn from(err: CryptoError) -> AuthzError {
        AuthzError::CryptoError(err)
    }
}


impl From<indy_crypto::errors::IndyCryptoError> for AuthzError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        match err {
            _ => AuthzError::CommonError(CommonError::InvalidState("Invalid error code".to_string()))
        }
    }
}