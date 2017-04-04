extern crate libc;

pub mod anoncreds;
pub mod ledger;
pub mod wallet;

use self::libc::{c_char};

#[repr(i32)]
pub enum ErrorCode {
    Success = 0,

    // Common errors
    // Called passed invalid session handle
    CommonInvalidSession = 100,

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState,

    // Wallet errors
    // Unknown type of wallet was passed on open_session
    WalletUnknownType = 200,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegistered,

    // Requested entity id isn't present in wallet
    WalletNotFound,

    // Wallet files referenced in open_session have invalid data format
    WalletInvalidDataFormat,

    // IO error during access wallet backend
    WalletIOError,

    // Ledger errors
    // Pool ledger files referenced in open_session have invalid data format
    LedgerPoolInvalidDataFormat = 300,

    // IO error during access pool ledger files
    LedgerPoolIOError,

    // No concensus during ledger operation
    LedgerNoConsensus,

    // Attempt to send unknown or incomplete transaction message
    LedgerInvalidTransaction,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError,

    // IO error during sending of ledger transactions or catchup process
    LedgerIOError,

    // Crypto errors
    // Invalid structure of any crypto promitives (keys, signatures, seeds and etc...)
    CryptoInvalidStructure = 400,

    // Unknown crypto type was requested for signing/verifiyng or encoding/decoding
    CryptoUnknownType,

    // Revocation registry is full and creation of new registry is necessary
    CryptoRevocationRegistryFull
}

/// Creates a new session. A session is associated with a wallet and a pool ledger.
/// The call is synchronous.
///
/// Note that there can be only one session for each wallet if wallet implementation doesn't provide
/// concurrent access.
///
/// #Params
/// wallet_config: Wallet configuration json. Example:
/// {
///     "type": string, (optional; if not provided then the default wallet type will be used.
///             Custom wallet type can be created with register_wallet_type call)
///     "freshnessTime": int, (optional; if not provided then 60*24 value will be used.
///             Amount of minutes to consider wallet value as fresh.
///     "config": json, (optional; if not provided default configuration will be used. Configuration
///             is specific for concrete wallet type)
/// }
/// ledger_config: (optional) Ledger configuration json. Example:
/// {
///     "genesis_txn": string, (optional; a path to genesis transaction file. If NULL, then a default one will be used.)
///     // TODO: Provide description of additional params like timeouts
/// }
///
/// #Returns
/// session handle
///
/// #Errors
/// CommonInvalidParam1
/// CommonInvalidParam2
/// WalletUnknownType
/// WalletInvalidDataFormat
/// WalletIOError
/// LedgerPoolInvalidDataFormat
/// LedgerPoolIOError
/// LedgerIOError
///
#[no_mangle]
pub extern fn open_session(wallet_config: *const c_char, ledger_config: *const c_char) -> ErrorCode {
    unimplemented!();
}

/// Closes a session and frees allocated resources. The call is synchronous.
///
/// #Params
/// session_handle: session handler (created by open_session).
///
/// #Returns
/// error code
///
/// #Errors
#[no_mangle]
pub extern fn close_session(session_handle: i32) -> ErrorCode {
    unimplemented!();
}

/// Registers custom wallet implementation.
/// It allows library user to provide custom wallet implementation that can be referenced in
/// open_session call.
///
/// #Params
/// type_name: Wallet type name.
/// init: init operation handler
/// set: set operation handler
/// get: get operation handler
/// free: free operation handler
///
/// #Returns
/// error code
///
/// #Errors
/// CommonInvalidParam1
/// CommonInvalidParam2
/// CommonInvalidParam3
/// CommonInvalidParam4
/// CommonInvalidParam5
/// WalletTypeAlreadyRegistered
#[no_mangle]
pub extern fn register_wallet_type(type_name: *const c_char,
                                   init: extern fn(config: *const c_char, wallet_handle: *const *mut i32) -> ErrorCode,
                                   set: extern fn(wallet_handle: i32, key: *const c_char, sub_key: *const c_char, value: *const c_char) -> ErrorCode,
                                   get: extern fn(wallet_handle: i32, key: *const c_char, sub_key: *const c_char, value_ptr: *const *mut c_char) -> ErrorCode,
                                   free: extern fn(wallet_handle: i32, str: *const c_char) -> ErrorCode) -> ErrorCode {
    unimplemented!();
}

/// Refreshes a local copy of a pool ledger associated with the given genesis transactions file.
/// If there is not copy for this genesis transaction file, then a new one will be created.
///
/// #Params
/// ledger_config: (optional) Ledger configuration json. Example:
/// {
///     "genesis_txn": string, (optional; a path to genesis transaction file. If NULL, then a default one will be used.)
///     // TODO: Provide description of additional params like timeouts
/// }
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn refresh_pool_ledger(ledger_config: *const c_char,
                                  cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
}
