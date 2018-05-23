pub mod crypto;
pub mod did;
pub mod ledger;
pub mod payments;
pub mod pool;
pub mod wallet;

use super::ErrorCode;
use std::os::raw::c_char;

pub type ResponseEmptyCB = extern fn(xcommand_handle: i32, err: ErrorCode);
pub type ResponseBoolCB = extern fn(xcommand_handle: i32, err: ErrorCode, bool1: u8);
pub type ResponseI32CB = extern fn(xcommand_handle: i32, err: ErrorCode, pool_handle: i32);
pub type ResponseStringCB = extern fn(xcommand_handle: i32, err: ErrorCode, str1: *const c_char);
pub type ResponseStringStringCB = extern fn(xcommand_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char);
pub type ResponseSliceCB = extern fn(xcommand_handle: i32, err: ErrorCode, raw: *const u8, len: u32);
pub type ResponseStringSliceCB = extern fn(xcommand_handle: i32, err: ErrorCode, str1: *const c_char, raw: *const u8, len: u32);
pub type ResponseStringStringU64CB = extern fn(xcommand_handle: i32, err: ErrorCode, arg1: *const c_char, arg2: *const c_char, arg3: u64);
