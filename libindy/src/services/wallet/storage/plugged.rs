extern crate libc;
extern crate indy_crypto;
extern crate serde_json;

use std::error::Error;
use std::ffi::{CString, CStr, NulError};
use std::ptr;
use std::str::Utf8Error;
use std::{slice, str};
use std::collections::HashMap;

use api::wallet::*;
use api::ErrorCode;
use errors::common::CommonError;
use errors::wallet::WalletStorageError;
use services::wallet::wallet::WalletRuntimeConfig;
use services::wallet::language;


use super::{StorageIterator, WalletStorageType, WalletStorage, StorageEntity, StorageValue, TagValue};

use self::libc::c_char;
use self::indy_crypto::utils::json::JsonDecodable;


#[derive(Debug, Deserialize)]
pub struct PluggedWalletJSONValue {
    pub key: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct PluggedWalletJSONValues {
    pub values: Vec<PluggedWalletJSONValue>
}

impl<'a> JsonDecodable<'a> for PluggedWalletJSONValues {}


struct PluggedStorage {
    handle: i32,
    add_record_handler: WalletAddRecord,
    update_record_value_handler: WalletUpdateRecordValue,
    update_record_tags_handler: WalletUpdateRecordTags,
    add_record_tags_handler: WalletAddRecordTags,
    delete_record_tags_handler: WalletDeleteRecordTags,
    delete_record_handler: WalletDeleteRecord,
    get_record_handler: WalletGetRecord,
    get_record_id_handler: WalletGetRecordId,
    get_record_type_handler: WalletGetRecordType,
    get_record_value_handler: WalletGetRecordValue,
    get_record_tags_handler: WalletGetRecordTags,
    free_record_handler: WalletFreeRecord,
    search_records_handler: WalletSearchRecords,
    search_all_records_handler: WalletSearchAllRecords,
    get_search_total_count_handler: WalletGetSearchTotalCount,
    fetch_search_next_record_handler: WalletFetchSearchNextRecord,
    free_search_handler: WalletFreeSearch,
    close_handler: WalletClose
}

impl PluggedStorage {
    fn new(name: &str,
           pool_name: &str,
           handle: i32,
           add_record_handler: WalletAddRecord,
           update_record_value_handler: WalletUpdateRecordValue,
           update_record_tags_handler: WalletUpdateRecordTags,
           add_record_tags_handler: WalletAddRecordTags,
           delete_record_tags_handler: WalletDeleteRecordTags,
           delete_record_handler: WalletDeleteRecord,
           get_record_handler: WalletGetRecord,
           get_record_id_handler: WalletGetRecordId,
           get_record_type_handler: WalletGetRecordType,
           get_record_value_handler: WalletGetRecordValue,
           get_record_tags_handler: WalletGetRecordTags,
           free_record_handler: WalletFreeRecord,
           search_records_handler: WalletSearchRecords,
           search_all_records_handler: WalletSearchAllRecords,
           get_search_total_count_handler: WalletGetSearchTotalCount,
           fetch_search_next_record_handler: WalletFetchSearchNextRecord,
           free_search_handler: WalletFreeSearch,
           close_handler: WalletClose) -> PluggedStorage {
        PluggedStorage {
            handle,
            add_record_handler,
            update_record_value_handler,
            update_record_tags_handler,
            add_record_tags_handler,
            delete_record_tags_handler,
            delete_record_handler,
            get_record_handler,
            get_record_id_handler,
            get_record_type_handler,
            get_record_value_handler,
            close_handler,
            search_records_handler,
            search_all_records_handler,
            get_search_total_count_handler,
            free_record_handler,
            free_search_handler,
            get_record_tags_handler,
            fetch_search_next_record_handler,
        }
    }
}

impl WalletStorage for PluggedStorage {
    fn add(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &Vec<u8>, value_key: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>)
        -> Result<(), WalletStorageError> {
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//
//        let value = value.as_bytes();
//        let value_p = value.as_ptr();
//        let value_len = value.len();
//
//        let tags_json = CString::new(tags_json)?;
//
//        let err = (self.add_record_handler)(self.handle,
//                                            type_.as_ptr(),
//                                            id.as_ptr(),
//                                            value_p,
//                                            value_len,
//                                            tags_json.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(())
        unimplemented!();
    }

//    fn update_record_value(&self, type_: &str, id: &str, value: &str) -> Result<(), WalletError> {
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//        let value = value.as_bytes();
//        let value_p = value.as_ptr();
//        let value_len = value.len();
//
//        let err = (self.update_record_value_handler)(self.handle,
//                                                     type_.as_ptr(),
//                                                     id.as_ptr(),
//                                                     value_p,
//                                                     value_len);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(())
//    }

//    fn add_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletStorageError> {
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//        let tags_json = CString::new(tags_json)?;
//
//        let err = (self.add_record_tags_handler)(self.handle,
//                                                 type_.as_ptr(),
//                                                 id.as_ptr(),
//                                                 tags_json.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(())
//    }

//    fn delete_record_tags(&self, type_: &str, id: &str, tag_names_json: &str) -> Result<(), WalletStorageError> {
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//        let tag_names_json = CString::new(tag_names_json)?;
//
//        let err = (self.delete_record_tags_handler)(self.handle,
//                                                    type_.as_ptr(),
//                                                    id.as_ptr(),
//                                                    tag_names_json.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(())
//    }

