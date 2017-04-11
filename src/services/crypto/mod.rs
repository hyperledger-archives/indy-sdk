pub mod helpers;
pub mod anoncreds;
pub mod ed25519;
pub mod wrappers;

use self::anoncreds::AnoncredsService;
use self::ed25519::Sodium;

pub struct CryptoService {
    sodium: Sodium,
    anoncreds: AnoncredsService
}

impl CryptoService {
    pub fn new() -> CryptoService {
        CryptoService {
            sodium: Sodium::new(),
            anoncreds: AnoncredsService::new()
        }
    }
}