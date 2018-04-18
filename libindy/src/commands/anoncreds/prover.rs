extern crate serde_json;
extern crate indy_crypto;
extern crate uuid;

use errors::common::CommonError;
use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::AnoncredsService;
use services::anoncreds::helpers::parse_cred_rev_id;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use std::rc::Rc;
use services::blob_storage::BlobStorageService;
use std::collections::{HashMap, HashSet};
use self::indy_crypto::cl::{MasterSecret, Witness, RevocationRegistry, new_nonce};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use super::tails::SDKTailsAccessor;

use domain::schema::{Schema, schemas_map_to_schemas_v1_map};
use domain::credential::{Credential, CredentialInfo};
use domain::credential_definition::{CredentialDefinition, CredentialDefinitionV1, cred_defs_map_to_cred_defs_v1_map};
use domain::credential_offer::CredentialOffer;
use domain::credential_request::{CredentialRequest, CredentialRequestMetadata};
use domain::filter::Filter;
use domain::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::proof::Proof;
use domain::proof_request::ProofRequest;
use domain::requested_credential::RequestedCredentials;
use domain::revocation_state::RevocationState;

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
        String, // credential request json
        String, // credential request metadata json
        String, // credentials json
        String, // credential definition json
        Option<String>, // revocation registry definition json
        Box<Fn(Result<String, IndyError>) + Send>),
    GetCredentials(
        i32, // wallet handle
        Option<String>, // filter json
        Box<Fn(Result<String, IndyError>) + Send>),
    GetCredentialsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Box<Fn(Result<String, IndyError>) + Send>),
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

