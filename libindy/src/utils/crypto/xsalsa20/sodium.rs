extern crate sodiumoxide;

use errors::common::CommonError;
use errors::route::RouteError;
use domain::route::Payload;
use utils::byte_array::_clone_into_array;

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

pub fn encrypt_detached(key: &[u8], nonce: &[u8], doc: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut cipher = doc.to_vec();
    let tag = secretbox::seal_detached(cipher.as_mut_slice(),
                                &secretbox::Nonce(_clone_into_array(nonce)),
                                &secretbox::Key(_clone_into_array(key)));
    (cipher, tag[..].to_vec())
}

pub fn decrypt_detached(key: &[u8], nonce: &[u8], tag: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
    let mut plain = doc.to_vec();
    secretbox::open_detached(plain.as_mut_slice(),
                                &secretbox::Tag(_clone_into_array(tag)),
                                &secretbox::Nonce(_clone_into_array(nonce)),
                                &secretbox::Key(_clone_into_array(key))).map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))?;
    Ok(plain.to_vec())
}

pub fn encrypt_payload(plaintext: &str) -> Payload {

    //prep for payload encryption
    // TODO FIX ME: make this depend on functions above instead of old implementation
    let sym_key = secretbox::gen_key()[..].to_vec();
    let iv = secretbox::gen_nonce()[..].to_vec();

    //encrypt payload
    let (ciphertext, tag) = encrypt_detached(&sym_key, &iv, plaintext.as_bytes());

    //encode payload and return
    Payload {
        iv,
        tag,
        ciphertext,
        sym_key
    }
}

pub fn decrypt_payload(payload: &Payload) -> Result<String, RouteError> {
    //decrypt payload

    let result = decrypt_detached(&payload.sym_key,
                                                     &payload.iv.as_slice(),
                                                       &payload.tag.as_slice(),
                                                       &payload.ciphertext.as_slice());
    //return plaintext or throw error
    match result {
        Ok(v) => Ok(String::from_utf8(v).map_err(|err| RouteError::DecodeError(format!("{}", err)))?),
        Err(e) => Err(RouteError::EncryptionError(format!("{}", e)))
    }
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

    #[test]
    pub fn test_encrypt_payload(){
        let message = "This is a test message";
        let _payload= encrypt_payload(message);
    }

    #[test]
    pub fn test_encrypt_then_decrypt_payload_works(){
        let message = "This is a test message";
        let payload = encrypt_payload(message.clone());
        let decrypted_message = decrypt_payload(&payload).unwrap();

        assert_eq!(message, decrypted_message);
    }

}
