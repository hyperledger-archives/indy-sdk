
use libindy::ErrorCode;

use libc::c_char;
use utils::callbacks;
use std::sync::mpsc::channel;


pub type IndyHandle = i32;


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
/// credentials_json: wallet storage credentials (For example, database credentials)
/// storage_handle_p: pointer to store opened storage handle
pub type WalletOpen = extern fn(name: *const c_char,
                                config: *const c_char,
                                credentials_json: *const c_char,
                                storage_handle_p: *mut IndyHandle) -> ErrorCode;

/// Close the opened walled storage (For example, closing database connection)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
pub type WalletClose = extern fn(storage_handle: IndyHandle) -> ErrorCode;

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
pub type WalletAddRecord = extern fn(storage_handle: IndyHandle,
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
pub type WalletUpdateRecordValue = extern fn(storage_handle: IndyHandle,
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
pub type WalletUpdateRecordTags = extern fn(storage_handle: IndyHandle,
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
pub type WalletAddRecordTags = extern fn(storage_handle: IndyHandle,
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
pub type WalletDeleteRecordTags = extern fn(storage_handle: IndyHandle,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tag_names_json: *const c_char) -> ErrorCode;

/// Delete an existing record in the wallet storage
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: record type
/// id: the id of record
pub type WalletDeleteRecord = extern fn(storage_handle: IndyHandle,
                                        type_: *const c_char,
                                        id: *const c_char) -> ErrorCode;

/// Get an wallet storage record by id
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags
///  }
/// record_handle_p: pointer to store retrieved record handle
pub type WalletGetRecord = extern fn(storage_handle: IndyHandle,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     options_json: *const c_char,
                                     record_handle_p: *mut IndyHandle) -> ErrorCode;

/// Get an id for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record id
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
pub type WalletGetRecordId = extern fn(storage_handle: IndyHandle,
                                       record_handle: IndyHandle,
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
pub type WalletGetRecordType = extern fn(storage_handle: IndyHandle,
                                         record_handle: IndyHandle,
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
pub type WalletGetRecordValue = extern fn(storage_handle: IndyHandle,
                                          record_handle: IndyHandle,
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
pub type WalletGetRecordTags = extern fn(storage_handle: IndyHandle,
                                         record_handle: IndyHandle,
                                         record_tags_p: *mut *const c_char) -> ErrorCode;

/// Free retrieved wallet record (make retrieved record handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// record_handle: retrieved record handle (See wallet_storage_get_wallet_record)
pub type WalletFreeRecord = extern fn(storage_handle: IndyHandle,
                                      record_handle: IndyHandle) -> ErrorCode;

/// Get storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
///
/// returns: metadata as base64 value
///          Note that pointer lifetime is static
pub type WalletGetStorageMetadata = extern fn(storage_handle: IndyHandle,
                                              metadata_p: *mut *const c_char,
                                              metadata_handle: *mut IndyHandle) -> ErrorCode;

/// Set storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// metadata_p: base64 value of metadata
///
///   Note if storage already have metadata record it will be overwritten.
pub type WalletSetStorageMetadata = extern fn(storage_handle: IndyHandle,
                                              metadata_p: *const c_char) -> ErrorCode;

/// Free retrieved storage metadata record (make retrieved storage metadata handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// metadata_handle: retrieved record handle (See wallet_storage_get_storage_metadata)
pub type WalletFreeStorageMetadata = extern fn(storage_handle: IndyHandle,
                                               metadata_handle: IndyHandle) -> ErrorCode;

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
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
///    retrieveTotalCount: (optional, false by default) Calculate total count,
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags,
///  }
/// search_handle_p: pointer to store wallet search handle
pub type WalletSearchRecords = extern fn(storage_handle: IndyHandle,
                                         type_: *const c_char,
                                         query_json: *const c_char,
                                         options_json: *const c_char,
                                         search_handle_p: *mut IndyHandle) -> ErrorCode;

/// Search for all wallet storage records
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle_p: pointer to store wallet search handle
pub type WalletSearchAllRecords = extern fn(storage_handle: IndyHandle,
                                            search_handle_p: *mut IndyHandle) -> ErrorCode;

/// Get total count of records that corresponds to wallet storage search query
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: total count of records that corresponds to wallet storage search query
///          Note -1 will be returned if retrieveTotalCount set to false for search_records
pub type WalletGetSearchTotalCount = extern fn(storage_handle: IndyHandle,
                                               search_handle: IndyHandle,
                                               total_count_p: *mut usize) -> ErrorCode;

/// Get the next wallet storage record handle retrieved by this wallet search.
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: record handle (the same as for get_record handler)
///          Note if no more records WalletNoRecords error will be returned
pub type WalletFetchSearchNextRecord = extern fn(storage_handle: IndyHandle,
                                                 search_handle: IndyHandle,
                                                 record_handle_p: *mut IndyHandle) -> ErrorCode;

/// Free wallet search (make search handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
pub type WalletFreeSearch = extern fn(storage_handle: IndyHandle,
                                      search_handle: IndyHandle) -> ErrorCode;

pub fn register_wallet_storage(
    wallet_storage_name: *const c_char,
    create: WalletCreate,
    open: WalletOpen,
    close: WalletClose,
    delete: WalletDelete,
    add_record: WalletAddRecord,
    update_record_value: WalletUpdateRecordValue,
    update_record_tags: WalletUpdateRecordTags,
    add_record_tags: WalletAddRecordTags,
    delete_record_tags: WalletDeleteRecordTags,
    delete_record: WalletDeleteRecord,
    get_record: WalletGetRecord,
    get_record_id: WalletGetRecordId,
    get_record_type: WalletGetRecordType,
    get_record_value: WalletGetRecordValue,
    get_record_tags: WalletGetRecordTags,
    free_record: WalletFreeRecord,
    get_storage_metadata: WalletGetStorageMetadata,
    set_storage_metadata: WalletSetStorageMetadata,
    free_storage_metadata: WalletFreeStorageMetadata,
    search_records: WalletSearchRecords,
    search_all_records: WalletSearchAllRecords,
    get_search_total_count: WalletGetSearchTotalCount,
    fetch_search_next_record: WalletFetchSearchNextRecord,
    free_search: WalletFreeSearch,
) -> ErrorCode {
    let (sender, receiver) = channel();

    let closure: Box<dyn FnMut(ErrorCode) + Send> = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (cmd_handle, cb) = callbacks::closure_to_cb_ec(closure);

    unsafe {
        indy_register_wallet_storage(
            cmd_handle,
            wallet_storage_name,
            Some(create),
            Some(open),
            Some(close),
            Some(delete),
            Some(add_record),
            Some(update_record_value),
            Some(update_record_tags),
            Some(add_record_tags),
            Some(delete_record_tags),
            Some(delete_record),
            Some(get_record),
            Some(get_record_id),
            Some(get_record_type),
            Some(get_record_value),
            Some(get_record_tags),
            Some(free_record),
            Some(get_storage_metadata),
            Some(set_storage_metadata),
            Some(free_storage_metadata),
            Some(search_records),
            Some(search_all_records),
            Some(get_search_total_count),
            Some(fetch_search_next_record),
            Some(free_search),
            cb,
        );
    }

    receiver.recv().unwrap()
}

extern {
    #[no_mangle]
    pub fn indy_register_wallet_storage(command_handle: IndyHandle,
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
                                            cb: Option<extern fn(command_handle_: IndyHandle,
                                                                    err: ErrorCode)>) -> ErrorCode;
}


