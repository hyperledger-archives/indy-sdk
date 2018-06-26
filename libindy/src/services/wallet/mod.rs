extern crate indy_crypto;
extern crate sodiumoxide;

mod storage;
mod encryption;
mod query_encryption;
mod iterator;
mod language;
mod export_import;
mod wallet;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{OpenOptions, File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;
use named_type::NamedType;
use std::rc::Rc;

use serde_json;

use api::wallet::*;
use errors::wallet::WalletError;
use errors::common::CommonError;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;
use utils::crypto::chacha20poly1305_ietf::ChaCha20Poly1305IETFKey;

use self::export_import::{export, import};
use self::storage::{WalletStorage, WalletStorageType};
use self::storage::default::SQLiteStorageType;
use self::storage::plugged::PluggedStorageType;
use self::wallet::{Wallet, Keys};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use self::encryption::derive_key;
use utils::crypto::pwhash_argon2i13::PwhashArgon2i13;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WalletDescriptor {
    pool_name: String,
    #[serde(rename = "type")]
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

#[derive(Deserialize, Debug)]
pub struct WalletConfig {
    salt: [u8; PwhashArgon2i13::SALTBYTES]
}

impl<'a> JsonDecodable<'a> for WalletConfig {}


pub struct WalletCredentials {
    master_key: ChaCha20Poly1305IETFKey,
    rekey: Option<ChaCha20Poly1305IETFKey>,
    storage_credentials: String,
}


impl WalletCredentials {
    fn parse(json: &str, salt: &[u8; PwhashArgon2i13::SALTBYTES]) -> Result<WalletCredentials, WalletError> {
        if let serde_json::Value::Object(m) = serde_json::from_str(json)? {
            let master_key = if let Some(key) = m.get("key").and_then(|s| s.as_str()) {
                derive_key(key.as_bytes(), salt)?
            } else {
                return Err(WalletError::InputError(String::from("Credentials missing 'key' field")));
            };

            let rekey = if let Some(key) = m.get("rekey").and_then(|s| s.as_str()) {
                Some(derive_key(key.as_bytes(), salt)?)
            } else {
                None
            };

            let storage_credentials = serde_json::to_string(
                &m.get("storage_credentials")
                    .and_then(|storage_credentials| storage_credentials.as_object())
                    .unwrap_or(&serde_json::map::Map::new())
            )?;

            Ok(WalletCredentials {
                master_key,
                rekey,
                storage_credentials
            })
        } else {
            return Err(WalletError::InputError(String::from("Credentials must be JSON object")));
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct WalletExportConfig {
    key: String,
    path: String,
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
                                   get_storage_metadata: WalletGetStorageMetadata,
                                   set_storage_metadata: WalletSetStorageMetadata,
                                   free_storage_metadata: WalletFreeStorageMetadata,
                                   search_records: WalletSearchRecords,
                                   search_all_records: WalletSearchAllRecords,
                                   get_search_total_count: WalletGetSearchTotalCount,
                                   fetch_search_next_record: WalletFetchSearchNextRecord,
                                   free_search: WalletFreeSearch) -> Result<(), WalletError> {
        trace!("register_wallet_storage >>> type_: {:?}", type_);

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
                                                         get_record_type, get_record_value, get_record_tags, free_record,
                                                         get_storage_metadata, set_storage_metadata, free_storage_metadata,
                                                         search_records, search_all_records,
                                                         get_search_total_count,
                                                         fetch_search_next_record, free_search)));

        trace!("register_wallet_storage <<<");

        Ok(())
    }

    pub fn create_wallet(&self,
                         pool_name: &str,
                         name: &str,
                         storage_type: Option<&str>,
                         storage_config: Option<&str>,
                         credentials: &str) -> Result<(), WalletError> {
        trace!("create_wallet >>> pool_name: {:?}, name: {:?}, storage_type: {:?}, storage_config: {:?}, credentials: {:?}",
               pool_name, name, storage_type, storage_config, credentials);

        let xtype = storage_type.unwrap_or("default");

        let storage_types = self.storage_types.borrow();
        let storage_type = match storage_types.get(xtype) {
            None => return Err(WalletError::UnknownType(xtype.to_string())),
            Some(storage_type) => storage_type,
        };

        let wallet_path = _wallet_path(name);
        let wallet_descriptor_path = _wallet_descriptor_path(name);
        if wallet_path.exists() && wallet_descriptor_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()));
        }

        let mut config = match storage_config {
            Some(config) => serde_json::from_str::<serde_json::Value>(config)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize storage config: {:?}", err)))?,
            None => serde_json::Value::Object(serde_json::map::Map::new())
        };

        let salt = PwhashArgon2i13::gen_salt();
        config["salt"] = serde_json::Value::from(salt.to_vec());

        let config_json = serde_json::to_string(&config)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize storage config: {:?}", err)))?;

        let credentials = WalletCredentials::parse(credentials, &salt)?;

        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        storage_type.create_storage(name, storage_config, &credentials.storage_credentials, &Keys::gen_keys(&credentials.master_key))?;

        let wallet_descriptor_json = WalletDescriptor::new(pool_name, xtype, name).to_json()?;

        _write_file(wallet_descriptor_path, &wallet_descriptor_json)?;
        _write_file(_wallet_config_path(name), &config_json)?;

        trace!("create_wallet <<<");

        Ok(())
    }

    fn decrypt_keys(storage: &WalletStorage, credentials: &WalletCredentials) -> Result<Vec<u8>, WalletError> {
        let storage_metadata = storage.get_storage_metadata()?;

        let keys_vector = self::encryption::decrypt_merged(&storage_metadata, &credentials.master_key)
            .map_err(|_| WalletError::AccessFailed("Invalid master key provided".to_string()))?;

        Ok(keys_vector)
    }

    pub fn delete_wallet(&self, name: &str, credentials: &str) -> Result<(), WalletError> {
        trace!("delete_wallet >>> name: {:?}, credentials: {:?}", name, credentials);

        let wallets = self.wallets.borrow();
        if wallets.values().any(|ref wallet| wallet.get_name() == name) {
            return Err(WalletError::CommonError(CommonError::InvalidState("Wallet has to be closed before deleting".to_string())));
        }

        let wallet_descriptor_path = _wallet_descriptor_path(name);
        if !wallet_descriptor_path.exists() {
            return Err(WalletError::NotFound("Wallet descriptor path does not exist.".to_string()));
        }
        let descriptor_json = _read_file(wallet_descriptor_path)?;
        let descriptor: WalletDescriptor = WalletDescriptor::from_json(&descriptor_json)?;

        let storage_types = self.storage_types.borrow();
        let storage_type = match storage_types.get(descriptor.xtype.as_str()) {
            None => return Err(WalletError::UnknownType(descriptor.xtype)),
            Some(storage_type) => storage_type
        };

        let (config_json, config) = _read_config(name)?;

        let credentials = WalletCredentials::parse(credentials, &config.salt)?;

        {
            // check credentials and close connection before deleting directory
            let storage = storage_type.open_storage(name, Some(&config_json), &credentials.storage_credentials)?;

            WalletService::decrypt_keys(storage.as_ref(), &credentials)?;
        }

        storage_type.delete_storage(name, Some(&config_json), &credentials.storage_credentials)?;

        fs::remove_dir_all(_wallet_path(name))?;

        trace!("delete_wallet <<<");

        Ok(())
    }

    pub fn open_wallet(&self, name: &str, runtime_config: Option<&str>, credentials: &str) -> Result<i32, WalletError> {
        trace!("open_wallet >>> name: {:?}, runtime_config: {:?}, credentials: {:?}", name, runtime_config, credentials);

        let wallet_descriptor_path = _wallet_descriptor_path(name);
        if !wallet_descriptor_path.exists() {
            return Err(WalletError::NotFound("Wallet descriptor path does not exist.".to_string()));
        }
        let descriptor_json = _read_file(wallet_descriptor_path)?;
        let descriptor: WalletDescriptor = WalletDescriptor::from_json(&descriptor_json)?;

        let storage_types = self.storage_types.borrow();
        let storage_type = match storage_types.get(descriptor.xtype.as_str()) {
            None => return Err(WalletError::UnknownType(descriptor.xtype)),
            Some(storage_type) => storage_type,
        };

        let mut wallets = self.wallets.borrow_mut();
        if wallets.values().any(|ref wallet| wallet.get_name() == name) {
            return Err(WalletError::AlreadyOpened(name.to_string()));
        }

        let (config_json, config) = _read_config(name)?;

        let credentials = WalletCredentials::parse(credentials, &config.salt)?;

        let storage = storage_type.open_storage(name, Some(&config_json), &credentials.storage_credentials)?;

        let keys_vector = WalletService::decrypt_keys(storage.as_ref(), &credentials)?;
        let keys = Keys::new(keys_vector)?;

        let wallet = Wallet::new(name, &descriptor.pool_name, storage, Rc::new(keys));
        if let Some(ref rekey) = credentials.rekey {
            wallet.rotate_key(rekey)?;
        }
        let wallet_handle = SequenceUtils::get_next_id();
        wallets.insert(wallet_handle, Box::new(wallet));

        trace!("open_wallet <<< wallet_handle: {:?}", wallet_handle);
        Ok(wallet_handle)
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletDescriptor>, WalletError> {
        trace!("list_wallets >>>");

        let mut descriptors = Vec::new();
        let wallet_home_path = EnvironmentUtils::wallet_home_path();

        if let Ok(entries) = fs::read_dir(wallet_home_path) {
            for entry in entries {
                let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue };
                if let Some(wallet_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                    let mut descriptor_json = String::new();
                    File::open(_wallet_descriptor_path(wallet_name)).ok()
                        .and_then(|mut f| f.read_to_string(&mut descriptor_json).ok())
                        .and_then(|_| WalletDescriptor::from_json(descriptor_json.as_str()).ok())
                        .map(|descriptor| descriptors.push(descriptor));
                }
            }
        }

        trace!("list_wallets <<< descriptors: {:?}", descriptors);

        Ok(descriptors)
    }

    pub fn close_wallet(&self, handle: i32) -> Result<(), WalletError> {
        trace!("close_wallet >>> handle: {:?}", handle);

        match self.wallets.borrow_mut().remove(&handle) {
            Some(mut wallet) => wallet.close(),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }?;

        trace!("close_wallet <<<");

        Ok(())
    }

    pub fn add_record(&self, wallet_handle: i32, type_: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.add(type_, name, value, tags),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn add_indy_object<T>(&self, wallet_handle: i32, name: &str, object: &T, tags: &HashMap<String, String>)
                              -> Result<String, WalletError> where T: JsonEncodable, T: NamedType {
        let type_ = T::short_type_name();
        let object_json = object.to_json()
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", type_, err)))?;
        self.add_record(wallet_handle, &self.add_prefix(type_), name, &object_json, tags)?;
        Ok(object_json)
    }

    pub fn update_record_value(&self, wallet_handle: i32, type_: &str, name: &str, value: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.update(type_, name, value),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn update_indy_object<T>(&self, wallet_handle: i32, name: &str, object: &T) -> Result<String, WalletError> where T: JsonEncodable, T: NamedType {
        let type_ = T::short_type_name();
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let object_json = object.to_json()
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize {:?}: {:?}", type_, err)))?;
                wallet.update(&self.add_prefix(type_), name, &object_json)?;
                Ok(object_json)
            }
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn add_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.add_tags(type_, name, tags),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn add_indy_record_tags<T>(&self, wallet_handle: i32, name: &str, tags: &HashMap<String, String>)
                                   -> Result<(), WalletError> where T: NamedType {
        self.add_record_tags(wallet_handle, &self.add_prefix(T::short_type_name()), name, tags)
    }

    pub fn update_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.update_tags(type_, name, tags),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn update_indy_record_tags<T>(&self, wallet_handle: i32, name: &str, tags: &HashMap<String, String>)
                                      -> Result<(), WalletError> where T: NamedType {
        self.update_record_tags(wallet_handle, &self.add_prefix(T::short_type_name()), name, tags)
    }

    pub fn delete_record_tags(&self, wallet_handle: i32, type_: &str, name: &str, tag_names: &[&str]) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.delete_tags(type_, name, tag_names),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
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
    pub fn get_indy_object<'a, T>(&self, wallet_handle: i32, name: &str, options_json: &str, json: &'a mut String) -> Result<T, WalletError> where T: JsonDecodable<'a>, T: NamedType {
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
                WalletError::CommonError(CommonError::InvalidState(format!("Cannot deserialize {:?}: {:?}", type_, err))))
    }

    pub fn search_records(&self, wallet_handle: i32, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => Ok(WalletSearch { iter: wallet.search(type_, query_json, Some(options_json))? }),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn search_indy_records<T>(&self, wallet_handle: i32, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> where T: NamedType {
        self.search_records(wallet_handle, &self.add_prefix(T::short_type_name()), query_json, options_json)
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

    pub fn upsert_indy_object<'a, T>(&self, wallet_handle: i32, name: &str, object: &T) -> Result<(), WalletError>
        where T: JsonEncodable, T: JsonDecodable<'a>, T: NamedType {
        if self.record_exists::<T>(wallet_handle, name)? {
            self.update_indy_object::<T>(wallet_handle, name, object)?
        } else {
            self.add_indy_object::<T>(wallet_handle, name, object, &HashMap::new())?
        };
        Ok(())
    }

    pub fn record_exists<T>(&self, wallet_handle: i32, name: &str) -> Result<bool, WalletError> where T: NamedType {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) =>
                match wallet.get(&self.add_prefix(T::short_type_name()), name, &RecordOptions::id()) {
                    Ok(_) => Ok(true),
                    Err(WalletError::ItemNotFound) => Ok(false),
                    Err(err) => Err(err),
                }
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn check(&self, handle: i32) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(_) => Ok(()),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn export_wallet(&self, wallet_handle: i32, export_config_json: &str, version: u32) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let export_config: WalletExportConfig = match serde_json::from_str(export_config_json) {
                    Ok(config) => config,
                    Err(_) => return Err(WalletError::CommonError(CommonError::InvalidStructure("export config not valid json".to_string()))),
                };

                let path = PathBuf::from(&export_config.path);

                if let Some(parent_path) = path.parent() {
                    fs::DirBuilder::new()
                        .recursive(true)
                        .create(parent_path)?;
                }

                let export_file =
                    OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(export_config.path)?;
                let writer = Box::new(export_file);
                export(wallet, writer, &export_config.key, version)
            }
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }
    }

    pub fn import_wallet(&self,
                         pool_name: &str,
                         name: &str,
                         storage_type: Option<&str>,
                         storage_config: Option<&str>,
                         credentials: &str,
                         import_config_json: &str) -> Result<(), WalletError> {
        let import_config: WalletExportConfig = match serde_json::from_str(import_config_json) {
            Ok(config) => config,
            Err(_) => return Err(WalletError::CommonError(CommonError::InvalidStructure("import config not valid json".to_string()))),
        };
        let import_file =
            OpenOptions::new()
                .read(true)
                .open(&import_config.path)?;

        // TODO - this can be refactor to skip the entire wallet_handle ceremony,
        // but in order to do that a lot of WalletService needs to be refactored
        self.create_wallet(pool_name, name, storage_type, storage_config, credentials)?;
        match self.open_wallet(name, None, credentials) {
            Err(err) => {
                // Ignores the error, since there is nothing that can be done
                self.delete_wallet(name, credentials).ok(); // TODO: why do we ignore thr result here?  ok is used to avoid warning
                Err(err)
            }
            Ok(wallet_handle) => {
                let res = match self.wallets.borrow().get(&wallet_handle) {
                    Some(wallet) => {
                        let reader = Box::new(import_file);
                        match import(wallet, reader, &import_config.key) {
                            Ok(_) => Ok(()),
                            err @ Err(_) => {
                                self.delete_wallet(name, credentials).ok();  // TODO: why do we ignore thr result here?  ok is used to avoid warning
                                err
                            }
                        }
                    }
                    None => {
                        // This should never happen
                        Err(WalletError::InvalidHandle(wallet_handle.to_string()))
                    }
                };
                self.close_wallet(wallet_handle)?;
                res
            }
        }
    }

    pub const PREFIX: &'static str = "Indy";

    pub fn add_prefix(&self, type_: &str) -> String {
        format!("{}::{}", WalletService::PREFIX, type_)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletRecord {
    #[serde(rename = "id")]
    name: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    value: Option<String>,
    tags: Option<HashMap<String, String>>
}

impl JsonEncodable for WalletRecord {}

impl WalletRecord {
    pub fn new(name: String, type_: Option<String>, value: Option<String>, tags: Option<HashMap<String, String>>) -> WalletRecord {
        WalletRecord {
            name,
            type_,
            value,
            tags,
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

    pub fn get_tags(&self) -> Option<&HashMap<String, String>> {
        self.tags.as_ref()
    }
}

fn default_true() -> bool { true }

fn default_false() -> bool { false }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordOptions {
    #[serde(default = "default_false")]
    retrieve_type: bool,
    #[serde(default = "default_true")]
    retrieve_value: bool,
    #[serde(default = "default_false")]
    retrieve_tags: bool
}

impl JsonEncodable for RecordOptions {}

impl<'a> JsonDecodable<'a> for RecordOptions {}

impl RecordOptions {
    pub fn id() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: false,
            retrieve_tags: false
        };

        options.to_json().unwrap()
    }

    pub fn id_value() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: true,
            retrieve_tags: false
        };

        options.to_json().unwrap()
    }

    pub fn full() -> String {
        let options = RecordOptions {
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true
        };

        options.to_json().unwrap()
    }
}

