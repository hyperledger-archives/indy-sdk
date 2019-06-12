extern crate libc;

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
pub mod logger;
pub mod cache;

use self::libc::{c_void, c_char};

pub type CVoid = c_void;
pub type BString = *const u8;
pub type CString = *const c_char;

pub type WalletHandle = i32;
//#[repr(transparent)]
//#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
//pub struct WalletHandle(pub i32);
//pub const INVALID_WALLET_HANDLE : WalletHandle = WalletHandle(0);

//pub type Handle = i32;
pub type IndyHandle = i32;
pub type CommandHandle = i32;
pub type PoolHandle = i32;
pub type SearchHandle = i32;
pub type RecordHandle = i32;
pub type TailWriterHandle = i32;
pub type StorageHandle = i32;
pub type BlobStorageReaderHandle = i32;
pub type BlobStorageReaderCfgHandle = i32;
pub type MetadataHandle = i32;
pub type Timeout = i32;
pub type TailsWriterHandle = i32;

pub type Error = i32;

pub const INVALID_POOL_HANDLE: PoolHandle = 0;
pub const INVALID_WALLET_HANDLE: WalletHandle = 0;

pub type ResponseEmptyCB = extern fn(xcommand_handle: CommandHandle, err: Error);
pub type ResponseBoolCB = extern fn(xcommand_handle: CommandHandle, err: Error, bool1: bool);
pub type ResponseI32CB = extern fn(xcommand_handle: CommandHandle, err: Error, handle: IndyHandle);
pub type ResponseI32UsizeCB = extern fn(xcommand_handle: CommandHandle, err: Error, handle: IndyHandle, total_count: usize);
pub type ResponseStringCB = extern fn(xcommand_handle: CommandHandle, err: Error, str1: CString);
pub type ResponseStringStringCB = extern fn(xcommand_handle: CommandHandle, err: Error, str1: CString, str2: CString);
pub type ResponseStringStringStringCB = extern fn(xcommand_handle: CommandHandle, err: Error, str1: CString, str2: CString, str3: CString);
pub type ResponseSliceCB = extern fn(xcommand_handle: CommandHandle, err: Error, raw: BString, len: u32);
pub type ResponseStringSliceCB = extern fn(xcommand_handle: CommandHandle, err: Error, str1: CString, raw: BString, len: u32);
pub type ResponseStringStringU64CB = extern fn(xcommand_handle: CommandHandle, err: Error, arg1: CString, arg2: CString, arg3: u64);

extern {
    #[no_mangle]
    pub fn indy_set_runtime_config(config: CString) -> Error;

    #[no_mangle]
    pub fn indy_get_current_error(error_json_p: *mut CString);
}