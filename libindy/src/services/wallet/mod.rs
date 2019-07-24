use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

use named_type::NamedType;
use serde_json;

use api::wallet::*;

use domain::wallet::{Config, Credentials, ExportConfig, Metadata, MetadataArgon, MetadataRaw, Tags};
use errors::prelude::*;
pub use services::wallet::encryption::KeyDerivationData;
use utils::crypto::chacha20poly1305_ietf;
use utils::crypto::chacha20poly1305_ietf::Key as MasterKey;
use utils::sequence;

use self::export_import::{export_continue, finish_import, preparse_file_to_import};
use self::storage::{WalletStorage, WalletStorageType};
use self::storage::default::SQLiteStorageType;
use self::storage::plugged::PluggedStorageType;
use self::wallet::{Keys, Wallet};
use api::WalletHandle;

mod storage;
mod encryption;
mod query_encryption;
mod iterator;
// TODO: Remove query language out of wallet module
pub mod language;
mod export_import;
mod wallet;

pub struct WalletService {
    storage_types: RefCell<HashMap<String, Box<WalletStorageType>>>,
    wallets: RefCell<HashMap<WalletHandle, Box<Wallet>>>,
    pending_for_open: RefCell<HashMap<WalletHandle, (String /* id */, Box<WalletStorage>, Metadata, Option<KeyDerivationData>)>>,
    pending_for_import: RefCell<HashMap<WalletHandle, (BufReader<::std::fs::File>, chacha20poly1305_ietf::Nonce, usize, Vec<u8>, KeyDerivationData)>>,
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
            wallets: RefCell::new(HashMap::new()),
            pending_for_open: RefCell::new(HashMap::new()),
            pending_for_import: RefCell::new(HashMap::new()),
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
                                   free_search: WalletFreeSearch) -> IndyResult<()> {
        trace!("register_wallet_storage >>> type_: {:?}", type_);

        let mut storage_types = self.storage_types.borrow_mut();

        if storage_types.contains_key(type_) {
            return Err(err_msg(IndyErrorKind::WalletStorageTypeAlreadyRegistered, format!("Wallet storage is already registered for type: {}", type_)));
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
                         config: &Config,
                         credentials: &Credentials,
                         key: (&KeyDerivationData, &MasterKey)) -> IndyResult<()> {
        self._create_wallet(config, credentials, key).map(|_| ())
    }

    fn _create_wallet(&self,
                      config: &Config,
                      credentials: &Credentials,
                      (key_data, master_key): (&KeyDerivationData, &MasterKey)) -> IndyResult<Keys> {
        trace!("create_wallet >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        if config.id.is_empty() {
            Err(err_msg(IndyErrorKind::InvalidStructure, "Wallet id is empty"))?
        }

        let storage_types = self.storage_types.borrow();

        let (storage_type, storage_config, storage_credentials) = WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;

        let keys = Keys::new();
        let metadata = self._prepare_metadata(master_key, key_data, &keys)?;

        storage_type.create_storage(&config.id,
                                    storage_config
                                        .as_ref()
                                        .map(String::as_str),
                                    storage_credentials
                                        .as_ref()
                                        .map(String::as_str),
                                    &metadata)?;

        Ok(keys)
    }

    pub fn delete_wallet_prepare(&self, config: &Config, credentials: &Credentials) -> IndyResult<(Metadata, KeyDerivationData)> {
        trace!("delete_wallet >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        if self.wallets.borrow_mut().values().any(|ref wallet| wallet.get_id() == WalletService::_get_wallet_id(config)) {
            Err(err_msg(IndyErrorKind::InvalidState, format!("Wallet has to be closed before deleting: {:?}", WalletService::_get_wallet_id(config))))?
        }

        // check credentials and close connection before deleting wallet

        let (_, metadata, key_derivation_data) = self._open_storage_and_fetch_metadata(config, &credentials)?;

        Ok((metadata, key_derivation_data))
    }


    pub fn delete_wallet_continue(&self, config: &Config, credentials: &Credentials, metadata: &Metadata, master_key: &MasterKey) -> IndyResult<()> {
        trace!("delete_wallet >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        {
            self._restore_keys(metadata, &master_key)?;
        }

        let storage_types = self.storage_types.borrow();

        let (storage_type, storage_config, storage_credentials) = WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;

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

    pub fn open_wallet_prepare(&self, config: &Config, credentials: &Credentials) -> IndyResult<(WalletHandle, KeyDerivationData, Option<KeyDerivationData>)> {
        trace!("open_wallet >>> config: {:?}, credentials: {:?}", config, secret!(&credentials));

        self._is_id_from_config_not_used(config)?;

        let (storage, metadata, key_derivation_data) = self._open_storage_and_fetch_metadata(config, credentials)?;

        let wallet_handle = WalletHandle(sequence::get_next_id());

        let rekey_data: Option<KeyDerivationData> = credentials.rekey.as_ref().map(|ref rekey|
            KeyDerivationData::from_passphrase_with_new_salt(rekey, &credentials.rekey_derivation_method));

        self.pending_for_open.borrow_mut().insert(wallet_handle, (WalletService::_get_wallet_id(config), storage, metadata, rekey_data.clone()));

        Ok((wallet_handle, key_derivation_data, rekey_data))
    }

    pub fn open_wallet_continue(&self, wallet_handle: WalletHandle, master_key: (&MasterKey, Option<&MasterKey>)) -> IndyResult<WalletHandle> {
        let (id, storage, metadata, rekey_data) = self.pending_for_open.borrow_mut().remove(&wallet_handle)
            .ok_or(err_msg(IndyErrorKind::InvalidState, "Open data not found"))?;

        let (master_key, rekey) = master_key;
        let keys = self._restore_keys(&metadata, &master_key)?;

        // Rotate master key
        if let (Some(rekey), Some(rekey_data)) = (rekey, rekey_data) {
            let metadata = self._prepare_metadata(rekey, &rekey_data, &keys)?;
            storage.set_storage_metadata(&metadata)?;
        }

        let wallet = Wallet::new(id, storage, Rc::new(keys));

        let mut wallets = self.wallets.borrow_mut();
        wallets.insert(wallet_handle, Box::new(wallet));

        trace!("open_wallet <<< res: {:?}", wallet_handle);
        Ok(wallet_handle)
    }

    fn _open_storage_and_fetch_metadata(&self, config: &Config, credentials: &Credentials) -> IndyResult<(Box<WalletStorage>, Metadata, KeyDerivationData)> {
        let storage = self._open_storage(config, credentials)?;
        let metadata: Metadata = {
            let metadata = storage.get_storage_metadata()?;
            serde_json::from_slice(&metadata)
                .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize metadata")?
        };
        let key_derivation_data = KeyDerivationData::from_passphrase_and_metadata(&credentials.key, &metadata, &credentials.key_derivation_method)?;
        Ok((storage, metadata, key_derivation_data))
    }

    pub fn close_wallet(&self, handle: WalletHandle) -> IndyResult<()> {
        trace!("close_wallet >>> handle: {:?}", handle);

        match self.wallets.borrow_mut().remove(&handle) {
            Some(mut wallet) => wallet.close(),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }?;

        trace!("close_wallet <<<");
        Ok(())
    }

    fn _map_wallet_storage_error(err: IndyError, type_: &str, name: &str) -> IndyError {
        match err.kind() {
            IndyErrorKind::WalletItemAlreadyExists => err_msg(IndyErrorKind::WalletItemAlreadyExists, format!("Wallet item already exists with type: {}, id: {}", type_, name)),
            IndyErrorKind::WalletItemNotFound => err_msg(IndyErrorKind::WalletItemNotFound, format!("Wallet item not found with type: {}, id: {}", type_, name)),
            _ => err
        }
    }

    pub fn add_record(&self, wallet_handle: WalletHandle, type_: &str, name: &str, value: &str, tags: &Tags) -> IndyResult<()> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.add(type_, name, value, tags)
                .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn add_indy_object<T>(&self, wallet_handle: WalletHandle, name: &str, object: &T, tags: &Tags)
                              -> IndyResult<String> where T: ::serde::Serialize + Sized, T: NamedType {
        let type_ = T::short_type_name();

        let object_json = serde_json::to_string(object)
            .to_indy(IndyErrorKind::InvalidState, format!("Cannot serialize {:?}", type_))?;

        self.add_record(wallet_handle, &self.add_prefix(type_), name, &object_json, tags)?;
        Ok(object_json)
    }

    pub fn update_record_value(&self, wallet_handle: WalletHandle, type_: &str, name: &str, value: &str) -> IndyResult<()> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) =>
                wallet.update(type_, name, value)
                    .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn update_indy_object<T>(&self, wallet_handle: WalletHandle, name: &str, object: &T) -> IndyResult<String> where T: ::serde::Serialize + Sized, T: NamedType {
        let type_ = T::short_type_name();
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => {
                let object_json = serde_json::to_string(object)
                    .to_indy(IndyErrorKind::InvalidState, format!("Cannot serialize {:?}", type_))?;
                wallet.update(&self.add_prefix(type_), name, &object_json)?;
                Ok(object_json)
            }
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn add_record_tags(&self, wallet_handle: WalletHandle, type_: &str, name: &str, tags: &Tags) -> IndyResult<()> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.add_tags(type_, name, tags)
                .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn update_record_tags(&self, wallet_handle: WalletHandle, type_: &str, name: &str, tags: &Tags) -> IndyResult<()> {
        match self.wallets.borrow_mut().get_mut(&wallet_handle) {
            Some(wallet) => wallet.update_tags(type_, name, tags)
                .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn delete_record_tags(&self, wallet_handle: WalletHandle, type_: &str, name: &str, tag_names: &[&str]) -> IndyResult<()> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.delete_tags(type_, name, tag_names)
                .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn delete_record(&self, wallet_handle: WalletHandle, type_: &str, name: &str) -> IndyResult<()> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.delete(type_, name)
                .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn delete_indy_record<T>(&self, wallet_handle: WalletHandle, name: &str) -> IndyResult<()> where T: NamedType {
        self.delete_record(wallet_handle, &self.add_prefix(T::short_type_name()), name)
    }

    pub fn get_record(&self, wallet_handle: WalletHandle, type_: &str, name: &str, options_json: &str) -> IndyResult<WalletRecord> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) =>
                wallet.get(type_, name, options_json)
                    .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name)),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn get_indy_record<T>(&self, wallet_handle: WalletHandle, name: &str, options_json: &str) -> IndyResult<WalletRecord> where T: NamedType {
        self.get_record(wallet_handle, &self.add_prefix(T::short_type_name()), name, options_json)
    }

    // Dirty hack. json must live longer then result T
    pub fn get_indy_object<T>(&self, wallet_handle: WalletHandle, name: &str, options_json: &str) -> IndyResult<T> where T: ::serde::de::DeserializeOwned, T: NamedType {
        let type_ = T::short_type_name();

        let record: WalletRecord = match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => wallet.get(&self.add_prefix(type_), name, options_json),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }?;

        let record_value = record.get_value()
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("{} not found for id: {:?}", type_, name)))?.to_string();

        serde_json::from_str(&record_value)
            .to_indy(IndyErrorKind::InvalidState, format!("Cannot deserialize {:?}", type_))
    }

    // Dirty hack. json must live longer then result T
    pub fn get_indy_opt_object<T>(&self, wallet_handle: WalletHandle, name: &str, options_json: &str) -> IndyResult<Option<T>> where T: ::serde::de::DeserializeOwned, T: NamedType {
        match self.get_indy_object::<T>(wallet_handle, name, options_json) {
            Ok(res) => Ok(Some(res)),
            Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => Ok(None),
            Err(err) => Err(err)
        }
    }

    pub fn search_records(&self, wallet_handle: WalletHandle, type_: &str, query_json: &str, options_json: &str) -> IndyResult<WalletSearch> {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) => Ok(WalletSearch { iter: wallet.search(type_, query_json, Some(options_json))? }),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn search_indy_records<T>(&self, wallet_handle: WalletHandle, query_json: &str, options_json: &str) -> IndyResult<WalletSearch> where T: NamedType {
        self.search_records(wallet_handle, &self.add_prefix(T::short_type_name()), query_json, options_json)
    }

    #[allow(dead_code)] // TODO: Should we implement getting all records or delete everywhere?
    pub fn search_all_records(&self, _wallet_handle: WalletHandle) -> IndyResult<WalletSearch> {
        //        match self.wallets.borrow().get(&wallet_handle) {
        //            Some(wallet) => wallet.search_all_records(),
        //            None => Err(IndyError::InvalidHandle(wallet_handle.to_string()))
        //        }
        unimplemented!()
    }

    pub fn upsert_indy_object<T>(&self, wallet_handle: WalletHandle, name: &str, object: &T) -> IndyResult<String>
        where T: ::serde::Serialize + Sized, T: NamedType {
        if self.record_exists::<T>(wallet_handle, name)? {
            self.update_indy_object::<T>(wallet_handle, name, object)
        } else {
            self.add_indy_object::<T>(wallet_handle, name, object, &HashMap::new())
        }
    }

    pub fn record_exists<T>(&self, wallet_handle: WalletHandle, name: &str) -> IndyResult<bool> where T: NamedType {
        match self.wallets.borrow().get(&wallet_handle) {
            Some(wallet) =>
                match wallet.get(&self.add_prefix(T::short_type_name()), name, &RecordOptions::id()) {
                    Ok(_) => Ok(true),
                    Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => Ok(false),
                    Err(err) => Err(err),
                }
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn check(&self, handle: WalletHandle) -> IndyResult<()> {
        match self.wallets.borrow().get(&handle) {
            Some(_) => Ok(()),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))
        }
    }

    pub fn export_wallet(&self, wallet_handle: WalletHandle, export_config: &ExportConfig, version: u32, key: (&KeyDerivationData, &MasterKey)) -> IndyResult<()> {
        trace!("export_wallet >>> wallet_handle: {:?}, export_config: {:?}, version: {:?}", wallet_handle, secret!(export_config), version);

        if version != 0 {
            Err(err_msg(IndyErrorKind::InvalidState, "Unsupported version"))?;
        }

        let (key_data, key) = key;

        let wallets = self.wallets.borrow();
        let wallet = wallets
            .get(&wallet_handle)
            .ok_or(err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown wallet handle"))?;

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
                .open(export_config.path.clone())?;

        let res = export_continue(wallet, &mut export_file, version, key.clone(), key_data);

        trace!("export_wallet <<<");

        res
    }

    pub fn import_wallet_prepare(&self,
                                 config: &Config,
                                 credentials: &Credentials,
                                 export_config: &ExportConfig) -> IndyResult<(WalletHandle, KeyDerivationData, KeyDerivationData)> {
        trace!("import_wallet_prepare >>> config: {:?}, credentials: {:?}, export_config: {:?}", config, secret!(export_config), secret!(export_config));

        let exported_file_to_import =
            fs::OpenOptions::new()
                .read(true)
                .open(&export_config.path)?;

        let (reader, import_key_derivation_data, nonce, chunk_size, header_bytes) = preparse_file_to_import(exported_file_to_import, &export_config.key)?;
        let key_data = KeyDerivationData::from_passphrase_with_new_salt(&credentials.key, &credentials.key_derivation_method);

        let wallet_handle = WalletHandle(sequence::get_next_id());

        let stashed_key_data = key_data.clone();

        self.pending_for_import.borrow_mut().insert(wallet_handle, (reader, nonce, chunk_size, header_bytes, stashed_key_data));

        Ok((wallet_handle, key_data, import_key_derivation_data))
    }

    pub fn import_wallet_continue(&self, wallet_handle: WalletHandle, config: &Config, credentials: &Credentials, key: (MasterKey, MasterKey)) -> IndyResult<()> {
        let (reader, nonce, chunk_size, header_bytes, key_data) = self.pending_for_import.borrow_mut().remove(&wallet_handle).unwrap();

        let (import_key, master_key) = key;

        let keys = self._create_wallet(config, credentials, (&key_data, &master_key))?;

        self._is_id_from_config_not_used(config)?;
        let storage = self._open_storage(config, credentials)?;
        let metadata = storage.get_storage_metadata()?;

        let res = {
            let wallet = Wallet::new(WalletService::_get_wallet_id(&config), storage, Rc::new(keys));

            finish_import(&wallet, reader, import_key, nonce, chunk_size, header_bytes)
        };

        if res.is_err() {
            let metadata: Metadata = serde_json::from_slice(&metadata)
                .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize metadata")?;

            self.delete_wallet_continue(config, credentials, &metadata, &master_key)?;
        }

        //        self.close_wallet(wallet_handle)?;

        trace!("import_wallet <<<");
        res
    }

    fn _get_config_and_cred_for_storage<'a>(config: &Config, credentials: &Credentials, storage_types: &'a HashMap<String, Box<WalletStorageType>>) -> IndyResult<(&'a Box<WalletStorageType>, Option<String>, Option<String>)> {
        let storage_type = {
            let storage_type = config.storage_type
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            storage_types
                .get(storage_type)
                .ok_or(err_msg(IndyErrorKind::UnknownWalletStorageType, "Unknown wallet storage type"))?
        };

        let storage_config = config.storage_config.as_ref().map(|value| value.to_string());
        let storage_credentials = credentials.storage_credentials.as_ref().map(|value| value.to_string());

        Ok((storage_type, storage_config, storage_credentials))
    }

    fn _is_id_from_config_not_used(&self, config: &Config) -> IndyResult<()> {
        if config.id.is_empty() {
            Err(err_msg(IndyErrorKind::InvalidStructure, "Wallet id is empty"))?
        }

        if self.wallets.borrow_mut().values().any(|ref wallet| wallet.get_id() == WalletService::_get_wallet_id(config)) {
            Err(err_msg(IndyErrorKind::WalletAlreadyOpened, format!("Wallet {} already opened", WalletService::_get_wallet_id(config))))?
        }

        Ok(())
    }

    fn _get_wallet_id(config: &Config) -> String {
        let wallet_path = config.storage_config.as_ref().and_then(|storage_config| storage_config["path"].as_str()).unwrap_or("");
        let wallet_id = format!("{}{}", config.id, wallet_path);
        wallet_id
    }

    fn _open_storage(&self, config: &Config, credentials: &Credentials) -> IndyResult<Box<WalletStorage>> {
        let storage_types = self.storage_types.borrow();
        let (storage_type, storage_config, storage_credentials) =
            WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;
        let storage = storage_type.open_storage(&config.id,
                                                storage_config.as_ref().map(String::as_str),
                                                storage_credentials.as_ref().map(String::as_str))?;
        Ok(storage)
    }

    fn _prepare_metadata(&self, master_key: &chacha20poly1305_ietf::Key, key_data: &KeyDerivationData, keys: &Keys) -> IndyResult<Vec<u8>> {
        let encrypted_keys = keys.serialize_encrypted(master_key)?;
        let metadata = match key_data {
            KeyDerivationData::Raw(_) => {
                Metadata::MetadataRaw(
                    MetadataRaw { keys: encrypted_keys }
                )
            }
            KeyDerivationData::Argon2iInt(_, salt) | KeyDerivationData::Argon2iMod(_, salt) => {
                Metadata::MetadataArgon(
                    MetadataArgon {
                        keys: encrypted_keys,
                        master_key_salt: salt[..].to_vec(),
                    }
                )
            }
        };

        let res = serde_json::to_vec(&metadata)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize wallet metadata")?;

        Ok(res)
    }

    fn _restore_keys(&self, metadata: &Metadata, master_key: &MasterKey) -> IndyResult<Keys> {
        let metadata_keys = metadata.get_keys();

        let res = Keys::deserialize_encrypted(&metadata_keys, master_key)
            .map_err(|err| err.map(IndyErrorKind::WalletAccessFailed, "Invalid master key provided"))?;

        Ok(res)
    }

    pub const PREFIX: &'static str = "Indy";

    pub fn add_prefix(&self, type_: &str) -> String {
        format!("{}::{}", WalletService::PREFIX, type_)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletRecord {
    #[serde(rename = "type")]
    type_: Option<String>,
    id: String,
    value: Option<String>,
    tags: Option<Tags>,
}

impl Ord for WalletRecord {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        (&self.type_, &self.id).cmp(&(&other.type_, &other.id))
    }
}

