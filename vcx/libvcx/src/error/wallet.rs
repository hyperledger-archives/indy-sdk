use std::fmt;
use error::ToErrorCode;
use utils::error::{
    INVALID_WALLET_CREATION,
    INVALID_WALLET_HANDLE,
    WALLET_ALREADY_EXISTS,
    INVALID_JSON,
    IOERROR,
    WALLET_RECORD_NOT_FOUND,
    INVALID_WALLET_STORAGE_PARAMETER,
};

#[derive(Debug)]
pub enum WalletError {
    InvalidWalletCreation(),
    InvalidParamters(),
    InvalidHandle(),
    InvalidJson(),
    IoError(),
    DuplicateWallet(String),
    RecordNotFound(),
    CommonError(u32),
}

impl ToErrorCode for WalletError {
    fn to_error_code(&self) -> u32 {
        match *self {
            WalletError::InvalidWalletCreation() => INVALID_WALLET_CREATION.code_num,
            WalletError::InvalidHandle() => INVALID_WALLET_HANDLE.code_num,
            WalletError::DuplicateWallet(_) => WALLET_ALREADY_EXISTS.code_num,
            WalletError::InvalidJson() => INVALID_JSON.code_num,
            WalletError::IoError() => IOERROR.code_num,
            WalletError::InvalidParamters() => INVALID_WALLET_STORAGE_PARAMETER.code_num,
            WalletError::RecordNotFound() => WALLET_RECORD_NOT_FOUND.code_num,
            WalletError::CommonError(x) => x,
        }
    }
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidWalletCreation() => write!(f, "{}", INVALID_WALLET_CREATION.message),
            WalletError::InvalidHandle() => write!(f, "{}", INVALID_WALLET_HANDLE.message),
            WalletError::IoError() => write!(f, "{}", IOERROR.message),
            WalletError::DuplicateWallet(ref s) => write!(f, "{}", s),
            WalletError::InvalidJson() => write!(f, "{}", INVALID_JSON.message),
            WalletError::InvalidParamters() => write!(f, "{}", INVALID_WALLET_STORAGE_PARAMETER.message),
            WalletError::RecordNotFound() => write!(f, "{}", WALLET_RECORD_NOT_FOUND.message),
            WalletError::CommonError(x) => write!(f, "This Wallet Common Error had a value of {}", x),
        }
    }
}

impl PartialEq for WalletError {
    fn eq(&self, other: &WalletError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

impl From<u32> for WalletError {
    fn from(ec: u32) -> WalletError {
        match ec {
            114 => WalletError::IoError(),
            200 => WalletError::InvalidWalletCreation(),
            1067 => WalletError::InvalidParamters(),
            e => WalletError::CommonError(e),
        }
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
