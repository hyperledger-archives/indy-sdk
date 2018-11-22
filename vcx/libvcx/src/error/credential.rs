
use std::fmt;
use error::ToErrorCode;
use utils::error::{NO_PAYMENT_INFORMATION, INVALID_CREDENTIAL_HANDLE, INVALID_STATE, NOT_READY, INVALID_JSON};
use error::payment;

#[derive(Debug)]
pub enum CredentialError {
    NotReady(),
    InvalidHandle(),
    InvalidCredentialJson(),
    InvalidState(),
    PaymentError(payment::PaymentError),
    NoPaymentInformation(),
    CommonError(u32),
}

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredentialError::InvalidState() => write!(f, "{}", INVALID_STATE.message),
            CredentialError::NotReady() => write!(f, "{}", NOT_READY.message),
            CredentialError::InvalidHandle() => write!(f, "{}", INVALID_CREDENTIAL_HANDLE.message),
            CredentialError::InvalidCredentialJson() => write!(f, "{}", INVALID_JSON.message),
            CredentialError::PaymentError(ref pe) => write!(f, "Payment Error: {}", pe.to_error_code()),
            CredentialError::NoPaymentInformation() => write!(f, "{}", NO_PAYMENT_INFORMATION.message),
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
            CredentialError::InvalidState() => INVALID_STATE.code_num,
            CredentialError::NotReady() => NOT_READY.code_num,
            CredentialError::InvalidHandle() => INVALID_CREDENTIAL_HANDLE.code_num,
            CredentialError::InvalidCredentialJson() => INVALID_JSON.code_num,
            CredentialError::PaymentError(ref pe) => pe.to_error_code(),
            CredentialError::NoPaymentInformation() => NO_PAYMENT_INFORMATION.code_num,
            CredentialError::CommonError(x) => x,
        }
    }
}

impl From<payment::PaymentError> for CredentialError {
    fn from(error:payment::PaymentError) -> Self {
        CredentialError::PaymentError(error)
    }
}