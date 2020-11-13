extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate libc;
extern crate failure;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate indy_sys as ffi;

#[macro_use]
mod macros;

pub use futures::future;
use libc::c_char;

pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod logger;
pub mod payments;
pub mod pairwise;
pub mod pool;
pub mod wallet;
pub mod cache;
pub mod metrics;
mod utils;

use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::ffi::CStr;

use failure::{Backtrace, Fail};

pub use ffi::{
    RecordHandle,
    TailWriterHandle,
    BlobStorageReaderHandle,
    BlobStorageReaderCfgHandle,
    MetadataHandle,
    Timeout,
    TailsWriterHandle,
    IndyHandle,
    CommandHandle,
    WalletHandle,
    PoolHandle,
    SearchHandle,
    StorageHandle,
    INVALID_WALLET_HANDLE,
    INVALID_POOL_HANDLE,
    INVALID_COMMAND_HANDLE
};

/// Set libindy runtime configuration. Can be optionally called to change current params.
///
/// # Arguments
/// * `config` - {
///     "crypto_thread_pool_size": <int> - size of thread pool for the most expensive crypto operations. (4 by default)
/// }
pub fn set_runtime_config(config: &str) -> ErrorCode {
    let config = c_str!(config);

    ErrorCode::from(unsafe {
        ffi::indy_set_runtime_config(config.as_ptr())
    })
}

#[derive(Fail, Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    #[fail(display = "Success")]
    Success = 0,
    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam1")]
    CommonInvalidParam1 = 100,
    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam2")]
    CommonInvalidParam2 = 101,
    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam3")]
    CommonInvalidParam3 = 102,
    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam4")]
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam5")]
    CommonInvalidParam5 = 104,
    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam6")]
    CommonInvalidParam6 = 105,
    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam7")]
    CommonInvalidParam7 = 106,
    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam8")]
    CommonInvalidParam8 = 107,
    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam9")]
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam10")]
    CommonInvalidParam10 = 109,
    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam11")]
    CommonInvalidParam11 = 110,
    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam12")]
    CommonInvalidParam12 = 111,
    // Invalid library state was detected in runtime. It signals library bug
    #[fail(display = "CommonInvalidState")]
    CommonInvalidState = 112,
    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    #[fail(display = "CommonInvalidStructure")]
    CommonInvalidStructure = 113,

    // IO Error
    #[fail(display = "CommonIOError")]
    CommonIOError = 114,
    // Caller passed invalid value as param 13 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam13")]
    CommonInvalidParam13 = 115,
    // Caller passed invalid value as param 14 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam14")]
    CommonInvalidParam14 = 116,
    // Caller passed invalid value as param 15 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam15")]
    CommonInvalidParam15 = 117,
    // Caller passed invalid value as param 16 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam16")]
    CommonInvalidParam16 = 118,

    // Caller passed invalid value as param 17 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam17")]
    CommonInvalidParam17 = 119,
    // Caller passed invalid value as param 18 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam18")]
    CommonInvalidParam18 = 120,
    // Caller passed invalid value as param 19 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam19")]
    CommonInvalidParam19 = 121,
    // Caller passed invalid value as param 20 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam20")]
    CommonInvalidParam20 = 122,
    // Caller passed invalid value as param 21 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam21")]
    CommonInvalidParam21 = 123,

    // Caller passed invalid value as param 22 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam22")]
    CommonInvalidParam22 = 124,
    // Caller passed invalid value as param 23 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam23")]
    CommonInvalidParam23 = 125,
    // Caller passed invalid value as param 24 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam24")]
    CommonInvalidParam24 = 126,
    // Caller passed invalid value as param 25 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam25")]
    CommonInvalidParam25 = 127,
    // Caller passed invalid value as param 26 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam26")]
    CommonInvalidParam26 = 128,

    // Caller passed invalid value as param 27 (null, invalid json and etc..)
    #[fail(display = "CommonInvalidParam27")]
    CommonInvalidParam27 = 129,
    // Wallet errors
    // Caller passed invalid wallet handle
    #[fail(display = "WalletInvalidHandle")]
    WalletInvalidHandle = 200,
    // Unknown type of wallet was passed on create_wallet
    #[fail(display = "WalletUnknownTypeError")]
    WalletUnknownTypeError = 201,
    // Attempt to register already existing wallet type
    #[fail(display = "WalletTypeAlreadyRegisteredError")]
    WalletTypeAlreadyRegisteredError = 202,
    // Attempt to create wallet with name used for another exists wallet
    #[fail(display = "WalletAlreadyExistsError")]
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    #[fail(display = "WalletNotFoundError")]
    WalletNotFoundError = 204,
    // Trying to use wallet with pool that has different name
    #[fail(display = "WalletIncompatiblePoolError")]
    WalletIncompatiblePoolError = 205,
    // Trying to open wallet that was opened already
    #[fail(display = "WalletAlreadyOpenedError")]
    WalletAlreadyOpenedError = 206,
    // Attempt to open encrypted wallet with invalid credentials
    #[fail(display = "WalletAccessFailed")]
    WalletAccessFailed = 207,
    // Input provided to wallet operations is considered not valid
    #[fail(display = "WalletInputError")]
    WalletInputError = 208,

    // Decoding of wallet data during input/output failed
    #[fail(display = "WalletDecodingError")]
    WalletDecodingError = 209,
    // Storage error occurred during wallet operation
    #[fail(display = "WalletStorageError")]
    WalletStorageError = 210,
    // Error during encryption-related operations
    #[fail(display = "WalletEncryptionError")]
    WalletEncryptionError = 211,
    // Requested wallet item not found
    #[fail(display = "WalletItemNotFound")]
    WalletItemNotFound = 212,
    // Returned if wallet's add_record operation is used with record name that already exists
    #[fail(display = "WalletItemAlreadyExists")]
    WalletItemAlreadyExists = 213,

    // Returned if provided wallet query is invalid
    #[fail(display = "WalletQueryError")]
    WalletQueryError = 214,
    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    #[fail(display = "PoolLedgerNotCreatedError")]
    PoolLedgerNotCreatedError = 300,
    // Caller passed invalid pool ledger handle
    #[fail(display = "PoolLedgerInvalidPoolHandle")]
    PoolLedgerInvalidPoolHandle = 301,
    // Pool ledger terminated
    #[fail(display = "PoolLedgerTerminated")]
    PoolLedgerTerminated = 302,
    // No concensus during ledger operation
    #[fail(display = "LedgerNoConsensusError")]
    LedgerNoConsensusError = 303,

    // Attempt to parse invalid transaction response
    #[fail(display = "LedgerInvalidTransaction")]
    LedgerInvalidTransaction = 304,
    // Attempt to send transaction without the necessary privileges
    #[fail(display = "LedgerSecurityError")]
    LedgerSecurityError = 305,
    // Attempt to create pool ledger config with name used for another existing pool
    #[fail(display = "PoolLedgerConfigAlreadyExistsError")]
    PoolLedgerConfigAlreadyExistsError = 306,
    // Timeout for action
    #[fail(display = "PoolLedgerTimeout")]
    PoolLedgerTimeout = 307,
    // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
    // Call pool.indy_set_protocol_version to set correct Protocol version.
    #[fail(display = "PoolIncompatibleProtocolVersion")]
    PoolIncompatibleProtocolVersion = 308,

    // Item not found on ledger.
    #[fail(display = "LedgerNotFound")]
    LedgerNotFound = 309,

    // Revocation registry is full and creation of new registry is necessary
    #[fail(display = "AnoncredsRevocationRegistryFullError")]
    AnoncredsRevocationRegistryFullError = 400,
    #[fail(display = "AnoncredsInvalidUserRevocId")]
    AnoncredsInvalidUserRevocId = 401,
    // Attempt to generate master secret with duplicated name
    #[fail(display = "AnoncredsMasterSecretDuplicateNameError")]
    AnoncredsMasterSecretDuplicateNameError = 404,
    #[fail(display = "AnoncredsProofRejected")]
    AnoncredsProofRejected = 405,
    #[fail(display = "AnoncredsCredentialRevoked")]
    AnoncredsCredentialRevoked = 406,

    // Attempt to create credential definition with duplicated did schema pair
    #[fail(display = "AnoncredsCredDefAlreadyExistsError")]
    AnoncredsCredDefAlreadyExistsError = 407,
    // Signus errors
    // Unknown format of DID entity keys
    #[fail(display = "UnknownCryptoTypeError")]
    UnknownCryptoTypeError = 500,
    // Attempt to create duplicate did
    #[fail(display = "DidAlreadyExistsError")]
    DidAlreadyExistsError = 600,
    // Unknown payment method was given
    #[fail(display = "UnknownPaymentMethod")]
    UnknownPaymentMethod = 700,
    //No method were scraped from inputs/outputs or more than one were scraped
    #[fail(display = "IncompatiblePaymentError")]
    IncompatiblePaymentError = 701,

    // Insufficient funds on inputs
    #[fail(display = "PaymentInsufficientFundsError")]
    PaymentInsufficientFundsError = 702,

    // No such source on a ledger
    #[fail(display = "PaymentSourceDoesNotExistError")]
    PaymentSourceDoesNotExistError = 703,

    // Operation is not supported for payment method
    #[fail(display = "PaymentOperationNotSupportedError")]
    PaymentOperationNotSupportedError = 704,

    // Extra funds on inputs
    #[fail(display = "PaymentExtraFundsError")]
    PaymentExtraFundsError = 705,

    // The transaction is not allowed to a requester
    #[fail(display = "The transaction is not allowed to a requester")]
    TransactionNotAllowed,
}


