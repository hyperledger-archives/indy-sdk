pub mod constants;
pub mod helpers;
pub mod issuer;
pub mod prover;
pub mod types;
pub mod verifier;

use services::crypto::anoncreds::issuer::Issuer;
use services::crypto::anoncreds::prover::Prover;
use services::crypto::anoncreds::verifier::Verifier;

pub struct Anoncreds {
    pub issuer: Issuer,
    pub prover: Prover,
    pub verifier: Verifier
}

impl Anoncreds {
    pub fn new() -> Anoncreds {
        Anoncreds {
            issuer: Issuer::new(),
            prover: Prover::new(),
            verifier: Verifier::new()
        }
    }
}