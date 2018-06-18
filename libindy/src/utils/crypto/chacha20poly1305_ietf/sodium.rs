extern crate sodiumoxide;
use sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;

use errors::common::CommonError;

use sodiumoxide::utils::increment_le;
use utils::byte_array::_clone_into_array;
use utils::crypto::hmacsha256::{HMACSHA256, HMACSHA256Key};

pub const NONCE_LENGTH: usize = chacha20poly1305_ietf::NONCEBYTES;
pub const KEY_LENGTH: usize = chacha20poly1305_ietf::KEYBYTES;
pub const TAG_LENGTH: usize = chacha20poly1305_ietf::TAGBYTES;


#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20Poly1305IETFKey {
    key: chacha20poly1305_ietf::Key,
}

impl ChaCha20Poly1305IETFKey {
    pub fn get_bytes(&self) -> &[u8] {
        &self.key.0
    }
}

#[derive(Debug, PartialEq)]
pub struct ChaCha20Poly1305IETFNonce {
    nonce: chacha20poly1305_ietf::Nonce
}

impl ChaCha20Poly1305IETFNonce {
    pub fn get_bytes(&self) -> &[u8] {
        &self.nonce.0
    }
}

pub struct ChaCha20Poly1305IETF {}

impl ChaCha20Poly1305IETF {
    pub fn clone_key_from_slice(bytes: &[u8]) -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETFKey { key: chacha20poly1305_ietf::Key(_clone_into_array(bytes)) }
    }

    pub fn generate_key() -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETFKey { key : chacha20poly1305_ietf::gen_key() }
    }

    #[allow(dead_code)]
    pub fn gen_nonce() -> ChaCha20Poly1305IETFNonce {
        ChaCha20Poly1305IETFNonce { nonce: chacha20poly1305_ietf::gen_nonce() }
    }

    pub fn clone_nonce_from_slice(bytes: &[u8]) -> ChaCha20Poly1305IETFNonce {
        ChaCha20Poly1305IETFNonce { nonce: chacha20poly1305_ietf::Nonce(_clone_into_array(&bytes[..chacha20poly1305_ietf::NONCEBYTES])) }
    }

    pub fn increment_nonce(mut nonce: &mut ChaCha20Poly1305IETFNonce) {
        increment_le(&mut nonce.nonce.0);
    }

    pub fn generate_nonce_and_encrypt(data: &[u8], key: &ChaCha20Poly1305IETFKey) -> (Vec<u8>, ChaCha20Poly1305IETFNonce) {
        let nonce = ChaCha20Poly1305IETF::gen_nonce();
        let encrypted_data = chacha20poly1305_ietf::seal(
            data,
            None,
            &nonce.nonce,
            &key.key
        );
        (encrypted_data, nonce)
    }

    pub fn encrypt(data: &[u8], key: &ChaCha20Poly1305IETFKey, nonce: &ChaCha20Poly1305IETFNonce) -> Vec<u8> {
        chacha20poly1305_ietf::seal(
            data,
            None,
            &nonce.nonce,
            &key.key,
        )
    }

    pub fn decrypt(data: &[u8], key: &ChaCha20Poly1305IETFKey, nonce: &ChaCha20Poly1305IETFNonce) -> Result<Vec<u8>, CommonError> {
        chacha20poly1305_ietf::open(
            &data,
            None,
            &nonce.nonce,
            &key.key,
        )
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let data = randombytes::randombytes(100);
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();

        let (c, nonce) = ChaCha20Poly1305IETF::generate_nonce_and_encrypt(&data, &key);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key, &nonce).unwrap();
        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_decrypt_works_for_nonce() {
        let data = randombytes::randombytes(16);
        let key = ChaCha20Poly1305IETF::generate_key();
        let nonce = ChaCha20Poly1305IETF::gen_nonce();
        let c = ChaCha20Poly1305IETF::encrypt(&data, &key, &nonce);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key, &nonce).unwrap();
        assert_eq!(data, u)
    }
}