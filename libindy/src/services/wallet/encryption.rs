extern crate serde_json;

use std::str;
use std::collections::HashMap;

use utils::crypto::chacha20poly1305_ietf::ChaCha20Poly1305IETF;

use errors::common::CommonError;
use errors::wallet::WalletError;

use super::storage::{Tag, TagName};

pub(super) fn encrypt_tag_names(tag_names: &str, tag_name_key: &[u8], tags_hmac_key: &[u8]) -> Result<Vec<TagName>, WalletError> {
    let tag_names: Vec<String> = serde_json::from_str(tag_names)
        .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tag_names")))?;

    let etag_names = tag_names
        .iter()
        .map(|tag_name|
            if tag_name.starts_with("~") {
                TagName::OfPlain(encrypt_as_searchable(
                    &tag_name.as_bytes()[1..], tag_name_key, tags_hmac_key))
            } else {
                TagName::OfEncrypted(encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key))
            })
        .collect::<Vec<TagName>>();

    Ok(etag_names)
}

pub(super) fn encrypt_tags(tags: &str, tag_name_key: &[u8], tag_value_key: &[u8], tags_hmac_key: &[u8]) -> Result<Vec<Tag>, WalletError> {
    let tags: HashMap<String, String> = serde_json::from_str(tags)
        .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize tags")))?;

    let etags = tags
        .iter()
        .map(|(tag_name, tag_value)|
            if tag_name.starts_with("~") {
                // '~' character on start is skipped.
                Tag::PlainText(
                    encrypt_as_searchable(&tag_name.as_bytes()[1..], tag_name_key, tags_hmac_key),
                    tag_value.to_string()
                )
            } else {
                Tag::Encrypted(
                    encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key),
                    encrypt_as_searchable(tag_value.as_bytes(), tag_value_key, tags_hmac_key)
                )
            })
        .collect::<Vec<Tag>>();

    Ok(etags)
}

pub(super) fn encrypt_as_searchable(data: &[u8], key: &[u8], hmac_key: &[u8]) -> Vec<u8> {
    let nonce = ChaCha20Poly1305IETF::hmacsha256_authenticate(data, hmac_key);
    let (ct, nonce) = ChaCha20Poly1305IETF::encrypt(data, key, Some(&nonce[..ChaCha20Poly1305IETF::NONCEBYTES]));

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn encrypt_as_not_searchable(data: &[u8], key: &[u8]) -> Vec<u8> {
    let (ct, nonce) = ChaCha20Poly1305IETF::encrypt(data, key, None);

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn decrypt(joined_data: &[u8], key: &[u8]) -> Result<Vec<u8>, WalletError> {
    let nonce = &joined_data[..ChaCha20Poly1305IETF::NONCEBYTES];
    let data = &joined_data[ChaCha20Poly1305IETF::NONCEBYTES..];

    let res = ChaCha20Poly1305IETF::decrypt(data, key, nonce)?;
    Ok(res)
}

pub(super) fn decrypt_tags(etags: &Option<Vec<Tag>>, tag_name_key: &[u8], tag_value_key: &[u8]) -> Result<Option<HashMap<String, String>>, WalletError> {
    match etags {
        &None => Ok(None),
        &Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for etag in etags {
                let (name, value) = match etag {
                    &Tag::PlainText(ref ename, ref value) => {
                        let name = match decrypt(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name_bytes) => format!("~{}", str::from_utf8(&tag_name_bytes)?)
                        };
                        (name, value.clone())
                    }
                    &Tag::Encrypted(ref ename, ref evalue) => {
                        let name = match decrypt(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name) => String::from_utf8(tag_name)?
                        };
                        let value = match decrypt(&evalue, tag_value_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag value".to_string())),
                            Ok(tag_value) => String::from_utf8(tag_value)?
                        };
                        (name, value)
                    }
                };
                tags.insert(name, value);
            }

            Ok(Some(tags))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_tags() {
        let tags = r#"{"tag1":"value1", "tag2":"value2", "~tag3":"value3"}"#;

        let tag_name_key = ChaCha20Poly1305IETF::create_key();
        let tag_value_key = ChaCha20Poly1305IETF::create_key();
        let hmac_key = ChaCha20Poly1305IETF::create_key();

        let c = encrypt_tags(&tags, &tag_name_key, &tag_value_key, &hmac_key).unwrap();
        let u = decrypt_tags(&Some(c), &tag_name_key, &tag_value_key).unwrap().unwrap();
        assert_eq!(serde_json::from_str::<HashMap<String, String>>(tags).unwrap(), u);
    }

    #[test]
    fn test_decrypt_tags_works_for_none() {
        let tag_name_key = ChaCha20Poly1305IETF::create_key();
        let tag_value_key = ChaCha20Poly1305IETF::create_key();

        let u = decrypt_tags(&None, &tag_name_key, &tag_value_key).unwrap();
        assert!(u.is_none());
    }
}