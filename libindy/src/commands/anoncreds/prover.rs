extern crate serde_json;
extern crate uuid;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::AnoncredsService;
use services::wallet::WalletService;
use std::rc::Rc;
use services::anoncreds::helpers::get_composite_id;
use services::anoncreds::types::*;
use std::collections::{HashMap, HashSet};
use utils::crypto::base58::Base58;
use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use self::uuid::Uuid;

pub enum ProverCommand {
    StoreClaimOffer(
        i32, // wallet handle
        String, // claim offer json
        Box<Fn(Result<(), IndyError>) + Send>),
    GetClaimOffers(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateMasterSecret(
        i32, // wallet handle
        String, // master secret name
        Box<Fn(Result<(), IndyError>) + Send>),
    CreateAndStoreClaimRequest(
        i32, // wallet handle
        String, // prover did
        String, // claim offer json
        String, // claim def json
        String, // master secret name
        Box<Fn(Result<String, IndyError>) + Send>),
    StoreClaim(
        i32, // wallet handle
        String, // claims json
        Option<String>, // revocation registry json
        Box<Fn(Result<(), IndyError>) + Send>),
    GetClaims(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, IndyError>) + Send>),
    GetClaimsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateProof(
        i32, // wallet handle
        String, // proof request json
        String, // requested claims json
        String, // schemas json
        String, // master secret name
        String, // claim defs json
        String, // revoc regs json
        Box<Fn(Result<String, IndyError>) + Send>),
}

pub struct ProverCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    wallet_service: Rc<WalletService>
}

