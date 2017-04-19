pub mod constants;
pub mod issuer;
pub mod prover;
pub mod types;
pub mod verifier;

use services::crypto::anoncreds::issuer::Issuer;
use services::crypto::anoncreds::prover::Prover;
use services::crypto::anoncreds::verifier::Verifier;

pub struct Anoncreds {
    issuer: Issuer,
    prover: Prover,
    verifier: Verifier
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