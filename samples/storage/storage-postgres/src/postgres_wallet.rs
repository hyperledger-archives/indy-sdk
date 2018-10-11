extern crate libc;
extern crate time;
extern crate indy;
extern crate indy_crypto;
extern crate serde_json;

use indy::api::ErrorCode;
use utils::sequence::SequenceUtils;
use utils::ctypes;
use postgres_storage::storage::WalletStorageType;

use self::libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;

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
                             _: *const c_char,
                             _: *const c_char,
                             metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidStructure);

        // TODO start create Postgres database, create schema, and insert metadata
        // TODO PostgresStorageType::create_storage()
        let mut wallets = INMEM_WALLETS.lock().unwrap();
        if wallets.contains_key(&id) {
            // Invalid state as "already exists" case must be checked on service layer
            return ErrorCode::CommonInvalidState;
        }

        wallets.insert(id.clone(), InmemWalletEntity { metadata: CString::new(metadata).unwrap(), records: HashMap::new() });
        // TODO end

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
        let phandle = storage_type.open_storage(&id, Some(&config), Some(&credentials)).unwrap();

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
                                 value: *const u8,
                                 value_len: usize,
                                 tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_byte_array!(value, value_len, ErrorCode::CommonInvalidStructure, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        // TODO start add record
        // TODO PostgresStorage::add() from the handle
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

        wallet.records.insert(PostgresWallet::build_record_id(&type_, &id), InmemWalletRecord {
            type_: CString::new(type_).unwrap(),
            id: CString::new(id).unwrap(),
            value,
            tags: CString::new(tags_json).unwrap(),
        });
        ErrorCode::Success
        // TODO end
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
        // TODO start
        // TODO PostgresStorage::close() from the handle
        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        handles.remove(&xhandle);
        ErrorCode::Success
        // TODO end
    }

#[no_mangle]
    pub extern "C" fn postgreswallet_fn_delete(name: *const c_char,
                             _: *const c_char,
                             _: *const c_char) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);

        // TODO start
        // TODO PostgresStorageType::delete_storage()
        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&name) {
            return ErrorCode::CommonInvalidState;
        }

        wallets.remove(&name);
        ErrorCode::Success
        // TODO end
    }

    pub fn cleanup() {
        let mut wallets = INMEM_WALLETS.lock().unwrap();
        wallets.clear();

        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();
        handles.clear();
    }
}