impl PartialOrd for WalletRecord {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        (&self.type_, &self.id).partial_cmp(&(&other.type_, &other.id))
    }
}

impl WalletRecord {
    pub fn new(name: String, type_: Option<String>, value: Option<String>, tags: Option<Tags>) -> WalletRecord {
        WalletRecord {
            id: name,
            type_,
            value,
            tags,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> Option<&str> {
        self.type_.as_ref().map(String::as_str)
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_ref().map(String::as_str)
    }

    #[allow(dead_code)]
    pub fn get_tags(&self) -> Option<&Tags> {
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
    retrieve_tags: bool,
}

impl RecordOptions {
    pub fn id() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: false,
            retrieve_tags: false,
        };

        serde_json::to_string(&options).unwrap()
    }

    pub fn id_value() -> String {
        let options = RecordOptions {
            retrieve_type: false,
            retrieve_value: true,
            retrieve_tags: false,
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
    pub fn get_total_count(&self) -> IndyResult<Option<usize>> {
        self.iter.get_total_count()
    }

    pub fn fetch_next_record(&mut self) -> IndyResult<Option<WalletRecord>> {
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
    retrieve_tags: bool,
}

impl SearchOptions {
    pub fn id_value() -> String {
        let options = SearchOptions {
            retrieve_records: true,
            retrieve_total_count: true,
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: false,
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
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    use api::INVALID_WALLET_HANDLE;

    use domain::wallet::KeyDerivationMethod;
    use utils::environment;
    use utils::inmem_wallet::InmemWallet;
    use utils::test;

    use super::*;

    impl WalletService {
        fn open_wallet(&self, config: &Config, credentials: &Credentials) -> IndyResult<WalletHandle> {
            self._is_id_from_config_not_used(config)?;

            let (storage, metadata, key_derivation_data) = self._open_storage_and_fetch_metadata(config, credentials)?;

            let wallet_handle = WalletHandle(sequence::get_next_id());

            let rekey_data: Option<KeyDerivationData> = credentials.rekey.as_ref().map(|ref rekey|
                KeyDerivationData::from_passphrase_with_new_salt(rekey, &credentials.rekey_derivation_method));

            self.pending_for_open.borrow_mut().insert(wallet_handle, (WalletService::_get_wallet_id(config), storage, metadata, rekey_data.clone()));

            let key = key_derivation_data.calc_master_key()?;

            let rekey = match rekey_data {
                Some(rekey_data) => {
                    let rekey_result = rekey_data.calc_master_key()?;
                    Some(rekey_result)
                }
                None => None
            };

            self.open_wallet_continue(wallet_handle, (&key, rekey.as_ref()))
        }

        pub fn import_wallet(&self,
                             config: &Config,
                             credentials: &Credentials,
                             export_config: &ExportConfig) -> IndyResult<()> {
            trace!("import_wallet_prepare >>> config: {:?}, credentials: {:?}, export_config: {:?}", config, secret!(export_config), secret!(export_config));

            let exported_file_to_import =
                fs::OpenOptions::new()
                    .read(true)
                    .open(&export_config.path)?;

            let (reader, import_key_derivation_data, nonce, chunk_size, header_bytes) = preparse_file_to_import(exported_file_to_import, &export_config.key)?;
            let key_data = KeyDerivationData::from_passphrase_with_new_salt(&credentials.key, &credentials.key_derivation_method);

            let wallet_handle = WalletHandle(sequence::get_next_id());

            let import_key = import_key_derivation_data.calc_master_key()?;
            let master_key = key_data.calc_master_key()?;

            self.pending_for_import.borrow_mut().insert(wallet_handle, (reader, nonce, chunk_size, header_bytes, key_data));

            self.import_wallet_continue(wallet_handle, config, credentials, (import_key, master_key))
        }

        pub fn delete_wallet(&self, config: &Config, credentials: &Credentials) -> IndyResult<()> {
            if self.wallets.borrow_mut().values().any(|ref wallet| wallet.get_id() == WalletService::_get_wallet_id(config)) {
                return Err(err_msg(IndyErrorKind::InvalidState, format!("Wallet has to be closed before deleting: {:?}", WalletService::_get_wallet_id(config))))?;
            }

            let (_, metadata, key_derivation_data) = self._open_storage_and_fetch_metadata(config, credentials)?;

            let master_key = key_derivation_data.calc_master_key()?;

            self.delete_wallet_continue(config, credentials, &metadata, &master_key)
        }
    }

    #[test]
    fn wallet_service_new_works() {
        WalletService::new();
    }

    #[test]
    fn wallet_service_register_type_works() {
        _cleanup("wallet_service_register_type_works");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        _cleanup("wallet_service_register_type_works");
    }

    #[test]
    fn wallet_service_create_wallet_works() {
        test::cleanup_wallet("wallet_service_create_wallet_works");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config_default("wallet_service_create_wallet_works"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_create_wallet_works");
    }

    #[test]
    fn wallet_service_create_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_interactive_key_derivation");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config_default("wallet_service_create_wallet_works_for_interactive_key_derivation"), &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_create_wallet_works_for_interactive_key_derivation");
    }

    #[test]
    fn wallet_service_create_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_moderate_key_derivation");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config_default("wallet_service_create_wallet_works_for_moderate_key_derivation"), &ARGON_MOD_CREDENTIAL, (&MODERATE_KDD, &MODERATE_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_create_wallet_works_for_moderate_key_derivation");
    }

    #[test]
    #[ignore]
    fn wallet_service_create_wallet_works_for_comparision_time_of_different_key_types() {
        use std::time::Instant;
        test::cleanup_wallet("wallet_service_create_wallet_works_for_comparision_time_of_different_key_types");
        {
            let wallet_service = WalletService::new();

            let config = _config_default("wallet_service_create_wallet_works_for_comparision_time_of_different_key_types");
            let time = Instant::now();
            wallet_service.create_wallet(&config, &ARGON_MOD_CREDENTIAL, (&MODERATE_KDD, &MODERATE_MASTER_KEY)).unwrap();
            let time_diff_moderate_key = time.elapsed();
            wallet_service.delete_wallet(&config, &ARGON_MOD_CREDENTIAL).unwrap();

            _cleanup("wallet_service_create_wallet_works_for_comparision_time_of_different_key_types");

            let time = Instant::now();
            wallet_service.create_wallet(&config, &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
            let time_diff_interactive_key = time.elapsed();
            wallet_service.delete_wallet(&config, &ARGON_INT_CREDENTIAL).unwrap();

            assert!(time_diff_interactive_key < time_diff_moderate_key);
        }
        test::cleanup_wallet("wallet_service_create_wallet_works_for_comparision_time_of_different_key_types");
    }

    #[test]
    fn wallet_service_create_works_for_plugged() {
        _cleanup("wallet_service_create_works_for_plugged");
        {
            let wallet_service = WalletService::new();
            _register_inmem_wallet(&wallet_service);

            wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        }
        _cleanup("wallet_service_create_works_for_plugged");
    }

    #[test]
    fn wallet_service_create_wallet_works_for_none_type() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_none_type");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_create_wallet_works_for_none_type"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_create_wallet_works_for_none_type");
    }

    #[test]
    fn wallet_service_create_wallet_works_for_unknown_type() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_unknown_type");
        {
            let wallet_service = WalletService::new();
            let res = wallet_service.create_wallet(&_config_unknown("wallet_service_create_wallet_works_for_unknown_type"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY));
            assert_kind!(IndyErrorKind::UnknownWalletStorageType, res);
        }
    }

    #[test]
    fn wallet_service_create_wallet_works_for_twice() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_twice");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_create_wallet_works_for_twice"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();

            let res = wallet_service.create_wallet(&_config("wallet_service_create_wallet_works_for_twice"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY));
            assert_kind!(IndyErrorKind::WalletAlreadyExists, res);
        }
        test::cleanup_wallet("wallet_service_create_wallet_works_for_twice");
    }
    /*
        #[test]
        fn wallet_service_create_wallet_works_for_invalid_raw_key() {
            _cleanup("wallet_service_create_wallet_works_for_invalid_raw_key");

            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_create_wallet_works_for_invalid_raw_key"), &_credentials()).unwrap();
            let res = wallet_service.create_wallet(&_config("wallet_service_create_wallet_works_for_invalid_raw_key"), &_credentials_invalid_raw());
            assert_match!(Err(IndyError::CommonError(CommonError::InvalidStructure(_))), res);
        }
    */
    #[test]
    fn wallet_service_delete_wallet_works() {
        test::cleanup_wallet("wallet_service_delete_wallet_works");
        {
            let config: &Config = &_config("wallet_service_delete_wallet_works");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            wallet_service.delete_wallet(config, &RAW_CREDENTIAL).unwrap();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_delete_wallet_works");
    }

    #[test]
    fn wallet_service_delete_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_interactive_key_derivation");
        {
            let config: &Config = &_config("wallet_service_delete_wallet_works_for_interactive_key_derivation");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
            wallet_service.delete_wallet(config, &ARGON_INT_CREDENTIAL).unwrap();
            wallet_service.create_wallet(config, &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_interactive_key_derivation");
    }

    #[test]
    fn wallet_service_delete_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_moderate_key_derivation");
        {
            let config: &Config = &_config("wallet_service_delete_wallet_works_for_moderate_key_derivation");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &ARGON_MOD_CREDENTIAL, (&MODERATE_KDD, &MODERATE_MASTER_KEY)).unwrap();
            wallet_service.delete_wallet(config, &ARGON_MOD_CREDENTIAL).unwrap();
            wallet_service.create_wallet(config, &ARGON_MOD_CREDENTIAL, (&MODERATE_KDD, &MODERATE_MASTER_KEY)).unwrap();
        }
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_moderate_key_derivation");
    }