impl Default for RecordOptions {
    fn default() -> RecordOptions {
        RecordOptions {
            retrieve_type: false,
            retrieve_value: true,
            retrieve_tags: false,
        }
    }
}

pub struct WalletSearch {
    iter: iterator::WalletIterator,
}

impl WalletSearch {
    pub fn get_total_count(&self) -> Result<Option<usize>, WalletError> {
        self.iter.get_total_count()
    }

    pub fn fetch_next_record(&mut self) -> Result<Option<WalletRecord>, WalletError> {
        self.iter.next()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchOptions {
    #[serde(default = "default_true")]
    retrieve_records: bool,
    #[serde(default = "default_false")]
    retrieve_total_count: bool,
    #[serde(default = "default_false")]
    retrieve_type: bool,
    #[serde(default = "default_true")]
    retrieve_value: bool,
    #[serde(default = "default_false")]
    retrieve_tags: bool
}

impl SearchOptions {
    pub fn full() -> String {
        let options = SearchOptions {
            retrieve_records: true,
            retrieve_total_count: true,
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true
        };

        options.to_json().unwrap()
    }

    pub fn id_value() -> String {
        let options = SearchOptions {
            retrieve_records: true,
            retrieve_total_count: true,
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: false
        };

        options.to_json().unwrap()
    }
}

impl Default for SearchOptions {
    fn default() -> SearchOptions {
        SearchOptions {
            retrieve_records: true,
            retrieve_total_count: false,
            retrieve_type: false,
            retrieve_value: true,
            retrieve_tags: false,
        }
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

fn _write_file(path: PathBuf, data: &str) -> Result<(), WalletError> {
    let mut config_file = File::create(path)?;
    config_file.write_all(data.as_bytes())?;
    config_file.sync_all()?;
    Ok(())
}

fn _read_file(path: PathBuf) -> Result<String, WalletError> {
    let mut data = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn _read_config(name: &str) -> Result<(String, WalletConfig), WalletError> {
    let config_path = _wallet_config_path(name);
    let config_json = _read_file(config_path)?;
    let config = serde_json::from_str::<WalletConfig>(&config_json)
        .map_err(|_| CommonError::InvalidState("Cannot deserialize Storage Config".to_string()))?;
    Ok((config_json, config))
}

#[cfg(test)]
mod tests {
    use std;
    use std::collections::HashMap;
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;
    use std::path::Path;

    type Tags = HashMap<String, String>;

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        let mut map = HashMap::new();
        map.insert("retrieveType", type_);
        map.insert("retrieveValue", value);
        map.insert("retrieveTags", tags);
        serde_json::to_string(&map).unwrap()
    }

    fn _credentials() -> String {
        String::from(r#"{"key":"my_key"}"#)
    }

    fn _rekey_credentials() -> String {
        String::from(r#"{"key":"my_key", "rekey": "my_new_key"}"#)
    }

    fn _credentials_for_new_key() -> String {
        String::from(r#"{"key": "my_new_key"}"#)
    }

    fn _cleanup() {
        let mut path = std::env::home_dir().unwrap();
        path.push(".indy_client");
        path.push("wallet");
        path.push("test_wallet");
        if path.exists() {
            std::fs::remove_dir_all(path.clone()).unwrap();
        }
    }

    fn _register_inmem_wallet(wallet_service: &WalletService) {
        wallet_service
            .register_wallet_storage(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::add_record,
                InmemWallet::update_record_value,
                InmemWallet::update_record_tags,
                InmemWallet::add_record_tags,
                InmemWallet::delete_record_tags,
                InmemWallet::delete_record,
                InmemWallet::get_record,
                InmemWallet::get_record_id,
                InmemWallet::get_record_type,
                InmemWallet::get_record_value,
                InmemWallet::get_record_tags,
                InmemWallet::free_record,
                InmemWallet::get_storage_metadata,
                InmemWallet::set_storage_metadata,
                InmemWallet::free_storage_metadata,
                InmemWallet::search_records,
                InmemWallet::search_all_records,
                InmemWallet::get_search_total_count,
                InmemWallet::fetch_search_next_record,
                InmemWallet::free_search
            )
            .unwrap();
    }

    #[test]
    fn wallet_service_new_works() {
        WalletService::new();
    }

    #[test]
    fn wallet_service_register_type_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);
    }

    #[test]
    fn wallet_service_create_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", Some("default"), None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_wallet_works_for_none_type() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_wallet_works_for_unknown_type() {
        _cleanup();

        let wallet_service = WalletService::new();
        let res = wallet_service.create_wallet("pool1", "test_wallet", Some("unknown"), None, &_credentials());
        assert_match!(Err(WalletError::UnknownType(_)), res);
    }

    #[test]
    fn wallet_service_create_wallet_works_for_twice() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();

        let res = wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials());
        assert_match!(Err(WalletError::AlreadyExists(_)), res);
    }

    #[test]
    fn wallet_service_delete_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        wallet_service.delete_wallet("test_wallet", &_credentials()).unwrap();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_delete_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        wallet_service.delete_wallet("wallet1", &_credentials()).unwrap();
        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_delete_wallet_returns_error_if_wallet_opened() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        let res = wallet_service.delete_wallet("test_wallet", &_credentials());

        assert_match!(Err(WalletError::CommonError(CommonError::InvalidState(_))), res);
    }

    #[test]
    fn wallet_service_open_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_open_unknown_wallet() {
        _cleanup();

        let wallet_service = WalletService::new();
        let res = wallet_service.open_wallet("test_wallet_unknown", None, &_credentials());
        assert_match!(Err(WalletError::NotFound(_)), res);
    }

    #[test]
    fn wallet_service_open_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_list_wallets_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

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
        wallet_service.create_wallet(&w1_meta.pool_name,
                                     &w1_meta.name,
                                     Some(&w1_meta.xtype),
                                     None, &_credentials()).unwrap();
        wallet_service.create_wallet(&w2_meta.pool_name,
                                     &w2_meta.name,
                                     Some(&w2_meta.xtype),
                                     None, &_credentials()).unwrap();
        wallet_service.create_wallet(&w3_meta.pool_name,
                                     &w3_meta.name,
                                     None,
                                     None, &_credentials()).unwrap();

        let wallets = wallet_service.list_wallets().unwrap();

        assert!(wallets.contains(&w1_meta));
        assert!(wallets.contains(&w2_meta));
        assert!(wallets.contains(&w3_meta));
    }

