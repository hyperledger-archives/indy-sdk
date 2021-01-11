use std::{
    collections::{HashMap, HashSet},
    ops::DerefMut,
    sync::Arc,
};

use futures::lock::Mutex;
use indy_api_types::{errors::prelude::*, SearchHandle, WalletHandle};
use indy_utils::next_search_handle;
use indy_wallet::{RecordOptions, SearchOptions, WalletRecord, WalletSearch, WalletService};
use serde_json::Value;
use ursa::cl::{new_nonce, RevocationRegistry, Witness};

use crate::{
    domain::{
        anoncreds::{
            credential::{Credential, CredentialInfo},
            credential_attr_tag_policy::CredentialAttrTagPolicy,
            credential_definition::{
                cred_defs_map_to_cred_defs_v1_map, CredentialDefinition, CredentialDefinitionId,
                CredentialDefinitionV1, CredentialDefinitions,
            },
            credential_for_proof_request::{CredentialsForProofRequest, RequestedCredential},
            requested_credential::RequestedCredentials,
            revocation_registry_definition::{
                RevocationRegistryDefinition, RevocationRegistryDefinitionV1,
            },
            revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1},
            revocation_state::{RevocationState, RevocationStates},
            schema::{schemas_map_to_schemas_v1_map, Schemas},
        },
        anoncreds::{
            credential_offer::CredentialOffer,
            credential_request::{CredentialRequest, CredentialRequestMetadata},
            master_secret::MasterSecret,
            proof_request::{
                NonRevocedInterval, PredicateInfo, ProofRequest, ProofRequestExtraQuery,
            },
        },
        crypto::did::DidValue,
    },
    services::{
        anoncreds::{
            helpers::{get_non_revoc_interval, parse_cred_rev_id},
            AnoncredsService,
        },
        blob_storage::BlobStorageService,
        crypto::CryptoService,
    },
    utils::wql::Query,
};

use super::tails::SDKTailsAccessor;

struct SearchForProofRequest {
    search: WalletSearch,
    interval: Option<NonRevocedInterval>,
    predicate_info: Option<PredicateInfo>,
}

impl SearchForProofRequest {
    fn new(
        search: WalletSearch,
        interval: Option<NonRevocedInterval>,
        predicate_info: Option<PredicateInfo>,
    ) -> Self {
        Self {
            search,
            interval,
            predicate_info,
        }
    }
}

pub(crate) struct ProverCommandExecutor {
    anoncreds_service: Arc<AnoncredsService>,
    wallet_service: Arc<WalletService>,
    crypto_service: Arc<CryptoService>,
    blob_storage_service: Arc<BlobStorageService>,
    searches: Mutex<HashMap<SearchHandle, Box<WalletSearch>>>,
    searches_for_proof_requests:
        Mutex<HashMap<SearchHandle, HashMap<String, Arc<Mutex<SearchForProofRequest>>>>>,
}

