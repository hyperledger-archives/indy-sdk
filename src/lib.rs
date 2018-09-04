#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate num_traits;
#[macro_use]
extern crate num_derive;

#[macro_use]
mod macros;

pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod payments;
pub mod pairwise;
pub mod pool;
pub mod wallet;
pub mod utils;
pub mod native;

use std::sync::mpsc;

pub type IndyHandle = i32;

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2 = 101,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3 = 102,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5 = 104,

    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6 = 105,

    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7 = 106,

    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8 = 107,

    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10 = 109,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11 = 110,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam12 = 111,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112,

    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    CommonInvalidStructure = 113,

    // IO Error
    CommonIOError = 114,

    // Caller passed invalid value as param 13 (null, invalid json and etc..)
    CommonInvalidParam13 = 115,

    // Caller passed invalid value as param 14 (null, invalid json and etc..)
    CommonInvalidParam14 = 116,

    // Caller passed invalid value as param 15 (null, invalid json and etc..)
    CommonInvalidParam15 = 117,

    // Caller passed invalid value as param 16 (null, invalid json and etc..)
    CommonInvalidParam16 = 118,

    // Caller passed invalid value as param 17 (null, invalid json and etc..)
    CommonInvalidParam17 = 119,

    // Caller passed invalid value as param 18 (null, invalid json and etc..)
    CommonInvalidParam18 = 120,

    // Caller passed invalid value as param 19 (null, invalid json and etc..)
    CommonInvalidParam19 = 121,

    // Caller passed invalid value as param 20 (null, invalid json and etc..)
    CommonInvalidParam20 = 122,

    // Caller passed invalid value as param 21 (null, invalid json and etc..)
    CommonInvalidParam21 = 123,

    // Caller passed invalid value as param 22 (null, invalid json and etc..)
    CommonInvalidParam22 = 124,

    // Caller passed invalid value as param 23 (null, invalid json and etc..)
    CommonInvalidParam23 = 125,

    // Caller passed invalid value as param 24 (null, invalid json and etc..)
    CommonInvalidParam24 = 126,

    // Caller passed invalid value as param 25 (null, invalid json and etc..)
    CommonInvalidParam25 = 127,

    // Caller passed invalid value as param 26 (null, invalid json and etc..)
    CommonInvalidParam26 = 128,

    // Caller passed invalid value as param 27 (null, invalid json and etc..)
    CommonInvalidParam27 = 129,

    // Wallet errors
    // Caller passed invalid wallet handle
    WalletInvalidHandle = 200,

    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 201,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError = 202,

    // Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    WalletNotFoundError = 204,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError = 205,

    // Trying to open wallet that was opened already
    WalletAlreadyOpenedError = 206,

    // Attempt to open encrypted wallet with invalid credentials
    WalletAccessFailed = 207,

    // Input provided to wallet operations is considered not valid
    WalletInputError = 208,

    // Decoding of wallet data during input/output failed
    WalletDecodingError = 209,

    // Storage error occurred during wallet operation
    WalletStorageError = 210,

    // Error during encryption-related operations
    WalletEncryptionError = 211,

    // Requested wallet item not found
    WalletItemNotFound = 212,

    // Returned if wallet's add_record operation is used with record name that already exists
    WalletItemAlreadyExists = 213,

    // Returned if provided wallet query is invalid
    WalletQueryError = 214,

    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300,

    // Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle = 301,

    // Pool ledger terminated
    PoolLedgerTerminated = 302,

    // No concensus during ledger operation
    LedgerNoConsensusError = 303,

    // Attempt to parse invalid transaction response
    LedgerInvalidTransaction = 304,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError = 305,

    // Attempt to create pool ledger config with name used for another existing pool
    PoolLedgerConfigAlreadyExistsError = 306,

    // Timeout for action
    PoolLedgerTimeout = 307,

    // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
    // Call pool.indy_set_protocol_version to set correct Protocol version.
    PoolIncompatibleProtocolVersion = 308,

    // Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFullError = 400,

    AnoncredsInvalidUserRevocIndex = 401,

    // Attempt to generate master secret with duplicated name
    AnoncredsMasterSecretDuplicateNameError = 404,

    AnoncredsProofRejected = 405,

    AnoncredsCredentialRevoked = 406,

    // Attempt to create credential definition with duplicated did schema pair
    AnoncredsCredDefAlreadyExistsError = 407,

    // Signus errors
    // Unknown format of DID entity keys
    UnknownCryptoTypeError = 500,

    // Attempt to create duplicate did
    DidAlreadyExistsError = 600,

    // Unknown payment method was given
    UnknownPaymentMethod = 700,

    //No method were scraped from inputs/outputs or more than one were scraped
    IncompatiblePaymentError = 701,

    // Insufficient funds on inputs
    PaymentInsufficientFundsError = 702,
    
    // No such source on a ledger
    PaymentSourceDoesNotExistError = 703,

    // Extra funds on inputs
    PaymentExtraFundsError = 705,
}

