pub mod common;
pub mod crypto;
pub mod wallet;

use libindy::ErrorCode;

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
