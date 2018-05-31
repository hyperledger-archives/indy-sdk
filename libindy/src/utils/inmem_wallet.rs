extern crate libc;
extern crate time;
extern crate indy_crypto;
extern crate serde_json;

use api::ErrorCode;
use utils::cstring::CStringUtils;
use utils::sequence::SequenceUtils;

use self::libc::c_char;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;

#[derive(Debug)]
struct InmemWalletContext {
    name: String
}

#[derive(Debug, Clone)]
struct InmemWalletRecord {
    type_: String,
    id: String,
    value: Vec<u8>,
    tags: String
}

#[derive(Debug, Clone)]
struct InmemWalletEntity {
    metadata: String,
    records: HashMap<String, InmemWalletRecord>
}

lazy_static! {
    static ref INMEM_WALLETS: Mutex<HashMap<String, InmemWalletEntity>> = Default::default();
}

lazy_static! {
    static ref INMEM_OPEN_WALLETS: Mutex<HashMap<i32, InmemWalletContext>> = Default::default();
}

lazy_static! {
    static ref ACTIVE_METADATAS: Mutex<HashMap<i32, CString>> = Default::default();
}

lazy_static! {
    static ref ACTIVE_RECORDS: Mutex<HashMap<i32, InmemWalletRecord>> = Default::default();
}

lazy_static! {
    static ref ACTIVE_SEARCHES: Mutex<HashMap<i32, Vec<InmemWalletRecord>,>> = Default::default();
}

pub struct InmemWallet {}

impl InmemWallet {
    pub extern "C" fn create(name: *const c_char,
                             _: *const c_char,
                             _: *const c_char,
                             metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidStructure);

        let mut wallets = INMEM_WALLETS.lock().unwrap();
        if wallets.contains_key(&name) {
            // Invalid state as "already exists" case must be checked on service layer
            return ErrorCode::CommonInvalidState;
        }
        wallets.insert(name.clone(), InmemWalletEntity { metadata, records: HashMap::new() });

