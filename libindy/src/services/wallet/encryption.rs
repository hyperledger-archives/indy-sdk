use std::str;
use std::collections::HashMap;

use utils::crypto::chacha20poly1305_ietf::ChaCha20Poly1305IETF;

use errors::wallet::WalletError;
use services::wallet::WalletRecord;

use super::wallet::Keys;
use super::storage::{Tag, TagName, StorageEntity};


pub(super) fn encrypt_tag_names(tag_names: &[&str], tag_name_key: &[u8], tags_hmac_key: &[u8]) -> Vec<TagName> {
    let mut encrypted_tag_names = Vec::new();

    for tag_name in tag_names {
        let tag_name_bytes = tag_name.as_bytes();
        let tag_name = if tag_name_bytes.len() > 0 && tag_name_bytes[0] as char == '~' {
            TagName::OfPlain(
                ChaCha20Poly1305IETF::encrypt_as_searchable(&tag_name_bytes[1..], tag_name_key, tags_hmac_key)
            )
        } else {
            TagName::OfEncrypted(
                ChaCha20Poly1305IETF::encrypt_as_searchable(tag_name_bytes, tag_name_key, tags_hmac_key)
            )
        };
        encrypted_tag_names.push(tag_name)
    }

    encrypted_tag_names
}

pub(super) fn encrypt_tags(tags: &HashMap<String, String>, tag_name_key: &[u8], tag_value_key: &[u8], tags_hmac_key: &[u8]) -> Vec<Tag> {
    let mut etags: Vec<Tag> = Vec::new();

    for (tag_name, tag_value) in tags {
        let tag_name_bytes = tag_name.as_bytes();
        if tag_name_bytes.len() > 0 && tag_name_bytes[0] as char == '~' {
            // '~' character on start is skipped.
            etags.push(
                Tag::PlainText(
                    ChaCha20Poly1305IETF::encrypt_as_searchable(&tag_name_bytes[1..], tag_name_key, tags_hmac_key),
                    tag_value.to_string()
                )
            );
        }
        else {
            etags.push(
                Tag::Encrypted(
                    ChaCha20Poly1305IETF::encrypt_as_searchable(tag_name_bytes, tag_name_key, tags_hmac_key),
                    ChaCha20Poly1305IETF::encrypt_as_searchable(tag_value.as_bytes(), tag_value_key, tags_hmac_key)
                )
            );
        }
    }
    etags
}

pub(super) fn decrypt_tags(etags: &Option<Vec<Tag>>, tag_name_key: &[u8], tag_value_key: &[u8]) -> Result<Option<HashMap<String, String>>, WalletError> {
    match etags {
        &None => Ok(None),
        &Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for etag in etags {
                let (name, value) = match etag {
                    &Tag::PlainText(ref ename, ref value) => {
                        let name = match ChaCha20Poly1305IETF::decrypt_merged(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name_bytes) => format!("~{}", str::from_utf8(&tag_name_bytes)?)
                        };
                        (name, value.clone())
                    },
                    &Tag::Encrypted(ref ename, ref evalue) => {
                        let name = match ChaCha20Poly1305IETF::decrypt_merged(&ename, tag_name_key) {
                            Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                            Ok(tag_name) => String::from_utf8(tag_name)?
                        };
                        let value = match ChaCha20Poly1305IETF::decrypt_merged(&evalue, tag_value_key) {
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
    let decrypted_name = ChaCha20Poly1305IETF::decrypt_merged(&record.name, &keys.name_key)?;
    let decrypted_name = String::from_utf8(decrypted_name)?;

    let decrypted_value = match record.value {
        Some(ref value) => {
            let decrypted_value_key = ChaCha20Poly1305IETF::decrypt_merged(&value.key, &keys.value_key)?;
            let decrypted_value = ChaCha20Poly1305IETF::decrypt_merged(&value.data, &decrypted_value_key)?;
            Some(String::from_utf8(decrypted_value)?)
        },
        None => None
    };

    let decrypted_type = match record.type_ {
        Some(ref type_) => {
            let decrypted_type = ChaCha20Poly1305IETF::decrypt_merged(type_, &keys.type_key)?;
            Some(String::from_utf8(decrypted_type)?)
        },
        None => None,
    };

    let decrypted_tags = decrypt_tags(&record.tags, &keys.tag_name_key, &keys.tag_value_key)?;
    Ok(WalletRecord::new(decrypted_name, decrypted_type, decrypted_value, decrypted_tags))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_tags() {
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "value1".to_string());
        tags.insert("tag2".to_string(), "value2".to_string());
        tags.insert("~tag3".to_string(), "value3".to_string());

        let tag_name_key = ChaCha20Poly1305IETF::create_key();
        let tag_value_key = ChaCha20Poly1305IETF::create_key();
        let hmac_key = ChaCha20Poly1305IETF::create_key();

        let c = encrypt_tags(&tags, &tag_name_key, &tag_value_key, &hmac_key);
        let u = decrypt_tags(&Some(c), &tag_name_key, &tag_value_key).unwrap().unwrap();
        assert_eq!(tags, u);
    }

    #[test]
    fn test_decrypt_tags_works_for_none() {
        let tag_name_key = ChaCha20Poly1305IETF::create_key();
        let tag_value_key = ChaCha20Poly1305IETF::create_key();

        let u = decrypt_tags(&None, &tag_name_key, &tag_value_key).unwrap();
        assert!(u.is_none());
    }
}