impl ErrorCode {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn description(&self) -> &'static str {
        match self {
            CommonInvalidParam1 => "Caller passed invalid value as param 1",
            CommonInvalidParam2 => "Caller passed invalid value as param 2",
            CommonInvalidParam3 => "Caller passed invalid value as param 3",
            CommonInvalidParam4 => "Caller passed invalid value as param 4",
            CommonInvalidParam5 => "Caller passed invalid value as param 5",
            CommonInvalidParam6 => "Caller passed invalid value as param 6",
            CommonInvalidParam7 => "Caller passed invalid value as param 7",
            CommonInvalidParam8 => "Caller passed invalid value as param 8",
            CommonInvalidParam9 => "Caller passed invalid value as param 9",
            CommonInvalidParam10 => "Caller passed invalid value as param 10",
            CommonInvalidParam11 => "Caller passed invalid value as param 11",
            CommonInvalidParam12 => "Caller passed invalid value as param 12",
            CommonInvalidParam13 => "Caller passed invalid value as param 13",
            CommonInvalidParam14 => "Caller passed invalid value as param 14",
            CommonInvalidParam15 => "Caller passed invalid value as param 15",
            CommonInvalidParam16 => "Caller passed invalid value as param 16",
            CommonInvalidParam17 => "Caller passed invalid value as param 17",
            CommonInvalidParam18 => "Caller passed invalid value as param 18",
            CommonInvalidParam19 => "Caller passed invalid value as param 19",
            CommonInvalidParam20 => "Caller passed invalid value as param 20",
            CommonInvalidParam21 => "Caller passed invalid value as param 21",
            CommonInvalidParam22 => "Caller passed invalid value as param 22",
            CommonInvalidParam23 => "Caller passed invalid value as param 23",
            CommonInvalidParam24 => "Caller passed invalid value as param 24",
            CommonInvalidParam25 => "Caller passed invalid value as param 25",
            CommonInvalidParam26 => "Caller passed invalid value as param 26",
            CommonInvalidParam27 => "Caller passed invalid value as param 27",
            CommonInvalidState => "Invalid library state was detected in runtime. It signals library bug",
            CommonInvalidStructure => "Object (json, config, key, credential and etc...) passed by library caller has invalid structure",
            CommonIOError => "IO Error",
            WalletInvalidHandle => "Caller passed invalid wallet handle",
            WalletUnknownTypeError => "Caller passed invalid wallet handle",
            WalletTypeAlreadyRegisteredError => "Attempt to register already existing wallet type",
            WalletAlreadyExistsError => "Attempt to create wallet with name used for another exists wallet",
            WalletNotFoundError => "Requested entity id isn't present in wallet",
            WalletIncompatiblePoolError => "Trying to use wallet with pool that has different name",
            WalletAccessFailed => "Trying to open wallet encrypted wallet with invalid credentials",
            WalletAlreadyOpenedError => "Trying to open wallet that was opened already",
            WalletInputError => "Input provided to wallet operations is considered not valid",
            WalletDecodingError => "Decoding of wallet data during input/output failed",
            WalletStorageError => "Storage error occurred during wallet operation",
            WalletEncryptionError => "Error during encryption-related operations",
            WalletItemNotFound => "Requested wallet item not found",
            WalletItemAlreadyExists => "Returned if wallet's add_record operation is used with record name that already exists",
            WalletQueryError => "Returned if provided wallet query is invalid",
            PoolLedgerNotCreatedError => "Trying to open pool ledger that wasn't created before",
            PoolLedgerInvalidPoolHandle => "Caller passed invalid pool ledger handle",
            PoolLedgerTerminated => "Pool ledger terminated",
            LedgerNoConsensusError => "No concensus during ledger operation",
            LedgerInvalidTransaction => "Attempt to send unknown or incomplete transaction message",
            LedgerSecurityError => "Attempt to send transaction without the necessary privileges",
            PoolLedgerConfigAlreadyExistsError => "Attempt to create pool ledger config with name used for another existing pool",
            PoolLedgerTimeout => "Timeout for action",
            PoolIncompatibleProtocolVersion => "Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version. Set the correct Protocol version first.",
            AnoncredsRevocationRegistryFullError => "Revocation registry is full and creation of new registry is necessary",
            AnoncredsInvalidUserRevocIndex => "Invalid user revocation index",
            AnoncredsMasterSecretDuplicateNameError => "Attempt to generate master secret with duplicated name",
            AnoncredsProofRejected => "Proof rejected",
            AnoncredsCredentialRevoked => "Credential revoked",
            AnoncredsCredDefAlreadyExistsError => "Credential definition already exists",
            UnknownCryptoTypeError => "Unknown format of DID entity keys",
            DidAlreadyExistsError => "Did already exists",
            UnknownPaymentMethod => "Unknown payment method was given",
            IncompatiblePaymentError => "Multiple different payment methods were specified",
            PaymentInsufficientFundsError => "Payment cannot be processed because there was insufficient funds",
            PaymentSourceDoesNotExistError => "No such source on a ledger.",
            PaymentExtraFundsError => "Payment cannot be processed because there were more funds than required",
        }
    }

    pub fn is_ok(&self) -> bool {
        *self == ErrorCode::Success
    }

    pub fn is_err(&self) -> bool {
        *self != ErrorCode::Success
    }

    pub fn try_err(&self) -> Result<(), ErrorCode> {
        if self.is_err() {
            return Err(*self)
        }
        Ok(())
    }
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

impl From<mpsc::RecvTimeoutError> for ErrorCode {
    fn from(err: mpsc::RecvTimeoutError) -> Self {
        match err {
            mpsc::RecvTimeoutError::Timeout => {
                warn!("Timed out waiting for libindy to call back");
                ErrorCode::CommonIOError
            },
            mpsc::RecvTimeoutError::Disconnected => {
                warn!("Channel to libindy was disconnected unexpectedly");
                ErrorCode::CommonIOError
            }
        }
    }
}

impl From<mpsc::RecvError> for ErrorCode {
    fn from(e: mpsc::RecvError) -> Self {
        warn!("Channel returned an error - {:?}", e);
        ErrorCode::CommonIOError
    }
}
