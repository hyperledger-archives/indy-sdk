extern crate sodiumoxide;

use errors::crypto::CryptoError;

use self::sodiumoxide::crypto::secretbox;
use std::convert::AsMut;

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
        let sodium = XSalsa20::new();
        secretbox::seal(
            doc,
            &secretbox::Nonce(sodium.clone_into_array(nonce)),
            &secretbox::Key(sodium.clone_into_array(key))
        )
    }

    pub fn decrypt(&self, key: &[u8], nonce: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let sodium = XSalsa20::new();
        secretbox::open(
            doc,
            &secretbox::Nonce(sodium.clone_into_array(nonce)),
            &secretbox::Key(sodium.clone_into_array(key))
        )
            .map_err(|_| CryptoError::InvalidStructure("Unable to decrypt data".to_string()))
    }

    fn clone_into_array<A, T>(&self, slice: &[T]) -> A
        where A: Sized + Default + AsMut<[T]>, T: Clone
    {
        let mut a = Default::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
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