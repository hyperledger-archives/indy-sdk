use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use indy_crypto::cl::{new_nonce, RevocationRegistry, Witness};

use domain::anoncreds::credential::{Credential, CredentialInfo};
use domain::anoncreds::credential_definition::{cred_defs_map_to_cred_defs_v1_map, CredentialDefinition, CredentialDefinitionV1};
use domain::anoncreds::credential_for_proof_request::{CredentialsForProofRequest, RequestedCredential};
use domain::anoncreds::credential_offer::CredentialOffer;
use domain::anoncreds::credential_request::{CredentialRequest, CredentialRequestMetadata};
use domain::anoncreds::master_secret::MasterSecret;
use domain::anoncreds::proof_request::{NonRevocedInterval, PredicateInfo, ProofRequest, ProofRequestExtraQuery};
use domain::anoncreds::requested_credential::RequestedCredentials;
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::revocation_state::RevocationState;
use domain::anoncreds::schema::{Schema, schemas_map_to_schemas_v1_map, SchemaV1};
use errors::prelude::*;
use services::anoncreds::AnoncredsService;
use services::anoncreds::helpers::parse_cred_rev_id;
use services::blob_storage::BlobStorageService;
use services::crypto::CryptoService;
use services::wallet::{RecordOptions, SearchOptions, WalletRecord, WalletSearch, WalletService};
use utils::sequence;

use super::tails::SDKTailsAccessor;
use api::WalletHandle;

pub enum ProverCommand {
    CreateMasterSecret(
        WalletHandle,
        Option<String>, // master secret id
        Box<Fn(IndyResult<String>) + Send>),
    CreateCredentialRequest(
        WalletHandle,
        String, // prover did
        CredentialOffer, // credential offer
        CredentialDefinition, // credential def
        String, // master secret name
        Box<Fn(IndyResult<(String, String)>) + Send>),
    StoreCredential(
        WalletHandle,
        Option<String>, // credential id
        CredentialRequestMetadata, // credential request metadata
        Credential, // credentials
        CredentialDefinition, // credential definition
        Option<RevocationRegistryDefinition>, // revocation registry definition
        Box<Fn(IndyResult<String>) + Send>),
    GetCredentials(
        WalletHandle,
        Option<String>, // filter json
        Box<Fn(IndyResult<String>) + Send>),
    GetCredential(
        WalletHandle,
        String, // credential id
        Box<Fn(IndyResult<String>) + Send>),
    SearchCredentials(
        WalletHandle,
        Option<String>, // query json
        Box<Fn(IndyResult<(i32, usize)>) + Send>),
    FetchCredentials(
        i32, // search handle
        usize, // count
        Box<Fn(IndyResult<String>) + Send>),
    CloseCredentialsSearch(
        i32, // search handle
        Box<Fn(IndyResult<()>) + Send>),
    GetCredentialsForProofReq(
        WalletHandle,
        ProofRequest, // proof request
        Box<Fn(IndyResult<String>) + Send>),
    SearchCredentialsForProofReq(
        WalletHandle,
        ProofRequest, // proof request
        Option<ProofRequestExtraQuery>, // extra query
        Box<Fn(IndyResult<i32>) + Send>),
    FetchCredentialForProofReq(
        i32, // search handle
        String, // item referent
        usize, // count
        Box<Fn(IndyResult<String>) + Send>),
    CloseCredentialsSearchForProofReq(
        i32, // search handle
        Box<Fn(IndyResult<()>) + Send>),
    CreateProof(
        WalletHandle,
        ProofRequest, // proof request
        RequestedCredentials, // requested credentials
        String, // master secret name
        HashMap<String, Schema>, // schemas
        HashMap<String, CredentialDefinition>, // credential defs
        HashMap<String, HashMap<u64, RevocationState>>, // revocation states
        Box<Fn(IndyResult<String>) + Send>),
    CreateRevocationState(
        i32, // blob storage reader handle
        RevocationRegistryDefinition, // revocation registry definition
        RevocationRegistryDelta, // revocation registry delta
        u64, //timestamp
        String, //credential revocation id
        Box<Fn(IndyResult<String>) + Send>),
    UpdateRevocationState(
        i32, // tails reader _handle
        RevocationState, // revocation state
        RevocationRegistryDefinition, // revocation registry definition
        RevocationRegistryDelta, // revocation registry delta
        u64, //timestamp
        String, //credential revocation id
        Box<Fn(IndyResult<String>) + Send>)
}

