mod ed25519;
pub mod types;

use self::ed25519::ED25519Signus;
use errors::crypto::CryptoError;
use std::collections::HashMap;

pub trait Signus {
    fn create_key_pair(&self) -> (Vec<u8>, Vec<u8>);
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8>;
    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn gen_nonce(&self) -> Vec<u8>;
    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>);
    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Vec<u8>;
    fn verify(&self, public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

pub struct SignusService {
    pub types: HashMap<&'static str, Box<Signus>>
}

impl SignusService {
    pub fn new() -> SignusService {
        let mut types: HashMap<&str, Box<Signus>> = HashMap::new();
        types.insert("ed25519", Box::new(ED25519Signus::new()));

        SignusService {
            types: types,
        }
    }
}