impl ProverCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               wallet_service: Rc<WalletService>) -> ProverCommandExecutor {
        ProverCommandExecutor {
            anoncreds_service,
            wallet_service,
        }
    }

    pub fn execute(&self, command: ProverCommand) {
        match command {
            ProverCommand::StoreClaimOffer(wallet_handle, claim_offer_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaimOffer command received");
                cb(self.store_claim_offer(wallet_handle, &claim_offer_json));
            }
            ProverCommand::GetClaimOffers(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimOffers command received");
                cb(self.get_claim_offers(wallet_handle, &filter_json));
            }
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateMasterSecret command received");
                cb(self.create_master_secret(wallet_handle, &master_secret_name));
            }
            ProverCommand::CreateAndStoreClaimRequest(wallet_handle, prover_did, claim_offer_json,
                                                      claim_def_json, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateAndStoreClaimRequest command received");
                cb(self.create_and_store_claim_request(wallet_handle, &prover_did, &claim_offer_json,
                                                       &claim_def_json, &master_secret_name));
            }
            ProverCommand::StoreClaim(wallet_handle, claims_json, rev_reg_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaim command received");
                cb(self.store_claim(wallet_handle, &claims_json, rev_reg_json.as_ref().map(String::as_str)));
            }
            ProverCommand::GetClaims(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaims command received");
                cb(self.get_claims(wallet_handle, &filter_json));
            }
            ProverCommand::GetClaimsForProofReq(wallet_handle, proof_req_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimsForProofReq command received");
                cb(self.get_claims_for_proof_req(wallet_handle, &proof_req_json));
            }
            ProverCommand::CreateProof(wallet_handle, proof_req_json, requested_claims_json, schemas_jsons,
                                       master_secret_name, claim_def_jsons, revoc_regs_jsons, cb) => {
                info!(target: "prover_command_executor", "CreateProof command received");
                cb(self.create_proof(wallet_handle, &proof_req_json, &requested_claims_json, &schemas_jsons,
                                     &master_secret_name, &claim_def_jsons, &revoc_regs_jsons));
            }
        };
    }

    fn store_claim_offer(&self, wallet_handle: i32, claim_offer_json: &str) -> Result<(), IndyError> {
        info!("store_claim_offer >>> wallet_handle: {:?}, claim_offer_json: {:?}", wallet_handle, claim_offer_json);

        let claim_offer: ClaimOffer = ClaimOffer::from_json(claim_offer_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize claim offer: {:?}", err)))?;

        Base58::decode(&claim_offer.issuer_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid issuer did: {:?}", err)))?;

        let id = get_composite_id(&claim_offer.issuer_did, &claim_offer.schema_key);
        self.wallet_service.set(wallet_handle, &format!("claim_offer::{}", &id), &claim_offer_json)?;

        info!("store_claim_offer <<<");

        Ok(())
    }

    fn get_claim_offers(&self,
                        wallet_handle: i32,
                        filter_json: &str) -> Result<String, IndyError> {
        info!("get_claim_offers >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

        let claim_offer_jsons: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("claim_offer::"))?;

        let mut claim_offers: Vec<ClaimOffer> = Vec::new();
        for &(ref id, ref claim_offer_json) in claim_offer_jsons.iter() {
            claim_offers.push(ClaimOffer::from_json(claim_offer_json)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize claim offer: {:?}", err)))?);
        }

        let filter: Filter = Filter::from_json(filter_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize filter: {:?}", err)))?;

        claim_offers.retain(move |claim_offer| {
            let mut condition = true;
            if let Some(ref issuer_did) = filter.issuer_did {
                condition = condition && claim_offer.issuer_did == issuer_did.clone();
            }
            if let Some(ref schema_key) = filter.schema_key {
                if let Some(ref name) = schema_key.name {
                    condition = condition && claim_offer.schema_key.name == name.clone();
                }
                if let Some(ref version) = schema_key.version {
                    condition = condition && claim_offer.schema_key.version == version.clone();
                }
                if let Some(ref did) = schema_key.did {
                    condition = condition && claim_offer.schema_key.did == did.clone();
                }
            }
            condition
        });

        let claim_offers_json = serde_json::to_string(&claim_offers)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of claim offer: {:?}", err)))?;

        info!("get_claim_offers <<< claim_offers_json: {:?}", claim_offers_json);

        Ok(claim_offers_json)
    }

    fn create_master_secret(&self, wallet_handle: i32, master_secret_name: &str) -> Result<(), IndyError> {
        info!("create_master_secret >>> wallet_handle: {:?}, master_secret_name: {:?}", wallet_handle, master_secret_name);

        if let Ok(_) = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", master_secret_name)) {
            return Err(IndyError::AnoncredsError(
                AnoncredsError::MasterSecretDuplicateNameError(format!("Master Secret already exists {}", master_secret_name))));
        };

        let master_secret = self.anoncreds_service.prover.new_master_secret()?;
        let master_secret_json = master_secret.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize master secret: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("master_secret::{}", master_secret_name), &master_secret_json)?;

        info!("create_master_secret <<<");

        Ok(())
    }

    fn create_and_store_claim_request(&self,
                                      wallet_handle: i32,
                                      prover_did: &str,
                                      claim_offer_json: &str,
                                      claim_def_json: &str,
                                      master_secret_name: &str) -> Result<String, IndyError> {
        info!("create_and_store_claim_request >>> wallet_handle: {:?}, prover_did: {:?}, claim_offer_json: {:?}, claim_def_json: {:?}, \
               master_secret_name: {:?}", wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name);

        Base58::decode(&prover_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid prover did: {:?}", err)))?;

        let master_secret_json = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", &master_secret_name))?;
        let master_secret = MasterSecret::from_json(&master_secret_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize master secret: {:?}", err)))?;

        let claim_def: ClaimDefinition = ClaimDefinition::from_json(&claim_def_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize claim definition: {:?}", err)))?;

        let claim_offer: ClaimOffer = ClaimOffer::from_json(&claim_offer_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize claim offer: {:?}", err)))?;

        if claim_def.issuer_did != claim_offer.issuer_did {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("ClaimOffer issuer_did {:?} does not correspond to ClaimDef issuer_did {:?}", claim_offer.issuer_did, claim_def.issuer_did))));
        }

        let (claim_request, master_secret_blinding_data) =
            self.anoncreds_service.prover.new_claim_request(&claim_def.data, &master_secret, &claim_offer, prover_did)?;

        let nonce = claim_request.nonce.clone()
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize nonce: {:?}", err)))?;

        let claim_request_metadata = ClaimRequestMetadata {
            master_secret_blinding_data,
            nonce,
            master_secret_name: master_secret_name.to_string()
        };

        let claim_request_metadata_json = claim_request_metadata.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize claim request metadata {:?}", err)))?;

        let id = get_composite_id(&claim_offer.issuer_did, &claim_offer.schema_key);
        self.wallet_service.set(wallet_handle, &format!("claim_request_metadata::{}", id), &claim_request_metadata_json)?;

        let claim_request_json = claim_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize claim request: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("claim_definition::{}", id), &claim_def_json)?;

        info!("create_and_store_claim_request <<< claim_request_json: {:?}", claim_request_json);

        Ok(claim_request_json)
    }

    fn store_claim(&self, wallet_handle: i32, claim_json: &str, rev_reg_json: Option<&str>) -> Result<(), IndyError> {
        info!("store_claim >>> wallet_handle: {:?}, claim_json: {:?}, rev_reg_json: {:?}", wallet_handle, claim_json, rev_reg_json);

        let mut claim: Claim = Claim::from_json(&claim_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannon deserialize claim: {:?}", err)))?;

        let id = get_composite_id(&claim.issuer_did, &claim.schema_key);

        let rev_reg_pub = match rev_reg_json {
            Some(rev_reg_pub_json) =>
                Some(RevocationRegistry::from_json(&rev_reg_pub_json)
                    .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize revocation registry: {:?}", err)))?
                    .data),
            None => None
        };

        let claim_request_metadata_json = self.wallet_service.get(wallet_handle, &format!("claim_request_metadata::{}", &id))?;
        let claim_request_metadata = ClaimRequestMetadata::from_json(&claim_request_metadata_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize claim request metadata: {:?}", err)))?;

        let master_secret_json = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", &claim_request_metadata.master_secret_name))?;
        let master_secret = MasterSecret::from_json(&master_secret_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize master secret: {:?}", err)))?;

        let claim_def_json = self.wallet_service.get(wallet_handle, &format!("claim_definition::{}", id))?;
        let claim_def: ClaimDefinition = ClaimDefinition::from_json(&claim_def_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize claim definition: {:?}", err)))?;

        self.anoncreds_service.prover.process_claim(&mut claim,
                                                    &claim_request_metadata,
                                                    &master_secret,
                                                    &claim_def.data,
                                                    rev_reg_pub.as_ref())?;

        let claim_json = claim.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize claim: {:?}", err)))?;

        let referent = Uuid::new_v4().to_string();
        self.wallet_service.set(wallet_handle, &format!("claim::{}", &referent), &claim_json)?;

        if let Some(rev_reg_json) = rev_reg_json {
            self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", id), &rev_reg_json)?;
        }

        info!("store_claim <<<");

        Ok(())
    }

    fn get_claims(&self,
                  wallet_handle: i32,
                  filter_json: &str) -> Result<String, IndyError> {
        info!("get_claims >>> wallet_handle: {:?}, filter_json: {:?}", wallet_handle, filter_json);

        let mut claims_info: Vec<ClaimInfo> = self.get_claims_info(wallet_handle)?;

        let filter: Filter = Filter::from_json(filter_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize filter: {:?}", err)))?;

        claims_info.retain(move |claim_info| self.anoncreds_service.prover.claim_satisfy_restriction(claim_info, &filter));

        let claims_info_json = serde_json::to_string(&claims_info)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize claims info: {:?}", err)))?;

        info!("get_claims <<< claims_info_json: {:?}", claims_info_json);

        Ok(claims_info_json)
    }

    fn get_claims_info(&self, wallet_handle: i32) -> Result<Vec<ClaimInfo>, IndyError> {
        info!("get_claims_info >>>");

        let claims: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("claim::"))?;

        let mut claims_info: Vec<ClaimInfo> = Vec::new();

        for &(ref referent, ref claim) in claims.iter() {
            let claim: Claim = Claim::from_json(claim)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize claim: {:?}", err)))?;

            let mut claim_values: HashMap<String, String> = HashMap::new();
            for (attr, values) in claim.values {
                claim_values.insert(attr.clone(), values[0].clone());
            }

            claims_info.push(
                ClaimInfo {
                    referent: referent.clone(),
                    attrs: claim_values,
                    schema_key: claim.schema_key.clone(),
                    issuer_did: claim.issuer_did.clone(),
                    revoc_reg_seq_no: claim.rev_reg_seq_no.clone()
                });
        }

        info!("get_claims_info <<< claims_info: {:?}", claims_info);

        Ok(claims_info)
    }

    fn get_claims_for_proof_req(&self,
                                wallet_handle: i32,
                                proof_req_json: &str, ) -> Result<String, IndyError> {
        info!("get_claims_for_proof_req >>> wallet_handle: {:?}, proof_req_json: {:?}", wallet_handle, proof_req_json);

        let proof_req: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize proof request: {:?}", err)))?;

        let claims_info: Vec<ClaimInfo> = self.get_claims_info(wallet_handle)?;

        let claims_for_proof_request = self.anoncreds_service.prover.get_claims_for_proof_req(&proof_req, &claims_info)?;
        let claims_for_proof_request_json = claims_for_proof_request.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize claims for proof request: {:?}", err)))?;

        info!("get_claims_for_proof_req <<< claims_for_proof_request_json: {:?}", claims_for_proof_request_json);

        Ok(claims_for_proof_request_json)
    }

    fn create_proof(&self,
                    wallet_handle: i32,
                    proof_req_json: &str,
                    requested_claims_json: &str,
                    schemas_jsons: &str,
                    master_secret_name: &str,
                    claim_def_jsons: &str,
                    revoc_regs_jsons: &str) -> Result<String, IndyError> {
        let proof_req: ProofRequest = ProofRequest::from_json(proof_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize proof request: {:?}", err)))?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_jsons)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of schemas: {:?}", err)))?;

        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_def_jsons)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of claim definitions: {:?}", err)))?;

        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_jsons)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of revocation registries: {:?}", err)))?;

        let requested_claims: RequestedClaims = RequestedClaims::from_json(requested_claims_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize requested claims: {:?}", err)))?;

        if schemas.keys().collect::<HashSet<&String>>() != claim_defs.keys().collect::<HashSet<&String>>() {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Claim definition keys {:?} do not correspond to schema received {:?}", schemas.keys(), claim_defs.keys()))));
        }

        let mut claims: HashMap<String, Claim> = HashMap::new();
        for key_id in claim_defs.keys() {
            let claim_json = self.wallet_service.get(wallet_handle, key_id)?;
            let claim = Claim::from_json(&claim_json)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize claim: {:?}", err)))?;

            claims.insert(key_id.clone(), claim);
        }

        let master_secret_json = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", master_secret_name))?;
        let master_secret = MasterSecret::from_json(&master_secret_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize master secret: {:?}", err)))?;


        let proof_claims = self.anoncreds_service.prover.create_proof(&claims,
                                                                      &proof_req,
                                                                      &schemas,
                                                                      &claim_defs,
                                                                      &revoc_regs,
                                                                      &requested_claims,
                                                                      &master_secret)?;

        let proof_claims_json = FullProof::to_json(&proof_claims)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize proof: {:?}", err)))?;

        Ok(proof_claims_json)
    }
}