    #[test]
    fn wallet_service_open_wallet_without_master_key_in_credentials_returns_error() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let res = wallet_service.open_wallet("test_wallet", None, "{}");
        assert_match!(Err(WalletError::InputError(_)), res);
    }

    #[test]
    fn wallet_service_close_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();
    }

    #[test]
    fn wallet_service_close_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();
    }

    #[test]
    fn wallet_service_add_record_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_add_record_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_get_record_works_for_id_only() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, false, false)).unwrap();
        assert!(record.get_value().is_none());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_id_only() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, false, false)).unwrap();
        assert!(record.get_value().is_none());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_id_value() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();
        assert_eq!("value1", record.get_value().unwrap());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_id_value() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();
        assert_eq!("value1", record.get_value().unwrap());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_all_fields() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        let mut tags = HashMap::new();
        tags.insert(String::from("1"), String::from("some"));

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &tags).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        assert_eq!(&tags, record.get_tags().unwrap());
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_for_all_fields() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();
        let tags = serde_json::from_str(r#"{"1":"some"}"#).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &tags).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        assert_eq!(tags, record.get_tags().unwrap().clone());
    }

    #[test]
    fn wallet_service_add_get_works_for_reopen() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();
        assert_eq!("value1", record.get_value().unwrap());
    }

    #[test]
    fn wallet_service_get_works_for_unknown() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        let res = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false));

        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_service_get_works_for_plugged_and_unknown() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        let res = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    /**
     * Update tests
    */
    #[test]
    fn wallet_service_update() {
        _cleanup();
        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());
        wallet_service.update_record_value(wallet_handle, type_, name, new_value).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(new_value, record.get_value().unwrap());
    }

    #[test]
    fn wallet_service_update_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());
        wallet_service.update_record_value(wallet_handle, type_, name, new_value).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(new_value, record.get_value().unwrap());
    }

    /**
     * Delete tests
    */
    #[test]
    fn wallet_service_delete_record() {
        _cleanup();
        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());
        wallet_service.delete_record(wallet_handle, type_, name).unwrap();
        let res = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_service_delete_record_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());
        wallet_service.delete_record(wallet_handle, type_, name).unwrap();
        let res = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    /**
     * Add tags tests
     */
    #[test]
    fn wallet_service_add_tags() {
        _cleanup();
        let type_ = "type";
        let name = "name";
        let value = "value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        wallet_service.add_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    #[test]
    fn wallet_service_add_tags_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        wallet_service.add_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    /**
     * Update tags tests
     */
    #[test]
    fn wallet_service_update_tags() {
        _cleanup();
        let type_ = "type";
        let name = "name";
        let value = "value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let wallet_service = WalletService::new();

        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

        wallet_service.update_record_tags(wallet_handle, type_, name, &new_tags).unwrap();
        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(new_tags, retrieved_tags);
    }

    #[test]
    fn wallet_service_update_tags_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

        wallet_service.update_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(new_tags, retrieved_tags);
    }

    /**
     * Delete tags tests
     */
    #[test]
    fn wallet_service_delete_tags() {
        _cleanup();
        let type_ = "type";
        let name = "name";
        let value = "value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

        let wallet_service = WalletService::new();

        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let tag_names = vec!["tag_name_1", "~tag_name_3"];
        wallet_service.delete_record_tags(wallet_handle, type_, name, &tag_names).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }


    #[test]
    fn wallet_service_delete_tags_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let tag_names = vec!["tag_name_1", "~tag_name_3"];
        wallet_service.delete_record_tags(wallet_handle, type_, name, &tag_names).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    #[test]
    fn wallet_service_get_pool_name_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        let wallet_name = "test_wallet";
        let pool_name = "pool1";
        wallet_service.create_wallet(pool_name, wallet_name, None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(wallet_name, None, &_credentials()).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), pool_name);
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), "pool1");
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_incorrect_wallet_handle() {
        _cleanup();

        let wallet_service = WalletService::new();
        let wallet_name = "test_wallet";
        let pool_name = "pool1";
        wallet_service.create_wallet(pool_name, wallet_name, None, None, &_credentials()).unwrap();

        let get_pool_name_res = wallet_service.get_pool_name(1);
        assert_match!(Err(WalletError::InvalidHandle(_)), get_pool_name_res);
    }

    #[test]
    fn wallet_service_search_records_works() {
        _cleanup();
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.add_record(wallet_handle, "type", "key2", "value2", &HashMap::new()).unwrap();
        wallet_service.add_record(wallet_handle, "type3", "key3", "value3", &HashMap::new()).unwrap();

        let mut search = wallet_service.search_records(wallet_handle, "type3", "{}", &_fetch_options(true, true, true)).unwrap();

        let record = search.fetch_next_record().unwrap().unwrap();
        assert_eq!("value3", record.get_value().unwrap());
        assert_eq!(HashMap::new(), record.get_tags().unwrap().clone());

        assert!(search.fetch_next_record().unwrap().is_none());
    }

    #[test]
    fn wallet_service_search_records_works_for_plugged_wallet() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet("pool1", "wallet1", Some("inmem"), None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("wallet1", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.add_record(wallet_handle, "type", "key2", "value2", &HashMap::new()).unwrap();
        wallet_service.add_record(wallet_handle, "type3", "key3", "value3", &HashMap::new()).unwrap();

        let mut search = wallet_service.search_records(wallet_handle, "type3", "{}", &_fetch_options(true, true, true)).unwrap();

        let record = search.fetch_next_record().unwrap().unwrap();
        assert_eq!("value3", record.get_value().unwrap());
        assert_eq!(HashMap::new(), record.get_tags().unwrap().clone());

        assert!(search.fetch_next_record().unwrap().is_none());
    }

    /**
        Key rotation test
    */
    #[test]
    fn wallet_service_key_rotation() {
        _cleanup();
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());

        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_rekey_credentials()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        wallet_service.close_wallet(wallet_handle).unwrap();

        // Access failed for old key
        let res = wallet_service.open_wallet("test_wallet", None, &_credentials());
        assert_match!(Err(WalletError::AccessFailed(_)), res);

        // Works ok with new key when reopening
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials_for_new_key()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
    }

    /**
        Export/Import tests
    */
    fn _get_export_dir_path() -> PathBuf {
        EnvironmentUtils::tmp_file_path("export_tests")
    }

    fn _get_export_file_path() -> PathBuf {
        let mut path = EnvironmentUtils::tmp_file_path("export_tests");
        path.push("export_test");
        path
    }

    fn _get_export_config() -> String {
        let json = json!({
            "path": _get_export_file_path().to_str().unwrap(),
            "key": "export_key",
        });
        serde_json::to_string(&json).unwrap()
    }

    fn _prepare_export_path() {
        let export_directory_path = _get_export_dir_path();
        if export_directory_path.exists() {
            std::fs::remove_dir_all(export_directory_path.clone()).unwrap();
        }
        std::fs::create_dir(export_directory_path).unwrap();
    }

    #[test]
    fn wallet_service_export_wallet_when_empty() {
        _cleanup();
        _prepare_export_path();
        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        let export_config = _get_export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();

        assert!(Path::new(&_get_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_wallet_1_item() {
        _cleanup();
        _prepare_export_path();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

        let export_config = _get_export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();
        assert!(Path::new(&_get_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_file_exists() {
        _cleanup();
        _prepare_export_path();
        let export_file_path = _get_export_file_path();
        File::create(Path::new(&export_file_path)).unwrap();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        let export_config = _get_export_config();
        let res = wallet_service.export_wallet(wallet_handle, &export_config, 0);
        assert_match!(Err(WalletError::CommonError(CommonError::IOError(_))), res);
        assert!(Path::new(&_get_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_wrong_handle() {
        _cleanup();
        _prepare_export_path();
        let export_file_path = _get_export_file_path();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        let export_config = _get_export_config();
        let res = wallet_service.export_wallet(wallet_handle + 1, &export_config, 0);
        assert_match!(Err(WalletError::InvalidHandle(_)), res);
        assert!(!Path::new(&_get_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item() {
        _cleanup();
        _prepare_export_path();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

        let export_config = _get_export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();
        assert!(Path::new(&_get_export_file_path()).exists());

        wallet_service.close_wallet(wallet_handle).unwrap();
        wallet_service.delete_wallet("test_wallet", &_credentials()).unwrap();

        wallet_service.import_wallet("pool1", "test_wallet", None, None, &_credentials(), &export_config).unwrap();
        let wallet_handle_2 = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
        wallet_service.get_record(wallet_handle_2, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_export_import_wallet_if_empty() {
        _cleanup();
        _prepare_export_path();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet("pool1", "test_wallet", None, None, &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();

        let export_config = _get_export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();
        assert!(Path::new(&_get_export_file_path()).exists());

        wallet_service.close_wallet(wallet_handle).unwrap();
        wallet_service.delete_wallet("test_wallet", &_credentials()).unwrap();

        wallet_service.import_wallet("pool1", "test_wallet", None, None, &_credentials(), &export_config).unwrap();
        wallet_service.open_wallet("test_wallet", None, &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_export_import_returns_error_if_path_missing() {
        _cleanup();
        _prepare_export_path();

        let wallet_service = WalletService::new();

        let export_config = _get_export_config();

        let res = wallet_service.import_wallet("pool1", "test_wallet", None, None, &_credentials(), &export_config);
        assert_match!(Err(WalletError::CommonError(CommonError::IOError(_))), res);

        let res = wallet_service.open_wallet("test_wallet", None, &_credentials());
        assert_match!(Err(_), res);
    }
}