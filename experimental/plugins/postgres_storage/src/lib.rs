#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

#[macro_use]
extern crate log;

extern crate serde;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// Note that to use macroses from indy_common::util inside of other modules it must me loaded first!
extern crate indy_crypto;
extern crate libc;
extern crate time;
extern crate rand;
extern crate postgres;

pub mod libindy;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
pub mod utils;
pub mod errors;
pub mod postgres_storage;
pub mod wql;

use libindy::ErrorCode;
use utils::sequence::SequenceUtils;
use utils::crypto::base64 as util_base64;
use utils::ctypes;
use wql::storage::{WalletStorage, StorageRecord, StorageIterator, Tag, TagName, EncryptedValue};
use wql::language;
use errors::wallet::WalletStorageError;
use postgres_storage::WalletStorageType;

use self::libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;
use std::str;

pub static POSTGRES_STORAGE_NAME: &str = "postgres_storage";


#[no_mangle]
pub extern fn postgresstorage_init() -> libindy::ErrorCode {
    //if let Err(err) = utils::logger::LibnullpayLogger::init() {
    //    return err;
    //}

    let postgres_storage_name = CString::new(POSTGRES_STORAGE_NAME).unwrap();

    libindy::wallet::register_wallet_storage(
        postgres_storage_name.as_ptr(),
        PostgresWallet::create,
        PostgresWallet::open,
        PostgresWallet::close,
        PostgresWallet::delete,
        PostgresWallet::add_record,
        PostgresWallet::update_record_value,
        PostgresWallet::update_record_tags,
        PostgresWallet::add_record_tags,
        PostgresWallet::delete_record_tags,
        PostgresWallet::delete_record,
        PostgresWallet::get_record,
        PostgresWallet::get_record_id,
        PostgresWallet::get_record_type,
        PostgresWallet::get_record_value,
        PostgresWallet::get_record_tags,
        PostgresWallet::free_record,
        PostgresWallet::get_storage_metadata,
        PostgresWallet::set_storage_metadata,
        PostgresWallet::free_storage_metadata,
        PostgresWallet::search_records,
        PostgresWallet::search_all_records,
        PostgresWallet::get_search_total_count,
        PostgresWallet::fetch_search_next_record,
        PostgresWallet::free_search,
    )
}

struct PostgresStorageContext {
    // TODO save handle, config and credentials in case we need to re-connect to database
    _xhandle: i32,        // reference returned to client to track open wallet connection
    id: String,          // wallet name
    _config: String,      // wallet config
    _credentials: String, // wallet credentials
    phandle: Box<::postgres_storage::PostgresStorage>  // reference to a postgres database connection
}

#[derive(Debug, Clone)]
struct PostgresWalletRecord {
    id: CString,
    type_: CString,
    value: Vec<u8>,
    tags: CString
}

#[derive(Debug, Clone)]
struct PostgresWalletRecordSet {
    idx: usize,
    records: Vec<PostgresWalletRecord>,
    count: usize
}

lazy_static! {
    // store a PostgresStorage object (contains a connection) 
    static ref POSTGRES_OPEN_WALLETS: Mutex<HashMap<i32, PostgresStorageContext>> = Default::default();
}

lazy_static! {
    // metadata for active wallets
    static ref POSTGRES_ACTIVE_METADATAS: Mutex<HashMap<i32, CString>> = Default::default();
}

lazy_static! {
    // cache of Postgres fetched records
    static ref POSTGRES_ACTIVE_RECORDS: Mutex<HashMap<i32, PostgresWalletRecord>> = Default::default();
}

lazy_static! {
    // cache of active Postgres searches
    // TODO figure out  athread-safe PostgresStorageIterator
    // static ref POSTGRES_ACTIVE_SEARCHES: Mutex<HashMap<i32, Box<::postgres_storage::PostgresStorageIterator>>> = Default::default();
    static ref POSTGRES_ACTIVE_SEARCHES: Mutex<HashMap<i32, PostgresWalletRecordSet>> = Default::default();
}

pub struct PostgresWallet {}

impl PostgresWallet {

