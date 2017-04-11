extern crate libc;

use api::ErrorCode;

use self::libc::{c_char, c_uchar};

/// Registers custom wallet implementation.
///
/// It allows library user to provide custom wallet implementation.
///
/// #Params
/// xtype: Wallet type name.
/// create: create operation handler
/// create: open operation handler
/// set: set operation handler
/// get: get operation handler
/// close: close operation handler
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
pub extern fn sovrin_register_wallet_type(xtype: *const c_char,
                                          create: extern fn(name: *const c_char,
                                                            config: *const c_char,
                                                            credentials: *const c_char) -> ErrorCode,
                                          open: extern fn(name: *const c_char,
                                                          config: *const c_char,
                                                          credentials: *const c_char,
                                                          handle: *const *mut i32) -> ErrorCode,
                                          set: extern fn(handle: i32,
                                                         key: *const c_char, sub_key: *const c_char,
                                                         value: *const c_char) -> ErrorCode,
                                          get: extern fn(handle: i32,
                                                         key: *const c_char, sub_key: *const c_char,
                                                         value_ptr: *const *mut c_char,
                                                         value_life_time: *const *mut i32) -> ErrorCode,
                                          close: extern fn(handle: i32) -> ErrorCode,
                                          delete: extern fn(name: *const c_char) -> ErrorCode,
                                          free: extern fn(wallet_handle: i32, str: *const c_char) -> ErrorCode) -> ErrorCode {
    unimplemented!();
}

/// Creates a new secure wallet with the given unique name.
///
/// #Params
/// pool_name: Name of the pool that corresponds to this wallet.
/// name: Name of the wallet.
/// xtype(optional): Type of the wallet. Defaults to 'default'.
///                  Custom types can be registered with sovrin_register_wallet_type call.
/// config(optional): Wallet configuration json. List of supported keys are defined by wallet type.
///                    if NULL, then default config will be used.
/// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
///                    if NULL, then default config will be used.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_create_wallet(command_handle: i32,
                                   pool_name: *const c_char,
                                   name: *const c_char,
                                   xtype: *const c_char,
                                   config: *const c_char,
                                   credentials: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Opens the wallet with specific name.
///
/// Wallet with corresponded name must be previously created with sovrin_create_pool method.
/// It is impossible to open wallet with the same name more than once.
///
/// #Params
/// pool_handle: pool handle returned by sovrin_open_pool
/// name: Name of the wallet.
/// config (optional): Runtime wallet configuration json. if NULL, then default config will be used. Example:
/// {
///     "freshnessTime": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
///     ... List of additional supported keys are defined by wallet type.
/// }
/// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
///                    if NULL, then default config will be used.
///
/// #Returns
/// Handle to opened wallet to use in methods that require wallet access.
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_open_wallet(command_handle: i32,
                                 pool_handle: i32,
                                 name: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char,
                                 cb: extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)) -> ErrorCode {
    unimplemented!();
}


/// Closes opened wallet and frees allocated resources.
///
/// #Params
/// handle: wallet handle returned by sovrin_open_wallet.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_close_wallet(command_handle: i32,
                                  handle: i32,
                                  cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Deletes created wallet.
///
/// #Params
/// name: Name of the wallet to delete.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_delete_wallet(command_handle: i32,
                                   name: *const c_char,
                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Sets a seq_no (the corresponding Ledger transaction unique sequence number) for the a value
/// in a secure wallet identified by the given number.
/// The number identifying the value in the wallet is returned when the value is stored in the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// wallet_key: unique number identifying the value in the wallet
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_wallet_set_seq_no_for_value(command_handle: i32,
                                                 wallet_handle: i32,
                                                 wallet_key: *const c_char,
                                                 cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}