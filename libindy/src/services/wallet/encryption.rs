extern crate sodiumoxide;
extern crate serde_json;

use std::str;
use std::collections::HashMap;

use self::sodiumoxide::utils::memzero;

use utils::crypto::chacha20poly1305_ietf::{ChaCha20Poly1305IETF, ChaCha20Poly1305IETFKey, ChaCha20Poly1305IETFNonce, KEY_LENGTH, NONCE_LENGTH};
use utils::crypto::hmacsha256::{HMACSHA256Key, HMACSHA256};
use utils::crypto::pwhash_argon2i13::PwhashArgon2i13;
use services::wallet::WalletRecord;

use super::wallet::Keys;
use super::storage::{Tag, TagName, StorageEntity};

use errors::wallet::WalletError;


pub(super) fn derive_key(input: &[u8], salt: &[u8; 32]) -> Result<ChaCha20Poly1305IETFKey, WalletError> {
    let mut key_bytes: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
    PwhashArgon2i13::derive_key(&mut key_bytes, input, salt)?;
    let key = ChaCha20Poly1305IETF::clone_key_from_slice(&key_bytes[..]);
    memzero(&mut key_bytes[..]);
    Ok(key)
}


pub(super) fn encrypt_tag_names(tag_names: &[&str], tag_name_key: &ChaCha20Poly1305IETFKey, tags_hmac_key: &HMACSHA256Key) -> Vec<TagName> {
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

pub(super) fn encrypt_tags(tags: &HashMap<String, String>, tag_name_key: &ChaCha20Poly1305IETFKey, tag_value_key: &ChaCha20Poly1305IETFKey, tags_hmac_key: &HMACSHA256Key) -> Vec<Tag> {
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


pub(super) fn encrypt_as_searchable(data: &[u8], key: &ChaCha20Poly1305IETFKey, hmac_key: &HMACSHA256Key) -> Vec<u8> {
    let tag = HMACSHA256::create_tag(data, hmac_key);
    let nonce = ChaCha20Poly1305IETF::clone_nonce_from_slice(&tag[..NONCE_LENGTH]);
    let ct = ChaCha20Poly1305IETF::encrypt(data, key, &nonce);

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce.get_bytes());
    result.extend_from_slice(&ct);
    result
}

pub(super) fn encrypt_as_not_searchable(data: &[u8], key: &ChaCha20Poly1305IETFKey) -> Vec<u8> {
    let (ct, nonce) = ChaCha20Poly1305IETF::generate_nonce_and_encrypt(data, key);

    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(nonce.get_bytes());
    result.extend_from_slice(&ct);
    result
}

pub(super) fn decrypt(data: &[u8], key: &ChaCha20Poly1305IETFKey, nonce: &ChaCha20Poly1305IETFNonce) -> Result<Vec<u8>, WalletError> {
    let res = ChaCha20Poly1305IETF::decrypt(data, key, nonce)?;
    Ok(res)
}

pub(super) fn decrypt_merged(joined_data: &[u8], key: &ChaCha20Poly1305IETFKey) -> Result<Vec<u8>, WalletError> {
    let nonce = ChaCha20Poly1305IETF::clone_nonce_from_slice(&joined_data[..NONCE_LENGTH]);
    let data = &joined_data[NONCE_LENGTH..];

    let res = ChaCha20Poly1305IETF::decrypt(data, key, &nonce)?;
    Ok(res)
}


pub(super) fn decrypt_tags(etags: &Option<Vec<Tag>>, tag_name_key: &ChaCha20Poly1305IETFKey, tag_value_key: &ChaCha20Poly1305IETFKey) -> Result<Option<HashMap<String, String>>, WalletError> {
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

pub(super) fn decrypt_storage_record(record: &StorageEntity, keys: &Keys) -> Result<WalletRecord, WalletError> {
    let decrypted_name = decrypt_merged(&record.name, &keys.name_key)?;
    let decrypted_name = String::from_utf8(decrypted_name)?;

    let decrypted_value = match record.value {
        Some(ref value) => {
            let mut decrypted_value_key_bytes = decrypt_merged(&value.key, &keys.value_key)?;
            let decrypted_value_key = ChaCha20Poly1305IETF::clone_key_from_slice(&decrypted_value_key_bytes);
            memzero(&mut decrypted_value_key_bytes);
            let decrypted_value = decrypt_merged(&value.data, &decrypted_value_key)?;
            Some(String::from_utf8(decrypted_value)?)
        },
        None => None
    };

    let decrypted_type = match record.type_ {
        Some(ref type_) => {
            let decrypted_type = decrypt_merged(type_, &keys.type_key)?;
            Some(String::from_utf8(decrypted_type)?)
        },
        None => None,
    };

    let decrypted_tags = decrypt_tags(&record.tags, &keys.tag_name_key, &keys.tag_value_key)?;
    Ok(WalletRecord::new(decrypted_name, decrypted_type, decrypted_value, decrypted_tags))
}


#[cfg(test)]
mod tests {
    use utils::crypto::hmacsha256::HMACSHA256;
    use services::wallet::wallet::Keys;
    use super::*;
    use services::wallet::wallet::EncryptedValue;


    fn _generate_keys() -> Keys {
        let master_key = ChaCha20Poly1305IETF::generate_key();
        let keys_encrypted = Keys::gen_keys(&master_key);
        Keys::new(decrypt_merged(&keys_encrypted, &master_key).unwrap()).unwrap()
    }

    #[test]
    fn test_encrypt_decrypt_searchable() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let decrypted_data = decrypt_merged(&encrypted_data, &key).unwrap();

        assert_eq!(&decrypted_data[..], data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_wrong_key() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let key2 = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let res = decrypt_merged(&encrypted_data, &key2);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_nonce_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();
        let data = "test_data";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let byte_value = encrypted_data[3];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[3] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_data_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let index = encrypted_data.len() - 1;
        let byte_value = encrypted_data[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[index] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_tag_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let byte_value = encrypted_data[NONCE_LENGTH + 1];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[NONCE_LENGTH + 1] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

        #[test]
    fn test_encrypt_decrypt_not_searchable() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let decrypted_data = decrypt_merged(&encrypted_data, &key).unwrap();

        assert_eq!(&decrypted_data[..], data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_wrong_key() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let key2 = ChaCha20Poly1305IETF::generate_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let res = decrypt_merged(&encrypted_data, &key2);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_nonce_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let data = "test_data";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let byte_value = encrypted_data[3];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[3] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_data_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let index = encrypted_data.len() - 1;
        let byte_value = encrypted_data[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[index] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_tag_modified() {
        let key = ChaCha20Poly1305IETF::generate_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let byte_value = encrypted_data[NONCE_LENGTH + 1];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[NONCE_LENGTH + 1] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_encrypt_decrypt_tags() {
        let tags = serde_json::from_str(r#"{"tag1":"value1", "tag2":"value2", "~tag3":"value3"}"#).unwrap();

        let tag_name_key = ChaCha20Poly1305IETF::generate_key();
        let tag_value_key = ChaCha20Poly1305IETF::generate_key();
        let hmac_key = HMACSHA256::generate_key();

        let c = encrypt_tags(&tags, &tag_name_key, &tag_value_key, &hmac_key);
        let u = decrypt_tags(&Some(c), &tag_name_key, &tag_value_key).unwrap().unwrap();
        assert_eq!(tags, u);
    }

    #[test]
    fn test_decrypt_tags_works_for_none() {
        let tag_name_key = ChaCha20Poly1305IETF::generate_key();
        let tag_value_key = ChaCha20Poly1305IETF::generate_key();

        let u = decrypt_tags(&None, &tag_name_key, &tag_value_key).unwrap();
        assert!(u.is_none());
    }

    #[test]
    fn test_decrypt_storage_record_works() {
        let keys = _generate_keys();
        let name = "test_name";
        let value = "test_value";
        let encrypted_value = EncryptedValue::encrypt(value, &keys.value_key);
        let type_ = "test_type";
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &keys.name_key, &keys.item_hmac_key);
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &keys.type_key, &keys.item_hmac_key);
        let mut tags = HashMap::new();
        tags.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("~tag_name_2".to_string(), "tag_value_2".to_string());
        let encrypted_tags = encrypt_tags(&tags, &keys.tag_name_key, &keys.tag_value_key, &keys.tags_hmac_key);

        let storage_record = StorageEntity::new(encrypted_name, Some(encrypted_value), Some(encrypted_type), Some(encrypted_tags));
        let decrypted_wallet_record = decrypt_storage_record(&storage_record, &keys).unwrap();

        assert_eq!(&decrypted_wallet_record.name, name);
        assert_eq!(&decrypted_wallet_record.value.unwrap(), value);
        assert_eq!(&decrypted_wallet_record.type_.unwrap(), type_);
        assert_eq!(&decrypted_wallet_record.tags.unwrap(), &tags);
    }

    #[test]
    fn test_decrypt_storage_record_fails_if_wrong_keys() {
        let keys = _generate_keys();
        let keys2 = _generate_keys();
        let name = "test_name";
        let value = "test_value";
        let encrypted_value = EncryptedValue::encrypt(value, &keys.value_key);
        let type_ = "test_type";
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &keys.name_key, &keys.item_hmac_key);
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &keys.type_key, &keys.item_hmac_key);
        let mut tags = HashMap::new();
        tags.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("~tag_name_2".to_string(), "tag_value_2".to_string());
        let encrypted_tags = encrypt_tags(&tags, &keys.tag_name_key, &keys.tag_value_key, &keys.tags_hmac_key);

        let storage_record = StorageEntity::new(encrypted_name, Some(encrypted_value), Some(encrypted_type), Some(encrypted_tags));
        let res = decrypt_storage_record(&storage_record, &keys2);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }
}