    pub extern fn create(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char,
                             metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(config, ErrorCode::CommonInvalidState);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidState);
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidState);

        // create Postgres database, create schema, and insert metadata
        let storage_type = ::postgres_storage::PostgresStorageType::new();
        let res = storage_type.create_storage(&id, Some(&config), Some(&credentials), &metadata.as_bytes()[..]);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::AlreadyExists => ErrorCode::WalletAlreadyExistsError,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn open(id: *const c_char,
                           config: *const c_char,
                           credentials: *const c_char,
                           handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(config, ErrorCode::CommonInvalidState);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidState);

        // open wallet and return handle
        // PostgresStorageType::open_storage(), returns a PostgresStorage that goes into the handle
        let mut handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        // check if we have opened this wallet already
        for (_key, value) in &*handles {
            if value.id == id {
                return ErrorCode::WalletAlreadyOpenedError;
            }
        }

        // open the wallet
        let storage_type = ::postgres_storage::PostgresStorageType::new();
        let phandle = match storage_type.open_storage(&id, Some(&config), Some(&credentials))  {
            Ok(phandle) => phandle,
            Err(_err) => {
                return ErrorCode::WalletNotFoundError;
            }
        };

        // get a handle (to use to identify wallet for subsequent calls)
        let xhandle = SequenceUtils::get_next_id();

        // create a storage context (keep all info in case we need to recycle wallet connection)
        let context = PostgresStorageContext {
            _xhandle: xhandle,      // reference returned to client to track open wallet connection
            id,           // wallet name
            _config: config,       // wallet config
            _credentials: credentials,  // wallet credentials
            phandle       // reference to a postgres database connection
        };

        // add to our open wallet list
        handles.insert(xhandle, context);

        // return handle = index into our collection of open wallets
        unsafe { *handle = xhandle };
        ErrorCode::Success
    }


    pub extern fn add_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 value: *const u8,
                                 value_len: usize,
                                 tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(value, value_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let value = EncryptedValue::from_bytes(&value).unwrap();
        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.add(&type_.as_bytes(), &id.as_bytes(), &value, &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn update_record_value(xhandle: i32,
                                          type_: *const c_char,
                                          id: *const c_char,
                                          joined_value: *const u8,
                                          joined_value_len: usize) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(joined_value, joined_value_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let value = EncryptedValue::from_bytes(&joined_value).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.update(&type_.as_bytes(), &id.as_bytes(), &value);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn get_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 options_json: *const c_char,
                                 handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.get(&type_.as_bytes(), &id.as_bytes(), &options_json);

        match res {
            Ok(record) => {
                let record_handle = SequenceUtils::get_next_id();
                let p_rec = _storagerecord_to_postgresrecord(&record).unwrap();

                let mut handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();
                handles.insert(record_handle, p_rec);

                unsafe { *handle = record_handle };
                ErrorCode::Success
            },
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn get_record_id(xhandle: i32,
                                    record_handle: i32,
                                    id_ptr: *mut *const c_char) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *id_ptr = record.id.as_ptr() as *const i8; }

        ErrorCode::Success
    }


    pub extern fn get_record_type(xhandle: i32,
                                      record_handle: i32,
                                      type_ptr: *mut *const c_char) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *type_ptr = record.type_.as_ptr() as *const i8; }

        ErrorCode::Success
    }


    pub extern fn get_record_value(xhandle: i32,
                                       record_handle: i32,
                                       value_ptr: *mut *const u8,
                                       value_len: *mut usize) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *value_ptr = record.value.as_ptr() as *const u8; }
        unsafe { *value_len = record.value.len() as usize; }

        ErrorCode::Success
    }


    pub extern fn get_record_tags(xhandle: i32,
                                      record_handle: i32,
                                      tags_json_ptr: *mut *const c_char) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *tags_json_ptr = record.tags.as_ptr() as *const i8; }

        ErrorCode::Success
    }



    pub extern fn free_record(xhandle: i32, record_handle: i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&record_handle);

        ErrorCode::Success
    }


    pub extern fn add_record_tags(xhandle: i32,
                                      type_: *const c_char,
                                      id: *const c_char,
                                      tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.add_tags(&type_.as_bytes(), &id.as_bytes(), &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn update_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.update_tags(&type_.as_bytes(), &id.as_bytes(), &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn delete_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tag_names_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(tag_names_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        // convert to [TagName]
        let tag_names = _tag_names_from_json(&tag_names_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.delete_tags(&type_.as_bytes(), &id.as_bytes(), &tag_names);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn delete_record(xhandle: i32,
                                    type_: *const c_char,
                                    id: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.delete(&type_.as_bytes(), &id.as_bytes());

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemNotFound => ErrorCode::WalletItemNotFound,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn get_storage_metadata(xhandle: i32, metadata_ptr: *mut *const c_char, metadata_handle: *mut i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.get_storage_metadata();

        match res {
            Ok(metadata) => {
                let metadata = CString::new(metadata.clone()).unwrap();
                let metadata_pointer = metadata.as_ptr();

                let handle = SequenceUtils::get_next_id();

                let mut metadatas = POSTGRES_ACTIVE_METADATAS.lock().unwrap();
                metadatas.insert(handle, metadata);

                unsafe { *metadata_ptr = metadata_pointer; }
                unsafe { *metadata_handle = handle };

                ErrorCode::Success
            },
            Err(_err) => {
                ErrorCode::CommonInvalidState
            }
        }
    }


    pub extern fn set_storage_metadata(xhandle: i32, metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.set_storage_metadata(&metadata.as_bytes());

        match res {
            Ok(_) => ErrorCode::Success,
            Err(err) => {
                match err {
                    WalletStorageError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
                    _ => ErrorCode::WalletStorageError
                }
            }
        }
    }


    pub extern fn free_storage_metadata(xhandle: i32, metadata_handler: i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = POSTGRES_ACTIVE_METADATAS.lock().unwrap();

        if !handles.contains_key(&metadata_handler) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&metadata_handler);

        ErrorCode::Success
    }


    pub extern fn search_records(xhandle: i32, type_: *const c_char, query_json: *const c_char, options_json: *const c_char, handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidState);
        check_useful_c_str!(query_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidState);

        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let query = language::parse_from_json_encrypted(&query_json).unwrap();
        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.search(&type_.as_bytes(), &query, Some(&options_json));
        
        match res {
            Ok(iter) => {
                // iter: Box<StorageIterator>
                let total_count = iter.get_total_count();
                let search_records = _iterator_to_record_set(iter).unwrap();
                let total_count = match total_count {
                    Ok(count) => {
                        match count {
                            Some(ct) => ct,
                            None => 0
                        }
                    },
                    _ => search_records.len()
                };
                let search_set = PostgresWalletRecordSet {
                    idx: 0,
                    records: search_records,
                    count: total_count
                };

                let search_handle = SequenceUtils::get_next_id();

                let mut searches = POSTGRES_ACTIVE_SEARCHES.lock().unwrap();

                // TODO store the iterator rather than the fetched records
                // searches.insert(search_handle, iter);
                searches.insert(search_handle, search_set);

                unsafe { *handle = search_handle };
                return ErrorCode::Success
            },
            Err(_err) => {
                // err: WalletStorageError
                return ErrorCode::WalletStorageError
            }
        }
    }


    pub extern fn search_all_records(xhandle: i32, handle: *mut i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.get_all();
        
        match res {
            Ok(iter) => {
                // iter: Box<StorageIterator>
                let total_count = iter.get_total_count();
                let search_records = _iterator_to_record_set(iter).unwrap();
                let total_count = match total_count {
                    Ok(count) => {
                        match count {
                            Some(ct) => ct,
                            None => 0
                        }
                    },
                    _ => search_records.len()
                };
                let search_set = PostgresWalletRecordSet {
                    idx: 0,
                    records: search_records,
                    count: total_count
                };

                let search_handle = SequenceUtils::get_next_id();

                let mut searches = POSTGRES_ACTIVE_SEARCHES.lock().unwrap();

                // TODO store the iterator rather than the fetched records
                // searches.insert(search_handle, iter);
                searches.insert(search_handle, search_set);

                unsafe { *handle = search_handle };
                return ErrorCode::Success
            },
            Err(_err) => {
                // err: WalletStorageError
                return ErrorCode::WalletStorageError
            }
        }
    }


    pub extern fn get_search_total_count(xhandle: i32, search_handle: i32, count: *mut usize) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let searches = POSTGRES_ACTIVE_SEARCHES.lock().unwrap();

        match searches.get(&search_handle) {
            Some(records) => {
                unsafe { *count = records.count };
            }
            None => return ErrorCode::CommonInvalidState
        }

        ErrorCode::Success
    }


    pub extern fn fetch_search_next_record(xhandle: i32, search_handle: i32, record_handle: *mut i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut searches = POSTGRES_ACTIVE_SEARCHES.lock().unwrap();

        match searches.get_mut(&search_handle) {
            Some(records) => {
                if records.idx < records.records.len() {
                    let handle = SequenceUtils::get_next_id();

                    let mut handles = POSTGRES_ACTIVE_RECORDS.lock().unwrap();
                    handles.insert(handle, records.records.get(records.idx).unwrap().clone());
                    records.idx = records.idx + 1;

                    unsafe { *record_handle = handle };
                    ErrorCode::Success
                } else {
                    ErrorCode::WalletItemNotFound
                }
            },
            None => ErrorCode::CommonInvalidState
        }
    }


    pub extern fn free_search(xhandle: i32, search_handle: i32) -> ErrorCode {
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = POSTGRES_ACTIVE_SEARCHES.lock().unwrap();

        if !handles.contains_key(&search_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&search_handle);

        ErrorCode::Success
    }


    pub extern fn close(xhandle: i32) -> ErrorCode {
        let mut handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.remove(&xhandle).unwrap();

        let mut storage = *wallet_context.phandle;

        let res = storage.close();

        match res {
            Ok(_) => ErrorCode::Success,
            Err(_err) => ErrorCode::WalletStorageError
        }
    }


    pub extern fn delete(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidState);
        check_useful_c_str!(config, ErrorCode::CommonInvalidState);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidState);

        let storage_type = ::postgres_storage::PostgresStorageType::new();
        match storage_type.delete_storage(&id, Some(&config), Some(&credentials)) {
            Ok(_) => ErrorCode::Success,
            Err(_err) => ErrorCode::WalletStorageError
        }
    }
}

