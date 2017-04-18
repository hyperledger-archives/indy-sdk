pub mod helpers;
pub mod anoncreds;
pub mod wrappers;

use self::anoncreds::Anoncreds;
use self::wrappers::base58::Base58;
use self::wrappers::ed25519::ED25519;
use self::wrappers::xsalsa20::XSalsa20;

pub struct CryptoService {
    anoncreds: Anoncreds,
    base58: Base58,
    ed25519: ED25519,
    xsalsa20: XSalsa20
}

impl CryptoService {
    pub fn new() -> CryptoService {
        CryptoService {
            anoncreds: Anoncreds::new(),
            base58: Base58::new(),
            ed25519: ED25519::new(),
            xsalsa20: XSalsa20::new()
        }
    }
}