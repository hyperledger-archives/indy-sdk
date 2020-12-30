use std::{
    collections::{HashMap, HashSet},
    fs,
    io::BufReader,
    path::PathBuf,
    sync::Arc,
    unimplemented,
};

use futures::lock::Mutex;
use indy_api_types::{
    domain::wallet::{Config, Credentials, ExportConfig, Tags},
    errors::prelude::*,
    wallet::*,
    WalletHandle,
};
use indy_utils::{
    crypto::chacha20poly1305_ietf::{self, Key as MasterKey},
    secret,
};
use log::trace;
use serde::{Deserialize, Serialize};
use serde_json::Value as SValue;

use crate::{
    export_import::{export_continue, finish_import, preparse_file_to_import},
    storage::{
        default::SQLiteStorageType, mysql::MySqlStorageType, WalletStorage, WalletStorageType,
    },
    wallet::{Keys, Wallet},
};
pub use crate::encryption::KeyDerivationData;

//use crate::storage::plugged::PluggedStorageType; FXIME:

mod encryption;
mod iterator;
mod query_encryption;
mod storage;

// TODO: Remove query language out of wallet module
pub mod language;

mod export_import;
mod wallet;

pub struct WalletService {
    storage_types: Mutex<HashMap<String, Box<dyn WalletStorageType>>>,
    wallets: Mutex<HashMap<WalletHandle, Arc<Wallet>>>,
    wallet_ids: Mutex<HashSet<String>>,
    pending_for_open: Mutex<
        HashMap<
            WalletHandle,
            (
                String, /* id */
                Box<dyn WalletStorage>,
                Metadata,
                Option<KeyDerivationData>,
            ),
        >,
    >,
    pending_for_import: Mutex<
        HashMap<
            WalletHandle,
            (
                BufReader<::std::fs::File>,
                chacha20poly1305_ietf::Nonce,
                usize,
                Vec<u8>,
                KeyDerivationData,
            ),
        >,
    >,
}

impl WalletService {
    pub fn new() -> WalletService {
        let storage_types = {
            let mut map: HashMap<String, Box<dyn WalletStorageType>> = HashMap::new();
            map.insert("default".to_string(), Box::new(SQLiteStorageType::new()));
            map.insert("mysql".to_string(), Box::new(MySqlStorageType::new()));
            Mutex::new(map)
        };

        WalletService {
            storage_types,
            wallets: Mutex::new(HashMap::new()),
            wallet_ids: Mutex::new(HashSet::new()),
            pending_for_open: Mutex::new(HashMap::new()),
            pending_for_import: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_wallet_storage(
        &self,
        type_: &str,
        _create: WalletCreate,
        _open: WalletOpen,
        _close: WalletClose,
        _delete: WalletDelete,
        _add_record: WalletAddRecord,
        _update_record_value: WalletUpdateRecordValue,
        _update_record_tags: WalletUpdateRecordTags,
        _add_record_tags: WalletAddRecordTags,
        _delete_record_tags: WalletDeleteRecordTags,
        _delete_record: WalletDeleteRecord,
        _get_record: WalletGetRecord,
        _get_record_id: WalletGetRecordId,
        _get_record_type: WalletGetRecordType,
        _get_record_value: WalletGetRecordValue,
        _get_record_tags: WalletGetRecordTags,
        _free_record: WalletFreeRecord,
        _get_storage_metadata: WalletGetStorageMetadata,
        _set_storage_metadata: WalletSetStorageMetadata,
        _free_storage_metadata: WalletFreeStorageMetadata,
        _search_records: WalletSearchRecords,
        _search_all_records: WalletSearchAllRecords,
        _get_search_total_count: WalletGetSearchTotalCount,
        _fetch_search_next_record: WalletFetchSearchNextRecord,
        _free_search: WalletFreeSearch,
    ) -> IndyResult<()> {
        trace!("register_wallet_storage >>> type_: {:?}", type_);
        Ok(()) // FIXME: !!!

        //         let mut storage_types = self.storage_types.lock().await;

        //         if storage_types.contains_key(type_) {
        //             return Err(err_msg(IndyErrorKind::WalletStorageTypeAlreadyRegistered, format!("Wallet storage is already registered for type: {}", type_)));
        //         }

        //         storage_types.insert(type_.to_string(),
        //                              Box::new(
        //                                  PluggedStorageType::new(create, open, close, delete,
        //                                                          add_record, update_record_value,
        //                                                          update_record_tags, add_record_tags, delete_record_tags,
        //                                                          delete_record, get_record, get_record_id,
        //                                                          get_record_type, get_record_value, get_record_tags, free_record,
        //                                                          get_storage_metadata, set_storage_metadata, free_storage_metadata,
        //                                                          search_records, search_all_records,
        //                                                          get_search_total_count,
        //                                                          fetch_search_next_record, free_search)));

        //         trace!("register_wallet_storage <<<");
        //         Ok(())
    }

    pub async fn create_wallet(
        &self,
        config: &Config,
        credentials: &Credentials,
        key: (&KeyDerivationData, &MasterKey),
    ) -> IndyResult<()> {
        self._create_wallet(config, credentials, key).await?;
        Ok(())
    }

    async fn _create_wallet(
        &self,
        config: &Config,
        credentials: &Credentials,
        (key_data, master_key): (&KeyDerivationData, &MasterKey),
    ) -> IndyResult<Keys> {
        trace!(
            "create_wallet >>> config: {:?}, credentials: {:?}",
            config,
            secret!(credentials)
        );

        let storage_types = self.storage_types.lock().await;

        let (storage_type, storage_config, storage_credentials) =
            WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;

        let keys = Keys::new();
        let metadata = self._prepare_metadata(master_key, key_data, &keys)?;

        storage_type
            .create_storage(
                &config.id,
                storage_config.as_ref().map(String::as_str),
                storage_credentials.as_ref().map(String::as_str),
                &metadata,
            )
            .await?;

        Ok(keys)
    }

    pub async fn delete_wallet_prepare(
        &self,
        config: &Config,
        credentials: &Credentials,
    ) -> IndyResult<(Metadata, KeyDerivationData)> {
        trace!(
            "delete_wallet >>> config: {:?}, credentials: {:?}",
            config,
            secret!(credentials)
        );

        if self
            .wallet_ids
            .lock()
            .await
            .contains(&WalletService::_get_wallet_id(config))
        {
            return Err(err_msg(
                IndyErrorKind::InvalidState,
                format!(
                    "Wallet has to be closed before deleting: {:?}",
                    WalletService::_get_wallet_id(config)
                ),
            ));
        }

        // check credentials and close connection before deleting wallet

        let (_, metadata, key_derivation_data) = self
            ._open_storage_and_fetch_metadata(config, &credentials)
            .await?;

        Ok((metadata, key_derivation_data))
    }

    pub async fn delete_wallet_continue(
        &self,
        config: &Config,
        credentials: &Credentials,
        metadata: &Metadata,
        master_key: &MasterKey,
    ) -> IndyResult<()> {
        trace!(
            "delete_wallet >>> config: {:?}, credentials: {:?}",
            config,
            secret!(credentials)
        );

        {
            self._restore_keys(metadata, &master_key)?;
        }

        let storage_types = self.storage_types.lock().await;

        let (storage_type, storage_config, storage_credentials) =
            WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;

        storage_type
            .delete_storage(
                &config.id,
                storage_config.as_ref().map(String::as_str),
                storage_credentials.as_ref().map(String::as_str),
            )
            .await?;

        trace!("delete_wallet <<<");
        Ok(())
    }

    pub async fn open_wallet_prepare(
        &self,
        config: &Config,
        credentials: &Credentials,
    ) -> IndyResult<(WalletHandle, KeyDerivationData, Option<KeyDerivationData>)> {
        trace!(
            "open_wallet >>> config: {:?}, credentials: {:?}",
            config,
            secret!(&credentials)
        );

        self._is_id_from_config_not_used(config).await?;

        let (storage, metadata, key_derivation_data) = self
            ._open_storage_and_fetch_metadata(config, credentials)
            .await?;

        let wallet_handle = indy_utils::next_wallet_handle();

        let rekey_data: Option<KeyDerivationData> = credentials.rekey.as_ref().map(|ref rekey| {
            KeyDerivationData::from_passphrase_with_new_salt(
                rekey,
                &credentials.rekey_derivation_method,
            )
        });

        self.pending_for_open.lock().await.insert(
            wallet_handle,
            (
                WalletService::_get_wallet_id(config),
                storage,
                metadata,
                rekey_data.clone(),
            ),
        );

        Ok((wallet_handle, key_derivation_data, rekey_data))
    }

    pub async fn open_wallet_continue(
        &self,
        wallet_handle: WalletHandle,
        master_key: (&MasterKey, Option<&MasterKey>),
    ) -> IndyResult<WalletHandle> {
        let (id, storage, metadata, rekey_data) = self
            .pending_for_open
            .lock().await
            .remove(&wallet_handle)
            .ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "Open data not found"))?;

        let (master_key, rekey) = master_key;
        let keys = self._restore_keys(&metadata, &master_key)?;

        // Rotate master key
        if let (Some(rekey), Some(rekey_data)) = (rekey, rekey_data) {
            let metadata = self._prepare_metadata(rekey, &rekey_data, &keys)?;
            storage.set_storage_metadata(&metadata).await?;
        }

        let wallet = Wallet::new(id.clone(), storage, Arc::new(keys));

        {
            let mut wallets = self.wallets.lock().await;
            wallets.insert(wallet_handle, Arc::new(wallet));
        }

        {
            let mut wallet_ids = self.wallet_ids.lock().await;
            wallet_ids.insert(id.to_string());
        }

        trace!("open_wallet <<< res: {:?}", wallet_handle);
        Ok(wallet_handle)
    }