fn _storagerecord_to_postgresrecord(in_rec: &StorageRecord) -> Result<PostgresWalletRecord, WalletStorageError> {
    let out_id = CString::new(in_rec.id.clone()).unwrap();
    let out_type = match in_rec.type_ {
        Some(ref val) => CString::new(val.clone()).unwrap(),
        None => CString::new("").unwrap()
    };
    let out_val = match in_rec.value {
        Some(ref val) => val.to_bytes(),
        None => Vec::<u8>::new()
    };
    let out_tags = match in_rec.tags {
        Some(ref val) => CString::new(_tags_to_json(&val).unwrap()).unwrap(),
        None => CString::new("").unwrap()
    };
    let out_rec = PostgresWalletRecord {
        id: out_id,
        type_: out_type,
        value: out_val,
        tags: out_tags
    };
    Ok(out_rec)
}

fn _iterator_to_record_set(mut iter: Box<StorageIterator>) -> Result<Vec<PostgresWalletRecord>, ErrorCode> {
    let mut search_continue: bool = true;
    let mut search_records = Vec::new();
    while search_continue {
        let rec = iter.next();
        match rec {
            Ok(record) => {
                match record {
                    Some(record) => {
                        // record: StorageRecord
                        search_records.push(_storagerecord_to_postgresrecord(&record).unwrap());
                    },
                    None => {
                        search_continue = false;
                    }
                };
            },
            Err(_err) => {
                return Err(ErrorCode::WalletStorageError);
            }
        };
    }
    Ok(search_records)
}