impl ProverCommandExecutor {
    pub(crate) fn new(
        anoncreds_service: Arc<AnoncredsService>,
        wallet_service: Arc<WalletService>,
        crypto_service: Arc<CryptoService>,
        blob_storage_service: Arc<BlobStorageService>,
    ) -> ProverCommandExecutor {
        ProverCommandExecutor {
            anoncreds_service,
            wallet_service,
            crypto_service,
            blob_storage_service,
            searches: Mutex::new(HashMap::new()),
            searches_for_proof_requests: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn create_master_secret(
        &self,
        wallet_handle: WalletHandle,
        master_secret_id: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "create_master_secret > wallet_handle {:?} master_secret_id {:?}",
            wallet_handle, master_secret_id
        );

        let master_secret_id = master_secret_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        if self
            .wallet_service
            .record_exists::<MasterSecret>(wallet_handle, &master_secret_id)
            .await?
        {
            return Err(err_msg(
                IndyErrorKind::MasterSecretDuplicateName,
                format!("MasterSecret already exists {}", master_secret_id),
            ));
        }

        let master_secret = self.anoncreds_service.prover.new_master_secret()?;

        let master_secret = MasterSecret {
            value: master_secret,
        };

        self.wallet_service
            .add_indy_object(
                wallet_handle,
                &master_secret_id,
                &master_secret,
                &HashMap::new(),
            )
            .await?;

        let res = Ok(master_secret_id);
        debug!("create_master_secret < {:?}", res);
        res
    }

    pub(crate) async fn create_credential_request(
        &self,
        wallet_handle: WalletHandle,
        prover_did: DidValue,
        cred_offer: CredentialOffer,
        cred_def: CredentialDefinition,
        master_secret_id: String,
    ) -> IndyResult<(String, String)> {
        debug!(
            "create_credential_request > wallet_handle {:?} \
                prover_did {:?} cred_offer {:?} cred_def {:?} \
                master_secret_id: {:?}",
            wallet_handle, prover_did, cred_offer, cred_def, master_secret_id
        );

        let cred_def = CredentialDefinitionV1::from(cred_def);

        self.crypto_service.validate_did(&prover_did)?;

        let master_secret: MasterSecret = self
            ._wallet_get_master_secret(wallet_handle, &master_secret_id)
            .await?;

        let (blinded_ms, ms_blinding_data, blinded_ms_correctness_proof) = self
            .anoncreds_service
            .prover
            .new_credential_request(&cred_def, &master_secret.value, &cred_offer)?;

        let nonce = new_nonce()?;

        let credential_request = CredentialRequest {
            prover_did,
            cred_def_id: cred_offer.cred_def_id.clone(),
            blinded_ms,
            blinded_ms_correctness_proof,
            nonce,
        };

        let credential_request_metadata = CredentialRequestMetadata {
            master_secret_blinding_data: ms_blinding_data,
            nonce: credential_request.nonce.try_clone()?,
            master_secret_name: master_secret_id.to_string(),
        };

        let cred_req_json = serde_json::to_string(&credential_request).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize CredentialRequest",
        )?;

        let cred_req_metadata_json = serde_json::to_string(&credential_request_metadata).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize CredentialRequestMetadata",
        )?;

