use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;
use std::time::Duration;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

use native::pool;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseI32CB};

pub struct Pool {}

impl Pool {
    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// {
    ///     "genesis_txn": string (required), A path to genesis transaction file.
    /// }
    pub fn create_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// {
    ///     "genesis_txn": string (required), A path to genesis transaction file.
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_ledger_config_timeout(pool_name: &str, pool_config: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_ledger_config_async<F: 'static>(pool_name: &str, pool_config: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb)
    }

    fn _create_ledger_config(command_handle: IndyHandle, pool_name: &str, pool_config: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
    pub fn open_ledger(pool_name: &str, config: Option<&str>) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Pool::_open_ledger(command_handle, pool_name, config, cb);

        ResultHandler::one(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Handle to opened pool to use in methods that require pool connection.
    pub fn open_ledger_timeout(pool_name: &str, config: Option<&str>, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Pool::_open_ledger(command_handle, pool_name, config, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn open_ledger_async<F: 'static>(pool_name: &str, config: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Pool::_open_ledger(command_handle, pool_name, config, cb)
    }

    fn _open_ledger(command_handle: IndyHandle, pool_name: &str, config: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);
        let config_str = opt_c_str!(config);

        ErrorCode::from(unsafe { pool::indy_open_pool_ledger(command_handle, pool_name.as_ptr(), opt_c_ptr!(config, config_str), cb) })
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    pub fn refresh(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_refresh(command_handle, pool_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    /// * `timeout` - the maximum time this function waits for a response
    pub fn refresh_timeout(pool_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_refresh(command_handle, pool_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn refresh_async<F: 'static>(pool_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_refresh(command_handle, pool_handle, cb)
    }

    fn _refresh(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_refresh_pool_ledger(command_handle, pool_handle, cb) })
    }

    /// Lists names of created pool ledgers
    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pool::_list(command_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// Lists names of created pool ledgers
    /// * `timeout` - the maximum time this function waits for a response
    pub fn list_timeout(timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pool::_list(command_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Lists names of created pool ledgers
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_async<F: 'static>(closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Pool::_list(command_handle, cb)
    }

    fn _list(command_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_list_pools(command_handle, cb) })
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    pub fn close(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_close(command_handle, pool_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn close_timeout(pool_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_close(command_handle, pool_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn close_async<F: 'static>(pool_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_close(command_handle, pool_handle, cb)
    }

    fn _close(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_close_pool_ledger(command_handle, pool_handle, cb) })
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_delete(command_handle, pool_name, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_timeout(pool_name: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_delete(command_handle, pool_name, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_async<F: 'static>(pool_name: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_delete(command_handle, pool_name, cb)
    }

    fn _delete(command_handle: IndyHandle, pool_name: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
    pub fn set_protocol_version(protocol_version: usize) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_set_protocol_version(command_handle, protocol_version, cb);

        ResultHandler::empty(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_protocol_version_timeout(protocol_version: usize, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_set_protocol_version(command_handle, protocol_version, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn set_protocol_version_async<F: 'static>(protocol_version: usize, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_set_protocol_version(command_handle, protocol_version, cb)
    }

    fn _set_protocol_version(command_handle: IndyHandle, protocol_version: usize, cb: Option<ResponseEmptyCB>) -> ErrorCode {

        ErrorCode::from(unsafe {
          pool::indy_set_protocol_version(command_handle, protocol_version, cb)
        })
    }
}
