extern crate libc;
extern crate indy_crypto;

use super::{WalletStorage, WalletStorageType, WalletRecord, WalletSearch};
use super::callbacks::*;

use api::ErrorCode;
use errors::common::CommonError;
use errors::wallet::WalletError;

use self::libc::c_char;

use std::error::Error;
use std::ffi::{CString, CStr, NulError};
use std::ptr;
use std::str::Utf8Error;

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

struct PluggedWallet {
    name: String,
    pool_name: String,
    handle: i32,
    add_record_handler: WalletAddRecord,
    update_record_value_handler: WalletUpdateRecordValue,
    update_record_tags_handler: WalletUpdateRecordTags,
    add_record_tags_handler: WalletAddRecordTags,
    delete_record_tags_handler: WalletDeleteRecordTags,
    delete_record_handler: WalletDeleteRecord,
    get_record_handler: WalletGetRecord,
    get_record_id_handler: WalletGetRecordId,
    get_record_value_handler: WalletGetRecordValue,
    get_record_tags_handler: WalletGetRecordTags,
    free_record_handler: WalletFreeRecord,
    search_records_handler: WalletSearchRecords,
    get_search_total_count_handler: WalletGetSearchTotalCount,
    fetch_search_next_record_handler: WalletFetchSearchNextRecord,
    free_search_handler: WalletFreeSearch,
    close_handler: WalletClose
}

impl PluggedWallet {
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
           get_record_value_handler: WalletGetRecordValue,
           get_record_tags_handler: WalletGetRecordTags,
           free_record_handler: WalletFreeRecord,
           search_records_handler: WalletSearchRecords,
           get_search_total_count_handler: WalletGetSearchTotalCount,
           fetch_search_next_record_handler: WalletFetchSearchNextRecord,
           free_search_handler: WalletFreeSearch,
           close_handler: WalletClose) -> PluggedWallet {
        PluggedWallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            handle,
            add_record_handler,
            update_record_value_handler,
            update_record_tags_handler,
            add_record_tags_handler,
            delete_record_tags_handler,
            delete_record_handler,
            get_record_handler,
            get_record_id_handler,
            get_record_value_handler,
            close_handler,
            search_records_handler,
            get_search_total_count_handler,
            free_record_handler,
            free_search_handler,
            get_record_tags_handler,
            fetch_search_next_record_handler,
        }
    }
}

