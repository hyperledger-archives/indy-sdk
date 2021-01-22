use std::sync::Arc;

use indy_api_types::{
    domain::wallet::{Config, Credentials, ExportConfig, KeyConfig},
    errors::prelude::*,
    wallet::*,
    WalletHandle,
};
use indy_utils::crypto::{
    chacha20poly1305_ietf, chacha20poly1305_ietf::Key as MasterKey, randombytes,
};
use indy_wallet::{KeyDerivationData, WalletService};
use rust_base58::ToBase58;

use crate::services::crypto::CryptoService;

pub(crate) struct WalletController {
    wallet_service: Arc<WalletService>,
    crypto_service: Arc<CryptoService>,
}

impl WalletController {
    pub(crate) fn new(
        wallet_service: Arc<WalletService>,
        crypto_service: Arc<CryptoService>,
    ) -> WalletController {
        WalletController {
            wallet_service,
            crypto_service,
        }
    }

    pub(crate) fn register_type(
        &self,
        type_: String,
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
        free_search: WalletFreeSearch,
    ) -> IndyResult<()> {
        trace!("register_type > type_: {:?}", type_);

        self.wallet_service.register_wallet_storage(
            &type_,
            create,
            open,
            close,
            delete,
            add_record,
            update_record_value,
            update_record_tags,
            add_record_tags,
            delete_record_tags,
            delete_record,
            get_record,
            get_record_id,
            get_record_type,
            get_record_value,
            get_record_tags,
            free_record,
            get_storage_metadata,
            set_storage_metadata,
            free_storage_metadata,
            search_records,
            search_all_records,
            get_search_total_count,
            fetch_search_next_record,
            free_search,
        )?;

        trace!("register_type < res: ()");
        Ok(())
    }

    pub(crate) async fn create(&self, config: Config, credentials: Credentials) -> IndyResult<()> {
        trace!(
            "_create > config: {:?} credentials: {:?}",
            &config,
            secret!(&credentials)
        );

        let key_data = KeyDerivationData::from_passphrase_with_new_salt(
            &credentials.key,
            &credentials.key_derivation_method,
        );

        let key = Self::_derive_key(key_data.clone()).await?;

        let res = self
            .wallet_service
            .create_wallet(&config, &credentials, (&key_data, &key))
            .await;

        trace!("create < {:?}", res);
        res
    }

    pub(crate) async fn open(&self, config: Config, credentials: Credentials) -> IndyResult<WalletHandle> {
        trace!(
            "open > config: {:?} credentials: {:?}",
            &config,
            secret!(&credentials)
        );
        // TODO: try to refactor to avoid usage of continue methods

        let (wallet_handle, key_derivation_data, rekey_data) = self
            .wallet_service
            .open_wallet_prepare(&config, &credentials)
            .await?;

        let key = Self::_derive_key(key_derivation_data).await?;

        let rekey = if let Some(rekey_data) = rekey_data {
            Some(Self::_derive_key(rekey_data).await?)
        } else {
            None
        };

        let res = self
            .wallet_service
            .open_wallet_continue(wallet_handle, (&key, rekey.as_ref()))
            .await;

        trace!("open < res: {:?}", res);

        res
    }

    pub(crate) async fn close(&self, wallet_handle: WalletHandle) -> IndyResult<()> {
        trace!("close > handle: {:?}", wallet_handle);

        self.wallet_service.close_wallet(wallet_handle).await?;

        trace!("close < res: ()");
        Ok(())
    }

    pub(crate) async fn delete(
        &self,
        config: Config,
        credentials: Credentials,
    ) -> IndyResult<()> {
        trace!(
            "delete > config: {:?} credentials: {:?}",
            &config,
            secret!(&credentials)
        );
        // TODO: try to refactor to avoid usage of continue methods

        let (metadata, key_derivation_data) = self
            .wallet_service
            .delete_wallet_prepare(&config, &credentials)
            .await?;

        let key = Self::_derive_key(key_derivation_data).await?;

        let res = self
            .wallet_service
            .delete_wallet_continue(&config, &credentials, &metadata, &key)
            .await;

        trace!("delete < {:?}", res);
        res
    }

    pub(crate) async fn export(
        &self,
        wallet_handle: WalletHandle,
        export_config: ExportConfig,
    ) -> IndyResult<()> {
        trace!(
            "export > handle: {:?} export_config: {:?}",
            wallet_handle,
            secret!(&export_config)
        );

        let key_data = KeyDerivationData::from_passphrase_with_new_salt(
            &export_config.key,
            &export_config.key_derivation_method,
        );

        let key = Self::_derive_key(key_data.clone()).await?;

        let res = self
            .wallet_service
            .export_wallet(wallet_handle, &export_config, 0, (&key_data, &key))
            .await;

        trace!("export < {:?}", res);
        res
    }

    pub(crate) async fn import(
        &self,
        config: Config,
        credentials: Credentials,
        import_config: ExportConfig,
    ) -> IndyResult<()> {
        trace!(
            "import > config: {:?} credentials: {:?} import_config: {:?}",
            &config,
            secret!(&credentials),
            secret!(&import_config)
        );
        // TODO: try to refactor to avoid usage of continue methods

        let (wallet_handle, key_data, import_key_data) = self
            .wallet_service
            .import_wallet_prepare(&config, &credentials, &import_config)
            .await?;

        let import_key = Self::_derive_key(import_key_data).await?;
        let key = Self::_derive_key(key_data).await?;

        let res = self
            .wallet_service
            .import_wallet_continue(wallet_handle, &config, &credentials, (import_key, key))
            .await;

        trace!("import < {:?}", res);

        res
    }

    pub(crate) fn generate_key(
        &self,
        config: Option<KeyConfig>,
    ) -> IndyResult<String> {
        trace!("generate_key > config: {:?}", secret!(&config));

        let seed = config
            .as_ref()
            .and_then(|config| config.seed.as_ref().map(String::as_str));

        let key = match self.crypto_service.convert_seed(seed)? {
            Some(seed) => randombytes::randombytes_deterministic(
                chacha20poly1305_ietf::KEYBYTES,
                &randombytes::Seed::from_slice(&seed[..])?,
            ),
            None => randombytes::randombytes(chacha20poly1305_ietf::KEYBYTES),
        };

        let res = key[..].to_base58();

        trace!("generate_key < res: {:?}", res);
        Ok(res)
    }

    async fn _derive_key(key_data: KeyDerivationData) -> IndyResult<MasterKey> {
        let (s, r) = futures::channel::oneshot::channel();
        crate::commands::THREADPOOL
            .lock()
            .unwrap()
            .execute(move || {
                let res = key_data.calc_master_key();
                s.send(res).unwrap();
            });
        r.await?
    }
}
