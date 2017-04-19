pub mod anoncreds;
pub mod ed25519;
pub mod wrappers;

use self::anoncreds::AnoncredsService;
use self::ed25519::ED25519;

pub struct CryptoService {
    ed25519: ED25519,
    anoncreds: AnoncredsService
}

impl CryptoService {
    pub fn new() -> CryptoService {
        CryptoService {
            ed25519: ED25519::new(),
            anoncreds: AnoncredsService::new()
        }
    }
}