extern crate serde_json;

use errors::anoncreds::AnoncredsError;
use errors::crypto::CryptoError;
use errors::wallet::WalletError;

use self::serde_json::Value;
use services::crypto::CryptoService;
use services::crypto::anoncreds::types::{
    ClaimRequest,
    PublicKey,
    RevocationPublicKey
};
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::wrappers::bn::BigNumber;
use std::rc::Rc;
use types::claim_offer::ClaimOffer;
use types::claim_definition::ClaimDefinition;

use utils::json::{JsonEncodable, JsonDecodable};

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
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let claim_offer: ClaimOffer = ClaimOffer::decode(&claim_offer_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    wallet.set(&format!("claim_offer {}", &claim_offer.issuer_did), claim_offer_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    Ok(())
                });

        match result {
            Ok(()) => cb(Ok(())),
            Err(err) => cb(Err(err))
        }
    }

    fn get_claim_offers(&self,
                        wallet_handle: i32,
                        filter_json: &str,
                        cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let filter: Value = serde_json::from_str(&filter_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let claim_offers = wallet.get(&format!("claim_offer {}", &filter["issuer_did"])) //TODO LIST METHOD
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    Ok((claim_offers))
                });

        match result {
            Ok(claim_offers) => cb(Ok(claim_offers)),
            Err(err) => cb(Err(err))
        }
    }

    fn create_master_secret(&self,
                            wallet_handle: i32,
                            master_secret_name: &str,
                            cb: Box<Fn(Result<(), AnoncredsError>) + Send>) {
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let master_secret = self.crypto_service.anoncreds.prover.generate_master_secret()
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::BackendError(err.to_string())))?;

                    let master_secret_string = master_secret.to_dec()
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("master_secret {}", &master_secret_name), &master_secret_string)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    Ok(())
                });

        match result {
            Ok(()) => cb(Ok(())),
            Err(err) => cb(Err(err))
        }
    }

    fn create_and_store_claim_request(&self,
                                      wallet_handle: i32,
                                      claim_offer_json: &str,
                                      claim_def_json: &str,
                                      master_secret_name: &str,
                                      cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let claim_offer: ClaimOffer = ClaimOffer::decode(&claim_offer_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let claim_def_json = ClaimDefinition::decode(&claim_def_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let pk_string = wallet.get(&format!("public_key {}", &claim_offer.issuer_did))
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    let pkr_string = wallet.get(&format!("public_key_revocation {}", &claim_offer.issuer_did))
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    let ms_string = wallet.get(&format!("master_secret {}", master_secret_name))
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    let pk = PublicKey::decode(&pk_string)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let pkr = RevocationPublicKey::decode(&pkr_string)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let ms = BigNumber::from_dec(&ms_string)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let (claim_request, claim_init_data, revocation_claim_init_data) =
                        self.crypto_service.anoncreds.prover.create_claim_request(pk, pkr, ms, "1".to_string(), true)
                            .map_err(|err| AnoncredsError::CryptoError(CryptoError::BackendError(err.to_string())))?;

                    let claim_request_json = ClaimRequest::encode(&claim_request)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let claim_init_data_json = ClaimRequest::encode(&claim_request)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let revocation_claim_init_data_json = ClaimRequest::encode(&claim_request)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    Ok(claim_request_json)
                });

        match result {
            Ok(claim_request_json) => cb(Ok(claim_request_json)),
            Err(err) => cb(Err(err))
        }
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