pub struct ProverCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    blob_storage_service: Rc<BlobStorageService>,
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
            blob_storage_service
        }
    }

    pub fn execute(&self, command: ProverCommand) {
        match command {
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_id, cb) => {
                trace!(target: "prover_command_executor", "CreateMasterSecret command received");
                cb(self.create_master_secret(wallet_handle, master_secret_id.as_ref().map(String::as_str)));
            }
            ProverCommand::CreateCredentialRequest(wallet_handle, prover_did, credential_offer_json,
                                                   credential_def_json, master_secret_name, cb) => {
                trace!(target: "prover_command_executor", "CreateCredentialRequest command received");
                cb(self.create_credential_request(wallet_handle, &prover_did, &credential_offer_json,
                                                  &credential_def_json, &master_secret_name));
            }
            ProverCommand::StoreCredential(wallet_handle, cred_id, cred_req_json, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb) => {
                trace!(target: "prover_command_executor", "StoreCredential command received");
                cb(self.store_credential(wallet_handle, cred_id.as_ref().map(String::as_str),
                                         &cred_req_json, &cred_req_metadata_json, &cred_json, &cred_def_json,
                                         rev_reg_def_json.as_ref().map(String::as_str)));
            }
            ProverCommand::GetCredentials(wallet_handle, filter_json, cb) => {
                trace!(target: "prover_command_executor", "GetCredentials command received");
                cb(self.get_credentials(wallet_handle, filter_json.as_ref().map(String::as_str)));
            }
            ProverCommand::GetCredentialsForProofReq(wallet_handle, proof_req_json, cb) => {
                trace!(target: "prover_command_executor", "GetCredentialsForProofReq command received");
                cb(self.get_credentials_for_proof_req(wallet_handle, &proof_req_json));
            }
            ProverCommand::CreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name,
                                       schemas_json, credential_defs_json, rev_states_json, cb) => {
                trace!(target: "prover_command_executor", "CreateProof command received");
                cb(self.create_proof(wallet_handle, &proof_req_json, &requested_credentials_json, &master_secret_name,
                                     &schemas_json, &credential_defs_json, &rev_states_json));
            }
            ProverCommand::CreateRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb) => {
                trace!(target: "prover_command_executor", "CreateRevocationState command received");
                cb(self.create_revocation_state(blob_storage_reader_handle, &rev_reg_def_json, &rev_reg_delta_json, timestamp, &cred_rev_id));
            }
            ProverCommand::UpdateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb) => {
                trace!(target: "prover_command_executor", "UpdateRevocationState command received");
                cb(self.update_revocation_state(blob_storage_reader_handle, &rev_state_json, &rev_reg_def_json, &rev_reg_delta_json, timestamp, &cred_rev_id));
            }
        };
    }

    fn create_master_secret(&self,
                            wallet_handle: i32,
                            master_secret_id: Option<&str>) -> Result<String, IndyError> {
        trace!("create_master_secret >>> wallet_handle: {:?}, master_secret_id: {:?}", wallet_handle, master_secret_id);

        let master_secret_id = master_secret_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        if let Ok(_) = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", master_secret_id)) {
            return Err(IndyError::AnoncredsError(
                AnoncredsError::MasterSecretDuplicateNameError(format!("MasterSecret already exists {}", master_secret_id))));
        };

        let master_secret = self.anoncreds_service.prover.new_master_secret()?;
        self.wallet_service.set_object(wallet_handle, &format!("master_secret::{}", master_secret_id), &master_secret, "MasterSecret")?;

        trace!("create_master_secret <<< master_secret_id: {:?}", master_secret_id);

        Ok(master_secret_id)
    }

    fn create_credential_request(&self,
                                 wallet_handle: i32,
                                 prover_did: &str,
                                 cred_offer_json: &str,
                                 cred_def_json: &str,
                                 master_secret_id: &str) -> Result<(String, String), IndyError> {
        trace!("create_credential_request >>> wallet_handle: {:?}, prover_did: {:?}, cred_offer_json: {:?}, cred_def_json: {:?}, master_secret_id: {:?}",
               wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id);

        self.crypto_service.validate_did(&prover_did)?;

        let master_secret: MasterSecret =
            self.wallet_service.get_object(wallet_handle, &format!("master_secret::{}", &master_secret_id), "MasterSecret", &mut String::new())?;

        let cred_def: CredentialDefinition = CredentialDefinition::from_json(cred_def_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialDefinition: {:?}", err)))?;

        let cred_offer: CredentialOffer = CredentialOffer::from_json(&cred_offer_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialOffer: {:?}", err)))?;

        let (blinded_ms, ms_blinding_data, blinded_ms_correctness_proof) =
            self.anoncreds_service.prover.new_credential_request(&CredentialDefinitionV1::from(cred_def),
                                                                 &master_secret,
                                                                 &cred_offer)?;

        let nonce = new_nonce()
            .map_err(|err| IndyError::AnoncredsError(AnoncredsError::from(err)))?;

        let credential_request = CredentialRequest {
            prover_did: prover_did.to_string(),
            cred_def_id: cred_offer.cred_def_id.clone(),
            blinded_ms,
            blinded_ms_correctness_proof,
            nonce
        };

        let nonce = credential_request.nonce.clone()
            .map_err(|err| IndyError::AnoncredsError(AnoncredsError::from(err)))?;

        let credential_request_metadata = CredentialRequestMetadata {
            master_secret_blinding_data: ms_blinding_data,
            nonce,
            master_secret_name: master_secret_id.to_string()
        };

        let cred_req_json = credential_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialRequest: {:?}", err)))?;

        let cred_req_metadata_json = credential_request_metadata.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialRequestMetadata: {:?}", err)))?;

        trace!("create_credential_request <<< cred_req_json: {:?}, cred_req_metadata_json: {:?}", cred_req_json, cred_req_metadata_json);

        Ok((cred_req_json, cred_req_metadata_json))
    }

    fn store_credential(&self,
                        wallet_handle: i32,
                        cred_id: Option<&str>,
                        cred_req_json: &str,
                        cred_req_metadata_json: &str,
                        cred_json: &str,
                        cred_def_json: &str,
                        rev_reg_def_json: Option<&str>) -> Result<String, IndyError> {
        trace!("store_credential >>> wallet_handle: {:?}, cred_id: {:?}, cred_req_json: {:?}, cred_req_metadata_json: {:?}, cred_json: {:?}, cred_def_json: {:?}, \
        rev_reg_def_json: {:?}", wallet_handle, cred_id, cred_req_json, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json);

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

        let master_secret: MasterSecret =
            self.wallet_service.get_object(wallet_handle, &format!("master_secret::{}", &cred_request_metadata.master_secret_name), "MasterSecret", &mut String::new())?;

        self.anoncreds_service.prover.process_credential(&mut credential,
                                                         &cred_request_metadata,
                                                         &master_secret,
                                                         &CredentialDefinitionV1::from(cred_def),
                                                         rev_reg_def.as_ref())?;

        credential.rev_reg = None;
        credential.witness = None;

        let out_cred_id = cred_id.map(String::from).unwrap_or(uuid::Uuid::new_v4().to_string());

        self.wallet_service.set_object(wallet_handle, &format!("credential::{}", &out_cred_id), &credential, "Credential")?;

        trace!("store_credential <<< out_cred_id: {:?}", out_cred_id);

        Ok(out_cred_id)
    }

    fn get_credentials(&self,
                       wallet_handle: i32,
                       filter_json: Option<&str>) -> Result<String, IndyError> {
        trace!("get_credentials >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

        let mut credentials_info: Vec<CredentialInfo> = self.get_credentials_info(wallet_handle)?;

        let filter: Filter = Filter::from_json(filter_json.unwrap_or("{}"))
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Filter: {:?}", err)))?;

        credentials_info.retain(move |credential_info|
            self.anoncreds_service.prover.satisfy_restriction(credential_info, &filter));

        let credentials_info_json = serde_json::to_string(&credentials_info)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of CredentialInfo: {:?}", err)))?;

        trace!("get_credentials <<< credentials_info_json: {:?}", credentials_info_json);

        Ok(credentials_info_json)
    }

    fn get_credentials_info(&self,
                            wallet_handle: i32) -> Result<Vec<CredentialInfo>, IndyError> {
        trace!("get_credentials_info >>> wallet_handle: {:?}", wallet_handle);

        let credentials: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("credential::"))?;

        let mut credentials_info: Vec<CredentialInfo> = Vec::new();

        for &(ref referent, ref credential) in credentials.iter() {
            let credential: Credential = Credential::from_json(credential)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize Credential: {:?}", err)))?;

            let mut credential_values: HashMap<String, String> = HashMap::new();
            for (attr, values) in credential.values {
                credential_values.insert(attr.clone(), values.raw.clone());
            }

            credentials_info.push(
                CredentialInfo {
                    referent: referent.replace("credential::", ""),
                    attrs: credential_values,
                    schema_id: credential.schema_id.clone(),
                    cred_def_id: credential.cred_def_id.clone(),
                    rev_reg_id: credential.rev_reg_id.as_ref().map(|s| s.to_string()),
                    cred_rev_id: credential.signature.extract_index().map(|idx| idx.to_string())
                });
        }

        trace!("get_credentials_info <<< credentials_info: {:?}", credentials_info);

        Ok(credentials_info)
    }

    fn get_credentials_for_proof_req(&self,
                                     wallet_handle: i32,
                                     proof_req_json: &str, ) -> Result<String, IndyError> {
        trace!("get_credentials_for_proof_req >>> wallet_handle: {:?}, proof_req_json: {:?}", wallet_handle, proof_req_json);

        let proof_request: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ProofRequest: {:?}", err)))?;

        let mut credentials: Vec<CredentialInfo> = self.get_credentials_info(wallet_handle)?;

        let credentials_for_proof_request = self.anoncreds_service.prover.get_credentials_for_proof_req(&proof_request, &mut credentials)?;
        let credentials_for_proof_request_json = credentials_for_proof_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialsForProofRequest: {:?}", err)))?;

        trace!("get_credentials_for_proof_req <<< credentials_for_proof_request_json: {:?}", credentials_for_proof_request_json);

        Ok(credentials_for_proof_request_json)
    }

    fn create_proof(&self,
                    wallet_handle: i32,
                    proof_req_json: &str,
                    requested_credentials_json: &str,
                    master_secret_id: &str,
                    schemas_json: &str,
                    cred_defs_json: &str,
                    rev_states_json: &str) -> Result<String, IndyError> {
        trace!("create_proof >>> wallet_handle: {:?}, proof_req_json: {:?}, requested_credentials_json: {:?}, master_secret_id: {:?}, schemas_json: {:?}, \
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

        let master_secret: MasterSecret =
            self.wallet_service.get_object(wallet_handle, &format!("master_secret::{}", master_secret_id), "MasterSecret", &mut String::new())?;

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

        for cred_referent in cred_referents {
            let credential: Credential = self.wallet_service.get_object(wallet_handle, &format!("credential::{}", &cred_referent), "Credential", &mut String::new())?;
            credentials.insert(cred_referent.clone(), credential);
        }

        let proof = self.anoncreds_service.prover.create_proof(&credentials,
                                                               &proof_req,
                                                               &requested_credentials,
                                                               &master_secret,
                                                               &schemas_map_to_schemas_v1_map(schemas),
                                                               &cred_defs_map_to_cred_defs_v1_map(cred_defs),
                                                               &rev_states)?;

        let proof_json = Proof::to_json(&proof)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize FullProof: {:?}", err)))?;

        trace!("create_proof <<< proof_json: {:?}", proof_json);

        Ok(proof_json)
    }

    fn create_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               rev_reg_def: &str,
                               rev_reg_delta_json: &str,
                               timestamp: u64,
                               cred_rev_id: &str) -> Result<String, IndyError> {
        trace!("create_revocation_state >>> , blob_storage_reader_handle: {:?}, rev_reg_def: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
               blob_storage_reader_handle, rev_reg_def, rev_reg_delta_json, timestamp, cred_rev_id);

        let revoc_reg_def: RevocationRegistryDefinitionV1 =
            RevocationRegistryDefinitionV1::from(RevocationRegistryDefinition::from_json(rev_reg_def)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDefinition: {:?}", err)))?);

        let rev_reg_delta: RevocationRegistryDelta = RevocationRegistryDelta::from_json(rev_reg_delta_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDelta: {:?}", err)))?;

        let rev_idx = parse_cred_rev_id(cred_rev_id)?;
        ;

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

        trace!("create_revocation_state <<< revocation_state_json: {:?}", revocation_state_json);

        Ok(revocation_state_json)
    }

    fn update_revocation_state(&self,
                               blob_storage_reader_handle: i32,
                               rev_state_json: &str,
                               rev_reg_def_json: &str,
                               rev_reg_delta_json: &str,
                               timestamp: u64,
                               cred_rev_id: &str) -> Result<String, IndyError> {
        trace!("update_revocation_state >>> blob_storage_reader_handle: {:?}, rev_state_json: {:?}, rev_reg_def_json: {:?}, rev_reg_delta_json: {:?}, timestamp: {:?}, cred_rev_id: {:?}",
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

        trace!("update_revocation_state <<< rev_state_json: {:?}", rev_state_json);

        Ok(rev_state_json)
    }
}

