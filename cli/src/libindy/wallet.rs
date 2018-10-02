extern crate sharedlib;

use super::ErrorCode;
use self::sharedlib::{Lib, Func, Symbol};

use libc::c_char;
use std::ffi::CString;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;


pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_create_wallet(command_handle,
                               config.as_ptr(),
                               credentials.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_open_wallet(command_handle,
                             config.as_ptr(),
                             credentials.as_ptr(),
                             cb)
        };

        super::results::result_to_int(err, receiver)
    }

    pub fn delete_wallet(wallet_name: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let wallet_name = CString::new(wallet_name).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = unsafe {
            indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               credentials.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();


        let err = unsafe { indy_close_wallet(command_handle, wallet_handle, cb) };

        super::results::result_to_empty(err, receiver)
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let export_config_json = CString::new(export_config_json).unwrap();

        let err = unsafe {
            indy_export_wallet(command_handle,
                               wallet_handle,
                               export_config_json.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();
        let import_config_json = CString::new(import_config_json).unwrap();

        let err = unsafe {
            indy_import_wallet(command_handle,
                               config.as_ptr(),
                               credentials.as_ptr(),
                               import_config_json.as_ptr(),
                               cb)
        };

        super::results::result_to_empty(err, receiver)
    }

    pub fn register_wallet_storage(stg_type: &str, library_path: &str, fn_pfx: &str) -> Result<(), ErrorCode> {
        println!("Loading {} {} {}", stg_type, library_path, fn_pfx);
        lazy_static! {
                static ref STG_REGISERED_WALLETS: Mutex<HashMap<String, Lib>> = Default::default();
            }

        let mut wallets = STG_REGISERED_WALLETS.lock().unwrap();

        if wallets.contains_key(stg_type) {
            // as registering of plugged wallet with
            // already registered so just return
            return Ok(());
        }

        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let xxtype = CString::new(stg_type).unwrap();

        let err;
        let lib;
        let lib_path = Path::new(library_path);
        unsafe {
            println!("Loading {:?}", lib_path);
            lib = match Lib::new(lib_path) {
                Ok(rlib) => {
                    println!("Loaded lib");
                    rlib
                },
                Err(err) => {
                    println!("Load error {:?}", err);
                    panic!("Load error {:?}", err)
                }
            };

            println!("Get fn pointers ...");
            let fn_create_handler: Func<WalletCreate> = lib.find_func(&format!("{}create", fn_pfx)).unwrap();
            let fn_open_handler: Func<WalletOpen> = lib.find_func(&format!("{}open", fn_pfx)).unwrap();
            let fn_close_handler: Func<WalletClose> = lib.find_func(&format!("{}close", fn_pfx)).unwrap();
            let fn_delete_handler: Func<WalletDelete> = lib.find_func(&format!("{}delete", fn_pfx)).unwrap();
            let fn_add_record_handler: Func<WalletAddRecord> = lib.find_func(&format!("{}add_record", fn_pfx)).unwrap();
            let fn_update_record_value_handler: Func<WalletUpdateRecordValue> = lib.find_func(&format!("{}update_record_value", fn_pfx)).unwrap();
            let fn_update_record_tags_handler: Func<WalletUpdateRecordTags> = lib.find_func(&format!("{}update_record_tags", fn_pfx)).unwrap();
            let fn_add_record_tags_handler: Func<WalletAddRecordTags> = lib.find_func(&format!("{}add_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_tags_handler: Func<WalletDeleteRecordTags> = lib.find_func(&format!("{}delete_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_handler: Func<WalletDeleteRecord> = lib.find_func(&format!("{}delete_record", fn_pfx)).unwrap();
            let fn_get_record_handler: Func<WalletGetRecord> = lib.find_func(&format!("{}get_record", fn_pfx)).unwrap();
            let fn_get_record_id_handler: Func<WalletGetRecordId> = lib.find_func(&format!("{}get_record_id", fn_pfx)).unwrap();
            let fn_get_record_type_handler: Func<WalletGetRecordType> = lib.find_func(&format!("{}get_record_type", fn_pfx)).unwrap();
            let fn_get_record_value_handler: Func<WalletGetRecordValue> = lib.find_func(&format!("{}get_record_value", fn_pfx)).unwrap();
            let fn_get_record_tags_handler: Func<WalletGetRecordTags> = lib.find_func(&format!("{}get_record_tags", fn_pfx)).unwrap();
            let fn_free_record_handler: Func<WalletFreeRecord> = lib.find_func(&format!("{}free_record", fn_pfx)).unwrap();
            let fn_get_storage_metadata_handler: Func<WalletGetStorageMetadata> = lib.find_func(&format!("{}get_storage_metadata", fn_pfx)).unwrap();
            let fn_set_storage_metadata_handler: Func<WalletSetStorageMetadata> = lib.find_func(&format!("{}set_storage_metadata", fn_pfx)).unwrap();
            let fn_free_storage_metadata_handler: Func<WalletFreeStorageMetadata> = lib.find_func(&format!("{}free_storage_metadata", fn_pfx)).unwrap();
            let fn_search_records_handler: Func<WalletSearchRecords> = lib.find_func(&format!("{}search_records", fn_pfx)).unwrap();
            let fn_search_all_records_handler: Func<WalletSearchAllRecords> = lib.find_func(&format!("{}search_all_records", fn_pfx)).unwrap();
            let fn_get_search_total_count_handler: Func<WalletGetSearchTotalCount> = lib.find_func(&format!("{}get_search_total_count", fn_pfx)).unwrap();
            let fn_fetch_search_next_record_handler: Func<WalletFetchSearchNextRecord> = lib.find_func(&format!("{}fetch_search_next_record", fn_pfx)).unwrap();
            let fn_free_search_handler: Func<WalletFreeSearch> = lib.find_func(&format!("{}free_search", fn_pfx)).unwrap();

            println!("Register wallet ...");
            err = indy_register_wallet_storage(
                command_handle,
                xxtype.as_ptr(),
                Some(fn_create_handler.get()),
                Some(fn_open_handler.get()),
                Some(fn_close_handler.get()),
                Some(fn_delete_handler.get()),
                Some(fn_add_record_handler.get()),
                Some(fn_update_record_value_handler.get()),
                Some(fn_update_record_tags_handler.get()),
                Some(fn_add_record_tags_handler.get()),
                Some(fn_delete_record_tags_handler.get()),
                Some(fn_delete_record_handler.get()),
                Some(fn_get_record_handler.get()),
                Some(fn_get_record_id_handler.get()),
                Some(fn_get_record_type_handler.get()),
                Some(fn_get_record_value_handler.get()),
                Some(fn_get_record_tags_handler.get()),
                Some(fn_free_record_handler.get()),
                Some(fn_get_storage_metadata_handler.get()),
                Some(fn_set_storage_metadata_handler.get()),
                Some(fn_free_storage_metadata_handler.get()),
                Some(fn_search_records_handler.get()),
                Some(fn_search_all_records_handler.get()),
                Some(fn_get_search_total_count_handler.get()),
                Some(fn_fetch_search_next_record_handler.get()),
                Some(fn_free_search_handler.get()),
                cb
            );
        }

        println!("Finish up ...");
        wallets.insert(stg_type.to_string(), lib);

        super::results::result_to_empty(err, receiver)
    }
}

extern {
    #[no_mangle]
    fn indy_create_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_open_wallet(command_handle: i32,
                        config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>) -> ErrorCode;

    #[no_mangle]
    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_delete_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_export_wallet(command_handle: i32,
                          wallet_handle: i32,
                          export_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_import_wallet(command_handle: i32,
                          config: *const c_char,
                          credentials: *const c_char,
                          import_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_register_wallet_storage(command_handle: i32,
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
                                            err: ErrorCode)>) -> ErrorCode;
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
/// credentials_json: wallet storage credentials (For example, database credentials)
/// storage_handle_p: pointer to store opened storage handle
pub type WalletOpen = extern fn(name: *const c_char,
                                config: *const c_char,
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
/// options_json: //TODO: FIXME: Think about replacing by bitmask
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
/// options_json: //TODO: FIXME: Think about replacing by bitmask
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

