use std::sync::Arc;

use indy_api_types::wallet::*;
use indy_api_types::domain::wallet::{Config, Credentials, ExportConfig, KeyConfig};
use indy_api_types::errors::prelude::*;
use crate::services::crypto::CryptoService;
use indy_wallet::{KeyDerivationData, WalletService};
use indy_utils::crypto::{chacha20poly1305_ietf, randombytes};
use indy_utils::crypto::chacha20poly1305_ietf::Key as MasterKey;
use indy_api_types::WalletHandle;
use rust_base58::ToBase58;

pub enum WalletCommand {
    RegisterWalletType(String, // type_
                       WalletCreate, // create
                       WalletOpen, // open
                       WalletClose, // close
                       WalletDelete, // delete
                       WalletAddRecord, // add record
                       WalletUpdateRecordValue, // update record value
                       WalletUpdateRecordTags, // update record value
                       WalletAddRecordTags, // add record tags
                       WalletDeleteRecordTags, // delete record tags
                       WalletDeleteRecord, // delete record
                       WalletGetRecord, // get record
                       WalletGetRecordId, // get record id
                       WalletGetRecordType, // get record id
                       WalletGetRecordValue, // get record value
                       WalletGetRecordTags, // get record tags
                       WalletFreeRecord, // free record
                       WalletGetStorageMetadata, // get storage metadata
                       WalletSetStorageMetadata, // set storage metadata
                       WalletFreeStorageMetadata, // free storage metadata
                       WalletSearchRecords, // search records
                       WalletSearchAllRecords, // search all records
                       WalletGetSearchTotalCount, // get search total count
                       WalletFetchSearchNextRecord, // fetch search next record
                       WalletFreeSearch, // free search
                       Box<dyn Fn(IndyResult<()>) + Send>),
    Create(Config, // config
           Credentials, // credentials
           Box<dyn Fn(IndyResult<()>) + Send>),
    Open(Config, // config
         Credentials, // credentials
         Box<dyn Fn(IndyResult<WalletHandle>) + Send>),
    Close(WalletHandle,
          Box<dyn Fn(IndyResult<()>) + Send>),
    Delete(Config, // config
           Credentials, // credentials
           Box<dyn Fn(IndyResult<()>) + Send>),
    Export(WalletHandle,
           ExportConfig, // export config
           Box<dyn Fn(IndyResult<()>) + Send>),
    Import(Config, // config
           Credentials, // credentials
           ExportConfig, // import config
           Box<dyn Fn(IndyResult<()>) + Send>),
    GenerateKey(Option<KeyConfig>, // config
                Box<dyn Fn(IndyResult<String>) + Send>),
}

pub struct WalletCommandExecutor {
    wallet_service: Arc<WalletService>,
    crypto_service: Arc<CryptoService>,
}

