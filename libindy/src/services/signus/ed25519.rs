use super::CryptoType;
use utils::crypto::box_::CryptoBox;
use utils::crypto::sealedbox::Sealbox;
use errors::common::CommonError;


pub struct ED25519Signus {}

impl ED25519Signus {
    pub fn new() -> ED25519Signus {
        ED25519Signus {}
    }
}

impl CryptoType for ED25519Signus {
    fn encrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::encrypt(private_key, public_key, doc, nonce)
    }

    fn decrypt(&self, private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::decrypt(private_key, public_key, doc, nonce)
    }

    fn gen_nonce(&self) -> Vec<u8> {
        CryptoBox::gen_nonce()
    }

    fn create_key_pair_for_signature(&self, seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError> {
        CryptoBox::create_key_pair_for_signature(seed)
    }

    fn sign(&self, private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::sign(private_key, doc)
    }

    fn verify(&self, public_key: &[u8], doc: &[u8], signature: &[u8]) -> Result<bool, CommonError> {
        CryptoBox::verify(public_key, doc, signature)
    }

    fn verkey_to_public_key(&self, vk: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::vk_to_curve25519(vk)
    }

    fn signkey_to_private_key(&self, sk: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::sk_to_curve25519(sk)
    }
    fn encrypt_sealed(&self, public_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        Sealbox::encrypt(public_key, doc)
    }

    fn decrypt_sealed(&self, public_key: &[u8], private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        Sealbox::decrypt(public_key, private_key, doc)
    }
}