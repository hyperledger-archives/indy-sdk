extern crate serde_json;
extern crate uuid;

use self::uuid::Uuid;
use errors::anoncreds::AnoncredsError;
use services::crypto::CryptoService;
use services::crypto::wrappers::bn::BigNumber;
use services::pool::PoolService;
use utils::json::{JsonDecodable, JsonEncodable};
use services::wallet::WalletService;
use std::rc::Rc;
use services::crypto::anoncreds::types::{
    ClaimDefinition,
    Schema,
    RevocationRegistry,
    ClaimOfferFilter,
    ClaimInfoFilter,
    ProofJson,
    ClaimInfo,
    ProofClaimsJson,
    ProofRequestJson,
    RequestedClaimsJson,
    ClaimJson,
    ClaimOffer,
    ClaimInitData,
    RevocationClaimInitData,
    ClaimRequestJson
};
use std::collections::HashMap;
use services::crypto::wrappers::pair::PointG1;
use std::cell::RefCell;

pub enum ProverCommand {
    StoreClaimOffer(
        i32, // wallet handle
        String, // claim offer json
        Box<Fn(Result<(), AnoncredsError>) + Send>),
    GetClaimOffers(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, AnoncredsError>) + Send>),
    CreateMasterSecret(
        i32, // wallet handle
        String, // master secret name
        Box<Fn(Result<(), AnoncredsError>) + Send>),
    CreateAndStoreClaimRequest(
        i32, // wallet handle
        String, // prover_did
        String, // claim offer json
        String, // claim def json
        String, // master secret name
        Box<Fn(Result<String, AnoncredsError>) + Send>),
    StoreClaim(
        i32, // wallet handle
        String, // claims json
        Box<Fn(Result<(), AnoncredsError>) + Send>),
    GetClaims(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, AnoncredsError>) + Send>),
    GetClaimsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Box<Fn(Result<String, AnoncredsError>) + Send>),
    CreateProof(
        i32, // wallet handle
        String, // proof request json
        String, // requested claims json
        String, // schemas json
        String, // claim defs json
        String, // revoc regs json
        Box<Fn(Result<String, AnoncredsError>) + Send>),
}

pub struct ProverCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>
}

