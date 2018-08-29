extern crate sodiumoxide;

use errors::common::CommonError;

use self::sodiumoxide::crypto::secretbox;
use utils::byte_array::_clone_into_array;

pub struct XSalsa20 {}

impl XSalsa20 {
    pub fn new() -> XSalsa20 {
        XSalsa20 {}
    }

    pub fn create_key(&self) -> Vec<u8> {
        secretbox::gen_key()[..].to_vec()
    }

    pub fn gen_nonce(&self) -> Vec<u8> {
        secretbox::gen_nonce()[..].to_vec()
    }

    pub fn encrypt(&self, key: &[u8], nonce: &[u8], doc: &[u8]) -> Vec<u8> {
        secretbox::seal(
            doc,
            &secretbox::Nonce(_clone_into_array(nonce)),
            &secretbox::Key(_clone_into_array(key))
        )
    }

    pub fn encrypt_detached(&self, key: &[u8], nonce: &[u8], doc: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut cipher = doc.to_vec();
        let tag = secretbox::seal_detached(cipher.as_mut_slice(),
                                 &secretbox::Nonce(_clone_into_array(nonce)),
                                 &secretbox::Key(_clone_into_array(key)));
        (cipher, tag[..].to_vec())
    }

    pub fn decrypt(&self, key: &[u8], nonce: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        secretbox::open(
            doc,
            &secretbox::Nonce(_clone_into_array(nonce)),
            &secretbox::Key(_clone_into_array(key))
        )
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }

    pub fn decrypt_detached(&self, key: &[u8], nonce: &[u8], tag: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        let mut plain = doc.to_vec();
        secretbox::open_detached(plain.as_mut_slice(),
                                 &secretbox::Tag(_clone_into_array(tag)),
                                 &secretbox::Nonce(_clone_into_array(nonce)),
                                 &secretbox::Key(_clone_into_array(key))).map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))?;
        Ok(plain.to_vec())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let xsalsa20 = XSalsa20::new();

        let nonce = xsalsa20.gen_nonce();
        let key = xsalsa20.create_key();
        let data = randombytes::randombytes(16);

        let encrypted_data = xsalsa20.encrypt(&key, &nonce, &data);
        let decrypt_result = xsalsa20.decrypt(&key, &nonce, &encrypted_data);

        assert!(decrypt_result.is_ok());
        assert_eq!(data, decrypt_result.unwrap());
    }

}