struct SearchForProofRequest {
    search: WalletSearch,
    interval: Option<NonRevocedInterval>,
    predicate_info: Option<PredicateInfo>,
}

impl SearchForProofRequest {
    fn new(search: WalletSearch,
           interval: Option<NonRevocedInterval>,
           predicate_info: Option<PredicateInfo>, ) -> Self {
        Self {
            search,
            interval,
            predicate_info,
        }
    }
}

pub struct ProverCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    blob_storage_service: Rc<BlobStorageService>,
    searches: RefCell<HashMap<i32, Box<WalletSearch>>>,
    searches_for_proof_requests: RefCell<HashMap<i32, Box<HashMap<String, SearchForProofRequest>>>>,
}

impl ProverCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>,
               blob_storage_service: Rc<BlobStorageService>) -> ProverCommandExecutor {
        ProverCommandExecutor {
            anoncreds_service,
            wallet_service,
            crypto_service,
            blob_storage_service,
            searches: RefCell::new(HashMap::new()),
            searches_for_proof_requests: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: ProverCommand) {
        match command {
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_id, cb) => {
                info!(target: "prover_command_executor", "CreateMasterSecret command received");
                cb(self.create_master_secret(wallet_handle, master_secret_id.as_ref().map(String::as_str)));
            }
            ProverCommand::CreateCredentialRequest(wallet_handle, prover_did, credential_offer,
                                                   credential_def, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateCredentialRequest command received");
                cb(self.create_credential_request(wallet_handle, &prover_did, &credential_offer,
                                                  &CredentialDefinitionV1::from(credential_def), &master_secret_name));
            }
            ProverCommand::StoreCredential(wallet_handle, cred_id, cred_req_metadata, mut cred, cred_def, rev_reg_def, cb) => {
                info!(target: "prover_command_executor", "StoreCredential command received");
                cb(self.store_credential(wallet_handle, cred_id.as_ref().map(String::as_str),
                                         &cred_req_metadata, &mut cred,
                                         &CredentialDefinitionV1::from(cred_def),
                                         rev_reg_def.map(RevocationRegistryDefinitionV1::from).as_ref()));
            }
            ProverCommand::GetCredentials(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetCredentials command received");
                cb(self.get_credentials(wallet_handle, filter_json.as_ref().map(String::as_str)));
            }
            ProverCommand::GetCredential(wallet_handle, cred_id, cb) => {
                info!(target: "prover_command_executor", "GetCredential command received");
                cb(self.get_credential(wallet_handle, &cred_id));
            }
            ProverCommand::SearchCredentials(wallet_handle, query_json, cb) => {
                info!(target: "prover_command_executor", "SearchCredentials command received");
                cb(self.search_credentials(wallet_handle, query_json.as_ref().map(String::as_str)));
            }
            ProverCommand::FetchCredentials(search_handle, count, cb) => {
                info!(target: "prover_command_executor", "FetchCredentials command received");
                cb(self.fetch_credentials(search_handle, count));
            }
            ProverCommand::CloseCredentialsSearch(search_handle, cb) => {
                info!(target: "prover_command_executor", "CloseCredentialsSearch command received");
                cb(self.close_credentials_search(search_handle));
            }
            ProverCommand::GetCredentialsForProofReq(wallet_handle, proof_req, cb) => {
                info!(target: "prover_command_executor", "GetCredentialsForProofReq command received");
                cb(self.get_credentials_for_proof_req(wallet_handle, &proof_req));
            }
            ProverCommand::SearchCredentialsForProofReq(wallet_handle, proof_req, extra_query, cb) => {
                info!(target: "prover_command_executor", "SearchCredentialsForProofReq command received");
                cb(self.search_credentials_for_proof_req(wallet_handle, &proof_req, extra_query.as_ref()));
            }
            ProverCommand::FetchCredentialForProofReq(search_handle, item_ref, count, cb) => {
                info!(target: "prover_command_executor", "FetchCredentialForProofReq command received");
                cb(self.fetch_credential_for_proof_request(search_handle, &item_ref, count));
            }
            ProverCommand::CloseCredentialsSearchForProofReq(search_handle, cb) => {
                info!(target: "prover_command_executor", "CloseCredentialsSearchForProofReq command received");
                cb(self.close_credentials_search_for_proof_req(search_handle));
            }
            ProverCommand::CreateProof(wallet_handle, proof_req, requested_credentials, master_secret_name,
                                       schemas, cred_defs, rev_states, cb) => {
                info!(target: "prover_command_executor", "CreateProof command received");
                cb(self.create_proof(wallet_handle, &proof_req,& requested_credentials, &master_secret_name,
                                     &schemas_map_to_schemas_v1_map(schemas),
                                     &cred_defs_map_to_cred_defs_v1_map(cred_defs),
                                     &rev_states));
            }
            ProverCommand::CreateRevocationState(blob_storage_reader_handle, rev_reg_def, rev_reg_delta, timestamp, cred_rev_id, cb) => {
                info!(target: "prover_command_executor", "CreateRevocationState command received");
                cb(self.create_revocation_state(blob_storage_reader_handle, rev_reg_def, rev_reg_delta, timestamp, &cred_rev_id));
            }
            ProverCommand::UpdateRevocationState(blob_storage_reader_handle, rev_state, rev_reg_def, rev_reg_delta, timestamp, cred_rev_id, cb) => {
                info!(target: "prover_command_executor", "UpdateRevocationState command received");
                cb(self.update_revocation_state(blob_storage_reader_handle, rev_state, rev_reg_def, rev_reg_delta, timestamp, &cred_rev_id));
            }
        };
    }

    fn create_master_secret(&self,
                            wallet_handle: WalletHandle,
                            master_secret_id: Option<&str>) -> IndyResult<String> {
        debug!("create_master_secret >>> wallet_handle: {:?}, master_secret_id: {:?}", wallet_handle, master_secret_id);

        let master_secret_id = master_secret_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        if self.wallet_service.record_exists::<MasterSecret>(wallet_handle, &master_secret_id)? {
            return Err(err_msg(IndyErrorKind::MasterSecretDuplicateName, format!("MasterSecret already exists {}", master_secret_id)));
        }

        let master_secret = self.anoncreds_service.prover.new_master_secret()?;

        let master_secret = MasterSecret {
            value: master_secret
        };

        self.wallet_service.add_indy_object(wallet_handle, &master_secret_id, &master_secret, &HashMap::new())?;

        debug!("create_master_secret <<< master_secret_id: {:?}", master_secret_id);

        Ok(master_secret_id)
    }

    fn create_credential_request(&self,
                                 wallet_handle: WalletHandle,
                                 prover_did: &str,
                                 cred_offer: &CredentialOffer,
                                 cred_def: &CredentialDefinitionV1,
                                 master_secret_id: &str) -> IndyResult<(String, String)> {
        debug!("create_credential_request >>> wallet_handle: {:?}, prover_did: {:?}, cred_offer: {:?}, cred_def: {:?}, master_secret_id: {:?}",
               wallet_handle, prover_did, cred_offer, cred_def, master_secret_id);

        self.crypto_service.validate_did(&prover_did)?;

        let master_secret: MasterSecret = self._wallet_get_master_secret(wallet_handle, &master_secret_id)?;

        let (blinded_ms, ms_blinding_data, blinded_ms_correctness_proof) =
            self.anoncreds_service.prover.new_credential_request(cred_def,
                                                                 &master_secret.value,
                                                                 &cred_offer)?;

        let nonce = new_nonce()?;

        let credential_request = CredentialRequest {
            prover_did: prover_did.to_string(),
            cred_def_id: cred_offer.cred_def_id.clone(),
            blinded_ms,
            blinded_ms_correctness_proof,
            nonce
        };

        let credential_request_metadata = CredentialRequestMetadata {
            master_secret_blinding_data: ms_blinding_data,
            nonce: credential_request.nonce.clone()?,
            master_secret_name: master_secret_id.to_string()
        };

        let cred_req_json = serde_json::to_string(&credential_request)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize CredentialRequest")?;

        let cred_req_metadata_json = serde_json::to_string(&credential_request_metadata)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize CredentialRequestMetadata")?;

        debug!("create_credential_request <<< cred_req_json: {:?}, cred_req_metadata_json: {:?}", cred_req_json, cred_req_metadata_json);

        Ok((cred_req_json, cred_req_metadata_json))
    }

    fn store_credential(&self,
                        wallet_handle: WalletHandle,
                        cred_id: Option<&str>,
                        cred_req_metadata: &CredentialRequestMetadata,
                        credential: &mut Credential,
                        cred_def: &CredentialDefinitionV1,
                        rev_reg_def: Option<&RevocationRegistryDefinitionV1>) -> IndyResult<String> {
        debug!("store_credential >>> wallet_handle: {:?}, cred_id: {:?}, cred_req_metadata: {:?}, credential: {:?}, cred_def: {:?}, \
        rev_reg_def: {:?}", wallet_handle, cred_id, cred_req_metadata, credential, cred_def, rev_reg_def);

        let master_secret: MasterSecret = self._wallet_get_master_secret(wallet_handle, &cred_req_metadata.master_secret_name)?;

        self.anoncreds_service.prover.process_credential(credential,
                                                         &cred_req_metadata,
                                                         &master_secret.value,
                                                         cred_def,
                                                         rev_reg_def)?;

        credential.rev_reg = None;
        credential.witness = None;

        let out_cred_id = cred_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        let cred_tags = self.anoncreds_service.prover.build_credential_tags(&credential);
        self.wallet_service.add_indy_object(wallet_handle, &out_cred_id, credential, &cred_tags)?;

        debug!("store_credential <<< out_cred_id: {:?}", out_cred_id);

        Ok(out_cred_id)
    }

    fn get_credentials(&self,
                       wallet_handle: WalletHandle,
                       filter_json: Option<&str>) -> IndyResult<String> {
        debug!("get_credentials >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

        let filter_json = filter_json.unwrap_or("{}");
        let mut credentials_info: Vec<CredentialInfo> = Vec::new();

        let mut credentials_search =
            self.wallet_service.search_indy_records::<Credential>(wallet_handle, filter_json, &SearchOptions::id_value())?;

        while let Some(credential_record) = credentials_search.fetch_next_record()? {
            let (referent, credential) = self._get_credential(&credential_record)?;
            credentials_info.push(self._get_credential_info(&referent, credential))
        }

        let credentials_info_json = serde_json::to_string(&credentials_info)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize list of CredentialInfo")?;

        debug!("get_credentials <<< credentials_info_json: {:?}", credentials_info_json);

        Ok(credentials_info_json)
    }

    fn get_credential(&self,
                      wallet_handle: WalletHandle,
                      cred_id: &str) -> IndyResult<String> {
        debug!("get_credentials >>> wallet_handle: {:?}, cred_id: {:?}", wallet_handle, cred_id);

        let credential: Credential = self.wallet_service.get_indy_object(wallet_handle, &cred_id, &RecordOptions::id_value())?;

        let credential_info = self._get_credential_info(&cred_id, credential);

        let credential_info_json = serde_json::to_string(&credential_info)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize CredentialInfo")?;

        debug!("get_credential <<< credential_info_json: {:?}", credential_info_json);

        Ok(credential_info_json)
    }

    fn search_credentials(&self,
                          wallet_handle: WalletHandle,
                          query_json: Option<&str>) -> IndyResult<(i32, usize)> {
        debug!("search_credentials >>> wallet_handle: {:?}, query_json: {:?}", wallet_handle, query_json);

        let credentials_search =
            self.wallet_service.search_indy_records::<Credential>(wallet_handle, query_json.unwrap_or("{}"), &SearchOptions::id_value())?;

        let total_count = credentials_search.get_total_count()?.unwrap_or(0);

        let handle = sequence::get_next_id();

        self.searches.borrow_mut().insert(handle, Box::new(credentials_search));

        let res = (handle, total_count);

        trace!("search_credentials <<< res: {:?}", res);

        Ok(res)
    }

    fn fetch_credentials(&self,
                         search_handle: i32,
                         count: usize, ) -> IndyResult<String> {
        trace!("fetch_credentials >>> search_handle: {:?}, count: {:?}", search_handle, count);

        let mut searches = self.searches.borrow_mut();
        let search = searches.get_mut(&search_handle)
            .ok_or(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown CredentialsSearch handle: {}", search_handle)))?;

        let mut credentials_info: Vec<CredentialInfo> = Vec::new();

        for _ in 0..count {
            match search.fetch_next_record()? {
                Some(credential_record) => {
                    let (referent, credential) = self._get_credential(&credential_record)?;
                    credentials_info.push(self._get_credential_info(&referent, credential))
                }
                None => break
            }
        }

        let credentials_info_json = serde_json::to_string(&credentials_info)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize list of CredentialInfo")?;

        trace!("fetch_credentials <<< credentials_info_json: {:?}", credentials_info_json);

        Ok(credentials_info_json)
    }

    fn close_credentials_search(&self, search_handle: i32) -> IndyResult<()> {
        trace!("close_credentials_search >>> search_handle: {:?}", search_handle);

        let res = match self.searches.borrow_mut().remove(&search_handle) {
            Some(_) => Ok(()),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown CredentialsSearch handle: {}", search_handle)))
        }?;

        trace!("close_credentials_search <<< res: {:?}", res);

        Ok(res)
    }

    fn get_credentials_for_proof_req(&self,
                                     wallet_handle: WalletHandle,
                                     proof_request: &ProofRequest) -> IndyResult<String> {
        debug!("get_credentials_for_proof_req >>> wallet_handle: {:?}, proof_request: {:?}", wallet_handle, proof_request);

        let mut credentials_for_proof_request = CredentialsForProofRequest::default();

        for (attr_id, requested_attr) in &proof_request.requested_attributes {
            let query_json = self.anoncreds_service.prover.build_query(&requested_attr.name,
                                                                       &attr_id,
                                                                       &requested_attr.restrictions,
                                                                       &None)?;

            let interval = self.anoncreds_service.prover.get_non_revoc_interval(&proof_request.non_revoked, &requested_attr.non_revoked);

            let credentials_for_attribute = self._query_requested_credentials(wallet_handle, &query_json, None, &interval)?;

            credentials_for_proof_request.attrs.insert(attr_id.to_string(), credentials_for_attribute);
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let query_json = self.anoncreds_service.prover.build_query(&requested_predicate.name,
                                                                       &predicate_id,
                                                                       &requested_predicate.restrictions,
                                                                       &None)?;

            let interval = self.anoncreds_service.prover.get_non_revoc_interval(&proof_request.non_revoked, &requested_predicate.non_revoked);

            let credentials_for_predicate =
                self._query_requested_credentials(wallet_handle, &query_json, Some(&requested_predicate), &interval)?;

            credentials_for_proof_request.predicates.insert(predicate_id.to_string(), credentials_for_predicate);
        }

        let credentials_for_proof_request_json = serde_json::to_string(&credentials_for_proof_request)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize CredentialsForProofRequest")?;

        debug!("get_credentials_for_proof_req <<< credentials_for_proof_request_json: {:?}", credentials_for_proof_request_json);

        Ok(credentials_for_proof_request_json)
    }

    fn search_credentials_for_proof_req(&self,
                                        wallet_handle: WalletHandle,
                                        proof_request: &ProofRequest,
                                        extra_query: Option<&ProofRequestExtraQuery>) -> IndyResult<i32> {
        debug!("search_credentials_for_proof_req >>> wallet_handle: {:?}, proof_request: {:?}, extra_query: {:?}", wallet_handle, proof_request, extra_query);

        let mut credentials_for_proof_request_search = HashMap::<String, SearchForProofRequest>::new();

        for (attr_id, requested_attr) in &proof_request.requested_attributes {
            let query_json = self.anoncreds_service.prover.build_query(&requested_attr.name,
                                                                       &attr_id,
                                                                       &requested_attr.restrictions,
                                                                       &extra_query)?;
            let mut credentials_search =
                self.wallet_service.search_indy_records::<Credential>(wallet_handle, &query_json, &SearchOptions::id_value())?;

            let interval = self.anoncreds_service.prover.get_non_revoc_interval(&proof_request.non_revoked, &requested_attr.non_revoked);

            credentials_for_proof_request_search.insert(attr_id.to_string(),
                                                        SearchForProofRequest::new(
                                                            credentials_search, interval, None));
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let query_json = self.anoncreds_service.prover.build_query(&requested_predicate.name,
                                                                       &predicate_id,
                                                                       &requested_predicate.restrictions,
                                                                       &extra_query)?;
            let mut credentials_search =
                self.wallet_service.search_indy_records::<Credential>(wallet_handle, &query_json, &SearchOptions::id_value())?;

            let interval = self.anoncreds_service.prover.get_non_revoc_interval(&proof_request.non_revoked, &requested_predicate.non_revoked);

            credentials_for_proof_request_search.insert(predicate_id.to_string(),
                                                        SearchForProofRequest::new(
                                                            credentials_search, interval, Some(requested_predicate.clone())));
        }

        let search_handle = sequence::get_next_id();
        self.searches_for_proof_requests.borrow_mut().insert(search_handle, Box::new(credentials_for_proof_request_search));

        debug!("search_credentials_for_proof_req <<< credentials_for_proof_request_json: {:?}", search_handle);

        Ok(search_handle)
    }

    fn fetch_credential_for_proof_request(&self, search_handle: i32, item_referent: &str, count: usize) -> IndyResult<String> {
        trace!("fetch_credential_for_proof_request >>> search_handle: {:?}, item_referent: {:?}, count: {:?}", search_handle, item_referent, count);

        let mut searches = self.searches_for_proof_requests.borrow_mut();
        let search: &mut SearchForProofRequest = searches.get_mut(&search_handle)
            .ok_or(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown CredentialsSearch handle: {}", search_handle)))?
            .get_mut(item_referent)
            .ok_or(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown item referent {} for CredentialsSearch handle: {}", item_referent, search_handle)))?;

        let requested_credentials: Vec<RequestedCredential> =
            self._get_requested_credentials(&mut search.search, search.predicate_info.as_ref(), &search.interval, Some(count))?;

        let requested_credentials_json = serde_json::to_string(&requested_credentials)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize list of RequestedCredential")?;

        trace!("fetch_credential_for_proof_request <<< requested_credentials_json: {:?}", requested_credentials_json);

        Ok(requested_credentials_json)
    }

    fn close_credentials_search_for_proof_req(&self, search_handle: i32) -> IndyResult<()> {
        trace!("close_credentials_search_for_proof_req >>> search_handle: {:?}", search_handle);

        let res = match self.searches_for_proof_requests.borrow_mut().remove(&search_handle) {
            Some(_) => Ok(()),
            None => Err(err_msg(IndyErrorKind::InvalidWalletHandle, format!("Unknown CredentialsSearch handle: {}", search_handle)))
        }?;

        trace!("close_credentials_search_for_proof_req <<< res: {:?}", res);

        Ok(res)
    }

    fn create_proof(&self,
                    wallet_handle: WalletHandle,
                    proof_req: &ProofRequest,
                    requested_credentials: &RequestedCredentials,
                    master_secret_id: &str,
                    schemas: &HashMap<String, SchemaV1>,
                    cred_defs: &HashMap<String, CredentialDefinitionV1>,
                    rev_states: &HashMap<String, HashMap<u64, RevocationState>>) -> IndyResult<String> {
        debug!("create_proof >>> wallet_handle: {:?}, proof_req: {:?}, requested_credentials: {:?}, master_secret_id: {:?}, schemas: {:?}, \
        cred_defs: {:?}, rev_states: {:?}",
               wallet_handle, proof_req, requested_credentials, master_secret_id, schemas, cred_defs, rev_states);

        let master_secret: MasterSecret = self._wallet_get_master_secret(wallet_handle, &master_secret_id)?;

        let cred_refs_for_attrs =
            requested_credentials.requested_attributes
                .values()
                .map(|requested_attr| requested_attr.cred_id.clone())
                .collect::<HashSet<String>>();

        let cred_refs_for_predicates =
            requested_credentials.requested_predicates
                .values()
                .map(|requested_predicate| requested_predicate.cred_id.clone())
                .collect::<HashSet<String>>();

        let cred_referents = cred_refs_for_attrs.union(&cred_refs_for_predicates).cloned().collect::<Vec<String>>();

        let mut credentials: HashMap<String, Credential> = HashMap::new();

        for cred_referent in cred_referents.into_iter() {
            let credential: Credential = self.wallet_service.get_indy_object(wallet_handle, &cred_referent, &RecordOptions::id_value())?;
            credentials.insert(cred_referent, credential);
        }

        let proof = self.anoncreds_service.prover.create_proof(&credentials,
                                                               &proof_req,
                                                               &requested_credentials,
                                                               &master_secret.value,
                                                               schemas,
                                                               cred_defs,
                                                               &rev_states)?;

        let proof_json = serde_json::to_string(&proof)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize FullProof")?;

        debug!("create_proof <<< proof_json: {:?}", proof_json);

        Ok(proof_json)
    }

    fn create_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               revoc_reg_def: RevocationRegistryDefinition,
                               rev_reg_delta: RevocationRegistryDelta,
                               timestamp: u64,
                               cred_rev_id: &str) -> IndyResult<String> {
        debug!("create_revocation_state >>> , blob_storage_reader_handle: {:?}, revoc_reg_def: {:?}, rev_reg_delta: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
               blob_storage_reader_handle, revoc_reg_def, rev_reg_delta, timestamp, cred_rev_id);

        let revoc_reg_def = RevocationRegistryDefinitionV1::from(revoc_reg_def);

        let rev_idx = parse_cred_rev_id(cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revoc_reg_def)?;

        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);

        let witness = Witness::new(rev_idx, revoc_reg_def.value.max_cred_num, revoc_reg_def.value.issuance_type.to_bool(), &rev_reg_delta.value, &sdk_tails_accessor)?;

        let revocation_state = RevocationState {
            witness,
            rev_reg: RevocationRegistry::from(rev_reg_delta.value),
            timestamp,
        };

        let revocation_state_json = serde_json::to_string(&revocation_state)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationState")?;

        debug!("create_revocation_state <<< revocation_state_json: {:?}", revocation_state_json);

        Ok(revocation_state_json)
    }

    fn update_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               mut rev_state: RevocationState,
                               rev_reg_def: RevocationRegistryDefinition,
                               rev_reg_delta: RevocationRegistryDelta,
                               timestamp: u64,
                               cred_rev_id: &str) -> IndyResult<String> {
        debug!("update_revocation_state >>> blob_storage_reader_handle: {:?}, rev_state: {:?}, rev_reg_def: {:?}, rev_reg_delta: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
               blob_storage_reader_handle, rev_state, rev_reg_def, rev_reg_delta, timestamp, cred_rev_id);

        let revocation_registry_definition = RevocationRegistryDefinitionV1::from(rev_reg_def);

        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);

        let rev_idx = parse_cred_rev_id(cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revocation_registry_definition)?;

        rev_state.witness.update(rev_idx, revocation_registry_definition.value.max_cred_num, &rev_reg_delta.value, &sdk_tails_accessor)?;

        rev_state.rev_reg = RevocationRegistry::from(rev_reg_delta.value);
        rev_state.timestamp = timestamp;

        let rev_state_json = serde_json::to_string(&rev_state)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationState")?;

        debug!("update_revocation_state <<< rev_state: {:?}", rev_state_json);

        Ok(rev_state_json)
    }

    fn _get_credential_info(&self,
                            referent: &str,
                            credential: Credential) -> CredentialInfo {
        let credential_values: HashMap<String, String> =
            credential.values
                .into_iter()
                .map(|(attr, values)| (attr, values.raw))
                .collect();

        CredentialInfo {
            referent: referent.to_string(),
            attrs: credential_values,
            schema_id: credential.schema_id,
            cred_def_id: credential.cred_def_id,
            rev_reg_id: credential.rev_reg_id.as_ref().map(|s| s.to_string()),
            cred_rev_id: credential.signature.extract_index().map(|idx| idx.to_string())
        }
    }

    fn _get_credential(&self,
                       record: &WalletRecord) -> IndyResult<(String, Credential)> {
        let referent = record.get_id();

        let value = record.get_value()
            .ok_or(err_msg(IndyErrorKind::InvalidState, format!("Credential not found for id: {}", referent)))?;

        let credential: Credential = serde_json::from_str(value)
            .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize Credential")?;

        Ok((referent.to_string(), credential))
    }

    fn _query_requested_credentials(&self,
                                    wallet_handle: WalletHandle,
                                    query_json: &str,
                                    predicate_info: Option<&PredicateInfo>,
                                    interval: &Option<NonRevocedInterval>) -> IndyResult<Vec<RequestedCredential>> {
        debug!("_query_requested_credentials >>> wallet_handle: {:?}, query_json: {:?}, predicate_info: {:?}",
               wallet_handle, query_json, predicate_info);

        let mut credentials_search =
            self.wallet_service.search_indy_records::<Credential>(wallet_handle, query_json, &SearchOptions::id_value())?;

        let credentials = self._get_requested_credentials(&mut credentials_search, predicate_info, interval, None)?;

        debug!("_query_requested_credentials <<< credentials: {:?}", credentials);

        Ok(credentials)
    }

    fn _get_requested_credentials(&self,
                                  credentials_search: &mut WalletSearch,
                                  predicate_info: Option<&PredicateInfo>,
                                  interval: &Option<NonRevocedInterval>,
                                  max_count: Option<usize>) -> IndyResult<Vec<RequestedCredential>> {
        let mut credentials: Vec<RequestedCredential> = Vec::new();

        if let Some(0) = max_count {
            return Ok(vec![]);
        }

        while let Some(credential_record) = credentials_search.fetch_next_record()? {
            let (referent, credential) = self._get_credential(&credential_record)?;

            if let Some(predicate) = predicate_info {
                let values = self.anoncreds_service.prover.get_credential_values_for_attribute(&credential.values, &predicate.name)
                    .ok_or(err_msg(IndyErrorKind::InvalidState, "Credential values not found"))?;

                let satisfy = self.anoncreds_service.prover.attribute_satisfy_predicate(predicate, &values.encoded)?;
                if !satisfy { continue; }
            }

            credentials.push(
                RequestedCredential {
                    cred_info: self._get_credential_info(&referent, credential),
                    interval: interval.clone()
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


    fn _wallet_get_master_secret(&self, wallet_handle: WalletHandle, key: &str) -> IndyResult<MasterSecret> {
        self.wallet_service.get_indy_object(wallet_handle, &key, &RecordOptions::id_value())
    }
}

