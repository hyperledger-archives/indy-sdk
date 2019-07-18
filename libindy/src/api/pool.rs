
use api::{ErrorCode, CommandHandle, PoolHandle};
use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use domain::pool::{PoolConfig, PoolOpenConfig};
use errors::prelude::*;
use utils::ctypes;

use serde_json;
use libc::c_char;

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
pub extern fn indy_create_pool_ledger_config(command_handle: CommandHandle,
                                             config_name: *const c_char,
                                             config: *const c_char,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                                  err: ErrorCode)>) -> ErrorCode {
    trace!("indy_create_pool_ledger_config: >>> config_name: {:?}, config: {:?}", config_name, config);

    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_json!(config, ErrorCode::CommonInvalidParam3, PoolConfig);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_create_pool_ledger_config: entities >>> config_name: {:?}, config: {:?}", config_name, config);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Create(
            config_name,
            config,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_create_pool_ledger_config:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_create_pool_ledger_config: <<< res: {:?}", res);

    res
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
///     "timeout": int (optional), timeout for network request (in sec).
///     "extended_timeout": int (optional), extended timeout for network request (in sec).
///     "preordered_nodes": array<string> -  (optional), names of nodes which will have a priority during request sending:
///         ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
///         Note: Not specified nodes will be placed in a random way.
/// }
///
/// #Returns
/// Handle to opened pool to use in methods that require pool connection.
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn indy_open_pool_ledger(command_handle: CommandHandle,
                                    config_name: *const c_char,
                                    config: *const c_char,
                                    cb: Option<extern fn(command_handle_: CommandHandle,
                                                         err: ErrorCode,
                                                         pool_handle: PoolHandle)>) -> ErrorCode {
    trace!("indy_open_pool_ledger: >>> config_name: {:?}, config: {:?}", config_name, config);

    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_json!(config, ErrorCode::CommonInvalidParam3, PoolOpenConfig);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_open_pool_ledger: entities >>> config_name: {:?}, config: {:?}", config_name, config);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Open(
            config_name,
            config,
            Box::new(move |result| {
                let (err, pool_handle) = prepare_result_1!(result, 0);
                trace!("indy_open_pool_ledger: pool_handle: {:?}", pool_handle);
                cb(command_handle, err, pool_handle)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_open_pool_ledger: <<< res: {:?}", res);

    res
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
pub extern fn indy_refresh_pool_ledger(command_handle: CommandHandle,
                                       handle: PoolHandle,
                                       cb: Option<extern fn(command_handle_: CommandHandle,
                                                            err: ErrorCode)>) -> ErrorCode {
    trace!("indy_refresh_pool_ledger: >>> handle: {:?}", handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_refresh_pool_ledger: entities >>> handle: {:?}", handle);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Refresh(
            handle,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_refresh_pool_ledger:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_refresh_pool_ledger: <<< res: {:?}", res);

    res
}

/// Lists names of created pool ledgers
///
/// #Params
///
/// #Returns
/// Error code
///
/// #Errors
#[no_mangle]
pub extern fn indy_list_pools(command_handle: CommandHandle,
                              cb: Option<extern fn(command_handle_: CommandHandle,
                                                   err: ErrorCode,
                                                   pools: *const c_char)>) -> ErrorCode {
    trace!("indy_list_pools: >>>");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    trace!("indy_list_pools: entities >>>");

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::List(
            Box::new(move |result| {
                let (err, pools) = prepare_result_1!(result, String::new());
                trace!("indy_list_pools: pools: {:?}", pools);
                let pools = ctypes::string_to_cstring(pools);
                cb(command_handle, err, pools.as_ptr())
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_list_pools: <<< res: {:?}", res);

    res
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
pub extern fn indy_close_pool_ledger(command_handle: CommandHandle,
                                     handle: PoolHandle,
                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                          err: ErrorCode)>) -> ErrorCode {
    trace!("indy_close_pool_ledger: >>> handle: {:?}", handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_close_pool_ledger: entities >>> handle: {:?}", handle);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Close(
            handle,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_close_pool_ledger:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_close_pool_ledger: <<< res: {:?}", res);

    res
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
pub extern fn indy_delete_pool_ledger_config(command_handle: CommandHandle,
                                             config_name: *const c_char,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                                  err: ErrorCode)>) -> ErrorCode {
    trace!("indy_delete_pool_ledger_config: >>> config_name: {:?}", config_name);

    check_useful_c_str!(config_name, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_delete_pool_ledger_config: entities >>> config_name: {:?}", config_name);

    let result = CommandExecutor::instance()
        .send(Command::Pool(PoolCommand::Delete(
            config_name,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_delete_pool_ledger_config:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_delete_pool_ledger_config: <<< res: {:?}", res);

    res
}

/// Set PROTOCOL_VERSION to specific version.
///
/// There is a global property PROTOCOL_VERSION that used in every request to the pool and
/// specified version of Indy Node which Libindy works.
///
/// By default PROTOCOL_VERSION=1.
///
/// #Params
/// protocol_version: Protocol version will be used:
///     1 - for Indy Node 1.3
///     2 - for Indy Node 1.4 and greater
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_set_protocol_version(command_handle: CommandHandle,
                                        protocol_version: usize,
                                        cb: Option<extern fn(command_handle_: CommandHandle,
                                                             err: ErrorCode)>) -> ErrorCode {
    trace!("indy_set_protocol_version: >>> protocol_version: {:?}", protocol_version);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_set_protocol_version: entities >>> protocol_version: {:?}", protocol_version);

    let result = CommandExecutor::instance()
        .send(Command::Pool(
            PoolCommand::SetProtocolVersion(
            protocol_version,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_set_protocol_version:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_set_protocol_version: <<< res: {:?}", res);

    res
}
