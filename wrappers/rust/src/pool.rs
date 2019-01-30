use {ErrorCode, IndyHandle, IndyError};

use std::ffi::CString;
use std::ptr::null;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::pool;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseI32CB};

use futures::Future;

/// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
///
/// # Arguments
/// * `config_name` - Name of the pool ledger configuration.
/// * `config`  (required)- Pool configuration json. Example:
/// {
///     "genesis_txn": string (required), A path to genesis transaction file.
/// }
pub fn create_pool_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _create_pool_ledger_config(command_handle, pool_name, pool_config, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _create_pool_ledger_config(command_handle: IndyHandle, pool_name: &str, pool_config: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let pool_name = c_str!(pool_name);
    let pool_config_str = opt_c_str!(pool_config);

    ErrorCode::from(unsafe { pool::indy_create_pool_ledger_config(command_handle, pool_name.as_ptr(), opt_c_ptr!(pool_config, pool_config_str), cb) })
}

/// Opens pool ledger and performs connecting to pool nodes.
///
/// Pool ledger configuration with corresponded name must be previously created
/// with indy_create_pool_ledger_config method.
/// It is impossible to open pool with the same name more than once.
///
/// # Arguments
/// * `config_name` - Name of the pool ledger configuration.
/// * `config`  (optional)- Runtime pool configuration json.
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
/// # Returns
/// Handle to opened pool to use in methods that require pool connection.
pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Box<Future<Item=IndyHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_pool_ledger(command_handle, pool_name, config, cb);

    ResultHandler::handle(command_handle, err, receiver)
}

fn _open_pool_ledger(command_handle: IndyHandle, pool_name: &str, config: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
    let pool_name = c_str!(pool_name);
    let config_str = opt_c_str!(config);

    ErrorCode::from(unsafe { pool::indy_open_pool_ledger(command_handle, pool_name.as_ptr(), opt_c_ptr!(config, config_str), cb) })
}

/// Refreshes a local copy of a pool ledger and updates pool nodes connections.
///
/// # Arguments
/// * `handle` - pool handle returned by open_ledger
pub fn refresh_pool_ledger(pool_handle: IndyHandle) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _refresh_pool_ledger(command_handle, pool_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _refresh_pool_ledger(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    ErrorCode::from(unsafe { pool::indy_refresh_pool_ledger(command_handle, pool_handle, cb) })
}

/// Lists names of created pool ledgers
pub fn list_pools() -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _list_pools(command_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _list_pools(command_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe { pool::indy_list_pools(command_handle, cb) })
}

/// Closes opened pool ledger, opened nodes connections and frees allocated resources.
///
/// # Arguments
/// * `handle` - pool handle returned by open_ledger.
pub fn close_pool_ledger(pool_handle: IndyHandle) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _close_pool_ledger(command_handle, pool_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _close_pool_ledger(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    ErrorCode::from(unsafe { pool::indy_close_pool_ledger(command_handle, pool_handle, cb) })
}

/// Deletes created pool ledger configuration.
///
/// # Arguments
/// * `config_name` - Name of the pool ledger configuration to delete.
pub fn delete_pool_ledger(pool_name: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _delete_pool_ledger(command_handle, pool_name, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _delete_pool_ledger(command_handle: IndyHandle, pool_name: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let pool_name = c_str!(pool_name);

    ErrorCode::from(unsafe { pool::indy_delete_pool_ledger_config(command_handle, pool_name.as_ptr(), cb) })
}

/// Set PROTOCOL_VERSION to specific version.
///
/// There is a global property PROTOCOL_VERSION that used in every request to the pool and
/// specified version of Indy Node which Libindy works.
///
/// By default PROTOCOL_VERSION=1.
///
/// # Arguments
/// * `protocol_version` - Protocol version will be used:
///     1 - for Indy Node 1.3
///     2 - for Indy Node 1.4
pub fn set_protocol_version(protocol_version: usize) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _set_protocol_version(command_handle, protocol_version, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _set_protocol_version(command_handle: IndyHandle, protocol_version: usize, cb: Option<ResponseEmptyCB>) -> ErrorCode {

    ErrorCode::from(unsafe {
      pool::indy_set_protocol_version(command_handle, protocol_version, cb)
    })
}