    async fn _open_storage_and_fetch_metadata(
        &self,
        config: &Config,
        credentials: &Credentials,
    ) -> IndyResult<(Box<dyn WalletStorage>, Metadata, KeyDerivationData)> {
        let storage = self._open_storage(config, credentials).await?;

        let metadata: Metadata = {
            let metadata = storage.get_storage_metadata().await?;

            serde_json::from_slice(&metadata)
                .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize metadata")?
        };

        let key_derivation_data = KeyDerivationData::from_passphrase_and_metadata(
            &credentials.key,
            &metadata,
            &credentials.key_derivation_method,
        )?;

        Ok((storage, metadata, key_derivation_data))
    }

    pub async fn close_wallet(&self, handle: WalletHandle) -> IndyResult<()> {
        trace!("close_wallet >>> handle: {:?}", handle);

        let wallet = self.wallets.lock().await.remove(&handle);

        let wallet = if let Some(wallet) = wallet {
            wallet
        } else {
            return Err(err_msg(
                IndyErrorKind::InvalidWalletHandle,
                "Unknown wallet handle",
            ));
        };

        self.wallet_ids.lock().await.remove(wallet.get_id());

        trace!("close_wallet <<<");
        Ok(())
    }

    fn _map_wallet_storage_error(err: IndyError, type_: &str, name: &str) -> IndyError {
        match err.kind() {
            IndyErrorKind::WalletItemAlreadyExists => err_msg(
                IndyErrorKind::WalletItemAlreadyExists,
                format!(
                    "Wallet item already exists with type: {}, id: {}",
                    type_, name
                ),
            ),
            IndyErrorKind::WalletItemNotFound => err_msg(
                IndyErrorKind::WalletItemNotFound,
                format!("Wallet item not found with type: {}, id: {}", type_, name),
            ),
            _ => err,
        }
    }

    pub async fn add_record(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        value: &str,
        tags: &Tags,
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.add(type_, name, value, tags).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn add_indy_record<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        value: &str,
        tags: &Tags,
    ) -> IndyResult<()>
    where
        T: Sized,
    {
        self.add_record(
            wallet_handle,
            &self.add_prefix(short_type_name::<T>()),
            name,
            value,
            tags,
        )
        .await?;

        Ok(())
    }