    fn delete(&self, type_: &Vec<u8>, name: &Vec<u8>) -> Result<(), WalletStorageError> {
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//
//        let err = (self.delete_record_handler)(self.handle,
//                                               type_.as_ptr(),
//                                               id.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(())
        unimplemented!();
    }

    fn get(&self, type_: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, WalletStorageError> {
//        let options: RecordOptions = RecordOptions::from_json(options_json)
//            .map_err(|err|
//                WalletError::CommonError(
//                    CommonError::InvalidStructure(format!("Cannot deserialize RecordRetrieveOptions: {:?}", err))))?;
//
//        let type_ = CString::new(type_)?;
//        let id = CString::new(id)?;
//        let options_json = CString::new(options_json)?;
//        let mut record_handle_p: u32 = 0;
//        let err = (self.get_record_handler)(self.handle,
//                                            type_.as_ptr(),
//                                            id.as_ptr(),
//                                            options_json.as_ptr(),
//                                            &mut record_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        let mut id_ptr: *const c_char = ptr::null_mut();
//        let err = (self.get_record_id_handler)(self.handle,
//                                               record_handle_p,
//                                               &mut id_ptr);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        let id = unsafe { CStr::from_ptr(id_ptr).to_str()?.to_string() };
//
//        let type_ = if let Some(true) = options.retrieve_type {
//            let mut type_ptr: *const c_char = ptr::null_mut();
//            let err = (self.get_record_type_handler)(self.handle,
//                                                     record_handle_p,
//                                                     &mut type_ptr);
//
//            if err != ErrorCode::Success {
//                return Err(WalletError::PluggedWallerError(err));
//            }
//
//            Some(unsafe { CStr::from_ptr(type_ptr).to_str()?.to_string() })
//        } else { None };
//
//        let value = if let Some(false) = options.retrieve_value {
//            None
//        } else {
//            let mut value_bytes: *const u8 = ptr::null();
//            let mut value_bytes_len: usize = 0;
//            let err = (self.get_record_value_handler)(self.handle,
//                                                      record_handle_p,
//                                                      &mut value_bytes,
//                                                      &mut value_bytes_len);
//
//            if err != ErrorCode::Success {
//                return Err(WalletError::PluggedWallerError(err));
//            }
//
//            let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
//            Some(str::from_utf8(&value).unwrap().to_string())
//        };
//
//        let tags = if let Some(false) = options.retrieve_tags {
//            None
//        } else {
//            let mut tags_ptr: *const c_char = ptr::null_mut();
//            let err = (self.get_record_tags_handler)(self.handle,
//                                                     record_handle_p,
//                                                     &mut tags_ptr);
//
//            if err != ErrorCode::Success {
//                return Err(WalletError::PluggedWallerError(err));
//            }
//
//            Some(unsafe { CStr::from_ptr(tags_ptr).to_str()?.to_string() })
//        };
//
//        let result = WalletRecord {
//            id,
//            type_,
//            value,
//            tags
//        };
//
//        let err = (self.free_record_handler)(self.handle, record_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(result)
        unimplemented!();
    }

    fn get_all<'a>(&'a self) -> Result<Box<StorageIterator + 'a>, WalletStorageError> {
        unimplemented!();
    }

