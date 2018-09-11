extern crate sodiumoxide;

use utils::crypto::xsalsa20::XSalsa20;
use errors::route::RouteError;

pub struct Payload {
    pub iv: Vec<u8>,
    pub tag: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub sym_key: Vec<u8>
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
    use super::{encrypt_payload, decrypt_payload};

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
