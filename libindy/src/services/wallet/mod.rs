extern crate libc;
extern crate indy_crypto;

mod default;
mod plugged;

use self::default::DefaultWalletType;
use self::plugged::PluggedWalletType;

use errors::indy::IndyError;
use errors::wallet::WalletError;
use errors::common::CommonError;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use api::wallet::*;


pub trait WalletStorage {
    fn add_record(&self, type_: &str, id: &str, value: &str, tags_json: &str) -> Result<(), WalletError>;
    fn update_record_value(&self, type_: &str, id: &str, value: &str) -> Result<(), WalletError>;
    fn update_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError>;
    fn add_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError>;
    fn delete_record_tags(&self, type_: &str, id: &str, tag_names_json: &str) -> Result<(), WalletError>;
    fn delete_record(&self, type_: &str, id: &str) -> Result<(), WalletError>;
    fn get_record(&self, type_: &str, id: &str, options_json: &str) -> Result<WalletRecord, WalletError>;
    fn search_records(&self, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError>;
    fn search_all_records(&self) -> Result<WalletSearch, WalletError>;
    fn close(&self) -> Result<(), WalletError>;
    fn get_pool_name(&self) -> String;
    fn get_name(&self) -> String;
}

trait WalletStorageType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError>;
    fn delete(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError>;
    fn open(&self, name: &str, pool_name: &str, config: Option<&str>, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<Box<WalletStorage>, WalletError>;
}

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

pub struct WalletService {
    types: RefCell<HashMap<String, Box<WalletStorageType>>>,
    wallets: RefCell<HashMap<i32, Box<WalletStorage>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let mut types: HashMap<String, Box<WalletStorageType>> = HashMap::new();
        types.insert("default".to_string(), Box::new(DefaultWalletType::new()));

        WalletService {
            types: RefCell::new(types),
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
        let mut wallet_types = self.types.borrow_mut();

        if wallet_types.contains_key(type_) {
            return Err(WalletError::TypeAlreadyRegistered(type_.to_string()));
        }

        wallet_types.insert(type_.to_string(),
                            Box::new(
                                PluggedWalletType::new(create, open, close, delete,
                                                       add_record, update_record_value,
                                                       update_record_tags, add_record_tags, delete_record_tags,
                                                       delete_record, get_record, get_record_id,
                                                       get_record_type, get_record_value, get_record_tags,
                                                       free_record, search_records, search_all_records,
                                                       get_search_total_count,
                                                       fetch_search_next_record, free_search)));
        Ok(())
    }

    pub fn create(&self,
                  pool_name: &str,
                  name: &str,
                  storage_type: Option<&str>,
                  storage_config: Option<&str>,
                  credentials: Option<&str>) -> Result<(), WalletError> {
        let xtype = storage_type.unwrap_or("default");

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(xtype) {
            return Err(WalletError::UnknownType(xtype.to_string()));
        }

        let wallet_path = _wallet_path(name);
        if wallet_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()));
        }
        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        let wallet_type = wallet_types.get(xtype).unwrap();
        wallet_type.create(name, storage_config, credentials)?;

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

