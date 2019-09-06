use futures::Future;

use {ErrorCode, IndyError};

use std::ffi::CString;
use std::ptr::null;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::{wallet, non_secrets};
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseI32CB,
          ResponseWalletHandleCB};
use {CommandHandle, WalletHandle, SearchHandle};

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
pub fn register_wallet_storage(xtype: &str,
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
                               free_search: Option<wallet::WalletFreeSearch>) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _register_storage(command_handle,
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

    ResultHandler::empty(command_handle, err, receiver)
}

fn _register_storage(command_handle: CommandHandle,
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
pub fn create_wallet(config: &str, credentials: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _create_wallet(command_handle, config, credentials, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _create_wallet(command_handle: CommandHandle, config: &str, credentials: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn open_wallet(config: &str, credentials: &str) -> Box<dyn Future<Item=WalletHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_wallethandle();

    let err = _open_wallet(command_handle, config, credentials, cb);

    ResultHandler::wallethandle(command_handle, err, receiver)
}

fn _open_wallet(command_handle: CommandHandle, config: &str, credentials: &str, cb: Option<ResponseWalletHandleCB>) -> ErrorCode {
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
pub fn export_wallet(wallet_handle: WalletHandle, export_config: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _export_wallet(command_handle, wallet_handle, export_config, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _export_wallet(command_handle: CommandHandle, wallet_handle: WalletHandle, export_config: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let export_config = c_str!(export_config);

    ErrorCode::from(unsafe {
      wallet::indy_export_wallet(command_handle, wallet_handle, export_config.as_ptr(), cb)
    })
}

/// Creates a new secure wallet with the given unique name and then imports its content
/// according to fields provided in import_config
/// This can be seen as an create call with additional content import
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
pub fn import_wallet(config: &str, credentials: &str, import_config: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _import_wallet(command_handle, config, credentials, import_config, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _import_wallet(command_handle: CommandHandle, config: &str, credentials: &str, import_config: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let config = c_str!(config);
    let credentials = c_str!(credentials);
    let import_config = c_str!(import_config);

    ErrorCode::from(unsafe {
      wallet::indy_import_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), import_config.as_ptr(), cb)
    })
}

/// Deletes created wallet.
pub fn delete_wallet(config: &str, credentials: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _delete_wallet(command_handle, config, credentials, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _delete_wallet(command_handle: CommandHandle, config: &str, credentials: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let config = c_str!(config);
    let credentials = c_str!(credentials);

    ErrorCode::from(unsafe {
      wallet::indy_delete_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb)
    })
}

/// Closes opened wallet and frees allocated resources.
///
/// # Arguments
/// * `handle` - wallet handle returned by open.
pub fn close_wallet(wallet_handle: WalletHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _close_wallet(command_handle, wallet_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _close_wallet(command_handle: CommandHandle, wallet_handle: WalletHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn add_wallet_record(wallet_handle: WalletHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _add_wallet_record(command_handle, wallet_handle, xtype, id, value, tags_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _add_wallet_record(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, value: &str, tags_json: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn update_wallet_record_value(wallet_handle: WalletHandle, xtype: &str, id: &str, value: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _update_wallet_record_value(command_handle, wallet_handle, xtype, id, value, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _update_wallet_record_value(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, value: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn update_wallet_record_tags(wallet_handle: WalletHandle, xtype: &str, id: &str, tags_json: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _update_wallet_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _update_wallet_record_tags(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, tags_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn add_wallet_record_tags(wallet_handle: WalletHandle, xtype: &str, id: &str, tags_json: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _add_wallet_record_tags(command_handle, wallet_handle, xtype, id, tags_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _add_wallet_record_tags(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, tags_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn delete_wallet_record_tags(wallet_handle: WalletHandle, xtype: &str, id: &str, tag_names_json: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _delete_wallet_record_tags(command_handle, wallet_handle, xtype, id, tag_names_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _delete_wallet_record_tags(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, tag_names_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn delete_wallet_record(wallet_handle: WalletHandle, xtype: &str, id: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _delete_wallet_record(command_handle, wallet_handle, xtype, id, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _delete_wallet_record(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
pub fn get_wallet_record(wallet_handle: WalletHandle, xtype: &str, id: &str, options_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_wallet_record(command_handle, wallet_handle, xtype, id, options_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_wallet_record(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, id: &str, options_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
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
pub fn open_wallet_search(wallet_handle: WalletHandle, xtype: &str, query_json: &str, options_json: &str) -> Box<dyn Future<Item=SearchHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_wallet_search(command_handle, wallet_handle, xtype, query_json, options_json, cb);

    ResultHandler::handle(command_handle, err, receiver)
}

fn _open_wallet_search(command_handle: CommandHandle, wallet_handle: WalletHandle, xtype: &str, query_json: &str, options_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
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
pub fn fetch_wallet_search_next_records(wallet_handle: WalletHandle, wallet_search_handle: SearchHandle, count: usize) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _fetch_wallet_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _fetch_wallet_search_next_records(command_handle: CommandHandle, wallet_handle: WalletHandle, wallet_search_handle: SearchHandle, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
      non_secrets::indy_fetch_wallet_search_next_records(command_handle, wallet_handle, wallet_search_handle, count, cb)
    })
}

/// Close wallet search (make search handle invalid)
///
/// # Arguments
/// * `wallet_search_handle` - wallet search handle
pub fn close_wallet_search(wallet_search_handle: SearchHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _close_wallet_search(command_handle, wallet_search_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _close_wallet_search(command_handle: CommandHandle, wallet_search_handle: SearchHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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

/// Generate wallet master key.
/// Returned key is compatible with "RAW" key derivation method.
/// It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.
///
/// # Arguments
/// * `config` - (optional) key configuration json.
/// {
///   "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
///                              Can be UTF-8, base64 or hex string.
/// }
///
/// # Returns
/// wallet key can be used with RAW derivation type
pub fn generate_wallet_key(config: Option<&str>) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _generate_wallet_key(command_handle, config, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _generate_wallet_key(command_handle: CommandHandle, config: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let config = opt_c_str_json!(config);

    ErrorCode::from(unsafe { wallet::indy_generate_wallet_key(command_handle, config.as_ptr(), cb) })
}
