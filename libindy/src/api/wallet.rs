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
/// command_handle: Command handle to map callback to caller context.
/// xtype: Wallet type name.
/// create: WalletType create operation handler
/// open: WalletType open operation handler
/// set: Wallet set operation handler
/// get: Wallet get operation handler
/// get_not_expired: Wallet get_not_expired operation handler
/// list: Wallet list operation handler(must to return data in the following format: {"values":[{"key":"", "value":""}, {"key":"", "value":""}]}
/// close: Wallet close operation handler
/// delete: WalletType delete operation handler
/// free: Handler that allows to de-allocate strings allocated in caller code
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_register_wallet_type(command_handle: i32,
                                        xtype: *const c_char,
                                        create: Option<extern fn(name: *const c_char,
                                                                 config: *const c_char,
                                                                 credentials: *const c_char) -> ErrorCode>,
                                        open: Option<extern fn(name: *const c_char,
                                                               config: *const c_char,
                                                               runtime_config: *const c_char,
                                                               credentials: *const c_char,
                                                               handle: *mut i32) -> ErrorCode>,
                                        set: Option<extern fn(handle: i32,
                                                              key: *const c_char,
                                                              value: *const c_char) -> ErrorCode>,
                                        get: Option<extern fn(handle: i32,
                                                              key: *const c_char,
                                                              value_ptr: *mut *const c_char) -> ErrorCode>,
                                        get_not_expired: Option<extern fn(handle: i32,
                                                                          key: *const c_char,
                                                                          value_ptr: *mut *const c_char) -> ErrorCode>,
                                        list: Option<extern fn(handle: i32,
                                                               key_prefix: *const c_char,
                                                               values_json_ptr: *mut *const c_char) -> ErrorCode>,
                                        close: Option<extern fn(handle: i32) -> ErrorCode>,
                                        delete: Option<extern fn(name: *const c_char,
                                                                 config: *const c_char,
                                                                 credentials: *const c_char) -> ErrorCode>,
                                        free: Option<extern fn(wallet_handle: i32,
                                                               value: *const c_char) -> ErrorCode>,
                                        cb: Option<extern fn(xcommand_handle: i32,
                                                             err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(xtype, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(create, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(open, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(set, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(get, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(get_not_expired, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(list, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(close, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(delete, ErrorCode::CommonInvalidParam10);
    check_useful_c_callback!(free, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam12);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::RegisterWalletType(
            xtype,
            create,
            open,
            set,
            get,
            get_not_expired,
            list,
            close,
            delete,
            free,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Creates a new secure wallet with the given unique name.
///
/// #Params
/// pool_name: Name of the pool that corresponds to this wallet.
/// name: Name of the wallet.
/// xtype(optional): Type of the wallet. Defaults to 'default'.
///                  Custom types can be registered with indy_register_wallet_type call.
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
pub extern fn indy_create_wallet(command_handle: i32,
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
/// Wallet with corresponded name must be previously created with indy_create_wallet method.
/// It is impossible to open wallet with the same name more than once.
///
/// #Params
/// name: Name of the wallet.
/// runtime_config (optional): Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
/// {
///     "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
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
pub extern fn indy_open_wallet(command_handle: i32,
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

/// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
#[no_mangle]
pub extern fn indy_list_wallets(command_handle: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     wallets: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::ListWallets(
            Box::new(move |result| {
                let (err, wallets) = result_to_err_code_1!(result, String::new());
                let wallets = CStringUtils::string_to_cstring(wallets);
                cb(command_handle, err, wallets.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Closes opened wallet and frees allocated resources.
///
/// #Params
/// handle: wallet handle returned by indy_open_wallet.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_close_wallet(command_handle: i32,
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
pub extern fn indy_delete_wallet(command_handle: i32,
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