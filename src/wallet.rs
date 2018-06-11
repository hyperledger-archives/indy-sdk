use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;
use std::time::Duration;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use ffi::{wallet, non_secrets};
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
                    create: Option<wallet::WalletCreateCB>,
                    open: Option<wallet::WalletOpenCB>,
                    close: Option<wallet::WalletCloseCB>,
                    delete: Option<wallet::WalletDeleteCB>,
                    add_record: Option<wallet::WalletAddRecordCB>,
                    update_record_value: Option<wallet::WalletUpdateRecordValueCB>,
                    update_record_tags: Option<wallet::WalletUpdateRecordTagsCB>,
                    add_record_tags: Option<wallet::WalletAddRecordTagsCB>,
                    delete_record_tags: Option<wallet::WalletDeleteRecordTagsCB>,
                    delete_record: Option<wallet::WalletDeleteRecordCB>,
                    get_record: Option<wallet::WalletGetRecordCB>,
                    get_record_id: Option<wallet::WalletGetRecordIdCB>,
                    get_record_type: Option<wallet::WalletGetRecordTypeCB>,
                    get_record_value: Option<wallet::WalletGetRecordValueCB>,
                    get_record_tags: Option<wallet::WalletGetRecordTagsCB>,
                    free_record: Option<wallet::WalletFreeRecordCB>,
                    get_storage_metadata: Option<wallet::WalletGetStorageMetadataCB>,
                    set_storage_metadata: Option<wallet::WalletSetStorageMetadataCB>,
                    free_storage_metadata: Option<wallet::WalletFreeStorageMetadataCB>,
                    search_records: Option<wallet::WalletSearchRecordsCB>,
                    search_all_records: Option<wallet::WalletSearchAllRecordsCB>,
                    get_search_total_count: Option<wallet::WalletGetSearchTotalCountCB>,
                    fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecordCB>,
                    free_search: Option<wallet::WalletFreeSearchCB>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register(command_handle,
                                              xtype,
                                                  create,
                                                  open,
                                                  close,
                                                  delete,
                                                  add_record,
                                                  update_record_value,
                                                  update_record_tags,
                                                  add_record_tags,
                                                  delete_record_tags,
                                                  delete_record,
                                                  get_record,
                                                  get_record_id,
                                                  get_record_type,
                                                  get_record_value,
                                                  get_record_tags,
                                                  free_record,
                                                  get_storage_metadata,
                                                  set_storage_metadata,
                                                  free_storage_metadata,
                                                  search_records,
                                                  search_all_records,
                                                  get_search_total_count,
                                                  fetch_search_next_record,
                                                  free_search,
                                                  cb);

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
                            create: Option<wallet::WalletCreateCB>,
                            open: Option<wallet::WalletOpenCB>,
                            close: Option<wallet::WalletCloseCB>,
                            delete: Option<wallet::WalletDeleteCB>,
                            add_record: Option<wallet::WalletAddRecordCB>,
                            update_record_value: Option<wallet::WalletUpdateRecordValueCB>,
                            update_record_tags: Option<wallet::WalletUpdateRecordTagsCB>,
                            add_record_tags: Option<wallet::WalletAddRecordTagsCB>,
                            delete_record_tags: Option<wallet::WalletDeleteRecordTagsCB>,
                            delete_record: Option<wallet::WalletDeleteRecordCB>,
                            get_record: Option<wallet::WalletGetRecordCB>,
                            get_record_id: Option<wallet::WalletGetRecordIdCB>,
                            get_record_type: Option<wallet::WalletGetRecordTypeCB>,
                            get_record_value: Option<wallet::WalletGetRecordValueCB>,
                            get_record_tags: Option<wallet::WalletGetRecordTagsCB>,
                            free_record: Option<wallet::WalletFreeRecordCB>,
                            get_storage_metadata: Option<wallet::WalletGetStorageMetadataCB>,
                            set_storage_metadata: Option<wallet::WalletSetStorageMetadataCB>,
                            free_storage_metadata: Option<wallet::WalletFreeStorageMetadataCB>,
                            search_records: Option<wallet::WalletSearchRecordsCB>,
                            search_all_records: Option<wallet::WalletSearchAllRecordsCB>,
                            get_search_total_count: Option<wallet::WalletGetSearchTotalCountCB>,
                            fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecordCB>,
                            free_search: Option<wallet::WalletFreeSearchCB>,
                            timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register(command_handle,
                                              xtype,
                                                  create,
                                                  open,
                                                  close,
                                                  delete,
                                                  add_record,
                                                  update_record_value,
                                                  update_record_tags,
                                                  add_record_tags,
                                                  delete_record_tags,
                                                  delete_record,
                                                  get_record,
                                                  get_record_id,
                                                  get_record_type,
                                                  get_record_value,
                                                  get_record_tags,
                                                  free_record,
                                                  get_storage_metadata,
                                                  set_storage_metadata,
                                                  free_storage_metadata,
                                                  search_records,
                                                  search_all_records,
                                                  get_search_total_count,
                                                  fetch_search_next_record,
                                                  free_search,
                                                  cb);

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
                                      create: Option<wallet::WalletCreateCB>,
                                      open: Option<wallet::WalletOpenCB>,
                                      close: Option<wallet::WalletCloseCB>,
                                      delete: Option<wallet::WalletDeleteCB>,
                                      add_record: Option<wallet::WalletAddRecordCB>,
                                      update_record_value: Option<wallet::WalletUpdateRecordValueCB>,
                                      update_record_tags: Option<wallet::WalletUpdateRecordTagsCB>,
                                      add_record_tags: Option<wallet::WalletAddRecordTagsCB>,
                                      delete_record_tags: Option<wallet::WalletDeleteRecordTagsCB>,
                                      delete_record: Option<wallet::WalletDeleteRecordCB>,
                                      get_record: Option<wallet::WalletGetRecordCB>,
                                      get_record_id: Option<wallet::WalletGetRecordIdCB>,
                                      get_record_type: Option<wallet::WalletGetRecordTypeCB>,
                                      get_record_value: Option<wallet::WalletGetRecordValueCB>,
                                      get_record_tags: Option<wallet::WalletGetRecordTagsCB>,
                                      free_record: Option<wallet::WalletFreeRecordCB>,
                                      get_storage_metadata: Option<wallet::WalletGetStorageMetadataCB>,
                                      set_storage_metadata: Option<wallet::WalletSetStorageMetadataCB>,
                                      free_storage_metadata: Option<wallet::WalletFreeStorageMetadataCB>,
                                      search_records: Option<wallet::WalletSearchRecordsCB>,
                                      search_all_records: Option<wallet::WalletSearchAllRecordsCB>,
                                      get_search_total_count: Option<wallet::WalletGetSearchTotalCountCB>,
                                      fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecordCB>,
                                      free_search: Option<wallet::WalletFreeSearchCB>,
                                      closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_register(command_handle,
                                              xtype,
                                                  create,
                                                  open,
                                                  close,
                                                  delete,
                                                  add_record,
                                                  update_record_value,
                                                  update_record_tags,
                                                  add_record_tags,
                                                  delete_record_tags,
                                                  delete_record,
                                                  get_record,
                                                  get_record_id,
                                                  get_record_type,
                                                  get_record_value,
                                                  get_record_tags,
                                                  free_record,
                                                  get_storage_metadata,
                                                  set_storage_metadata,
                                                  free_storage_metadata,
                                                  search_records,
                                                  search_all_records,
                                                  get_search_total_count,
                                                  fetch_search_next_record,
                                                  free_search,
                                                  cb)
    }

    fn _register(command_handle: IndyHandle,
                 xtype: &str,
                 create: Option<wallet::WalletCreateCB>,
                 open: Option<wallet::WalletOpenCB>,
                 close: Option<wallet::WalletCloseCB>,
                 delete: Option<wallet::WalletDeleteCB>,
                 add_record: Option<wallet::WalletAddRecordCB>,
                 update_record_value: Option<wallet::WalletUpdateRecordValueCB>,
                 update_record_tags: Option<wallet::WalletUpdateRecordTagsCB>,
                 add_record_tags: Option<wallet::WalletAddRecordTagsCB>,
                 delete_record_tags: Option<wallet::WalletDeleteRecordTagsCB>,
                 delete_record: Option<wallet::WalletDeleteRecordCB>,
                 get_record: Option<wallet::WalletGetRecordCB>,
                 get_record_id: Option<wallet::WalletGetRecordIdCB>,
                 get_record_type: Option<wallet::WalletGetRecordTypeCB>,
                 get_record_value: Option<wallet::WalletGetRecordValueCB>,
                 get_record_tags: Option<wallet::WalletGetRecordTagsCB>,
                 free_record: Option<wallet::WalletFreeRecordCB>,
                 get_storage_metadata: Option<wallet::WalletGetStorageMetadataCB>,
                 set_storage_metadata: Option<wallet::WalletSetStorageMetadataCB>,
                 free_storage_metadata: Option<wallet::WalletFreeStorageMetadataCB>,
                 search_records: Option<wallet::WalletSearchRecordsCB>,
                 search_all_records: Option<wallet::WalletSearchAllRecordsCB>,
                 get_search_total_count: Option<wallet::WalletGetSearchTotalCountCB>,
                 fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecordCB>,
                 free_search: Option<wallet::WalletFreeSearchCB>,
                 cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);

        ErrorCode::from(unsafe {
                wallet::indy_register_wallet_storage(command_handle,
                                              xtype.as_ptr(),
                                                  create,
                                                  open,
                                                  close,
                                                  delete,
                                                  add_record,
                                                  update_record_value,
                                                  update_record_tags,
                                                  add_record_tags,
                                                  delete_record_tags,
                                                  delete_record,
                                                  get_record,
                                                  get_record_id,
                                                  get_record_type,
                                                  get_record_value,
                                                  get_record_tags,
                                                  free_record,
                                                  get_storage_metadata,
                                                  set_storage_metadata,
                                                  free_storage_metadata,
                                                  search_records,
                                                  search_all_records,
                                                  get_search_total_count,
                                                  fetch_search_next_record,
                                                  free_search,
                                                  cb)
        })
    }
    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `storage_type` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    pub fn create(pool_name: &str, wallet_name: &str, storage_type: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, pool_name, wallet_name, storage_type, config, credentials, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `storage_type` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_timeout(pool_name: &str, wallet_name: &str, storage_type: Option<&str>, config: Option<&str>, credentials: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, pool_name, wallet_name, storage_type, config, credentials, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `pool_name` - Name of the pool that corresponds to this wallet.
    /// * `name` - Name of the wallet.
    /// * `storage_type` (optional) - Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// * `config` (optional) - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` (optional) - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(pool_name: &str, wallet_name: &str, storage_type: Option<&str>, config: Option<&str>, credentials: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_create(command_handle, pool_name, wallet_name, storage_type, config, credentials, cb)
    }

    fn _create(command_handle: IndyHandle, pool_name: &str, wallet_name: &str, storage_type: Option<&str>, config: Option<&str>, credentials: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);
        let wallet_name = c_str!(wallet_name);
        let storage_type_str = opt_c_str!(storage_type);
        let config_str = opt_c_str!(config);
        let credentials = Wallet::_default_credentials(credentials);

        ErrorCode::from(unsafe {
            wallet::indy_create_wallet(command_handle,
                                       pool_name.as_ptr(),
                                       wallet_name.as_ptr(),
                                       opt_c_ptr!(storage_type, storage_type_str),
                                       opt_c_ptr!(config, config_str),
                                       credentials.as_ptr(),
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
        let credentials = Wallet::_default_credentials(credentials);

        ErrorCode::from(unsafe {
            wallet::indy_open_wallet(command_handle,
                                     wallet_name.as_ptr(),
                                     opt_c_ptr!(config, config_str),
                                     credentials.as_ptr(),
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
        let credentials = Wallet::_default_credentials(credentials);

        ErrorCode::from(unsafe { wallet::indy_delete_wallet(command_handle, wallet_name.as_ptr(), credentials.as_ptr(), cb) })
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

    /// Create a new non-secret record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the value of record
    /// * `tags_json` -  the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   Note that null means no tags
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    pub fn add_record(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_add_record(command_handle, wallet_handle, xtype, id, value, tags_json, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Create a new non-secret record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the value of record
    /// * `tags_json` -  the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   Note that null means no tags
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    /// * `timeout` - the maximum time this function waits for a response
    pub fn add_record_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_add_record(command_handle, wallet_handle, xtype, id, value, tags_json, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Create a new non-secret record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the value of record
    /// * `tags_json` -  the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   Note that null means no tags
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn add_record_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_add_record(command_handle, wallet_handle, xtype, id, value, tags_json, cb)
    }

    fn _add_record(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let value = c_str!(value);
        let tags_json_str = opt_c_str!(tags_json);
        ErrorCode::from(unsafe {
            non_secrets::indy_add_wallet_record(command_handle,
                                                wallet_handle,
                                                xtype.as_ptr(),
                                                id.as_ptr(),
                                                value.as_ptr(),
                                                opt_c_ptr!(tags_json, tags_json_str),
                                                 cb)
        })
    }

    /// Update a non-secret wallet record value
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the new value of record
    pub fn update_record_value(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_update_record_value(command_handle, wallet_handle, xtype, id, value, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Update a non-secret wallet record value
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the new value of record
    /// * `timeout` - the maximum time this function waits for a response
    pub fn update_record_value_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_update_record_value(command_handle, wallet_handle, xtype, id, value, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Update a non-secret wallet record value
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `value` - the new value of record
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn update_record_value_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_update_record_value(command_handle, wallet_handle, xtype, id, value, cb)
    }

    fn _update_record_value(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, value: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let value = c_str!(value);

        ErrorCode::from(unsafe{
            non_secrets::indy_update_wallet_record_value(command_handle,
                                                         wallet_handle,
                                                         xtype.as_ptr(),
                                                         id.as_ptr(),
                                                         value.as_ptr(),
                                                         cb)
        })
    }

    /// Update a non-secret wallet record tags
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    pub fn update_record_tags(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_update_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Update a non-secret wallet record tags
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    /// * `timeout` - the maximum time this function waits for a response
    pub fn update_record_tags_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_update_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Update a non-secret wallet record tags
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn update_record_tags_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_update_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb)
    }

    fn _update_record_tags(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let tags_json = c_str!(tags_json);

        ErrorCode::from(unsafe {
          non_secrets::indy_update_wallet_record_tags(command_handle, wallet_handle, xtype.as_ptr(), id.as_ptr(), tags_json.as_ptr(), cb)
        })
    }

    /// Add new tags to the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    ///   Note if some from provided tags already assigned to the record than
    ///     corresponding tags values will be replaced
    pub fn add_record_tags(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_add_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Add new tags to the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    ///   Note if some from provided tags already assigned to the record than
    ///     corresponding tags values will be replaced
    /// * `timeout` - the maximum time this function waits for a response
    pub fn add_record_tags_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_add_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Add new tags to the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tags_json` - the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    ///   Note if some from provided tags already assigned to the record than
    ///     corresponding tags values will be replaced
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn add_record_tags_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_add_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb)
    }

    fn _add_record_tags(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, tags_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let tags_json = c_str!(tags_json);

        ErrorCode::from(unsafe {
          non_secrets::indy_add_wallet_record_tags(command_handle, wallet_handle, xtype.as_ptr(), id.as_ptr(), tags_json.as_ptr(), cb)
        })
    }

    /// Delete tags from the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tag_names_json` - the list of tag names to remove from the record as json array:
    ///   ["tagName1", "tagName2", ...]
    pub fn delete_record_tags(wallet_handle: IndyHandle, xtype: &str, id: &str, tag_names_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete_record_tags(command_handle, wallet_handle, xtype, id, tag_names_json, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Delete tags from the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tag_names_json` - the list of tag names to remove from the record as json array:
    ///   ["tagName1", "tagName2", ...]
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_record_tags_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, tag_names_json: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete_record_tags(command_handle, wallet_handle, xtype, id, tag_names_json, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Delete tags from the wallet record
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `tag_names_json` - the list of tag names to remove from the record as json array:
    ///   ["tagName1", "tagName2", ...]
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_record_tags_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, tag_names_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_delete_record_tags(command_handle, wallet_handle, xtype, id, tag_names_json, cb)
    }

    fn _delete_record_tags(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, tag_names_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let tag_names_json = c_str!(tag_names_json);

        ErrorCode::from(unsafe {
          non_secrets::indy_delete_wallet_record_tags(command_handle, wallet_handle, xtype.as_ptr(), id.as_ptr(), tag_names_json.as_ptr(), cb)
        })
    }

    /// Delete an existing wallet record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - record type
    /// * `id` - the id of record
    pub fn delete_record(wallet_handle: IndyHandle, xtype: &str, id: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete_record(command_handle, wallet_handle, xtype, id, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Delete an existing wallet record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - record type
    /// * `id` - the id of record
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_record_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete_record(command_handle, wallet_handle, xtype, id, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Delete an existing wallet record in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - record type
    /// * `id` - the id of record
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_record_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_delete_record(command_handle, wallet_handle, xtype, id, cb)
    }

    fn _delete_record(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);

        ErrorCode::from(unsafe {
          non_secrets::indy_delete_wallet_record(command_handle, wallet_handle, xtype.as_ptr(), id.as_ptr(), cb)
        })
    }

    /// Get an wallet record by id
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags
    ///  }
    /// # Returns
    /// * `wallet record json` -
    /// {
    ///   id: "Some id",
    ///   type: "Some type", // present only if retrieveType set to true
    ///   value: "Some value", // present only if retrieveValue set to true
    ///   tags: <tags json>, // present only if retrieveTags set to true
    /// }
    pub fn get_record(wallet_handle: IndyHandle, xtype: &str, id: &str, options_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_get_record(command_handle, wallet_handle, xtype, id, options_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Get an wallet record by id
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags
    ///  }
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// * `wallet record json` -
    /// {
    ///   id: "Some id",
    ///   type: "Some type", // present only if retrieveType set to true
    ///   value: "Some value", // present only if retrieveValue set to true
    ///   tags: <tags json>, // present only if retrieveTags set to true
    /// }
    pub fn get_record_timeout(wallet_handle: IndyHandle, xtype: &str, id: &str, options_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_get_record(command_handle, wallet_handle, xtype, id, options_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Get an wallet record by id
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `id` - the id of record
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags
    ///  }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_record_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, id: &str, options_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Wallet::_get_record(command_handle, wallet_handle, xtype, id, options_json, cb)
    }

    fn _get_record(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, id: &str, options_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let id = c_str!(id);
        let options_json = c_str!(options_json);

        ErrorCode::from(unsafe {
          non_secrets::indy_get_wallet_record(command_handle, wallet_handle, xtype.as_ptr(), id.as_ptr(), options_json.as_ptr(), cb)
        })
    }

    /// Search for wallet records.
    ///
    /// Note instead of immediately returning of fetched records
    /// this call returns wallet_search_handle that can be used later
    /// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `query_json` - MongoDB style query to wallet record tags:
    ///  {
    ///    "tagName": "tagValue",
    ///    $or: {
    ///      "tagName2": { $regex: 'pattern' },
    ///      "tagName3": { $gte: '123' },
    ///    },
    ///  }
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
    ///    retrieveTotalCount: (optional, false by default) Calculate total count,
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags,
    ///  }
    /// # Returns
    /// * `search_handle` - Wallet search handle that can be used later
    ///   to fetch records by small batches (with indy_fetch_wallet_search_next_records)
    pub fn open_search(wallet_handle: IndyHandle, xtype: &str, query_json: &str, options_json: &str) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open_search(command_handle, wallet_handle, xtype, query_json, options_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Search for wallet records.
    ///
    /// Note instead of immediately returning of fetched records
    /// this call returns wallet_search_handle that can be used later
    /// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `query_json` - MongoDB style query to wallet record tags:
    ///  {
    ///    "tagName": "tagValue",
    ///    $or: {
    ///      "tagName2": { $regex: 'pattern' },
    ///      "tagName3": { $gte: '123' },
    ///    },
    ///  }
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
    ///    retrieveTotalCount: (optional, false by default) Calculate total count,
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags,
    ///  }
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// * `search_handle` - Wallet search handle that can be used later
    ///   to fetch records by small batches (with indy_fetch_wallet_search_next_records)
    pub fn open_search_timeout(wallet_handle: IndyHandle, xtype: &str, query_json: &str, options_json: &str, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open_search(command_handle, wallet_handle, xtype, query_json, options_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Search for wallet records.
    ///
    /// Note instead of immediately returning of fetched records
    /// this call returns wallet_search_handle that can be used later
    /// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `xtype` - allows to separate different record types collections
    /// * `query_json` - MongoDB style query to wallet record tags:
    ///  {
    ///    "tagName": "tagValue",
    ///    $or: {
    ///      "tagName2": { $regex: 'pattern' },
    ///      "tagName3": { $gte: '123' },
    ///    },
    ///  }
    /// * `options_json` - //TODO: FIXME: Think about replacing by bitmaks
    ///  {
    ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
    ///    retrieveTotalCount: (optional, false by default) Calculate total count,
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, false by default) Retrieve record tags,
    ///  }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn open_search_async<F: 'static>(wallet_handle: IndyHandle, xtype: &str, query_json: &str, options_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Wallet::_open_search(command_handle, wallet_handle, xtype, query_json, options_json, cb)
    }

    fn _open_search(command_handle: IndyHandle, wallet_handle: IndyHandle, xtype: &str, query_json: &str, options_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
        let xtype = c_str!(xtype);
        let query_json = c_str!(query_json);
        let options_json = c_str!(options_json);

        ErrorCode::from(unsafe {
          non_secrets::indy_open_wallet_search(command_handle, wallet_handle, xtype.as_ptr(), query_json.as_ptr(), options_json.as_ptr(), cb)
        })
    }

    /// Fetch next records for wallet search.
    ///
    /// Not if there are no records this call returns WalletNoRecords error.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `wallet_search_handle` - wallet search handle (created by indy_open_wallet_search)
    /// * `count` - Count of records to fetch
    ///
    /// # Returns
    /// * `wallet records json` -
    /// {
    ///   totalCount: <str>, // present only if retrieveTotalCount set to true
    ///   records: [{ // present only if retrieveRecords set to true
    ///       id: "Some id",
    ///       type: "Some type", // present only if retrieveType set to true
    ///       value: "Some value", // present only if retrieveValue set to true
    ///       tags: <tags json>, // present only if retrieveTags set to true
    ///   }],
    /// }
    pub fn fetch_search_next_records(wallet_handle: IndyHandle, wallet_search_handle: IndyHandle, count: usize) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_fetch_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb);

        ResultHandler::one(err, receiver)
    }

    /// Fetch next records for wallet search.
    ///
    /// Not if there are no records this call returns WalletNoRecords error.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `wallet_search_handle` - wallet search handle (created by indy_open_wallet_search)
    /// * `count` - Count of records to fetch
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `wallet records json` -
    /// {
    ///   totalCount: <str>, // present only if retrieveTotalCount set to true
    ///   records: [{ // present only if retrieveRecords set to true
    ///       id: "Some id",
    ///       type: "Some type", // present only if retrieveType set to true
    ///       value: "Some value", // present only if retrieveValue set to true
    ///       tags: <tags json>, // present only if retrieveTags set to true
    ///   }],
    /// }
    pub fn fetch_search_next_records_timeout(wallet_handle: IndyHandle, wallet_search_handle: IndyHandle, count: usize, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Wallet::_fetch_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Fetch next records for wallet search.
    ///
    /// Not if there are no records this call returns WalletNoRecords error.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by open_wallet)
    /// * `wallet_search_handle` - wallet search handle (created by indy_open_wallet_search)
    /// * `count` - Count of records to fetch
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn fetch_search_next_records_async<F: 'static>(wallet_handle: IndyHandle, wallet_search_handle: IndyHandle, count: usize, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Wallet::_fetch_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb)
    }

    fn _fetch_search_next_records(command_handle: IndyHandle, wallet_handle: IndyHandle, wallet_search_handle: IndyHandle, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe {
          non_secrets::indy_fetch_wallet_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb)
        })
    }

    /// Close wallet search (make search handle invalid)
    ///
    /// # Arguments
    /// * `wallet_search_handle` - wallet search handle
    pub fn close_search(wallet_search_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_close_search(command_handle, wallet_search_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Close wallet search (make search handle invalid)
    ///
    /// # Arguments
    /// * `wallet_search_handle` - wallet search handle
    /// * `timeout` - the maximum time this function waits for a response
    pub fn close_search_timeout(wallet_search_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_close_search(command_handle, wallet_search_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Close wallet search (make search handle invalid)
    ///
    /// # Arguments
    /// * `wallet_search_handle` - wallet search handle
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn close_search_async<F: 'static>(wallet_search_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_close_search(command_handle, wallet_search_handle, cb)
    }

    fn _close_search(command_handle: IndyHandle, wallet_search_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe {
          non_secrets::indy_close_wallet_search(command_handle, wallet_search_handle, cb)
        })
    }

    fn _default_credentials(credentials: Option<&str>) -> CString {
        match credentials {
            Some(s) => c_str!(s),
            None => c_str!(r#"{"key":""}"#)
        }
    }
}
