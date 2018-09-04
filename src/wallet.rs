use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;
use std::time::Duration;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use native::{wallet, non_secrets};
use native::{ResponseEmptyCB,
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
    pub fn register_storage(xtype: &str,
                            create: Option<wallet::WalletCreate>,
                            open: Option<wallet::WalletOpen>,
                            close: Option<wallet::WalletClose>,
                            delete: Option<wallet::WalletDelete>,
                            add_record: Option<wallet::WalletAddRecord>,
                            update_record_value: Option<wallet::WalletUpdateRecordValue>,
                            update_record_tags: Option<wallet::WalletUpdateRecordTags>,
                            add_record_tags: Option<wallet::WalletAddRecordTags>,
                            delete_record_tags: Option<wallet::WalletDeleteRecordTags>,
                            delete_record: Option<wallet::WalletDeleteRecord>,
                            get_record: Option<wallet::WalletGetRecord>,
                            get_record_id: Option<wallet::WalletGetRecordId>,
                            get_record_type: Option<wallet::WalletGetRecordType>,
                            get_record_value: Option<wallet::WalletGetRecordValue>,
                            get_record_tags: Option<wallet::WalletGetRecordTags>,
                            free_record: Option<wallet::WalletFreeRecord>,
                            get_storage_metadata: Option<wallet::WalletGetStorageMetadata>,
                            set_storage_metadata: Option<wallet::WalletSetStorageMetadata>,
                            free_storage_metadata: Option<wallet::WalletFreeStorageMetadata>,
                            search_records: Option<wallet::WalletSearchRecords>,
                            search_all_records: Option<wallet::WalletSearchAllRecords>,
                            get_search_total_count: Option<wallet::WalletGetSearchTotalCount>,
                            fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecord>,
                            free_search: Option<wallet::WalletFreeSearch>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register_storage(command_handle,
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
    pub fn register_storage_timeout(xtype: &str,
                                    create: Option<wallet::WalletCreate>,
                                    open: Option<wallet::WalletOpen>,
                                    close: Option<wallet::WalletClose>,
                                    delete: Option<wallet::WalletDelete>,
                                    add_record: Option<wallet::WalletAddRecord>,
                                    update_record_value: Option<wallet::WalletUpdateRecordValue>,
                                    update_record_tags: Option<wallet::WalletUpdateRecordTags>,
                                    add_record_tags: Option<wallet::WalletAddRecordTags>,
                                    delete_record_tags: Option<wallet::WalletDeleteRecordTags>,
                                    delete_record: Option<wallet::WalletDeleteRecord>,
                                    get_record: Option<wallet::WalletGetRecord>,
                                    get_record_id: Option<wallet::WalletGetRecordId>,
                                    get_record_type: Option<wallet::WalletGetRecordType>,
                                    get_record_value: Option<wallet::WalletGetRecordValue>,
                                    get_record_tags: Option<wallet::WalletGetRecordTags>,
                                    free_record: Option<wallet::WalletFreeRecord>,
                                    get_storage_metadata: Option<wallet::WalletGetStorageMetadata>,
                                    set_storage_metadata: Option<wallet::WalletSetStorageMetadata>,
                                    free_storage_metadata: Option<wallet::WalletFreeStorageMetadata>,
                                    search_records: Option<wallet::WalletSearchRecords>,
                                    search_all_records: Option<wallet::WalletSearchAllRecords>,
                                    get_search_total_count: Option<wallet::WalletGetSearchTotalCount>,
                                    fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecord>,
                                    free_search: Option<wallet::WalletFreeSearch>,
                                    timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_register_storage(command_handle,
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
    pub fn register_storage_async<F: 'static>(xtype: &str,
                                              create: Option<wallet::WalletCreate>,
                                              open: Option<wallet::WalletOpen>,
                                              close: Option<wallet::WalletClose>,
                                              delete: Option<wallet::WalletDelete>,
                                              add_record: Option<wallet::WalletAddRecord>,
                                              update_record_value: Option<wallet::WalletUpdateRecordValue>,
                                              update_record_tags: Option<wallet::WalletUpdateRecordTags>,
                                              add_record_tags: Option<wallet::WalletAddRecordTags>,
                                              delete_record_tags: Option<wallet::WalletDeleteRecordTags>,
                                              delete_record: Option<wallet::WalletDeleteRecord>,
                                              get_record: Option<wallet::WalletGetRecord>,
                                              get_record_id: Option<wallet::WalletGetRecordId>,
                                              get_record_type: Option<wallet::WalletGetRecordType>,
                                              get_record_value: Option<wallet::WalletGetRecordValue>,
                                              get_record_tags: Option<wallet::WalletGetRecordTags>,
                                              free_record: Option<wallet::WalletFreeRecord>,
                                              get_storage_metadata: Option<wallet::WalletGetStorageMetadata>,
                                              set_storage_metadata: Option<wallet::WalletSetStorageMetadata>,
                                              free_storage_metadata: Option<wallet::WalletFreeStorageMetadata>,
                                              search_records: Option<wallet::WalletSearchRecords>,
                                              search_all_records: Option<wallet::WalletSearchAllRecords>,
                                              get_search_total_count: Option<wallet::WalletGetSearchTotalCount>,
                                              fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecord>,
                                              free_search: Option<wallet::WalletFreeSearch>,
                                              closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_register_storage(command_handle,
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

    fn _register_storage(command_handle: IndyHandle,
                         xtype: &str,
                         create: Option<wallet::WalletCreate>,
                         open: Option<wallet::WalletOpen>,
                         close: Option<wallet::WalletClose>,
                         delete: Option<wallet::WalletDelete>,
                         add_record: Option<wallet::WalletAddRecord>,
                         update_record_value: Option<wallet::WalletUpdateRecordValue>,
                         update_record_tags: Option<wallet::WalletUpdateRecordTags>,
                         add_record_tags: Option<wallet::WalletAddRecordTags>,
                         delete_record_tags: Option<wallet::WalletDeleteRecordTags>,
                         delete_record: Option<wallet::WalletDeleteRecord>,
                         get_record: Option<wallet::WalletGetRecord>,
                         get_record_id: Option<wallet::WalletGetRecordId>,
                         get_record_type: Option<wallet::WalletGetRecordType>,
                         get_record_value: Option<wallet::WalletGetRecordValue>,
                         get_record_tags: Option<wallet::WalletGetRecordTags>,
                         free_record: Option<wallet::WalletFreeRecord>,
                         get_storage_metadata: Option<wallet::WalletGetStorageMetadata>,
                         set_storage_metadata: Option<wallet::WalletSetStorageMetadata>,
                         free_storage_metadata: Option<wallet::WalletFreeStorageMetadata>,
                         search_records: Option<wallet::WalletSearchRecords>,
                         search_all_records: Option<wallet::WalletSearchAllRecords>,
                         get_search_total_count: Option<wallet::WalletGetSearchTotalCount>,
                         fetch_search_next_record: Option<wallet::WalletFetchSearchNextRecord>,
                         free_search: Option<wallet::WalletFreeSearch>,
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
    /// * `config` - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    pub fn create(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, config, credentials, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `config` - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_timeout(config: &str, credentials: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_create(command_handle, config, credentials, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new secure wallet with the given unique name.
    ///
    /// # Arguments
    /// * `config` - Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `credentials` - Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(config: &str, credentials: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_create(command_handle, config, credentials, cb)
    }

    fn _create(command_handle: IndyHandle, config: &str, credentials: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let config = c_str!(config);
        let credentials = c_str!(credentials);

        ErrorCode::from(unsafe {
          wallet::indy_create_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        })
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
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
    pub fn open(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open(command_handle, config, credentials, cb);

        ResultHandler::one(err, receiver)
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
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
    pub fn open_timeout(config: &str, credentials: &str, timeout: Duration) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Wallet::_open(command_handle, config, credentials, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// # Arguments
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
    pub fn open_async<F: 'static>(config: &str, credentials: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, i32) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Wallet::_open(command_handle, config, credentials, cb)
    }

    fn _open(command_handle: IndyHandle, config: &str, credentials: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
        let config = c_str!(config);
        let credentials = c_str!(credentials);

        ErrorCode::from(unsafe {
          wallet::indy_open_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        })
    }

    /// Exports opened wallet
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments:
    /// * `wallet_handle` - wallet handle returned by indy_open_wallet
    /// * `export_config` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    pub fn export(wallet_handle: IndyHandle, export_config: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_export(command_handle, wallet_handle, export_config, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Exports opened wallet
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments:
    /// * `wallet_handle` - wallet handle returned by indy_open_wallet
    /// * `export_config` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    /// * `timeout` - the maximum time this function waits for a response
    pub fn export_timeout(wallet_handle: IndyHandle, export_config: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_export(command_handle, wallet_handle, export_config, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Exports opened wallet
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments:
    /// * `wallet_handle` - wallet handle returned by indy_open_wallet
    /// * `export_config` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn export_async<F: 'static>(wallet_handle: IndyHandle, export_config: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_export(command_handle, wallet_handle, export_config, cb)
    }

    fn _export(command_handle: IndyHandle, wallet_handle: IndyHandle, export_config: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let export_config = c_str!(export_config);

        ErrorCode::from(unsafe {
          wallet::indy_export_wallet(command_handle, wallet_handle, export_config.as_ptr(), cb)
        })
    }

    /// Creates a new secure wallet with the given unique name and then imports its content
    /// according to fields provided in import_config
    /// This can be seen as an Wallet::create call with additional content import
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments
    /// * `config` - Wallet configuration json.
    ///   {
    ///       "storage": <object>  List of supported keys are defined by wallet type.
    ///   }
    /// * `credentials` - Wallet credentials json (if NULL, then default config will be used).
    ///   {
    ///       "key": string,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    /// * `import_config` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    pub fn import(config: &str, credentials: &str, import_config: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_import(command_handle, config, credentials, import_config, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new secure wallet with the given unique name and then imports its content
    /// according to fields provided in import_config
    /// This can be seen as an Wallet::create call with additional content import
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments
    /// * `config` - Wallet configuration json.
    ///   {
    ///       "storage": <object>  List of supported keys are defined by wallet type.
    ///   }
    /// * `credentials` - Wallet credentials json (if NULL, then default config will be used).
    ///   {
    ///       "key": string,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    /// * `import_config` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    /// * `timeout` - the maximum time this function waits for a response
    pub fn import_timeout(config: &str, credentials: &str, import_config: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_import(command_handle, config, credentials, import_config, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new secure wallet with the given unique name and then imports its content
    /// according to fields provided in import_config
    /// This can be seen as an Wallet::create call with additional content import
    ///
    /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
    /// in the future releases.
    ///
    /// # Arguments
    /// * `config` - Wallet configuration json.
    ///   {
    ///       "storage": <object>  List of supported keys are defined by wallet type.
    ///   }
    /// * `credentials` - Wallet credentials json (if NULL, then default config will be used).
    ///   {
    ///       "key": string,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    /// * `import_config_json` - JSON containing settings for input operation.
    ///   {
    ///     "path": path of the file that contains exported wallet content
    ///     "key": passphrase used to derive export key
    ///   }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn import_async<F: 'static>(config: &str, credentials: &str, import_config: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_import(command_handle, config, credentials, import_config, cb)
    }

    fn _import(command_handle: IndyHandle, config: &str, credentials: &str, import_config: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let config = c_str!(config);
        let credentials = c_str!(credentials);
        let import_config = c_str!(import_config);

        ErrorCode::from(unsafe {
          wallet::indy_import_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), import_config.as_ptr(), cb)
        })
    }

    /// Deletes created wallet.
    pub fn delete(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete(command_handle, config, credentials, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Deletes created wallet.
    ///
    /// # Arguments
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_timeout(config: &str, credentials: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Wallet::_delete(command_handle, config, credentials, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Deletes created wallet.
    ///
    /// # Arguments
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_async<F: 'static>(config: &str, credentials: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Wallet::_delete(command_handle, config, credentials, cb)
    }

    fn _delete(command_handle: IndyHandle, config: &str, credentials: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let config = c_str!(config);
        let credentials = c_str!(credentials);

        ErrorCode::from(unsafe {
          wallet::indy_delete_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
        })
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
