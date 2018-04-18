extern crate sodiumoxide;

use errors::common::CommonError;

use self::sodiumoxide::crypto::box_;
use self::sodiumoxide::crypto::sealedbox;
use utils::byte_array::_clone_into_array;

pub struct Sealbox {}

impl Sealbox {
    pub fn encrypt(pk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        Ok(sealedbox::seal(doc,
                           &box_::PublicKey(_clone_into_array(pk))))
    }

    pub fn decrypt(pk: &[u8], sk: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        sealedbox::open(&doc,
                        &box_::PublicKey(_clone_into_array(pk)),
                        &box_::SecretKey(_clone_into_array(sk)))
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let (pk, sk) = box_::gen_keypair();
        let (pk, sk) = (pk[..].to_vec(), sk[..].to_vec());

        let doc = randombytes::randombytes(16);

        let encrypted_data = Sealbox::encrypt(&pk, &doc).unwrap();
        let decrypt_result = Sealbox::decrypt(&pk, &sk, &encrypted_data).unwrap();

        assert_eq!(doc, decrypt_result);
    }
}