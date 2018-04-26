extern crate libc;

use api::ErrorCode;
use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::ToErrorCode;
use utils::cstring::CStringUtils;

use self::libc::c_char;

/// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
///
/// #Params
/// config_name: Name of the pool ledger configuration.
/// config (optional): Pool configuration json. if NULL, then default config will be used. Example:
/// {
///     "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
///                    If file doesn't exists default one will be created.
/// }
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_create_pool_ledger_config(command_handle: i32,
                                             config_name: *const c_char,
                                             config: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_c_str!(config, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Create(
            config_name,
            config,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Opens pool ledger and performs connecting to pool nodes.
///
/// Pool ledger configuration with corresponded name must be previously created
/// with indy_create_pool_ledger_config method.
/// It is impossible to open pool with the same name more than once.
///
/// config_name: Name of the pool ledger configuration.
/// config (optional): Runtime pool configuration json.
///                         if NULL, then default config will be used. Example:
/// {
///     "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
///                      Defaults to true.
///     "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
///                        Use 0 to disable automatic refresh. Defaults to 24*60.
///     "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
///                       Defaults to 20000.
/// }
///
/// #Returns
/// Handle to opened pool to use in methods that require pool connection.
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_open_pool_ledger(command_handle: i32,
                                    config_name: *const c_char,
                                    config: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, pool_handle: i32)>) -> ErrorCode {
    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_c_str!(config, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Open(
            config_name,
            config,
            Box::new(move |result| {
                let (err, pool_handle) = result_to_err_code_1!(result, 0);
                cb(command_handle, err, pool_handle)
            })
        )));

    result_to_err_code!(result)
}

/// Refreshes a local copy of a pool ledger and updates pool nodes connections.
///
/// #Params
/// handle: pool handle returned by indy_open_pool_ledger
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_refresh_pool_ledger(command_handle: i32,
                                       handle: i32,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Refresh(
            handle,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Lists names of created pool ledgers
#[no_mangle]
pub extern fn indy_list_pools(command_handle: i32,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                   pools: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::List(
            Box::new(move |result| {
                let (err, pools) = result_to_err_code_1!(result, String::new());
                let pools = CStringUtils::string_to_cstring(pools);
                cb(command_handle, err, pools.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Closes opened pool ledger, opened nodes connections and frees allocated resources.
///
/// #Params
/// handle: pool handle returned by indy_open_pool_ledger.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_close_pool_ledger(command_handle: i32,
                                     handle: i32,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Close(
            handle,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Deletes created pool ledger configuration.
///
/// #Params
/// config_name: Name of the pool ledger configuration to delete.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_delete_pool_ledger_config(command_handle: i32,
                                             config_name: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Delete(
            config_name,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}