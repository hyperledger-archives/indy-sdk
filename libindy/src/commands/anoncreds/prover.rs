extern crate serde_json;
extern crate indy_crypto;
extern crate uuid;

use errors::common::CommonError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::AnoncredsService;
use services::anoncreds::helpers::parse_cred_rev_id;
use services::wallet::{WalletService, WalletSearch, RecordOptions, SearchOptions, WalletRecord};
use services::crypto::CryptoService;
use std::rc::Rc;
use std::cell::RefCell;
use services::blob_storage::BlobStorageService;
use std::collections::{HashMap, HashSet};
use self::indy_crypto::cl::{Witness, RevocationRegistry, new_nonce};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use super::tails::SDKTailsAccessor;

use domain::anoncreds::schema::{Schema, schemas_map_to_schemas_v1_map};
use domain::anoncreds::credential::{Credential, CredentialInfo};
use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1, cred_defs_map_to_cred_defs_v1_map};
use domain::anoncreds::credential_offer::CredentialOffer;
use domain::anoncreds::credential_request::{CredentialRequest, CredentialRequestMetadata};
use domain::anoncreds::credential_for_proof_request::{CredentialsForProofRequest, RequestedCredential};
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::proof::Proof;
use domain::anoncreds::proof_request::{ProofRequest, ProofRequestExtraQuery, PredicateInfo, NonRevocedInterval};
use domain::anoncreds::requested_credential::RequestedCredentials;
use domain::anoncreds::revocation_state::RevocationState;
use domain::anoncreds::master_secret::MasterSecret;
use services::anoncreds::helpers::attr_common_view;
use utils::sequence::SequenceUtils;

