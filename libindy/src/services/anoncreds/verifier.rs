extern crate indy_crypto;

use services::anoncreds::types::*;
use std::collections::HashMap;
use errors::common::CommonError;
use self::indy_crypto::cl::IssuerPublicKey;
use self::indy_crypto::cl::verifier::Verifier as CryptoVerifier;
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
        info!("verify >>> full_proof: {:?}, proof_req: {:?}, claim_defs: {:?}, revoc_regs: {:?}, schemas: {:?}",
              full_proof, proof_req, claim_defs, revoc_regs, schemas);

        let mut proof_verifier = CryptoVerifier::new_proof_verifier()?;

        for (referent, claim_definition) in claim_defs {
            let schema = schemas.get(referent.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let revocation_registry = revoc_regs.get(referent.as_str());

            let attrs_for_claim = Verifier::_get_revealed_attributes_for_claim(referent.as_str(), &full_proof.requested_proof, proof_req)?;
            let predicates_for_claim = Verifier::_get_predicates_for_claim(referent.as_str(), &full_proof.requested_proof, proof_req)?;

            let claim_schema = build_claim_schema(&schema.data.attr_names)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_claim, &predicates_for_claim)?;

            let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_definition.data.primary, claim_definition.data.revocation.as_ref())?;

            proof_verifier.add_sub_proof_request(referent.as_str(),
                                                 &sub_proof_request,
                                                 &claim_schema,
                                                 &issuer_pub_key,
                                                 revocation_registry.map(|rev_reg| &rev_reg.data))?;
        }

        let valid = proof_verifier.verify(&full_proof.proof, &proof_req.nonce)?;

        info!("verify <<< valid: {:?}", valid);

        Ok(valid)
    }

    fn _get_revealed_attributes_for_claim(referent: &str, requested_proof: &RequestedProof, proof_req: &ProofRequest) -> Result<Vec<String>, CommonError> {
        info!("_get_revealed_attributes_for_claim >>> referent: {:?}, requested_claims: {:?}, proof_req: {:?}",
              referent, requested_proof, proof_req);

        let mut revealed_attrs_for_claim: Vec<String> = Vec::new();

        for (attr_referent, &(ref requested_referent, _, _)) in &requested_proof.revealed_attrs {
            if referent.eq(requested_referent.as_str()) {
                if let Some(attr) = proof_req.requested_attrs.get(attr_referent) {
                    revealed_attrs_for_claim.push(attr.name.clone());
                }
            }
        }

        info!("_get_revealed_attributes_for_claim <<< revealed_attrs_for_claim: {:?}", revealed_attrs_for_claim);

        Ok(revealed_attrs_for_claim)
    }

    fn _get_predicates_for_claim(referent: &str, requested_proof: &RequestedProof, proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!("_get_predicates_for_claim >>> referent: {:?}, requested_claims: {:?}, proof_req: {:?}",
              referent, requested_proof, proof_req);

        let mut predicates_for_claim: Vec<PredicateInfo> = Vec::new();

        for (predicate_referent, requested_referent) in &requested_proof.predicates {
            if referent.eq(requested_referent) {
                if let Some(predicate) = proof_req.requested_predicates.get(predicate_referent) {
                    predicates_for_claim.push(predicate.clone());
                }
            }
        }

        info!("_get_predicates_for_claim <<< predicates_for_claim: {:?}", predicates_for_claim);

        Ok(predicates_for_claim)
    }
}