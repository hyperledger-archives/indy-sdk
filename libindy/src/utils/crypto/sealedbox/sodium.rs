extern crate sodiumoxide;

use errors::common::CommonError;
use errors::crypto::CryptoError;

use utils::crypto::ed25519_box;
use self::sodiumoxide::crypto::sealedbox;

pub fn encrypt(pk: &ed25519_box::PublicKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ok(sealedbox::seal(doc, &pk.0))
}

pub fn decrypt(pk: &ed25519_box::PublicKey, sk: &ed25519_box::SecretKey, doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sealedbox::open(&doc,
                    &pk.0,
                    &sk.0)
        .map_err(|err|
            CryptoError::CommonError(
                CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::sodiumoxide::crypto::box_;
    use utils::crypto::ed25519_box::{PublicKey, SecretKey};
    use utils::crypto::randombytes::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let (pk, sk) = box_::gen_keypair();
        let (pk, sk) = (PublicKey(pk), SecretKey(sk));
        let doc = randombytes(16);

        let encrypted_data = encrypt(&pk, &doc).unwrap();
        let decrypt_result = decrypt(&pk, &sk, &encrypted_data).unwrap();

        assert_eq!(doc, decrypt_result);
    }
}