impl ProverCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> ProverCommandExecutor {
        ProverCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: ProverCommand) {
        match command {
            ProverCommand::StoreClaimOffer(wallet_handle, claim_offer_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaimOffer command received");
                self.store_claim_offer(wallet_handle, &claim_offer_json, cb);
            }
            ProverCommand::GetClaimOffers(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimOffers command received");
                self.get_claim_offers(wallet_handle, &filter_json, cb);
            }
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateMasterSecret command received");
                self.create_master_secret(wallet_handle, &master_secret_name, cb);
            }
            ProverCommand::CreateAndStoreClaimRequest(wallet_handle, prover_did, claim_offer_json,
                                                      claim_def_json, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateAndStoreClaimRequest command received");
                self.create_and_store_claim_request(wallet_handle, &prover_did, &claim_offer_json,
                                                    &claim_def_json, &master_secret_name, cb);
            }
            ProverCommand::StoreClaim(wallet_handle, claims_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaim command received");
                self.store_claim(wallet_handle, &claims_json, cb);
            }
            ProverCommand::GetClaims(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaims command received");
                self.get_claims(wallet_handle, &filter_json, cb);
            }
            ProverCommand::GetClaimsForProofReq(wallet_handle, proof_req_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimsForProofReq command received");
                self.get_claims_for_proof_req(wallet_handle, &proof_req_json, cb);
            }
            ProverCommand::CreateProof(wallet_handle, proof_req_json, requested_claims_json, schemas_jsons,
                                       claim_def_jsons, revoc_regs_jsons, cb) => {
                info!(target: "prover_command_executor", "CreateProof command received");
                self.create_proof(wallet_handle, &proof_req_json, &requested_claims_json, &schemas_jsons,
                                  &claim_def_jsons, &revoc_regs_jsons, cb);
            }
        };
    }

    fn store_claim_offer(&self,
                         wallet_handle: i32,
                         claim_offer_json: &str,
                         cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        cb(self._store_claim_offer(wallet_handle, claim_offer_json));
    }

    fn _store_claim_offer(&self, wallet_handle: i32, claim_offer_json: &str) -> Result<(), AnoncredsError> {
        let uuid = Uuid::new_v4().to_string();
        self.wallet_service.set(wallet_handle, &format!("claim_offer_json::{}", &uuid), &claim_offer_json)?;

        Ok(())
    }

    fn get_claim_offers(&self,
                        wallet_handle: i32,
                        filter_json: &str,
                        cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(self._get_claim_offers(wallet_handle, filter_json));
    }

    fn _get_claim_offers(&self,
                         wallet_handle: i32,
                         filter_json: &str) -> Result<String, AnoncredsError> {
        let claim_offer_jsons: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("claim_offer_json::"))?;

        let mut claim_offers: Vec<ClaimOffer> = Vec::new();

        for &(ref uuid, ref claim_offer_json) in claim_offer_jsons.iter() {
            claim_offers.push(ClaimOffer::from_json(claim_offer_json)?);
        }

        let filter = ClaimOfferFilter::from_json(filter_json)?;

        claim_offers.retain(move |claim_offer| {
            let mut condition = true;
            if let Some(ref claim_def_seq_no) = filter.claim_def_seq_no {
                condition = claim_offer.claim_def_seq_no == claim_def_seq_no.clone();
            }
            if let Some(ref issuer_did) = filter.issuer_did {
                condition = claim_offer.issuer_did == issuer_did.clone();
            }
            condition
        });

        let claim_offers_json = serde_json::to_string(&claim_offers)?;

        Ok(claim_offers_json)
    }

    fn create_master_secret(&self,
                            wallet_handle: i32,
                            master_secret_name: &str,
                            cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        cb(self._create_master_secret(wallet_handle, master_secret_name))
    }

    fn _create_master_secret(&self, wallet_handle: i32, master_secret_name: &str) -> Result<(), AnoncredsError> {
        let master_secret = self.crypto_service.anoncreds.prover.generate_master_secret()?;
        self.wallet_service.set(wallet_handle, &format!("master_secret::{}", master_secret_name), &master_secret.to_dec()?)?;
        Ok(())
    }

    fn create_and_store_claim_request(&self,
                                      wallet_handle: i32,
                                      prover_did: &str,
                                      claim_offer_json: &str,
                                      claim_def_json: &str,
                                      master_secret_name: &str,
                                      cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(self._create_and_store_claim_request(wallet_handle, prover_did, claim_offer_json,
                                                claim_def_json, master_secret_name))
    }

    fn _create_and_store_claim_request(&self, wallet_handle: i32,
                                       prover_did: &str,
                                       claim_offer_json: &str,
                                       claim_def_json: &str,
                                       master_secret_name: &str) -> Result<String, AnoncredsError> {
        let master_secret_str = self.wallet_service.get(wallet_handle, &format!("master_secret::{}", &master_secret_name))?;
        let master_secret = BigNumber::from_dec(&master_secret_str)?;
        let claim_def = ClaimDefinition::from_json(&claim_def_json)?;
        let claim_offer = ClaimOffer::from_json(&claim_offer_json)?;

        let (claim_request,
            primary_claim_init_data,
            revocation_claim_init_data) = self.crypto_service.anoncreds.prover.
            create_claim_request(claim_def.public_key,
                                 claim_def.public_key_revocation,
                                 master_secret, prover_did)?;

        let primary_claim_init_data_json = ClaimInitData::to_json(&primary_claim_init_data)?;
        self.wallet_service.set(
            wallet_handle,
            &format!("primary_claim_init_data::{}", &claim_offer.claim_def_seq_no),
            &primary_claim_init_data_json)?;

        if let Some(data) = revocation_claim_init_data {
            let revocation_claim_init_data_json = RevocationClaimInitData::to_json(&data)?;
            self.wallet_service.set(
                wallet_handle,
                &format!("revocation_claim_init_data::{}", &claim_offer.claim_def_seq_no),
                &revocation_claim_init_data_json)?;
        }

        let claim_request = ClaimRequestJson::new(claim_request, claim_offer.issuer_did, claim_offer.claim_def_seq_no);
        let claim_request_json = ClaimRequestJson::to_json(&claim_request)?;

        Ok(claim_request_json)
    }

    fn store_claim(&self,
                   wallet_handle: i32,
                   claims_json: &str,
                   cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        cb(self._store_claim(wallet_handle, claims_json));
    }

    fn _store_claim(&self, wallet_handle: i32, claims_json: &str) -> Result<(), AnoncredsError> {
        let claim_json = ClaimJson::from_json(&claims_json)?;

        let revocation_registry_uuid = self.wallet_service.get(
            wallet_handle,
            &format!("revocation_registry_uuid::{}", &claim_json.revoc_reg_seq_no))?;
        let revocation_registry_json = self.wallet_service.get(
            wallet_handle,
            &format!("revocation_registry::{}", &revocation_registry_uuid))?;
        let revocation_registry = RevocationRegistry::from_json(&revocation_registry_json)?;

        let primary_claim_init_data_json = self.wallet_service.get(
            wallet_handle,
            &format!("primary_claim_init_data::{}", &claim_json.claim_def_seq_no))?;
        let primary_claim_init_data = ClaimInitData::from_json(&primary_claim_init_data_json)?;

        let revocation_claim_init_data_json = self.wallet_service.get(
            wallet_handle,
            &format!("revocation_claim_init_data::{}", &claim_json.claim_def_seq_no))?;
        let revocation_claim_init_data = RevocationClaimInitData::from_json(&revocation_claim_init_data_json)?;

        let claim_def_uuid = self.wallet_service.get(
            wallet_handle,
            &format!("claim_definition_uuid::{}", &claim_json.claim_def_seq_no))?;
        let claim_def_json = self.wallet_service.get(
            wallet_handle, &format!("claim_definition::{}", &claim_def_uuid))?;
        let claim_def = ClaimDefinition::from_json(&claim_def_json)?;

        let claim_json = RefCell::new(claim_json);

        self.crypto_service.anoncreds.prover.process_claim(
            &claim_json, primary_claim_init_data, Some(revocation_claim_init_data),
            claim_def.public_key_revocation, Some(revocation_registry))?;

        let claim = ClaimJson::to_json(&claim_json.borrow())?;

        let uuid = Uuid::new_v4().to_string();

        self.wallet_service.set(wallet_handle, &format!("claim::{}", &uuid), &claim)?;

        Ok(())
    }

    fn get_claims(&self,
                  wallet_handle: i32,
                  filter_json: &str,
                  cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn _get_claims(&self,
                   wallet_handle: i32,
                   filter_json: &str) -> Result<String, AnoncredsError> {
        let claims: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("claim::"))?;
        let mut claims_info: Vec<ClaimInfo> = ProverCommandExecutor::get_all_claims(claims)?;

        let filter = ClaimInfoFilter::from_json(filter_json)?;

        claims_info.retain(move |claim_info| {
            let mut condition = true;

            if let Some(schema_seq_no) = filter.schema_seq_no {
                condition = claim_info.schema_seq_no == schema_seq_no;
            }

            if let Some(_) = filter.issuer_did {}//TODO Claim info does not contain issuer_did

            if let Some(claim_def_seq_no) = filter.claim_def_seq_no {
                condition = claim_info.claim_def_seq_no == claim_def_seq_no;
            }
            condition
        });

        let claims_info_json = serde_json::to_string(&claims_info)?;

        Ok(claims_info_json)
    }

    fn get_all_claims(claims: Vec<(String, String)>) -> Result<Vec<ClaimInfo>, AnoncredsError> {
        let mut claims_info: Vec<ClaimInfo> = Vec::new();

        for &(ref uuid, ref claim) in claims.iter() {
            let claim_json: ClaimJson = ClaimJson::from_json(claim)?;

            let mut attrs: HashMap<String, String> = HashMap::new();

            for (attr, values) in claim_json.claim {
                attrs.insert(attr.clone(), values[1].clone());
            }

            claims_info.push(ClaimInfo::new(uuid.clone(), attrs, claim_json.claim_def_seq_no.clone(),
                                            claim_json.revoc_reg_seq_no.clone(), claim_json.schema_seq_no.clone()));
        }

        Ok(claims_info)
    }

    fn get_claims_for_proof_req(&self,
                                wallet_handle: i32,
                                proof_req_json: &str,
                                cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        let result = self._get_claims_for_proof_req(wallet_handle, proof_req_json);
        cb(result)
    }

    fn _get_claims_for_proof_req(&self,
                                 wallet_handle: i32,
                                 proof_req_json: &str, ) -> Result<String, AnoncredsError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_json(proof_req_json)?;

        let claims: Vec<(String, String)> = self.wallet_service.list(wallet_handle, &format!("claim::"))?;
        let claims_info: Vec<ClaimInfo> = ProverCommandExecutor::get_all_claims(claims)?;

        let (attributes, predicates) =
            self.crypto_service.anoncreds.prover.find_claims(
                proof_req.requested_attrs, proof_req.requested_predicates, claims_info)?;

        let proof_claims = ProofClaimsJson::new(attributes, predicates);

        let proof_claims_json = ProofClaimsJson::to_json(&proof_claims)?;

        Ok(proof_claims_json)
    }
    fn create_proof(&self,
                    wallet_handle: i32,
                    proof_req_json: &str,
                    requested_claims_json: &str,
                    schemas_jsons: &str,
                    claim_def_jsons: &str,
                    revoc_regs_jsons: &str,
                    cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        let result = self._create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_jsons, claim_def_jsons, revoc_regs_jsons);
        cb(result)
    }

    fn _create_proof(&self,
                     wallet_handle: i32,
                     proof_req_json: &str,
                     requested_claims_json: &str,
                     schemas_jsons: &str,
                     claim_def_jsons: &str,
                     revoc_regs_jsons: &str) -> Result<String, AnoncredsError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_json(proof_req_json)?;
        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_jsons)?;
        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_def_jsons)?;
        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_jsons)?;
        let requested_claims: RequestedClaimsJson = RequestedClaimsJson::from_json(requested_claims_json)?;

        let mut claims: HashMap<String, ClaimJson> = HashMap::new();

        for claim_uuid in claim_defs.keys() {
            let claim_json = self.wallet_service.get(wallet_handle, &format!("claim::{}", &claim_uuid))?;
            let claim = ClaimJson::from_json(&claim_json)?;
            claims.insert(claim_uuid.clone(), claim);
        }

        let ms = self.wallet_service.get(wallet_handle, &format!("master_secret"))?;
        let ms: BigNumber = serde_json::from_str(&ms)?;

        let tails = self.wallet_service.get(wallet_handle, &format!("tails"))?;
        let tails: HashMap<i32, PointG1> = serde_json::from_str(&tails)?;

        let proof_claims = self.crypto_service.anoncreds.prover.create_proof(claims,
                                                                             &proof_req,
                                                                             &schemas,
                                                                             &claim_defs,
                                                                             &revoc_regs,
                                                                             &requested_claims,
                                                                             &ms,
                                                                             &tails)?;

        let proof_claims_json = ProofJson::to_json(&proof_claims)?;

        Ok(proof_claims_json)
    }
}