pub enum ProverCommand {
    CreateMasterSecret(
        i32, // wallet handle
        Option<String>, // master secret id
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateCredentialRequest(
        i32, // wallet handle
        String, // prover did
        String, // credential offer json
        String, // credential def json
        String, // master secret name
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    StoreCredential(
        i32, // wallet handle
        Option<String>, // credential id
        String, // credential request metadata json
        String, // credentials json
        String, // credential definition json
        Option<String>, // revocation registry definition json
        Box<Fn(Result<String, IndyError>) + Send>),
    GetCredentials(
        i32, // wallet handle
        Option<String>, // filter json
        Box<Fn(Result<String, IndyError>) + Send>),
    GetCredential(
        i32, // wallet handle
        String, // credential id
        Box<Fn(Result<String, IndyError>) + Send>),
    SearchCredentials(
        i32, // wallet handle
        Option<String>, // query json
        Box<Fn(Result<(i32, usize), IndyError>) + Send>),
    FetchCredentials(
        i32, // search handle
        usize, // count
        Box<Fn(Result<String, IndyError>) + Send>),
    CloseCredentialsSearch(
        i32, // search handle
        Box<Fn(Result<(), IndyError>) + Send>),
    GetCredentialsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Box<Fn(Result<String, IndyError>) + Send>),
    SearchCredentialsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Option<String>, // extra query json
        Box<Fn(Result<i32, IndyError>) + Send>),
    FetchCredentialForProofReq(
        i32, // search handle
        String, // item referent
        usize, // count
        Box<Fn(Result<String, IndyError>) + Send>),
    CloseCredentialsSearchForProofReq(
        i32, // search handle
        Box<Fn(Result<(), IndyError>) + Send>),
    CreateProof(
        i32, // wallet handle
        String, // proof request json
        String, // requested credentials json
        String, // master secret name
        String, // schemas json
        String, // credential defs json
        String, // revocation states json
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateRevocationState(
        i32, // blob storage reader handle
        String, // revocation registry definition json
        String, // revocation registry delta json
        u64, //timestamp
        String, //credential revocation id
        Box<Fn(Result<String, IndyError>) + Send>),
    UpdateRevocationState(
        i32, // tails reader _handle
        String, // revocation state json
        String, // revocation registry definition json
        String, // revocation registry delta json
        u64, //timestamp
        String, //credential revocation id
        Box<Fn(Result<String, IndyError>) + Send>)
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
            ProverCommand::CreateCredentialRequest(wallet_handle, prover_did, credential_offer_json,
                                                   credential_def_json, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateCredentialRequest command received");
                cb(self.create_credential_request(wallet_handle, &prover_did, &credential_offer_json,
                                                  &credential_def_json, &master_secret_name));
            }
            ProverCommand::StoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb) => {
                info!(target: "prover_command_executor", "StoreCredential command received");
                cb(self.store_credential(wallet_handle, cred_id.as_ref().map(String::as_str),
                                         &cred_req_metadata_json, &cred_json, &cred_def_json,
                                         rev_reg_def_json.as_ref().map(String::as_str)));
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
            ProverCommand::GetCredentialsForProofReq(wallet_handle, proof_req_json, cb) => {
                info!(target: "prover_command_executor", "GetCredentialsForProofReq command received");
                cb(self.get_credentials_for_proof_req(wallet_handle, &proof_req_json));
            }
            ProverCommand::SearchCredentialsForProofReq(wallet_handle, proof_req_json, extra_query_json, cb) => {
                info!(target: "prover_command_executor", "SearchCredentialsForProofReq command received");
                cb(self.search_credentials_for_proof_req(wallet_handle, &proof_req_json, extra_query_json.as_ref().map(String::as_str)));
            }
            ProverCommand::FetchCredentialForProofReq(search_handle, item_ref, count, cb) => {
                info!(target: "prover_command_executor", "FetchCredentialForProofReq command received");
                cb(self.fetch_credential_for_proof_request(search_handle, &item_ref, count));
            }
            ProverCommand::CloseCredentialsSearchForProofReq(search_handle, cb) => {
                info!(target: "prover_command_executor", "CloseCredentialsSearchForProofReq command received");
                cb(self.close_credentials_search_for_proof_req(search_handle));
            }
            ProverCommand::CreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name,
                                       schemas_json, credential_defs_json, rev_states_json, cb) => {
                info!(target: "prover_command_executor", "CreateProof command received");
                cb(self.create_proof(wallet_handle, &proof_req_json, &requested_credentials_json, &master_secret_name,
                                     &schemas_json, &credential_defs_json, &rev_states_json));
            }
            ProverCommand::CreateRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb) => {
                info!(target: "prover_command_executor", "CreateRevocationState command received");
                cb(self.create_revocation_state(blob_storage_reader_handle, &rev_reg_def_json, &rev_reg_delta_json, timestamp, &cred_rev_id));
            }
            ProverCommand::UpdateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb) => {
                info!(target: "prover_command_executor", "UpdateRevocationState command received");
                cb(self.update_revocation_state(blob_storage_reader_handle, &rev_state_json, &rev_reg_def_json, &rev_reg_delta_json, timestamp, &cred_rev_id));
            }
        };
    }

    fn create_master_secret(&self,
                            wallet_handle: i32,
                            master_secret_id: Option<&str>) -> Result<String, IndyError> {
        debug!("create_master_secret >>> wallet_handle: {:?}, master_secret_id: {:?}", wallet_handle, master_secret_id);

        let master_secret_id = master_secret_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        if self.wallet_service.record_exists::<MasterSecret>(wallet_handle, &master_secret_id)? {
            return Err(IndyError::AnoncredsError(
                AnoncredsError::MasterSecretDuplicateNameError(format!("MasterSecret already exists {}", master_secret_id))));
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
                                 wallet_handle: i32,
                                 prover_did: &str,
                                 cred_offer_json: &str,
                                 cred_def_json: &str,
                                 master_secret_id: &str) -> Result<(String, String), IndyError> {
        debug!("create_credential_request >>> wallet_handle: {:?}, prover_did: {:?}, cred_offer_json: {:?}, cred_def_json: {:?}, master_secret_id: {:?}",
               wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id);

        self.crypto_service.validate_did(&prover_did)?;

        let master_secret: MasterSecret = self._wallet_get_master_secret(wallet_handle, &master_secret_id)?;

        let cred_def: CredentialDefinition = CredentialDefinition::from_json(cred_def_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialDefinition: {:?}", err)))?;

        let cred_offer: CredentialOffer = CredentialOffer::from_json(&cred_offer_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialOffer: {:?}", err)))?;

        let (blinded_ms, ms_blinding_data, blinded_ms_correctness_proof) =
            self.anoncreds_service.prover.new_credential_request(&CredentialDefinitionV1::from(cred_def),
                                                                 &master_secret.value,
                                                                 &cred_offer)?;

        let nonce = new_nonce().map_err(AnoncredsError::from)?;

        let credential_request = CredentialRequest {
            prover_did: prover_did.to_string(),
            cred_def_id: cred_offer.cred_def_id,
            blinded_ms,
            blinded_ms_correctness_proof,
            nonce
        };

        let credential_request_metadata = CredentialRequestMetadata {
            master_secret_blinding_data: ms_blinding_data,
            nonce: credential_request.nonce.clone().map_err(AnoncredsError::from)?,
            master_secret_name: master_secret_id.to_string()
        };

        let cred_req_json = credential_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialRequest: {:?}", err)))?;

        let cred_req_metadata_json = credential_request_metadata.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialRequestMetadata: {:?}", err)))?;

        debug!("create_credential_request <<< cred_req_json: {:?}, cred_req_metadata_json: {:?}", cred_req_json, cred_req_metadata_json);

        Ok((cred_req_json, cred_req_metadata_json))
    }

    fn store_credential(&self,
                        wallet_handle: i32,
                        cred_id: Option<&str>,
                        cred_req_metadata_json: &str,
                        cred_json: &str,
                        cred_def_json: &str,
                        rev_reg_def_json: Option<&str>) -> Result<String, IndyError> {
        debug!("store_credential >>> wallet_handle: {:?}, cred_id: {:?}, cred_req_metadata_json: {:?}, cred_json: {:?}, cred_def_json: {:?}, \
        rev_reg_def_json: {:?}", wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json);

        let cred_request_metadata: CredentialRequestMetadata = CredentialRequestMetadata::from_json(&cred_req_metadata_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialRequestMetadata: {:?}", err)))?;

        let mut credential: Credential = Credential::from_json(&cred_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Credential: {:?}", err)))?;

        let cred_def: CredentialDefinition = CredentialDefinition::from_json(cred_def_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialDefinition: {:?}", err)))?;

        let rev_reg_def = match rev_reg_def_json {
            Some(r_reg_def_json) =>
                Some(RevocationRegistryDefinitionV1::from(
                    RevocationRegistryDefinition::from_json(&r_reg_def_json)
                        .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize RevocationRegistryDefinition: {:?}", err)))?)),
            None => None
        };

        let master_secret: MasterSecret = self._wallet_get_master_secret(wallet_handle, &cred_request_metadata.master_secret_name)?;

        self.anoncreds_service.prover.process_credential(&mut credential,
                                                         &cred_request_metadata,
                                                         &master_secret.value,
                                                         &CredentialDefinitionV1::from(cred_def),
                                                         rev_reg_def.as_ref())?;

        credential.rev_reg = None;
        credential.witness = None;

        let out_cred_id = cred_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        let cred_tags = self.anoncreds_service.prover.build_credential_tags(&credential);
        self.wallet_service.add_indy_object(wallet_handle, &out_cred_id, &credential, &cred_tags)?;

        debug!("store_credential <<< out_cred_id: {:?}", out_cred_id);

        Ok(out_cred_id)
    }

    fn get_credentials(&self,
                       wallet_handle: i32,
                       filter_json: Option<&str>) -> Result<String, IndyError> {
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
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of CredentialInfo: {:?}", err)))?;

        debug!("get_credentials <<< credentials_info_json: {:?}", credentials_info_json);

        Ok(credentials_info_json)
    }

    fn get_credential(&self,
                      wallet_handle: i32,
                      cred_id: &str) -> Result<String, IndyError> {
        debug!("get_credentials >>> wallet_handle: {:?}, cred_id: {:?}", wallet_handle, cred_id);

        let credential: Credential = self.wallet_service.get_indy_object(wallet_handle, &cred_id, &RecordOptions::id_value(), &mut String::new())?;

        let credential_info = self._get_credential_info(&cred_id, credential);

        let credential_info_json = credential_info.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialInfo: {:?}", err)))?;

        debug!("get_credential <<< credential_info_json: {:?}", credential_info_json);

        Ok(credential_info_json)
    }

    fn search_credentials(&self,
                          wallet_handle: i32,
                          query_json: Option<&str>) -> Result<(i32, usize), IndyError> {
        debug!("search_credentials >>> wallet_handle: {:?}, query_json: {:?}", wallet_handle, query_json);

        let credentials_search =
            self.wallet_service.search_indy_records::<Credential>(wallet_handle, query_json.unwrap_or("{}"), &SearchOptions::id_value())?;

        let total_count = credentials_search.get_total_count()?.unwrap_or(0);

        let handle = SequenceUtils::get_next_id();

        self.searches.borrow_mut().insert(handle, Box::new(credentials_search));

        let res = (handle, total_count);

        trace!("search_credentials <<< res: {:?}", res);

        Ok(res)
    }

    fn fetch_credentials(&self,
                         search_handle: i32,
                         count: usize, ) -> Result<String, IndyError> {
        trace!("fetch_credentials >>> search_handle: {:?}, count: {:?}", search_handle, count);

        let mut searches = self.searches.borrow_mut();
        let search = searches.get_mut(&search_handle)
            .ok_or(WalletError::InvalidHandle(format!("Unknown CredentialsSearch handle: {}", search_handle)))?;

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
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of CredentialInfo: {:?}", err)))?;

        trace!("fetch_credentials <<< credentials_info_json: {:?}", credentials_info_json);

        Ok(credentials_info_json)
    }

    fn close_credentials_search(&self, search_handle: i32) -> Result<(), IndyError> {
        trace!("close_credentials_search >>> search_handle: {:?}", search_handle);

        let res = match self.searches.borrow_mut().remove(&search_handle) {
            Some(_) => Ok(()),
            None => Err(WalletError::InvalidHandle(format!("Unknown CredentialsSearch handle: {}", search_handle)))
        }?;

        trace!("close_credentials_search <<< res: {:?}", res);

        Ok(res)
    }

    fn get_credentials_for_proof_req(&self,
                                     wallet_handle: i32,
                                     proof_req_json: &str) -> Result<String, IndyError> {
        debug!("get_credentials_for_proof_req >>> wallet_handle: {:?}, proof_req_json: {:?}", wallet_handle, proof_req_json);

        let proof_request: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ProofRequest: {:?}", err)))?;

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

        let credentials_for_proof_request_json = credentials_for_proof_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialsForProofRequest: {:?}", err)))?;

        debug!("get_credentials_for_proof_req <<< credentials_for_proof_request_json: {:?}", credentials_for_proof_request_json);

        Ok(credentials_for_proof_request_json)
    }

    fn search_credentials_for_proof_req(&self,
                                        wallet_handle: i32,
                                        proof_req_json: &str,
                                        extra_query_json: Option<&str>) -> Result<i32, IndyError> {
        debug!("search_credentials_for_proof_req >>> wallet_handle: {:?}, proof_req_json: {:?}, extra_query_json: {:?}", wallet_handle, proof_req_json, extra_query_json);

        let proof_request: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ProofRequest: {:?}", err)))?;

        let extra_query: Option<ProofRequestExtraQuery> = match extra_query_json {
            Some(query) =>
                serde_json::from_str(query)
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ExtraQuery: {:?}", err)))?,
            None => None
        };

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

        for (predicate_id, requested_predicate) in proof_request.requested_predicates {
            let query_json = self.anoncreds_service.prover.build_query(&requested_predicate.name,
                                                                       &predicate_id,
                                                                       &requested_predicate.restrictions,
                                                                       &extra_query)?;
            let mut credentials_search =
                self.wallet_service.search_indy_records::<Credential>(wallet_handle, &query_json, &SearchOptions::id_value())?;

            let interval = self.anoncreds_service.prover.get_non_revoc_interval(&proof_request.non_revoked, &requested_predicate.non_revoked);

            credentials_for_proof_request_search.insert(predicate_id,
                                                        SearchForProofRequest::new(
                                                            credentials_search, interval, Some(requested_predicate)));
        }

        let search_handle = SequenceUtils::get_next_id();
        self.searches_for_proof_requests.borrow_mut().insert(search_handle, Box::new(credentials_for_proof_request_search));

        debug!("search_credentials_for_proof_req <<< credentials_for_proof_request_json: {:?}", search_handle);

        Ok(search_handle)
    }

    fn fetch_credential_for_proof_request(&self, search_handle: i32, item_referent: &str, count: usize) -> Result<String, IndyError> {
        trace!("fetch_credential_for_proof_request >>> search_handle: {:?}, item_referent: {:?}, count: {:?}", search_handle, item_referent, count);

        let mut searches = self.searches_for_proof_requests.borrow_mut();
        let search: &mut SearchForProofRequest = searches.get_mut(&search_handle)
            .ok_or(WalletError::InvalidHandle(format!("Unknown CredentialsSearch handle: {}", search_handle)))?
            .get_mut(item_referent)
            .ok_or(WalletError::InvalidHandle(format!("Unknown item referent {} for CredentialsSearch handle: {}", item_referent, search_handle)))?;

        let requested_credentials: Vec<RequestedCredential> =
            self._get_requested_credentials(&mut search.search, search.predicate_info.as_ref(), &search.interval, Some(count))?;

        let requested_credentials_json = serde_json::to_string(&requested_credentials)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of RequestedCredential: {:?}", err)))?;

        trace!("fetch_credential_for_proof_request <<< requested_credentials_json: {:?}", requested_credentials_json);

        Ok(requested_credentials_json)
    }

    fn close_credentials_search_for_proof_req(&self, search_handle: i32) -> Result<(), IndyError> {
        trace!("close_credentials_search_for_proof_req >>> search_handle: {:?}", search_handle);

        let res = match self.searches_for_proof_requests.borrow_mut().remove(&search_handle) {
            Some(_) => Ok(()),
            None => Err(WalletError::InvalidHandle(format!("Unknown CredentialsSearch handle: {}", search_handle)))
        }?;

        trace!("close_credentials_search_for_proof_req <<< res: {:?}", res);

        Ok(res)
    }

    fn create_proof(&self,
                    wallet_handle: i32,
                    proof_req_json: &str,
                    requested_credentials_json: &str,
                    master_secret_id: &str,
                    schemas_json: &str,
                    cred_defs_json: &str,
                    rev_states_json: &str) -> Result<String, IndyError> {
        debug!("create_proof >>> wallet_handle: {:?}, proof_req_json: {:?}, requested_credentials_json: {:?}, master_secret_id: {:?}, schemas_json: {:?}, \
        cred_defs_json: {:?}, rev_states_json: {:?}",
               wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, cred_defs_json, rev_states_json);

        let proof_req: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ProofRequest: {:?}", err)))?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of Schema: {:?}", err)))?;

        let cred_defs: HashMap<String, CredentialDefinition> = serde_json::from_str::<HashMap<String, CredentialDefinition>>(cred_defs_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of CredentialDefinition: {:?}", err)))?;

        let rev_states: HashMap<String, HashMap<u64, RevocationState>> = serde_json::from_str(rev_states_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of RevocationState: {:?}", err)))?;

        let requested_credentials: RequestedCredentials = RequestedCredentials::from_json(requested_credentials_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RequestedCredentials: {:?}", err)))?;

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
            let credential: Credential = self.wallet_service.get_indy_object(wallet_handle, &cred_referent, &RecordOptions::id_value(), &mut String::new())?;
            credentials.insert(cred_referent, credential);
        }

        let proof = self.anoncreds_service.prover.create_proof(&credentials,
                                                               &proof_req,
                                                               &requested_credentials,
                                                               &master_secret.value,
                                                               &schemas_map_to_schemas_v1_map(schemas),
                                                               &cred_defs_map_to_cred_defs_v1_map(cred_defs),
                                                               &rev_states)?;

        let proof_json = Proof::to_json(&proof)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize FullProof: {:?}", err)))?;

        debug!("create_proof <<< proof_json: {:?}", proof_json);

        Ok(proof_json)
    }

    fn create_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               rev_reg_def: &str,
                               rev_reg_delta_json: &str,
                               timestamp: u64,
                               cred_rev_id: &str) -> Result<String, IndyError> {
        debug!("create_revocation_state >>> , blob_storage_reader_handle: {:?}, rev_reg_def: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
               blob_storage_reader_handle, rev_reg_def, rev_reg_delta_json, timestamp, cred_rev_id);

        let revoc_reg_def: RevocationRegistryDefinitionV1 =
            RevocationRegistryDefinitionV1::from(RevocationRegistryDefinition::from_json(rev_reg_def)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDefinition: {:?}", err)))?);

        let rev_reg_delta: RevocationRegistryDelta = RevocationRegistryDelta::from_json(rev_reg_delta_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDelta: {:?}", err)))?;

        let rev_idx = parse_cred_rev_id(cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revoc_reg_def)?;

        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);

        let witness = Witness::new(rev_idx, revoc_reg_def.value.max_cred_num, revoc_reg_def.value.issuance_type.to_bool(), &rev_reg_delta.value, &sdk_tails_accessor)
            .map_err(|err| IndyError::CommonError(CommonError::from(err)))?;

        let revocation_state = RevocationState {
            witness,
            rev_reg: RevocationRegistry::from(rev_reg_delta.value),
            timestamp,
        };

        let revocation_state_json = revocation_state.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationState: {:?}", err)))?;

        debug!("create_revocation_state <<< revocation_state_json: {:?}", revocation_state_json);

        Ok(revocation_state_json)
    }

    fn update_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               rev_state_json: &str,
                               rev_reg_def_json: &str,
                               rev_reg_delta_json: &str,
                               timestamp: u64,
                               cred_rev_id: &str) -> Result<String, IndyError> {
        debug!("update_revocation_state >>> blob_storage_reader_handle: {:?}, rev_state_json: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
               blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id);

        let mut rev_state: RevocationState = RevocationState::from_json(rev_state_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationState: {:?}", err)))?;

        let revocation_registry_definition: RevocationRegistryDefinitionV1 =
            RevocationRegistryDefinitionV1::from(RevocationRegistryDefinition::from_json(rev_reg_def_json)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDefinition: {:?}", err)))?);

        let rev_reg_delta: RevocationRegistryDelta = RevocationRegistryDelta::from_json(rev_reg_delta_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDelta: {:?}", err)))?;

        let rev_idx = parse_cred_rev_id(cred_rev_id)?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revocation_registry_definition)?;

        let rev_reg_delta = RevocationRegistryDeltaV1::from(rev_reg_delta);

        rev_state.witness.update(rev_idx, revocation_registry_definition.value.max_cred_num, &rev_reg_delta.value, &sdk_tails_accessor)
            .map_err(|err| IndyError::CommonError(CommonError::from(err)))?;

        rev_state.rev_reg = RevocationRegistry::from(rev_reg_delta.value);
        rev_state.timestamp = timestamp;

        let rev_state_json = rev_state.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationState: {:?}", err)))?;

        debug!("update_revocation_state <<< rev_state_json: {:?}", rev_state_json);

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
                       record: &WalletRecord) -> Result<(String, Credential), IndyError> {
        let referent = record.get_id();
        let value = record.get_value()
            .ok_or(CommonError::InvalidStructure(format!("Credential not found for id: {}", referent)))?;

        let credential: Credential = Credential::from_json(value)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize Credential: {:?}", err)))?;

        Ok((referent.to_string(), credential))
    }

    fn _query_requested_credentials(&self,
                                    wallet_handle: i32,
                                    query_json: &str,
                                    predicate_info: Option<&PredicateInfo>,
                                    interval: &Option<NonRevocedInterval>) -> Result<Vec<RequestedCredential>, IndyError> {
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
                                  max_count: Option<usize>) -> Result<Vec<RequestedCredential>, IndyError> {

        let mut credentials: Vec<RequestedCredential> = Vec::new();

        if let Some(0) = max_count {
            return Ok(vec![]);
        }

        while let Some(credential_record) = credentials_search.fetch_next_record()? {
            let (referent, credential) = self._get_credential(&credential_record)?;

            if let Some(predicate) = predicate_info {
                let satisfy = self.anoncreds_service.prover.attribute_satisfy_predicate(predicate,
                                                                                        &credential.values[&attr_common_view(&predicate.name)].encoded)?;
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


    fn _wallet_get_master_secret(&self, wallet_handle: i32, key: &str) -> Result<MasterSecret, WalletError> {
        self.wallet_service.get_indy_object(wallet_handle, &key, &RecordOptions::id_value(), &mut String::new())
    }
}