fn _tags_to_json(tags: &[Tag]) -> Result<String, WalletStorageError> {
    let mut string_tags = HashMap::new();
    for tag in tags {
        match tag {
            &Tag::Encrypted(ref name, ref value) => string_tags.insert(util_base64::encode(&name), util_base64::encode(&value)),
            &Tag::PlainText(ref name, ref value) => string_tags.insert(format!("~{}", &util_base64::encode(&name)), value.to_string()),
        };
    }
    serde_json::to_string(&string_tags).map_err(|err| WalletStorageError::IOError(err.to_string()))
}

fn _tags_from_json(json: &str) -> Result<Vec<Tag>, WalletStorageError> {
    let string_tags: HashMap<String, String> = serde_json::from_str(json).map_err(|err| WalletStorageError::IOError(err.to_string()))?;
    let mut tags = Vec::new();

    for (k, v) in string_tags {
        if k.chars().next() == Some('~') {
            let mut key = k;
            key.remove(0);
            tags.push(
                Tag::PlainText(
                    util_base64::decode(&key).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    v
                )
            );
        } else {
            tags.push(
                Tag::Encrypted(
                    util_base64::decode(&k).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    util_base64::decode(&v).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        }
    }
    Ok(tags)
}

fn _tag_names_to_json(tag_names: &[TagName]) -> Result<String, WalletStorageError> {
    let mut tags: Vec<String> = Vec::new();

    for tag_name in tag_names {
        tags.push(
            match tag_name {
                &TagName::OfEncrypted(ref tag_name) => util_base64::encode(tag_name),
                &TagName::OfPlain(ref tag_name) => format!("~{}", util_base64::encode(tag_name))
            }
        )
    }
    serde_json::to_string(&tags).map_err(|err| WalletStorageError::IOError(err.to_string()))
}

fn _tag_names_from_json(json: &str) -> Result<Vec<TagName>, WalletStorageError> {
    let string_tag_names: Vec<String> = serde_json::from_str(json).map_err(|err| WalletStorageError::IOError(err.to_string()))?;
    let mut tag_names = Vec::new();

    for k in string_tag_names {
        if k.chars().next() == Some('~') {
            let mut key = k;
            key.remove(0);
            tag_names.push(
                TagName::OfPlain(
                    util_base64::decode(&key).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        } else {
            tag_names.push(
                TagName::OfEncrypted(
                    util_base64::decode(&k).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        }
    }
    Ok(tag_names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CString, CStr};
    use std::{slice, ptr};
    use wql::storage::ENCRYPTED_KEY_LEN;

    #[test]
    fn postgres_wallet_crud_works() {
        _cleanup();

        let id = _wallet_id();
        let config = _wallet_config();
        let credentials = _wallet_credentials();
        let metadata = _metadata();

        // open wallet - should return error
        let mut handle: i32 = -1;
        let err = PostgresWallet::open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::WalletNotFoundError);
        
        // create wallet
        let err = PostgresWallet::create(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            metadata.as_ptr());
        assert_eq!(err, ErrorCode::Success);

        // open wallet
        let err = PostgresWallet::open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::Success);

        // ensure we can fetch metadata
        let mut metadata_handle: i32 = -1;
        let mut metadata_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_storage_metadata(handle, 
                                            &mut metadata_ptr, 
                                            &mut metadata_handle);
        assert_eq!(err, ErrorCode::Success);
        let _metadata = unsafe { CStr::from_ptr(metadata_ptr).to_bytes() };
        let _metadata = unsafe { &*(_metadata as *const [u8] as *const [i8]) };
        //assert_eq!(_metadata.to_vec(), metadata);

        let err = PostgresWallet::free_storage_metadata(handle, metadata_handle);
        assert_eq!(err, ErrorCode::Success);

        // update metadata to some new metadata
        let metadata2 = _metadata2();
        let err = PostgresWallet::set_storage_metadata(handle, metadata2.as_ptr());
        assert_eq!(err, ErrorCode::Success);

        let mut metadata_handle2: i32 = -1;
        let mut metadata_ptr2: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_storage_metadata(handle, 
                                            &mut metadata_ptr2, 
                                            &mut metadata_handle2);
        assert_eq!(err, ErrorCode::Success);
        let _metadata2 = unsafe { CStr::from_ptr(metadata_ptr2).to_bytes() };
        let _metadata2 = unsafe { &*(_metadata2 as *const [u8] as *const [i8]) };
        //assert_eq!(_metadata2.to_vec(), metadata2);

        let err = PostgresWallet::free_storage_metadata(handle, metadata_handle2);
        assert_eq!(err, ErrorCode::Success);

        // close wallet
        let err = PostgresWallet::close(handle);
        assert_eq!(err, ErrorCode::Success);

        // delete wallet
        let err = PostgresWallet::delete(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));
        assert_eq!(err, ErrorCode::Success);

        // open wallet - should return error
        let err = PostgresWallet::open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::WalletNotFoundError);
    }

    #[test]
    fn postgres_wallet_add_record_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type_  = _type1();
        let id     = _id1();
        let value_ = _value1();
        let tags_  = _tags();

        let joined_value = value_.to_bytes();
        let tags  = _tags_json(&tags_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type_.as_ptr(),
                                id.as_ptr(),
                                joined_value.as_ptr(),
                                joined_value.len(),
                                tags.as_ptr());
        assert_match!(ErrorCode::Success, err);

        let err = PostgresWallet::add_record(handle,
                                type_.as_ptr(),
                                id.as_ptr(),
                                joined_value.as_ptr(),
                                joined_value.len(),
                                tags.as_ptr());
        assert_match!(ErrorCode::WalletItemAlreadyExists, err);

        let err = PostgresWallet::add_record(handle,
                                type_.as_ptr(),
                                id.as_ptr(),
                                joined_value.as_ptr(),
                                joined_value.len(),
                                tags.as_ptr());
        assert_match!(ErrorCode::WalletItemAlreadyExists, err);

        _close_and_delete_wallet(handle);
    }

    #[test]
    fn postgres_wallet_get_record_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let id1_    = _id_bytes1();
        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        let type2_  = _type2();
        let id2     = _id2();
        let value2_ = _value2();
        let tags2_  = _tags();

        let joined_value2 = value2_.to_bytes();
        let tags2  = _tags_json(&tags2_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type2_.as_ptr(),
                                id2.as_ptr(),
                                joined_value2.as_ptr(),
                                joined_value2.len(),
                                tags2.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the 2 records and verify
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::Success, err);

        let mut id_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_id(handle,
                                rec_handle,
                                &mut id_ptr);
        assert_match!(ErrorCode::Success, err);
        let _id = unsafe { CStr::from_ptr(id_ptr).to_bytes() };
        assert_eq!(_id.to_vec(), id1_);

        let mut type_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_type(handle,
                                rec_handle,
                                &mut type_ptr);
        assert_match!(ErrorCode::Success, err);
        let _type_ = unsafe { CStr::from_ptr(type_ptr).to_str().unwrap() };
        assert_eq!(_type_, type1_.to_str().unwrap());

        let mut value_bytes: *const u8 = ptr::null();
        let mut value_bytes_len: usize = 0;
        let err = PostgresWallet::get_record_value(handle,
                                rec_handle,
                                &mut value_bytes,
                                &mut value_bytes_len);
        assert_match!(ErrorCode::Success, err);
        let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
        let _value = EncryptedValue::from_bytes(value).unwrap();
        assert_eq!(_value, value1_);

        let mut tags_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_tags(handle,
                                rec_handle,
                                &mut tags_ptr);
        assert_match!(ErrorCode::Success, err);
        let tags_json = unsafe { CStr::from_ptr(tags_ptr).to_str().unwrap() };
        let _tags = _tags_from_json(tags_json).unwrap();
        let _tags = _sort_tags(_tags);
        let tags1_ = _sort_tags(tags1_);
        assert_eq!(_tags, tags1_);

        let err = PostgresWallet::free_record(handle, rec_handle);
        assert_match!(ErrorCode::Success, err);

        _close_and_delete_wallet(handle);
    }

    #[test]
    fn postgres_wallet_update_record_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let id1_    = _id_bytes1();
        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        let value2_ = _value2();
        let tags2_  = _tags();

        let joined_value2 = value2_.to_bytes();
        let tags2  = _tags_json(&tags2_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::update_record_value(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value2.as_ptr(),
                                joined_value2.len());
        assert_match!(ErrorCode::Success, err);

        let err = PostgresWallet::update_record_tags(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                tags2.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the record and verify updates
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::Success, err);

        let mut id_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_id(handle,
                                rec_handle,
                                &mut id_ptr);
        assert_match!(ErrorCode::Success, err);
        let _id = unsafe { CStr::from_ptr(id_ptr).to_bytes() };
        assert_eq!(_id.to_vec(), id1_);

        let mut type_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_type(handle,
                                rec_handle,
                                &mut type_ptr);
        assert_match!(ErrorCode::Success, err);
        let _type_ = unsafe { CStr::from_ptr(type_ptr).to_str().unwrap() };
        assert_eq!(_type_, type1_.to_str().unwrap());

        let mut value_bytes: *const u8 = ptr::null();
        let mut value_bytes_len: usize = 0;
        let err = PostgresWallet::get_record_value(handle,
                                rec_handle,
                                &mut value_bytes,
                                &mut value_bytes_len);
        assert_match!(ErrorCode::Success, err);
        let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
        let _value = EncryptedValue::from_bytes(value).unwrap();
        assert_eq!(_value, value2_);

        let mut tags_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_tags(handle,
                                rec_handle,
                                &mut tags_ptr);
        assert_match!(ErrorCode::Success, err);
        let tags_json = unsafe { CStr::from_ptr(tags_ptr).to_str().unwrap() };
        let _tags = _tags_from_json(tags_json).unwrap();
        let _tags = _sort_tags(_tags);
        let tags2_ = _sort_tags(tags2_);
        assert_eq!(_tags, tags2_);

        let err = PostgresWallet::free_record(handle, rec_handle);
        assert_match!(ErrorCode::Success, err);

        _close_and_delete_wallet(handle);
    }

    #[test]
    fn postgres_wallet_delete_record_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // add record to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the record
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::Success, err);

        let err = PostgresWallet::free_record(handle, rec_handle);
        assert_match!(ErrorCode::Success, err);

        // delete record
        let err = PostgresWallet::delete_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the record and verify it is not found
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::WalletItemNotFound, err);

        _close_and_delete_wallet(handle);
    }

    #[test]
    fn postgres_wallet_delete_tags_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // add record to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the record
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::Success, err);

        let err = PostgresWallet::free_record(handle, rec_handle);
        assert_match!(ErrorCode::Success, err);

        // delete tags
        let tag_names = _tag_names_to_delete();
        let tag_names = _tag_names_json(&tag_names);
        let err = PostgresWallet::delete_record_tags(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                tag_names.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the record
        let mut rec_handle: i32 = -1;
        let get_options = _fetch_options(true, true, true);
        let err = PostgresWallet::get_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                get_options.as_ptr() as *const i8,
                                &mut rec_handle);
        assert_match!(ErrorCode::Success, err);

        let mut tags_ptr: *const c_char = ptr::null_mut();
        let err = PostgresWallet::get_record_tags(handle,
                                rec_handle,
                                &mut tags_ptr);
        assert_match!(ErrorCode::Success, err);
        let tags_json = unsafe { CStr::from_ptr(tags_ptr).to_str().unwrap() };
        let _tags = _tags_from_json(tags_json).unwrap();
        let _tags = _sort_tags(_tags);
        let tags2_  = _tags_removed();
        let tags2_ = _sort_tags(tags2_);
        assert_eq!(_tags, tags2_);

        let err = PostgresWallet::free_record(handle, rec_handle);
        assert_match!(ErrorCode::Success, err);

        _close_and_delete_wallet(handle);
    }

    #[test]
    fn postgres_wallet_get_all_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        let type2_  = _type2();
        let id2     = _id2();
        let value2_ = _value2();
        let tags2_  = _tags();

        let joined_value2 = value2_.to_bytes();
        let tags2  = _tags_json(&tags2_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type2_.as_ptr(),
                                id2.as_ptr(),
                                joined_value2.as_ptr(),
                                joined_value2.len(),
                                tags2.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // fetch the 2 records and verify
        let mut search_handle: i32 = -1;
        let err = PostgresWallet::search_all_records(handle, &mut search_handle);
        assert_match!(ErrorCode::Success, err);
        
        let mut rec_count: i32 = 0;
        let mut search_continue: bool = true;
        while search_continue {
            let mut rec_handle = -1;
            let err = PostgresWallet::fetch_search_next_record(handle, search_handle, &mut rec_handle);
            if err == ErrorCode::WalletItemNotFound {
                search_continue = false;
            } else if err != ErrorCode::Success {
                search_continue = false;
            }

            if search_continue {
                rec_count = rec_count + 1;

                // fetch the record just to verify we can ...
                let mut value_bytes: *const u8 = ptr::null();
                let mut value_bytes_len: usize = 0;
                let err = PostgresWallet::get_record_value(handle,
                                        rec_handle,
                                        &mut value_bytes,
                                        &mut value_bytes_len);
                assert_match!(ErrorCode::Success, err);
                let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
                let _value = EncryptedValue::from_bytes(value).unwrap();

                let mut tags_ptr: *const c_char = ptr::null_mut();
                let err = PostgresWallet::get_record_tags(handle,
                                        rec_handle,
                                        &mut tags_ptr);
                assert_match!(ErrorCode::Success, err);
                let tags_json = unsafe { CStr::from_ptr(tags_ptr).to_str().unwrap() };
                let _tags = _tags_from_json(tags_json).unwrap();
                let _tags = _sort_tags(_tags);

                // free record once done
                let err = PostgresWallet::free_record(handle, rec_handle);
                assert_match!(ErrorCode::Success, err);
            }
        }
        // confirm 2 records total
        assert_eq!(2, rec_count);

        _close_and_delete_wallet(handle);
    }
