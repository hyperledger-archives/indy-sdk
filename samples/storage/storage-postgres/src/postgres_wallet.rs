extern crate libc;
extern crate time;
extern crate indy;
extern crate indy_crypto;
extern crate serde_json;

use indy::api::ErrorCode;
use utils::sequence::SequenceUtils;
use utils::ctypes;
use utils::crypto::base64;
use postgres_storage::storage::{WalletStorage, WalletStorageType, Tag, TagName, EncryptedValue};
use indy::errors::wallet::WalletStorageError;

use self::libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;
use std::{slice, str};


// TODO replaced by PostgresStorage
#[derive(Debug)]
struct InmemWalletContext {
    id: String
}

struct PostgresStorageContext {
    xhandle: i32,        // reference returned to client to track open wallet connection
    id: String,          // wallet name
    config: String,      // wallet config
    credentials: String, // wallet credentials
    phandle: Box<::postgres_storage::PostgresStorage>  // reference to a postgres database connection
}

#[derive(Debug, Clone)]
struct InmemWalletRecord {
    type_: CString,
    id: CString,
    value: Vec<u8>,
    tags: CString
}

#[derive(Debug, Clone)]
struct InmemWalletEntity {
    metadata: CString,
    records: HashMap<String, InmemWalletRecord>
}

lazy_static! {
    // TODO we don't need to keep this list - wallets are in Postgres, not inmem
    static ref INMEM_WALLETS: Mutex<HashMap<String, InmemWalletEntity>> = Default::default();
}

lazy_static! {
    // TODO store a PostgresStorage object (contains a connection) instead of an InmemWalletContext
    static ref INMEM_OPEN_WALLETS: Mutex<HashMap<i32, InmemWalletContext>> = Default::default();
}

lazy_static! {
    // store a PostgresStorage object (contains a connection) instead of an InmemWalletContext
    static ref POSTGRES_OPEN_WALLETS: Mutex<HashMap<i32, PostgresStorageContext>> = Default::default();
}

lazy_static! {
    // TODO I don't think we need
    static ref ACTIVE_METADATAS: Mutex<HashMap<i32, CString>> = Default::default();
}

lazy_static! {
    // TODO I don't think we need
    static ref ACTIVE_RECORDS: Mutex<HashMap<i32, InmemWalletRecord>> = Default::default();
}

lazy_static! {
    // TODO potentially we need, review when we review searches
    static ref ACTIVE_SEARCHES: Mutex<HashMap<i32, Vec<InmemWalletRecord>,>> = Default::default();
}

pub struct PostgresWallet {}

impl PostgresWallet {
#[no_mangle]
    pub extern "C" fn postgreswallet_fn_create(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char,
                             metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(config, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidStructure);

        // create Postgres database, create schema, and insert metadata
        let storage_type = ::postgres_storage::PostgresStorageType::new();
        storage_type.create_storage(&id, Some(&config), Some(&credentials), &metadata.as_bytes()[..]).unwrap();

        ErrorCode::Success
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_open(id: *const c_char,
                           config: *const c_char,
                           credentials: *const c_char,
                           handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(config, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidStructure);

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
            xhandle,      // reference returned to client to track open wallet connection
            id,           // wallet name
            config,       // wallet config
            credentials,  // wallet credentials
            phandle       // reference to a postgres database connection
        };

        // add to our open wallet list
        handles.insert(xhandle, context);

        // return handle = index into our collection of open wallets
        unsafe { *handle = xhandle };
        ErrorCode::Success
    }