    fn search<'a>(&'a self, type_: &Vec<u8>, query: &language::Operator, options: Option<&str>)
        -> Result<Box<StorageIterator + 'a>, WalletStorageError> {
//        let type_ = CString::new(type_)?;
//        let query_json = CString::new(query_json)?;
//        let options_json = CString::new(options_json)?;
//        let mut search_handle_p: u32 = 0;
//
//        let err = (self.search_records_handler)(self.handle,
//                                                type_.as_ptr(),
//                                                query_json.as_ptr(),
//                                                options_json.as_ptr(),
//                                                &mut search_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        //TODO:
//        let records: Vec<WalletRecord> = Vec::new();
//        let result = WalletSearch {
//            total_count: Some(0),
//            iter: Some(Box::new(records.into_iter())),
//        };
//
//        let err = (self.free_search_handler)(self.handle, search_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(result)
        unimplemented!();
    }

//    fn search_all_records(&self) -> Result<WalletSearch, WalletStorageError> {
//        let mut search_handle_p: u32 = 0;
//
//        let err = (self.search_all_records_handler)(self.handle,
//                                                    &mut search_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        //TODO:
//        let records: Vec<WalletRecord> = Vec::new();
//        let result = WalletSearch {
//            total_count: Some(0),
//            iter: Some(Box::new(records.into_iter())),
//        };
//
//        let err = (self.free_search_handler)(self.handle, search_handle_p);
//
//        if err != ErrorCode::Success {
//            return Err(WalletError::PluggedWallerError(err));
//        }
//
//        Ok(result)
//    }

    fn clear(&self) -> Result<(), WalletStorageError> {
        unimplemented!();
    }

    fn close(&mut self) -> Result<(), WalletStorageError> {
        unimplemented!();
    }
//    fn close_search(&self, search_handle: u32) -> Result<(), WalletStorageError> {
//        let err = (self.free_search_handler)(self.handle, search_handle);
//
//        if err != ErrorCode::Success {
//            return Err(WalletStorageError::PluggedStorageError(err));
//        }
//
//        Ok(())
//    }
}


pub struct PluggedStorageType {
    create_handler: WalletCreate,
    open_handler: WalletOpen,
    close_handler: WalletClose,
    delete_handler: WalletDelete,
    add_record_handler: WalletAddRecord,
    update_record_value_handler: WalletUpdateRecordValue,
    update_record_tags_handler: WalletUpdateRecordTags,
    add_record_tags_handler: WalletAddRecordTags,
    delete_record_tags_handler: WalletDeleteRecordTags,
    delete_record_handler: WalletDeleteRecord,
    get_record_handler: WalletGetRecord,
    get_record_id_handler: WalletGetRecordId,
    get_record_type_handler: WalletGetRecordType,
    get_record_value_handler: WalletGetRecordValue,
    get_record_tags_handler: WalletGetRecordTags,
    free_record_handler: WalletFreeRecord,
    search_records_handler: WalletSearchRecords,
    search_all_records_handler: WalletSearchAllRecords,
    get_search_total_count_handler: WalletGetSearchTotalCount,
    fetch_search_next_record_handler: WalletFetchSearchNextRecord,
    free_search_handler: WalletFreeSearch
}


