extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::wallet::WalletCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

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
#[allow(unused_variables)] /* FIXME */
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
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(pool_name, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(xtype, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(config, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(credentials, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Create(
            pool_name,
            name,
            xtype,
            config,
            credentials,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Opens the wallet with specific name.
///
/// Wallet with corresponded name must be previously created with sovrin_create_wallet method.
/// It is impossible to open wallet with the same name more than once.
///
/// #Params
/// name: Name of the wallet.
/// runtime_config (optional): Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
/// {
///     "freshnessTime": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
///     ... List of additional supported keys are defined by wallet type.
/// }
/// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
///                    if NULL, then default credentials will be used.
///
/// #Returns
/// Handle to opened wallet to use in methods that require wallet access.
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn sovrin_open_wallet(command_handle: i32,
                                 name: *const c_char,
                                 runtime_config: *const c_char,
                                 credentials: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>) -> ErrorCode {
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_c_str!(runtime_config, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(credentials, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Open(
            name,
            runtime_config,
            credentials,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                cb(command_handle, err, handle)
            })
        )));

    result_to_err_code!(result)
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
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Close(
            handle,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Deletes created wallet.
///
/// #Params
/// name: Name of the wallet to delete.
/// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
///                    if NULL, then default credentials will be used.
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
                                   credentials: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_c_str!(credentials, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Delete(
            name,
            credentials,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Sets a seq_no (the corresponding Ledger transaction unique sequence number) for the a value
/// in a secure wallet identified by the given string.
/// The string identifying the value in the wallet is returned when the value is stored in the wallet.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// wallet_key: unique string identifying the value in the wallet.
/// seq_no: transaction sequence number.
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
                                                 seq_no: i32,
                                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(wallet_key, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::SetSeqNoForValue(
            wallet_handle,
            wallet_key,
            seq_no,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}