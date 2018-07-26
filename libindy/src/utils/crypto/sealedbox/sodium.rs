extern crate sodiumoxide;

use errors::common::CommonError;
use errors::crypto::CryptoError;

use utils::crypto::box_;
use self::sodiumoxide::crypto::sealedbox;

pub struct Sealbox {}

impl Sealbox {
    pub fn encrypt(pk: &box_::PublicKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(sealedbox::seal(doc, &pk.0))
    }

    pub fn decrypt(pk: &box_::PublicKey, sk: &box_::SecretKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sealedbox::open(&doc,
                        &pk.0,
                        &sk.0)
            .map_err(|err|
                CryptoError::CommonError(
                    CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::sodiumoxide::crypto::box_;
    use self::sodiumoxide::randombytes;
    use utils::crypto::box_::{PublicKey, SecretKey};

    #[test]
    fn encrypt_decrypt_works() {
        let (pk, sk) = box_::gen_keypair();
        let (pk, sk) = (PublicKey(pk), SecretKey(sk));
        let doc = randombytes::randombytes(16);

        let encrypted_data = Sealbox::encrypt(&pk, &doc).unwrap();
        let decrypt_result = Sealbox::decrypt(&pk, &sk, &encrypted_data).unwrap();

        assert_eq!(doc, decrypt_result);
    }
}