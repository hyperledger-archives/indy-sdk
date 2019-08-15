use std::collections::HashMap;
use std::rc::Rc;

use domain::anoncreds::credential_definition::{cred_defs_map_to_cred_defs_v1_map, CredentialDefinition, CredentialDefinitionV1};
use domain::anoncreds::proof::Proof;
use domain::anoncreds::proof_request::ProofRequest;
use domain::anoncreds::revocation_registry::{rev_regs_map_to_rev_regs_local_map, RevocationRegistry, RevocationRegistryV1};
use domain::anoncreds::revocation_registry_definition::{rev_reg_defs_map_to_rev_reg_defs_v1_map, RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::schema::{Schema, schemas_map_to_schemas_v1_map, SchemaV1};
use errors::prelude::*;
use services::anoncreds::AnoncredsService;

pub enum VerifierCommand {
    VerifyProof(
        ProofRequest, // proof request
        Proof, // proof
        HashMap<String, Schema>, // credential schemas
        HashMap<String, CredentialDefinition>, // credential defs
        HashMap<String, RevocationRegistryDefinition>, // rev reg defs
        HashMap<String, HashMap<u64, RevocationRegistry>>, // rev reg entries
        Box<dyn Fn(IndyResult<bool>) + Send>),
    GenerateNonce(
        Box<dyn Fn(IndyResult<String>) + Send>)
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
            VerifierCommand::VerifyProof(proof_request, proof, schemas, credential_defs, rev_reg_defs, rev_regs, cb) => {
                info!(target: "verifier_command_executor", "VerifyProof command received");
                cb(self.verify_proof(proof_request, proof,
                                     &schemas_map_to_schemas_v1_map(schemas),
                                     &cred_defs_map_to_cred_defs_v1_map(credential_defs),
                                     &rev_reg_defs_map_to_rev_reg_defs_v1_map(rev_reg_defs),
                                     &rev_regs_map_to_rev_regs_local_map(rev_regs)));
            }
            VerifierCommand::GenerateNonce(cb) => {
                info!(target: "verifier_command_executor", "GenerateNonce command received");
                cb(self.generate_nonce());
            }
        };
    }

    fn verify_proof(&self,
                    proof_req: ProofRequest,
                    proof: Proof,
                    schemas: &HashMap<String, SchemaV1>,
                    cred_defs: &HashMap<String, CredentialDefinitionV1>,
                    rev_reg_defs: &HashMap<String, RevocationRegistryDefinitionV1>,
                    rev_regs: &HashMap<String, HashMap<u64, RevocationRegistryV1>>) -> IndyResult<bool> {
        debug!("verify_proof >>> proof_req: {:?}, proof: {:?}, schemas: {:?}, cred_defs: {:?},  \
               rev_reg_defs: {:?}, rev_regs: {:?}",
               proof_req, proof, schemas, cred_defs, rev_reg_defs, rev_regs);

        let result = self.anoncreds_service.verifier.verify(&proof,
                                                            &proof_req,
                                                            schemas,
                                                            cred_defs,
                                                            rev_reg_defs,
                                                            rev_regs)?;

        debug!("verify_proof <<< result: {:?}", result);

        Ok(result)
    }

    fn generate_nonce(&self) -> IndyResult<String> {
        debug!("generate_nonce >>> ");

        let nonce = self.anoncreds_service.verifier.generate_nonce()?;

        let result = nonce.to_dec()
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize Nonce")?;

        debug!("generate_nonce <<< result: {:?}", result);

        Ok(result)
    }
}