    #[test]
    fn wallet_service_delete_works_for_plugged() {
        test::cleanup_wallet("wallet_service_delete_works_for_plugged");

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        wallet_service.delete_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();
        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
    }

    #[test]
    fn wallet_service_delete_wallet_returns_error_if_wallet_opened() {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_wallet_opened");
        {
            let config: &Config = &_config("wallet_service_delete_wallet_returns_error_if_wallet_opened");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            let res = wallet_service.delete_wallet(config, &RAW_CREDENTIAL);
            assert_eq!(IndyErrorKind::InvalidState, res.unwrap_err().kind());
        }
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_wallet_opened");
    }

    #[test]
    fn wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method() {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");
        {
            let config: &Config = &_config("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();

            let res = wallet_service.delete_wallet(config, &ARGON_INT_CREDENTIAL);
            assert_eq!(IndyErrorKind::WalletAccessFailed, res.unwrap_err().kind());
        }
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");
    }

    #[test]
    fn wallet_service_delete_wallet_returns_error_for_nonexistant_wallet() {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_for_nonexistant_wallet");

        let wallet_service = WalletService::new();

        let res = wallet_service.delete_wallet(&_config("wallet_service_delete_wallet_returns_error_for_nonexistant_wallet"), &RAW_CREDENTIAL);
        assert_eq!(IndyErrorKind::WalletNotFound, res.unwrap_err().kind());
    }

    #[test]
    fn wallet_service_open_wallet_works() {
        test::cleanup_wallet("wallet_service_open_wallet_works");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_open_wallet_works"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let handle = wallet_service.open_wallet(&_config("wallet_service_open_wallet_works"), &RAW_CREDENTIAL).unwrap();

            // cleanup
            wallet_service.close_wallet(handle).unwrap();
        }
        test::cleanup_wallet("wallet_service_open_wallet_works");
    }

