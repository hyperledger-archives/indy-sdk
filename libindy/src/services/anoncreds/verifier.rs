extern crate indy_crypto;

use services::anoncreds::types::*;
use std::collections::HashMap;
use errors::common::CommonError;
use self::indy_crypto::cl::verifier;
use services::anoncreds::helpers::*;

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {}
    }

    pub fn verify(&self,
                  full_proof: &FullProof,
                  proof_req: &ProofRequest,
                  claim_defs: &HashMap<String, ClaimDefinition>,
                  revoc_regs: &HashMap<String, RevocationRegistry>,
                  schemas: &HashMap<String, Schema>) -> Result<bool, CommonError> {
        info!(target: "services/anoncreds/verifier", "verify >>> full_proof: {:?}, proof_req: {:?}, claim_defs: {:?}, revoc_regs: {:?}, schemas: {:?}",
              full_proof, proof_req, claim_defs, revoc_regs, schemas);

        let mut proof_verifier = verifier::Verifier::new_proof_verifier()?;

        for (claim_uuid, claim_definition) in claim_defs {
            let schema = schemas.get(claim_uuid.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let revocation_registry = revoc_regs.get(claim_uuid.as_str());

            let attrs_for_claim = Verifier::_get_revealed_attributes_for_claim(claim_uuid.as_str(), &full_proof.requested_proof, proof_req)?;
            let predicates_for_claim = Verifier::_get_predicates_for_claim(claim_uuid.as_str(), &full_proof.requested_proof, proof_req)?;

            let claim_schema = build_claim_schema(&schema.data.attr_names)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_claim, &predicates_for_claim)?;

            proof_verifier.add_sub_proof_request(claim_uuid.as_str(),
                                                 &sub_proof_request,
                                                 &claim_schema,
                                                 &claim_definition.data,
                                                 revocation_registry.map(|rev_reg| &rev_reg.data))?;
        }

        let valid = proof_verifier.verify(&full_proof.proof, &proof_req.nonce)?;

        info!(target: "services/anoncreds/verifier", "verify <<< valid: {:?}", valid);

        Ok(valid)
    }

    fn _get_revealed_attributes_for_claim(claim_uuid: &str, requested_proof: &RequestedProof, proof_req: &ProofRequest) -> Result<Vec<String>, CommonError> {
        info!(target: "services/anoncreds/verifier", "_get_revealed_attributes_for_claim >>> claim_uuid: {:?}, requested_claims: {:?}, proof_req: {:?}",
              claim_uuid, requested_proof, proof_req);

        let mut revealed_attrs_for_claim: Vec<String> = Vec::new();

        for (attr_uuid, &(ref requested_claim_uuid, _, _)) in &requested_proof.revealed_attrs {
            if claim_uuid.eq(requested_claim_uuid.as_str()) {
                if let Some(attr) = proof_req.requested_attrs.get(attr_uuid) {
                    revealed_attrs_for_claim.push(attr.name.clone());
                }
            }
        }

        info!(target: "services/anoncreds/verifier", "_get_revealed_attributes_for_claim <<< revealed_attrs_for_claim: {:?}", revealed_attrs_for_claim);

        Ok(revealed_attrs_for_claim)
    }

    fn _get_predicates_for_claim(claim_uuid: &str, requested_proof: &RequestedProof, proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!(target: "services/anoncreds/verifier", "_get_predicates_for_claim >>> claim_uuid: {:?}, requested_claims: {:?}, proof_req: {:?}",
              claim_uuid, requested_proof, proof_req);

        let mut predicates_for_claim: Vec<PredicateInfo> = Vec::new();

        for (predicate_uuid, requested_claim_uuid) in &requested_proof.predicates {
            if claim_uuid.eq(requested_claim_uuid) {
                if let Some(predicate) = proof_req.requested_predicates.get(predicate_uuid) {
                    predicates_for_claim.push(predicate.clone());
                }
            }
        }

        info!(target: "services/anoncreds/verifier", "_get_predicates_for_claim <<< predicates_for_claim: {:?}", predicates_for_claim);

        Ok(predicates_for_claim)
    }
}