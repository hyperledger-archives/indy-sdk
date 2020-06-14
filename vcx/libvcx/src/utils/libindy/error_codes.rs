extern crate num_traits;

use self::num_traits::int::PrimInt;

use indy::IndyError;
use utils::error;
use error::prelude::{VcxError, VcxErrorKind};

impl From<IndyError> for VcxError {
    fn from(error: IndyError) -> Self {
        match error.error_code as u32 {
            100..=111 => VcxError::from_msg(VcxErrorKind::InvalidLibindyParam, error.message),
            113 => VcxError::from_msg(VcxErrorKind::LibindyInvalidStructure, error.message),
            114 => VcxError::from_msg(VcxErrorKind::IOError, error.message),
            200 => VcxError::from_msg(VcxErrorKind::InvalidWalletHandle, error.message),
            203 => VcxError::from_msg(VcxErrorKind::DuplicationWallet, error.message),
            204 => VcxError::from_msg(VcxErrorKind::WalletNotFound, error.message),
            206 => VcxError::from_msg(VcxErrorKind::WalletAlreadyOpen, error.message),
            212 => VcxError::from_msg(VcxErrorKind::WalletRecordNotFound, error.message),
            213 => VcxError::from_msg(VcxErrorKind::DuplicationWalletRecord, error.message),
            306 => VcxError::from_msg(VcxErrorKind::CreatePoolConfig, error.message),
            404 => VcxError::from_msg(VcxErrorKind::DuplicationMasterSecret, error.message),
            407 => VcxError::from_msg(VcxErrorKind::CredDefAlreadyCreated, error.message),
            600 => VcxError::from_msg(VcxErrorKind::DuplicationDid, error.message),
            702 => VcxError::from_msg(VcxErrorKind::InsufficientTokenAmount, error.message),
            error_code => VcxError::from_msg(VcxErrorKind::LibndyError(error_code), error.message)
        }
    }
}

pub fn map_indy_error<T, C: PrimInt>(rtn: T, error_code: C) -> Result<T, u32> {
    if error_code == C::zero() {
        return Ok(rtn);
    }

    Err(map_indy_error_code(error_code))
}

pub fn map_indy_error_code<C: PrimInt>(error_code: C) -> u32 {
    let error_code = match error_code.to_u32() {
        Some(n) => {
            error!("MAPPING ERROR: {:?} -- {}", n, error::error_message(&n));
            n
        }
        None => return error::UNKNOWN_LIBINDY_ERROR.code_num
    };

    if error_code >= error::UNKNOWN_ERROR.code_num {
        return error_code;
    }

    match error_code {
        100..=111 => error::INVALID_LIBINDY_PARAM.code_num,
        113 => error::LIBINDY_INVALID_STRUCTURE.code_num,
        200 => error::INVALID_WALLET_HANDLE.code_num,
        203 => error::WALLET_ALREADY_EXISTS.code_num,
        206 => error::WALLET_ALREADY_OPEN.code_num,
        212 => error::WALLET_RECORD_NOT_FOUND.code_num,
        213 => error::DUPLICATE_WALLET_RECORD.code_num,
        306 => error::CREATE_POOL_CONFIG.code_num,
        404 => error::DUPLICATE_MASTER_SECRET.code_num,
        407 => error::CREDENTIAL_DEF_ALREADY_CREATED.code_num,
        600 => error::DID_ALREADY_EXISTS_IN_WALLET.code_num,
        702 => error::INSUFFICIENT_TOKEN_AMOUNT.code_num,
        error_cde => error_cde
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use indy::ErrorCode;
    use utils::devsetup::SetupDefaults;

    #[test]
    fn test_invalid_param_err() {
        let _setup = SetupDefaults::init();

        let err100: IndyError = IndyError {
            error_code: ErrorCode::CommonInvalidParam1,
            message: String::new(),
            indy_backtrace: None,
        };
        let err107: IndyError = IndyError {
            error_code: ErrorCode::CommonInvalidParam8,
            message: String::new(),
            indy_backtrace: None,
        };
        let err111: IndyError = IndyError {
            error_code: ErrorCode::CommonInvalidParam12,
            message: String::new(),
            indy_backtrace: None,
        };
        let err112: IndyError = IndyError {
            error_code: ErrorCode::CommonInvalidState,
            message: String::new(),
            indy_backtrace: None,
        };

        assert_eq!(VcxError::from(err100).kind(), VcxErrorKind::InvalidLibindyParam);
        assert_eq!(VcxError::from(err107).kind(), VcxErrorKind::InvalidLibindyParam);
        assert_eq!(VcxError::from(err111).kind(), VcxErrorKind::InvalidLibindyParam);
        // Test that RC 112 falls out of the range 100...112
        assert_ne!(VcxError::from(err112).kind(), VcxErrorKind::InvalidLibindyParam);
    }
}
