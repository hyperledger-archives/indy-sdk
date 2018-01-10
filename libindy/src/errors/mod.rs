pub mod anoncreds;
pub mod common;
pub mod ledger;
pub mod pool;
pub mod crypto;
pub mod indy;
pub mod wallet;
pub mod did;

use api::ErrorCode;

pub trait ToErrorCode {
    fn to_error_code(&self) -> ErrorCode;
}