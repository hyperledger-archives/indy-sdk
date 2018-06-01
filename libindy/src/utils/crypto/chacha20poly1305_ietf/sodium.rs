extern crate sodiumoxide;
use sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;

use errors::common::CommonError;
use utils::byte_array::_clone_into_array;
use utils::crypto::hmacsha256::{HMACSHA256, HMACSHA256Key};



// TODO - figure out the best approach
pub struct ChaCha20Poly1305IETFKey {
    key: chacha20poly1305_ietf::Key,
}

impl ChaCha20Poly1305IETFKey {
    pub fn get_bytes(&self) -> &[u8] {
        &self.key.0
    }
}


pub(super) type ChaCha20Poly1305IETFNonce = chacha20poly1305_ietf::Nonce;


pub struct ChaCha20Poly1305IETF {}

impl ChaCha20Poly1305IETF {
    pub const NONCEBYTES: usize = chacha20poly1305_ietf::NONCEBYTES;
    pub const KEYBYTES: usize = chacha20poly1305_ietf::KEYBYTES;
    pub const TAGBYTES: usize = chacha20poly1305_ietf::TAGBYTES;

    pub fn create_key(bytes: [u8; chacha20poly1305_ietf::KEYBYTES]) -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETFKey { key: chacha20poly1305_ietf::Key(bytes) }
    }

    pub fn clone_key_from_slice(bytes: &[u8]) -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETFKey { key: chacha20poly1305_ietf::Key(_clone_into_array(bytes)) }
    }

    pub fn generate_key() -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETFKey { key : chacha20poly1305_ietf::gen_key() }
    }

    #[allow(dead_code)]
    pub fn gen_nonce() -> ChaCha20Poly1305IETFNonce {
        chacha20poly1305_ietf::gen_nonce()
    }

    pub fn encrypt_as_searchable(data: &[u8], key: &ChaCha20Poly1305IETFKey, hmac_key: &HMACSHA256Key) -> Vec<u8> {
        let tag = HMACSHA256::create_tag(data, hmac_key);

        let ct = chacha20poly1305_ietf::seal(
            data,
            None,
            &chacha20poly1305_ietf::Nonce(_clone_into_array(&tag[..chacha20poly1305_ietf::NONCEBYTES])),
            &key.key
        );

        let mut result: Vec<u8> = Default::default();
        result.extend_from_slice(&tag[..chacha20poly1305_ietf::NONCEBYTES]);
        result.extend_from_slice(&ct);
        result
    }

    pub fn encrypt_as_not_searchable(data: &[u8], key: &ChaCha20Poly1305IETFKey) -> Vec<u8> {
        let nonce = chacha20poly1305_ietf::gen_nonce();
        let ct = chacha20poly1305_ietf::seal(
            data,
            None,
            &nonce,
            &key.key
        );

        let mut result: Vec<u8> = Default::default();
        result.extend_from_slice(&nonce.0);
        result.extend_from_slice(&ct);
        result
    }

    pub fn decrypt(enc_text: &[u8], key: &ChaCha20Poly1305IETFKey) -> Result<Vec<u8>, CommonError> {
        if enc_text.len() <= chacha20poly1305_ietf::NONCEBYTES {
            return Err(CommonError::InvalidStructure(format!("Unable to decrypt data: Cyphertext too short")));
        }

        chacha20poly1305_ietf::open(
            &enc_text[chacha20poly1305_ietf::NONCEBYTES..],
            None,
            &chacha20poly1305_ietf::Nonce(_clone_into_array(&enc_text[..chacha20poly1305_ietf::NONCEBYTES])),
            &key.key
        )
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_as_searchable_decrypt_works() {
        let data = randombytes::randombytes(100);
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();

        let c = ChaCha20Poly1305IETF::encrypt_as_searchable(&data, &key, &hmac_key);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key).unwrap();
        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_as_not_searchable_decrypt_works() {
        let data = randombytes::randombytes(16);
        let key = ChaCha20Poly1305IETF::generate_key();

        let c = ChaCha20Poly1305IETF::encrypt_as_not_searchable(&data, &key);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key).unwrap();
        assert_eq!(data, u);
    }
}