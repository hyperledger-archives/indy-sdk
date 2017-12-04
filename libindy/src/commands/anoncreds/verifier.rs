extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;

use services::anoncreds::AnoncredsService;
use services::anoncreds::types::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

pub enum VerifierCommand {
    CreateNonce(
        Box<Fn(Result<String, IndyError>) + Send>),
    VerifyProof(
        String, // proof request json
        String, // proof json
        String, // schemas json
        String, // claim defs jsons
        String, // revoc regs json
        Box<Fn(Result<bool, IndyError>) + Send>)
}

pub struct VerifierCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
}

impl VerifierCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>) -> VerifierCommandExecutor {
        VerifierCommandExecutor {
            anoncreds_service,
        }
    }

    pub fn execute(&self, command: VerifierCommand) {
        match command {
            VerifierCommand::VerifyProof(proof_request_json,
                                         proof_json, schemas_json,
                                         claim_defs_jsons, revoc_regs_json, cb) => {
                info!(target: "verifier_command_executor", "VerifyProof command received");
                self.verify_proof(&proof_request_json, &proof_json, &schemas_json,
                                  &claim_defs_jsons, &revoc_regs_json, cb);
            }
            VerifierCommand::CreateNonce(cb) => {
                info!(target: "verifier_command_executor", "CreateNonce command received");
                self.create_nonce(cb);
            }
        };
    }

    fn create_nonce(&self,
                    cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let result = self._create_nonce();
        cb(result)
    }
    fn _create_nonce(&self) -> Result<String, IndyError> {
        let nonce = self.anoncreds_service.verifier.new_nonce()?;
        let nonce_json = nonce.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize nonce: {:?}", err)))?;

        Ok(nonce_json)
    }

    fn verify_proof(&self,
                    proof_request_json: &str,
                    proof_json: &str,
                    schemas_json: &str,
                    claim_defs_jsons: &str,
                    revoc_regs_json: &str,
                    cb: Box<Fn(Result<bool, IndyError>) + Send>) {
        let result = self._verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json);
        cb(result)
    }

    fn _verify_proof(&self,
                     proof_request_json: &str,
                     proof_json: &str,
                     schemas_json: &str,
                     claim_defs_jsons: &str,
                     revoc_regs_json: &str) -> Result<bool, IndyError> {
        let proof_req: ProofRequest = ProofRequest::from_json(proof_request_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize proof request: {:?}", err)))?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of schemas: {:?}", err)))?;

        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_defs_jsons)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of claim definitions: {:?}", err)))?;

        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(revoc_regs_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of revocation registries: {:?}", err)))?;

        let proof_claims: FullProof = FullProof::from_json(&proof_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize proof: {:?}", err)))?;

        if schemas.keys().collect::<HashSet<&String>>() != claim_defs.keys().collect::<HashSet<&String>>() {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Claim definition keys {:?} do not correspond to schema received {:?}", schemas.keys(), claim_defs.keys()))));
        }

        let requested_attrs: HashSet<String> =
            proof_req.requested_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_revealed_attrs: HashSet<String> =
            proof_claims.requested_proof.revealed_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_unrevealed_attrs: HashSet<String> =
            proof_claims.requested_proof.unrevealed_attrs
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_attrs = received_revealed_attrs
            .union(&received_unrevealed_attrs)
            .map(|attr| attr.clone())
            .collect::<HashSet<String>>();

        if requested_attrs != received_attrs {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested attributes {:?} do not correspond to received {:?}", requested_attrs, received_attrs))));
        }

        let requested_predicates: HashSet<String> =
            proof_req.requested_predicates
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_predicates: HashSet<String> =
            proof_claims.requested_proof.predicates
                .keys()
                .map(|uuid| uuid.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        if requested_predicates != received_predicates {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested predicates {:?} do not correspond to received {:?}", requested_predicates, received_predicates))));
        }

        let result = self.anoncreds_service.verifier.verify(&proof_claims,
                                                            &proof_req,
                                                            &claim_defs,
                                                            &revoc_regs,
                                                            &schemas)?;

        Ok(result)
    }
}