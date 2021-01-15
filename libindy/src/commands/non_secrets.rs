use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use indy_api_types::{domain::wallet::Tags, errors::prelude::*, SearchHandle, WalletHandle};
use indy_utils::next_search_handle;
use indy_wallet::{RecordOptions, SearchOptions, WalletRecord, WalletSearch, WalletService};

pub(crate) struct NonSecretsCommandExecutor {
    wallet_service: Arc<WalletService>,
    searches: Mutex<HashMap<SearchHandle, Arc<Mutex<WalletSearch>>>>,
}

impl NonSecretsCommandExecutor {
    pub(crate) fn new(wallet_service: Arc<WalletService>) -> NonSecretsCommandExecutor {
        NonSecretsCommandExecutor {
            wallet_service,
            searches: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn add_record(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        value: String,
        tags: Option<Tags>,
    ) -> IndyResult<()> {
        trace!(
            "add_record > wallet_handle {:?} type_ {:?} \
                id {:?} value {:?} tags {:?}",
            wallet_handle,
            type_,
            id,
            value,
            tags
        );

        self._check_type(&type_)?;

        self.wallet_service
            .add_record(
                wallet_handle,
                &type_,
                &id,
                &value,
                &tags.unwrap_or_else(|| Tags::new()),
            )
            .await?;

        let res = Ok(());
        trace!("add_record < {:?}", res);
        res
    }

    pub(crate) async fn update_record_value(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        value: String,
    ) -> IndyResult<()> {
        trace!(
            "update_record_value > wallet_handle {:?} type_ {:?} \
                id {:?} value {:?}",
            wallet_handle,
            type_,
            id,
            value
        );

        self._check_type(&type_)?;

        self.wallet_service
            .update_record_value(wallet_handle, &type_, &id, &value)
            .await?;

        let res = Ok(());
        trace!("update_record_value < {:?}", res);
        res
    }

    pub(crate) async fn update_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        tags: Tags,
    ) -> IndyResult<()> {
        trace!(
            "update_record_tags > wallet_handle {:?} type_ {:?} \
                id {:?} tags {:?}",
            wallet_handle,
            type_,
            id,
            tags
        );

        self._check_type(&type_)?;

        self.wallet_service
            .update_record_tags(wallet_handle, &type_, &id, &tags)
            .await?;

        let res = Ok(());
        trace!("update_record_tags < {:?}", res);
        res
    }

    pub(crate) async fn add_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        tags: Tags,
    ) -> IndyResult<()> {
        trace!(
            "add_record_tags > wallet_handle {:?} type_ {:?} \
                id {:?} tags {:?}",
            wallet_handle,
            type_,
            id,
            tags
        );

        self._check_type(&type_)?;

        self.wallet_service
            .add_record_tags(wallet_handle, &type_, &id, &tags)
            .await?;

        let res = Ok(());
        trace!("add_record_tags < {:?}", tags);
        res
    }

    pub(crate) async fn delete_record_tags(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        tag_names_json: String,
    ) -> IndyResult<()> {
        trace!(
            "delete_record_tags > wallet_handle {:?} type_ {:?} \
                id {:?} tag_names_json {:?}",
            wallet_handle,
            type_,
            id,
            tag_names_json
        );

        self._check_type(&type_)?;

        let tag_names: Vec<&str> = serde_json::from_str(&tag_names_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize tag names",
        )?;

        self.wallet_service
            .delete_record_tags(wallet_handle, &type_, &id, &tag_names)
            .await?;

        let res = Ok(());
        trace!("delete_record_tags < {:?}", res);
        res
    }

    pub(crate) async fn delete_record(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
    ) -> IndyResult<()> {
        trace!(
            "delete_record > wallet_handle {:?} type_ {:?} id {:?}",
            wallet_handle,
            type_,
            id
        );

        self._check_type(&type_)?;

        self.wallet_service
            .delete_record(wallet_handle, &type_, &id)
            .await?;

        let res = Ok(());
        trace!("delete_record < {:?}", res);
        res
    }

    pub(crate) async fn get_record(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        id: String,
        options_json: String,
    ) -> IndyResult<String> {
        trace!(
            "get_record > wallet_handle {:?} type_ {:?} \
                id {:?} options_json {:?}",
            wallet_handle,
            type_,
            id,
            options_json
        );

        self._check_type(&type_)?;

        serde_json::from_str::<RecordOptions>(&options_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize options",
        )?;

        let record = self
            .wallet_service
            .get_record(wallet_handle, &type_, &id, &options_json)
            .await?;

        let record = serde_json::to_string(&record).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot serialize WalletRecord",
        )?;

        let res = Ok(record);
        trace!("get_record < {:?}", res);
        res
    }

    pub(crate) async fn open_search(
        &self,
        wallet_handle: WalletHandle,
        type_: String,
        query_json: String,
        options_json: String,
    ) -> IndyResult<SearchHandle> {
        trace!(
            "open_search > wallet_handle {:?} type_ {:?} \
                query_json {:?} options_json {:?}",
            wallet_handle,
            type_,
            query_json,
            options_json
        );

        self._check_type(&type_)?;

        serde_json::from_str::<SearchOptions>(&options_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize options",
        )?;

        let search = self
            .wallet_service
            .search_records(wallet_handle, &type_, &query_json, &options_json)
            .await?;

        let search_handle = next_search_handle();

        self.searches
            .lock()
            .await
            .insert(search_handle, Arc::new(Mutex::new(search)));

        let res = Ok(search_handle);
        trace!("open_search < {:?}", search_handle);
        res
    }

    pub(crate) async fn fetch_search_next_records(
        &self,
        wallet_handle: WalletHandle,
        wallet_search_handle: SearchHandle,
        count: usize,
    ) -> IndyResult<String> {
        trace!(
            "fetch_search_next_records > wallet_handle {:?} wallet_search_handle {:?} count {:?}",
            wallet_handle,
            wallet_search_handle,
            count
        );

        let search_mut = {
            self.searches
                .lock()
                .await
                .get(&wallet_search_handle)
                .ok_or_else(|| {
                    err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown search handle")
                })?
                .clone()
        };

        let mut search = search_mut.lock().await;

        let mut records: Vec<WalletRecord> = Vec::new();

        for _ in 0..count {
            match search.fetch_next_record().await? {
                Some(record) => records.push(record),
                None => break,
            }
        }

        let search_result = SearchRecords {
            total_count: search.get_total_count()?,
            records: if records.is_empty() {
                None
            } else {
                Some(records)
            },
        };

        let search_result = serde_json::to_string(&search_result).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize SearchRecords",
        )?;

        let res = Ok(search_result);
        trace!("fetch_search_next_records < {:?}", res);
        res
    }

    pub(crate) async fn close_search(&self, wallet_search_handle: SearchHandle) -> IndyResult<()> {
        trace!(
            "close_search > wallet_search_handle {:?}",
            wallet_search_handle
        );

        self.searches
            .lock()
            .await
            .remove(&wallet_search_handle)
            .ok_or_else(|| err_msg(IndyErrorKind::InvalidWalletHandle, "Unknown search handle"))?;

        let res = Ok(());
        trace!("close_search < {:?}", res);
        res
    }

    fn _check_type(&self, type_: &str) -> IndyResult<()> {
        if type_.starts_with(WalletService::PREFIX) {
            Err(err_msg(
                IndyErrorKind::WalletAccessFailed,
                format!("Record of type \"{}\" is not available for fetching", type_),
            ))?;
        }

        Ok(())
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRecords {
    pub total_count: Option<usize>,
    pub records: Option<Vec<WalletRecord>>,
}
