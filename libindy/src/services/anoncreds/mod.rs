pub mod helpers;
pub mod issuer;
pub mod prover;
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