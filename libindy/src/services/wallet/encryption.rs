use std::collections::HashMap;

use sodiumoxide::crypto::aead::xchacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;

use errors::wallet::WalletError;

use super::storage::TagValue;


pub(super) fn encrypt_as_searchable(data: &[u8], key1: [u8; 32], key2: [u8; 32]) -> Vec<u8> {
    let hmacsha256::Tag(hash) = hmacsha256::authenticate(data, &hmacsha256::Key(key2));
    let mut nonce: [u8; xchacha20poly1305_ietf::NONCEBYTES] = Default::default();
    nonce.copy_from_slice(&hash[..xchacha20poly1305_ietf::NONCEBYTES]);
    let ct = xchacha20poly1305_ietf::seal(data, None, &xchacha20poly1305_ietf::Nonce(nonce), &xchacha20poly1305_ietf::Key(key1));
    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn encrypt_as_not_searchable(data: &[u8], key: [u8; 32]) -> Vec<u8> {
    let nonce = xchacha20poly1305_ietf::gen_nonce();
    let ct = xchacha20poly1305_ietf::seal(data, None, &nonce, &xchacha20poly1305_ietf::Key(key));
    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce.0);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn decrypt(enc_text: &Vec<u8>, key1: [u8; 32]) -> Result<Vec<u8>, WalletError> {
    let mut nonce: [u8; xchacha20poly1305_ietf::NONCEBYTES] = Default::default();
    nonce.copy_from_slice(&enc_text[..xchacha20poly1305_ietf::NONCEBYTES]);

    let mut cypher_text: Vec<u8> = Default::default();
    cypher_text.extend_from_slice(&enc_text[xchacha20poly1305_ietf::NONCEBYTES..]);

    match xchacha20poly1305_ietf::open(&cypher_text, None, &xchacha20poly1305_ietf::Nonce(nonce), &xchacha20poly1305_ietf::Key(key1)) {
        Err(_) => Err(WalletError::EncryptionError("Decryption error".to_string())),
        Ok(x) => Ok(x)
    }
}

pub(super) fn encrypt_tags(tags: &HashMap<String, String>, tag_name_key: [u8; 32], tag_value_key: [u8; 32], tags_hmac_key: [u8; 32]) -> HashMap<Vec<u8>, TagValue> {
    let mut etags: HashMap<Vec<u8>, TagValue> = HashMap::new();

    for (tag_name, tag_value) in tags {
        let ekey = encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key);
        if tag_name.chars().next() == Some('~') {
            etags.insert(ekey, TagValue::Plain(tag_value.to_string()));
        }
        else {
            etags.insert(ekey, TagValue::Encrypted(encrypt_as_searchable(tag_value.as_bytes(), tag_value_key, tags_hmac_key)));
        }
    }
    etags
}

pub(super) fn decrypt_tags(etags: &Option<HashMap<Vec<u8>, TagValue>>, tag_name_key: [u8; 32], tag_value_key: [u8; 32]) -> Result<Option<HashMap<String, String>>, WalletError> {
    match etags {
        &None => Ok(None),
        &Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for (ref etag_name, ref etag_value) in etags {
                let tag_name = match decrypt(&etag_name, tag_name_key) {
                    Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                    Ok(tag_name) => String::from_utf8(tag_name)?
                };

                let tag_value = match etag_value {
                    &&TagValue::Plain(ref plain_value) => plain_value.clone(),
                    &&TagValue::Encrypted(ref evalue) |
                    &&TagValue::Meta(ref evalue) => match decrypt(&evalue, tag_value_key) {
                        Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag value".to_string())),
                        Ok(tag_value) => String::from_utf8(tag_value)?
                    }
                };
                tags.insert(tag_name, tag_value);
            }

            Ok(Some(tags))
        }
    }
}


#[test]
    fn testencrypt_searchabledecrypt() {
        let x = "Some text".to_string();
        let xchacha20poly1305_ietf::Key(key1) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(key2) = xchacha20poly1305_ietf::gen_key();

        let c = encrypt_as_searchable(x.as_bytes(), key1, key2);
        let u = decrypt(&c, key1).unwrap();
        assert_eq!(x.into_bytes(), u);
    }

    #[test]
    fn testencrypt_notsearchabledecrypt() {
        let x = "Some text".to_string();
        let xchacha20poly1305_ietf::Key(key1) = xchacha20poly1305_ietf::gen_key();

        let c = encrypt_as_not_searchable(x.as_bytes(), key1);
        let u = decrypt(&c, key1).unwrap();
        assert_eq!(x.into_bytes(), u);
    }

    #[test]
    fn testencryptdecrypt_tags() {
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "value1".to_string());
        tags.insert("tag2".to_string(), "value2".to_string());
        let xchacha20poly1305_ietf::Key(tag_name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(hmac_key) = xchacha20poly1305_ietf::gen_key();

        let c = encrypt_tags(&tags, tag_name_key, tag_value_key, hmac_key);
        let u = decrypt_tags(&Some(c), tag_name_key, tag_value_key).unwrap().unwrap();
        assert_eq!(tags, u);
    }

    #[test]
    fn testdecrypt_tags_works_for_none() {
        let xchacha20poly1305_ietf::Key(tag_name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(hmac_key) = xchacha20poly1305_ietf::gen_key();

        let u = decrypt_tags(&None, tag_name_key, tag_value_key).unwrap();
        assert!(u.is_none());
    }