    // TODO this is not required for postgres wallet (?)
    fn build_record_id(type_: &str, id: &str) -> String {
        format!("{}-{}", type_, id)
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_add_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 value_bytes: *const u8,
                                 value_bytes_len: usize,
                                 tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_byte_array!(value_bytes, value_bytes_len, ErrorCode::CommonInvalidStructure, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        // call PostgresStorage::add() from the handle
        let handles = POSTGRES_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let value = unsafe { slice::from_raw_parts(value_bytes.as_ptr(), value_bytes_len) };
        let value = EncryptedValue::from_bytes(value).unwrap();
        let tags = _tags_from_json(&tags_json).unwrap();

        let wallet_context = handles.get(&xhandle).unwrap();
        let wallet_box = &wallet_context.phandle;
        let storage = &*wallet_box;

        let res = storage.add(&type_.as_bytes(), &id.as_bytes(), &value, &tags);

        match res {
            Ok(_) => ErrorCode::Success,
            Err(_err) => ErrorCode::WalletStorageError
        }
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_update_record_value(xhandle: i32,
                                          type_: *const c_char,
                                          id: *const c_char,
                                          joined_value: *const u8,
                                          joined_value_len: usize) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_byte_array!(joined_value, joined_value_len, ErrorCode::CommonInvalidStructure, ErrorCode::CommonInvalidStructure);

        // TODO start update record value
        // TODO PostgresStorage::update() from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        match wallet.records.get_mut(&PostgresWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => record.value = joined_value,
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 options_json: *const c_char,
                                 handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidStructure);

        // TODO start get record
        // TODO PostgresStorage::get(options) from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.id).unwrap();

        let key = PostgresWallet::build_record_id(&type_, &id);

        if !wallet.records.contains_key(&key) {
            return ErrorCode::WalletItemNotFound;
        }

        let record = wallet.records.get(&key).unwrap();

        let record_handle = SequenceUtils::get_next_id();

        let mut handles = ACTIVE_RECORDS.lock().unwrap();
        handles.insert(record_handle, record.clone());

        unsafe { *handle = record_handle };

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_record_id(xhandle: i32,
                                    record_handle: i32,
                                    id_ptr: *mut *const c_char) -> ErrorCode {
        // TODO start get record id
        // TODO PostgresStorage::get(options) from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *id_ptr = record.id.as_ptr(); }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_record_type(xhandle: i32,
                                      record_handle: i32,
                                      type_ptr: *mut *const c_char) -> ErrorCode {
        // TODO start get record type
        // TODO PostgresStorage::get(options) from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *type_ptr = record.type_.as_ptr(); }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_record_value(xhandle: i32,
                                       record_handle: i32,
                                       value_ptr: *mut *const u8,
                                       value_len: *mut usize) -> ErrorCode {
        // TODO start get record value
        // TODO PostgresStorage::get(options) from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *value_ptr = record.value.as_ptr() as *const u8; }
        unsafe { *value_len = record.value.len() as usize; }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_record_tags(xhandle: i32,
                                      record_handle: i32,
                                      tags_json_ptr: *mut *const c_char) -> ErrorCode {
        // TODO start get record tags
        // TODO PostgresStorage::get(options) from the handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        unsafe { *tags_json_ptr = record.tags.as_ptr(); }

        ErrorCode::Success
        // TODO end
    }


#[no_mangle]
    pub extern "C" fn postgreswallet_fn_free_record(xhandle: i32, record_handle: i32) -> ErrorCode {
        // TODO start free record
        // TODO not sure how this maps
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&record_handle);

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_add_record_tags(xhandle: i32,
                                      type_: *const c_char,
                                      id: *const c_char,
                                      tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        // TODO start add record tags
        // TODO PostgresStorage::add_tags() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        match wallet.records.get_mut(&PostgresWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => {
                let curr_tags_json =record.tags.to_str().unwrap().to_string() ;

                let new_tags_result = serde_json::from_str::<HashMap<String, String>>(&tags_json);
                let curr_tags_result = serde_json::from_str::<HashMap<String, String>>(&curr_tags_json);

                let (new_tags, mut curr_tags) = match (new_tags_result, curr_tags_result) {
                    (Ok(new), Ok(cur)) => (new, cur),
                    _ => return ErrorCode::CommonInvalidStructure
                };

                curr_tags.extend(new_tags);

                let new_tags_json = serde_json::to_string(&curr_tags).unwrap();

                record.tags = CString::new(new_tags_json).unwrap();
            }
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_update_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        // TODO update tags
        // TODO PostgresStorage::update_tags() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        match wallet.records.get_mut(&PostgresWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => record.tags = CString::new(tags_json).unwrap(),
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_delete_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tag_names: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tag_names, ErrorCode::CommonInvalidStructure);

        // TODO delete tags
        // TODO PostgresStorage::delete_tags() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        match wallet.records.get_mut(&PostgresWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => {
                let curr_tags_json = record.tags.to_str().unwrap().to_string() ;

                let mut curr_tags_res = serde_json::from_str::<HashMap<String, String>>(&curr_tags_json);
                let tags_names_to_delete = serde_json::from_str::<Vec<String>>(&tag_names);

                let (mut curr_tags, tags_delete) = match (curr_tags_res, tags_names_to_delete) {
                    (Ok(cur), Ok(to_delete)) => (cur, to_delete),
                    _ => return ErrorCode::CommonInvalidStructure
                };

                for tag_name in tags_delete {
                    curr_tags.remove(&tag_name);
                }

                let new_tags_json = serde_json::to_string(&curr_tags).unwrap();

                record.tags = CString::new(new_tags_json).unwrap()
            }
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
        // END TODO
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_delete_record(xhandle: i32,
                                    type_: *const c_char,
                                    id: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);

