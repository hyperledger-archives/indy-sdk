mod storage;
mod encryption;
mod query_encryption;
mod iterator;
mod language;
mod export_import;
mod wallet;

use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use named_type::NamedType;
use std::rc::Rc;

use api::wallet::*;
use domain::wallet::{Config, Credentials, ExportConfig, Metadata};
use errors::wallet::WalletError;
use errors::common::CommonError;
use utils::sequence::SequenceUtils;

use self::export_import::{export, import};
use self::storage::WalletStorageType;
use self::storage::default::SQLiteStorageType;
use self::storage::plugged::PluggedStorageType;
use self::wallet::{Wallet, Keys};

pub struct WalletService {
    storage_types: RefCell<HashMap<String, Box<WalletStorageType>>>,
    wallets: RefCell<HashMap<i32, Box<Wallet>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let storage_types = {
            let mut map: HashMap<String, Box<WalletStorageType>> = HashMap::new();
            map.insert("default".to_string(), Box::new(SQLiteStorageType::new()));
            RefCell::new(map)
        };

        WalletService {
            storage_types,
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
                         config: &str,
                         credentials: &str) -> Result<(), WalletError> {
        trace!("create_wallet >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: Log credentials in debug

        let config: Config = serde_json::from_str(config)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let credentials: Credentials = serde_json::from_str(credentials)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize credentials: {:?}", err)))?;

        if config.id.is_empty() {
            Err(CommonError::InvalidStructure("Wallet id is empty".to_string()))?
        }

        let storage_types = self.storage_types.borrow();

        let storage_type = {
            let storage_type = config.storage_type
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            storage_types
                .get(storage_type)
                .ok_or(WalletError::UnknownType(storage_type.to_string()))?
        };

        let metadata = {
            let master_key_salt = encryption::gen_master_key_salt()?;
            let master_key = encryption::derive_master_key(&credentials.key, &master_key_salt)?;

            let metadata = Metadata {
                master_key_salt: master_key_salt[..].to_vec(),
                keys: Keys::new().serialize_encrypted(&master_key)?,
            };

            serde_json::to_vec(&metadata)
                .map_err(|err| CommonError::InvalidState(format!("Cannot serialize wallet metadata: {:?}", err)))?
        };

        storage_type.create_storage(&config.id,
                                    config.storage_config
                                        .map(|value| value.to_string())
                                        .as_ref()
                                        .map(String::as_str),
                                    credentials.storage_credentials
                                        .map(|value| value.to_string())
                                        .as_ref()
                                        .map(String::as_str),
                                    &metadata)?;

        Ok(())
    }

    pub fn delete_wallet(&self, config: &str, credentials: &str) -> Result<(), WalletError> {
        trace!("delete_wallet >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: Log credentials in debug

        let config: Config = serde_json::from_str(config)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let credentials: Credentials = serde_json::from_str(credentials)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize credentials: {:?}", err)))?;

        if self.wallets.borrow_mut().values().any(|ref wallet| wallet.get_id() == config.id) {
            Err(CommonError::InvalidState(format!("Wallet has to be closed before deleting: {:?}", config.id)))?
        }

        let storage_types = self.storage_types.borrow();

        let storage_type = {
            let storage_type = config.storage_type
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            storage_types
                .get(storage_type)
                .ok_or(WalletError::UnknownType(storage_type.to_string()))?
        };

        let storage_config = config.storage_config.map(|value| value.to_string());
        let storage_credentials = credentials.storage_credentials.map(|value| value.to_string());

        // check credentials and close connection before deleting wallet
        {
            let storage = storage_type.open_storage(&config.id,
                                                    storage_config
                                                        .as_ref()
                                                        .map(String::as_str),
                                                    storage_credentials
                                                        .as_ref()
                                                        .map(String::as_str))?;

            let metadata: Metadata = {
                let metadata = storage.get_storage_metadata()?;
                serde_json::from_slice(&metadata)
                    .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize metadata: {:?}", err)))?
            };

            let master_key = {
                let master_key_salt = encryption::master_key_salt_from_slice(&metadata.master_key_salt)?;
                encryption::derive_master_key(&credentials.key, &master_key_salt)?
            };

            Keys::deserialize_encrypted(&metadata.keys, &master_key)
                .map_err(|_| WalletError::AccessFailed("Invalid master key provided".to_string()))?;
        }

        storage_type.delete_storage(&config.id,
                                    storage_config
                                        .as_ref()
                                        .map(String::as_str),
                                    storage_credentials
                                        .as_ref()
                                        .map(String::as_str))?;

        trace!("delete_wallet <<<");
        Ok(())
    }

    pub fn open_wallet(&self, config: &str, credentials: &str) -> Result<i32, WalletError> {
        trace!("open_wallet >>> config: {:?}, credentials: {:?}", config, "_"); // TODO: FIXME: Log secrets in debug

        let config: Config = serde_json::from_str(config)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let credentials: Credentials = serde_json::from_str(credentials)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize credentials: {:?}", err)))?;

        if config.id.is_empty() {
            Err(CommonError::InvalidStructure("Wallet id is empty".to_string()))?
        }

        if self.wallets.borrow_mut().values().any(|ref wallet| wallet.get_id() == config.id) {
            Err(WalletError::AlreadyOpened(config.id.clone()))?
        }

        let storage_types = self.storage_types.borrow();

        let storage_type = {
            let storage_type = config.storage_type
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            storage_types
                .get(storage_type)
                .ok_or(WalletError::UnknownType(storage_type.to_string()))?
        };

        let storage_config = config.storage_config.map(|value| value.to_string());
        let storage_credentials = credentials.storage_credentials.map(|value| value.to_string());
        let storage = storage_type.open_storage(&config.id,
                                                storage_config
                                                    .as_ref()
                                                    .map(String::as_str),
                                                storage_credentials
                                                    .as_ref()
                                                    .map(String::as_str))?;

        let metadata: Metadata = {
            let metadata = storage.get_storage_metadata()?;
            serde_json::from_slice(&metadata)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize metadata: {:?}", err)))?
        };

        let master_key = {
            let master_key_salt = encryption::master_key_salt_from_slice(&metadata.master_key_salt)?;
            encryption::derive_master_key(&credentials.key, &master_key_salt)?
        };

        let keys = Keys::deserialize_encrypted(&metadata.keys, &master_key)?;

        // Rotate master key
        if let Some(rekey) = credentials.rekey {

            let metadata = {
                let master_key_salt = encryption::gen_master_key_salt()?;
                let master_key = encryption::derive_master_key(&rekey, &master_key_salt)?;

                let metadata = Metadata {
                    master_key_salt: master_key_salt[..].to_vec(),
                    keys: keys.serialize_encrypted(&master_key)?,
                };

                serde_json::to_vec(&metadata)
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize wallet metadata: {:?}", err)))?
            };

            storage.set_storage_metadata(&metadata)?;
        }

        let wallet = Wallet::new(config.id, storage, Rc::new(keys));

        let wallet_handle = SequenceUtils::get_next_id();
        let mut wallets = self.wallets.borrow_mut();
        wallets.insert(wallet_handle, Box::new(wallet));

        trace!("open_wallet <<< res: {:?}", wallet_handle);
        Ok(wallet_handle)
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
                              -> Result<String, WalletError> where T: ::serde::Serialize + Sized, T: NamedType {
        let type_ = T::short_type_name();
        let object_json = serde_json::to_string(object)
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

    pub fn update_indy_object<T>(&self, wallet_handle: i32, name: &str, object: &T) -> Result<String, WalletError> where T: ::serde::Serialize + Sized, T: NamedType {
        let type_ = T::short_type_name();
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let object_json = serde_json::to_string(object)
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
    pub fn get_indy_object<'a, T>(&self, wallet_handle: i32, name: &str, options_json: &str, json: &'a mut String) -> Result<T, WalletError> where T: ::serde::Deserialize<'a>, T: NamedType {
        let type_ = T::short_type_name();

        let record: WalletRecord = match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.get(&self.add_prefix(type_), name, options_json),
            None => Err(WalletError::InvalidHandle(wallet_handle.to_string()))
        }?;

        *json = record.get_value()
            .ok_or(CommonError::InvalidStructure(format!("{} not found for id: {:?}", type_, name)))?.to_string();

        serde_json::from_str(json)
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

    pub fn upsert_indy_object<T>(&self, wallet_handle: i32, name: &str, object: &T) -> Result<String, WalletError>
        where T: ::serde::Serialize + Sized, T: NamedType {
        if self.record_exists::<T>(wallet_handle, name)? {
            self.update_indy_object::<T>(wallet_handle, name, object)
        } else {
            self.add_indy_object::<T>(wallet_handle, name, object, &HashMap::new())
        }
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

    pub fn export_wallet(&self, wallet_handle: i32, export_config: &str, version: u32) -> Result<(), WalletError> {
        let wallets = self.wallets.borrow();
        let wallet = wallets
            .get(&wallet_handle)
            .ok_or(WalletError::InvalidHandle(wallet_handle.to_string()))?;

        let export_config: ExportConfig = serde_json::from_str(export_config)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize export config: {:?}", err)))?;

        let path = PathBuf::from(&export_config.path);

        if let Some(parent_path) = path.parent() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(parent_path)?;
        }

        let mut export_file =
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(export_config.path)?;

        export(wallet, &mut export_file, &export_config.key, version)
    }

    pub fn import_wallet(&self,
                         config: &str,
                         credentials: &str,
                         export_config: &str) -> Result<(), WalletError> {
        let export_config: ExportConfig = serde_json::from_str(export_config)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize export config: {:?}", err)))?;

        let mut export_file =
            fs::OpenOptions::new()
                .read(true)
                .open(&export_config.path)?;

        // TODO - this can be refactor to skip the entire wallet_handle ceremony,
        // but in order to do that a lot of WalletService needs to be refactored

        self.create_wallet(config, credentials)?;
        let wallet_handle = self.open_wallet(config, credentials)?;

        let res = {
            // to finish self.wallets borrowing
            let wallets = self.wallets.borrow();
            let wallet = wallets
                .get(&wallet_handle)
                .ok_or(WalletError::InvalidHandle(wallet_handle.to_string()))?; // This should never happen

            import(wallet, &mut export_file, &export_config.key)
        };

        self.close_wallet(wallet_handle)?;

        if res.is_err() {
            self.delete_wallet(config, credentials)?;
        }

        res
    }

    pub const PREFIX: &'static str = "Indy";

    pub fn add_prefix(&self, type_: &str) -> String {
        format!("{}::{}", WalletService::PREFIX, type_)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletRecord {
    #[serde(rename = "id")]
    name: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    value: Option<String>,
    tags: Option<HashMap<String, String>>
}

impl Ord for WalletRecord {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        (&self.type_, &self.name).cmp(&(&other.type_, &other.name))
    }
}

impl PartialOrd for WalletRecord {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        (&self.type_, &self.name).partial_cmp(&(&other.type_, &other.name))
    }
}

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

impl RecordOptions {
    pub fn id() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: false,
            retrieve_tags: false
        };

        serde_json::to_string(&options).unwrap()
    }

    pub fn id_value() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: true,
            retrieve_tags: false
        };