        ErrorCode::Success
    }

    pub extern "C" fn open(name: *const c_char,
                           _: *const c_char,
                           _: *const c_char,
                           _: *const c_char,
                           handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&name) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();
        let xhandle = SequenceUtils::get_next_id();
        handles.insert(xhandle, InmemWalletContext {
            name,
        });

        unsafe { *handle = xhandle };
        ErrorCode::Success
    }

    fn build_record_id(type_: &str, id: &str) -> String {
        format!("{}-{}", type_, id)
    }

    pub extern "C" fn add_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 value: *const u8,
                                 value_len: usize,
                                 tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_byte_array!(value, value_len, ErrorCode::CommonInvalidStructure, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        wallet.records.insert(InmemWallet::build_record_id(&type_, &id), InmemWalletRecord {
            type_,
            id,
            value,
            tags: tags_json,
        });
        ErrorCode::Success
    }

    pub extern "C" fn update_record_value(xhandle: i32,
                                          type_: *const c_char,
                                          id: *const c_char,
                                          joined_value: *const u8,
                                          joined_value_len: usize) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_byte_array!(joined_value, joined_value_len, ErrorCode::CommonInvalidStructure, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        match wallet.records.get_mut(&InmemWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => record.value = joined_value,
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
    }

    pub extern "C" fn get_record(xhandle: i32,
                                 type_: *const c_char,
                                 id: *const c_char,
                                 options_json: *const c_char,
                                 handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(options_json, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

        let key = InmemWallet::build_record_id(&type_, &id);

        if !wallet.records.contains_key(&key) {
            return ErrorCode::WalletItemNotFound;
        }

        let record = wallet.records.get(&key).unwrap();

        let record_handle = SequenceUtils::get_next_id();

        let mut handles = ACTIVE_RECORDS.lock().unwrap();
        handles.insert(record_handle, record.clone());

        unsafe { *handle = record_handle };

        ErrorCode::Success
    }

    pub extern "C" fn get_record_id(xhandle: i32,
                                    record_handle: i32,
                                    id_ptr: *mut *const c_char) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        let value = record.id.clone();

        unsafe { *id_ptr = CString::new(value.as_str()).unwrap().into_raw(); }

        ErrorCode::Success
    }

    pub extern "C" fn get_record_type(xhandle: i32,
                                      record_handle: i32,
                                      type_ptr: *mut *const c_char) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        let type_ = record.type_.clone();

        unsafe { *type_ptr = CString::new(type_.as_str()).unwrap().into_raw(); }

        ErrorCode::Success
    }

    pub extern "C" fn get_record_value(xhandle: i32,
                                       record_handle: i32,
                                       value_ptr: *mut *const u8,
                                       value_len: *mut usize) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        let value = record.value.clone();

        unsafe { *value_ptr = value.as_ptr() as *const u8; }
        unsafe { *value_len = value.len() as usize; }

        ErrorCode::Success
    }

    pub extern "C" fn get_record_tags(xhandle: i32,
                                      record_handle: i32,
                                      tags_json_ptr: *mut *const c_char) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let handles = ACTIVE_RECORDS.lock().unwrap();

        if !handles.contains_key(&record_handle) {
            return ErrorCode::CommonInvalidState;
        }

        let record = handles.get(&record_handle).unwrap();

        let tags = record.tags.clone();

        unsafe { *tags_json_ptr = CString::new(tags.as_str()).unwrap().into_raw(); }

        ErrorCode::Success
    }


    pub extern "C" fn free_record(xhandle: i32, record_handle: i32) -> ErrorCode {
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
    }

    pub extern "C" fn add_record_tags(xhandle: i32,
                                      type_: *const c_char,
                                      id: *const c_char,
                                      tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        match wallet.records.get_mut(&InmemWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => {
                let curr_tags_json = record.tags.clone();

                let new_tags_result = serde_json::from_str::<HashMap<String, String>>(&tags_json);
                let curr_tags_result = serde_json::from_str::<HashMap<String, String>>(&curr_tags_json);

                let (new_tags, mut curr_tags) = match (new_tags_result, curr_tags_result) {
                    (Ok(new), Ok(cur)) => (new, cur),
                    _ => return ErrorCode::CommonInvalidStructure
                };

                curr_tags.extend(new_tags);

                let new_tags_json = serde_json::to_string(&curr_tags).unwrap();

                record.tags = new_tags_json
            }
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
    }

    pub extern "C" fn update_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags_json: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tags_json, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        match wallet.records.get_mut(&InmemWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => record.tags = tags_json,
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
    }

    pub extern "C" fn delete_record_tags(xhandle: i32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tag_names: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(tag_names, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        match wallet.records.get_mut(&InmemWallet::build_record_id(&type_, &id)) {
            Some(ref mut record) => {
                let curr_tags_json = record.tags.clone();

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

                record.tags = new_tags_json
            }
            None => return ErrorCode::WalletItemNotFound
        }

        ErrorCode::Success
    }

    pub extern "C" fn delete_record(xhandle: i32,
                                    type_: *const c_char,
                                    id: *const c_char) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(id, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        let key = InmemWallet::build_record_id(&type_, &id);

        if !wallet.records.contains_key(&key) {
            return ErrorCode::WalletItemNotFound;
        }

        wallet.records.remove(&key);

        ErrorCode::Success
    }

    pub extern "C" fn get_storage_metadata(xhandle: i32, metadata_ptr: *mut *const c_char, metadata_handle: *mut i32) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

        let metadata = wallet.metadata.clone();
        let metadata = CString::new(metadata.as_str()).unwrap();
        let metadata_pointer = metadata.as_ptr();

        let handle = SequenceUtils::get_next_id();

        let mut metadatas = ACTIVE_METADATAS.lock().unwrap();
        metadatas.insert(handle, metadata);

        unsafe { *metadata_ptr = metadata_pointer; }
        unsafe { *metadata_handle = handle };

        ErrorCode::Success
    }

    pub extern "C" fn set_storage_metadata(xhandle: i32, metadata: *const c_char) -> ErrorCode {
        check_useful_c_str!(metadata, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        wallet.metadata = metadata;

        ErrorCode::Success
    }

    pub extern "C" fn free_storage_metadata(xhandle: i32, metadata_handler: i32) -> ErrorCode {
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
    }

    pub extern "C" fn search_records(xhandle: i32, type_: *const c_char, _query_json: *const c_char, _options_json: *const c_char, handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(type_, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

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
    }

    pub extern "C" fn search_all_records(xhandle: i32, handle: *mut i32) -> ErrorCode {
        let handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

        let search_records = wallet.records
            .values()
            .cloned()
            .collect::<Vec<InmemWalletRecord>>();

        let search_handle = SequenceUtils::get_next_id();

        let mut searches = ACTIVE_SEARCHES.lock().unwrap();

        searches.insert(search_handle, search_records);

        unsafe { *handle = search_handle };

        ErrorCode::Success
    }

    pub extern "C" fn get_search_total_count(xhandle: i32, search_handle: i32, count: *mut usize) -> ErrorCode {
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
    }

    pub extern "C" fn fetch_search_next_record(xhandle: i32, search_handle: i32, record_handle: *mut i32) -> ErrorCode {
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
    }

    pub extern "C" fn free_search(xhandle: i32, search_handle: i32) -> ErrorCode {
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
    }

    pub extern "C" fn close(xhandle: i32) -> ErrorCode {
        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        handles.remove(&xhandle);
        ErrorCode::Success
    }

    pub extern "C" fn delete(name: *const c_char,
                             _: *const c_char,
                             _: *const c_char) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&name) {
            return ErrorCode::CommonInvalidState;
        }

        wallets.remove(&name);
        ErrorCode::Success
    }

    pub fn cleanup() {
        let mut wallets = INMEM_WALLETS.lock().unwrap();
        wallets.clear();

        let mut handles = INMEM_OPEN_WALLETS.lock().unwrap();
        handles.clear();
    }
}