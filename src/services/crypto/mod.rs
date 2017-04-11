pub mod helpers;
pub mod anoncreds;
pub mod ed25519;
pub mod wrappers;

use self::ed25519::Sodium;
use self::anoncreds::issuer::Issuer;
use self::anoncreds::prover::Prover;
use self::anoncreds::verifier::Verifier;

pub struct CryptoService {
    sodium: Sodium,
    issuer: Issuer,
    prover: Prover,
    verifier: Verifier
}

impl CryptoService {
    pub fn new() -> CryptoService {
        CryptoService {
            sodium: Sodium::new(),
            issuer: Issuer::new(),
            prover: Prover::new(),
            verifier: Verifier::new()
        }
    }
}