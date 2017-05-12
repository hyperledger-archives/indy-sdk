use super::CryptoType;
use utils::crypto::ed25519::ED25519;
use errors::crypto::CryptoError;


pub struct ED25519Signus {}

impl ED25519Signus {
    pub fn new() -> ED25519Signus {
        ED25519Signus {}
    }
}

impl CryptoType for ED25519Signus {
    fn create_key_pair(&self) -> (Vec<u8>, Vec<u8>) {
        ED25519::create_key_pair()
    }

    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Vec<u8> {
        ED25519::encrypt(private_key, public_key, doc, nonce)
    }

    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        ED25519::decrypt(private_key, public_key, doc, nonce)
    }

    fn gen_nonce(&self) -> Vec<u8> {
        ED25519::gen_nonce()
    }

    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> (Vec<u8>, Vec<u8>) {
        ED25519::create_key_pair_for_signature(seed)
    }

    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Vec<u8> {
        ED25519::sign(private_key, doc)
    }

    fn verify(&self, public_key: &[u8], doc: &[u8], signature: &[u8]) -> bool {
        ED25519::verify(public_key, doc, signature)
    }
}