impl From<i32> for ErrorCode {
    fn from(i: i32) -> Self {
        let conversion = num_traits::FromPrimitive::from_i32(i);
        if conversion.is_some() {
            conversion.unwrap()
        } else {
            panic!("Unable to convert from {}, unknown error code", i)
        }
    }
}

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        num_traits::ToPrimitive::to_i32(&self).unwrap()
    }
}

#[derive(Debug)]
pub struct IndyError {
    pub error_code: ErrorCode,
    pub message: String,
    pub indy_backtrace: Option<String>
}

impl Fail for IndyError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.error_code.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> { self.error_code.backtrace() }
}

impl fmt::Display for IndyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        Ok(())
    }
}

impl IndyError {
    pub(crate) fn new(error_code: ErrorCode) -> Self {
        let mut error_json_p: *const c_char = ptr::null();

        unsafe { ffi::indy_get_current_error(&mut error_json_p); }
        let error_json = opt_rust_str!(error_json_p);

        let error_json = match error_json {
            Some(error_json_) => error_json_,
            None => {
                return IndyError {
                    error_code: ErrorCode::CommonInvalidState,
                    message: String::from("Invalid ErrorMessage pointer"),
                    indy_backtrace: None,
                };
            }
        };

        match ::serde_json::from_str::<ErrorDetails>(&error_json) {
            Ok(error) => IndyError {
                error_code,
                message: error.message,
                indy_backtrace: error.backtrace,
            },
            Err(err) => IndyError {
                error_code: ErrorCode::CommonInvalidState,
                message: err.to_string(),
                indy_backtrace: None,
            }
        }
    }
}

#[derive(Deserialize)]
struct ErrorDetails {
    message: String,
    backtrace: Option<String>
}