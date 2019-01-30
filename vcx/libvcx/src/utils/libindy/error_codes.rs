extern crate num_traits;

use indy::IndyError;
use utils::error;
use std::ffi::NulError;
use self::num_traits::int::PrimInt;

pub fn map_indy_error<T, C: PrimInt>(rtn: T, error_code: C) -> Result<T, u32> {
    if error_code == C::zero() {
        return Ok(rtn);
    }

    Err(map_indy_error_code(error_code))
}

// Todo - this will replace map_indy_error_code once we stop using our own indy cbs and move everything to rust-indy-sdk
// Todo - rename once it replaces map_indy_error
pub fn map_rust_indy_sdk_error(error_code: IndyError) -> u32 {
    let error_code = error_code.error_code as u32;
    if error_code >= error::UNKNOWN_ERROR.code_num {
        return error_code;
    }

    warn!("indy-sdk error code: {}", error_code);

    match error_code {
        100 ... 111 => error::INVALID_LIBINDY_PARAM.code_num,
        113 => error::LIBINDY_INVALID_STRUCTURE.code_num,
        114 => error::IOERROR.code_num,
        200 => error::INVALID_WALLET_HANDLE.code_num,
        203 => error::WALLET_ALREADY_EXISTS.code_num,
        204 => error::WALLET_NOT_FOUND.code_num,
        206 => error::WALLET_ALREADY_OPEN.code_num,
        212 => error::WALLET_RECORD_NOT_FOUND.code_num,
        213 => error::DUPLICATE_WALLET_RECORD.code_num,
        306 => error::CREATE_POOL_CONFIG.code_num,
        404 => error::DUPLICATE_MASTER_SECRET.code_num,
        407 => error::CREDENTIAL_DEF_ALREADY_CREATED.code_num,
        600 => error::DID_ALREADY_EXISTS_IN_WALLET.code_num,
        702 => error::INSUFFICIENT_TOKEN_AMOUNT.code_num,

        _ => error::UNKNOWN_LIBINDY_ERROR.code_num
    }
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
        100 ... 111 => error::INVALID_LIBINDY_PARAM.code_num,
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
        _ => error::UNKNOWN_LIBINDY_ERROR.code_num
    }
}

pub fn map_string_error(err: NulError) -> u32 {
    error!("Invalid String: {:?}", err);
    error::UNKNOWN_LIBINDY_ERROR.code_num
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use indy::ErrorCode;

    #[test]
    fn test_invalid_param_err() {
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

        assert_eq!(map_rust_indy_sdk_error(err100), error::INVALID_LIBINDY_PARAM.code_num);
        assert_eq!(map_rust_indy_sdk_error(err107), error::INVALID_LIBINDY_PARAM.code_num);
        assert_eq!(map_rust_indy_sdk_error(err111), error::INVALID_LIBINDY_PARAM.code_num);
        // Test that RC 112 falls out of the range 100...112
        assert_ne!(map_rust_indy_sdk_error(err112), error::INVALID_LIBINDY_PARAM.code_num);
    }
}
