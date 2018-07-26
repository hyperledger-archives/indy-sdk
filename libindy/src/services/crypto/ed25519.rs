use super::CryptoType;
use utils::crypto::sign;
use utils::crypto::sign::CryptoSign;
use utils::crypto::box_;
use utils::crypto::box_::CryptoBox;
use utils::crypto::sealedbox::Sealbox;
use errors::crypto::CryptoError;


pub struct ED25519CryptoType {}

impl ED25519CryptoType {
    pub fn new() -> ED25519CryptoType {
        ED25519CryptoType {}
    }
}

impl CryptoType for ED25519CryptoType {
    fn encrypt(&self, sk: &sign::SecretKey, vk: &sign::PublicKey, doc: &[u8], nonce: &box_::Nonce) -> Result<Vec<u8>, CryptoError> {
        CryptoBox::encrypt(&CryptoSign::sk_to_curve25519(sk)?,
                           &CryptoSign::vk_to_curve25519(vk)?, doc, nonce)
    }

    fn decrypt(&self, sk: &sign::SecretKey, vk: &sign::PublicKey, doc: &[u8], nonce: &box_::Nonce) -> Result<Vec<u8>, CryptoError> {
        CryptoBox::decrypt(&CryptoSign::sk_to_curve25519(sk)?,
                           &CryptoSign::vk_to_curve25519(vk)?, doc, nonce)
    }

    fn gen_nonce(&self) -> box_::Nonce {
        CryptoBox::gen_nonce()
    }

    fn create_key(&self, seed: Option<&sign::Seed>) -> Result<(sign::PublicKey, sign::SecretKey), CryptoError> {
        CryptoSign::create_key_pair_for_signature(seed)
    }

    fn sign(&self, sk: &sign::SecretKey, doc: &[u8]) -> Result<sign::Signature, CryptoError> {
        CryptoSign::sign(sk, doc)
    }

    fn verify(&self, vk: &sign::PublicKey, doc: &[u8], signature: &sign::Signature) -> Result<bool, CryptoError> {
        CryptoSign::verify(vk, doc, signature)
    }

    fn encrypt_sealed(&self, vk: &sign::PublicKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Sealbox::encrypt(&CryptoSign::vk_to_curve25519(vk)?, doc)
    }

    fn decrypt_sealed(&self, vk: &sign::PublicKey, sk: &sign::SecretKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Sealbox::decrypt(&CryptoSign::vk_to_curve25519(vk)?,
                         &CryptoSign::sk_to_curve25519(sk)?, doc)
    }
    fn validate_key(&self, _vk: &sign::PublicKey) -> Result<(), CryptoError> {
        // TODO: FIXME: Validate key
        Ok(())
    }
}