impl WalletCommandExecutor {
    pub fn new(wallet_service: Arc<WalletService>, crypto_service: Arc<CryptoService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
            wallet_service,
            crypto_service,
        }
    }

    pub async fn execute(&self, command: WalletCommand) {
        match command {
            WalletCommand::RegisterWalletType(type_, create, open, close, delete, add_record,
                                              update_record_value, update_record_tags, add_record_tags,
                                              delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                              get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                                              free_storage_metadata, search_records, search_all_records, get_search_total_count,
                                              fetch_search_next_record, free_search, cb) => {
                debug!(target: "wallet_command_executor", "RegisterWalletType command received");
                let result = self._register_type(&type_, create, open, close, delete, add_record,
                                                 update_record_value, update_record_tags, add_record_tags,
                                                 delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                                                 get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                                                 free_storage_metadata, search_records, search_all_records, get_search_total_count,
                                                 fetch_search_next_record, free_search).await;
                cb(result);
            }
            WalletCommand::Create(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Create command received");
                let result1 = self._create(&config, &credentials).await;
                cb(result1);
            }
            WalletCommand::Open(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Open command received");
                let result2 = self._open(&config, &credentials).await;
                cb(result2);
            }
            WalletCommand::Close(handle, cb) => {
                debug!(target: "wallet_command_executor", "Close command received");
                let result3 = self._close(handle).await;
                cb(result3);
            }
            WalletCommand::Delete(config, credentials, cb) => {
                debug!(target: "wallet_command_executor", "Delete command received");
                let result4 = self._delete(&config, &credentials).await;
                cb(result4)
            }
            WalletCommand::Export(wallet_handle, export_config, cb) => {
                debug!(target: "wallet_command_executor", "Export command received");
                let result5 = self._export(wallet_handle, &export_config).await;
                cb(result5)
            }
            WalletCommand::Import(config, credentials, import_config, cb) => {
                debug!(target: "wallet_command_executor", "Import command received");
                let result6 = self._import(&config, &credentials, &import_config).await;
                cb(result6);
            }
            WalletCommand::GenerateKey(config, cb) => {
                debug!(target: "wallet_command_executor", "DeriveKey command received");
                cb(self._generate_key(config.as_ref()));
            }
        };
    }

    async fn _register_type(&self,
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
        trace!("_register_type >>> type_: {:?}", type_);

        self
            .wallet_service
            .register_wallet_storage(
                type_, create, open, close, delete, add_record, update_record_value, update_record_tags,
                add_record_tags, delete_record_tags, delete_record, get_record, get_record_id, get_record_type,
                get_record_value, get_record_tags, free_record, get_storage_metadata, set_storage_metadata,
                free_storage_metadata, search_records, search_all_records,
                get_search_total_count, fetch_search_next_record, free_search).await?;

        trace!("_register_type <<< res: ()");
        Ok(())
    }

    async fn _create<'a>(&'a self,
                         config: &'a Config,
                         credentials: &'a Credentials) -> IndyResult<()> {
        trace!("_create >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        let key_data = KeyDerivationData::from_passphrase_with_new_salt(&credentials.key, &credentials.key_derivation_method);

        let key = (self._derive_key(key_data.clone()).await)?;

        let res = self.wallet_service.create_wallet(config, credentials, (&key_data, &key)).await;

        trace!("_create <<< {:?}", res);

        res
    }

    async fn _open<'a>(&'a self,
                       config: &'a Config,
                       credentials: &'a Credentials) -> IndyResult<WalletHandle> {
        trace!("_open >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        let (wallet_handle, key_derivation_data, rekey_data) = self.wallet_service.open_wallet_prepare(config, credentials).await?;

        let key = self._derive_key(key_derivation_data).await?;

        let rekey = if let Some(rekey_data) = rekey_data {
            Some(self._derive_key(rekey_data).await?)
        } else {
            None
        };

        let res = self.wallet_service.open_wallet_continue(wallet_handle, (&key, rekey.as_ref())).await;

        trace!("_open <<< res: {:?}", res);

        res
    }

    async fn _close(&self, wallet_handle: WalletHandle) -> IndyResult<()> {
        trace!("_close >>> handle: {:?}", wallet_handle);

        self.wallet_service.close_wallet(wallet_handle).await?;

        trace!("_close <<< res: ()");
        Ok(())
    }

    async fn _delete<'a>(&'a self,
                         config: &'a Config,
                         credentials: &'a Credentials) -> IndyResult<()> {
        trace!("_delete >>> config: {:?}, credentials: {:?}", config, secret!(credentials));

        let (metadata, key_derivation_data) = self.wallet_service.delete_wallet_prepare(&config, &credentials).await?;

        let key = self._derive_key(key_derivation_data).await?;

        let res = self.wallet_service.delete_wallet_continue(config, credentials, &metadata, &key).await;

        trace!("_delete <<< {:?}", res);

        res
    }

    async fn _export(&self, wallet_handle: WalletHandle, export_config: &ExportConfig) -> IndyResult<()> {
        trace!("_export >>> handle: {:?}, export_config: {:?}", wallet_handle, secret!(export_config));

        let key_data = KeyDerivationData::from_passphrase_with_new_salt(&export_config.key, &export_config.key_derivation_method);

        let key = self._derive_key(key_data.clone()).await?;

        let res = self.wallet_service.export_wallet(wallet_handle, export_config, 0, (&key_data, &key)).await;

        trace!("_export <<< {:?}", res);

        res
    }

    async fn _import(&self,
                     config: &Config,
                     credentials: &Credentials,
                     import_config: &ExportConfig) -> IndyResult<()> {
        trace!("_import >>> config: {:?}, credentials: {:?}, import_config: {:?}",
               config, secret!(credentials), secret!(import_config));

        let (wallet_handle, key_data, import_key_data) = self.wallet_service.import_wallet_prepare(&config, &credentials, &import_config).await?;

        let import_key = self._derive_key(import_key_data).await?;
        let key = self._derive_key(key_data).await?;

        let res = self.wallet_service.import_wallet_continue(wallet_handle, config, credentials, (import_key, key)).await;

        trace!("_import <<< {:?}", res);

        res
    }

    fn _generate_key(&self, config: Option<&KeyConfig>) -> IndyResult<String> {
        trace!("_generate_key >>>config: {:?}", secret!(config));

        let seed = config.and_then(|config| config.seed.as_ref().map(String::as_str));

        let key = match self.crypto_service.convert_seed(seed)? {
            Some(seed) => randombytes::randombytes_deterministic(chacha20poly1305_ietf::KEYBYTES, &randombytes::Seed::from_slice(&seed[..])?),
            None => randombytes::randombytes(chacha20poly1305_ietf::KEYBYTES)
        };

        let res = key[..].to_base58();

        trace!("_generate_key <<< res: {:?}", res);
        Ok(res)
    }

    async fn _derive_key(&self, key_data: KeyDerivationData) -> IndyResult<MasterKey> {
        let (s, r) = futures::channel::oneshot::channel();
        crate::commands::THREADPOOL.lock().unwrap().execute(move || {
            let res = key_data.calc_master_key();
            s.send(res).unwrap();
        });
        r.await?
    }
}