    #[test]
    fn wallet_service_open_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_open_wallet_works_for_interactive_key_derivation");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_open_wallet_works_for_interactive_key_derivation"), &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
            let handle = wallet_service.open_wallet(&_config("wallet_service_open_wallet_works_for_interactive_key_derivation"), &ARGON_INT_CREDENTIAL).unwrap();

            // cleanup
            wallet_service.close_wallet(handle).unwrap();
        }
        test::cleanup_wallet("wallet_service_open_wallet_works_for_interactive_key_derivation");
    }

    #[test]
    fn wallet_service_open_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_open_wallet_works_for_moderate_key_derivation");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_open_wallet_works_for_moderate_key_derivation"), &ARGON_MOD_CREDENTIAL, (&MODERATE_KDD, &MODERATE_MASTER_KEY)).unwrap();
            let handle = wallet_service.open_wallet(&_config("wallet_service_open_wallet_works_for_moderate_key_derivation"), &ARGON_MOD_CREDENTIAL).unwrap();

            // cleanup
            wallet_service.close_wallet(handle).unwrap();
        }
        test::cleanup_wallet("wallet_service_open_wallet_works_for_moderate_key_derivation");
    }

    #[test]
    fn wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths() {
        _cleanup("wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths");

        let wallet_service = WalletService::new();

        let config_1 = Config {
            id: String::from("same_id"),
            storage_type: None,
            storage_config: None,
        };

        wallet_service.create_wallet(&config_1, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let handle_1 = wallet_service.open_wallet(&config_1, &RAW_CREDENTIAL).unwrap();

        let config_2 = Config {
            id: String::from("same_id"),
            storage_type: None,
            storage_config: Some(json!({
                "path": _custom_path()
            })),
        };

        wallet_service.create_wallet(&config_2, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let handle_2 = wallet_service.open_wallet(&config_2, &RAW_CREDENTIAL).unwrap();

        // cleanup
        wallet_service.close_wallet(handle_1).unwrap();
        wallet_service.close_wallet(handle_2).unwrap();

        wallet_service.delete_wallet(&config_1, &RAW_CREDENTIAL).unwrap();
        wallet_service.delete_wallet(&config_2, &RAW_CREDENTIAL).unwrap();
    }

    #[test]
    fn wallet_service_open_unknown_wallet() {
        test::cleanup_wallet("wallet_service_open_unknown_wallet");

        let wallet_service = WalletService::new();
        let res = wallet_service.open_wallet(&_config("wallet_service_open_unknown_wallet"), &RAW_CREDENTIAL);
        assert_eq!(IndyErrorKind::WalletNotFound, res.unwrap_err().kind());
    }

    #[test]
    fn wallet_service_open_wallet_returns_appropriate_error_if_already_opened() {
        test::cleanup_wallet("wallet_service_open_wallet_returns_appropriate_error_if_already_opened");
        {
            let config: &Config = &_config("wallet_service_open_wallet_returns_appropriate_error_if_already_opened");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL);
            assert_eq!(IndyErrorKind::WalletAlreadyOpened, res.unwrap_err().kind());
        }
        test::cleanup_wallet("wallet_service_open_wallet_returns_appropriate_error_if_already_opened");
    }

    #[test]
    fn wallet_service_open_works_for_plugged() {
        _cleanup("wallet_service_open_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();
    }

    #[test]
    fn wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening() {
        test::cleanup_wallet("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();

            let res = wallet_service.open_wallet(&_config("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening"), &ARGON_INT_CREDENTIAL);
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);
        }
        test::cleanup_wallet("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening");
    }

    #[test]
    fn wallet_service_close_wallet_works() {
        test::cleanup_wallet("wallet_service_close_wallet_works");
        {
            let config: &Config = &_config("wallet_service_close_wallet_works");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();
            wallet_service.close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();
            wallet_service.close_wallet(wallet_handle).unwrap();
        }
        test::cleanup_wallet("wallet_service_close_wallet_works");
    }

    #[test]
    fn wallet_service_close_works_for_plugged() {
        _cleanup("wallet_service_close_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();
        wallet_service.close_wallet(wallet_handle).unwrap();
    }

    #[test]
    fn wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle() {
        test::cleanup_wallet("wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle"), &RAW_CREDENTIAL).unwrap();

            let res = wallet_service.close_wallet(INVALID_WALLET_HANDLE);
            assert_kind!(IndyErrorKind::InvalidWalletHandle, res);

            wallet_service.close_wallet(wallet_handle).unwrap();
        }
        test::cleanup_wallet("wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle");
    }

    #[test]
    fn wallet_service_add_record_works() {
        test::cleanup_wallet("wallet_service_add_record_works");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_add_record_works"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_add_record_works"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        test::cleanup_wallet("wallet_service_add_record_works");
    }

    #[test]
    fn wallet_service_add_record_works_for_plugged() {
        _cleanup("wallet_service_add_record_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
    }

    #[test]
    fn wallet_service_get_record_works_for_id_only() {
        test::cleanup_wallet("wallet_service_get_record_works_for_id_only");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_get_record_works_for_id_only"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_get_record_works_for_id_only"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, false, false)).unwrap();

            assert!(record.get_value().is_none());
            assert!(record.get_type().is_none());
            assert!(record.get_tags().is_none());
        }
        test::cleanup_wallet("wallet_service_get_record_works_for_id_only");
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_id_only() {
        test::cleanup_indy_home("wallet_service_get_record_works_for_plugged_for_id_only");
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, false, false)).unwrap();

        assert!(record.get_value().is_none());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_id_value() {
        test::cleanup_wallet("wallet_service_get_record_works_for_id_value");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_get_record_works_for_id_value"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_get_record_works_for_id_value"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();

            assert_eq!("value1", record.get_value().unwrap());
            assert!(record.get_type().is_none());
            assert!(record.get_tags().is_none());
        }
        test::cleanup_wallet("wallet_service_get_record_works_for_id_value");
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_id_value() {
        _cleanup("wallet_service_get_record_works_for_plugged_for_id_value");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();

        assert_eq!("value1", record.get_value().unwrap());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[test]
    fn wallet_service_get_record_works_for_all_fields() {
        test::cleanup_wallet("wallet_service_get_record_works_for_all_fields");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_get_record_works_for_all_fields"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_get_record_works_for_all_fields"), &RAW_CREDENTIAL).unwrap();
            let mut tags = HashMap::new();
            tags.insert(String::from("1"), String::from("some"));

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &tags).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            assert_eq!(&tags, record.get_tags().unwrap());
        }
        test::cleanup_wallet("wallet_service_get_record_works_for_all_fields");
    }

    #[test]
    fn wallet_service_get_record_works_for_plugged_for_for_all_fields() {
        _cleanup("wallet_service_get_record_works_for_plugged_for_for_all_fields");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();
        let tags = serde_json::from_str(r#"{"1":"some"}"#).unwrap();

        wallet_service.add_record(wallet_handle, "type", "key1", "value1", &tags).unwrap();
        let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();

        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        assert_eq!(tags, record.get_tags().unwrap().clone());
    }

    #[test]
    fn wallet_service_add_get_works_for_reopen() {
        test::cleanup_wallet("wallet_service_add_get_works_for_reopen");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_add_get_works_for_reopen"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_add_get_works_for_reopen"), &RAW_CREDENTIAL).unwrap();
            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_add_get_works_for_reopen"), &RAW_CREDENTIAL).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false)).unwrap();
            assert_eq!("value1", record.get_value().unwrap());
        }
        test::cleanup_wallet("wallet_service_add_get_works_for_reopen");
    }

    #[test]
    fn wallet_service_get_works_for_unknown() {
        test::cleanup_wallet("wallet_service_get_works_for_unknown");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_get_works_for_unknown"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_get_works_for_unknown"), &RAW_CREDENTIAL).unwrap();

            let res = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false));
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }
        test::cleanup_wallet("wallet_service_get_works_for_unknown");
    }

    #[test]
    fn wallet_service_get_works_for_plugged_and_unknown() {
        _cleanup("wallet_service_get_works_for_plugged_and_unknown");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        let res = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(false, true, false));
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    /**
     * Update tests
    */
    #[test]
    fn wallet_service_update() {
        test::cleanup_wallet("wallet_service_update");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let new_value = "new_value";

            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_update"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_update"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
            assert_eq!(value, record.get_value().unwrap());

            wallet_service.update_record_value(wallet_handle, type_, name, new_value).unwrap();
            let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
            assert_eq!(new_value, record.get_value().unwrap());
        }
        test::cleanup_wallet("wallet_service_update");
    }

    #[test]
    fn wallet_service_update_for_plugged() {
        _cleanup("wallet_service_update_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

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
        test::cleanup_wallet("wallet_service_delete_record");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";

            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_delete_record"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_delete_record"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
            assert_eq!(value, record.get_value().unwrap());

            wallet_service.delete_record(wallet_handle, type_, name).unwrap();
            let res = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false));
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }
        test::cleanup_wallet("wallet_service_delete_record");
    }

    #[test]
    fn wallet_service_delete_record_for_plugged() {
        _cleanup("wallet_service_delete_record_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &HashMap::new()).unwrap();
        let record = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false)).unwrap();
        assert_eq!(value, record.get_value().unwrap());

        wallet_service.delete_record(wallet_handle, type_, name).unwrap();
        let res = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(false, true, false));
        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    /**
     * Add tags tests
     */
    #[test]
    fn wallet_service_add_tags() {
        test::cleanup_wallet("wallet_service_add_tags");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();

            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_add_tags"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_add_tags"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

            let new_tags = serde_json::from_str(r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            wallet_service.add_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

            let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

            let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }
        test::cleanup_wallet("wallet_service_add_tags");
    }

    #[test]
    fn wallet_service_add_tags_for_plugged() {
        _cleanup("wallet_service_add_tags_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";
        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

        wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

        let new_tags = serde_json::from_str(r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        wallet_service.add_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

        let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

        let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    /**
     * Update tags tests
     */
    #[test]
    fn wallet_service_update_tags() {
        test::cleanup_wallet("wallet_service_update_tags");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let wallet_service = WalletService::new();

            wallet_service.create_wallet(&_config("wallet_service_update_tags"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_update_tags"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

            let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            wallet_service.update_record_tags(wallet_handle, type_, name, &new_tags).unwrap();
            let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(new_tags, retrieved_tags);
        }
        test::cleanup_wallet("wallet_service_update_tags");
    }

    #[test]
    fn wallet_service_update_tags_for_plugged() {
        _cleanup("wallet_service_update_tags_for_plugged");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let wallet_service = WalletService::new();

            _register_inmem_wallet(&wallet_service);

            wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

            let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            wallet_service.update_record_tags(wallet_handle, type_, name, &new_tags).unwrap();

            let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(new_tags, retrieved_tags);
        }
        _cleanup("wallet_service_update_tags_for_plugged");
    }

    /**
     * Delete tags tests
     */
    #[test]
    fn wallet_service_delete_tags() {
        test::cleanup_wallet("wallet_service_delete_tags");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            let wallet_service = WalletService::new();

            wallet_service.create_wallet(&_config("wallet_service_delete_tags"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_delete_tags"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

            let tag_names = vec!["tag_name_1", "~tag_name_3"];
            wallet_service.delete_record_tags(wallet_handle, type_, name, &tag_names).unwrap();

            let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

            let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }
        test::cleanup_wallet("wallet_service_delete_tags");
    }


    #[test]
    fn wallet_service_delete_tags_for_plugged() {
        _cleanup("wallet_service_delete_tags_for_plugged");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            let wallet_service = WalletService::new();
            _register_inmem_wallet(&wallet_service);

            wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, type_, name, value, &tags).unwrap();

            let tag_names = vec!["tag_name_1", "~tag_name_3"];
            wallet_service.delete_record_tags(wallet_handle, type_, name, &tag_names).unwrap();

            let item = wallet_service.get_record(wallet_handle, type_, name, &_fetch_options(true, true, true)).unwrap();

            let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }
        _cleanup("wallet_service_delete_tags_for_plugged");
    }

    #[test]
    fn wallet_service_search_records_works() {
        test::cleanup_wallet("wallet_service_search_records_works");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_search_records_works"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_search_records_works"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.add_record(wallet_handle, "type", "key2", "value2", &HashMap::new()).unwrap();
            wallet_service.add_record(wallet_handle, "type3", "key3", "value3", &HashMap::new()).unwrap();

            let mut search = wallet_service.search_records(wallet_handle, "type3", "{}", &_fetch_options(true, true, true)).unwrap();

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("value3", record.get_value().unwrap());
            assert_eq!(HashMap::new(), record.get_tags().unwrap().clone());

            assert!(search.fetch_next_record().unwrap().is_none());
        }
        test::cleanup_wallet("wallet_service_search_records_works");
    }

    #[test]
    fn wallet_service_search_records_works_for_plugged_wallet() {
        _cleanup("wallet_service_search_records_works_for_plugged_wallet");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service.create_wallet(&_config_inmem(), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
        let wallet_handle = wallet_service.open_wallet(&_config_inmem(), &RAW_CREDENTIAL).unwrap();

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
        test::cleanup_wallet("wallet_service_key_rotation");
        {
            let config: &Config = &_config("wallet_service_key_rotation");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet_service.open_wallet(config, &_rekey_credentials_moderate()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL);
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service.open_wallet(config, &_credentials_for_new_key_moderate()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }
        test::cleanup_wallet("wallet_service_key_rotation");
    }

    #[test]
    fn wallet_service_key_rotation_for_rekey_interactive_method() {
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_interactive_method");
        {
            let config: &Config = &_config("wallet_service_key_rotation_for_rekey_interactive_method");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet_service.open_wallet(config, &_rekey_credentials_interactive()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL);
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service.open_wallet(config, &_credentials_for_new_key_interactive()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_interactive_method");
    }

    #[test]
    fn wallet_service_key_rotation_for_rekey_raw_method() {
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_raw_method");
        {
            let config: &Config = &_config("wallet_service_key_rotation_for_rekey_raw_method");
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet_service.open_wallet(config, &_rekey_credentials_raw()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL);
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service.open_wallet(config, &_credentials_for_new_key_raw()).unwrap();
            let record = wallet_service.get_record(wallet_handle, "type", "key1", &_fetch_options(true, true, true)).unwrap();
            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_raw_method");
    }

    fn remove_exported_wallet(export_config: &ExportConfig) -> &Path {
        let export_path = Path::new(&export_config.path );
        if export_path.exists() {
            fs::remove_file(export_path).unwrap();
        }
        export_path
    }

    #[test]
    fn wallet_service_export_wallet_when_empty() {
        test::cleanup_wallet("wallet_service_export_wallet_when_empty");
        let export_config = _export_config_raw("export_wallet_service_export_wallet_when_empty");
        {
            let wallet_service = WalletService::new();
            let wallet_config = _config("wallet_service_export_wallet_when_empty");
            wallet_service.create_wallet(&wallet_config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_when_empty"), &RAW_CREDENTIAL).unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_when_empty");
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();

            assert!(export_path.exists());
        }
        remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_when_empty");
    }

    #[test]
    fn wallet_service_export_wallet_1_item() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item");
        let export_config = _export_config_raw("export_config_wallet_service_export_wallet_1_item");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_wallet_1_item"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_1_item"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_1_item");
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item");
    }

    #[test]
    fn wallet_service_export_wallet_1_item_interactive_method() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item_interactive_method");
        let export_config = _export_config_interactive("wallet_service_export_wallet_1_item_interactive_method");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_wallet_1_item_interactive_method"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_1_item_interactive_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key_interactive("wallet_service_export_wallet_1_item_interactive_method");
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item_interactive_method");
    }

    #[test]
    fn wallet_service_export_wallet_1_item_raw_method() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item_raw_method");
        let export_config = _export_config_raw("wallet_service_export_wallet_1_item_raw_method");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_wallet_1_item_raw_method"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_1_item_raw_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key("wallet_service_export_wallet_1_item_raw_method");
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(&export_path.exists());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item_raw_method");
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_file_exists() {
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_file_exists");

        {
            fs::create_dir_all(_export_file_path("wallet_service_export_wallet_returns_error_if_file_exists").parent().unwrap()).unwrap();
            fs::File::create(_export_file_path("wallet_service_export_wallet_returns_error_if_file_exists")).unwrap();
        }

        assert!(_export_file_path("wallet_service_export_wallet_returns_error_if_file_exists").exists());

        let export_config = _export_config_raw("wallet_service_export_wallet_returns_error_if_file_exists");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_wallet_returns_error_if_file_exists"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_returns_error_if_file_exists"), &RAW_CREDENTIAL).unwrap();

            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_returns_error_if_file_exists");
            let res = wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key));
            assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_file_exists");
    }

    #[test]
    fn wallet_service_export_wallet_returns_error_if_wrong_handle() {
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_wrong_handle");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_wallet_returns_error_if_wrong_handle"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let _wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_wallet_returns_error_if_wrong_handle"), &RAW_CREDENTIAL).unwrap();

            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_returns_error_if_wrong_handle");
            let export_config = _export_config_raw("wallet_service_export_wallet_returns_error_if_wrong_handle");
            let export_path = remove_exported_wallet(&export_config);
            let res = wallet_service.export_wallet(INVALID_WALLET_HANDLE, &export_config, 0, (&kdd, &master_key));
            assert_kind!(IndyErrorKind::InvalidWalletHandle, res);
            assert!(!export_path.exists());
        }
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_wrong_handle");
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item");
        let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_import_wallet_1_item"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_import_wallet_1_item");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(&_config("wallet_service_export_import_wallet_1_item"), &RAW_CREDENTIAL).unwrap();

            let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item");
            wallet_service.import_wallet(&_config("wallet_service_export_import_wallet_1_item"), &RAW_CREDENTIAL, &export_config).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item"), &RAW_CREDENTIAL).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item");
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item_for_interactive_method() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_interactive_method");
        let export_config = _export_config_interactive("wallet_service_export_import_wallet_1_item_for_interactive_method");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_import_wallet_1_item_for_interactive_method"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item_for_interactive_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let (kdd, master_key) = _export_key_interactive("wallet_service_export_import_wallet_1_item_for_interactive_method");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(&_config("wallet_service_export_import_wallet_1_item_for_interactive_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.import_wallet(&_config("wallet_service_export_import_wallet_1_item_for_interactive_method"), &RAW_CREDENTIAL, &_export_config_interactive("wallet_service_export_import_wallet_1_item_for_interactive_method")).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item_for_interactive_method"), &RAW_CREDENTIAL).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_interactive_method");
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item_for_moderate_method() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_moderate_method");
        let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item_for_moderate_method");
        {
            let wallet_service = WalletService::new();
            wallet_service.create_wallet(&_config("wallet_service_export_import_wallet_1_item_for_moderate_method"), &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item_for_moderate_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_import_wallet_1_item_for_moderate_method");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(&_config("wallet_service_export_import_wallet_1_item_for_moderate_method"), &RAW_CREDENTIAL).unwrap();

            wallet_service.import_wallet(&_config("wallet_service_export_import_wallet_1_item_for_moderate_method"), &ARGON_MOD_CREDENTIAL, &export_config).unwrap();
            let wallet_handle = wallet_service.open_wallet(&_config("wallet_service_export_import_wallet_1_item_for_moderate_method"), &ARGON_MOD_CREDENTIAL).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_moderate_method");
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw");
        let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw");
        {
            let wallet_service = WalletService::new();
            let config: &Config = &_config("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw");
            wallet_service.create_wallet(config, &ARGON_INT_CREDENTIAL, (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &ARGON_INT_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let (kdd, master_key) = _export_key_interactive("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(config, &ARGON_INT_CREDENTIAL).unwrap();

            wallet_service.import_wallet(config, &ARGON_MOD_CREDENTIAL, &_export_config_moderate("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw")).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &ARGON_MOD_CREDENTIAL).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw");
    }

    #[test]
    fn wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive");
        let export_config = _export_config_interactive("wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive");
        {
            let wallet_service = WalletService::new();
            let config: &Config = &_config("wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive");
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.add_record(wallet_handle, "type", "key1", "value1", &HashMap::new()).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();

            let (kdd, master_key) = _export_key_interactive("wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.import_wallet(config, &ARGON_INT_CREDENTIAL, &export_config).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &ARGON_INT_CREDENTIAL).unwrap();
            wallet_service.get_record(wallet_handle, "type", "key1", "{}").unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive");
    }

    #[test]
    fn wallet_service_export_import_wallet_if_empty() {
        test::cleanup_wallet("wallet_service_export_import_wallet_if_empty");
        let export_config = _export_config_raw("wallet_service_export_import_wallet_if_empty");
        {
            let wallet_service = WalletService::new();
            let config: &Config = &_config("wallet_service_export_import_wallet_if_empty");
            wallet_service.create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY)).unwrap();
            let wallet_handle = wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();

            let (kdd, master_key) = _export_key("wallet_service_export_import_wallet_if_empty");
            let export_path = remove_exported_wallet(&export_config);
            wallet_service.export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key)).unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).unwrap();
            wallet_service.delete_wallet(config, &RAW_CREDENTIAL).unwrap();

            wallet_service.import_wallet(config, &RAW_CREDENTIAL, &export_config).unwrap();
            wallet_service.open_wallet(config, &RAW_CREDENTIAL).unwrap();
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_if_empty");
    }

    #[test]
    fn wallet_service_export_import_returns_error_if_path_missing() {
        _cleanup("wallet_service_export_import_returns_error_if_path_missing");

        let wallet_service = WalletService::new();
        let config : &Config = &_config("wallet_service_export_import_returns_error_if_path_missing");
        let export_config =_export_config_raw("wallet_service_export_import_returns_error_if_path_missing");
        let res = wallet_service.import_wallet(config, &RAW_CREDENTIAL, &export_config);
        assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());

        let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL);
        assert_match!(Err(_), res);

        _cleanup("wallet_service_export_import_returns_error_if_path_missing");
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
          "retrieveType": type_,
          "retrieveValue": value,
          "retrieveTags": tags,
        }).to_string()
    }

    fn _config(name: &str) -> Config {
        Config {
            id: name.to_string(),
            storage_type: None,
            storage_config: None,
        }
    }

    fn _config_default(name: &str) -> Config {
        Config {
            id: name.to_string(),
            storage_type: Some("default".to_string()),
            storage_config: None,
        }
    }

    fn _config_inmem() -> Config {
        Config {
            id: "w1".to_string(),
            storage_type: Some("inmem".to_string()),
            storage_config: None,
        }
    }

    fn _config_unknown(name: &str) -> Config {
        Config {
            id: name.to_string(),
            storage_type: Some("unknown".to_string()),
            storage_config: None,
        }
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref ARGON_MOD_CREDENTIAL: Credentials = Credentials {
            key: "my_key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
        };
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref ARGON_INT_CREDENTIAL: Credentials = Credentials {
            key: "my_key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::ARGON2I_INT,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_INT,
        };
    }


    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref RAW_CREDENTIAL: Credentials = Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW,
        };
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref MODERATE_KDD: KeyDerivationData = KeyDerivationData::from_passphrase_with_new_salt("my_key", &KeyDerivationMethod::ARGON2I_MOD);
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref MODERATE_MASTER_KEY: MasterKey =  MODERATE_KDD.calc_master_key().unwrap();
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref INTERACTIVE_KDD: KeyDerivationData = KeyDerivationData::from_passphrase_with_new_salt("my_key", &KeyDerivationMethod::ARGON2I_INT);
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref INTERACTIVE_MASTER_KEY: MasterKey =  INTERACTIVE_KDD.calc_master_key().unwrap();
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref RAW_KDD: KeyDerivationData = KeyDerivationData::from_passphrase_with_new_salt("6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw", &KeyDerivationMethod::RAW);
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref RAW_MASTER_KEY: MasterKey =  RAW_KDD.calc_master_key().unwrap();
    }

    fn _credentials_invalid_raw() -> Credentials {
        Credentials {
            key: "key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW,
        }
    }

    fn _rekey_credentials_moderate() -> Credentials {
        Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: Some("my_new_key".to_string()),
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
        }
    }

    fn _rekey_credentials_interactive() -> Credentials {
        Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: Some("my_new_key".to_string()),
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_INT,
        }
    }

    fn _rekey_credentials_raw() -> Credentials {
        Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: Some("7nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string()),
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW,
        }
    }

    fn _credentials_for_new_key_moderate() -> Credentials {
        Credentials {
            key: "my_new_key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
        }
    }

    fn _credentials_for_new_key_interactive() -> Credentials {
        Credentials {
            key: "my_new_key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::ARGON2I_INT,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_INT,
        }
    }

    fn _credentials_for_new_key_raw() -> Credentials {
        Credentials {
            key: "7nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW,
        }
    }

    fn _export_file_path(name: &str) -> PathBuf {
        let mut path = environment::tmp_file_path("export_tests");
        path.push(name);
        path
    }

    fn _export_config_moderate(name: &str) -> ExportConfig {
        ExportConfig {
            key: "export_key".to_string(),
            path: _export_file_path(name).to_str().unwrap().to_string(),
            key_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
        }
    }

    fn _calc_key(export_config: &ExportConfig) -> (KeyDerivationData, MasterKey) {
        let kdd = KeyDerivationData::from_passphrase_with_new_salt(&export_config.key, &export_config.key_derivation_method);
        let master_key = kdd.calc_master_key().unwrap();
        (kdd, master_key)
    }

    fn _export_key(name: &str) -> (KeyDerivationData, MasterKey) {
        _calc_key(&_export_config_raw(name))
    }

    fn _export_config_interactive(name: &str) -> ExportConfig {
        ExportConfig {
            key: "export_key".to_string(),
            path: _export_file_path(name).to_str().unwrap().to_string(),
            key_derivation_method: KeyDerivationMethod::ARGON2I_INT,
        }
    }

    fn _export_key_interactive(name: &str) -> (KeyDerivationData, MasterKey) {
        _calc_key(&_export_config_interactive(name))
    }

    fn _export_config_raw(name: &str) -> ExportConfig {
        ExportConfig {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            path: _export_file_path(name).to_str().unwrap().to_string(),
            key_derivation_method: KeyDerivationMethod::RAW,
        }
    }

    fn _export_key_raw(name: &str) -> (KeyDerivationData, MasterKey) {
        _calc_key(&_export_config_raw(name))
    }

    fn _cleanup(name: &str) {
        test::cleanup_storage(name);
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
                InmemWallet::free_search,
            )
            .unwrap();
    }

    fn _custom_path() -> String {
        let mut path = environment::tmp_path();
        path.push("custom_wallet_path");
        path.to_str().unwrap().to_owned()
    }
}
