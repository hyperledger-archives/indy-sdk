extern crate serde_json;

use errors::anoncreds::AnoncredsError;
use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;
use std::rc::Rc;

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
            },
            ProverCommand::GetClaimOffers(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimOffers command received");
                self.get_claim_offers(wallet_handle, &filter_json, cb);
            },
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateMasterSecret command received");
                self.create_master_secret(wallet_handle, &master_secret_name, cb);
            },
            ProverCommand::CreateAndStoreClaimRequest(wallet_handle, claim_offer_json,
                                                      claim_def_json, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateAndStoreClaimRequest command received");
                self.create_and_store_claim_request(wallet_handle, &claim_offer_json,
                                                    &claim_def_json, &master_secret_name, cb);
            },
            ProverCommand::StoreClaim(wallet_handle, claims_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaim command received");
                self.store_claim(wallet_handle, &claims_json, cb);
            },
            ProverCommand::GetClaims(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaims command received");
                self.get_claims(wallet_handle, &filter_json, cb);
            },
            ProverCommand::GetClaimsForProofReq(wallet_handle, proof_req_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimsForProofReq command received");
                self.get_claims_for_proof_req(wallet_handle, &proof_req_json, cb);
            },
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
        unimplemented!();
//        let result =
//            ClaimOffer::from_str(&claim_offer_json)
//                .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))
//                .and_then(|claim_offer| {
//                    self.wallet_service.set(wallet_handle, &format!("claim_offer {}", &claim_offer.issuer_did), claim_offer_json)?;
//
//                    Ok(())
//                });
//
//        match result {
//            Ok(()) => cb(Ok(())),
//            Err(err) => cb(Err(err))
//        }
    }

    fn get_claim_offers(&self,
                        wallet_handle: i32,
                        filter_json: &str,
                        cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
//        let result =
//            serde_json::from_str(&filter_json)
//                .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))
//                .and_then(|filter: Value| {
//                    let claim_offers =
//                        self.wallet_service.get(wallet_handle, &format!("claim_offer {}", &filter["issuer_did"]))?; //TODO LIST METHOD
//
//                    Ok((claim_offers))
//                });
//
//        match result {
//            Ok(claim_offers) => cb(Ok(claim_offers)),
//            Err(err) => cb(Err(err))
//        }
    }

    fn create_master_secret(&self,
                            wallet_handle: i32,
                            master_secret_name: &str,
                            cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        unimplemented!();
//        let result =
//            self.crypto_service.anoncreds.prover.generate_master_secret()
//                .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))
//                .and_then(|master_secret| {
//                    let master_secret_string = master_secret.to_dec()?;
//
//                    self.wallet_service.set(wallet_handle, &format!("master_secret {}", &master_secret_name), &master_secret_string)?;
//
//                    Ok(())
//                });
//
//        match result {
//            Ok(()) => cb(Ok(())),
//            Err(err) => cb(Err(err))
//        }
    }

    fn create_and_store_claim_request(&self,
                                      wallet_handle: i32,
                                      claim_offer_json: &str,
                                      claim_def_json: &str,
                                      master_secret_name: &str,
                                      cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
//        let result =
//            ClaimOffer::from_str(&claim_offer_json)
//                .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))
//                .and_then(|claim_offer| {
//                    let claim_def_json = ClaimDefinition::from_str(&claim_def_json)?;
//
//                    let pk_string = self.wallet_service.get(wallet_handle, &format!("public_key {}", &claim_offer.issuer_did))?;
//                    let pkr_string = self.wallet_service.get(wallet_handle, &format!("public_key_revocation {}", &claim_offer.issuer_did))?;
//                    let ms_string = self.wallet_service.get(wallet_handle,&format!("master_secret {}", master_secret_name))?;
//
//                    let pk = PublicKey::from_str(&pk_string)?;
//                    let pkr = RevocationPublicKey::from_str(&pkr_string)?;
//                    let ms = BigNumber::from_dec(&ms_string)?;
//
//                    let (claim_request, claim_init_data, revocation_claim_init_data) =
//                        self.crypto_service.anoncreds.prover.create_claim_request(pk, pkr, ms, "1".to_string(), true)?;
//
//                    let claim_request_json = ClaimRequest::to_string(&claim_request)?;
//                    let claim_init_data_json = ClaimRequest::to_string(&claim_request);
//                    let revocation_claim_init_data_json = ClaimRequest::to_string(&claim_request)?;
//
//                    Ok(claim_request_json)
//                });
//
//        match result {
//            Ok(claim_request_json) => cb(Ok(claim_request_json)),
//            Err(err) => cb(Err(err))
//        }
    }

    fn store_claim(&self,
                   wallet_handle: i32,
                   claims_json: &str,
                   cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        cb(Ok(()));
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
        cb(Ok("".to_string()));
    }

    fn parse_proof_request(&self,
                           wallet_handle: i32,
                           proof_request_json: &str,
                           cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(Ok("".to_string()));
    }

    fn create_proof(&self,
                    wallet_handle: i32,
                    proof_req_json: &str,
                    requested_claims_json: &str,
                    schemas_jsons: &str,
                    claim_def_jsons: &str,
                    revoc_regs_jsons: &str,
                    cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(Ok("".to_string()));
    }
}