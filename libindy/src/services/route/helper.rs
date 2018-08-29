extern crate sodiumoxide;

use services::microledger::did_doc::DidDoc;
use services::microledger::constants::AUTHZ_ALL;
use services::microledger::helpers::create_storage_options;
use services::route::jwm::{JWM, Header};
use services::route::route_table::RouteTable;
use utils::crypto::xsalsa20::XSalsa20;
use utils::environment::EnvironmentUtils;
use utils::crypto::base64::{encode, decode};

pub fn encrypt_payload(plaintext: String) -> (String, String, String, String) {

    //prep for payload encryption
    let xsalsa = XSalsa20::new();
    let ephem_sym_key = xsalsa.create_key();
    let iv = xsalsa.gen_nonce();

    //encrypt payload
    let ciphertext_with_tag = xsalsa.encrypt(&ephem_sym_key, &iv, plaintext.as_bytes());
    let split_tag_and_ciphertext = ciphertext_with_tag.split_at(sodiumoxide::crypto::secretbox::MACBYTES);
    let tag = encode(split_tag_and_ciphertext.0);
    let ciphertext = encode(split_tag_and_ciphertext.1);


    (encode(&iv), tag, ciphertext, encode(&ephem_sym_key))
}

pub fn decrypt_payload(iv : String, tag : String, ciphertext : String, sym_key : String) -> String {
    let iv_bytes = match decode(&iv) {
        Ok(v) => v,
        Err(e) => return e.to_string()
    };

    let tag_bytes = match decode(&tag) {
        Ok(v) => v,
        Err(e) => return e.to_string()
    };

    let ciphertext_bytes = match decode(&iv) {
        Ok(v) => v,
        Err(e) => return e.to_string()
    };

    let sym_key_bytes = match decode(&sym_key) {
        Ok(v) => v,
        Err(e) => return e.to_string()
    };

    let xsalsa = XSalsa20::new();
    let mut cipher_with_tag: Vec<u8> = vec![];
    cipher_with_tag.extend(tag_bytes);
    cipher_with_tag.extend(ciphertext_bytes);

    let result = xsalsa.decrypt(&sym_key_bytes, &iv_bytes, cipher_with_tag.as_slice());

    let message= match result {
        Ok(v) => match String::from_utf8(v) {
            Ok(m) => m,
            Err(e) => e.to_string()
        },
        Err(e) => return e.to_string()
    };

    return message;
}


#[cfg(test)]
mod tests {
    use super::{encrypt_payload, decrypt_payload};

    #[test]
    pub fn test_encrypt_payload(){
        let message = "This is a test message".to_string();
        let (iv, tag, ciphertext, sym_key) = encrypt_payload(message);
    }

    #[test]
    pub fn test_encrypt_then_decrypt_payload_works(){
        let message = "This is a test message".to_string();
        let (iv, tag, ciphertext, sym_key) = encrypt_payload(message.clone());
        let decrypted_message = decrypt_payload(iv, tag, ciphertext, sym_key);

        assert_eq!(message, decrypted_message);
    }
}