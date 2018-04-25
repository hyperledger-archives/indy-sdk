extern crate libc;
extern crate indy_crypto;

mod storage;
mod encryption;
mod query_encryption;
mod iterator;
mod language;
mod wallet;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;
use named_type::NamedType;

use serde_json;
use base64;

use api::wallet::*;
use errors::indy::IndyError;
use errors::wallet::WalletError;
use errors::common::CommonError;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;

use self::encryption::decrypt;
use self::storage::WalletStorageType;
use self::storage::default::SQLiteStorageType;
use self::storage::plugged::PluggedStorageType;
use self::wallet::{Wallet, Keys, Tags};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};


#[derive(Serialize, Deserialize)]
pub struct WalletDescriptor {
    pool_name: String,
    xtype: String,
    name: String
}

impl WalletDescriptor {
    pub fn new(pool_name: &str, xtype: &str, name: &str) -> WalletDescriptor {
        WalletDescriptor {
            pool_name: pool_name.to_string(),
            xtype: xtype.to_string(),
            name: name.to_string()
        }
    }
}

impl JsonEncodable for WalletDescriptor {}

impl<'a> JsonDecodable<'a> for WalletDescriptor {}


#[derive(Debug)]
pub struct WalletCredentials {
    master_key: [u8; 32],
    storage_credentials: String,
}


impl WalletCredentials {
    fn from_json(json: &str) -> Result<WalletCredentials, WalletError> {
        if let serde_json::Value::Object(m) = try!(serde_json::from_str(json)) {
            let master_key = if let Some(&serde_json::Value::String(ref master_key_encoded)) = m.get("master_key") {
                let decoded_vector = try!(base64::decode(&master_key_encoded));
                let mut master_key: [u8; 32] = [0; 32];
                master_key.clone_from_slice(&decoded_vector[0..32]);
                master_key
            } else {
                return Err(WalletError::InputError(String::from("Credentials missing 'master_key' field")));
            };

            let storage_credentials = if let Some(&serde_json::Value::Object(ref storage_credentials)) = m.get("storage_credentials") {
                serde_json::to_string(&storage_credentials).unwrap()
            } else {
                return Err(WalletError::InputError(String::from("Credentials missing 'storage_credentials' field")));
            };

            Ok(WalletCredentials {
                master_key: master_key,
                storage_credentials: storage_credentials
            })
        } else {
            return Err(WalletError::InputError(String::from("Credentials must be JSON object")));
        }
    }
}


pub struct WalletService {
    storage_types: RefCell<HashMap<String, Box<WalletStorageType>>>,
    wallets: RefCell<HashMap<i32, Box<Wallet>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let mut types: HashMap<String, Box<WalletStorageType>> = HashMap::new();
        types.insert("default".to_string(), Box::new(SQLiteStorageType::new()));

