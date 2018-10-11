use std::error::Error;
use error::ToErrorCode;
use utils::error:: { INVALID_OBJ_HANDLE, INSUFFICIENT_TOKEN_AMOUNT, INVALID_JSON };
use std::fmt;

#[derive(Debug)]
pub enum PaymentError {
    InvalidHandle(),
    InsufficientFunds(),
    InvalidWalletJson(),
    CommonError(u32),
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
        match *self {
            PaymentError::InvalidHandle() => write!(f, "{}", INVALID_OBJ_HANDLE.message),
            PaymentError::InsufficientFunds() => write!(f, "{}", INSUFFICIENT_TOKEN_AMOUNT.message),
            PaymentError::InvalidWalletJson() => write!(f, "{}", INVALID_JSON.message),
            PaymentError::CommonError(ec) => write!( f, "Libindy/Commmon Error: {}", ec),
        }
    }
}
impl Error for PaymentError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            PaymentError::InvalidHandle() => None,
            PaymentError::InsufficientFunds() => None,
            PaymentError::InvalidWalletJson() => None,
            PaymentError::CommonError(ec) => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            PaymentError::InvalidHandle() => INVALID_OBJ_HANDLE.message,
            PaymentError::InsufficientFunds() => INSUFFICIENT_TOKEN_AMOUNT.message,
            PaymentError::InvalidWalletJson() => INVALID_JSON.message,
            PaymentError::CommonError(ec) => "Libindy/Common Error",
        }
    }
}

impl ToErrorCode for PaymentError {
    fn to_error_code(&self) -> u32 {
        match *self {
            PaymentError::InvalidHandle() => INVALID_OBJ_HANDLE.code_num,
            PaymentError::InsufficientFunds() => INSUFFICIENT_TOKEN_AMOUNT.code_num,
            PaymentError::InvalidWalletJson() => INVALID_JSON.code_num,
            PaymentError::CommonError(ec) => ec,
        }
    }
}

impl PartialEq for PaymentError {
    fn eq(&self, other: &PaymentError) -> bool {self.to_error_code() == other.to_error_code() }
}

