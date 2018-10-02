pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod non_secrets;
pub mod pairwise;
pub mod payments;
pub mod pool;
pub mod wallet;

use std::os::raw::c_char;

pub type BString = *const u8;
pub type CString = *const c_char;
pub type Handle = i32;
pub type Error = i32;

pub type ResponseEmptyCB = extern fn(xcommand_handle: Handle, err: Error);
pub type ResponseBoolCB = extern fn(xcommand_handle: Handle, err: Error, bool1: u8);
pub type ResponseI32CB = extern fn(xcommand_handle: Handle, err: Error, handle: Handle);
pub type ResponseI32UsizeCB = extern fn(xcommand_handle: Handle, err: Error, handle: Handle, total_count: usize);
pub type ResponseStringCB = extern fn(xcommand_handle: Handle, err: Error, str1: CString);
pub type ResponseStringStringCB = extern fn(xcommand_handle: Handle, err: Error, str1: CString, str2: CString);
pub type ResponseStringStringStringCB = extern fn(xcommand_handle: Handle, err: Error, str1: CString, str2: CString, str3: CString);
pub type ResponseSliceCB = extern fn(xcommand_handle: Handle, err: Error, raw: BString, len: u32);
pub type ResponseStringSliceCB = extern fn(xcommand_handle: Handle, err: Error, str1: CString, raw: BString, len: u32);
pub type ResponseStringStringU64CB = extern fn(xcommand_handle: Handle, err: Error, arg1: CString, arg2: CString, arg3: u64);
