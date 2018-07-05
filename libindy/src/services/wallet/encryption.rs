use std::str;
use std::collections::HashMap;

use utils::crypto::{chacha20poly1305_ietf, hmacsha256, pwhash_argon2i13};
use utils::crypto::memzero::memzero;

use super::{Keys, WalletRecord};
use super::storage::{Tag, TagName, StorageRecord};

use errors::wallet::WalletError;

pub(super) fn gen_master_key_salt() -> Result<pwhash_argon2i13::Salt, WalletError> {
    Ok(pwhash_argon2i13::gen_salt())
}

pub(super) fn master_key_salt_from_slice(slice: &[u8]) -> Result<pwhash_argon2i13::Salt, WalletError> {
    let salt = pwhash_argon2i13::Salt::from_slice(slice)
        .map_err(|err| ::errors::common::CommonError::InvalidState("Invalid master key salt".to_string()))?;

    Ok(salt)
}

pub(super) fn derive_master_key(passphrase: &str, salt: &pwhash_argon2i13::Salt) -> Result<chacha20poly1305_ietf::Key, WalletError> {
    let mut key_bytes = [0u8; chacha20poly1305_ietf::KEYBYTES];
    pwhash_argon2i13::pwhash(&mut key_bytes, passphrase.as_bytes(), salt)?;
    Ok(chacha20poly1305_ietf::Key::new(key_bytes))
}

pub(super) fn encrypt_tag_names(tag_names: &[&str], tag_name_key: &chacha20poly1305_ietf::Key, tags_hmac_key: &hmacsha256::Key) -> Vec<TagName> {
    tag_names
        .iter()
        .map(|tag_name|
            if tag_name.starts_with("~") {
                TagName::OfPlain(encrypt_as_searchable(
                    &tag_name.as_bytes()[1..], tag_name_key, tags_hmac_key))
            } else {
                TagName::OfEncrypted(encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key))
            })
        .collect::<Vec<TagName>>()
}

pub(super) fn encrypt_tags(tags: &HashMap<String, String>,
                           tag_name_key: &chacha20poly1305_ietf::Key,
                           tag_value_key: &chacha20poly1305_ietf::Key,
                           tags_hmac_key: &hmacsha256::Key) -> Vec<Tag> {
    tags
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
        .collect::<Vec<Tag>>()
}


pub(super) fn encrypt_as_searchable(data: &[u8], key: &chacha20poly1305_ietf::Key, hmac_key: &hmacsha256::Key) -> Vec<u8> {
    let tag = hmacsha256::authenticate(data, hmac_key);
    let nonce = chacha20poly1305_ietf::Nonce::from_slice(&tag[..chacha20poly1305_ietf::NONCEBYTES]).unwrap(); // We can safely unwrap here
    let ct = chacha20poly1305_ietf::encrypt(data, key, &nonce);

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce[..]);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn encrypt_as_not_searchable(data: &[u8], key: &chacha20poly1305_ietf::Key) -> Vec<u8> {
    let (ct, nonce) = chacha20poly1305_ietf::gen_nonce_and_encrypt(data, key);

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce[..]);
    result.extend_from_slice(&ct);
    result
}

pub(super) fn decrypt(data: &[u8], key: &chacha20poly1305_ietf::Key, nonce: &chacha20poly1305_ietf::Nonce) -> Result<Vec<u8>, WalletError> {
    let res = chacha20poly1305_ietf::decrypt(data, key, nonce)?;
    Ok(res)
}

pub(super) fn decrypt_merged(joined_data: &[u8], key: &chacha20poly1305_ietf::Key) -> Result<Vec<u8>, WalletError> {
    let nonce = chacha20poly1305_ietf::Nonce::from_slice(&joined_data[..chacha20poly1305_ietf::NONCEBYTES]).unwrap(); // We can safety unwrap here
    let data = &joined_data[chacha20poly1305_ietf::NONCEBYTES..];
    let res = chacha20poly1305_ietf::decrypt(data, key, &nonce)?;
    Ok(res)
}

pub(super) fn decrypt_tags(etags: &Option<Vec<Tag>>, tag_name_key: &chacha20poly1305_ietf::Key, tag_value_key: &chacha20poly1305_ietf::Key) -> Result<Option<HashMap<String, String>>, WalletError> {
    match etags {
        &None => Ok(None),
        &Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for etag in etags {
                let (name, value) = match etag {
                    &Tag::PlainText(ref ename, ref value) => {
                        let name = match decrypt_merged(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name_bytes) => format!("~{}", str::from_utf8(&tag_name_bytes)?)
                        };
                        (name, value.clone())
                    }
                    &Tag::Encrypted(ref ename, ref evalue) => {
                        let name = match decrypt_merged(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name) => String::from_utf8(tag_name)?
                        };
                        let value = match decrypt_merged(&evalue, tag_value_key) {
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

pub(super) fn decrypt_storage_record(record: &StorageRecord, keys: &Keys) -> Result<WalletRecord, WalletError> {
    let decrypted_name = decrypt_merged(&record.id, &keys.name_key)?;
    let decrypted_name = String::from_utf8(decrypted_name)?;

    let decrypted_value = match record.value {
        Some(ref value) => {
            let mut decrypted_value_key_bytes = decrypt_merged(&value.key, &keys.value_key)?;
            let decrypted_value_key = chacha20poly1305_ietf::Key::from_slice(&decrypted_value_key_bytes).unwrap(); // FIXME:
            memzero(&mut decrypted_value_key_bytes);
            let decrypted_value = decrypt_merged(&value.data, &decrypted_value_key)?;
            Some(String::from_utf8(decrypted_value)?)
        }
        None => None
    };

    let decrypted_type = match record.type_ {
        Some(ref type_) => {
            let decrypted_type = decrypt_merged(type_, &keys.type_key)?;
            Some(String::from_utf8(decrypted_type)?)
        }
        None => None,
    };

    let decrypted_tags = decrypt_tags(&record.tags, &keys.tag_name_key, &keys.tag_value_key)?;
    Ok(WalletRecord::new(decrypted_name, decrypted_type, decrypted_value, decrypted_tags))
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_encrypt_decrypt_tags() {
        let tags = serde_json::from_str(r#"{"tag1":"value1", "tag2":"value2", "~tag3":"value3"}"#).unwrap();

        let tag_name_key = chacha20poly1305_ietf::gen_key();
        let tag_value_key = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();

        let c = encrypt_tags(&tags, &tag_name_key, &tag_value_key, &hmac_key);
        let u = decrypt_tags(&Some(c), &tag_name_key, &tag_value_key).unwrap().unwrap();
        assert_eq!(tags, u);
    }

    #[test]
    fn test_decrypt_tags_works_for_none() {
        let tag_name_key = chacha20poly1305_ietf::gen_key();
        let tag_value_key = chacha20poly1305_ietf::gen_key();

        let u = decrypt_tags(&None, &tag_name_key, &tag_value_key).unwrap();
        assert!(u.is_none());
    }
}
