use std::{collections::HashMap, sync::Arc};

use indy_api_types::{errors::prelude::*, PoolHandle, WalletHandle};
use indy_wallet::{RecordOptions, SearchOptions, WalletService};
use rust_base58::{FromBase58, ToBase58};
use serde_json;

use crate::{
    domain::crypto::did::{
        Did, DidMetadata, DidMethod, DidValue, DidWithMeta, MyDidInfo, TemporaryDid, TheirDid,
        TheirDidInfo,
    },
    domain::crypto::key::KeyInfo,
    domain::{
        ledger::attrib::{AttribData, Endpoint, GetAttrReplyResult},
        ledger::nym::{GetNymReplyResult, GetNymResultDataV0},
        ledger::response::Reply,
        pairwise::Pairwise,
    },
    services::{crypto::CryptoService, ledger::LedgerService, pool::PoolService},
};

pub struct DidCommandExecutor {
    wallet_service: Arc<WalletService>,
    crypto_service: Arc<CryptoService>,
    ledger_service: Arc<LedgerService>,
    pool_service: Arc<PoolService>,
}

impl DidCommandExecutor {
    pub fn new(
        wallet_service: Arc<WalletService>,
        crypto_service: Arc<CryptoService>,
        ledger_service: Arc<LedgerService>,
        pool_service: Arc<PoolService>,
    ) -> DidCommandExecutor {
        DidCommandExecutor {
            wallet_service,
            crypto_service,
            ledger_service,
            pool_service,
        }
    }

    pub(crate) async fn create_and_store_my_did(
        &self,
        wallet_handle: WalletHandle,
        my_did_info: MyDidInfo,
    ) -> IndyResult<(String, String)> {
        debug!(
            "create_and_store_my_did > wallet_handle {:?} my_did_info_json {:?}",
            wallet_handle,
            secret!(&my_did_info)
        );

        let (did, key) = self.crypto_service.create_my_did(&my_did_info).await?;

        if let Ok(current_did) = self._wallet_get_my_did(wallet_handle, &did.did).await {
            if did.verkey == current_did.verkey {
                let res = Ok((did.did.0, did.verkey));
                debug!("create_and_store_my_did < already exists {:?}", res);
                return res;
            } else {
                let res = Err(err_msg(
                    IndyErrorKind::DIDAlreadyExists,
                    format!(
                        "DID \"{}\" already exists but with different Verkey. \
                                            You should specify Seed used for initial generation",
                        did.did.0
                    ),
                ));

                debug!(
                    "create_and_store_my_did < exists different verkey {:?}",
                    res
                );

                return res;
            }
        }

        self.wallet_service
            .add_indy_object(wallet_handle, &did.did.0, &did, &HashMap::new())
            .await?;

        let _ = self
            .wallet_service
            .add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())
            .await
            .ok();

