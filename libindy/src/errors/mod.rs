pub mod anoncreds;
pub mod common;
pub mod ledger;
pub mod pool;
pub mod signus;
pub mod indy;
pub mod wallet;

pub mod authz;

pub mod sss;

use api::ErrorCode;

pub trait ToErrorCode {
    fn to_error_code(&self) -> ErrorCode;
}