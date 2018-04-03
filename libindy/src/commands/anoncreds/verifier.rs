extern crate serde_json;
extern crate indy_crypto;

use errors::common::CommonError;
use errors::indy::IndyError;

use services::anoncreds::AnoncredsService;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use self::indy_crypto::utils::json::JsonDecodable;

use domain::schema::{Schema, schemas_map_to_schemas_v1_map};
use domain::credential_definition::{CredentialDefinition, cred_defs_map_to_cred_defs_v1_map};
use domain::proof::Proof;
use domain::proof_request::ProofRequest;
use domain::revocation_registry_definition::{RevocationRegistryDefinition, rev_reg_defs_map_to_rev_reg_defs_v1_map};
use domain::revocation_registry::{RevocationRegistry, rev_regs_map_to_rev_regs_local_map};

pub enum VerifierCommand {
    VerifyProof(
        String, // proof request json
        String, // proof json
        String, // credential schemas json
        String, // credential defs jsons
        String, // rev reg defs json
        String, // rev reg entries json
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
            VerifierCommand::VerifyProof(proof_request_json, proof_json, credential_schemas_json, credential_defs_json, rev_reg_defs_json, rev_reg_entries_json, cb) => {
                trace!(target: "verifier_command_executor", "VerifyProof command received");
                cb(self.verify_proof(&proof_request_json, &proof_json, &credential_schemas_json, &credential_defs_json, &rev_reg_defs_json, &rev_reg_entries_json));
            }
        };
    }

    fn verify_proof(&self,
                    proof_request_json: &str,
                    proof_json: &str,
                    schemas_json: &str,
                    cred_defs_json: &str,
                    rev_reg_defs_json: &str,
                    rev_reg_json: &str) -> Result<bool, IndyError> {
        trace!("verify_proof >>> proof_request_json: {:?}, proof_json: {:?}, schemas_json: {:?}, cred_defs_json: {:?},  \
               rev_reg_defs_json: {:?}, rev_reg_json: {:?}",
               proof_request_json, proof_json, schemas_json, cred_defs_json, rev_reg_defs_json, rev_reg_json);

        let proof_req: ProofRequest = ProofRequest::from_json(proof_request_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize ProofRequest: {:?}", err)))?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of Schema: {:?}", err)))?;

        let cred_defs: HashMap<String, CredentialDefinition> = serde_json::from_str(cred_defs_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of CredentialDefinition: {:?}", err)))?;

        let rev_reg_defs: HashMap<String, RevocationRegistryDefinition> = serde_json::from_str(rev_reg_defs_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of RevocationRegistryDef: {:?}", err)))?;

        let rev_regs: HashMap<String, HashMap<u64, RevocationRegistry>> = serde_json::from_str(rev_reg_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize list of RevocationRegistry: {:?}", err)))?;

        let proof: Proof = Proof::from_json(&proof_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Proof: {:?}", err)))?;

        let requested_attrs: HashSet<String> =
            proof_req.requested_attributes
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_revealed_attrs: HashSet<String> =
            proof.requested_proof.revealed_attrs
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_unrevealed_attrs: HashSet<String> =
            proof.requested_proof.unrevealed_attrs
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_self_attested_attrs: HashSet<String> =
            proof.requested_proof.self_attested_attrs
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_attrs = received_revealed_attrs
            .union(&received_unrevealed_attrs)
            .map(|attr| attr.clone())
            .collect::<HashSet<String>>()
            .union(&received_self_attested_attrs)
            .map(|attr| attr.clone())
            .collect::<HashSet<String>>();

        if requested_attrs != received_attrs {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested attributes {:?} do not correspond to received {:?}", requested_attrs, received_attrs))));
        }

        let requested_predicates: HashSet<String> =
            proof_req.requested_predicates
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        let received_predicates: HashSet<String> =
            proof.requested_proof.predicates
                .keys()
                .map(|referent| referent.clone())
                .into_iter()
                .collect::<HashSet<String>>();

        if requested_predicates != received_predicates {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Requested predicates {:?} do not correspond to received {:?}", requested_predicates, received_predicates))));
        }

        let result = self.anoncreds_service.verifier.verify(&proof,
                                                            &proof_req,
                                                            &schemas_map_to_schemas_v1_map(schemas),
                                                            &cred_defs_map_to_cred_defs_v1_map(cred_defs),
                                                            &rev_reg_defs_map_to_rev_reg_defs_v1_map(rev_reg_defs),
                                                            &rev_regs_map_to_rev_regs_local_map(rev_regs))?;

        trace!("verify_proof <<< result: {:?}", result);

        Ok(result)
    }
}