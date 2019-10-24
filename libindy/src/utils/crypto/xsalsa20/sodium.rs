extern crate sodiumoxide;

use indy_api_types::errors::prelude::*;
use failure::{err_msg, ResultExt};
use self::sodiumoxide::crypto::secretbox;
use self::sodiumoxide::crypto::secretbox::xsalsa20poly1305;

pub const KEYBYTES: usize = xsalsa20poly1305::KEYBYTES;
pub const NONCEBYTES: usize = xsalsa20poly1305::NONCEBYTES;
pub const MACBYTES: usize = xsalsa20poly1305::MACBYTES;

sodium_type!(Key, xsalsa20poly1305::Key, KEYBYTES);
sodium_type!(Nonce, xsalsa20poly1305::Nonce, NONCEBYTES);
sodium_type!(Tag, xsalsa20poly1305::Tag, MACBYTES);

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
        &key.0,
    )
}

pub fn decrypt(key: &Key, nonce: &Nonce, doc: &[u8]) -> Result<Vec<u8>, IndyError> {
    secretbox::open(
        doc,
        &nonce.0,
        &key.0
    )
        .map_err(|_| err_msg("Unable to open sodium secretbox"))
        .context(IndyErrorKind::InvalidStructure)
        .map_err(|err| err.into())
}

pub fn encrypt_detached(key: &Key, nonce: &Nonce, doc: &[u8]) -> (Vec<u8>, Tag) {
    let mut cipher = doc.to_vec();
    let tag = secretbox::seal_detached(cipher.as_mut_slice(),
                                &nonce.0,
                                &key.0);


    (cipher, Tag(tag))
}

pub fn decrypt_detached(key: &Key, nonce: &Nonce, tag: &Tag, doc: &[u8]) -> Result<Vec<u8>, IndyError> {
    let mut plain = doc.to_vec();
    secretbox::open_detached(plain.as_mut_slice(),
                                &tag.0,
                                &nonce.0,
                                &key.0)
        .map_err(|_| err_msg("Unable to decrypt data"))
        .context(IndyErrorKind::InvalidStructure)
        .map_err(|err| err.into())
        .map(|()| plain)
}


#[cfg(test)]
mod tests {
    use self::sodiumoxide::randombytes;
    use super::*;

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