impl WalletStorage for PluggedWallet {
    fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn add_record(&self, type_: &str, id: &str, value: &str, tags_json: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;

        let value = value.as_bytes();
        let value_p = value.as_ptr();
        let value_len = value.len();

        let tags_json = CString::new(tags_json)?;

        let err = (self.add_record_handler)(self.handle,
                                            type_.as_ptr(),
                                            id.as_ptr(),
                                            value_p,
                                            value_len,
                                            tags_json.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn update_record_value(&self, type_: &str, id: &str, value: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;
        let value = value.as_bytes();
        let value_p = value.as_ptr();
        let value_len = value.len();

        let err = (self.update_record_value_handler)(self.handle,
                                                     type_.as_ptr(),
                                                     id.as_ptr(),
                                                     value_p,
                                                     value_len);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn update_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;
        let tags_json = CString::new(tags_json)?;

        let err = (self.update_record_tags_handler)(self.handle,
                                                    type_.as_ptr(),
                                                    id.as_ptr(),
                                                    tags_json.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn add_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;
        let tags_json = CString::new(tags_json)?;

        let err = (self.add_record_tags_handler)(self.handle,
                                                 type_.as_ptr(),
                                                 id.as_ptr(),
                                                 tags_json.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn delete_record_tags(&self, type_: &str, id: &str, tag_names_json: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;
        let tag_names_json = CString::new(tag_names_json)?;

        let err = (self.delete_record_tags_handler)(self.handle,
                                                    type_.as_ptr(),
                                                    id.as_ptr(),
                                                    tag_names_json.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn delete_record(&self, type_: &str, id: &str) -> Result<(), WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;

        let err = (self.delete_record_handler)(self.handle,
                                               type_.as_ptr(),
                                               id.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn get_record(&self, type_: &str, id: &str, options_json: &str) -> Result<WalletRecord, WalletError> {
        let type_ = CString::new(type_)?;
        let id = CString::new(id)?;
        let options_json = CString::new(options_json)?;
        let mut record_handle_p: u32 = 0;

        let err = (self.get_record_handler)(self.handle,
                                            type_.as_ptr(),
                                            id.as_ptr(),
                                            options_json.as_ptr(),
                                            &mut record_handle_p);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let mut id_ptr: *const c_char = ptr::null_mut();
        let err = (self.get_record_id_handler)(self.handle,
                                               record_handle_p,
                                               &mut id_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let id = unsafe { CStr::from_ptr(id_ptr).to_str()?.to_string() };

        let mut value_bytes: *const u8 = ptr::null();
        let mut value_bytes_len: usize = 0;
        let err = (self.get_record_value_handler)(self.handle,
                                                  record_handle_p,
                                                  &mut value_bytes,
                                                  &mut value_bytes_len);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        use std::{slice, str};

        let value = unsafe { slice::from_raw_parts(value_bytes, value_bytes_len) };
        let value = str::from_utf8(&value).unwrap().to_string();

        let mut tags_ptr: *const c_char = ptr::null_mut();
        let err = (self.get_record_tags_handler)(self.handle,
                                                 record_handle_p,
                                                 &mut tags_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let tags = unsafe { CStr::from_ptr(tags_ptr).to_str()?.to_string() };


        let result = WalletRecord {
            id,
            value,
            tags
        };

        let err = (self.free_record_handler)(self.handle, record_handle_p);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(result)
    }

    fn search_records(&self, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> {
        let type_ = CString::new(type_)?;
        let query_json = CString::new(query_json)?;
        let options_json = CString::new(options_json)?;
        let mut search_handle_p: u32 = 0;

        let err = (self.search_records_handler)(self.handle,
                                                type_.as_ptr(),
                                                query_json.as_ptr(),
                                                options_json.as_ptr(),
                                                &mut search_handle_p);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let result = WalletSearch {
            total_count: None
        };

        let err = (self.free_search_handler)(self.handle, search_handle_p);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(result)
    }

    fn close(&self) -> Result<(), WalletError> {
        let err = (self.close_handler)(self.handle);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }
}

pub struct PluggedWalletType {
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
    get_record_value_handler: WalletGetRecordValue,
    get_record_tags_handler: WalletGetRecordTags,
    free_record_handler: WalletFreeRecord,
    search_records_handler: WalletSearchRecords,
    get_search_total_count_handler: WalletGetSearchTotalCount,
    fetch_search_next_record_handler: WalletFetchSearchNextRecord,
    free_search_handler: WalletFreeSearch
}

impl PluggedWalletType {
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
               get_record_value_handler: WalletGetRecordValue,
               get_record_tags_handler: WalletGetRecordTags,
               free_record_handler: WalletFreeRecord,
               search_records_handler: WalletSearchRecords,
               get_search_total_count_handler: WalletGetSearchTotalCount,
               fetch_search_next_record_handler: WalletFetchSearchNextRecord,
               free_search_handler: WalletFreeSearch) -> PluggedWalletType {
        PluggedWalletType {
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
            get_record_value_handler,
            get_record_tags_handler,
            free_record_handler,
            search_records_handler,
            get_search_total_count_handler,
            fetch_search_next_record_handler,
            free_search_handler,
        }
    }
}

impl WalletStorageType for PluggedWalletType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError> {
        let name = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.create_handler)(name.as_ptr(),
                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                        credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn delete(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError> {
        let name = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.delete_handler)(name.as_ptr(),
                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                        credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn open(&self, name: &str, pool_name: &str, config: Option<&str>, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<Box<WalletStorage>, WalletError> {
        let mut handle: i32 = 0;
        let cname = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let runtime_config = match runtime_config {
            Some(runtime_config) => Some(CString::new(runtime_config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.open_handler)(cname.as_ptr(),
                                      config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      runtime_config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      &mut handle);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(Box::new(
            PluggedWallet::new(
                name,
                pool_name,
                handle,
                self.add_record_handler,
                self.update_record_value_handler,
                self.update_record_tags_handler,
                self.add_record_tags_handler,
                self.delete_record_tags_handler,
                self.delete_record_handler,
                self.get_record_handler,
                self.get_record_id_handler,
                self.get_record_value_handler,
                self.get_record_tags_handler,
                self.free_record_handler,
                self.search_records_handler,
                self.get_search_total_count_handler,
                self.fetch_search_next_record_handler,
                self.free_search_handler,
                self.close_handler)))
    }
}


impl From<NulError> for WalletError {
    fn from(err: NulError) -> WalletError {
        WalletError::CommonError(CommonError::InvalidState(format!("Null symbols in wallet keys or values: {}", err.description())))
    }
}

impl From<Utf8Error> for WalletError {
    fn from(err: Utf8Error) -> WalletError {
        WalletError::CommonError(CommonError::InvalidState(format!("Incorrect utf8 symbols in wallet keys or values: {}", err.description())))
    }
}

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