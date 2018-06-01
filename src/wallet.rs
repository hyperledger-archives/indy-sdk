use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;
use std::time::Duration;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use ffi::wallet;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseI32CB};

pub struct Wallet {}

impl Wallet {
    /// Registers custom wallet implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `xtype` - Wallet type name.
    /// * `create` - WalletType create operation handler
    /// * `open` - WalletType open operation handler
    /// * `set` - Wallet set operation handler
    /// * `get` - Wallet get operation handler
    /// * `get_not_expired` - Wallet get_not_expired operation handler
    /// * `list` - Wallet list operation handler(must to return data in the following format: {"values":[{"key":"", "value":""}, {"key":"", "value":""}]}
    /// * `close` - Wallet close operation handler
    /// * `delete` - WalletType delete operation handler
    /// * `free` - Handler that allows to de-allocate strings allocated in caller code
    pub fn register(xtype: &str,
                    create: Option<wallet::CreateWalletCB>,
                    open: Option<wallet::OpenWalletCB>,
                    set: Option<wallet::SetWalletCB>,
                    get: Option<wallet::GetWalletCB>,
                    get_not_expired: Option<wallet::GetNotExpiredWalletCB>,
                    list: Option<wallet::ListWalletCB>,
                    close: Option<wallet::CloseWalletCB>,
                    delete: Option<wallet::DeleteWalletCB>,
                    free: Option<wallet::FreeWalletCB>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register(command_handle, xtype, create, open, set, get, get_not_expired, list, close, delete, free, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Registers custom wallet implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `xtype` - Wallet type name.
    /// * `create` - WalletType create operation handler
    /// * `open` - WalletType open operation handler
    /// * `set` - Wallet set operation handler
    /// * `get` - Wallet get operation handler
    /// * `get_not_expired` - Wallet get_not_expired operation handler
    /// * `list` - Wallet list operation handler(must to return data in the following format: {"values":[{"key":"", "value":""}, {"key":"", "value":""}]}
    /// * `close` - Wallet close operation handler
    /// * `delete` - WalletType delete operation handler
    /// * `free` - Handler that allows to de-allocate strings allocated in caller code
    /// * `timeout` - the maximum time this function waits for a response
    pub fn register_timeout(xtype: &str,
                            create: Option<wallet::CreateWalletCB>,
                            open: Option<wallet::OpenWalletCB>,
                            set: Option<wallet::SetWalletCB>,
                            get: Option<wallet::GetWalletCB>,
                            get_not_expired: Option<wallet::GetNotExpiredWalletCB>,
                            list: Option<wallet::ListWalletCB>,
                            close: Option<wallet::CloseWalletCB>,
                            delete: Option<wallet::DeleteWalletCB>,
                            free: Option<wallet::FreeWalletCB>,
                            timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register(command_handle, xtype, create, open, set, get, get_not_expired, list, close, delete, free, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Registers custom wallet implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `xtype` - Wallet type name.
    /// * `create` - WalletType create operation handler
    /// * `open` - WalletType open operation handler
    /// * `set` - Wallet set operation handler
    /// * `get` - Wallet get operation handler
    /// * `get_not_expired` - Wallet get_not_expired operation handler
    /// * `list` - Wallet list operation handler(must to return data in the following format: {"values":[{"key":"", "value":""}, {"key":"", "value":""}]}
    /// * `close` - Wallet close operation handler
    /// * `delete` - WalletType delete operation handler
    /// * `free` - Handler that allows to de-allocate strings allocated in caller code
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn register_async<F: 'static>(xtype: &str,
                                      create: Option<wallet::CreateWalletCB>,
                                      open: Option<wallet::OpenWalletCB>,
                                      set: Option<wallet::SetWalletCB>,
                                      get: Option<wallet::GetWalletCB>,
                                      get_not_expired: Option<wallet::GetNotExpiredWalletCB>,
                                      list: Option<wallet::ListWalletCB>,
                                      close: Option<wallet::CloseWalletCB>,
                                      delete: Option<wallet::DeleteWalletCB>,
                                      free: Option<wallet::FreeWalletCB>,
                                      closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_register(command_handle, xtype, create, open, set, get, get_not_expired, list, close, delete, free, cb)
    }

    fn _register(command_handle: IndyHandle,
                 xtype: &str,
                 create: Option<wallet::CreateWalletCB>,
                 open: Option<wallet::OpenWalletCB>,
                 set: Option<wallet::SetWalletCB>,
                 get: Option<wallet::GetWalletCB>,
                 get_not_expired: Option<wallet::GetNotExpiredWalletCB>,
                 list: Option<wallet::ListWalletCB>,
                 close: Option<wallet::CloseWalletCB>,
                 delete: Option<wallet::DeleteWalletCB>,
                 free: Option<wallet::FreeWalletCB>,
                 cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);

        ErrorCode::from(unsafe {
                wallet::indy_register_wallet_type(command_handle,
                                              xtype.as_ptr(),
                                                  create,
                                                  open,
                                                  set,
                                                  get,
                                                  get_not_expired,
                                                  list,
                                                  close,
                                                  delete,
                                                  free,
                                                  cb)
        })
    }
    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `xtype` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    pub fn create(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, pool_name, wallet_name, xtype, config, credentials, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `xtype` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_timeout(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, pool_name, wallet_name, xtype, config, credentials, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `xtype` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_create(command_handle, pool_name, wallet_name, xtype, config, credentials, cb)
    }

    fn _create(command_handle: IndyHandle, pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);
        let wallet_name = c_str!(wallet_name);
        let xtype_str = opt_c_str!(xtype);
        let config_str = opt_c_str!(config);
        let credentials_str = opt_c_str!(credentials);

        ErrorCode::from(unsafe {
            wallet::indy_create_wallet(command_handle,
                                       pool_name.as_ptr(),
                                       wallet_name.as_ptr(),
                                       opt_c_ptr!(xtype, xtype_str),
                                       opt_c_ptr!(config, config_str),
                                       opt_c_ptr!(credentials, credentials_str),
                                       cb)
        })
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet.
    /// * `runtime_config`  (optional)- Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
    /// {
    ///     "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
    ///     ... List of additional supported keys are defined by wallet type.
    /// }
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    ///
    /// # Returns
    /// Handle to opened wallet to use in methods that require wallet access.
    pub fn open(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open(command_handle, wallet_name, config, credentials, cb);

        ResultHandler::one(err, receiver)
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet.
    /// * `runtime_config`  (optional)- Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
    /// {
    ///     "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
    ///     ... List of additional supported keys are defined by wallet type.
    /// }
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Handle to opened wallet to use in methods that require wallet access.
    pub fn open_timeout(wallet_name: &str, config: Option<&str>, credentials: Option<&str>, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open(command_handle, wallet_name, config, credentials, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet.
    /// * `runtime_config`  (optional)- Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
    /// {
    ///     "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
    ///     ... List of additional supported keys are defined by wallet type.
    /// }
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn open_async<F: 'static>(wallet_name: &str, config: Option<&str>, credentials: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Wallet::_open(command_handle, wallet_name, config, credentials, cb)
    }

    fn _open(command_handle: IndyHandle, wallet_name: &str, config: Option<&str>, credentials: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
        let wallet_name = c_str!(wallet_name);
        let config_str = opt_c_str!(config);
        let credentials_str = opt_c_str!(credentials);

        ErrorCode::from(unsafe {
            wallet::indy_open_wallet(command_handle,
                                     wallet_name.as_ptr(),
                                     opt_c_ptr!(config, config_str),
                                     opt_c_ptr!(credentials, credentials_str),
                                     cb)
        })
    }

    /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_list(command_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
    /// * `timeout` - the maximum time this function waits for a response
    pub fn list_timeout(timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_list(command_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_async<F: 'static>(closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Wallet::_list(command_handle, cb)
    }

    fn _list(command_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe { wallet::indy_list_wallets(command_handle, cb) })
    }

    /// Deletes created wallet.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet to delete.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    pub fn delete(wallet_name: &str, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete(command_handle, wallet_name, credentials, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Deletes created wallet.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet to delete.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_timeout(wallet_name: &str, credentials: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete(command_handle, wallet_name, credentials, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Deletes created wallet.
    ///
    /// # Arguments
    /// * `name` - Name of the wallet to delete.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_async<F: 'static>(wallet_name: &str, credentials: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_delete(command_handle, wallet_name, credentials, cb)
    }

    fn _delete(command_handle: IndyHandle, wallet_name: &str, credentials: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let wallet_name = c_str!(wallet_name);
        let credentials_str = opt_c_str!(credentials);

        ErrorCode::from(unsafe { wallet::indy_delete_wallet(command_handle, wallet_name.as_ptr(), opt_c_ptr!(credentials, credentials_str), cb) })
    }

    /// Closes opened wallet and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - wallet handle returned by Wallet::open.
    pub fn close(wallet_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_close(command_handle, wallet_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Closes opened wallet and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - wallet handle returned by Wallet::open.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn close_timeout(wallet_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_close(command_handle, wallet_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Closes opened wallet and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - wallet handle returned by Wallet::open.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn close_async<F: 'static>(wallet_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_close(command_handle, wallet_handle, cb)
    }

    fn _close(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe { wallet::indy_close_wallet(command_handle, wallet_handle, cb) })
    }
}
