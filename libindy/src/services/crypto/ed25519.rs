use super::CryptoType;
use utils::crypto::box_::CryptoBox;
use utils::crypto::sealedbox::Sealbox;
use errors::common::CommonError;


pub struct ED25519CryptoType {}

impl ED25519CryptoType {
    pub fn new() -> ED25519CryptoType {
        ED25519CryptoType {}
    }
}

impl CryptoType for ED25519CryptoType {
    fn encrypt(&self, sk: &[u8], vk: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::encrypt(CryptoBox::sk_to_curve25519(sk)?.as_ref(),
                           &CryptoBox::vk_to_curve25519(vk)?.as_ref(), doc, nonce)
    }

    fn decrypt(&self, sk: &[u8], vk: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::decrypt(CryptoBox::sk_to_curve25519(sk)?.as_ref(),
                           CryptoBox::vk_to_curve25519(vk)?.as_ref(), doc, nonce)
    }

    fn gen_nonce(&self) -> Vec<u8> {
        CryptoBox::gen_nonce()
    }

    fn create_key(&self, seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError> {
        CryptoBox::create_key_pair_for_signature(seed)
    }

    fn sign(&self, sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        CryptoBox::sign(sk, doc)
    }

    fn verify(&self, vk: &[u8], doc: &[u8], signature: &[u8]) -> Result<bool, CommonError> {
        CryptoBox::verify(vk, doc, signature)
    }

    fn encrypt_sealed(&self, vk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        Sealbox::encrypt(CryptoBox::vk_to_curve25519(vk)?.as_ref(), doc)
    }

    fn decrypt_sealed(&self, vk: &[u8], sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        Sealbox::decrypt(CryptoBox::vk_to_curve25519(vk)?.as_ref(),
                         CryptoBox::sk_to_curve25519(sk)?.as_ref(), doc)
    }
    fn validate_key(&self, _vk: &[u8]) -> Result<(), CommonError> {
        // TODO: FIXME: Validate key
        Ok(())
    }
}