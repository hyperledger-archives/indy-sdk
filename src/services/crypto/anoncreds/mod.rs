pub mod constants;
pub mod helpers;
pub mod issuer;
pub mod prover;
pub mod types;
pub mod verifier;

use services::crypto::anoncreds::issuer::Issuer;
use services::crypto::anoncreds::prover::Prover;
use services::crypto::anoncreds::verifier::Verifier;

pub struct AnoncredsService {
    issuer: Issuer,
    prover: Prover,
    verifier: Verifier
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
    use std::rc::Rc;

    #[test]
    fn anoncreds_works() {
        let anoncreds_service = AnoncredsService::new();
        let schema = types::Schema {
            name: "GVT".to_string(),
            version: "1.0".to_string(),
            attribute_names: vec!["name".to_string(), "age".to_string(), "height".to_string(), "sex".to_string()]
        };
        let prover_id = "1".to_string();
        let accumulator_id = "110".to_string();
        let attributes = Rc::new(::services::crypto::anoncreds::issuer::mocks::get_attributes());

        let (pk, sk) = anoncreds_service.issuer.generate_keys(&schema).unwrap();
        let ms = anoncreds_service.prover.generate_master_secret().unwrap();

        let (claim_req, claim_init_data) = anoncreds_service.prover.create_claim_request(&pk, &ms, &prover_id).unwrap();
        let mut claims = anoncreds_service.issuer.issue_claim(&pk, &sk, &accumulator_id, &prover_id, Rc::new(claim_req), attributes).unwrap();
        let claims = anoncreds_service.prover.process_claim(&mut claims, &claim_init_data).unwrap();

        let proof_input = ::services::crypto::anoncreds::prover::mocks::get_proof_input();
        let all_claims = ::services::crypto::anoncreds::prover::mocks::get_all_claims().unwrap();
        let nonce = anoncreds_service.verifier.generate_nonce().unwrap();

        let (proof, revealed_attrs) = anoncreds_service.prover.present_proof(&proof_input, &all_claims, &nonce, &pk, &ms).unwrap();

        let result = anoncreds_service.verifier.verify(&pk, &proof_input, proof, &revealed_attrs, &nonce, &schema.attribute_names);

        assert!(result.is_ok());
    }
}