    pub fn delete(&self, name: &str, credentials: Option<&str>) -> Result<(), WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }

        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();

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

        wallet_type.delete(name,
                           config.as_ref().map(String::as_str),
                           credentials)?;

        fs::remove_dir_all(_wallet_path(name))?;
        Ok(())
    }

    pub fn open(&self, name: &str, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<i32, WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }
        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();

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

        let wallet = wallet_type.open(name,
                                      descriptor.pool_name.as_str(),
                                      config.as_ref().map(String::as_str),
                                      runtime_config,
                                      credentials)?;

        let wallet_handle = SequenceUtils::get_next_id();
        wallets.insert(wallet_handle, wallet);
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

    pub fn close(&self, handle: i32) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().remove(&handle) {
            Some(wallet) => wallet.close(),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn add_record(&self, storage_handle: i32, type_: &str, id: &str, value: &str, tags_json: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.add_record(type_, id, value, tags_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn update_record_value(&self, storage_handle: i32, type_: &str, id: &str, value: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.update_record_value(type_, id, value),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn update_object<T>(&self, storage_handle: i32, type_: &str, id: &str, object: &T) -> Result<String, IndyError> where T: JsonEncodable {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => {
                let object_json = object.to_json()
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", type_, err)))?;
                wallet.update_record_value(type_, id, &object_json)?;
                Ok(object_json)
            }
            None => Err(IndyError::WalletError(WalletError::InvalidHandle(storage_handle.to_string())))
        }
    }

    pub fn add_record_tags(&self, storage_handle: i32, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.add_record_tags(type_, id, tags_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn update_record_tags(&self, storage_handle: i32, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.update_record_tags(type_, id, tags_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn delete_record_tags(&self, storage_handle: i32, type_: &str, id: &str, tag_names_json: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.delete_record_tags(type_, id, tag_names_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn delete_record(&self, storage_handle: i32, type_: &str, id: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.delete_record(type_, id),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn set_object<T>(&self, storage_handle: i32, _type: &str, id: &str, object: &T, tags_json: &str) -> Result<String, IndyError> where T: JsonEncodable {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => {
                let object_json = object.to_json()
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", _type, err)))?;
                wallet.add_record(_type, id, &object_json, tags_json)?;
                Ok(object_json)
            }
            None => Err(IndyError::WalletError(WalletError::InvalidHandle(storage_handle.to_string())))
        }
    }

    pub fn get_record(&self, storage_handle: i32, type_: &str, id: &str, options_json: &str) -> Result<WalletRecord, WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.get_record(type_, id, options_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    // Dirty hack. json must live longer then result T
    pub fn get_object<'a, T>(&self, handle: i32, type_: &str, id: &str, options_json: &str, json: &'a mut String) -> Result<T, IndyError> where T: JsonDecodable<'a> {
        let record: WalletRecord = match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.get_record(type_, id, options_json),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }?;
        *json = record.get_value()?.to_string();

        T::from_json(json)
            .map_err(map_err_trace!())
            .map_err(|err|
                IndyError::CommonError(CommonError::InvalidState(format!("Cannot deserialize {:?}: {:?}", type_, err))))
    }

    pub fn search_records(&self, storage_handle: i32, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.search_records(type_, query_json, options_json),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn search_all_records(&self, storage_handle: i32) -> Result<WalletSearch, WalletError> {
        match self.wallets.borrow().get(&storage_handle) {
            Some(wallet) => wallet.search_all_records(),
            None => Err(WalletError::InvalidHandle(storage_handle.to_string()))
        }
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => Ok(wallet.get_pool_name()),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletRecord {
    pub id: String,
    pub type_: String,
    pub value: String,
    pub tags: String
}

impl JsonEncodable for WalletRecord {}

impl<'a> JsonDecodable<'a> for WalletRecord {}

impl WalletRecord {
    pub fn get_id(&self) -> Result<&str, WalletError> {
        Ok(self.id.as_str())
    }

    pub fn get_type(&self) -> Result<&str, WalletError> { Ok(self.type_.as_str()) }

    pub fn get_value(&self) -> Result<&str, WalletError> {
        Ok(self.value.as_str())
    }

    pub fn get_tags(&self) -> Result<&str, WalletError> {
        Ok(self.tags.as_str())
    }
}

pub struct WalletRecordRetrieveOptions {}

impl WalletRecordRetrieveOptions {
    pub const RETRIEVE_ID: &'static str = r#"{"retrieveValue": false, "retrieveTags":false}"#;
    pub const RETRIEVE_ID_VALUE: &'static str = r#"{"retrieveTags":false}"#;
    pub const RETRIEVE_ID_TAGS: &'static str = r#"{"retrieveValue":false}"#;
    pub const RETRIEVE_ID_VALUE_TAGS: &'static str = r#"{}"#;
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

fn _wallet_path(name: &str) -> PathBuf {
    EnvironmentUtils::wallet_path(name)
}

fn _wallet_descriptor_path(name: &str) -> PathBuf {
    _wallet_path(name).join("wallet.json")
}

fn _wallet_config_path(name: &str) -> PathBuf {
    _wallet_path(name).join("config.json")
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;

    use std::time::Duration;
    use std::thread;

    #[test]
    fn wallet_service_new_works() {
        WalletService::new();
    }

    #[test]
    fn wallet_service_register_type_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_create_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_create_works_for_none_type() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_unknown_type() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let res = wallet_service.create("pool1", Some("unknown"), "wallet1", None, None);
        assert_match!(Err(WalletError::UnknownType(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_twice() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let res = wallet_service.create("pool1", None, "wallet1", None, None);
        assert_match!(Err(WalletError::AlreadyExists(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_delete_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.delete("wallet1", None).unwrap();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_delete_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        wallet_service.delete("wallet1", None).unwrap();
        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_open_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_open_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        wallet_service.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_list_wallets_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();
        let w1_meta = WalletDescriptor {
            name: "w1".to_string(),
            pool_name: "p1".to_string(),
            xtype: "default".to_string(),
        };
        let w2_meta = WalletDescriptor {
            name: "w2".to_string(),
            pool_name: "p2".to_string(),
            xtype: "inmem".to_string(),
        };
        let w3_meta = WalletDescriptor {
            name: "w3".to_string(),
            pool_name: "p1".to_string(),
            xtype: "default".to_string(),
        };
        wallet_service.create(&w1_meta.associated_pool_name,
                              Some(&w1_meta.type_),
                              &w1_meta.name,
                              None, None).unwrap();
        wallet_service.create(&w2_meta.associated_pool_name,
                              Some(&w2_meta.type_),
                              &w2_meta.name,
                              None, None).unwrap();
        wallet_service.create(&w3_meta.associated_pool_name,
                              None,
                              &w3_meta.name,
                              None, None).unwrap();

        let wallets = wallet_service.list_wallets().unwrap();

        assert!(wallets.contains(&w1_meta));
        assert!(wallets.contains(&w2_meta));
        assert!(wallets.contains(&w3_meta));

        InmemWallet::cleanup();
        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_close_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.close(wallet_handle).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_close_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.close(wallet_handle).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works_for_reopen() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        wallet_service.close(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_works_for_unknown() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let res = wallet_service.get(wallet_handle, "key1");
        assert_match!(Err(WalletError::NotFound(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_works_for_plugged_and_unknown() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let res = wallet_service.get(wallet_handle, "key1");
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), res);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works_for_update() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_works_for_plugged_and_update() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    #[ignore] //TODO: recover it
    fn wallet_service_list_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();

        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();

        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
        key_values.sort();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    #[ignore] //TODO: recover it
    fn wallet_service_list_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();

        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();

        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
        key_values.sort();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_get_pool_name_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();
        let wallet_handle = wallet_service.open(wallet_name, None, None).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), pool_name);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), "pool1");

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_incorrect_wallet_handle() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();

        let get_pool_name_res = wallet_service.get_pool_name(1);
        assert_match!(Err(WalletError::InvalidHandle(_)), get_pool_name_res);

        TestUtils::cleanup_indy_home();
    }
}