        WalletService {
            storage_types: RefCell::new(types),
            wallets: RefCell::new(HashMap::new())
        }
    }

    pub fn register_wallet_storage(&self,
                                   type_: &str,
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
                                   search_records: WalletSearchRecords,
                                   search_all_records: WalletSearchAllRecords,
                                   get_search_total_count: WalletGetSearchTotalCount,
                                   fetch_search_next_record: WalletFetchSearchNextRecord,
                                   free_search: WalletFreeSearch) -> Result<(), WalletError> {
        let mut storage_types = self.storage_types.borrow_mut();

        if storage_types.contains_key(type_) {
            return Err(WalletError::TypeAlreadyRegistered(type_.to_string()));
        }

        storage_types.insert(type_.to_string(),
                             Box::new(
                                  PluggedStorageType::new(create, open, close, delete,
                                                          add_record, update_record_value,
                                                          update_record_tags, add_record_tags, delete_record_tags,
                                                          delete_record, get_record, get_record_id,
                                                          get_record_type, get_record_value, get_record_tags,
                                                          free_record, search_records, search_all_records,
                                                          get_search_total_count,
                                                          fetch_search_next_record, free_search)));
        Ok(())
    }

    pub fn create_wallet(&self,
                         pool_name: &str,
                         name: &str,
                         storage_type: Option<&str>,
                         storage_config: Option<&str>,
                         credentials: &str) -> Result<(), WalletError> {
        let xtype = storage_type.unwrap_or("default");

        let storage_types = self.storage_types.borrow();
        if !storage_types.contains_key(xtype) {
            return Err(WalletError::UnknownType(xtype.to_string()));
        }

        let wallet_path = _wallet_path(name);
        if wallet_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()));
        }
        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        let storage_type = storage_types.get(xtype).unwrap();
        let credentials = WalletCredentials::from_json(credentials)?;
        storage_type.create_storage(name, storage_config, &credentials.storage_credentials, &Keys::gen_keys(credentials.master_key))?;

        let mut descriptor_file = File::create(_wallet_descriptor_path(name))?;
        descriptor_file
            .write_all({
                WalletDescriptor::new(pool_name, xtype, name)
                    .to_json()?
                    .as_bytes()
            })?;
        descriptor_file.sync_all()?;

        if storage_config.is_some() {
            let mut config_file = File::create(_wallet_config_path(name))?;
            config_file.write_all(storage_config.unwrap().as_bytes())?;
            config_file.sync_all()?;
        }

        Ok(())
    }

    pub fn delete_wallet(&self, name: &str, credentials: &str) -> Result<(), WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let storage_types = self.storage_types.borrow();
        if !storage_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }

        let storage_type = storage_types.get(descriptor.xtype.as_str()).unwrap();

        let config = {
            let config_path = _wallet_config_path(name);

            if config_path.exists() {
                let mut config_json = String::new();
                let mut file = File::open(config_path)?;
                file.read_to_string(&mut config_json)?;
                Some(config_json)
            } else {
                None
            }
        };

        let credentials = WalletCredentials::from_json(credentials)?;
        storage_type.delete_storage(name,
                                   config.as_ref().map(String::as_str),
                                   &credentials.storage_credentials  )?;

        fs::remove_dir_all(_wallet_path(name))?;
        Ok(())
    }

    pub fn open_wallet(&self, name: &str, runtime_config: Option<&str>, credentials: &str) -> Result<i32, WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let storage_types = self.storage_types.borrow();
        if !storage_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }
        let storage_type = storage_types.get(descriptor.xtype.as_str()).unwrap();

        let mut wallets = self.wallets.borrow_mut();
        if wallets.values().any(|ref wallet| wallet.get_name() == name) {
            return Err(WalletError::AlreadyOpened(name.to_string()));
        }

        let config = {
            let config_path = _wallet_config_path(name);

            if config_path.exists() {
                let mut config_json = String::new();
                let mut file = File::open(config_path)?;
                file.read_to_string(&mut config_json)?;
                Some(config_json)
            } else {
                None
            }
        };

        let credentials = WalletCredentials::from_json(credentials)?;
        let (storage, enc_keys) = storage_type.open_storage(name,
                                                            config.as_ref().map(String::as_str),
                                                            &credentials.storage_credentials)?;
        let key_vector = decrypt(&enc_keys, credentials.master_key)?;
        let keys = Keys::new(key_vector);
        let wallet = Wallet::new(name, &descriptor.pool_name, storage, keys);
        let wallet_handle = SequenceUtils::get_next_id();
        wallets.insert(wallet_handle, Box::new(wallet));
        Ok(wallet_handle)
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletDescriptor>, WalletError> {
        let mut descriptors = Vec::new();
        let wallet_home_path = EnvironmentUtils::wallet_home_path();

        for entry in fs::read_dir(wallet_home_path)? {
            let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue };
            if let Some(wallet_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                let mut descriptor_json = String::new();
                File::open(_wallet_descriptor_path(wallet_name)).ok()
                    .and_then(|mut f| f.read_to_string(&mut descriptor_json).ok())
                    .and_then(|_| WalletDescriptor::from_json(descriptor_json.as_str()).ok())
                    .map(|descriptor| descriptors.push(descriptor));
            }
        }

        Ok(descriptors)
    }

    pub fn close_wallet(&self, handle: i32) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().remove(&handle) {
            Some(mut wallet) => wallet.close(),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn add_record(&self, wallet_handle: i32, type_: &str, name: &str, value: &str, tags_json: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let tags: Tags = serde_json::from_str(tags_json)?;
                wallet.add(type_, name, value, &tags)
            },
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn add_indy_object<T>(&self, wallet_handle: i32, name: &str, object: &T, tags_json: &str) -> Result<String, IndyError> where T: JsonEncodable, T: NamedType {
        let type_ = T::short_type_name();
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let object_json = object.to_json()
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", type_, err)))?;
                self.add_record(wallet_handle, &self.add_prefix(type_), name, &object_json, tags_json)?;
                Ok(object_json)
            }
            None => Err(IndyError::WalletError(WalletError::InvalidHandle(wallet_handle.to_string())))
        }
    }

    pub fn update_record_value(&self, wallet_handle: i32, type_: &str, name: &str, value: &str) -> Result<(), WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.update_record_value(type_, name, value),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn update_indy_object<T>(&self, wallet_handle: i32, id: &str, object: &T) -> Result<String, IndyError> where T: JsonEncodable, T: NamedType {
//        let type_ = T::short_type_name();
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => {
//                let object_json = object.to_json()
//                    .map_err(map_err_trace!())
//                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", type_, err)))?;
//                wallet.update_record_value(&self.add_prefix(type_), id, &object_json)?;
//                Ok(object_json)
//            }
//            None => Err(IndyError::WalletError(WalletError::InvalidHandle(wallet_handle.to_string())))
//        }
        unimplemented!()
    }

    pub fn add_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tags_json: &str) -> Result<(), WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.add_record_tags(type_, id, tags_json),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn add_indy_record_tags<T>(&self, wallet_handle: i32, name: &str, tags_json: &str) -> Result<(), WalletError> where T: NamedType {
//        self.add_record_tags(wallet_handle, &self.add_prefix(T::short_type_name()), id, tags_json)
        unimplemented!()
    }

    pub fn update_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tags_json: &str) -> Result<(), WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.update_record_tags(type_, id, tags_json),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn update_indy_record_tags<T>(&self, wallet_handle: i32, name: &str, tags_json: &str) -> Result<(), WalletError> where T: NamedType {
//        self.update_record_tags(wallet_handle, &self.add_prefix(T::short_type_name()), id, tags_json)
        unimplemented!()
    }

    pub fn delete_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tag_names_json: &str) -> Result<(), WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.delete_record_tags(type_, id, tag_names_json),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn delete_record(&self, wallet_handle: i32, type_: &str, name: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.delete(type_, name),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn delete_indy_record<T>(&self, wallet_handle: i32, name: &str) -> Result<(), WalletError> where T: NamedType {
        self.delete_record(wallet_handle, &self.add_prefix(T::short_type_name()), name)
    }

    pub fn get_record(&self, wallet_handle: i32, type_: &str, name: &str, options_json: &str) -> Result<WalletRecord, WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.get(type_, name, options_json),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn get_indy_record<T>(&self, wallet_handle: i32, name: &str, options_json: &str) -> Result<WalletRecord, WalletError> where T: NamedType {
        self.get_record(wallet_handle, &self.add_prefix(T::short_type_name()), name, options_json)
    }

    // Dirty hack. json must live longer then result T
    pub fn get_indy_object<'a, T>(&self, wallet_handle: i32, name: &str, options_json: &str, json: &'a mut String) -> Result<T, IndyError> where T: JsonDecodable<'a>, T: NamedType {
        let type_ = T::short_type_name();

        let record: WalletRecord = match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.get(&self.add_prefix(type_), name, options_json),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }?;
        *json = record.get_value()
            .ok_or(CommonError::InvalidStructure(format!("{} not found for id: {:?}", type_, name)))?.to_string();

        T::from_json(json)
            .map_err(map_err_trace!())
            .map_err(|err|
                IndyError::CommonError(CommonError::InvalidState(format!("Cannot deserialize {:?}: {:?}", type_, err))))
    }

    pub fn search_records(&self, wallet_handle: i32, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.search_records(type_, query_json, options_json),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn search_indy_records<T>(&self, wallet_handle: i32, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> where T: NamedType {
//        self.search_records(wallet_handle, &self.add_prefix(T::short_type_name()), query_json, options_json)
        unimplemented!()
    }

    pub fn search_all_records(&self, wallet_handle: i32) -> Result<WalletSearch, WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.search_all_records(),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn close_search(&self, wallet_handle: i32, search_handle: u32) -> Result<(), WalletError> {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) => wallet.close_search(search_handle),
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub fn get_pool_name(&self, wallet_handle: i32) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => Ok(wallet.get_pool_name()),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn upsert_indy_object<'a, T>(&self, wallet_handle: i32, id: &str, object: &T) -> Result<(), IndyError> where T: JsonEncodable,
                                                                                                                     T: JsonDecodable<'a>,
                                                                                                                     T: NamedType {
//        if self.record_exists::<T>(wallet_handle, id)? {
//            self.update_indy_object::<T>(wallet_handle, id, object)?
//        } else {
//            self.add_indy_object::<T>(wallet_handle, id, object, "{}")?
//        };
//        Ok(())
        unimplemented!()
    }

    pub fn record_exists<T>(&self, wallet_handle: i32, id: &str) -> Result<bool, WalletError> where T: NamedType {
//        match self.wallets.borrow().get(&wallet_handle) {
//            Some(wallet) =>
//                match wallet.get(&self.add_prefix(T::short_type_name()), id, &RecordOptions::id()) {
//                    Ok(_) => Ok(true),
//                    Err(WalletError::NotFound(_)) => Ok(false),
//                    Err(err) => Err(err),
//                }
//            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
//        }
        unimplemented!()
    }

    pub const PREFIX: &'static str = "Indy::";

    fn add_prefix(&self, type_: &str) -> String {
        format!("{}{}", WalletService::PREFIX, type_)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletRecord {
    name: String,
    type_: Option<String>,
    value: Option<String>,
    tags: Option<String>
}

impl JsonEncodable for WalletRecord {}

impl<'a> JsonDecodable<'a> for WalletRecord {}

impl WalletRecord {
    pub fn new(name: String, type_: Option<String>, value: Option<String>, tags: Option<String>) -> WalletRecord {
        WalletRecord {
            name: name,
            type_: type_,
            value: value,
            tags: tags,
        }
    }
    pub fn get_id(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_type(&self) -> Option<&str> {
        self.type_.as_ref().map(|t|
            if t.starts_with(WalletService::PREFIX) {
                t[WalletService::PREFIX.len()..].as_ref()
            } else {
                t.as_str()
            }
        )
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_ref().map(String::as_str)
    }

    pub fn get_tags(&self) -> Option<&str> {
        self.tags.as_ref().map(String::as_str)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordOptions {
    retrieve_type: Option<bool>,
    retrieve_value: Option<bool>,
    retrieve_tags: Option<bool>
}

impl JsonEncodable for RecordOptions {}

impl<'a> JsonDecodable<'a> for RecordOptions {}

impl RecordOptions {
    pub fn id() -> String {
        let options = RecordOptions {
            retrieve_type: Some(false),
            retrieve_value: Some(false),
            retrieve_tags: Some(false)
        };

        options.to_json().unwrap()
    }

    pub fn id_value() -> String {
        let options = RecordOptions {
            retrieve_type: Some(false),
            retrieve_value: Some(true),
            retrieve_tags: Some(false)
        };

        options.to_json().unwrap()
    }

    pub fn full() -> String {
        let options = RecordOptions {
            retrieve_type: Some(true),
            retrieve_value: Some(true),
            retrieve_tags: Some(true)
        };

        options.to_json().unwrap()
    }
}

pub struct WalletSearch {
    // TODO
    total_count: Option<usize>,
    iter: Option<Box<Iterator<Item=WalletRecord>>>,
}

impl WalletSearch {
    pub fn get_total_count(&self) -> Result<Option<usize>, WalletError> {
        Ok(self.total_count)
    }

    pub fn fetch_next_record(&mut self) -> Result<Option<WalletRecord>, WalletError> {
        Ok(self.iter.as_mut().and_then(|i| i.next()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOptions {
    retrieve_records: Option<bool>,
    retrieve_total_count: Option<bool>,
    retrieve_type: Option<bool>,
    retrieve_value: Option<bool>,
    retrieve_tags: Option<bool>
}

impl SearchOptions {
    pub fn full() -> String {
        let options = SearchOptions {
            retrieve_records: Some(true),
            retrieve_total_count: Some(true),
            retrieve_type: Some(true),
            retrieve_value: Some(true),
            retrieve_tags: Some(true)
        };

        options.to_json().unwrap()
    }
}

impl JsonEncodable for SearchOptions {}

impl<'a> JsonDecodable<'a> for SearchOptions {}

fn _wallet_path(name: &str) -> PathBuf {
    EnvironmentUtils::wallet_path(name)
}

fn _wallet_descriptor_path(name: &str) -> PathBuf {
    _wallet_path(name).join("wallet.json")
}

fn _wallet_config_path(name: &str) -> PathBuf {
    _wallet_path(name).join("config.json")
}
//
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use api::ErrorCode;
//    use errors::wallet::WalletError;
//    use utils::inmem_wallet::InmemWallet;
//    use utils::test::TestUtils;
//
//    use std::time::Duration;
//    use std::thread;
//
//    #[test]
//    fn wallet_service_new_works() {
//        WalletService::new();
//    }
//
//    //    #[test]
//    //    fn wallet_service_register_type_works() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_create_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", Some("default"), None, r#"{"key":"key"}"#).unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_create_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_create_works_for_none_type() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_create_works_for_unknown_type() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        let res = wallet_service.create("pool1", "wallet1", Some("unknown"), None, r#"{"key":"key"}"#);
//        assert_match!(Err(WalletError::UnknownType(_)), res);
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_create_works_for_twice() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//
//        let res = wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#);
//        assert_match!(Err(WalletError::AlreadyExists(_)), res);
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_delete_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        wallet_service.delete("wallet1", r#"{"key":"key"}"#).unwrap();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_delete_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        wallet_service.delete("wallet1", None).unwrap();
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_open_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_open_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    //    #[test]
//    //    fn wallet_service_list_wallets_works() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //        let w1_meta = WalletDescriptor {
//    //            name: "w1".to_string(),
//    //            pool_name: "p1".to_string(),
//    //            xtype: "default".to_string(),
//    //        };
//    //        let w2_meta = WalletDescriptor {
//    //            name: "w2".to_string(),
//    //            pool_name: "p2".to_string(),
//    //            xtype: "inmem".to_string(),
//    //        };
//    //        let w3_meta = WalletDescriptor {
//    //            name: "w3".to_string(),
//    //            pool_name: "p1".to_string(),
//    //            xtype: "default".to_string(),
//    //        };
//    //        wallet_service.create(&w1_meta.associated_pool_name,
//    //                              &w1_meta.name,
//    //                              Some(&w1_meta.type_),
//    //                              None, None).unwrap();
//    //        wallet_service.create(&w2_meta.associated_pool_name,
//    //                              &w2_meta.name,
//    //                              Some(&w2_meta.type_),
//    //                              None, None).unwrap();
//    //        wallet_service.create(&w3_meta.associated_pool_name,
//    //                              &w3_meta.name,
//    //                              None,
//    //                              None, None).unwrap();
//    //
//    //        let wallets = wallet_service.list_wallets().unwrap();
//    //
//    //        assert!(wallets.contains(&w1_meta));
//    //        assert!(wallets.contains(&w2_meta));
//    //        assert!(wallets.contains(&w3_meta));
//    //
//    //        InmemWallet::cleanup();
//    //        TestUtils::cleanup_indy_home();
//    //    }
//
//    #[test]
//    fn wallet_service_close_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//        wallet_service.close(wallet_handle).unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_close_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //        wallet_service.close(wallet_handle).unwrap();
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_add_record_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type", "key1", "value1", "{}").unwrap();
//        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_get_record_works_for_id_only() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type", "key1", "value1", "{}").unwrap();
//        let record = wallet_service.get_record(wallet_handle, "type", "key1", &RecordOptions::id()).unwrap();
//        assert!(record.get_value().is_none());
//        assert!(record.get_type().is_none());
//        assert!(record.get_tags().is_none());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_get_record_works_for_id_value_only() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type", "key1", "value1", "{}").unwrap();
//        let record = wallet_service.get_record(wallet_handle, "type", "key1", &RecordOptions::id_value()).unwrap();
//        assert_eq!("value1", record.get_value().unwrap());
//        assert!(record.get_type().is_none());
//        assert!(record.get_tags().is_none());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_get_record_works_for_all_fields() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type", "key1", "value1", r#"{"1":"some"}"#).unwrap();
//        let record = wallet_service.get_record(wallet_handle, "type", "key1", &RecordOptions::full()).unwrap();
//        assert_eq!("type", record.get_type().unwrap());
//        assert_eq!("value1", record.get_value().unwrap());
//        assert_eq!(r#"{"1":"some"}"#, record.get_tags().unwrap());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_set_get_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
//    //        let value = wallet_service.get(wallet_handle, "key1").unwrap();
//    //        assert_eq!("value1", value);
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_add_get_works_for_reopen() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//        wallet_service.add_record(wallet_handle, "type", "key1", "value1", "{}").unwrap();
//        wallet_service.close(wallet_handle).unwrap();
//
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//        let record = wallet_service.get_record(wallet_handle, "type", "key1", &RecordOptions::id_value()).unwrap();
//        assert_eq!("value1", record.get_value().unwrap());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_get_works_for_unknown() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        let res = wallet_service.get_record(wallet_handle, "type", "key1", &RecordOptions::id_value());
//        assert_match!(Err(WalletError::NotFound(_)), res);
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_get_works_for_plugged_and_unknown() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        let res = wallet_service.get(wallet_handle, "key1");
//    //        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), res);
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    //    #[test]
//    //    fn wallet_service_set_get_works_for_update() {
//    //        TestUtils::cleanup_indy_home();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
//    //        let value = wallet_service.get(wallet_handle, "key1").unwrap();
//    //        assert_eq!("value1", value);
//    //
//    //        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
//    //        let value = wallet_service.get(wallet_handle, "key1").unwrap();
//    //        assert_eq!("value2", value);
//    //
//    //        TestUtils::cleanup_indy_home();
//    //    }
//
//    //    #[test]
//    //    fn wallet_service_set_get_works_for_plugged_and_update() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //        wallet_service.create("pool1", "wallet1", None, None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
//    //        let value = wallet_service.get(wallet_handle, "key1").unwrap();
//    //        assert_eq!("value1", value);
//    //
//    //        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
//    //        let value = wallet_service.get(wallet_handle, "key1").unwrap();
//    //        assert_eq!("value2", value);
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_search_records_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type1", "id1", "value1", "{}").unwrap();
//        wallet_service.add_record(wallet_handle, "type2", "id2", "value2", "{}").unwrap();
//
//        let mut search = wallet_service.search_records(wallet_handle, "type1", "{}", "{}").unwrap();
//        assert_eq!(1, search.get_total_count().unwrap().unwrap());
//
//        let record = search.fetch_next_record().unwrap().unwrap();
//        assert_eq!("id1", record.get_id());
//        assert_eq!("value1", record.get_value().unwrap());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    #[test]
//    fn wallet_service_search_all_records_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        wallet_service.create("pool1", "wallet1", None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open("wallet1", None, r#"{"key":"key"}"#).unwrap();
//
//        wallet_service.add_record(wallet_handle, "type1", "id1", "value1", "{}").unwrap();
//        wallet_service.add_record(wallet_handle, "type2", "id2", "value2", "{}").unwrap();
//
//        let mut search = wallet_service.search_all_records(wallet_handle).unwrap();
//        assert_eq!(2, search.get_total_count().unwrap().unwrap());
//
//        let record = search.fetch_next_record().unwrap().unwrap();
//        assert_eq!("value1", record.get_value().unwrap());
//
//        let record = search.fetch_next_record().unwrap().unwrap();
//        assert_eq!("value2", record.get_value().unwrap());
//
//        TestUtils::cleanup_indy_home();
//    }
//
//
//    //    #[test]
//    //    fn wallet_service_list_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();
//    //
//    //        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
//    //        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();
//    //
//    //        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
//    //        key_values.sort();
//    //        assert_eq!(2, key_values.len());
//    //
//    //        let (key, value) = key_values.pop().unwrap();
//    //        assert_eq!("key1::subkey2", key);
//    //        assert_eq!("value2", value);
//    //
//    //        let (key, value) = key_values.pop().unwrap();
//    //        assert_eq!("key1::subkey1", key);
//    //        assert_eq!("value1", value);
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_get_pool_name_works() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        let wallet_name = "wallet1";
//        let pool_name = "pool1";
//        wallet_service.create(pool_name, wallet_name, None, None, r#"{"key":"key"}"#).unwrap();
//        let wallet_handle = wallet_service.open(wallet_name, None, r#"{"key":"key"}"#).unwrap();
//
//        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), pool_name);
//
//        TestUtils::cleanup_indy_home();
//    }
//
//    //    #[test]
//    //    fn wallet_service_get_pool_name_works_for_plugged() {
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //
//    //        let wallet_service = WalletService::new();
//    //
//    //        wallet_service
//    //            .register_type(
//    //                "inmem",
//    //                InmemWallet::create,
//    //                InmemWallet::open,
//    //                InmemWallet::set,
//    //                InmemWallet::get,
//    //                InmemWallet::list,
//    //                InmemWallet::close,
//    //                InmemWallet::delete,
//    //                InmemWallet::free
//    //            )
//    //            .unwrap();
//    //
//    //        wallet_service.create("pool1", "wallet1", Some("inmem"), None, None).unwrap();
//    //        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
//    //
//    //        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), "pool1");
//    //
//    //        TestUtils::cleanup_indy_home();
//    //        InmemWallet::cleanup();
//    //    }
//
//    #[test]
//    fn wallet_service_get_pool_name_works_for_incorrect_wallet_handle() {
//        TestUtils::cleanup_indy_home();
//
//        let wallet_service = WalletService::new();
//        let wallet_name = "wallet1";
//        let pool_name = "pool1";
//        wallet_service.create(pool_name, wallet_name, None, None, r#"{"key":"key"}"#).unwrap();
//
//        let get_pool_name_res = wallet_service.get_pool_name(1);
//        assert_match!(Err(WalletError::InvalidHandle(_)), get_pool_name_res);
//
//        TestUtils::cleanup_indy_home();
//    }
//}