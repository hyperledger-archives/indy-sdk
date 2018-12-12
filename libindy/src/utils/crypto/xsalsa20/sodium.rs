extern crate sodiumoxide;

use errors::common::CommonError;

use self::sodiumoxide::crypto::secretbox;
use self::sodiumoxide::crypto::secretbox::xsalsa20poly1305;

pub const KEYBYTES: usize = xsalsa20poly1305::KEYBYTES;
pub const NONCEBYTES: usize = xsalsa20poly1305::NONCEBYTES;

sodium_type!(Key, xsalsa20poly1305::Key, KEYBYTES);
sodium_type!(Nonce, xsalsa20poly1305::Nonce, NONCEBYTES);

pub fn create_key() -> Key {
    Key(secretbox::gen_key())
}

pub fn gen_nonce() -> Nonce {
    Nonce(secretbox::gen_nonce())
}

pub fn encrypt(key: &Key, nonce: &Nonce, doc: &[u8]) -> Vec<u8> {
    secretbox::seal(
        doc,
        &nonce.0,
        &key.0
    )
}

pub fn decrypt(key: &Key, nonce: &Nonce, doc: &[u8]) -> Result<Vec<u8>, CommonError> {
    secretbox::open(
        doc,
        &nonce.0,
        &key.0
    )
        .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let nonce = gen_nonce();
        let key = create_key();
        let data = randombytes::randombytes(16);

        let encrypted_data = encrypt(&key, &nonce, &data);
        let decrypt_result = decrypt(&key, &nonce, &encrypted_data);

        assert!(decrypt_result.is_ok());
        assert_eq!(data, decrypt_result.unwrap());
    }
}