        serde_json::to_string(&options).unwrap()
    }

    pub fn full() -> String {
        let options = RecordOptions {
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true
        };

        serde_json::to_string(&options).unwrap()
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

        serde_json::to_string(&options).unwrap()
    }

    pub fn id_value() -> String {
        let options = SearchOptions {
            retrieve_records: true,
            retrieve_total_count: true,
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: false
        };

        serde_json::to_string(&options).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::collections::HashMap;
    use std::path::Path;

    use errors::wallet::WalletError;
    use utils::environment::EnvironmentUtils;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;

    #[test]
    fn wallet_service_new_works() {
        WalletService::new();
    }

    #[test]
    fn wallet_service_register_type_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);
    }

    #[test]
    fn wallet_service_create_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config_default(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_works_for_plugged() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_wallet_works_for_none_type() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_create_wallet_works_for_unknown_type() {
        _cleanup();

        let wallet_service = WalletService::new();
        let res = wallet_service.create_wallet(&_config_unknown(), &_credentials());
        assert_match!(Err(WalletError::UnknownType(_)), res);
    }

    #[test]
    fn wallet_service_create_wallet_works_for_twice() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();

        let res = wallet_service.create_wallet(&_config(), &_credentials());
        assert_match!(Err(WalletError::AlreadyExists(_)), res);
    }

    #[test]
    fn wallet_service_delete_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.delete_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_delete_works_for_plugged() {
        _cleanup();

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        wallet_service.delete_wallet(&_config_inmem(), &_credentials()).unwrap();
        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_delete_wallet_returns_error_if_wallet_opened() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let res = wallet_service.delete_wallet(&_config(), &_credentials());

        assert_match!(Err(WalletError::CommonError(CommonError::InvalidState(_))), res);
    }

    #[test]
    fn wallet_service_open_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        // cleanup
        wallet_service.close_wallet(handle).unwrap();
    }

    #[test]
    fn wallet_service_open_unknown_wallet() {
        _cleanup();

        let wallet_service = WalletService::new();
        let res = wallet_service.open_wallet(&_config(), &_credentials());
        assert_match!(Err(WalletError::NotFound(_)), res);
    }

    #[test]
    fn wallet_service_open_works_for_plugged() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_open_wallet_without_master_key_in_credentials_returns_error() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let res = wallet_service.open_wallet(&_config(), "{}");
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn wallet_service_close_wallet_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();
    }

    #[test]
    fn wallet_service_close_works_for_plugged() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();
    }

    #[test]
    fn wallet_service_add_record_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_add_record_works_for_plugged() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_get_record_works_for_id_only() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

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

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();

        assert_eq!("value1", record.get_value().unwrap());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_id_value() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
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
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();
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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();
        assert_eq!("value1", record.get_value().unwrap());
    }

    #[test]
    fn wallet_service_get_works_for_unknown() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let res = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false));

        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_service_get_works_for_plugged_and_unknown() {
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());

        wallet_service.update_record_value(wallet_handle, type_, name, new_value).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(new_value, record.get_value().unwrap());
    }

    #[test]
    fn wallet_service_update_for_plugged() {
        _cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());

        wallet_service.delete_record(wallet_handle, type_, name).unwrap();
        let res = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_service_delete_record_for_plugged() {
        _cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

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
        _cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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

        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

        wallet_service.update_record_tags(wallet_handle, type_, name, &new_tags).unwrap();
        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(new_tags, retrieved_tags);
    }

    #[test]
    fn wallet_service_update_tags_for_plugged() {
        _cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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

        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

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
        _cleanup();

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let tag_names = vec!["tag_name_1", "~tag_name_3"];
        wallet_service.delete_record_tags(wallet_handle, type_, name, &tag_names).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    #[test]
    fn wallet_service_search_records_works() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

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
        _cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &_credentials()).unwrap();

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
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());

        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet(&_config(), &_rekey_credentials()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        wallet_service.close_wallet(wallet_handle).unwrap();

        // Access failed for old key
        let res = wallet_service.open_wallet(&_config(), &_credentials());
        assert_match!(Err(WalletError::AccessFailed(_)), res);

        // Works ok with new key when reopening
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials_for_new_key()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
    }

    #[test]
    fn wallet_service_export_wallet_when_empty() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let export_config = _export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();

        assert!(Path::new(&_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_wallet_1_item() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

        let export_config = _export_config();
        wallet_service.export_wallet(wallet_handle, &export_config, 0).unwrap();
        assert!(Path::new(&_export_file_path()).exists());
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_file_exists() {
        _cleanup();

        {
            fs::create_dir_all(_export_file_path().parent().unwrap()).unwrap();
            fs::File::create(_export_file_path()).unwrap();
        }

        assert!(_export_file_path().exists());

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let res = wallet_service.export_wallet(wallet_handle, &_export_config(), 0);
        assert_match!(Err(WalletError::CommonError(CommonError::IOError(_))), res);
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_wrong_handle() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        let res = wallet_service.export_wallet(wallet_handle + 1, &_export_config(), 0);
        assert_match!(Err(WalletError::InvalidHandle(_)), res);
        assert!(!_export_file_path().exists());
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

        wallet_service.export_wallet(wallet_handle, &_export_config(), 0).unwrap();
        assert!(_export_file_path().exists());

        wallet_service.close_wallet(wallet_handle).unwrap();
        wallet_service.delete_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.import_wallet(&_config(), &_credentials(), &_export_config()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_export_import_wallet_if_empty() {
        _cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.export_wallet(wallet_handle, &_export_config(), 0).unwrap();
        assert!(_export_file_path().exists());

        wallet_service.close_wallet(wallet_handle).unwrap();
        wallet_service.delete_wallet(&_config(), &_credentials()).unwrap();

        wallet_service.import_wallet(&_config(), &_credentials(), &_export_config()).unwrap();
        wallet_service.open_wallet(&_config(), &_credentials()).unwrap();
    }

    #[test]
    fn wallet_service_export_import_returns_error_if_path_missing() {
        _cleanup();

        let wallet_service = WalletService::new();

        let res = wallet_service.import_wallet(&_config(), &_credentials(), &_export_config());
        assert_match!(Err(WalletError::CommonError(CommonError::IOError(_))), res);

        let res = wallet_service.open_wallet(&_config(), &_credentials());
        assert_match!(Err(_), res);
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
          "retrieveType": type_,
          "retrieveValue": value,
          "retrieveTags": tags,
        }).to_string()
    }

    fn _config() -> String {
        json!({"id": "w1"}).to_string()
    }

    fn _config_default() -> String {
        json!({"id": "w1", "storage_type": "default"}).to_string()
    }

    fn _config_inmem() -> String {
        json!({"id": "w1", "storage_type": "inmem"}).to_string()
    }

    fn _config_unknown() -> String {
        json!({"id": "w1", "storage_type": "unknown"}).to_string()
    }

    fn _credentials() -> String {
        json!({"key": "my_key"}).to_string()
    }

    fn _rekey_credentials() -> String {
        json!({"key": "my_key", "rekey": "my_new_key"}).to_string()
    }

    fn _credentials_for_new_key() -> String {
        json!({"key": "my_new_key"}).to_string()
    }

    fn _export_file_path() -> PathBuf {
        let mut path = EnvironmentUtils::tmp_file_path("export_tests");
        path.push("export_test");
        path
    }

    fn _export_config() -> String {
        json!({
            "path": _export_file_path().to_str().unwrap(),
            "key": "export_key",
        }).to_string()
    }

    fn _cleanup() {
        TestUtils::cleanup_storage();
        InmemWallet::cleanup();
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
}
