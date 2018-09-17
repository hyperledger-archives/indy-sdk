extern crate sodiumoxide;

use errors::common::CommonError;
use errors::route::RouteError;
use domain::route::Payload;

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

pub fn encrypt_payload(plaintext: &str) -> Payload {

    //prep for payload encryption
    let xsalsa = XSalsa20::new();
    let sym_key = xsalsa.create_key();
    let iv = xsalsa.gen_nonce();

    //encrypt payload
    let (ciphertext, tag) = xsalsa.encrypt_detached(&sym_key, &iv, plaintext.as_bytes());

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
    let xsalsa = XSalsa20::new();
    let result = xsalsa.decrypt_detached(&payload.sym_key,
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
        let xsalsa20 = XSalsa20::new();

        let nonce = xsalsa20.gen_nonce();
        let key = xsalsa20.create_key();
        let data = randombytes::randombytes(16);

        let encrypted_data = xsalsa20.encrypt(&key, &nonce, &data);
        let decrypt_result = xsalsa20.decrypt(&key, &nonce, &encrypted_data);

        assert!(decrypt_result.is_ok());
        assert_eq!(data, decrypt_result.unwrap());
    }

    #[test]
    pub fn test_encrypt_payload(){
        let message = "This is a test message";
        let payload= encrypt_payload(message);
    }

    #[test]
    pub fn test_encrypt_then_decrypt_payload_works(){
        let message = "This is a test message";
        let payload = encrypt_payload(message.clone());
        let decrypted_message = decrypt_payload(&payload).unwrap();

        assert_eq!(message, decrypted_message);
    }

}
