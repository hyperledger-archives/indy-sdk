extern crate sodiumoxide;

use services::microledger::did_doc::DidDoc;
use services::microledger::constants::AUTHZ_ALL;
use services::microledger::helpers::create_storage_options;
use services::route::jwm::{JWM, Header};
use services::route::route_table::RouteTable;
use utils::crypto::xsalsa20::XSalsa20;
use utils::environment::EnvironmentUtils;
use utils::crypto::base64::{encode, decode};

pub struct Payload {
    iv: String,
    tag: String,
    ciphertext: String,
    key: String
}

#[derive(Debug)]
pub enum RouteError {
    EncryptionError(String),
    EncodingError(String)
}

pub fn encrypt_payload(plaintext: &str) -> Payload {

    //prep for payload encryption
    let xsalsa = XSalsa20::new();
    let ephem_sym_key = xsalsa.create_key();
    let iv = xsalsa.gen_nonce();

    //encrypt payload
    let (ciphertext, tag) = xsalsa.encrypt_detached(&ephem_sym_key, &iv, plaintext.as_bytes());
//    let tag = encode(&ciphertext_with_tag[0..sodiumoxide::crypto::secretbox::MACBYTES]);
//    let ciphertext = encode(&ciphertext_with_tag[sodiumoxide::crypto::secretbox::MACBYTES..]);

    Payload {
        iv: encode(&iv),
        tag: encode(&tag),
        ciphertext: encode(&ciphertext),
        key: encode(&ephem_sym_key)
    }
}

pub fn decrypt_payload(payload: &Payload) -> Result<String, RouteError> {
    let iv = decode(&payload.iv).map_err(|err| RouteError::EncodingError(format!("{}", err)))?;
    let tag = decode(&payload.tag).map_err(|err| RouteError::EncodingError(format!("{}", err)))?;
    let ciphertext = decode(&payload.ciphertext).map_err(|err| RouteError::EncodingError(format!("{}", err)))?;
    let sym_key = decode(&payload.key).map_err(|err| RouteError::EncodingError(format!("{}", err)))?;

    let xsalsa = XSalsa20::new();
    let result = xsalsa.decrypt_detached(sym_key.as_slice(), iv.as_slice(), tag.as_slice(), ciphertext.as_slice());

    match result {
        Ok(v) => Ok(String::from_utf8(v).map_err(|err| RouteError::EncodingError(format!("{}", err)))?),
        Err(e) => Err(RouteError::EncodingError(format!("{}", e)))
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
