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