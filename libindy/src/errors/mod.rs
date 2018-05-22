pub mod anoncreds;
pub mod common;
pub mod ledger;
pub mod pool;
pub mod crypto;
pub mod indy;
pub mod wallet;
pub mod did;
pub mod payments;

use api::ErrorCode;

pub trait ToErrorCode {
    fn to_error_code(&self) -> ErrorCode;
}

impl<T> ToErrorCode for Result<(), T> where T: ToErrorCode {
    fn to_error_code(&self) -> ErrorCode {
        match self {
            &Ok(()) => ErrorCode::Success,
            &Err(ref err) => err.to_error_code(),
        }
    }
}
