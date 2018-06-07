extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::wallet::WalletCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

/// Registers custom wallet storage implementation.
///
/// It allows library user to provide custom wallet implementation.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// type_: Wallet type name.
/// create: WalletType create operation handler
/// open: WalletType open operation handler
/// close: Wallet close operation handler
/// delete: WalletType delete operation handler
/// add_record: WalletType add record operation handler
/// update_record_value: WalletType update record value operation handler
/// update_record_tags: WalletType update record tags operation handler
/// add_record_tags: WalletType add record tags operation handler
/// delete_record_tags: WalletType delete record tags operation handler
/// delete_record: WalletType delete record operation handler
/// get_record: WalletType get record operation handler
/// get_record_id: WalletType get record id operation handler
/// get_record_type: WalletType get record type operation handler
/// get_record_value: WalletType get record value operation handler
/// get_record_tags: WalletType get record tags operation handler
/// free_record: WalletType free record operation handler
/// search_records: WalletType search records operation handler
/// search_all_records: WalletType search all records operation handler
/// get_search_total_count: WalletType get search total count operation handler
/// fetch_search_next_record: WalletType fetch search next record operation handler
/// free_search: WalletType free search operation handler
/// free: Handler that allows to de-allocate strings allocated in caller code
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_register_wallet_storage(command_handle: i32,
                                           type_: *const c_char,
                                           create: Option<WalletCreate>,
                                           open: Option<WalletOpen>,
                                           close: Option<WalletClose>,
                                           delete: Option<WalletDelete>,
                                           add_record: Option<WalletAddRecord>,
                                           update_record_value: Option<WalletUpdateRecordValue>,
                                           update_record_tags: Option<WalletUpdateRecordTags>,
                                           add_record_tags: Option<WalletAddRecordTags>,
                                           delete_record_tags: Option<WalletDeleteRecordTags>,
                                           delete_record: Option<WalletDeleteRecord>,
                                           get_record: Option<WalletGetRecord>,
                                           get_record_id: Option<WalletGetRecordId>,
                                           get_record_type: Option<WalletGetRecordType>,
                                           get_record_value: Option<WalletGetRecordValue>,
                                           get_record_tags: Option<WalletGetRecordTags>,
                                           free_record: Option<WalletFreeRecord>,
                                           get_storage_metadata: Option<WalletGetStorageMetadata>,
                                           set_storage_metadata: Option<WalletSetStorageMetadata>,
                                           free_storage_metadata: Option<WalletFreeStorageMetadata>,
                                           search_records: Option<WalletSearchRecords>,
                                           search_all_records: Option<WalletSearchAllRecords>,
                                           get_search_total_count: Option<WalletGetSearchTotalCount>,
                                           fetch_search_next_record: Option<WalletFetchSearchNextRecord>,
                                           free_search: Option<WalletFreeSearch>,
                                           cb: Option<extern fn(xcommand_handle: i32,
                                                                err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(create, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(open, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(close, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(delete, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(add_record, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(update_record_value, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(update_record_tags, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(add_record_tags, ErrorCode::CommonInvalidParam10);
    check_useful_c_callback!(delete_record_tags, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(delete_record, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_record, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_record_id, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_record_type, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_record_value, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_record_tags, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(free_record, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_storage_metadata, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(set_storage_metadata, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(free_storage_metadata, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(search_records, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(search_all_records, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(get_search_total_count, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(fetch_search_next_record, ErrorCode::CommonInvalidParam11); // TODO: CommonInvalidParam.......
    check_useful_c_callback!(free_search, ErrorCode::CommonInvalidParam11); // TODO: CommonInvalidParam.......
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam12);

    trace!("indy_register_wallet_type: entities >>> type_: {:?}", type_);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(
            WalletCommand::RegisterWalletType(
                type_,
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
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_register_wallet_type: <<< res: {:?}", res);

    res
}

/// Creates a new secure wallet with the given unique name.
///
/// #Params
/// pool_name: Name of the pool that corresponds to this wallet.
/// name: Name of the wallet.
/// storage_type(optional): Type of the wallet storage. Defaults to 'default'.
///                  Custom storage types can be registered with indy_register_wallet_storage call.
/// config(optional): Wallet configuration json.
///   {
///       "storage": <object>  List of supported keys are defined by wallet type.
///   }
/// credentials: Wallet credentials json
///   {
///       "key": string,
///       "rekey": Optional<string>,
///       "storage": Optional<object>  List of supported keys are defined by wallet type.
///
///   }
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
                                 storage_type: *const c_char,
                                 config: *const c_char,
                                 credentials_json: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: ErrorCode)>) -> ErrorCode {
    trace!("indy_create_wallet: >>> pool_name: {:?}, name: {:?}, storage_type: {:?}, config: {:?}, credentials_json: {:?}",
           pool_name, name, storage_type, config, credentials_json);

    check_useful_c_str!(pool_name, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(name, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(storage_type, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(config, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(credentials_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_create_wallet: entities >>> pool_name: {:?}, name: {:?}, storage_type: {:?}, config: {:?}, credentials_json: {:?}",
           pool_name, name, storage_type, config, credentials_json);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Create(
            pool_name,
            name,
            storage_type,
            config,
            credentials_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                trace!("indy_create_wallet:");
                cb(command_handle, err)
            })
        )));

    let res = result_to_err_code!(result);

    trace!("indy_create_wallet: <<< res: {:?}", res);

    res
}

/// Opens the wallet with specific name.
///
/// Wallet with corresponded name must be previously created with indy_create_wallet method.
/// It is impossible to open wallet with the same name more than once.
///
/// #Params
/// name: Name of the wallet.
/// runtime_config (optional): Runtime wallet configuration json. if NULL, then default runtime_config will be used.
///   {
///       "storage": Optional<object>  List of supported keys are defined by wallet type.
///   }
/// credentials: Wallet credentials json
///   {
///       "key": string,
///       "rekey": Optional<string>,
///       "storage": Optional<object>  List of supported keys are defined by wallet type.
///
///   }
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
                               credentials_json: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32,
                                                    err: ErrorCode,
                                                    handle: i32)>) -> ErrorCode {
    trace!("indy_open_wallet: >>> name: {:?}, runtime_config: {:?}, credentials_json: {:?}", name, runtime_config, credentials_json);

    check_useful_c_str!(name, ErrorCode::CommonInvalidParam2);
    check_useful_opt_c_str!(runtime_config, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(credentials_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_open_wallet: entities >>> name: {:?}, runtime_config: {:?}, credentials_json: {:?}", name, runtime_config, credentials_json);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Open(
            name,
            runtime_config,
            credentials_json,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                trace!("indy_open_wallet: handle: {:?}", handle);
                cb(command_handle, err, handle)
            })
        )));

    let res = result_to_err_code!(result);

    trace!("indy_open_wallet: <<< res: {:?}", res);

    res
}

/// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
///
/// #Returns
/// wallets list json: [{
///    "name": <string>
///    "type": <string>
///    "pool_name": <string>
/// }].
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_list_wallets(command_handle: i32,
                                cb: Option<extern fn(xcommand_handle: i32,
                                                     err: ErrorCode,
                                                     wallets: *const c_char)>) -> ErrorCode {
    trace!("indy_list_wallets: >>>");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    trace!("indy_list_wallets: entities >>>");

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::ListWallets(
            Box::new(move |result| {
                let (err, wallets) = result_to_err_code_1!(result, String::new());
                trace!("indy_list_wallets: wallets: {:?}", wallets);
                let wallets = CStringUtils::string_to_cstring(wallets);
                cb(command_handle, err, wallets.as_ptr())
            })
        )));

    let res = result_to_err_code!(result);

    trace!("indy_list_wallets: <<< res: {:?}", res);

    res
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
                                cb: Option<extern fn(xcommand_handle: i32,
                                                     err: ErrorCode)>) -> ErrorCode {
    trace!("indy_close_wallet: >>> handle: {:?}", handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_close_wallet: entities >>> handle: {:?}", handle);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Close(
            handle,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                trace!("indy_close_wallet:");
                cb(command_handle, err)
            })
        )));

    let res = result_to_err_code!(result);

    trace!("indy_close_wallet: <<< res: {:?}", res);

    res
}

/// Deletes created wallet.
///
/// #Params
/// name: Name of the wallet to delete.
/// credentials: Wallet credentials json
///   {
///       "key": string,
///       "rekey": Optional<string>,
///       "storage": Optional<object>  List of supported keys are defined by wallet type.
///
///   }
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
                                 credentials_json: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: ErrorCode)>) -> ErrorCode {
    trace!("indy_delete_wallet: >>> name: {:?}, credentials_json: {:?}", name, credentials_json);

    check_useful_c_str!(name, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(credentials_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_delete_wallet: entities >>> name: {:?}, credentials_json: {:?}", name, credentials_json);

    let result = CommandExecutor::instance()
        .send(Command::Wallet(WalletCommand::Delete(
            name,
            credentials_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                trace!("indy_delete_wallet:");
                cb(command_handle, err)
            })
        )));

    let res = result_to_err_code!(result);

    trace!("indy_delete_wallet: <<< res: {:?}", res);

    res
}

/// Create the wallet storage (For example, database creation)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// metadata: wallet metadata (For example encrypted keys).
pub type WalletCreate = extern fn(name: *const c_char,
                                  config: *const c_char,
                                  credentials_json: *const c_char,
                                  metadata: *const c_char) -> ErrorCode;

/// Open the wallet storage (For example, opening database connection)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// runtime_config: wallet storage runtime config (For example, connection config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// storage_handle_p: pointer to store opened storage handle
pub type WalletOpen = extern fn(name: *const c_char,
                                config: *const c_char,
                                runtime_config: *const c_char,
                                credentials_json: *const c_char,
                                storage_handle_p: *mut i32) -> ErrorCode;

/// Close the opened walled storage (For example, closing database connection)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
pub type WalletClose = extern fn(storage_handle: i32) -> ErrorCode;

/// Delete the wallet storage (For example, database deletion)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
pub type WalletDelete = extern fn(name: *const c_char,
                                  config: *const c_char,
                                  credentials_json: *const c_char) -> ErrorCode;

/// Create a new record in the wallet storage
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record (pointer to buffer)
/// value_len: the value of record (buffer size)
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value
///   }
///   Note that null means no tags
pub type WalletAddRecord = extern fn(storage_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     value: *const u8,
                                     value_len: usize,
                                     tags_json: *const c_char) -> ErrorCode;

/// Update a record value
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record (pointer to buffer)
/// value_len: the value of record (buffer size)
pub type WalletUpdateRecordValue = extern fn(storage_handle: i32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             value: *const u8,
                                             value_len: usize, ) -> ErrorCode;

/// Update a record tags
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the new record tags used for search and storing meta information as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value
///   }
///   Note that null means no tags
pub type WalletUpdateRecordTags = extern fn(storage_handle: i32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tags_json: *const c_char) -> ErrorCode;

/// Add new tags to the record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the additional record tags as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value,
///     ...
///   }
///   Note that null means no tags
///   Note if some from provided tags already assigned to the record than
///     corresponding tags values will be replaced
pub type WalletAddRecordTags = extern fn(storage_handle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode;

/// Delete tags from the record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tag_names_json: the list of tag names to remove from the record as json array:
///   ["tagName1", "tagName2", ...]
///   Note that null means no tag names
pub type WalletDeleteRecordTags = extern fn(storage_handle: i32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tag_names_json: *const c_char) -> ErrorCode;

/// Delete an existing record in the wallet storage
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: record type
/// id: the id of record
pub type WalletDeleteRecord = extern fn(storage_handle: i32,
                                        type_: *const c_char,
                                        id: *const c_char) -> ErrorCode;

/// Get an wallet storage record by id
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// options_json: //TODO: FIXME: Think about replacing by bitmaks
///  {
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags
///  }
/// record_handle_p: pointer to store retrieved record handle
pub type WalletGetRecord = extern fn(storage_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     options_json: *const c_char,
                                     record_handle_p: *mut i32) -> ErrorCode;

/// Get an id for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record id
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
pub type WalletGetRecordId = extern fn(storage_handle: i32,
                                       record_handle: i32,
                                       record_id_p: *mut *const c_char) -> ErrorCode;

/// Get an type for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record type
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
pub type WalletGetRecordType = extern fn(storage_handle: i32,
                                         record_handle: i32,
                                         record_type_p: *mut *const c_char) -> ErrorCode;

/// Get an value for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record value
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no value retrieved
pub type WalletGetRecordValue = extern fn(storage_handle: i32,
                                          record_handle: i32,
                                          record_value_p: *mut *const u8,
                                          record_value_len_p: *mut usize) -> ErrorCode;

/// Get an tags for retrieved wallet record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record tags as json
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no tags retrieved
pub type WalletGetRecordTags = extern fn(storage_handle: i32,
                                         record_handle: i32,
                                         record_tags_p: *mut *const c_char) -> ErrorCode;

/// Free retrieved wallet record (make retrieved record handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// record_handle: retrieved record handle (See wallet_storage_get_wallet_record)
pub type WalletFreeRecord = extern fn(storage_handle: i32,
                                      record_handle: i32) -> ErrorCode;

/// Get storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
///
/// returns: metadata as base64 value
///          Note that pointer lifetime is static
pub type WalletGetStorageMetadata = extern fn(storage_handle: i32,
                                              metadata_p: *mut *const c_char,
                                              metadata_handle: *mut i32) -> ErrorCode;

/// Set storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// metadata_p: base64 value of metadata
///
///   Note if storage already have metadata record it will be overwritten.
pub type WalletSetStorageMetadata = extern fn(storage_handle: i32,
                                              metadata_p: *const c_char) -> ErrorCode;

/// Free retrieved storage metadata record (make retrieved storage metadata handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// metadata_handle: retrieved record handle (See wallet_storage_get_storage_metadata)
pub type WalletFreeStorageMetadata = extern fn(storage_handle: i32,
                                               metadata_handle: i32) -> ErrorCode;

/// Search for wallet storage records
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// query_json: MongoDB style query to wallet record tags:
///  {
///    "tagName": "tagValue",
///    $or: {
///      "tagName2": { $regex: 'pattern' },
///      "tagName3": { $gte: 123 },
///    },
///  }
/// options_json: //TODO: FIXME: Think about replacing by bitmaks
///  {
///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
///    retrieveTotalCount: (optional, false by default) Calculate total count,
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags,
///  }
/// search_handle_p: pointer to store wallet search handle
pub type WalletSearchRecords = extern fn(storage_handle: i32,
                                         type_: *const c_char,
                                         query_json: *const c_char,
                                         options_json: *const c_char,
                                         search_handle_p: *mut i32) -> ErrorCode;

/// Search for all wallet storage records
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle_p: pointer to store wallet search handle
pub type WalletSearchAllRecords = extern fn(storage_handle: i32,
                                            search_handle_p: *mut i32) -> ErrorCode;

/// Get total count of records that corresponds to wallet storage search query
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: total count of records that corresponds to wallet storage search query
///          Note -1 will be returned if retrieveTotalCount set to false for search_records
pub type WalletGetSearchTotalCount = extern fn(storage_handle: i32,
                                               search_handle: i32,
                                               total_count_p: *mut usize) -> ErrorCode;

/// Get the next wallet storage record handle retrieved by this wallet search.
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: record handle (the same as for get_record handler)
///          Note if no more records WalletNoRecords error will be returned
pub type WalletFetchSearchNextRecord = extern fn(storage_handle: i32,
                                                 search_handle: i32,
                                                 record_handle_p: *mut i32) -> ErrorCode;

/// Free wallet search (make search handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
pub type WalletFreeSearch = extern fn(storage_handle: i32,
                                      search_handle: i32) -> ErrorCode;
