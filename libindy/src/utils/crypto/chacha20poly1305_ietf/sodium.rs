extern crate sodiumoxide;

use self::sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use self::sodiumoxide::utils;

use errors::common::CommonError;

pub const KEYBYTES: usize = chacha20poly1305_ietf::KEYBYTES;
pub const NONCEBYTES: usize = chacha20poly1305_ietf::NONCEBYTES;
pub const TAGBYTES: usize = chacha20poly1305_ietf::TAGBYTES;

sodium_type!(Key, chacha20poly1305_ietf::Key, KEYBYTES);
sodium_type!(Nonce, chacha20poly1305_ietf::Nonce, NONCEBYTES);

impl Nonce {
    pub fn increment(&mut self) {
        utils::increment_le(&mut (self.0).0);
    }
}

pub fn gen_key() -> Key {
    Key(chacha20poly1305_ietf::gen_key())
}

#[allow(dead_code)]
pub fn gen_nonce() -> Nonce {
    Nonce(chacha20poly1305_ietf::gen_nonce())
}

pub fn gen_nonce_and_encrypt(data: &[u8], key: &Key) -> (Vec<u8>, Nonce) {
    let nonce = gen_nonce();

    let encrypted_data = chacha20poly1305_ietf::seal(
        data,
        None,
        &nonce.0,
        &key.0
    );

    (encrypted_data, nonce)
}

pub fn encrypt(data: &[u8], key: &Key, nonce: &Nonce) -> Vec<u8> {
    chacha20poly1305_ietf::seal(
        data,
        None,
        &nonce.0,
        &key.0,
    )
}

pub fn decrypt(data: &[u8], key: &Key, nonce: &Nonce) -> Result<Vec<u8>, CommonError> {
    chacha20poly1305_ietf::open(
        &data,
        None,
        &nonce.0,
        &key.0,
    )
        .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
}


#[cfg(test)]
mod tests {
    extern crate rmp_serde;

    use super::*;
    use self::sodiumoxide::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let data = randombytes::randombytes(100);
        let key = gen_key();

        let (c, nonce) = gen_nonce_and_encrypt(&data, &key);
        let u = decrypt(&c, &key, &nonce).unwrap();

        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_decrypt_works_for_nonce() {
        let data = randombytes::randombytes(16);

        let key = gen_key();
        let nonce = gen_nonce();
        let c = encrypt(&data, &key, &nonce);
        let u = decrypt(&c, &key, &nonce).unwrap();

        assert_eq!(data, u)
    }

    #[test]
    fn nonce_serialize_deserialize_works() {
        let nonce = gen_nonce();
        let serialized = rmp_serde::to_vec(&nonce).unwrap();
        let deserialized: Nonce = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(serialized.len(), NONCEBYTES + 2);
        assert_eq!(nonce, deserialized)
    }
    #[test]
    fn key_serialize_deserialize_works() {
        let key = gen_key();
        let serialized = rmp_serde::to_vec(&key).unwrap();
        let deserialized: Key = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(serialized.len(), KEYBYTES + 2);
        assert_eq!(key, deserialized)
    }

}
