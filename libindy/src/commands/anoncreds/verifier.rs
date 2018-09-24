use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use domain::anoncreds::schema::{Schema, SchemaV1, schemas_map_to_schemas_v1_map};
use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1, cred_defs_map_to_cred_defs_v1_map};
use domain::anoncreds::proof::Proof;
use domain::anoncreds::proof_request::ProofRequest;
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1, rev_reg_defs_map_to_rev_reg_defs_v1_map};
use domain::anoncreds::revocation_registry::{RevocationRegistry, RevocationRegistryV1, rev_regs_map_to_rev_regs_local_map};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::anoncreds::AnoncredsService;

pub enum VerifierCommand {
    VerifyProof(
        ProofRequest, // proof request
        Proof, // proof
        HashMap<String, Schema>, // credential schemas
        HashMap<String, CredentialDefinition>, // credential defs
        HashMap<String, RevocationRegistryDefinition>, // rev reg defs
        HashMap<String, HashMap<u64, RevocationRegistry>>, // rev reg entries
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
            VerifierCommand::VerifyProof(proof_request, proof, schemas, credential_defs, rev_reg_defs, rev_regs, cb) => {
                info!(target: "verifier_command_executor", "VerifyProof command received");
                cb(self.verify_proof(proof_request, proof,
                                     &schemas_map_to_schemas_v1_map(schemas),
                                     &cred_defs_map_to_cred_defs_v1_map(credential_defs),
                                     &rev_reg_defs_map_to_rev_reg_defs_v1_map(rev_reg_defs),
                                     &rev_regs_map_to_rev_regs_local_map(rev_regs)));
            }
        };
    }

    fn verify_proof(&self,
                    proof_req: ProofRequest,
                    proof: Proof,
                    schemas: &HashMap<String, SchemaV1>,
                    cred_defs: &HashMap<String, CredentialDefinitionV1>,
                    rev_reg_defs: &HashMap<String, RevocationRegistryDefinitionV1>,
                    rev_regs: &HashMap<String, HashMap<u64, RevocationRegistryV1>>) -> Result<bool, IndyError> {
        debug!("verify_proof >>> proof_req: {:?}, proof: {:?}, schemas: {:?}, cred_defs: {:?},  \
               rev_reg_defs: {:?}, rev_regs: {:?}",
               proof_req, proof, schemas, cred_defs, rev_reg_defs, rev_regs);

        let requested_attrs: HashSet<String> =
            proof_req.requested_attributes
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        let received_revealed_attrs: HashSet<String> =
            proof.requested_proof.revealed_attrs
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        let received_unrevealed_attrs: HashSet<String> =
            proof.requested_proof.unrevealed_attrs
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        let received_self_attested_attrs: HashSet<String> =
            proof.requested_proof.self_attested_attrs
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        let received_attrs = received_revealed_attrs
            .union(&received_unrevealed_attrs)
            .cloned()
            .collect::<HashSet<String>>()
            .union(&received_self_attested_attrs)
            .cloned()
            .collect::<HashSet<String>>();

        if requested_attrs != received_attrs {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested attributes {:?} do not correspond to received {:?}", requested_attrs, received_attrs))));
        }

        let requested_predicates: HashSet<String> =
            proof_req.requested_predicates
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        let received_predicates: HashSet<String> =
            proof.requested_proof.predicates
                .keys()
                .cloned()
                .into_iter()
                .collect::<HashSet<String>>();

        if requested_predicates != received_predicates {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested predicates {:?} do not correspond to received {:?}", requested_predicates, received_predicates))));
        }

        let result = self.anoncreds_service.verifier.verify(&proof,
                                                            &proof_req,
                                                            schemas,
                                                            cred_defs,
                                                            rev_reg_defs,
                                                            rev_regs)?;

        debug!("verify_proof <<< result: {:?}", result);

        Ok(result)
    }
}