/* TODO unit test for wallet search
    #[test]
    fn postgres_wallet_search_records_works() {
        _cleanup();

        let handle = _create_and_open_wallet();

        let type1_  = _type1();
        let id1     = _id1();
        let value1_ = _value1();
        let tags1_  = _tags();

        let joined_value1 = value1_.to_bytes();
        let tags1  = _tags_json(&tags1_);

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id1.as_ptr(),
                                joined_value1.as_ptr(),
                                joined_value1.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        let id2     = _id2();
        let value2_ = _value2();

        let joined_value2 = value2_.to_bytes();

        // unit test for adding record(s) to the wallet
        let err = PostgresWallet::add_record(handle,
                                type1_.as_ptr(),
                                id2.as_ptr(),
                                joined_value2.as_ptr(),
                                joined_value2.len(),
                                tags1.as_ptr());
        assert_match!(ErrorCode::Success, err);

        // search the records and verify
        println!("Setting up search query");
        let mut search_handle: i32 = -1;
        //let tag_name = String::from_utf8(vec![1, 5, 8]).unwrap();
        //let tag_value = String::from_utf8(vec![3, 5, 6]).unwrap();
        let tag_name = format!("{:?}", vec![1, 5, 8]);
        let tag_value = format!("{:?}", vec![3, 5, 6]);
        let query_json = format!(r#"{{{}:{}}}"#, tag_name, tag_value);
        let query_json = CString::new(query_json.to_string()).unwrap();
        println!("query_json {:?}", query_json);
        let options_json = _search_options(true, true, true, true, true);
        println!("Options {:?}", options_json);
        let err = PostgresWallet::search_records(handle, 
                                type1_.as_ptr(), 
                                query_json.as_ptr(), 
                                options_json.as_ptr() as *const i8, 
                                &mut search_handle);
        assert_match!(ErrorCode::Success, err);
        
        let mut rec_count: i32 = 0;
        let mut search_continue: bool = true;
        while search_continue {
            let mut rec_handle = -1;
            let err = PostgresWallet::fetch_search_next_record(handle, search_handle, &mut rec_handle);
            if err == ErrorCode::WalletItemNotFound {
                search_continue = false;
            } else if err != ErrorCode::Success {
                search_continue = false;
            }

            if search_continue {
                rec_count = rec_count + 1;

                // fetch the record just to verify we can ...
                let mut value_bytes: *const u8 = ptr::null();
                let mut value_bytes_len: usize = 0;
                let err = PostgresWallet::get_record_value(handle,
                                        rec_handle,
                                        &mut value_bytes,
                                        &mut value_bytes_len);
                assert_match!(ErrorCode::Success, err);
                let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
                let _value = EncryptedValue::from_bytes(value).unwrap();

                let mut tags_ptr: *const c_char = ptr::null_mut();
                let err = PostgresWallet::get_record_tags(handle,
                                        rec_handle,
                                        &mut tags_ptr);
                assert_match!(ErrorCode::Success, err);
                let tags_json = unsafe { CStr::from_ptr(tags_ptr).to_str().unwrap() };
                let _tags = _tags_from_json(tags_json).unwrap();
                let _tags = _sort_tags(_tags);

                // free record once done
                let err = PostgresWallet::free_record(handle, rec_handle);
                assert_match!(ErrorCode::Success, err);
            }
        }
        // confirm 2 records total
        assert_eq!(2, rec_count);

        _close_and_delete_wallet(handle);
    }
*/
    fn _create_and_open_wallet() -> i32 {
        let id = _wallet_id();
        let config = _wallet_config();
        let credentials = _wallet_credentials();
        let metadata = _metadata();

        // create wallet
        let err = PostgresWallet::create(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            metadata.as_ptr());
        assert_eq!(err, ErrorCode::Success);

        // open wallet
        let mut handle: i32 = -1;
        let err = PostgresWallet::open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::Success);

        handle
    }

    fn _close_and_delete_wallet(handle: i32) {
        let id = _wallet_id();
        let config = _wallet_config();
        let credentials = _wallet_credentials();

        // close wallet
        let err = PostgresWallet::close(handle);
        assert_eq!(err, ErrorCode::Success);

        // delete wallet
        let err = PostgresWallet::delete(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));
        assert_eq!(err, ErrorCode::Success);
    }

    fn _cleanup() {
        let id = _wallet_id();
        let config = _wallet_config();
        let credentials = _wallet_credentials();

        let _err = PostgresWallet::delete(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));
    }

    fn _wallet_id() -> CString {
        CString::new("my_walle1").unwrap()
    }

    fn _wallet_config() -> Option<CString> {
        let config = Some(json!({
            "url": "localhost:5432".to_owned()
        }).to_string());
        config.map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap()
    }

    fn _wallet_credentials() -> Option<CString> {
        let creds = Some(json!({
            "account": "postgres".to_owned(),
            "password": "mysecretpassword".to_owned(),
            "admin_account": Some("postgres".to_owned()),
            "admin_password": Some("mysecretpassword".to_owned())
        }).to_string());
        creds.map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap()
    }

    fn _metadata() -> Vec<i8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }

    fn _metadata2() -> Vec<i8> {
        return vec![
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 4, 5, 6, 7, 8, 9
        ];
    }

    fn _type(i: u8) -> CString {
        let type_ = vec![i, 1 + i, 2 + i];
        CString::new(type_.clone()).unwrap()
    }

    fn _type1() -> CString {
        _type(1)
    }

    fn _type2() -> CString {
        _type(2)
    }

    fn _id_bytes(i: u8) -> Vec<u8> {
        vec![3 + i, 4 + i, 5 + i]
    }

    fn _id_bytes1() -> Vec<u8> {
        _id_bytes(1)
    }

    fn _id_bytes2() -> Vec<u8> {
        _id_bytes(2)
    }

    fn _id(i: u8) -> CString {
        let id_ = _id_bytes(i);
        CString::new(id_.clone()).unwrap()
    }

    fn _id1() -> CString {
        _id(1)
    }

    fn _id2() -> CString {
        _id(2)
    }

    fn _value(i: u8) -> EncryptedValue {
        EncryptedValue { data: vec![6 + i, 7 + i, 8 + i], key: _key(i) }
    }

    fn _value1() -> EncryptedValue {
        _value(1)
    }

    fn _value2() -> EncryptedValue {
        _value(2)
    }

    fn _key(i: u8) -> Vec<u8> {
        vec![i; ENCRYPTED_KEY_LEN]
    }

    fn _tags() -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value 1".to_string()));
        tags.push(Tag::Encrypted(vec![2, 5, 8], vec![3, 5, 7]));
        tags.push(Tag::PlainText(vec![2, 5, 8, 1], "Plain value 2".to_string()));
        tags
    }

    fn _tags_removed() -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![2, 5, 8], vec![3, 5, 7]));
        tags.push(Tag::PlainText(vec![2, 5, 8, 1], "Plain value 2".to_string()));
        tags
    }

    fn _tag_names_to_delete() -> Vec<TagName> {
        vec![
            TagName::OfEncrypted(vec![1, 5, 8]),
            TagName::OfPlain(vec![1, 5, 8, 1])
        ]
    }

    fn _tag_names_json(tag_names_: &Vec<TagName>) -> CString {
        CString::new(_tag_names_to_json(tag_names_).unwrap()).unwrap()
    }

    fn _tags_json(tags_: &Vec<Tag>) -> CString {
        CString::new(_tags_to_json(tags_).unwrap()).unwrap()
    }

    fn _new_tags() -> Vec<Tag> {
        vec![
            Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]),
            Tag::PlainText(vec![1, 1, 1], String::from("tag_value_3"))
        ]
    }

    fn _sort_tags(mut v: Vec<Tag>) -> Vec<Tag> {
        v.sort();
        v
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
            "retrieveType": type_,
            "retrieveValue": value,
            "retrieveTags": tags,
        }).to_string()
    }

    fn _search_options(retrieve_records: bool, retrieve_total_count: bool, retrieve_value: bool, retrieve_tags: bool, retrieve_type: bool) -> String {
        let mut map = HashMap::new();

        map.insert("retrieveRecords", retrieve_records);
        map.insert("retrieveTotalCount", retrieve_total_count);
        map.insert("retrieveValue", retrieve_value);
        map.insert("retrieveTags", retrieve_tags);
        map.insert("retrieveType", retrieve_type);

        serde_json::to_string(&map).unwrap()
    }

}
