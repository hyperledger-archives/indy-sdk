use std::fmt;
use utils::error::{INVALID_CREDENTIAL_JSON, NOT_READY, INVALID_ISSUER_CREDENTIAL_HANDLE, INVALID_CREDENTIAL_REQUEST, INVALID_JSON};
use error::ToErrorCode;
#[derive(Debug)]
pub enum IssuerCredError {
    CommonError(u32),
    NotReadyError(),
    InvalidHandle(),
    InvalidCredRequest(),
    InvalidCred(),
    InvalidJson(),
}

impl fmt::Display for IssuerCredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IssuerCredError::CommonError(x) => write!(f, "This Common Error had value: {}", x),
            IssuerCredError::NotReadyError() => write!(f, "{}", NOT_READY.message),
            IssuerCredError::InvalidHandle() => write!(f, "{}", INVALID_ISSUER_CREDENTIAL_HANDLE.message),
            IssuerCredError::InvalidCredRequest() => write!(f, "{}", INVALID_CREDENTIAL_REQUEST.message),
            IssuerCredError::InvalidCred() => write!(f, "{}", INVALID_CREDENTIAL_JSON.message),
            IssuerCredError::InvalidJson() => write!(f, "{}", INVALID_JSON.message),
        }
    }
}
impl ToErrorCode for IssuerCredError {
    fn to_error_code(&self) -> u32 {
        match *self {
            IssuerCredError::NotReadyError() => NOT_READY.code_num,
            IssuerCredError::InvalidHandle() => INVALID_ISSUER_CREDENTIAL_HANDLE.code_num,
            IssuerCredError::InvalidCredRequest() => INVALID_CREDENTIAL_REQUEST.code_num,
            IssuerCredError::InvalidJson() => INVALID_JSON.code_num,
            IssuerCredError::InvalidCred() => INVALID_CREDENTIAL_JSON.code_num,
            IssuerCredError::CommonError(x) => x,
        }
    }
}

impl PartialEq for IssuerCredError {
    fn eq(&self, other: &IssuerCredError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}
