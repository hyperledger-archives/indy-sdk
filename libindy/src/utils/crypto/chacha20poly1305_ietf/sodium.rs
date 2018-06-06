extern crate sodiumoxide;

use errors::common::CommonError;

use self::sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;
use utils::byte_array::_clone_into_array;

pub struct ChaCha20Poly1305IETF {}

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

    pub fn hmacsha256_authenticate(data: &[u8], hmac_key: &[u8]) -> Vec<u8> {
        hmacsha256::authenticate(
            data,
            &hmacsha256::Key(_clone_into_array(hmac_key))
        )[..].to_vec()
    }

    pub fn encrypt(data: &[u8], key: &[u8], nonce: Option<&[u8]>) -> (Vec<u8>, Vec<u8>) {
        let nonce = match nonce {
            Some(n) => chacha20poly1305_ietf::Nonce(_clone_into_array(n)),
            None => chacha20poly1305_ietf::gen_nonce()
        };

        (chacha20poly1305_ietf::seal(
            data,
            None,
            &nonce,
            &chacha20poly1305_ietf::Key(_clone_into_array(key))
        ),
         nonce[..].to_vec())
    }

    pub fn decrypt(data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        if nonce.len() != chacha20poly1305_ietf::NONCEBYTES {
            return Err(CommonError::InvalidStructure(format!("Invalid nonce")));
        }

        chacha20poly1305_ietf::open(
            &data,
            None,
            &chacha20poly1305_ietf::Nonce(_clone_into_array(nonce)),
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
    fn encrypt_decrypt_works() {
        let data = randombytes::randombytes(100);
        let key = ChaCha20Poly1305IETF::create_key();

        let (c, nonce) = ChaCha20Poly1305IETF::encrypt(&data, &key, None);
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key, &nonce).unwrap();
        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_decrypt_works_for_nonce() {
        let data = randombytes::randombytes(16);
        let key = ChaCha20Poly1305IETF::create_key();
        let nonce = ChaCha20Poly1305IETF::gen_nonce();

        let (c, nonce) = ChaCha20Poly1305IETF::encrypt(&data, &key, Some(&nonce));
        let u = ChaCha20Poly1305IETF::decrypt(&c, &key, &nonce).unwrap();
        assert_eq!(data, u);
    }
}