impl PluggedStorageType {
    pub fn new(create_handler: WalletCreate,
               open_handler: WalletOpen,
               close_handler: WalletClose,
               delete_handler: WalletDelete,
               add_record_handler: WalletAddRecord,
               update_record_value_handler: WalletUpdateRecordValue,
               update_record_tags_handler: WalletUpdateRecordTags,
               add_record_tags_handler: WalletAddRecordTags,
               delete_record_tags_handler: WalletDeleteRecordTags,
               delete_record_handler: WalletDeleteRecord,
               get_record_handler: WalletGetRecord,
               get_record_id_handler: WalletGetRecordId,
               get_record_type_handler: WalletGetRecordType,
               get_record_value_handler: WalletGetRecordValue,
               get_record_tags_handler: WalletGetRecordTags,
               free_record_handler: WalletFreeRecord,
               search_records_handler: WalletSearchRecords,
               search_all_records_handler: WalletSearchAllRecords,
               get_search_total_count_handler: WalletGetSearchTotalCount,
               fetch_search_next_record_handler: WalletFetchSearchNextRecord,
               free_search_handler: WalletFreeSearch) -> PluggedStorageType {
        PluggedStorageType {
            create_handler,
            open_handler,
            close_handler,
            delete_handler,
            add_record_handler,
            update_record_value_handler,
            update_record_tags_handler,
            add_record_tags_handler,
            delete_record_tags_handler,
            delete_record_handler,
            get_record_handler,
            get_record_id_handler,
            get_record_type_handler,
            get_record_value_handler,
            get_record_tags_handler,
            free_record_handler,
            search_records_handler,
            search_all_records_handler,
            get_search_total_count_handler,
            fetch_search_next_record_handler,
            free_search_handler,
        }
    }
}

impl WalletStorageType for PluggedStorageType {
    fn create_storage(&self, name: &str, config: Option<&str>, credentials: &str, keys: &Vec<u8>) -> Result<(), WalletStorageError> {
//        let name = CString::new(name)?;
//
//        let config = match config {
//            Some(config) => Some(CString::new(config)?),
//            None => None
//        };
//
//        let credentials = CString::new(credentials)?;
//
//        let err = (self.create_handler)(name.as_ptr(),
//                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
//                                        credentials.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletStorageError::PluggedStorageError(err));
//        }
//
//        Ok(())
        unimplemented!()
    }

    fn delete_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(), WalletStorageError> {
//        let name = CString::new(name)?;
//
//        let config = match config {
//            Some(config) => Some(CString::new(config)?),
//            None => None
//        };
//
//        let credentials = CString::new(credentials)?;
//
//        let err = (self.delete_handler)(name.as_ptr(),
//                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
//                                        credentials.as_ptr());
//
//        if err != ErrorCode::Success {
//            return Err(WalletStorageError::PluggedStorageError(err));
//        }
//
//        Ok(())
        unimplemented!()
    }

    fn open_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(Box<WalletStorage>, Vec<u8>), WalletStorageError> {
//        let mut handle: i32 = 0;
//        let cname = CString::new(name)?;
//
//        let config = match config {
//            Some(config) => Some(CString::new(config)?),
//            None => None
//        };
//
//        let credentials = CString::new(credentials)?;
//
//        let err = (self.open_handler)(cname.as_ptr(),
//                                      config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
//                                      "".as_ptr(), // TODO!!!
//                                      credentials.as_ptr(),
//                                      &mut handle);
//
//        if err != ErrorCode::Success {
//            return Err(WalletStorageError::PluggedStorageError(err));
//        }
//
//        Ok(Box::new(
//            PluggedStorage::new(
//                handle,
//                self.add_record_handler,
//                self.update_record_value_handler,
//                self.update_record_tags_handler,
//                self.add_record_tags_handler,
//                self.delete_record_tags_handler,
//                self.delete_record_handler,
//                self.get_record_handler,
//                self.get_record_id_handler,
//                self.get_record_type_handler,
//                self.get_record_value_handler,
//                self.get_record_tags_handler,
//                self.free_record_handler,
//                self.search_records_handler,
//                self.search_all_records_handler,
//                self.get_search_total_count_handler,
//                self.fetch_search_next_record_handler,
//                self.free_search_handler,
//                self.close_handler)))
        unimplemented!()
    }
}

