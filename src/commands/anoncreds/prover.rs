extern crate serde_json;
extern crate uuid;

use self::uuid::Uuid;
use errors::anoncreds::AnoncredsError;
use errors::crypto::CryptoError;
use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;
use std::rc::Rc;
use services::crypto::anoncreds::types::{
    ClaimDefinition, Schema, RevocationRegistry, Claims, CreateProofJson, NonRevocProofXList, NonRevocProofCList,
    ClaimInfo, ProofClaimsJson, ProofRequestJson, RequestedClaimsJson, ClaimJson};
use utils::json::{JsonDecodable, JsonEncodable};
use std::collections::HashMap;
use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::PointG1;

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
            ProverCommand::CreateAndStoreClaimRequest(wallet_handle, claim_offer_json,
                                                      claim_def_json, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateAndStoreClaimRequest command received");
                self.create_and_store_claim_request(wallet_handle, &claim_offer_json,
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
        unimplemented!();
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
                                      claim_offer_json: &str,
                                      claim_def_json: &str,
                                      master_secret_name: &str,
                                      cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
    }

    fn store_claim(&self,
                   wallet_handle: i32,
                   claims_json: &str,
                   cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        cb(self._store_claim(wallet_handle, claims_json));
    }

    fn _store_claim(&self, wallet_handle: i32, claims_json: &str) -> Result<(), AnoncredsError> {
        Ok(())
    }

    fn get_claims(&self,
                  wallet_handle: i32,
                  filter_json: &str,
                  cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(Ok("".to_string()));
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
        let proof_req: ProofRequestJson = ProofRequestJson::from_str(proof_req_json)?;

        //        let claims = wallet.list("claims")  TODO get all claims from wallet
        let claims: Vec<ClaimInfo> = Vec::new();

        let (attributes, predicates) =
            self.crypto_service.anoncreds.prover.find_claims(
                proof_req.requested_attrs, proof_req.requested_predicates, claims)?;

        let proof_claims = ProofClaimsJson::new(attributes, predicates);

        let proof_claims_json = ProofClaimsJson::to_string(&proof_claims)?;

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
        let proof_req: ProofRequestJson = ProofRequestJson::from_str(proof_req_json)?;
        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_jsons)?;
        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_def_jsons)?;
        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_jsons)?;
        let requested_claims: RequestedClaimsJson = RequestedClaimsJson::from_str(revoc_regs_jsons)?;

        let mut claims: HashMap<String, ClaimJson> = HashMap::new();

        for claim_uuid in claim_defs.keys() {
            let claim_json = self.wallet_service.get(wallet_handle, &format!("claim::{}", &claim_uuid))?;
            let claim = ClaimJson::from_str(&claim_json)?;
            claims.insert(claim_uuid.clone(), claim);
        }

        let ms = self.wallet_service.get(wallet_handle, &format!("master_secret"))?;
        let ms: BigNumber = serde_json::from_str(&ms)?;

        let tails = self.wallet_service.get(wallet_handle, &format!("tails"))?;
        let tails: HashMap<i32, PointG1> = serde_json::from_str(&tails)?;

        let proof_claims = self.crypto_service.anoncreds.prover.prepare_proof_claims(&proof_req,
                                                                                     &schemas,
                                                                                     &claim_defs,
                                                                                     &revoc_regs,
                                                                                     &requested_claims,
                                                                                     claims)?;

        let (proofs, attributes, aggregated_proof) =
            self.crypto_service.anoncreds.prover.create_proof(proof_claims,
                                                              &proof_req.nonce,
                                                              &ms,
                                                              &tails)?;

        let requested_proof = self.crypto_service.anoncreds.prover.prepare_requested_proof_response(&proof_req,
                                                                                                    &requested_claims,
                                                                                                    &attributes)?;
        let proof_claims_json = CreateProofJson {
            proofs: proofs,
            aggregated_proof: aggregated_proof,
            requested_proof: requested_proof
        };
        let proof_claims_json = CreateProofJson::to_string(&proof_claims_json)?;

        Ok(proof_claims_json)
    }
}