
use std::fmt;
use error::ToErrorCode;
use utils::error::{INVALID_CREDENTIAL_HANDLE, NOT_READY, INVALID_JSON};

#[derive(Debug)]
pub enum CredentialError {
    NotReady(),
    InvalidHandle(),
    InvalidCredentialJson(),
    InvalidState(),
    CommonError(u32),
}

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredentialError::InvalidState() => write!(f, "This Credential is not in proper state for this operation"),
            CredentialError::NotReady() => write!(f, "{}", NOT_READY.message),
            CredentialError::InvalidHandle() => write!(f, "{}", INVALID_CREDENTIAL_HANDLE.message),
            CredentialError::InvalidCredentialJson() => write!(f, "{}", INVALID_JSON.message),
            CredentialError::CommonError(x) => write!(f, "This Credential Error had a value: {}", x),
        }
    }
}

impl PartialEq for CredentialError{
    fn eq(&self, other: &CredentialError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

impl ToErrorCode for CredentialError {
    fn to_error_code(&self) -> u32 {
        match *self {
            CredentialError::InvalidState() => 3001,
            CredentialError::NotReady() => NOT_READY.code_num,
            CredentialError::InvalidHandle() => INVALID_CREDENTIAL_HANDLE.code_num,
            CredentialError::InvalidCredentialJson() => INVALID_JSON.code_num,
            CredentialError::CommonError(x) => x,
        }
    }
}