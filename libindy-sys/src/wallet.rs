use super::*;

use {Error, Handle, BString, CString};

/// Create the wallet storage (For example, database creation)
///
/// # Arguments
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// metadata: wallet metadata (For example encrypted keys).
pub type WalletCreateCB = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString,
                                  metadata: CString) -> Error;

/// Open the wallet storage (For example, opening database connection)
///
/// # Arguments
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// runtime_config: wallet storage runtime config (For example, connection config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// storage_handle_p: pointer to store opened storage handle
pub type WalletOpenCB = extern fn(name: CString,
                                config: CString,
                                runtime_config: CString,
                                credentials_json: CString,
                                storage_handle_p: *mut Handle) -> Error;

/// Close the opened walled storage (For example, closing database connection)
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
pub type WalletCloseCB = extern fn(storage_handle: Handle) -> Error;

/// Delete the wallet storage (For example, database deletion)
///
/// # Arguments
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
pub type WalletDeleteCB = extern fn(name: CString,
                                  config: CString,
                                  credentials_json: CString) -> Error;

/// Create a new record in the wallet storage
///
/// # Arguments
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
pub type WalletAddRecordCB = extern fn(storage_handle: Handle,
                                     type_: CString,
                                     id: CString,
                                     value: BString,
                                     value_len: usize,
                                     tags_json: CString) -> Error;

/// Update a record value
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record (pointer to buffer)
/// value_len: the value of record (buffer size)
pub type WalletUpdateRecordValueCB = extern fn(storage_handle: Handle,
                                             type_: CString,
                                             id: CString,
                                             value: BString,
                                             value_len: usize) -> Error;

/// Update a record tags
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the new record tags used for search and storing meta information as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value
///   }
///   Note that null means no tags
pub type WalletUpdateRecordTagsCB = extern fn(storage_handle: Handle,
                                            type_: CString,
                                            id: CString,
                                            tags_json: CString) -> Error;

/// Add new tags to the record
///
/// # Arguments
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
pub type WalletAddRecordTagsCB = extern fn(storage_handle: Handle,
                                         type_: CString,
                                         id: CString,
                                         tags_json: CString) -> Error;

/// Delete tags from the record
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tag_names_json: the list of tag names to remove from the record as json array:
///   ["tagName1", "tagName2", ...]
///   Note that null means no tag names
pub type WalletDeleteRecordTagsCB = extern fn(storage_handle: Handle,
                                            type_: CString,
                                            id: CString,
                                            tag_names_json: CString) -> Error;

/// Delete an existing record in the wallet storage
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// type_: record type
/// id: the id of record
pub type WalletDeleteRecordCB = extern fn(storage_handle: Handle,
                                        type_: CString,
                                        id: CString) -> Error;

/// Get an wallet storage record by id
///
/// # Arguments
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
pub type WalletGetRecordCB = extern fn(storage_handle: Handle,
                                     type_: CString,
                                     id: CString,
                                     options_json: CString,
                                     record_handle_p: *mut Handle) -> Error;

/// Get an id for retrieved wallet storage record
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record id
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
pub type WalletGetRecordIdCB = extern fn(storage_handle: Handle,
                                       record_handle: Handle,
                                       record_id_p: *mut CString) -> Error;

/// Get an type for retrieved wallet storage record
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record type
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
pub type WalletGetRecordTypeCB = extern fn(storage_handle: Handle,
                                         record_handle: Handle,
                                         record_type_p: *mut CString) -> Error;

/// Get an value for retrieved wallet storage record
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record value
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no value retrieved
pub type WalletGetRecordValueCB = extern fn(storage_handle: Handle,
                                          record_handle: Handle,
                                          record_value_p: *mut BString,
                                          record_value_len_p: *mut usize) -> Error;

/// Get an tags for retrieved wallet record
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record tags as json
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no tags retrieved
pub type WalletGetRecordTagsCB = extern fn(storage_handle: Handle,
                                         record_handle: Handle,
                                         record_tags_p: *mut CString) -> Error;

/// Free retrieved wallet record (make retrieved record handle invalid)
///
/// # Arguments
/// storage_handle: opened storage handle (See open_wallet_storage)
/// record_handle: retrieved record handle (See wallet_storage_get_wallet_record)
pub type WalletFreeRecordCB = extern fn(storage_handle: Handle,
                                      record_handle: Handle) -> Error;

/// Get storage metadata
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
///
/// returns: metadata as base64 value
///          Note that pointer lifetime is static
pub type WalletGetStorageMetadataCB = extern fn(storage_handle: Handle,
                                              metadata_p: *mut CString,
                                              metadata_handle: *mut Handle) -> Error;

