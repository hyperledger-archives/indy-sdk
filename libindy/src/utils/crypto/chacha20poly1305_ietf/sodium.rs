extern crate sodiumoxide;

use errors::common::CommonError;

use self::sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;
use utils::byte_array::_clone_into_array;

pub struct ChaCha20Poly1305IETF {
}

impl ChaCha20Poly1305IETF {
    pub const NONCEBYTES: usize = chacha20poly1305_ietf::NONCEBYTES;
    pub const KEYBYTES: usize = chacha20poly1305_ietf::KEYBYTES;
    pub const TAGBYTES: usize = chacha20poly1305_ietf::TAGBYTES;

    pub fn create_key() -> Vec<u8> {
        chacha20poly1305_ietf::gen_key()[..].to_vec()
    }

    #[allow(dead_code)]
    pub fn gen_nonce() -> Vec<u8> {
        chacha20poly1305_ietf::gen_nonce()[..].to_vec()
    }

    pub fn encrypt_as_searchable(data: &[u8], key: &[u8], hmac_key: &[u8]) -> Vec<u8> {
        let hmacsha256::Tag(hash) = hmacsha256::authenticate(
            data,
            &hmacsha256::Key(_clone_into_array(hmac_key))
        );

        let ct = chacha20poly1305_ietf::seal(
            data,
            None,
            &chacha20poly1305_ietf::Nonce(_clone_into_array(&hash[..chacha20poly1305_ietf::NONCEBYTES])),
            &chacha20poly1305_ietf::Key(_clone_into_array(key))
        );

        let mut result: Vec<u8> = Default::default();
        result.extend_from_slice(&hash[..chacha20poly1305_ietf::NONCEBYTES]);
        result.extend_from_slice(&ct);
        result
    }

    pub fn encrypt_as_not_searchable(data: &[u8], key: &[u8]) -> Vec<u8> {
        let nonce = chacha20poly1305_ietf::gen_nonce();
        let ct = chacha20poly1305_ietf::seal(
            data,
            None,
            &nonce,
            &chacha20poly1305_ietf::Key(_clone_into_array(key))
        );

        let mut result: Vec<u8> = Default::default();
        result.extend_from_slice(&nonce.0);
        result.extend_from_slice(&ct);
        result
    }

    pub fn decrypt(enc_text: &[u8], key: &[u8]) -> Result<Vec<u8>, CommonError> {
        if enc_text.len() <= chacha20poly1305_ietf::NONCEBYTES {
            return Err(CommonError::InvalidStructure(format!("Unable to decrypt data: Cyphertext too short")));
        }

        chacha20poly1305_ietf::open(
            &enc_text[chacha20poly1305_ietf::NONCEBYTES..],
            None,
            &chacha20poly1305_ietf::Nonce(_clone_into_array(&enc_text[..chacha20poly1305_ietf::NONCEBYTES])),
            &chacha20poly1305_ietf::Key(_clone_into_array(key))
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
        let key = ChaCha20Poly1305IETF::create_key();
        let hmac_key = ChaCha20Poly1305IETF::create_key();

        let c = ChaCha20Poly1305IETF::encrypt_as_searchable(&data, &key, &hmac_key);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key).unwrap();
        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_as_not_searchable_decrypt_works() {
        let data = randombytes::randombytes(16);
        let key = ChaCha20Poly1305IETF::create_key();

        let c = ChaCha20Poly1305IETF::encrypt_as_not_searchable(&data, &key);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key).unwrap();
        assert_eq!(data, u);
    }
}