//
//impl From<NulError> for WalletError {
//    fn from(err: NulError) -> WalletError {
//        WalletError::CommonError(CommonError::InvalidState(format!("Null symbols in wallet keys or values: {}", err.description())))
//    }
//}
//
//impl From<Utf8Error> for WalletError {
//    fn from(err: Utf8Error) -> WalletError {
//        WalletError::CommonError(CommonError::InvalidState(format!("Incorrect utf8 symbols in wallet keys or values: {}", err.description())))
//    }
//}

//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use errors::wallet::WalletError;
//    use utils::inmem_wallet::InmemWallet;
//
//    use std::time::Duration;
//    use std::thread;
//
//    #[test]
//    fn plugged_wallet_type_new_works() {
//        InmemWallet::cleanup();
//
//        PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        InmemWallet::cleanup();
//    }
//
//
//    #[test]
//    fn plugged_wallet_type_create_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_type_create_works_for_twice() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        let res = wallet_type.create("wallet1", None, None);
//        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::CommonInvalidState)), res);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_type_delete_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        wallet_type.create("wallet1", None, None).unwrap();
//        wallet_type.delete("wallet1", None, None).unwrap();
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_type_open_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        wallet_type.create("wallet1", None, None).unwrap();
//        wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_set_get_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//
//        wallet_type.create("wallet1", None, None).unwrap();
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//
//        wallet.set("key1", "value1").unwrap();
//        let value = wallet.get("key1").unwrap();
//        assert_eq!("value1", value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_set_get_works_for_reopen() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        {
//            let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//            wallet.set("key1", "value1").unwrap();
//        }
//
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//        let value = wallet.get("key1").unwrap();
//        assert_eq!("value1", value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_get_works_for_unknown() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//        let value = wallet.get("key1");
//        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_set_get_works_for_update() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//
//        wallet.set("key1", "value1").unwrap();
//        let value = wallet.get("key1").unwrap();
//        assert_eq!("value1", value);
//
//        wallet.set("key1", "value2").unwrap();
//        let value = wallet.get("key1").unwrap();
//        assert_eq!("value2", value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_set_get_not_expired_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//        let wallet = wallet_type.open("wallet1", "pool1", None, Some("{\"freshness_time\": 1}"), None).unwrap();
//        wallet.set("key1", "value1").unwrap();
//
//        // Wait until value expires
//        thread::sleep(Duration::new(2, 0));
//
//        let value = wallet.get_not_expired("key1");
//        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_list_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//
//        wallet.set("key1::subkey1", "value1").unwrap();
//        wallet.set("key1::subkey2", "value2").unwrap();
//
//        let mut key_values = wallet.list("key1::").unwrap();
//        key_values.sort();
//        assert_eq!(2, key_values.len());
//
//        let (key, value) = key_values.pop().unwrap();
//        assert_eq!("key1::subkey2", key);
//        assert_eq!("value2", value);
//
//        let (key, value) = key_values.pop().unwrap();
//        assert_eq!("key1::subkey1", key);
//        assert_eq!("value1", value);
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_get_pool_name_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//        assert_eq!(wallet.get_pool_name(), "pool1");
//
//        InmemWallet::cleanup();
//    }
//
//    #[test]
//    fn plugged_wallet_get_name_works() {
//        InmemWallet::cleanup();
//
//        let wallet_type = PluggedWalletType::new(
//            InmemWallet::create,
//            InmemWallet::open,
//            InmemWallet::set,
//            InmemWallet::get,
//            InmemWallet::get_not_expired,
//            InmemWallet::list,
//            InmemWallet::close,
//            InmemWallet::delete,
//            InmemWallet::free
//        );
//        wallet_type.create("wallet1", None, None).unwrap();
//
//        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
//        assert_eq!(wallet.get_name(), "wallet1");
//
//        InmemWallet::cleanup();
//    }
//}