/// Set storage metadata
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// metadata_p: base64 value of metadata
///
///   Note if storage already have metadata record it will be overwritten.
pub type WalletSetStorageMetadataCB = extern fn(storage_handle: Handle,
                                              metadata_p: CString) -> Error;

/// Free retrieved storage metadata record (make retrieved storage metadata handle invalid)
///
/// # Arguments
/// storage_handle: opened storage handle (See open_wallet_storage)
/// metadata_handle: retrieved record handle (See wallet_storage_get_storage_metadata)
pub type WalletFreeStorageMetadataCB = extern fn(storage_handle: Handle,
                                               metadata_handle: Handle) -> Error;

/// Search for wallet storage records
///
/// # Arguments
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
pub type WalletSearchRecordsCB = extern fn(storage_handle: Handle,
                                         type_: CString,
                                         query_json: CString,
                                         options_json: CString,
                                         search_handle_p: *mut Handle) -> Error;

/// Search for all wallet storage records
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// search_handle_p: pointer to store wallet search handle
pub type WalletSearchAllRecordsCB = extern fn(storage_handle: Handle,
                                            search_handle_p: *mut Handle) -> Error;

/// Get total count of records that corresponds to wallet storage search query
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: total count of records that corresponds to wallet storage search query
///          Note -1 will be returned if retrieveTotalCount set to false for search_records
pub type WalletGetSearchTotalCountCB = extern fn(storage_handle: Handle,
                                               search_handle: Handle,
                                               total_count_p: *mut usize) -> Error;

/// Get the next wallet storage record handle retrieved by this wallet search.
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: record handle (the same as for get_record handler)
///          Note if no more records WalletNoRecords error will be returned
pub type WalletFetchSearchNextRecordCB = extern fn(storage_handle: Handle,
                                                 search_handle: Handle,
                                                 record_handle_p: *mut Handle) -> Error;

/// Free wallet search (make search handle invalid)
///
/// # Arguments
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
pub type WalletFreeSearchCB = extern fn(storage_handle: Handle,
                                      search_handle: Handle) -> Error;

extern {
    #[no_mangle]
    pub fn indy_register_wallet_storage(command_handle: Handle,
                                        xtype: CString,
                                        create: Option<WalletCreateCB>,
                                        open: Option<WalletOpenCB>,
                                        close: Option<WalletCloseCB>,
                                        delete: Option<WalletDeleteCB>,
                                        add_record: Option<WalletAddRecordCB>,
                                        update_record_value: Option<WalletUpdateRecordValueCB>,
                                        update_record_tags: Option<WalletUpdateRecordTagsCB>,
                                        add_record_tags: Option<WalletAddRecordTagsCB>,
                                        delete_record_tags: Option<WalletDeleteRecordTagsCB>,
                                        delete_record: Option<WalletDeleteRecordCB>,
                                        get_record: Option<WalletGetRecordCB>,
                                        get_record_id: Option<WalletGetRecordIdCB>,
                                        get_record_type: Option<WalletGetRecordTypeCB>,
                                        get_record_value: Option<WalletGetRecordValueCB>,
                                        get_record_tags: Option<WalletGetRecordTagsCB>,
                                        free_record: Option<WalletFreeRecordCB>,
                                        get_storage_metadata: Option<WalletGetStorageMetadataCB>,
                                        set_storage_metadata: Option<WalletSetStorageMetadataCB>,
                                        free_storage_metadata: Option<WalletFreeStorageMetadataCB>,
                                        search_records: Option<WalletSearchRecordsCB>,
                                        search_all_records: Option<WalletSearchAllRecordsCB>,
                                        get_search_total_count: Option<WalletGetSearchTotalCountCB>,
                                        fetch_search_next_record: Option<WalletFetchSearchNextRecordCB>,
                                        free_search: Option<WalletFreeSearchCB>,
                                        cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_create_wallet(command_handle: Handle,
                              pool_name: CString,
                              name: CString,
                              storage_type: CString,
                              config: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_open_wallet(command_handle: Handle,
                            name: CString,
                            runtime_config: CString,
                            credentials: CString,
                            cb: Option<ResponseI32CB>) -> Error;

    #[no_mangle]
    pub fn indy_list_wallets(command_handle: Handle,
                             cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_close_wallet(command_handle: Handle,
                             handle: Handle,
                             cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_delete_wallet(command_handle: Handle,
                              name: CString,
                              credentials: CString,
                              cb: Option<ResponseEmptyCB>) -> Error;
}