        // TODO delete record
        // TODO PostgresStorage::delete() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        let key = PostgresWallet::build_record_id(&type_, &id);

        if !wallet.records.contains_key(&key) {
            return ErrorCode::WalletItemNotFound;
        }

        wallet.records.remove(&key);

        ErrorCode::Success
        // END TODO
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_storage_metadata(xhandle: i32, metadata_ptr: *mut *const c_char, metadata_handle: *mut i32) -> ErrorCode {
        // TODO get metadata
        // TODO PostgresStorage::get_storage_metadata() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.id).unwrap();

        let metadata = wallet.metadata.clone();
        let metadata_pointer = metadata.as_ptr();

        let handle = SequenceUtils::get_next_id();

        let mut metadatas = ACTIVE_METADATAS.lock().unwrap();
        metadatas.insert(handle, metadata);

        unsafe { *metadata_ptr = metadata_pointer; }
        unsafe { *metadata_handle = handle };

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_set_storage_metadata(xhandle: i32, metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidStructure);

        // TODO set metadata
        // TODO PostgresStorage::set_storage_metadata() from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.id).unwrap();

        wallet.metadata = CString::new(metadata).unwrap();

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_free_storage_metadata(xhandle: i32, metadata_handler: i32) -> ErrorCode {
        // TODO start
        // TODO t.b.d. not sure
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = ACTIVE_METADATAS.lock().unwrap();

        if !handles.contains_key(&metadata_handler) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&metadata_handler);

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_search_records(xhandle: i32, type_: *const c_char, _query_json: *const c_char, _options_json: *const c_char, handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);

        // TODO start
        // TODO PostgresStorage::search(options) from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.id).unwrap();

        let search_records = wallet.records
            .iter()
            .filter(|&(key, _)| key.starts_with(&type_))
            .map(|(_, value)| value.clone())
            .collect::<Vec<InmemWalletRecord>>();

        let search_handle = SequenceUtils::get_next_id();

        let mut searches = ACTIVE_SEARCHES.lock().unwrap();

        searches.insert(search_handle, search_records);

        unsafe { *handle = search_handle };

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_search_all_records(xhandle: i32, handle: *mut i32) -> ErrorCode {
        // TODO start
        // TODO PostgresStorage::get_all(options) from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.id) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.id).unwrap();

        let search_records = wallet.records
            .values()
            .cloned()
            .collect::<Vec<InmemWalletRecord>>();

        let search_handle = SequenceUtils::get_next_id();

        let mut searches = ACTIVE_SEARCHES.lock().unwrap();

        searches.insert(search_handle, search_records);

        unsafe { *handle = search_handle };

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_get_search_total_count(xhandle: i32, search_handle: i32, count: *mut usize) -> ErrorCode {
        // TODO start
        // TODO PostgresStorage::get_all(options) from handle
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let searches = ACTIVE_SEARCHES.lock().unwrap();

        match searches.get(&search_handle) {
            Some(records) => {
                unsafe { *count = records.len() };
            }
            None => return ErrorCode::CommonInvalidState
        }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_fetch_search_next_record(xhandle: i32, search_handle: i32, record_handle: *mut i32) -> ErrorCode {
        // TODO start
        // TODO storage iterator???
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut searches = ACTIVE_SEARCHES.lock().unwrap();

        match searches.get_mut(&search_handle) {
            Some(records) => {
                match records.pop() {
                    Some(record) => {
                        let handle = SequenceUtils::get_next_id();

                        let mut handles = ACTIVE_RECORDS.lock().unwrap();
                        handles.insert(handle, record.clone());

                        unsafe { *record_handle = handle };
                    }
                    None => return ErrorCode::WalletItemNotFound
                }
            }
            None => return ErrorCode::CommonInvalidState
        }

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_free_search(xhandle: i32, search_handle: i32) -> ErrorCode {
        // TODO start
        // TODO not sure ???
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = ACTIVE_SEARCHES.lock().unwrap();

        if !handles.contains_key(&search_handle) {
            return ErrorCode::CommonInvalidState;
        }
        handles.remove(&search_handle);

        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_close(xhandle: i32) -> ErrorCode {
        // PostgresStorage::close() from the handle
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

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_delete(id: *const c_char,
                             config: *const c_char,
                             credentials: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(config, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(credentials, ErrorCode::CommonInvalidStructure);

        // PostgresStorageType::delete_storage()
        let storage_type = ::postgres_storage::PostgresStorageType::new();
        storage_type.delete_storage(&id, Some(&config), Some(&credentials)).unwrap();

        ErrorCode::Success
    }

    pub fn cleanup() {
        let mut wallets = INMEM_WALLETS.lock().unwrap();
        wallets.clear();

        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();
        handles.clear();
    }
}

fn _tags_to_json(tags: &[Tag]) -> Result<String, WalletStorageError> {
    let mut string_tags = HashMap::new();
    for tag in tags {
        match tag {
            &Tag::Encrypted(ref name, ref value) => string_tags.insert(base64::encode(&name), base64::encode(&value)),
            &Tag::PlainText(ref name, ref value) => string_tags.insert(format!("~{}", &base64::encode(&name)), value.to_string()),
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
                    base64::decode(&key).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    v
                )
            );
        } else {
            tags.push(
                Tag::Encrypted(
                    base64::decode(&k).map_err(|err| WalletStorageError::IOError(err.to_string()))?,
                    base64::decode(&v).map_err(|err| WalletStorageError::IOError(err.to_string()))?
                )
            );
        }
    }
    Ok(tags)
}

fn _tags_names_to_json(tag_names: &[TagName]) -> Result<String, WalletStorageError> {
    let mut tags: Vec<String> = Vec::new();

    for tag_name in tag_names {
        tags.push(
            match tag_name {
                &TagName::OfEncrypted(ref tag_name) => base64::encode(tag_name),
                &TagName::OfPlain(ref tag_name) => format!("~{}", base64::encode(tag_name))
            }
        )
    }
    serde_json::to_string(&tags).map_err(|err| WalletStorageError::IOError(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    //use utils::test;

    #[test]
    fn postgres_wallet_crud_works() {
        let id = _wallet_id();
        let config = Some(_wallet_config());
        let credentials = Some(_wallet_credentials());
        let metadata = _metadata();

        let id = CString::new(id).unwrap();
        let config = config
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();
        let credentials = credentials
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();

        // open wallet - should return error
        let mut handle: i32 = -1;
        let err = PostgresWallet::postgreswallet_fn_open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::WalletNotFoundError);
        
        // create wallet
        let err = PostgresWallet::postgreswallet_fn_create(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            metadata.as_ptr());
        assert_eq!(err, ErrorCode::Success);

        // open wallet
        let err = PostgresWallet::postgreswallet_fn_open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::Success);

        // close wallet
        let err = PostgresWallet::postgreswallet_fn_close(handle);
        assert_eq!(err, ErrorCode::Success);

        // delete wallet
        let err = PostgresWallet::postgreswallet_fn_delete(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));
        assert_eq!(err, ErrorCode::Success);

        // open wallet - should return error
        let err = PostgresWallet::postgreswallet_fn_open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::WalletNotFoundError);
    }

    #[test]
    fn postgres_wallet_add_record_works() {
        let handle = _create_and_open_wallet();

        // TODO construct unit test for adding record(s) to the wallet

        _close_and_delete_wallet(handle);
    }

    fn _create_and_open_wallet() -> i32 {
        let id = _wallet_id();
        let config = Some(_wallet_config());
        let credentials = Some(_wallet_credentials());
        let metadata = _metadata();

        let id = CString::new(id).unwrap();
        let config = config
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();
        let credentials = credentials
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();

        // create wallet
        let err = PostgresWallet::postgreswallet_fn_create(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            metadata.as_ptr());
        assert_eq!(err, ErrorCode::Success);

        // open wallet
        let mut handle: i32 = -1;
        let err = PostgresWallet::postgreswallet_fn_open(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            &mut handle);
        assert_eq!(err, ErrorCode::Success);

        handle
    }

    fn _close_and_delete_wallet(handle: i32) {
        let id = _wallet_id();
        let config = Some(_wallet_config());
        let credentials = Some(_wallet_credentials());

        let id = CString::new(id).unwrap();
        let config = config
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();
        let credentials = credentials
            .map(CString::new)
            .map_or(Ok(None), |r| r.map(Some)).unwrap();

        // close wallet
        let err = PostgresWallet::postgreswallet_fn_close(handle);
        assert_eq!(err, ErrorCode::Success);

        // delete wallet
        let err = PostgresWallet::postgreswallet_fn_delete(id.as_ptr(), 
                                            config.as_ref().map_or(ptr::null(), |x| x.as_ptr()), 
                                            credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));
        assert_eq!(err, ErrorCode::Success);
    }

    fn _wallet_id() -> &'static str {
        "my_walle1"
    }

    fn _wallet_config() -> String {
        let config = json!({
            "url": "localhost:5432".to_owned()
        }).to_string();
        config
    }

    fn _wallet_credentials() -> String {
        let creds = json!({
            "account": "postgres".to_owned(),
            "password": "mysecretpassword".to_owned(),
            "admin_account": Some("postgres".to_owned()),
            "admin_password": Some("mysecretpassword".to_owned())
        }).to_string();
        creds
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

}
