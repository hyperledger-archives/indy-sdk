pub mod constants;
pub mod helpers;
pub mod issuer;
pub mod prover;
pub mod types;
pub mod verifier;

use services::anoncreds::issuer::Issuer;
use services::anoncreds::prover::Prover;
use services::anoncreds::verifier::Verifier;

pub struct AnoncredsService {
    pub issuer: Issuer,
    pub prover: Prover,
    pub verifier: Verifier
}

impl AnoncredsService {
    pub fn new() -> AnoncredsService {
        AnoncredsService {
            issuer: Issuer::new(),
            prover: Prover::new(),
            verifier: Verifier::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_init_non_revoc_claim() {
        let issuer = Issuer::new();
        let prover = Prover::new();

        let (claim_definition, claim_definition_private) = issuer.generate_claim_definition(
            issuer::mocks::ISSUER_DID, issuer::mocks::get_gvt_schema(), None, true).unwrap();

        let (revocation_registry, revocation_registry_private) = issuer.issue_accumulator(
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap(),
            5, issuer::mocks::ISSUER_DID, 1).unwrap();

        let master_secret = prover.generate_master_secret().unwrap();

        let (claim_request, claim_init_data, revocation_claim_init_data) = prover.create_claim_request(
            claim_definition.clone().unwrap().data.public_key,
            claim_definition.clone().unwrap().data.public_key_revocation,
            master_secret, prover::mocks::PROVER_DID).unwrap();

        let revocation_registry_ref_cell = RefCell::new(revocation_registry.clone());

        let claim_signature = issuer.create_claim(
            &claim_definition, &claim_definition_private, &Some(revocation_registry_ref_cell),
            &Some(revocation_registry_private), &claim_request,
            &issuer::mocks::get_gvt_attributes(), None).unwrap();

        let non_revocation_claim = claim_signature.clone().unwrap().non_revocation_claim.unwrap();
        let old_v = non_revocation_claim.borrow().vr_prime_prime;

        let claim_json = types::ClaimJson::new(
            issuer::mocks::get_gvt_attributes(), claim_signature, 1,
            issuer::mocks::ISSUER_DID.to_string());

        let claim_json_ref_cell = RefCell::new(claim_json.clone().unwrap());

        prover.process_claim(&claim_json_ref_cell, claim_init_data,
                             revocation_claim_init_data.clone(),
                             Some(claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap()),
                             Some(revocation_registry));

        let non_revocation_claim = claim_json_ref_cell.borrow().clone().unwrap().signature.non_revocation_claim.unwrap();
        let new_v = non_revocation_claim.borrow().vr_prime_prime;

        let vr_prime = revocation_claim_init_data.unwrap().v_prime;
        assert_eq!(old_v.add_mod(&vr_prime).unwrap(), new_v);
    }
}