    pub async fn add_indy_object<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        object: &T,
        tags: &Tags,
    ) -> IndyResult<String>
    where
        T: ::serde::Serialize + Sized,
    {
        let object_json = serde_json::to_string(object).to_indy(
            IndyErrorKind::InvalidState,
            format!("Cannot serialize {:?}", short_type_name::<T>()),
        )?;

        self.add_indy_record::<T>(wallet_handle, name, &object_json, tags)
            .await?;

        Ok(object_json)
    }

    pub async fn update_record_value(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        value: &str,
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.update(type_, name, value).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn update_indy_object<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        object: &T,
    ) -> IndyResult<String>
    where
        T: ::serde::Serialize + Sized,
    {
        let type_ = short_type_name::<T>();

        let wallet = self.get_wallet(wallet_handle).await?;

        let object_json = serde_json::to_string(object).to_indy(
            IndyErrorKind::InvalidState,
            format!("Cannot serialize {:?}", type_),
        )?;

        wallet.update(&self.add_prefix(type_), name, &object_json).await?;

        Ok(object_json)
    }

    pub async fn add_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        tags: &Tags,
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.add_tags(type_, name, tags).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn update_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        tags: &Tags,
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.update_tags(type_, name, tags).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn delete_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        tag_names: &[&str],
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.delete_tags(type_, name, tag_names).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn delete_record(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
    ) -> IndyResult<()> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.delete(type_, name).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn delete_indy_record<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
    ) -> IndyResult<()>
    where
        T: Sized,
    {
        self.delete_record(
            wallet_handle,
            &self.add_prefix(short_type_name::<T>()),
            name,
        )
        .await?;

        Ok(())
    }

    pub async fn get_record(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        name: &str,
        options_json: &str,
    ) -> IndyResult<WalletRecord> {
        let wallet = self.get_wallet(wallet_handle).await?;
        wallet.get(type_, name, options_json).await
            .map_err(|err| WalletService::_map_wallet_storage_error(err, type_, name))
    }

    pub async fn get_indy_record<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        options_json: &str,
    ) -> IndyResult<WalletRecord>
    where
        T: Sized,
    {
        self.get_record(
            wallet_handle,
            &self.add_prefix(short_type_name::<T>()),
            name,
            options_json,
        )
        .await
    }

    pub async fn get_indy_record_value<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        options_json: &str,
    ) -> IndyResult<String>
    where
        T: Sized,
    {
        let type_ = short_type_name::<T>();

        let wallet = self.get_wallet(wallet_handle).await?;
        let record = wallet.get(&self.add_prefix(type_), name, options_json).await?;

        let record_value = record
            .get_value()
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidState,
                    format!("{} not found for id: {:?}", type_, name),
                )
            })?
            .to_string();

        Ok(record_value)
    }

    // Dirty hack. json must live longer then result T
    pub async fn get_indy_object<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        options_json: &str,
    ) -> IndyResult<T>
    where
        T: ::serde::de::DeserializeOwned + Sized,
    {
        let record_value = self
            .get_indy_record_value::<T>(wallet_handle, name, options_json)
            .await?;

        serde_json::from_str(&record_value).to_indy(
            IndyErrorKind::InvalidState,
            format!("Cannot deserialize {:?}", short_type_name::<T>()),
        )
    }

    // Dirty hack. json must live longer then result T
    pub async fn get_indy_opt_object<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        options_json: &str,
    ) -> IndyResult<Option<T>>
    where
        T: ::serde::de::DeserializeOwned + Sized,
    {
        match self
            .get_indy_object::<T>(wallet_handle, name, options_json)
            .await
        {
            Ok(res) => Ok(Some(res)),
            Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn search_records(
        &self,
        wallet_handle: WalletHandle,
        type_: &str,
        query_json: &str,
        options_json: &str,
    ) -> IndyResult<WalletSearch> {
        let wallet = self.get_wallet(wallet_handle).await?;
        Ok(WalletSearch {
            iter: wallet.search(type_, query_json, Some(options_json)).await?,
        })
    }

    pub async fn search_indy_records<T>(
        &self,
        wallet_handle: WalletHandle,
        query_json: &str,
        options_json: &str,
    ) -> IndyResult<WalletSearch>
    where
        T: Sized,
    {
        self.search_records(
            wallet_handle,
            &self.add_prefix(short_type_name::<T>()),
            query_json,
            options_json,
        )
        .await
    }

    #[allow(dead_code)] // TODO: Should we implement getting all records or delete everywhere?
    pub fn search_all_records(&self, _wallet_handle: WalletHandle) -> IndyResult<WalletSearch> {
        //        match self.wallets.lock().await.get(&wallet_handle) {
        //            Some(wallet) => wallet.search_all_records(),
        //            None => Err(IndyError::InvalidHandle(wallet_handle.to_string()))
        //        }
        unimplemented!()
    }

    pub async fn upsert_indy_object<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
        object: &T,
    ) -> IndyResult<String>
    where
        T: ::serde::Serialize + Sized,
    {
        if self.record_exists::<T>(wallet_handle, name).await? {
            self.update_indy_object::<T>(wallet_handle, name, object)
                .await
        } else {
            self.add_indy_object::<T>(wallet_handle, name, object, &HashMap::new())
                .await
        }
    }

    pub async fn record_exists<T>(
        &self,
        wallet_handle: WalletHandle,
        name: &str,
    ) -> IndyResult<bool>
    where
        T: Sized,
    {
        let wallet = self.get_wallet(wallet_handle).await?;
        match wallet.get(&self.add_prefix(short_type_name::<T>()),
                         name,
                         &RecordOptions::id()).await {
            Ok(_) => Ok(true),
            Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub async fn check(&self, handle: WalletHandle) -> IndyResult<()> {
        self.get_wallet(handle).await?;
        Ok(())
    }

    pub async fn export_wallet(
        &self,
        wallet_handle: WalletHandle,
        export_config: &ExportConfig,
        version: u32,
        key: (&KeyDerivationData, &MasterKey),
    ) -> IndyResult<()> {
        trace!(
            "export_wallet >>> wallet_handle: {:?}, export_config: {:?}, version: {:?}",
            wallet_handle,
            secret!(export_config),
            version
        );

        if version != 0 {
            return Err(err_msg(IndyErrorKind::InvalidState, "Unsupported version"));
        }

        let (key_data, key) = key;

        let wallet = self.get_wallet(wallet_handle).await?;

        let path = PathBuf::from(&export_config.path);

        if let Some(parent_path) = path.parent() {
            fs::DirBuilder::new().recursive(true).create(parent_path)?;
        }

        let mut export_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(export_config.path.clone())?;

        let res = export_continue(wallet, &mut export_file, version, key.clone(), key_data).await;

        trace!("export_wallet <<<");
        res
    }

    pub async fn import_wallet_prepare(
        &self,
        config: &Config,
        credentials: &Credentials,
        export_config: &ExportConfig,
    ) -> IndyResult<(WalletHandle, KeyDerivationData, KeyDerivationData)> {
        trace!(
            "import_wallet_prepare >>> config: {:?}, credentials: {:?}, export_config: {:?}",
            config,
            secret!(export_config),
            secret!(export_config)
        );

        let exported_file_to_import = fs::OpenOptions::new()
            .read(true)
            .open(&export_config.path)?;

        let (reader, import_key_derivation_data, nonce, chunk_size, header_bytes) =
            preparse_file_to_import(exported_file_to_import, &export_config.key)?;
        let key_data = KeyDerivationData::from_passphrase_with_new_salt(
            &credentials.key,
            &credentials.key_derivation_method,
        );

        let wallet_handle = indy_utils::next_wallet_handle();

        let stashed_key_data = key_data.clone();

        self.pending_for_import.lock().await.insert(
            wallet_handle,
            (reader, nonce, chunk_size, header_bytes, stashed_key_data),
        );

        Ok((wallet_handle, key_data, import_key_derivation_data))
    }

    pub async fn import_wallet_continue(
        &self,
        wallet_handle: WalletHandle,
        config: &Config,
        credentials: &Credentials,
        key: (MasterKey, MasterKey),
    ) -> IndyResult<()> {
        let (reader, nonce, chunk_size, header_bytes, key_data) = self
            .pending_for_import
            .lock().await
            .remove(&wallet_handle)
            .unwrap();

        let (import_key, master_key) = key;

        let keys = self
            ._create_wallet(config, credentials, (&key_data, &master_key))
            .await?;

        self._is_id_from_config_not_used(config).await?;
        let storage = self._open_storage(config, credentials).await?;
        let metadata = storage.get_storage_metadata().await?;

        let res = {
            let wallet = Wallet::new(
                WalletService::_get_wallet_id(&config),
                storage,
                Arc::new(keys),
            );

            finish_import(&wallet, reader, import_key, nonce, chunk_size, header_bytes).await
        };

        if res.is_err() {
            let metadata: Metadata = serde_json::from_slice(&metadata)
                .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize metadata")?;

            self.delete_wallet_continue(config, credentials, &metadata, &master_key)
                .await?;
        }

        //        self.close_wallet(wallet_handle)?;

        trace!("import_wallet <<<");
        res
    }

    pub async fn get_wallets_count(&self) -> usize {
        self.wallets.lock().await.len()
    }

    pub async fn get_wallet_ids_count(&self) -> usize {
        self.wallet_ids.lock().await.len()
    }

    pub async fn get_pending_for_import_count(&self) -> usize {
        self.pending_for_import.lock().await.len()
    }

    pub async fn get_pending_for_open_count(&self) -> usize {
        self.pending_for_open.lock().await.len()
    }

    fn _get_config_and_cred_for_storage<'a>(
        config: &Config,
        credentials: &Credentials,
        storage_types: &'a HashMap<String, Box<dyn WalletStorageType>>,
    ) -> IndyResult<(
        &'a Box<dyn WalletStorageType>,
        Option<String>,
        Option<String>,
    )> {
        let storage_type = {
            let storage_type = config
                .storage_type
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            storage_types.get(storage_type).ok_or_else(|| {
                err_msg(
                    IndyErrorKind::UnknownWalletStorageType,
                    "Unknown wallet storage type",
                )
            })?
        };

        let storage_config = config.storage_config.as_ref().map(SValue::to_string);

        let storage_credentials = credentials
            .storage_credentials
            .as_ref()
            .map(SValue::to_string);

        Ok((storage_type, storage_config, storage_credentials))
    }

    async fn _is_id_from_config_not_used(&self, config: &Config) -> IndyResult<()> {
        let id = WalletService::_get_wallet_id(config);
        if self.wallet_ids.lock().await.contains(&id) {
            return Err(err_msg(
                IndyErrorKind::WalletAlreadyOpened,
                format!(
                    "Wallet {} already opened",
                    WalletService::_get_wallet_id(config)
                ),
            ));
        }

        Ok(())
    }

    fn _get_wallet_id(config: &Config) -> String {
        let wallet_path = config
            .storage_config
            .as_ref()
            .and_then(|storage_config| storage_config["path"].as_str())
            .unwrap_or("");

        format!("{}{}", config.id, wallet_path)
    }

    async fn _open_storage(
        &self,
        config: &Config,
        credentials: &Credentials,
    ) -> IndyResult<Box<dyn WalletStorage>> {
        let storage_types = self.storage_types.lock().await;

        let (storage_type, storage_config, storage_credentials) =
            WalletService::_get_config_and_cred_for_storage(config, credentials, &storage_types)?;

        let storage = storage_type
            .open_storage(
                &config.id,
                storage_config.as_ref().map(String::as_str),
                storage_credentials.as_ref().map(String::as_str),
            )
            .await?;

        Ok(storage)
    }

    fn _prepare_metadata(
        &self,
        master_key: &chacha20poly1305_ietf::Key,
        key_data: &KeyDerivationData,
        keys: &Keys,
    ) -> IndyResult<Vec<u8>> {
        let encrypted_keys = keys.serialize_encrypted(master_key)?;

        let metadata = match key_data {
            KeyDerivationData::Raw(_) => Metadata::MetadataRaw(MetadataRaw {
                keys: encrypted_keys,
            }),
            KeyDerivationData::Argon2iInt(_, salt) | KeyDerivationData::Argon2iMod(_, salt) => {
                Metadata::MetadataArgon(MetadataArgon {
                    keys: encrypted_keys,
                    master_key_salt: salt[..].to_vec(),
                })
            }
        };

        let res = serde_json::to_vec(&metadata).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize wallet metadata",
        )?;

        Ok(res)
    }

    fn _restore_keys(&self, metadata: &Metadata, master_key: &MasterKey) -> IndyResult<Keys> {
        let metadata_keys = metadata.get_keys();

        let res = Keys::deserialize_encrypted(&metadata_keys, master_key).map_err(|err| {
            err.map(
                IndyErrorKind::WalletAccessFailed,
                "Invalid master key provided",
            )
        })?;

        Ok(res)
    }

    pub const PREFIX: &'static str = "Indy";

    pub fn add_prefix(&self, type_: &str) -> String {
        format!("{}::{}", WalletService::PREFIX, type_)
    }

    async fn get_wallet(&self, wallet_handle: WalletHandle) -> IndyResult<Arc<Wallet>> {
        let wallets = self.wallets.lock().await;
        let w = wallets.get(&wallet_handle);
        if let Some(w) = w {
            Ok(w.clone())
        } else {
            Err(err_msg(
                IndyErrorKind::InvalidWalletHandle,
                "Unknown wallet handle",
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Metadata {
    MetadataArgon(MetadataArgon),
    MetadataRaw(MetadataRaw),
}

impl Metadata {
    pub fn get_keys(&self) -> &Vec<u8> {
        match *self {
            Metadata::MetadataArgon(ref metadata) => &metadata.keys,
            Metadata::MetadataRaw(ref metadata) => &metadata.keys,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataArgon {
    pub keys: Vec<u8>,
    pub master_key_salt: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataRaw {
    pub keys: Vec<u8>,
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
    pub fn new(
        name: String,
        type_: Option<String>,
        value: Option<String>,
        tags: Option<Tags>,
    ) -> WalletRecord {
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

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

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

    pub async fn fetch_next_record(&mut self) -> IndyResult<Option<WalletRecord>> {
        self.iter.next().await
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

fn short_type_name<T>() -> &'static str {
    let type_name = std::any::type_name::<T>();
    type_name.rsplitn(2, "::").next().unwrap_or(type_name)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use indy_api_types::{domain::wallet::KeyDerivationMethod, INVALID_WALLET_HANDLE};
    use indy_utils::{
        assert_kind, assert_match, environment, inmem_wallet::InmemWallet, next_wallet_handle, test,
    };
    use serde_json::json;

    use lazy_static::lazy_static;

    use super::*;

    impl WalletService {
        async fn open_wallet(
            &self,
            config: &Config,
            credentials: &Credentials,
        ) -> IndyResult<WalletHandle> {
            self._is_id_from_config_not_used(config)?;

            let (storage, metadata, key_derivation_data) = self
                ._open_storage_and_fetch_metadata(config, credentials)
                .await?;

            let wallet_handle = next_wallet_handle();

            let rekey_data: Option<KeyDerivationData> =
                credentials.rekey.as_ref().map(|ref rekey| {
                    KeyDerivationData::from_passphrase_with_new_salt(
                        rekey,
                        &credentials.rekey_derivation_method,
                    )
                });

            self.pending_for_open.lock().await.insert(
                wallet_handle,
                (
                    WalletService::_get_wallet_id(config),
                    storage,
                    metadata,
                    rekey_data.clone(),
                ),
            );

            let key = key_derivation_data.calc_master_key()?;

            let rekey = match rekey_data {
                Some(rekey_data) => {
                    let rekey_result = rekey_data.calc_master_key()?;
                    Some(rekey_result)
                }
                None => None,
            };

            self.open_wallet_continue(wallet_handle, (&key, rekey.as_ref()))
                .await
        }

        pub async fn import_wallet(
            &self,
            config: &Config,
            credentials: &Credentials,
            export_config: &ExportConfig,
        ) -> IndyResult<()> {
            trace!(
                "import_wallet_prepare >>> config: {:?}, credentials: {:?}, export_config: {:?}",
                config,
                secret!(export_config),
                secret!(export_config)
            );

            let exported_file_to_import = fs::OpenOptions::new()
                .read(true)
                .open(&export_config.path)?;

            let (reader, import_key_derivation_data, nonce, chunk_size, header_bytes) =
                preparse_file_to_import(exported_file_to_import, &export_config.key)?;
            let key_data = KeyDerivationData::from_passphrase_with_new_salt(
                &credentials.key,
                &credentials.key_derivation_method,
            );

            let wallet_handle = next_wallet_handle();

            let import_key = import_key_derivation_data.calc_master_key()?;
            let master_key = key_data.calc_master_key()?;

            self.pending_for_import.lock().await.insert(
                wallet_handle,
                (reader, nonce, chunk_size, header_bytes, key_data),
            );

            self.import_wallet_continue(
                wallet_handle,
                config,
                credentials,
                (import_key, master_key),
            )
            .await
        }

        pub async fn delete_wallet(
            &self,
            config: &Config,
            credentials: &Credentials,
        ) -> IndyResult<()> {
            if self
                .wallets
                .lock().await
                .values()
                .any(|ref wallet| wallet.get_id() == WalletService::_get_wallet_id(config))
            {
                return Err(err_msg(
                    IndyErrorKind::InvalidState,
                    format!(
                        "Wallet has to be closed before deleting: {:?}",
                        WalletService::_get_wallet_id(config)
                    ),
                ))?;
            }

            let (_, metadata, key_derivation_data) = self
                ._open_storage_and_fetch_metadata(config, credentials)
                .await?;

            let master_key = key_derivation_data.calc_master_key()?;

            self.delete_wallet_continue(config, credentials, &metadata, &master_key)
                .await
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

    #[async_std::test]
    async fn wallet_service_create_wallet_works() {
        test::cleanup_wallet("wallet_service_create_wallet_works");

        {
            WalletService::new()
                .create_wallet(
                    &_config_default("wallet_service_create_wallet_works"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_create_wallet_works");
    }

    #[async_std::test]
    async fn wallet_service_create_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_interactive_key_derivation");

        {
            WalletService::new()
                .create_wallet(
                    &_config_default(
                        "wallet_service_create_wallet_works_for_interactive_key_derivation",
                    ),
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_create_wallet_works_for_interactive_key_derivation");
    }

    #[async_std::test]
    async fn wallet_service_create_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_moderate_key_derivation");

        {
            WalletService::new()
                .create_wallet(
                    &_config_default(
                        "wallet_service_create_wallet_works_for_moderate_key_derivation",
                    ),
                    &ARGON_MOD_CREDENTIAL,
                    (&MODERATE_KDD, &MODERATE_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_create_wallet_works_for_moderate_key_derivation");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_create_wallet_works_for_comparision_time_of_different_key_types() {
        use std::time::Instant;
        test::cleanup_wallet(
            "wallet_service_create_wallet_works_for_comparision_time_of_different_key_types",
        );
        {
            let wallet_service = WalletService::new();

            let config = _config_default(
                "wallet_service_create_wallet_works_for_comparision_time_of_different_key_types",
            );

            let time = Instant::now();

            wallet_service
                .create_wallet(
                    &config,
                    &ARGON_MOD_CREDENTIAL,
                    (&MODERATE_KDD, &MODERATE_MASTER_KEY),
                )
                .await
                .unwrap();

            let time_diff_moderate_key = time.elapsed();

            wallet_service
                .delete_wallet(&config, &ARGON_MOD_CREDENTIAL)
                .await
                .unwrap();

            _cleanup(
                "wallet_service_create_wallet_works_for_comparision_time_of_different_key_types",
            );

            let time = Instant::now();

            wallet_service
                .create_wallet(
                    &config,
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();

            let time_diff_interactive_key = time.elapsed();

            wallet_service
                .delete_wallet(&config, &ARGON_INT_CREDENTIAL)
                .await
                .unwrap();

            assert!(time_diff_interactive_key < time_diff_moderate_key);
        }

        test::cleanup_wallet(
            "wallet_service_create_wallet_works_for_comparision_time_of_different_key_types",
        );
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_create_works_for_plugged() {
        _cleanup("wallet_service_create_works_for_plugged");

        {
            let wallet_service = WalletService::new();
            _register_inmem_wallet(&wallet_service);

            wallet_service
                .create_wallet(
                    &_config_inmem(),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        _cleanup("wallet_service_create_works_for_plugged");
    }

    #[async_std::test]
    async fn wallet_service_create_wallet_works_for_none_type() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_none_type");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_create_wallet_works_for_none_type"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_create_wallet_works_for_none_type");
    }

    #[async_std::test]
    async fn wallet_service_create_wallet_works_for_unknown_type() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_unknown_type");

        {
            let wallet_service = WalletService::new();

            let res = wallet_service
                .create_wallet(
                    &_config_unknown("wallet_service_create_wallet_works_for_unknown_type"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await;

            assert_kind!(IndyErrorKind::UnknownWalletStorageType, res);
        }
    }

    #[async_std::test]
    async fn wallet_service_create_wallet_works_for_twice() {
        test::cleanup_wallet("wallet_service_create_wallet_works_for_twice");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_create_wallet_works_for_twice"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let res = wallet_service
                .create_wallet(
                    &_config("wallet_service_create_wallet_works_for_twice"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await;

            assert_kind!(IndyErrorKind::WalletAlreadyExists, res);
        }

        test::cleanup_wallet("wallet_service_create_wallet_works_for_twice");
    }

    // FIXME: !!!
    //
    // #[async_std::test]
    // async fn wallet_service_create_wallet_works_for_invalid_raw_key() {
    //     _cleanup("wallet_service_create_wallet_works_for_invalid_raw_key");

    //     let wallet_service = WalletService::new();
    //     wallet_service
    //         .create_wallet(
    //             &_config("wallet_service_create_wallet_works_for_invalid_raw_key"),
    //             &_credentials(),
    //         )
    //         .await.unwrap();
    //     let res = wallet_service.create_wallet(
    //         &_config("wallet_service_create_wallet_works_for_invalid_raw_key"),
    //         &_credentials_invalid_raw(),
    //     );
    //     assert_match!(
    //         Err(IndyError::CommonError(CommonError::InvalidStructure(_))),
    //         res
    //     );
    // }

    #[async_std::test]
    async fn wallet_service_delete_wallet_works() {
        test::cleanup_wallet("wallet_service_delete_wallet_works");

        {
            let config: &Config = &_config("wallet_service_delete_wallet_works");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            wallet_service
                .delete_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_delete_wallet_works");
    }

    #[async_std::test]
    async fn wallet_service_delete_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_interactive_key_derivation");

        {
            let config: &Config =
                &_config("wallet_service_delete_wallet_works_for_interactive_key_derivation");

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    config,
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();

            wallet_service
                .delete_wallet(config, &ARGON_INT_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .create_wallet(
                    config,
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_delete_wallet_works_for_interactive_key_derivation");
    }

    #[async_std::test]
    async fn wallet_service_delete_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_delete_wallet_works_for_moderate_key_derivation");

        {
            let config: &Config =
                &_config("wallet_service_delete_wallet_works_for_moderate_key_derivation");

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    config,
                    &ARGON_MOD_CREDENTIAL,
                    (&MODERATE_KDD, &MODERATE_MASTER_KEY),
                )
                .await
                .unwrap();

            wallet_service
                .delete_wallet(config, &ARGON_MOD_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .create_wallet(
                    config,
                    &ARGON_MOD_CREDENTIAL,
                    (&MODERATE_KDD, &MODERATE_MASTER_KEY),
                )
                .await
                .unwrap();
        }

        test::cleanup_wallet("wallet_service_delete_wallet_works_for_moderate_key_derivation");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_delete_works_for_plugged() {
        test::cleanup_wallet("wallet_service_delete_works_for_plugged");

        let wallet_service = WalletService::new();

        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        wallet_service
            .delete_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn wallet_service_delete_wallet_returns_error_if_wallet_opened() {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_wallet_opened");

        {
            let config: &Config =
                &_config("wallet_service_delete_wallet_returns_error_if_wallet_opened");

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            let res = wallet_service.delete_wallet(config, &RAW_CREDENTIAL).await;
            assert_eq!(IndyErrorKind::InvalidState, res.unwrap_err().kind());
        }

        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_wallet_opened");
    }

    #[async_std::test]
    async fn wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method(
    ) {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");

        {
            let config: &Config = &_config("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let res = wallet_service
                .delete_wallet(config, &ARGON_INT_CREDENTIAL)
                .await;

            assert_eq!(IndyErrorKind::WalletAccessFailed, res.unwrap_err().kind());
        }

        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_if_passed_different_value_for_interactive_method");
    }

    #[async_std::test]
    async fn wallet_service_delete_wallet_returns_error_for_nonexistant_wallet() {
        test::cleanup_wallet("wallet_service_delete_wallet_returns_error_for_nonexistant_wallet");

        let wallet_service = WalletService::new();

        let res = wallet_service
            .delete_wallet(
                &_config("wallet_service_delete_wallet_returns_error_for_nonexistant_wallet"),
                &RAW_CREDENTIAL,
            )
            .await;

        assert_eq!(IndyErrorKind::WalletNotFound, res.unwrap_err().kind());
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_works() {
        test::cleanup_wallet("wallet_service_open_wallet_works");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_open_wallet_works"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_open_wallet_works"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            // cleanup
            wallet_service.close_wallet(handle).await.unwrap();
        }

        test::cleanup_wallet("wallet_service_open_wallet_works");
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_works_for_interactive_key_derivation() {
        test::cleanup_wallet("wallet_service_open_wallet_works_for_interactive_key_derivation");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_open_wallet_works_for_interactive_key_derivation"),
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();

            let handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_open_wallet_works_for_interactive_key_derivation"),
                    &ARGON_INT_CREDENTIAL,
                )
                .await
                .unwrap();

            // cleanup
            wallet_service.close_wallet(handle).await.unwrap();
        }

        test::cleanup_wallet("wallet_service_open_wallet_works_for_interactive_key_derivation");
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_works_for_moderate_key_derivation() {
        test::cleanup_wallet("wallet_service_open_wallet_works_for_moderate_key_derivation");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_open_wallet_works_for_moderate_key_derivation"),
                    &ARGON_MOD_CREDENTIAL,
                    (&MODERATE_KDD, &MODERATE_MASTER_KEY),
                )
                .await
                .unwrap();

            let handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_open_wallet_works_for_moderate_key_derivation"),
                    &ARGON_MOD_CREDENTIAL,
                )
                .await
                .unwrap();

            // cleanup
            wallet_service.close_wallet(handle).await.unwrap();
        }

        test::cleanup_wallet("wallet_service_open_wallet_works_for_moderate_key_derivation");
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths() {
        _cleanup(
            "wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths",
        );

        let wallet_service = WalletService::new();

        let config_1 = Config {
            id: String::from("same_id"),
            storage_type: None,
            storage_config: None,
        };

        wallet_service
            .create_wallet(&config_1, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
            .await
            .unwrap();

        let handle_1 = wallet_service
            .open_wallet(&config_1, &RAW_CREDENTIAL)
            .await
            .unwrap();

        let config_2 = Config {
            id: String::from("same_id"),
            storage_type: None,
            storage_config: Some(json!({
                "path": _custom_path("wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths")
            })),
        };

        wallet_service
            .create_wallet(&config_2, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
            .await
            .unwrap();

        let handle_2 = wallet_service
            .open_wallet(&config_2, &RAW_CREDENTIAL)
            .await
            .unwrap();

        // cleanup
        wallet_service.close_wallet(handle_1).await.unwrap();
        wallet_service.close_wallet(handle_2).await.unwrap();

        wallet_service
            .delete_wallet(&config_1, &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .delete_wallet(&config_2, &RAW_CREDENTIAL)
            .await
            .unwrap();

        _cleanup(
            "wallet_service_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths",
        );
    }

    #[async_std::test]
    async fn wallet_service_open_unknown_wallet() {
        test::cleanup_wallet("wallet_service_open_unknown_wallet");

        let wallet_service = WalletService::new();

        let res = wallet_service
            .open_wallet(
                &_config("wallet_service_open_unknown_wallet"),
                &RAW_CREDENTIAL,
            )
            .await;

        assert_eq!(IndyErrorKind::WalletNotFound, res.unwrap_err().kind());
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_returns_appropriate_error_if_already_opened() {
        test::cleanup_wallet(
            "wallet_service_open_wallet_returns_appropriate_error_if_already_opened",
        );

        {
            let config: &Config =
                &_config("wallet_service_open_wallet_returns_appropriate_error_if_already_opened");

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL).await;

            assert_eq!(IndyErrorKind::WalletAlreadyOpened, res.unwrap_err().kind());
        }

        test::cleanup_wallet(
            "wallet_service_open_wallet_returns_appropriate_error_if_already_opened",
        );
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_open_works_for_plugged() {
        _cleanup("wallet_service_open_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening(
    ) {
        test::cleanup_wallet("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY))
                .await.unwrap();

            let res = wallet_service
                .open_wallet(
                    &_config("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening"),
                    &ARGON_INT_CREDENTIAL
                )
                .await;

            assert_kind!(IndyErrorKind::WalletAccessFailed, res);
        }

        test::cleanup_wallet("wallet_service_open_wallet_returns_error_if_used_different_methods_for_creating_and_opening");
    }

    #[async_std::test]
    async fn wallet_service_close_wallet_works() {
        test::cleanup_wallet("wallet_service_close_wallet_works");

        {
            let config: &Config = &_config("wallet_service_close_wallet_works");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service.close_wallet(wallet_handle).await.unwrap();
        }

        test::cleanup_wallet("wallet_service_close_wallet_works");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_close_works_for_plugged() {
        _cleanup("wallet_service_close_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service.close_wallet(wallet_handle).await.unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service.close_wallet(wallet_handle).await.unwrap();
    }

    #[async_std::test]
    async fn wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle() {
        test::cleanup_wallet(
            "wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle",
        );
        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config(
                        "wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle",
                    ),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config(
                        "wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle",
                    ),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let res = wallet_service.close_wallet(INVALID_WALLET_HANDLE).await;
            assert_kind!(IndyErrorKind::InvalidWalletHandle, res);

            wallet_service.close_wallet(wallet_handle).await.unwrap();
        }

        test::cleanup_wallet(
            "wallet_service_close_wallet_returns_appropriate_error_if_wrong_handle",
        );
    }

    #[async_std::test]
    async fn wallet_service_add_record_works() {
        test::cleanup_wallet("wallet_service_add_record_works");
        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_add_record_works"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_add_record_works"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        test::cleanup_wallet(
            "wallet_service_
        cord_works",
        );
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_add_record_works_for_plugged() {
        _cleanup("wallet_service_add_record_works_for_plugged");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
            .await
            .unwrap();

        wallet_service
            .get_record(wallet_handle, "type", "key1", "{}")
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn wallet_service_get_record_works_for_id_only() {
        test::cleanup_wallet("wallet_service_get_record_works_for_id_only");

        {
            let wallet_service = WalletService::new();
            wallet_service
                .create_wallet(
                    &_config("wallet_service_get_record_works_for_id_only"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_get_record_works_for_id_only"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(false, false, false),
                )
                .await
                .unwrap();

            assert!(record.get_value().is_none());
            assert!(record.get_type().is_none());
            assert!(record.get_tags().is_none());
        }

        test::cleanup_wallet("wallet_service_get_record_works_for_id_only");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_get_record_works_for_plugged_for_id_only() {
        test::cleanup_indy_home("wallet_service_get_record_works_for_plugged_for_id_only");
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                "type",
                "key1",
                &_fetch_options(false, false, false),
            )
            .await
            .unwrap();

        assert!(record.get_value().is_none());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[async_std::test]
    async fn wallet_service_get_record_works_for_id_value() {
        test::cleanup_wallet("wallet_service_get_record_works_for_id_value");
        {
            let wallet_service = WalletService::new();
            wallet_service
                .create_wallet(
                    &_config("wallet_service_get_record_works_for_id_value"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_get_record_works_for_id_value"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(false, true, false),
                )
                .await
                .unwrap();

            assert_eq!("value1", record.get_value().unwrap());
            assert!(record.get_type().is_none());
            assert!(record.get_tags().is_none());
        }

        test::cleanup_wallet("wallet_service_get_record_works_for_id_value");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_get_record_works_for_plugged_for_id_value() {
        _cleanup("wallet_service_get_record_works_for_plugged_for_id_value");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                "type",
                "key1",
                &_fetch_options(false, true, false),
            )
            .await
            .unwrap();

        assert_eq!("value1", record.get_value().unwrap());
        assert!(record.get_type().is_none());
        assert!(record.get_tags().is_none());
    }

    #[async_std::test]
    async fn wallet_service_get_record_works_for_all_fields() {
        test::cleanup_wallet("wallet_service_get_record_works_for_all_fields");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_get_record_works_for_all_fields"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_get_record_works_for_all_fields"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let mut tags = HashMap::new();
            tags.insert(String::from("1"), String::from("some"));

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &tags)
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            assert_eq!(&tags, record.get_tags().unwrap());
        }

        test::cleanup_wallet("wallet_service_get_record_works_for_all_fields");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_get_record_works_for_plugged_for_for_all_fields() {
        _cleanup("wallet_service_get_record_works_for_plugged_for_for_all_fields");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        let tags = serde_json::from_str(r#"{"1":"some"}"#).unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key1", "value1", &tags)
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                "type",
                "key1",
                &_fetch_options(true, true, true),
            )
            .await
            .unwrap();

        assert_eq!("type", record.get_type().unwrap());
        assert_eq!("value1", record.get_value().unwrap());
        assert_eq!(tags, record.get_tags().unwrap().clone());
    }

    #[async_std::test]
    async fn wallet_service_add_get_works_for_reopen() {
        test::cleanup_wallet("wallet_service_add_get_works_for_reopen");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_add_get_works_for_reopen"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_add_get_works_for_reopen"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_add_get_works_for_reopen"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(false, true, false),
                )
                .await
                .unwrap();

            assert_eq!("value1", record.get_value().unwrap());
        }

        test::cleanup_wallet("wallet_service_add_get_works_for_reopen");
    }

    #[async_std::test]
    async fn wallet_service_get_works_for_unknown() {
        test::cleanup_wallet("wallet_service_get_works_for_unknown");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_get_works_for_unknown"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_get_works_for_unknown"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let res = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(false, true, false),
                )
                .await;

            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        test::cleanup_wallet("wallet_service_get_works_for_unknown");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_get_works_for_plugged_and_unknown() {
        _cleanup("wallet_service_get_works_for_plugged_and_unknown");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        let res = wallet_service
            .get_record(
                wallet_handle,
                "type",
                "key1",
                &_fetch_options(false, true, false),
            )
            .await;

        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    /**
     * Update tests
     */
    #[async_std::test]
    async fn wallet_service_update() {
        test::cleanup_wallet("wallet_service_update");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let new_value = "new_value";

            let wallet_service = WalletService::new();
            wallet_service
                .create_wallet(
                    &_config("wallet_service_update"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_update"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(false, true, false),
                )
                .await
                .unwrap();

            assert_eq!(value, record.get_value().unwrap());

            wallet_service
                .update_record_value(wallet_handle, type_, name, new_value)
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(false, true, false),
                )
                .await
                .unwrap();

            assert_eq!(new_value, record.get_value().unwrap());
        }

        test::cleanup_wallet("wallet_service_update");
    }
    #[async_std::test]
    #[ignore]
    async fn wallet_service_update_for_plugged() {
        _cleanup("wallet_service_update_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, type_, name, value, &HashMap::new())
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                type_,
                name,
                &_fetch_options(false, true, false),
            )
            .await
            .unwrap();

        assert_eq!(value, record.get_value().unwrap());

        wallet_service
            .update_record_value(wallet_handle, type_, name, new_value)
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                type_,
                name,
                &_fetch_options(false, true, false),
            )
            .await
            .unwrap();

        assert_eq!(new_value, record.get_value().unwrap());
    }

    /**
     * Delete tests
     */
    #[async_std::test]
    async fn wallet_service_delete_record() {
        test::cleanup_wallet("wallet_service_delete_record");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_delete_record"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_delete_record"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(false, true, false),
                )
                .await
                .unwrap();

            assert_eq!(value, record.get_value().unwrap());

            wallet_service
                .delete_record(wallet_handle, type_, name)
                .await
                .unwrap();

            let res = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(false, true, false),
                )
                .await;

            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        test::cleanup_wallet("wallet_service_delete_record");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_delete_record_for_plugged() {
        _cleanup("wallet_service_delete_record_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, type_, name, value, &HashMap::new())
            .await
            .unwrap();

        let record = wallet_service
            .get_record(
                wallet_handle,
                type_,
                name,
                &_fetch_options(false, true, false),
            )
            .await
            .unwrap();

        assert_eq!(value, record.get_value().unwrap());

        wallet_service
            .delete_record(wallet_handle, type_, name)
            .await
            .unwrap();

        let res = wallet_service
            .get_record(
                wallet_handle,
                type_,
                name,
                &_fetch_options(false, true, false),
            )
            .await;

        assert_kind!(IndyErrorKind::WalletItemNotFound, res);
    }

    /**
     * Add tags tests
     */
    #[async_std::test]
    async fn wallet_service_add_tags() {
        test::cleanup_wallet("wallet_service_add_tags");

        {
            let type_ = "type";
            let name = "name";
            let value = "value";

            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_add_tags"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_add_tags"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &tags)
                .await
                .unwrap();

            let new_tags = serde_json::from_str(
                r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#,
            )
            .unwrap();

            wallet_service
                .add_record_tags(wallet_handle, type_, name, &new_tags)
                .await
                .unwrap();

            let item = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }

        test::cleanup_wallet("wallet_service_add_tags");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_add_tags_for_plugged() {
        _cleanup("wallet_service_add_tags_for_plugged");

        let type_ = "type";
        let name = "name";
        let value = "value";

        let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1"}"#).unwrap();

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, type_, name, value, &tags)
            .await
            .unwrap();

        let new_tags =
            serde_json::from_str(r#"{"tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#)
                .unwrap();

        wallet_service
            .add_record_tags(wallet_handle, type_, name, &new_tags)
            .await
            .unwrap();

        let item = wallet_service
            .get_record(
                wallet_handle,
                type_,
                name,
                &_fetch_options(true, true, true),
            )
            .await
            .unwrap();

        let expected_tags: Tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#)
            .unwrap();

        let retrieved_tags = item.tags.unwrap();
        assert_eq!(expected_tags, retrieved_tags);
    }

    /**
     * Update tags tests
     */
    #[async_std::test]
    async fn wallet_service_update_tags() {
        test::cleanup_wallet("wallet_service_update_tags");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_update_tags"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_update_tags"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &tags)
                .await
                .unwrap();

            let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            wallet_service
                .update_record_tags(wallet_handle, type_, name, &new_tags)
                .await
                .unwrap();

            let item = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let retrieved_tags = item.tags.unwrap();
            assert_eq!(new_tags, retrieved_tags);
        }

        test::cleanup_wallet("wallet_service_update_tags");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_update_tags_for_plugged() {
        _cleanup("wallet_service_update_tags_for_plugged");

        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"tag_value_2", "~tag_name_3":"tag_value_3"}"#).unwrap();
            let wallet_service = WalletService::new();

            _register_inmem_wallet(&wallet_service);

            wallet_service
                .create_wallet(
                    &_config_inmem(),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &tags)
                .await
                .unwrap();

            let new_tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            wallet_service
                .update_record_tags(wallet_handle, type_, name, &new_tags)
                .await
                .unwrap();

            let item = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let retrieved_tags = item.tags.unwrap();
            assert_eq!(new_tags, retrieved_tags);
        }

        _cleanup("wallet_service_update_tags_for_plugged");
    }

    /**
     * Delete tags tests
     */
    #[async_std::test]
    async fn wallet_service_delete_tags() {
        test::cleanup_wallet("wallet_service_delete_tags");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_delete_tags"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config("wallet_service_delete_tags"), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &tags)
                .await
                .unwrap();

            let tag_names = vec!["tag_name_1", "~tag_name_3"];

            wallet_service
                .delete_record_tags(wallet_handle, type_, name, &tag_names)
                .await
                .unwrap();

            let item = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let expected_tags: Tags =
                serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();

            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }

        test::cleanup_wallet("wallet_service_delete_tags");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_delete_tags_for_plugged() {
        _cleanup("wallet_service_delete_tags_for_plugged");
        {
            let type_ = "type";
            let name = "name";
            let value = "value";
            let tags = serde_json::from_str(r#"{"tag_name_1":"tag_value_1", "tag_name_2":"new_tag_value_2", "~tag_name_3":"new_tag_value_3"}"#).unwrap();

            let wallet_service = WalletService::new();
            _register_inmem_wallet(&wallet_service);

            wallet_service
                .create_wallet(
                    &_config_inmem(),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, type_, name, value, &tags)
                .await
                .unwrap();

            let tag_names = vec!["tag_name_1", "~tag_name_3"];

            wallet_service
                .delete_record_tags(wallet_handle, type_, name, &tag_names)
                .await
                .unwrap();

            let item = wallet_service
                .get_record(
                    wallet_handle,
                    type_,
                    name,
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let expected_tags: Tags =
                serde_json::from_str(r#"{"tag_name_2":"new_tag_value_2"}"#).unwrap();

            let retrieved_tags = item.tags.unwrap();
            assert_eq!(expected_tags, retrieved_tags);
        }

        _cleanup("wallet_service_delete_tags_for_plugged");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_search_records_works() {
        test::cleanup_wallet("wallet_service_search_records_works");
        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_search_records_works"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_search_records_works"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key2", "value2", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type3", "key3", "value3", &HashMap::new())
                .await
                .unwrap();

            let mut search = wallet_service
                .search_records(
                    wallet_handle,
                    "type3",
                    "{}",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            let record = search.fetch_next_record().await.unwrap().unwrap();
            assert_eq!("value3", record.get_value().unwrap());
            assert_eq!(HashMap::new(), record.get_tags().unwrap().clone());

            assert!(search.fetch_next_record().await.unwrap().is_none());
        }

        test::cleanup_wallet("wallet_service_search_records_works");
    }

    #[async_std::test]
    #[ignore]
    async fn wallet_service_search_records_works_for_plugged_wallet() {
        _cleanup("wallet_service_search_records_works_for_plugged_wallet");

        let wallet_service = WalletService::new();
        _register_inmem_wallet(&wallet_service);

        wallet_service
            .create_wallet(
                &_config_inmem(),
                &RAW_CREDENTIAL,
                (&RAW_KDD, &RAW_MASTER_KEY),
            )
            .await
            .unwrap();

        let wallet_handle = wallet_service
            .open_wallet(&_config_inmem(), &RAW_CREDENTIAL)
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type", "key2", "value2", &HashMap::new())
            .await
            .unwrap();

        wallet_service
            .add_record(wallet_handle, "type3", "key3", "value3", &HashMap::new())
            .await
            .unwrap();

        let mut search = wallet_service
            .search_records(
                wallet_handle,
                "type3",
                "{}",
                &_fetch_options(true, true, true),
            )
            .await
            .unwrap();

        let record = search.fetch_next_record().await.unwrap().unwrap();
        assert_eq!("value3", record.get_value().unwrap());
        assert_eq!(HashMap::new(), record.get_tags().unwrap().clone());

        assert!(search.fetch_next_record().await.unwrap().is_none());
    }

    /**
        Key rotation test
    */
    #[async_std::test]
    async fn wallet_service_key_rotation() {
        test::cleanup_wallet("wallet_service_key_rotation");
        {
            let config: &Config = &_config("wallet_service_key_rotation");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &_rekey_credentials_moderate())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).await.unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL).await;
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service
                .open_wallet(config, &_credentials_for_new_key_moderate())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }

        test::cleanup_wallet("wallet_service_key_rotation");
    }

    #[async_std::test]
    async fn wallet_service_key_rotation_for_rekey_interactive_method() {
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_interactive_method");
        {
            let config: &Config =
                &_config("wallet_service_key_rotation_for_rekey_interactive_method");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &_rekey_credentials_interactive())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).await.unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL).await;
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service
                .open_wallet(config, &_credentials_for_new_key_interactive())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }

        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_interactive_method");
    }

    #[async_std::test]
    async fn wallet_service_key_rotation_for_rekey_raw_method() {
        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_raw_method");

        {
            let config: &Config = &_config("wallet_service_key_rotation_for_rekey_raw_method");
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &_rekey_credentials_raw())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
            wallet_service.close_wallet(wallet_handle).await.unwrap();

            // Access failed for old key
            let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL).await;
            assert_kind!(IndyErrorKind::WalletAccessFailed, res);

            // Works ok with new key when reopening
            let wallet_handle = wallet_service
                .open_wallet(config, &_credentials_for_new_key_raw())
                .await
                .unwrap();

            let record = wallet_service
                .get_record(
                    wallet_handle,
                    "type",
                    "key1",
                    &_fetch_options(true, true, true),
                )
                .await
                .unwrap();

            assert_eq!("type", record.get_type().unwrap());
            assert_eq!("value1", record.get_value().unwrap());
        }

        test::cleanup_wallet("wallet_service_key_rotation_for_rekey_raw_method");
    }

    fn remove_exported_wallet(export_config: &ExportConfig) -> &Path {
        let export_path = Path::new(&export_config.path);

        if export_path.exists() {
            fs::remove_file(export_path).unwrap();
        }

        export_path
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_when_empty() {
        test::cleanup_wallet("wallet_service_export_wallet_when_empty");
        let export_config = _export_config_raw("export_wallet_service_export_wallet_when_empty");
        {
            let wallet_service = WalletService::new();
            let wallet_config = _config("wallet_service_export_wallet_when_empty");
            wallet_service
                .create_wallet(&wallet_config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();
            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_when_empty"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_when_empty");
            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());
        }
        remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_when_empty");
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_1_item() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item");
        let export_config = _export_config_raw("export_config_wallet_service_export_wallet_1_item");
        {
            let wallet_service = WalletService::new();
            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_wallet_1_item"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();
            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_1_item"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();
            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key_raw("key_wallet_service_export_wallet_1_item");
            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();
            assert!(export_path.exists());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item");
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_1_item_interactive_method() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item_interactive_method");
        let export_config =
            _export_config_interactive("wallet_service_export_wallet_1_item_interactive_method");
        {
            let wallet_service = WalletService::new();
            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_wallet_1_item_interactive_method"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();
            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_1_item_interactive_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();
            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) =
                _export_key_interactive("wallet_service_export_wallet_1_item_interactive_method");
            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();
            assert!(export_path.exists());
        }
        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item_interactive_method");
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_1_item_raw_method() {
        test::cleanup_wallet("wallet_service_export_wallet_1_item_raw_method");
        let export_config = _export_config_raw("wallet_service_export_wallet_1_item_raw_method");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_wallet_1_item_raw_method"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_1_item_raw_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let export_path = remove_exported_wallet(&export_config);
            let (kdd, master_key) = _export_key("wallet_service_export_wallet_1_item_raw_method");

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(&export_path.exists());
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_1_item_raw_method");
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_returns_error_if_file_exists() {
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_file_exists");

        {
            fs::create_dir_all(
                _export_file_path("wallet_service_export_wallet_returns_error_if_file_exists")
                    .parent()
                    .unwrap(),
            )
            .unwrap();

            fs::File::create(_export_file_path(
                "wallet_service_export_wallet_returns_error_if_file_exists",
            ))
            .unwrap();
        }

        assert!(
            _export_file_path("wallet_service_export_wallet_returns_error_if_file_exists").exists()
        );

        let export_config =
            _export_config_raw("wallet_service_export_wallet_returns_error_if_file_exists");
        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_wallet_returns_error_if_file_exists"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_returns_error_if_file_exists"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let (kdd, master_key) =
                _export_key_raw("key_wallet_service_export_wallet_returns_error_if_file_exists");

            let res = wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await;

            assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_file_exists");
    }

    #[async_std::test]
    async fn wallet_service_export_wallet_returns_error_if_wrong_handle() {
        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_wrong_handle");
        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_wallet_returns_error_if_wrong_handle"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let _wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_wallet_returns_error_if_wrong_handle"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let (kdd, master_key) =
                _export_key_raw("key_wallet_service_export_wallet_returns_error_if_wrong_handle");

            let export_config =
                _export_config_raw("wallet_service_export_wallet_returns_error_if_wrong_handle");

            let export_path = remove_exported_wallet(&export_config);

            let res = wallet_service
                .export_wallet(
                    INVALID_WALLET_HANDLE,
                    &export_config,
                    0,
                    (&kdd, &master_key),
                )
                .await;

            assert_kind!(IndyErrorKind::InvalidWalletHandle, res);
            assert!(!export_path.exists());
        }

        test::cleanup_wallet("wallet_service_export_wallet_returns_error_if_wrong_handle");
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_1_item() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item");
        let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_import_wallet_1_item"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();
            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let (kdd, master_key) =
                _export_key_raw("key_wallet_service_export_import_wallet_1_item");

            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(
                    &_config("wallet_service_export_import_wallet_1_item"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            let export_config = _export_config_raw("wallet_service_export_import_wallet_1_item");

            wallet_service
                .import_wallet(
                    &_config("wallet_service_export_import_wallet_1_item"),
                    &RAW_CREDENTIAL,
                    &export_config,
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item");
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_1_item_for_interactive_method() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_interactive_method");

        let export_config = _export_config_interactive(
            "wallet_service_export_import_wallet_1_item_for_interactive_method",
        );

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_interactive_method"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_interactive_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let (kdd, master_key) = _export_key_interactive(
                "wallet_service_export_import_wallet_1_item_for_interactive_method",
            );

            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_interactive_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .import_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_interactive_method"),
                    &RAW_CREDENTIAL,
                    &_export_config_interactive(
                        "wallet_service_export_import_wallet_1_item_for_interactive_method",
                    ),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_interactive_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_interactive_method");
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_1_item_for_moderate_method() {
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_moderate_method");

        let export_config =
            _export_config_raw("wallet_service_export_import_wallet_1_item_for_moderate_method");

        {
            let wallet_service = WalletService::new();

            wallet_service
                .create_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_moderate_method"),
                    &RAW_CREDENTIAL,
                    (&RAW_KDD, &RAW_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_moderate_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let (kdd, master_key) = _export_key_raw(
                "key_wallet_service_export_import_wallet_1_item_for_moderate_method",
            );

            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_moderate_method"),
                    &RAW_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .import_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_moderate_method"),
                    &ARGON_MOD_CREDENTIAL,
                    &export_config,
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(
                    &_config("wallet_service_export_import_wallet_1_item_for_moderate_method"),
                    &ARGON_MOD_CREDENTIAL,
                )
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_1_item_for_moderate_method");
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw() {
        test::cleanup_wallet(
            "wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw",
        );

        let export_config = _export_config_raw(
            "wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw",
        );

        {
            let wallet_service = WalletService::new();

            let config: &Config = &_config(
                "wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw",
            );

            wallet_service
                .create_wallet(
                    config,
                    &ARGON_INT_CREDENTIAL,
                    (&INTERACTIVE_KDD, &INTERACTIVE_MASTER_KEY),
                )
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &ARGON_INT_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let (kdd, master_key) = _export_key_interactive(
                "wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw",
            );

            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();
            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(config, &ARGON_INT_CREDENTIAL)
                .await
                .unwrap();

            wallet_service.import_wallet(config, &ARGON_MOD_CREDENTIAL, &_export_config_moderate("wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw")).await.unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &ARGON_MOD_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);

        test::cleanup_wallet(
            "wallet_service_export_import_wallet_1_item_for_export_interactive_import_as_raw",
        );
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive() {
        test::cleanup_wallet(
            "wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive",
        );

        let export_config = _export_config_interactive(
            "wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive",
        );

        {
            let wallet_service = WalletService::new();

            let config: &Config = &_config(
                "wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive",
            );

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .add_record(wallet_handle, "type", "key1", "value1", &HashMap::new())
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();

            let (kdd, master_key) = _export_key_interactive(
                "wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive",
            );

            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .import_wallet(config, &ARGON_INT_CREDENTIAL, &export_config)
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &ARGON_INT_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .get_record(wallet_handle, "type", "key1", "{}")
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);

        test::cleanup_wallet(
            "wallet_service_export_import_wallet_1_item_for_export_raw_import_as_interactive",
        );
    }

    #[async_std::test]
    async fn wallet_service_export_import_wallet_if_empty() {
        test::cleanup_wallet("wallet_service_export_import_wallet_if_empty");

        let export_config = _export_config_raw("wallet_service_export_import_wallet_if_empty");

        {
            let wallet_service = WalletService::new();
            let config: &Config = &_config("wallet_service_export_import_wallet_if_empty");

            wallet_service
                .create_wallet(config, &RAW_CREDENTIAL, (&RAW_KDD, &RAW_MASTER_KEY))
                .await
                .unwrap();

            let wallet_handle = wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            let (kdd, master_key) = _export_key("wallet_service_export_import_wallet_if_empty");
            let export_path = remove_exported_wallet(&export_config);

            wallet_service
                .export_wallet(wallet_handle, &export_config, 0, (&kdd, &master_key))
                .await
                .unwrap();

            assert!(export_path.exists());

            wallet_service.close_wallet(wallet_handle).await.unwrap();

            wallet_service
                .delete_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();

            wallet_service
                .import_wallet(config, &RAW_CREDENTIAL, &export_config)
                .await
                .unwrap();

            wallet_service
                .open_wallet(config, &RAW_CREDENTIAL)
                .await
                .unwrap();
        }

        let _export_path = remove_exported_wallet(&export_config);
        test::cleanup_wallet("wallet_service_export_import_wallet_if_empty");
    }

    #[async_std::test]
    async fn wallet_service_export_import_returns_error_if_path_missing() {
        _cleanup("wallet_service_export_import_returns_error_if_path_missing");

        let wallet_service = WalletService::new();

        let config: &Config =
            &_config("wallet_service_export_import_returns_error_if_path_missing");

        let export_config =
            _export_config_raw("wallet_service_export_import_returns_error_if_path_missing");

        let res = wallet_service
            .import_wallet(config, &RAW_CREDENTIAL, &export_config)
            .await;
        assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());

        let res = wallet_service.open_wallet(config, &RAW_CREDENTIAL).await;
        assert_match!(Err(_), res);

        _cleanup("wallet_service_export_import_returns_error_if_path_missing");
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
          "retrieveType": type_,
          "retrieveValue": value,
          "retrieveTags": tags,
        })
        .to_string()
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
        static ref MODERATE_KDD: KeyDerivationData =
            KeyDerivationData::from_passphrase_with_new_salt(
                "my_key",
                &KeyDerivationMethod::ARGON2I_MOD
            );
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref MODERATE_MASTER_KEY: MasterKey = MODERATE_KDD.calc_master_key().unwrap();
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref INTERACTIVE_KDD: KeyDerivationData =
            KeyDerivationData::from_passphrase_with_new_salt(
                "my_key",
                &KeyDerivationMethod::ARGON2I_INT
            );
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref INTERACTIVE_MASTER_KEY: MasterKey = INTERACTIVE_KDD.calc_master_key().unwrap();
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref RAW_KDD: KeyDerivationData = KeyDerivationData::from_passphrase_with_new_salt(
            "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw",
            &KeyDerivationMethod::RAW
        );
    }

    #[allow(non_upper_case_globals)]
    lazy_static! {
        static ref RAW_MASTER_KEY: MasterKey = RAW_KDD.calc_master_key().unwrap();
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
        let mut path = environment::tmp_path();
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
        let kdd = KeyDerivationData::from_passphrase_with_new_salt(
            &export_config.key,
            &export_config.key_derivation_method,
        );
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

    fn _custom_path(name: &str) -> String {
        let mut path = environment::tmp_path();
        path.push(name);
        path.to_str().unwrap().to_owned()
    }

    #[test]
    fn short_type_name_works() {
        assert_eq!("WalletRecord", short_type_name::<WalletRecord>());
    }
}
