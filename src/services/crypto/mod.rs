pub mod helpers;
pub mod constants;
pub mod issuer;
pub mod libsodium;
pub mod prover;
pub mod types;
pub mod verifier;

use self::libsodium::Sodium;
use self::issuer::Issuer;
use self::prover::Prover;
use self::verifier::Verifier;

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