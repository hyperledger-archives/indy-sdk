extern crate serde_json;

use errors::anoncreds::AnoncredsError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::anoncreds::types::{
    ClaimDefinition,
    Schema,
    ProofRequestJson,
    ProofJson,
    RevocationRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use utils::json::JsonDecodable;

pub enum VerifierCommand {
    VerifyProof(
        i32, // wallet handle
        String, // proof request json
        String, // proof json
        String, // schemas json
        String, // claim defs jsons
        String, // revoc regs json
        Box<Fn(Result<bool, AnoncredsError>) + Send>)
}

pub struct VerifierCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>
}

impl VerifierCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> VerifierCommandExecutor {
        VerifierCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: VerifierCommand) {
        match command {
            VerifierCommand::VerifyProof(wallet_handle, proof_request_json,
                                         proof_json, schemas_json,
                                         claim_defs_jsons, revoc_regs_json, cb) => {
                info!(target: "verifier_command_executor", "VerifyProof command received");
                self.verify_proof(wallet_handle,
                                  &proof_request_json, &proof_json, &schemas_json,
                                  &claim_defs_jsons, &revoc_regs_json, cb);
            }
        };
    }

    fn verify_proof(&self, wallet_handle: i32,
                    proof_request_json: &str,
                    proof_json: &str,
                    schemas_json: &str,
                    claim_defs_jsons: &str,
                    revoc_regs_json: &str,
                    cb: Box<Fn(Result<bool, AnoncredsError>) + Send>) {
        let result = self._verify_proof(wallet_handle, proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json);
        cb(result)
    }

    fn _verify_proof(&self, wallet_handle: i32,
                     proof_request_json: &str,
                     proof_json: &str,
                     schemas_json: &str,
                     claim_defs_jsons: &str,
                     revoc_regs_json: &str) -> Result<bool, AnoncredsError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_str(proof_request_json)?;
        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_json)?;
        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_defs_jsons)?;
        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_json)?;
        let proof_claims_json = ProofJson::from_str(&proof_json)?;

        Ok(false)
    }
}