        let res = Ok((cred_req_json, cred_req_metadata_json));
        debug!("create_credential_request < {:?}", res);
        res
    }

    pub(crate) async fn set_credential_attr_tag_policy(
        &self,
        wallet_handle: WalletHandle,
        cred_def_id: CredentialDefinitionId,
        catpol: Option<CredentialAttrTagPolicy>,
        retroactive: bool,
    ) -> IndyResult<()> {
        debug!(
            "set_credential_attr_tag_policy > wallet_handle {:?} \
                cred_def_id {:?} catpol {:?} retroactive {:?}",
            wallet_handle, cred_def_id, catpol, retroactive
        );

        match catpol {
            Some(ref pol) => {
                self.wallet_service
                    .upsert_indy_object(wallet_handle, &cred_def_id.0, pol)
                    .await?;
            }
            None => {
                if self
                    .wallet_service
                    .record_exists::<CredentialAttrTagPolicy>(wallet_handle, &cred_def_id.0)
                    .await?
                {
                    self.wallet_service
                        .delete_indy_record::<CredentialAttrTagPolicy>(
                            wallet_handle,
                            &cred_def_id.0,
                        )
                        .await?;
                }
            }
        };

        // Cascade whether we updated policy or not: could be a retroactive cred attr tags reset to existing policy
        if retroactive {
            let query_json = format!(r#"{{"cred_def_id": "{}"}}"#, cred_def_id.0);

            let mut credentials_search = self
                .wallet_service
                .search_indy_records::<Credential>(
                    wallet_handle,
                    query_json.as_str(),
                    &SearchOptions::id_value(),
                )
                .await?;

            while let Some(credential_record) = credentials_search.fetch_next_record().await? {
                let (_, credential) = self._get_credential(&credential_record)?;

                let cred_tags = self
                    .anoncreds_service
                    .prover
                    .build_credential_tags(&credential, catpol.as_ref())?;

                self.wallet_service
                    .update_record_tags(
                        wallet_handle,
                        self.wallet_service.add_prefix("Credential").as_str(),
                        credential_record.get_id(),
                        &cred_tags,
                    )
                    .await?;
            }
        }

        let res = Ok(());
        debug!("set_credential_attr_tag_policy < {:?}", res);
        res
    }

    pub(crate) async fn get_credential_attr_tag_policy(
        &self,
        wallet_handle: WalletHandle,
        cred_def_id: CredentialDefinitionId,
    ) -> IndyResult<String> {
        debug!(
            "get_credential_attr_tag_policy > wallet_handle {:?} \
                cred_def_id {:?}",
            wallet_handle, cred_def_id
        );

        let catpol = self
            ._get_credential_attr_tag_policy(wallet_handle, &cred_def_id)
            .await?;

        let res = Ok(catpol);
        debug!("get_credential_attr_tag_policy < {:?}", res);
        res
    }

    pub(crate) async fn store_credential(
        &self,
        wallet_handle: WalletHandle,
        cred_id: Option<String>,
        cred_req_metadata: CredentialRequestMetadata,
        mut credential: Credential,
        cred_def: CredentialDefinition,
        rev_reg_def: Option<RevocationRegistryDefinition>,
    ) -> IndyResult<String> {
        debug!(
            "store_credential > wallet_handle {:?} \
                cred_id {:?} cred_req_metadata {:?} \
                credential {:?} cred_def {:?} \
                rev_reg_def {:?}",
            wallet_handle, cred_id, cred_req_metadata, credential, cred_def, rev_reg_def
        );

        let cred_def = CredentialDefinitionV1::from(cred_def);
        let rev_reg_def = rev_reg_def.map(RevocationRegistryDefinitionV1::from);

        let master_secret: MasterSecret = self
            ._wallet_get_master_secret(wallet_handle, &cred_req_metadata.master_secret_name)
            .await?;

        self.anoncreds_service.prover.process_credential(
            &mut credential,
            &cred_req_metadata,
            &master_secret.value,
            &cred_def,
            rev_reg_def.as_ref(),
        )?;

        credential.rev_reg = None;
        credential.witness = None;

        let out_cred_id = cred_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let catpol_json = self
            ._get_credential_attr_tag_policy(wallet_handle, &credential.cred_def_id)
            .await?;

        let catpol: Option<CredentialAttrTagPolicy> = if catpol_json.ne("null") {
            Some(serde_json::from_str(catpol_json.as_str()).to_indy(
                IndyErrorKind::InvalidState,
                "Cannot deserialize CredentialAttrTagPolicy",
            )?)
        } else {
            None
        };

        let cred_tags = self
            .anoncreds_service
            .prover
            .build_credential_tags(&credential, catpol.as_ref())?;

        self.wallet_service
            .add_indy_object(wallet_handle, &out_cred_id, &credential, &cred_tags)
            .await?;

        let res = Ok(out_cred_id);
        debug!("store_credential < {:?}", res);
        res
    }

    pub(crate) async fn get_credentials(
        &self,
        wallet_handle: WalletHandle,
        filter_json: Option<String>,
    ) -> IndyResult<String> {
        debug!(
            "get_credentials > wallet_handle {:?} filter_json {:?}",
            wallet_handle, filter_json
        );

        let filter_json = filter_json.as_deref().unwrap_or("{}");
        let mut credentials_info: Vec<CredentialInfo> = Vec::new();

        let mut credentials_search = self
            .wallet_service
            .search_indy_records::<Credential>(
                wallet_handle,
                filter_json,
                &SearchOptions::id_value(),
            )
            .await?;

        while let Some(credential_record) = credentials_search.fetch_next_record().await? {
            let (referent, credential) = self._get_credential(&credential_record)?;
            credentials_info.push(self._get_credential_info(&referent, credential))
        }

        let credentials_info_json = serde_json::to_string(&credentials_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize list of CredentialInfo",
        )?;

        let res = Ok(credentials_info_json);
        debug!("get_credentials < {:?}", res);
        res
    }

    pub(crate) async fn get_credential(
        &self,
        wallet_handle: WalletHandle,
        cred_id: String,
    ) -> IndyResult<String> {
        debug!(
            "get_credentials > wallet_handle {:?} cred_id {:?}",
            wallet_handle, cred_id
        );

        let credential: Credential = self
            .wallet_service
            .get_indy_object(wallet_handle, &cred_id, &RecordOptions::id_value())
            .await?;

        let credential_info = self._get_credential_info(&cred_id, credential);

        let credential_info_json = serde_json::to_string(&credential_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize CredentialInfo",
        )?;

        let res = Ok(credential_info_json);
        debug!("get_credential < {:?}", res);
        res
    }

    pub(crate) async fn search_credentials(
        &self,
        wallet_handle: WalletHandle,
        query_json: Option<String>,
    ) -> IndyResult<(SearchHandle, usize)> {
        debug!(
            "search_credentials > wallet_handle {:?} query_json {:?}",
            wallet_handle, query_json
        );

        let credentials_search = self
            .wallet_service
            .search_indy_records::<Credential>(
                wallet_handle,
                query_json.as_deref().unwrap_or("{}"),
                &SearchOptions::id_value(),
            )
            .await?;

        let total_count = credentials_search.get_total_count()?.unwrap_or(0);

        let handle: SearchHandle = next_search_handle();

        self.searches
            .lock()
            .await
            .insert(handle, Box::new(credentials_search));

        let res = (handle, total_count);
        trace!("search_credentials < {:?}", res);
        Ok(res)
    }

    pub(crate) async fn fetch_credentials(
        &self,
        search_handle: SearchHandle,
        count: usize,
    ) -> IndyResult<String> {
        trace!(
            "fetch_credentials > search_handle {:?} count {:?}",
            search_handle,
            count
        );

        let mut searches = self.searches.lock().await;

        let search = searches.get_mut(&search_handle).ok_or_else(|| {
            err_msg(
                IndyErrorKind::InvalidWalletHandle,
                "Unknown CredentialsSearch handle",
            )
        })?;

        let mut credentials_info: Vec<CredentialInfo> = Vec::new();

        for _ in 0..count {
            match search.fetch_next_record().await? {
                Some(credential_record) => {
                    let (referent, credential) = self._get_credential(&credential_record)?;
                    credentials_info.push(self._get_credential_info(&referent, credential))
                }
                None => break,
            }
        }

        let credentials_info_json = serde_json::to_string(&credentials_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize list of CredentialInfo",
        )?;

        let res = Ok(credentials_info_json);
        trace!("fetch_credentials < {:?}", res);
        res
    }

    pub(crate) async fn close_credentials_search(
        &self,
        search_handle: SearchHandle,
    ) -> IndyResult<()> {
        trace!(
            "close_credentials_search > search_handle {:?}",
            search_handle
        );

        self.searches
            .lock()
            .await
            .remove(&search_handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidWalletHandle,
                    "Unknown CredentialsSearch handle",
                )
            })?;

        let res = Ok(());
        trace!("close_credentials_search < {:?}", res);
        res
    }

    pub(crate) async fn get_credentials_for_proof_req(
        &self,
        wallet_handle: WalletHandle,
        proof_request: ProofRequest,
    ) -> IndyResult<String> {
        debug!(
            "get_credentials_for_proof_req > wallet_handle {:?} proof_request {:?}",
            wallet_handle, proof_request
        );

        let proof_req = proof_request.value();
        let proof_req_version = proof_request.version();

        let mut credentials_for_proof_request: CredentialsForProofRequest =
            CredentialsForProofRequest::default();

        for (attr_id, requested_attr) in proof_req.requested_attributes.iter() {
            let query = self
                .anoncreds_service
                .prover
                .extend_proof_request_restrictions(
                    &proof_req_version,
                    &requested_attr.name,
                    &requested_attr.names,
                    &attr_id,
                    &requested_attr.restrictions,
                    &None,
                )?;

            let interval =
                get_non_revoc_interval(&proof_req.non_revoked, &requested_attr.non_revoked);

            let credentials_for_attribute = self
                ._query_requested_credentials(wallet_handle, &query, None, &interval)
                .await?;

            credentials_for_proof_request
                .attrs
                .insert(attr_id.to_string(), credentials_for_attribute);
        }

        for (predicate_id, requested_predicate) in proof_req.requested_predicates.iter() {
            let query = self
                .anoncreds_service
                .prover
                .extend_proof_request_restrictions(
                    &proof_req_version,
                    &Some(requested_predicate.name.clone()),
                    &None,
                    &predicate_id,
                    &requested_predicate.restrictions,
                    &None,
                )?;

            let interval =
                get_non_revoc_interval(&proof_req.non_revoked, &requested_predicate.non_revoked);

            let credentials_for_predicate = self
                ._query_requested_credentials(
                    wallet_handle,
                    &query,
                    Some(&requested_predicate),
                    &interval,
                )
                .await?;

            credentials_for_proof_request
                .predicates
                .insert(predicate_id.to_string(), credentials_for_predicate);
        }

        let credentials_for_proof_request_json =
            serde_json::to_string(&credentials_for_proof_request).to_indy(
                IndyErrorKind::InvalidState,
                "Cannot serialize CredentialsForProofRequest",
            )?;

        let res = Ok(credentials_for_proof_request_json);
        debug!("get_credentials_for_proof_req < {:?}", res);
        res
    }

    pub(crate) async fn search_credentials_for_proof_req(
        &self,
        wallet_handle: WalletHandle,
        proof_request: ProofRequest,
        extra_query: Option<ProofRequestExtraQuery>,
    ) -> IndyResult<SearchHandle> {
        debug!(
            "search_credentials_for_proof_req > wallet_handle {:?} \
                proof_request {:?} extra_query {:?}",
            wallet_handle, proof_request, extra_query
        );

        let proof_req = proof_request.value();
        let version = proof_request.version();

        let mut credentials_for_proof_request_search =
            HashMap::<String, Arc<Mutex<SearchForProofRequest>>>::new();

        for (attr_id, requested_attr) in proof_req.requested_attributes.iter() {
            let query = self
                .anoncreds_service
                .prover
                .extend_proof_request_restrictions(
                    &version,
                    &requested_attr.name,
                    &requested_attr.names,
                    &attr_id,
                    &requested_attr.restrictions,
                    &extra_query.as_ref(),
                )?;

            let credentials_search = self
                .wallet_service
                .search_indy_records::<Credential>(
                    wallet_handle,
                    &query.to_string(),
                    &SearchOptions::id_value(),
                )
                .await?;

            let interval =
                get_non_revoc_interval(&proof_req.non_revoked, &requested_attr.non_revoked);

            credentials_for_proof_request_search.insert(
                attr_id.to_string(),
                Arc::new(Mutex::new(SearchForProofRequest::new(
                    credentials_search,
                    interval,
                    None,
                ))),
            );
        }

        for (predicate_id, requested_predicate) in proof_req.requested_predicates.iter() {
            let query = self
                .anoncreds_service
                .prover
                .extend_proof_request_restrictions(
                    &version,
                    &Some(requested_predicate.name.clone()),
                    &None,
                    &predicate_id,
                    &requested_predicate.restrictions,
                    &extra_query.as_ref(),
                )?;

            let credentials_search = self
                .wallet_service
                .search_indy_records::<Credential>(
                    wallet_handle,
                    &query.to_string(),
                    &SearchOptions::id_value(),
                )
                .await?;

            let interval =
                get_non_revoc_interval(&proof_req.non_revoked, &requested_predicate.non_revoked);

            credentials_for_proof_request_search.insert(
                predicate_id.to_string(),
                Arc::new(Mutex::new(SearchForProofRequest::new(
                    credentials_search,
                    interval,
                    Some(requested_predicate.clone()),
                ))),
            );
        }

        let search_handle = next_search_handle();

        self.searches_for_proof_requests
            .lock()
            .await
            .insert(search_handle, credentials_for_proof_request_search);

        let res = Ok(search_handle);
        debug!("search_credentials_for_proof_req < {:?}", search_handle);
        res
    }

    pub(crate) async fn fetch_credential_for_proof_request(
        &self,
        search_handle: SearchHandle,
        item_referent: String,
        count: usize,
    ) -> IndyResult<String> {
        trace!(
            "fetch_credential_for_proof_request > search_handle {:?} \
                item_referent {:?} count {:?}",
            search_handle,
            item_referent,
            count
        );

        let search_mut = {
            let mut searches = self.searches_for_proof_requests.lock().await;

            searches
                .get_mut(&search_handle)
                .ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidWalletHandle,
                        "Unknown CredentialsSearch",
                    )
                })?
                .get(&item_referent)
                .ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidWalletHandle,
                        "Unknown item referent for CredentialsSearch handle",
                    )
                })?
                .clone()
        };

        let mut search_lock = search_mut.lock().await;
        let search: &mut SearchForProofRequest = search_lock.deref_mut();

        let requested_credentials: Vec<RequestedCredential> = self
            ._get_requested_credentials(
                &mut search.search,
                search.predicate_info.as_ref(),
                &search.interval,
                Some(count),
            )
            .await?;

        let requested_credentials_json = serde_json::to_string(&requested_credentials).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize list of RequestedCredential",
        )?;

        let res = Ok(requested_credentials_json);
        trace!("fetch_credential_for_proof_request < {:?}", res);
        res
    }

    pub(crate) async fn close_credentials_search_for_proof_req(
        &self,
        search_handle: SearchHandle,
    ) -> IndyResult<()> {
        trace!(
            "close_credentials_search_for_proof_req > search_handle {:?}",
            search_handle
        );

        self.searches_for_proof_requests
            .lock()
            .await
            .remove(&search_handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidWalletHandle,
                    "Unknown CredentialsSearch handle",
                )
            })?;

        let res = Ok(());
        trace!("close_credentials_search_for_proof_req < {:?}", res);
        res
    }

    pub(crate) async fn delete_credential(
        &self,
        wallet_handle: WalletHandle,
        cred_id: String,
    ) -> IndyResult<()> {
        trace!(
            "delete_credential > wallet_handle {:?} cred_id {:?}",
            wallet_handle,
            cred_id
        );

        if !self
            .wallet_service
            .record_exists::<Credential>(wallet_handle, &cred_id)
            .await?
        {
            return Err(err_msg(
                IndyErrorKind::WalletItemNotFound,
                "Credential not found",
            ));
        }

        self.wallet_service
            .delete_indy_record::<Credential>(wallet_handle, &cred_id)
            .await?;

        let res = Ok(());
        trace!("delete_credential < {:?}", res);
        res
    }

    pub(crate) async fn create_proof(
        &self,
        wallet_handle: WalletHandle,
        proof_req: ProofRequest,
        requested_credentials: RequestedCredentials,
        master_secret_id: String,
        schemas: Schemas,
        cred_defs: CredentialDefinitions,
        rev_states: RevocationStates,
    ) -> IndyResult<String> {
        debug!(
            "create_proof > wallet_handle {:?} \
                proof_req {:?} requested_credentials {:?} \
                master_secret_id {:?} schemas {:?} \
                cred_defs {:?} rev_states {:?}",
            wallet_handle,
            proof_req,
            requested_credentials,
            master_secret_id,
            schemas,
            cred_defs,
            rev_states
        );

        let schemas = schemas_map_to_schemas_v1_map(schemas);
        let cred_defs = cred_defs_map_to_cred_defs_v1_map(cred_defs);

        let master_secret = self
            ._wallet_get_master_secret(wallet_handle, &master_secret_id)
            .await?;

        let cred_refs_for_attrs = requested_credentials
            .requested_attributes
            .values()
            .map(|requested_attr| requested_attr.cred_id.clone())
            .collect::<HashSet<String>>();

        let cred_refs_for_predicates = requested_credentials
            .requested_predicates
            .values()
            .map(|requested_predicate| requested_predicate.cred_id.clone())
            .collect::<HashSet<String>>();

        let cred_referents = cred_refs_for_attrs
            .union(&cred_refs_for_predicates)
            .cloned()
            .collect::<Vec<String>>();

        let mut credentials: HashMap<String, Credential> =
            HashMap::with_capacity(cred_referents.len());

        for cred_referent in cred_referents.into_iter() {
            let credential: Credential = self
                .wallet_service
                .get_indy_object(wallet_handle, &cred_referent, &RecordOptions::id_value())
                .await?;
            credentials.insert(cred_referent, credential);
        }

        let proof = self.anoncreds_service.prover.create_proof(
            &credentials,
            &proof_req,
            &requested_credentials,
            &master_secret.value,
            &schemas,
            &cred_defs,
            &rev_states,
        )?;

        let proof_json = serde_json::to_string(&proof)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize FullProof")?;

        let res = Ok(proof_json);
        debug!("create_proof <{:?}", res);
        res
    }

    pub(crate) async fn create_revocation_state(
        &self,
        blob_storage_reader_handle: i32,
        revoc_reg_def: RevocationRegistryDefinition,
        rev_reg_delta: RevocationRegistryDelta,
        timestamp: u64,
        cred_rev_id: String,
    ) -> IndyResult<String> {
        debug!(
            "create_revocation_state > blob_storage_reader_handle {:?} \
                revoc_reg_def {:?} rev_reg_delta {:?} timestamp {:?} \
                cred_rev_id {:?}",
            blob_storage_reader_handle, revoc_reg_def, rev_reg_delta, timestamp, cred_rev_id
        );

        let revoc_reg_def = RevocationRegistryDefinitionV1::from(revoc_reg_def);
        let rev_idx = parse_cred_rev_id(&cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(
            self.blob_storage_service.clone(),
            blob_storage_reader_handle,
            &revoc_reg_def,
        )
        .await?;

        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);

        let witness = Witness::new(
            rev_idx,
            revoc_reg_def.value.max_cred_num,
            revoc_reg_def.value.issuance_type.to_bool(),
            &rev_reg_delta.value,
            &sdk_tails_accessor,
        )?;

        let revocation_state = RevocationState {
            witness,
            rev_reg: RevocationRegistry::from(rev_reg_delta.value),
            timestamp,
        };

        let revocation_state_json = serde_json::to_string(&revocation_state).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize RevocationState",
        )?;

        let res = Ok(revocation_state_json);
        debug!("create_revocation_state < {:?}", res);
        res
    }

    pub(crate) async fn update_revocation_state(
        &self,
        blob_storage_reader_handle: i32,
        mut rev_state: RevocationState,
        rev_reg_def: RevocationRegistryDefinition,
        rev_reg_delta: RevocationRegistryDelta,
        timestamp: u64,
        cred_rev_id: String,
    ) -> IndyResult<String> {
        debug!(
            "update_revocation_state > blob_storage_reader_handle {:?} \
                rev_state {:?} rev_reg_def {:?} rev_reg_delta {:?} \
                timestamp {:?} cred_rev_id {:?}",
            blob_storage_reader_handle,
            rev_state,
            rev_reg_def,
            rev_reg_delta,
            timestamp,
            cred_rev_id
        );

        let revocation_registry_definition = RevocationRegistryDefinitionV1::from(rev_reg_def);
        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);
        let rev_idx = parse_cred_rev_id(&cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(
            self.blob_storage_service.clone(),
            blob_storage_reader_handle,
            &revocation_registry_definition,
        )
        .await?;

        rev_state.witness.update(
            rev_idx,
            revocation_registry_definition.value.max_cred_num,
            &rev_reg_delta.value,
            &sdk_tails_accessor,
        )?;

        rev_state.rev_reg = RevocationRegistry::from(rev_reg_delta.value);
        rev_state.timestamp = timestamp;

        let rev_state_json = serde_json::to_string(&rev_state).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize RevocationState",
        )?;

        let res = Ok(rev_state_json);
        debug!("update_revocation_state < {:?}", res);
        res
    }

    fn _get_credential_info(&self, referent: &str, credential: Credential) -> CredentialInfo {
        let credential_values: HashMap<String, String> = credential
            .values
            .0
            .into_iter()
            .map(|(attr, values)| (attr, values.raw))
            .collect();

        CredentialInfo {
            referent: referent.to_string(),
            attrs: credential_values,
            schema_id: credential.schema_id,
            cred_def_id: credential.cred_def_id,
            rev_reg_id: credential.rev_reg_id,
            cred_rev_id: credential
                .signature
                .extract_index()
                .map(|idx| idx.to_string()),
        }
    }

    fn _get_credential(&self, record: &WalletRecord) -> IndyResult<(String, Credential)> {
        let referent = record.get_id();

        let value = record.get_value().ok_or_else(|| {
            err_msg(
                IndyErrorKind::InvalidState,
                "Credential not found for id: {}",
            )
        })?;

        let credential: Credential = serde_json::from_str(value)
            .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize Credential")?;

        Ok((referent.to_string(), credential))
    }

    async fn _query_requested_credentials(
        &self,
        wallet_handle: WalletHandle,
        query_json: &Query,
        predicate_info: Option<&PredicateInfo>,
        interval: &Option<NonRevocedInterval>,
    ) -> IndyResult<Vec<RequestedCredential>> {
        debug!(
            "_query_requested_credentials > wallet_handle {:?} \
                query_json {:?} predicate_info {:?}",
            wallet_handle, query_json, predicate_info
        );

        let mut credentials_search = self
            .wallet_service
            .search_indy_records::<Credential>(
                wallet_handle,
                &query_json.to_string(),
                &SearchOptions::id_value(),
            )
            .await?;

        let credentials = self
            ._get_requested_credentials(&mut credentials_search, predicate_info, interval, None)
            .await?;

        let res = Ok(credentials);
        debug!("_query_requested_credentials < {:?}", res);
        res
    }

    async fn _get_requested_credentials(
        &self,
        credentials_search: &mut WalletSearch,
        predicate_info: Option<&PredicateInfo>,
        interval: &Option<NonRevocedInterval>,
        max_count: Option<usize>,
    ) -> IndyResult<Vec<RequestedCredential>> {
        let mut credentials: Vec<RequestedCredential> = Vec::new();

        if let Some(0) = max_count {
            return Ok(vec![]);
        }

        while let Some(credential_record) = credentials_search.fetch_next_record().await? {
            let (referent, credential) = self._get_credential(&credential_record)?;

            if let Some(predicate) = predicate_info {
                let values = self
                    .anoncreds_service
                    .prover
                    .get_credential_values_for_attribute(&credential.values.0, &predicate.name)
                    .ok_or_else(|| {
                        err_msg(IndyErrorKind::InvalidState, "Credential values not found")
                    })?;

                let satisfy = self
                    .anoncreds_service
                    .prover
                    .attribute_satisfy_predicate(predicate, &values.encoded)?;
                if !satisfy {
                    continue;
                }
            }

            credentials.push(RequestedCredential {
                cred_info: self._get_credential_info(&referent, credential),
                interval: interval.clone(),
            });

            if let Some(mut count) = max_count {
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }

        Ok(credentials)
    }

    async fn _wallet_get_master_secret(
        &self,
        wallet_handle: WalletHandle,
        key: &str,
    ) -> IndyResult<MasterSecret> {
        self.wallet_service
            .get_indy_object(wallet_handle, &key, &RecordOptions::id_value())
            .await
    }

    async fn _get_credential_attr_tag_policy(
        &self,
        wallet_handle: WalletHandle,
        cred_def_id: &CredentialDefinitionId,
    ) -> IndyResult<String> {
        let catpol = self
            .wallet_service
            .get_indy_opt_object::<CredentialAttrTagPolicy>(
                wallet_handle,
                &cred_def_id.0,
                &RecordOptions::id_value(),
            )
            .await?
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .to_indy(
                IndyErrorKind::InvalidState,
                "Cannot serialize CredentialAttrTagPolicy",
            )?
            .unwrap_or_else(|| Value::Null.to_string());

        Ok(catpol)
    }
}