        let res = Ok((did.did.0, did.verkey));
        debug!("create_and_store_my_did < {:?}", res);
        res
    }

    pub(crate) async fn replace_keys_start(
        &self,
        wallet_handle: WalletHandle,
        key_info: KeyInfo,
        my_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "replace_keys_start > wallet_handle {:?} key_info_json {:?} my_did {:?}",
            wallet_handle,
            secret!(&key_info),
            my_did
        );

        self.crypto_service.validate_did(&my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, &my_did).await?;

        let temporary_key = self.crypto_service.create_key(&key_info).await?;

        let my_temporary_did = TemporaryDid {
            did: my_did.did,
            verkey: temporary_key.verkey.clone(),
        };

        self.wallet_service
            .add_indy_object(
                wallet_handle,
                &temporary_key.verkey,
                &temporary_key,
                &HashMap::new(),
            )
            .await?;

        self.wallet_service
            .add_indy_object(
                wallet_handle,
                &my_temporary_did.did.0,
                &my_temporary_did,
                &HashMap::new(),
            )
            .await?;

        let res = Ok(my_temporary_did.verkey);
        debug!("replace_keys_start < {:?}", res);
        res
    }

    pub(crate) async fn replace_keys_apply(
        &self,
        wallet_handle: WalletHandle,
        my_did: DidValue,
    ) -> IndyResult<()> {
        debug!(
            "replace_keys_apply > wallet_handle {:?} my_did {:?}",
            wallet_handle, my_did
        );

        self.crypto_service.validate_did(&my_did)?;

        let my_did = self._wallet_get_my_did(wallet_handle, &my_did).await?;

        let my_temporary_did: TemporaryDid = self
            .wallet_service
            .get_indy_object(wallet_handle, &my_did.did.0, &RecordOptions::id_value())
            .await?;

        let my_did = Did::from(my_temporary_did);

        self.wallet_service
            .update_indy_object(wallet_handle, &my_did.did.0, &my_did)
            .await?;

        self.wallet_service
            .delete_indy_record::<TemporaryDid>(wallet_handle, &my_did.did.0)
            .await?;

        let res = Ok(());
        debug!("replace_keys_apply < {:?}", res);
        res
    }

    pub(crate) async fn store_their_did(
        &self,
        wallet_handle: WalletHandle,
        their_did_info: TheirDidInfo,
    ) -> IndyResult<()> {
        debug!(
            "store_their_did > wallet_handle {:?} their_did_info {:?}",
            wallet_handle, their_did_info
        );

        let their_did = self
            .crypto_service
            .create_their_did(&their_did_info)
            .await?;

        self.wallet_service
            .upsert_indy_object(wallet_handle, &their_did.did.0, &their_did)
            .await?;

        let res = Ok(());
        debug!("store_their_did < {:?}", res);
        res
    }

    pub(crate) async fn get_my_did_with_meta(
        &self,
        wallet_handle: WalletHandle,
        my_did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "get_my_did_with_meta > wallet_handle {:?} my_did {:?}",
            wallet_handle, my_did
        );

        let did = self
            .wallet_service
            .get_indy_object::<Did>(wallet_handle, &my_did.0, &RecordOptions::id_value())
            .await?;

        let metadata = self
            .wallet_service
            .get_indy_opt_object::<DidMetadata>(
                wallet_handle,
                &did.did.0,
                &RecordOptions::id_value(),
            )
            .await?;

        let temp_verkey = self
            .wallet_service
            .get_indy_opt_object::<TemporaryDid>(
                wallet_handle,
                &did.did.0,
                &RecordOptions::id_value(),
            )
            .await?;

        let did_with_meta = DidWithMeta {
            did: did.did,
            verkey: did.verkey,
            temp_verkey: temp_verkey.map(|tv| tv.verkey),
            metadata: metadata.map(|m| m.value),
        };

        let did_with_meta = serde_json::to_string(&did_with_meta)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize DID")?;

        let res = Ok(did_with_meta);
        debug!("get_my_did_with_meta < {:?}", res);
        res
    }

    pub(crate) async fn list_my_dids_with_meta(
        &self,
        wallet_handle: WalletHandle,
    ) -> IndyResult<String> {
        debug!("list_my_dids_with_meta > wallet_handle {:?}", wallet_handle);

        let mut did_search = self
            .wallet_service
            .search_indy_records::<Did>(wallet_handle, "{}", &SearchOptions::id_value())
            .await?;

        let mut metadata_search = self
            .wallet_service
            .search_indy_records::<DidMetadata>(wallet_handle, "{}", &SearchOptions::id_value())
            .await?;

        let mut temporarydid_search = self
            .wallet_service
            .search_indy_records::<TemporaryDid>(wallet_handle, "{}", &SearchOptions::id_value())
            .await?;

        let mut dids: Vec<DidWithMeta> = Vec::new();

        let mut metadata_map: HashMap<String, String> = HashMap::new();
        let mut temporarydid_map: HashMap<String, String> = HashMap::new();

        while let Some(record) = metadata_search.fetch_next_record().await? {
            let did_id = record.get_id();

            let tup: DidMetadata = record
                .get_value()
                .ok_or(err_msg(
                    IndyErrorKind::InvalidState,
                    "No value for DID record",
                ))
                .and_then(|tags_json| {
                    serde_json::from_str(&tags_json).to_indy(
                        IndyErrorKind::InvalidState,
                        format!("Cannot deserialize Did {:?}", did_id),
                    )
                })?;

            metadata_map.insert(String::from(did_id), tup.value);
        }

        while let Some(record) = temporarydid_search.fetch_next_record().await? {
            let did_id = record.get_id();

            let did: TemporaryDid = record
                .get_value()
                .ok_or(err_msg(
                    IndyErrorKind::InvalidState,
                    "No value for DID record",
                ))
                .and_then(|tags_json| {
                    serde_json::from_str(&tags_json).to_indy(
                        IndyErrorKind::InvalidState,
                        format!("Cannot deserialize Did {:?}", did_id),
                    )
                })?;

            temporarydid_map.insert(did.did.0, did.verkey);
        }

        while let Some(did_record) = did_search.fetch_next_record().await? {
            let did_id = did_record.get_id();

            let did: Did = did_record
                .get_value()
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No value for DID record"))
                .and_then(|tags_json| {
                    serde_json::from_str(&tags_json).to_indy(
                        IndyErrorKind::InvalidState,
                        format!("Cannot deserialize Did {:?}", did_id),
                    )
                })?;

            let temp_verkey = temporarydid_map.remove(&did.did.0);
            let metadata = metadata_map.remove(&did.did.0);

            let did_with_meta = DidWithMeta {
                did: did.did,
                verkey: did.verkey,
                temp_verkey: temp_verkey,
                metadata: metadata,
            };

            dids.push(did_with_meta);
        }

        let dids = serde_json::to_string(&dids)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize DIDs list")?;

        let res = Ok(dids);
        debug!("list_my_dids_with_meta < {:?}", res);
        res
    }

    pub(crate) async fn key_for_did(
        &self,
        pool_handle: PoolHandle,
        wallet_handle: WalletHandle,
        did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "key_for_did > pool_handle {:?} wallet_handle {:?} did {:?}",
            pool_handle, wallet_handle, did
        );

        self.crypto_service.validate_did(&did)?;

        // Look to my did
        let my_did = match self._wallet_get_my_did(wallet_handle, &did).await {
            Ok(my_did) => Some(my_did),
            Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => None,
            Err(err) => Err(err)?,
        };

        if let Some(my_did) = my_did {
            let res = Ok(my_did.verkey);
            debug!("key_for_did < my key {:?}", res);
            return res;
        }

        // look to their did
        let their_did = match self._wallet_get_their_did(wallet_handle, &did).await {
            Ok(did) => did,
            // No their their_did present in the wallet. Defer this command until it is fetched from ledger.
            Err(ref err) if err.kind() == IndyErrorKind::WalletItemNotFound => {
                self._fetch_their_did_from_ledger(wallet_handle, pool_handle, &did)
                    .await?
            }
            Err(err) => Err(err)?,
        };

        let res = Ok(their_did.verkey);
        debug!("key_for_did < their did {:?}", res);
        res
    }

    pub(crate) async fn key_for_local_did(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "key_for_local_did > wallet_handle {:?} did {:?}",
            wallet_handle, did
        );

        self.crypto_service.validate_did(&did)?;

        // Look to my did
        let my_did = match self._wallet_get_my_did(wallet_handle, &did).await {
            Ok(my_did) => Some(my_did),
            Err(err) if err.kind() == IndyErrorKind::WalletItemNotFound => None,
            Err(err) => Err(err)?,
        };

        if let Some(my_did) = my_did {
            let res = Ok(my_did.verkey);
            debug!("key_for_local_did < my {:?}", res);
            return res;
        }

        // look to their did
        let their_did = self._wallet_get_their_did(wallet_handle, &did).await?;

        let res = Ok(their_did.verkey);
        debug!("key_for_local_did < {:?}", res);
        res
    }

    pub(crate) async fn set_endpoint_for_did(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
        endpoint: Endpoint,
    ) -> IndyResult<()> {
        debug!(
            "set_endpoint_for_did > wallet_handle {:?} did {:?} endpoint {:?}",
            wallet_handle, did, endpoint
        );

        self.crypto_service.validate_did(&did)?;

        if let Some(ref transport_key) = endpoint.verkey {
            self.crypto_service.validate_key(transport_key).await?;
        }

        self.wallet_service
            .upsert_indy_object(wallet_handle, &did.0, &endpoint)
            .await?;

        let res = Ok(());
        debug!("set_endpoint_for_did < {:?}", res);
        res
    }

    pub(crate) async fn get_endpoint_for_did(
        &self,
        wallet_handle: WalletHandle,
        pool_handle: PoolHandle,
        did: DidValue,
    ) -> IndyResult<(String, Option<String>)> {
        debug!(
            "get_endpoint_for_did > wallet_handle {:?} \
                pool_handle {:?} did {:?}",
            wallet_handle, pool_handle, did
        );

        self.crypto_service.validate_did(&did)?;

        let endpoint = match self
            .wallet_service
            .get_indy_object::<Endpoint>(wallet_handle, &did.0, &RecordOptions::id_value())
            .await
        {
            Ok(endpoint) => endpoint,
            Err(err) if err.kind() == IndyErrorKind::WalletItemNotFound => {
                self._fetch_attrib_from_ledger(wallet_handle, pool_handle, &did)
                    .await?
            }
            Err(err) => Err(err)?,
        };

        let res = Ok((endpoint.ha, endpoint.verkey));
        debug!("get_endpoint_for_did < {:?}", res);
        res
    }

    pub(crate) async fn set_did_metadata(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
        metadata: String,
    ) -> IndyResult<()> {
        debug!(
            "set_did_metadata > wallet_handle {:?} did {:?} metadata {:?}",
            wallet_handle, did, metadata
        );

        self.crypto_service.validate_did(&did)?;

        let metadata = DidMetadata { value: metadata };

        self.wallet_service
            .upsert_indy_object(wallet_handle, &did.0, &metadata)
            .await?;

        let res = Ok(());
        debug!("set_did_metadata < {:?}", res);
        res
    }

    pub(crate) async fn get_did_metadata(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
    ) -> IndyResult<String> {
        debug!(
            "get_did_metadata > wallet_handle {:?} did {:?}",
            wallet_handle, did
        );

        self.crypto_service.validate_did(&did)?;

        let metadata = self
            .wallet_service
            .get_indy_object::<DidMetadata>(wallet_handle, &did.0, &RecordOptions::id_value())
            .await?;

        let res = Ok(metadata.value);
        debug!("get_did_metadata < {:?}", res);
        res
    }

    pub(crate) async fn abbreviate_verkey(
        &self,
        did: DidValue,
        verkey: String,
    ) -> IndyResult<String> {
        debug!("abbreviate_verkey > did {:?} verkey {:?}", did, verkey);

        self.crypto_service.validate_did(&did)?;
        self.crypto_service.validate_key(&verkey).await?;

        if !did.is_abbreviatable() {
            let res = Ok(verkey);
            debug!("abbreviate_verkey < not abbreviatable {:?}", res);
            return res;
        }

        let did = &did.to_unqualified().0.from_base58()?;
        let dverkey = &verkey.from_base58()?;

        let (first_part, second_part) = dverkey.split_at(16);

        let res = if first_part.eq(did.as_slice()) {
            format!("~{}", second_part.to_base58())
        } else {
            verkey
        };

        let res = Ok(res);
        debug!("abbreviate_verkey < {:?}", res);
        res
    }

    pub(crate) async fn qualify_did(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
        method: DidMethod,
    ) -> IndyResult<String> {
        debug!(
            "qualify_did > wallet_handle {:?} curr_did {:?} method {:?}",
            wallet_handle, did, method
        );

        self.crypto_service.validate_did(&did)?;

        let mut curr_did: Did = self
            .wallet_service
            .get_indy_object::<Did>(wallet_handle, &did.0, &RecordOptions::id_value())
            .await?;

        curr_did.did = DidValue::new(&did.to_short().0, Some(&method.0));

        self.wallet_service
            .delete_indy_record::<Did>(wallet_handle, &did.0)
            .await?;

        self.wallet_service
            .add_indy_object(wallet_handle, &curr_did.did.0, &curr_did, &HashMap::new())
            .await?;

        // move temporary Did
        if let Ok(mut temp_did) = self
            .wallet_service
            .get_indy_object::<TemporaryDid>(wallet_handle, &did.0, &RecordOptions::id_value())
            .await
        {
            temp_did.did = curr_did.did.clone();

            self.wallet_service
                .delete_indy_record::<TemporaryDid>(wallet_handle, &did.0)
                .await?;

            self.wallet_service
                .add_indy_object(wallet_handle, &curr_did.did.0, &temp_did, &HashMap::new())
                .await?;
        }

        // move metadata
        self._update_dependent_entity_reference::<DidMetadata>(
            wallet_handle,
            &did.0,
            &curr_did.did.0,
        )
        .await?;

        // move endpoint
        self._update_dependent_entity_reference::<Endpoint>(wallet_handle, &did.0, &curr_did.did.0)
            .await?;

        // move all pairwise
        let mut pairwise_search = self
            .wallet_service
            .search_indy_records::<Pairwise>(wallet_handle, "{}", &RecordOptions::id_value())
            .await?;

        while let Some(pairwise_record) = pairwise_search.fetch_next_record().await? {
            let mut pairwise: Pairwise = pairwise_record
                .get_value()
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No value for Pairwise record"))
                .and_then(|pairwise_json| {
                    serde_json::from_str(&pairwise_json).map_err(|err| {
                        IndyError::from_msg(
                            IndyErrorKind::InvalidState,
                            format!("Cannot deserialize Pairwise {:?}", err),
                        )
                    })
                })?;

            if pairwise.my_did.eq(&did) {
                pairwise.my_did = curr_did.did.clone();

                self.wallet_service
                    .update_indy_object(wallet_handle, &pairwise.their_did.0, &pairwise)
                    .await?;
            }
        }

        let res = Ok(curr_did.did.0);
        debug!("qualify_did < {:?}", res);
        res
    }

    pub(crate) async fn get_nym_ack_process_and_store_their_did(
        &self,
        wallet_handle: WalletHandle,
        did: DidValue,
        get_nym_reply_result: IndyResult<String>,
    ) -> IndyResult<TheirDid> {
        trace!(
            "get_nym_ack_process_and_store_their_did > \
                wallet_handle {:?} get_nym_reply_result {:?}",
            wallet_handle,
            get_nym_reply_result
        );

        let get_nym_reply = get_nym_reply_result?;

        let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_reply)
            .to_indy(
                IndyErrorKind::InvalidState,
                "Invalid GetNymReplyResult json",
            )?;

        let their_did_info = match get_nym_response.result() {
            GetNymReplyResult::GetNymReplyResultV0(res) => {
                if let Some(data) = &res.data {
                    let gen_nym_result_data: GetNymResultDataV0 = serde_json::from_str(data)
                        .to_indy(IndyErrorKind::InvalidState, "Invalid GetNymResultData json")?;

                    TheirDidInfo::new(
                        gen_nym_result_data.dest.qualify(did.get_method()),
                        gen_nym_result_data.verkey,
                    )
                } else {
                    return Err(err_msg(
                        IndyErrorKind::WalletItemNotFound,
                        "Their DID isn't found on the ledger",
                    )); //TODO FIXME use separate error
                }
            }
            GetNymReplyResult::GetNymReplyResultV1(res) => TheirDidInfo::new(
                res.txn.data.did.qualify(did.get_method()),
                res.txn.data.verkey,
            ),
        };

        let their_did = self
            .crypto_service
            .create_their_did(&their_did_info)
            .await?;

        self.wallet_service
            .add_indy_object(wallet_handle, &their_did.did.0, &their_did, &HashMap::new())
            .await?;

        trace!("get_nym_ack_process_and_store_their_did <<<");

        Ok(their_did)
    }

    async fn _update_dependent_entity_reference<T>(
        &self,
        wallet_handle: WalletHandle,
        id: &str,
        new_id: &str,
    ) -> IndyResult<()>
    where
        T: ::serde::Serialize + ::serde::de::DeserializeOwned + Sized,
    {
        if let Ok(record) = self
            .wallet_service
            .get_indy_record_value::<T>(wallet_handle, id, "{}")
            .await
        {
            self.wallet_service
                .delete_indy_record::<T>(wallet_handle, id)
                .await?;
            self.wallet_service
                .add_indy_record::<T>(wallet_handle, new_id, &record, &HashMap::new())
                .await?;
        }

        Ok(())
    }

    async fn _get_attrib_ack_process_store_endpoint_to_wallet(
        &self,
        wallet_handle: WalletHandle,
        get_attrib_reply_result: IndyResult<String>,
    ) -> IndyResult<Endpoint> {
        trace!(
            "_get_attrib_ack_process_store_endpoint_to_wallet > \
                wallet_handle {:?} get_attrib_reply_result {:?}",
            wallet_handle,
            get_attrib_reply_result
        );

        let get_attrib_reply = get_attrib_reply_result?;

        let get_attrib_reply: Reply<GetAttrReplyResult> = serde_json::from_str(&get_attrib_reply)
            .to_indy(
            IndyErrorKind::InvalidState,
            "Invalid GetAttrReplyResult json",
        )?;

        let (raw, did) = match get_attrib_reply.result() {
            GetAttrReplyResult::GetAttrReplyResultV0(res) => (res.data, res.dest),
            GetAttrReplyResult::GetAttrReplyResultV1(res) => (res.txn.data.raw, res.txn.data.did),
        };

        let attrib_data: AttribData = serde_json::from_str(&raw)
            .to_indy(IndyErrorKind::InvalidState, "Invalid GetAttReply json")?;

        let endpoint = Endpoint::new(attrib_data.endpoint.ha, attrib_data.endpoint.verkey);

        self.wallet_service
            .add_indy_object(wallet_handle, &did.0, &endpoint, &HashMap::new())
            .await?;

        let res = Ok(endpoint);

        trace!(
            "_get_attrib_ack_process_store_endpoint_to_wallet < {:?}",
            res
        );

        res
    }

    async fn _fetch_their_did_from_ledger(
        &self,
        wallet_handle: WalletHandle,
        pool_handle: PoolHandle,
        did: &DidValue,
    ) -> IndyResult<TheirDid> {
        // TODO we need passing of my_did as identifier
        // TODO: FIXME: Remove this unwrap by sending GetNymAck with the error.
        let get_nym_request = self
            .ledger_service
            .build_get_nym_request(None, did)
            .unwrap();

        let did = did.clone();

        let get_nym_reply_result = self
            .pool_service
            .send_tx(pool_handle, &get_nym_request)
            .await;

        self.get_nym_ack_process_and_store_their_did(wallet_handle, did, get_nym_reply_result)
            .await
    }

    async fn _fetch_attrib_from_ledger(
        &self,
        wallet_handle: WalletHandle,
        pool_handle: PoolHandle,
        did: &DidValue,
    ) -> IndyResult<Endpoint> {
        // TODO we need passing of my_did as identifier
        // TODO: FIXME: Remove this unwrap by sending GetAttribAck with the error.
        let get_attrib_request = self
            .ledger_service
            .build_get_attrib_request(None, did, Some("endpoint"), None, None)
            .unwrap();

        let get_attrib_reply_result = self
            .pool_service
            .send_tx(pool_handle, &get_attrib_request)
            .await;

        self._get_attrib_ack_process_store_endpoint_to_wallet(
            wallet_handle,
            get_attrib_reply_result,
        )
        .await
    }

    async fn _wallet_get_my_did(
        &self,
        wallet_handle: WalletHandle,
        my_did: &DidValue,
    ) -> IndyResult<Did> {
        self.wallet_service
            .get_indy_object(wallet_handle, &my_did.0, &RecordOptions::id_value())
            .await
    }

    async fn _wallet_get_their_did(
        &self,
        wallet_handle: WalletHandle,
        their_did: &DidValue,
    ) -> IndyResult<TheirDid> {
        self.wallet_service
            .get_indy_object(wallet_handle, &their_did.0, &RecordOptions::id_value())
            .await
    }
}
