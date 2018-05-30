use std::fmt;
use error::ToErrorCode;
use utils::error::{
    INVALID_WALLET_CREATION,
    INVALID_WALLET_HANDLE,
    WALLET_ALREADY_EXISTS,
};

#[derive(Debug)]
pub enum WalletError {
    InvalidWalletCreation(),
    InvalidHandle(),
    DuplicateWallet(String),
    CommonError(u32),
}

impl ToErrorCode for WalletError {
    fn to_error_code(&self) -> u32 {
        match *self {
            WalletError::InvalidWalletCreation() => INVALID_WALLET_CREATION.code_num,
            WalletError::InvalidHandle() => INVALID_WALLET_HANDLE.code_num,
            WalletError::DuplicateWallet(_) => WALLET_ALREADY_EXISTS.code_num,
            WalletError::CommonError(x) => x,
        }
    }
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidWalletCreation() => write!(f, "{}", INVALID_WALLET_CREATION.code_num),
            WalletError::InvalidHandle() => write!(f, "{}", INVALID_WALLET_HANDLE.code_num),
            WalletError::DuplicateWallet(ref s) => write!(f, "{}", s),
            WalletError::CommonError(x) => write!(f, "This Wallet Common Error had a value of {}", x),
        }
    }
}

impl PartialEq for WalletError {
    fn eq(&self, other: &WalletError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_err () {
        assert_eq!(WalletError::InvalidHandle().to_error_code(), INVALID_WALLET_HANDLE.code_num);
    }

}
