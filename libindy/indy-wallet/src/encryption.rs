use std::collections::HashMap;
use std::str;

use indy_api_types::domain::wallet::KeyDerivationMethod;
use indy_api_types::errors::prelude::*;
use indy_utils::crypto::{chacha20poly1305_ietf, hmacsha256, pwhash_argon2i13};

use super::{Keys, WalletRecord, Metadata};
use super::storage::{StorageRecord, Tag, TagName};
use rust_base58::FromBase58;

#[cfg(test)]
pub(super) fn gen_master_key_salt() -> IndyResult<pwhash_argon2i13::Salt> {
    Ok(pwhash_argon2i13::gen_salt())
}

pub(super) fn master_key_salt_from_slice(slice: &[u8]) -> IndyResult<pwhash_argon2i13::Salt> {
    let salt = pwhash_argon2i13::Salt::from_slice(slice)
        .to_indy(IndyErrorKind::WalletAccessFailed, "Invalid master key salt")?;

    Ok(salt)
}

//TODO memzero for passphrase
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeyDerivationData {
    Raw(String),
    Argon2iMod(String, pwhash_argon2i13::Salt),
    Argon2iInt(String, pwhash_argon2i13::Salt),
}

impl KeyDerivationData {
    pub fn from_passphrase_with_new_salt(passphrase: &str, derivation_method: &KeyDerivationMethod) -> Self {
        let salt = pwhash_argon2i13::gen_salt();
        let passphrase = passphrase.to_owned();
        match *derivation_method {
            KeyDerivationMethod::ARGON2I_INT =>
                KeyDerivationData::Argon2iInt(passphrase, salt),
            KeyDerivationMethod::ARGON2I_MOD =>
                KeyDerivationData::Argon2iMod(passphrase, salt),
            KeyDerivationMethod::RAW =>
                KeyDerivationData::Raw(passphrase)
        }
    }

    pub(super) fn from_passphrase_and_metadata(passphrase: &str, metadata: &Metadata, derivation_method: &KeyDerivationMethod) -> IndyResult<Self> {
        let passphrase = passphrase.to_owned();

        let data = match (derivation_method, metadata) {
            (KeyDerivationMethod::RAW, &Metadata::MetadataRaw(_)) => {
                KeyDerivationData::Raw(passphrase)
            }
            (KeyDerivationMethod::ARGON2I_INT, &Metadata::MetadataArgon(ref metadata)) => {
                let master_key_salt = master_key_salt_from_slice(&metadata.master_key_salt)?;
                KeyDerivationData::Argon2iInt(passphrase, master_key_salt)
            }
            (KeyDerivationMethod::ARGON2I_MOD, &Metadata::MetadataArgon(ref metadata)) => {
                let master_key_salt = master_key_salt_from_slice(&metadata.master_key_salt)?;
                KeyDerivationData::Argon2iMod(passphrase, master_key_salt)
            }
            _ => return Err(err_msg(IndyErrorKind::WalletAccessFailed, "Invalid combination of KeyDerivationMethod and Metadata"))
        };

        Ok(data)
    }

    pub fn calc_master_key(&self) -> IndyResult<chacha20poly1305_ietf::Key> {
        match self {
            KeyDerivationData::Raw(passphrase) => _raw_master_key(passphrase),
            KeyDerivationData::Argon2iInt(passphrase, salt) => _derive_master_key(passphrase, &salt, &KeyDerivationMethod::ARGON2I_INT),
            KeyDerivationData::Argon2iMod(passphrase, salt) => _derive_master_key(passphrase, &salt, &KeyDerivationMethod::ARGON2I_MOD),
        }
    }
}

fn _derive_master_key(passphrase: &str, salt: &pwhash_argon2i13::Salt, key_derivation_method: &KeyDerivationMethod) -> IndyResult<chacha20poly1305_ietf::Key> {
    let key = chacha20poly1305_ietf::derive_key(passphrase, salt, key_derivation_method)?;
    Ok(key)
}

fn _raw_master_key(passphrase: &str) -> IndyResult<chacha20poly1305_ietf::Key> {
    let bytes = passphrase.from_base58()?;

    chacha20poly1305_ietf::Key::from_slice(&bytes)
        .map_err(|err| err.extend("Invalid mastery key"))
}

pub(super) fn encrypt_tag_names(tag_names: &[&str], tag_name_key: &chacha20poly1305_ietf::Key, tags_hmac_key: &hmacsha256::Key) -> Vec<TagName> {
    tag_names
        .iter()
        .map(|tag_name|
            if tag_name.starts_with('~') {
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
            if tag_name.starts_with('~') {
                // '~' character on start is skipped.
                Tag::PlainText(
                    encrypt_as_searchable(&tag_name.as_bytes()[1..], tag_name_key, tags_hmac_key),
                    tag_value.to_string(),
                )
            } else {
                Tag::Encrypted(
                    encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key),
                    encrypt_as_searchable(tag_value.as_bytes(), tag_value_key, tags_hmac_key),
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

pub(super) fn decrypt(data: &[u8], key: &chacha20poly1305_ietf::Key, nonce: &chacha20poly1305_ietf::Nonce) -> IndyResult<Vec<u8>> {
    let res = chacha20poly1305_ietf::decrypt(data, key, nonce)?;
    Ok(res)
}

pub(super) fn decrypt_merged(joined_data: &[u8], key: &chacha20poly1305_ietf::Key) -> IndyResult<Vec<u8>> {
    let nonce = chacha20poly1305_ietf::Nonce::from_slice(&joined_data[..chacha20poly1305_ietf::NONCEBYTES]).unwrap(); // We can safety unwrap here
    let data = &joined_data[chacha20poly1305_ietf::NONCEBYTES..];
    let res = decrypt(data, key, &nonce)?;
    Ok(res)
}

pub(super) fn decrypt_tags(etags: &Option<Vec<Tag>>, tag_name_key: &chacha20poly1305_ietf::Key, tag_value_key: &chacha20poly1305_ietf::Key) -> IndyResult<Option<HashMap<String, String>>> {
    match *etags {
        None => Ok(None),
        Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for etag in etags {
                let (name, value) = match *etag {
                    Tag::PlainText(ref ename, ref value) => {
                        let name = match decrypt_merged(&ename, tag_name_key) {
                            Err(err) => return Err(err.to_indy(IndyErrorKind::WalletEncryptionError, "Unable to decrypt tag name")),
                            Ok(tag_name_bytes) => format!("~{}", str::from_utf8(&tag_name_bytes).to_indy(IndyErrorKind::WalletEncryptionError, "Plaintext Tag name is invalid utf8")?)
                        };
                        (name, value.clone())
                    }
                    Tag::Encrypted(ref ename, ref evalue) => {
                        let name = String::from_utf8(decrypt_merged(&ename, tag_name_key)?).to_indy(IndyErrorKind::WalletEncryptionError, "Tag name is invalid utf8")?;
                        let value = String::from_utf8(decrypt_merged(&evalue, tag_value_key)?).to_indy(IndyErrorKind::WalletEncryptionError, "Tag value is invalid utf8")?;
                        (name, value)
                    }
                };
                tags.insert(name, value);
            }

            Ok(Some(tags))
        }
    }
}

pub(super) fn decrypt_storage_record(record: &StorageRecord, keys: &Keys) -> IndyResult<WalletRecord> {
    let decrypted_name = decrypt_merged(&record.id, &keys.name_key)?;

    let decrypted_name = String::from_utf8(decrypted_name)
        .to_indy(IndyErrorKind::WalletEncryptionError, "Record is invalid utf8")?;

    let decrypted_value = match record.value {
        Some(ref value) => Some(value.decrypt(&keys.value_key)?),
        None => None
    };

    let decrypted_type = match record.type_ {
        Some(ref type_) => {
            let decrypted_type = decrypt_merged(type_, &keys.type_key)?;
            Some(String::from_utf8(decrypted_type)
                .to_indy(IndyErrorKind::WalletEncryptionError, "Record type is invalid utf8")?)
        }
        None => None,
    };

    let decrypted_tags = decrypt_tags(&record.tags, &keys.tag_name_key, &keys.tag_value_key)?;
    Ok(WalletRecord::new(decrypted_name, decrypted_type, decrypted_value, decrypted_tags))
}


#[cfg(test)]
mod tests {
    use crate::wallet::EncryptedValue;
    use crate::wallet::Keys;
    use indy_utils::crypto::hmacsha256;

    use super::*;

    #[test]
    fn test_encrypt_decrypt_searchable() {
        let key = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let decrypted_data = decrypt_merged(&encrypted_data, &key).unwrap();

        assert_eq!(&decrypted_data[..], data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_wrong_key() {
        let key = chacha20poly1305_ietf::gen_key();
        let key2 = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let res = decrypt_merged(&encrypted_data, &key2);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_nonce_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();
        let data = "test_data";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let byte_value = encrypted_data[3];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[3] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_data_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let index = encrypted_data.len() - 1;
        let byte_value = encrypted_data[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[index] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_searchable_returns_error_if_tag_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let hmac_key = hmacsha256::gen_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_searchable(data.as_bytes(), &key, &hmac_key);
        let byte_value = encrypted_data[chacha20poly1305_ietf::NONCEBYTES + 1];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[chacha20poly1305_ietf::NONCEBYTES + 1] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable() {
        let key = chacha20poly1305_ietf::gen_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let decrypted_data = decrypt_merged(&encrypted_data, &key).unwrap();

        assert_eq!(&decrypted_data[..], data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_wrong_key() {
        let key = chacha20poly1305_ietf::gen_key();
        let key2 = chacha20poly1305_ietf::gen_key();
        let data = "test_data";

        let encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let res = decrypt_merged(&encrypted_data, &key2);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_nonce_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let data = "test_data";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let byte_value = encrypted_data[3];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[3] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_data_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let index = encrypted_data.len() - 1;
        let byte_value = encrypted_data[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[index] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn test_encrypt_decrypt_not_searchable_returns_error_if_tag_modified() {
        let key = chacha20poly1305_ietf::gen_key();
        let data = "12345678901234567890123456789012345678901234567890";

        let mut encrypted_data = encrypt_as_not_searchable(data.as_bytes(), &key);
        let byte_value = encrypted_data[chacha20poly1305_ietf::NONCEBYTES + 1];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        encrypted_data[chacha20poly1305_ietf::NONCEBYTES + 1] = new_byte_value;
        let res = decrypt_merged(&encrypted_data, &key);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

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

    #[test]
    fn test_decrypt_storage_record_works() {
        let keys = Keys::new();
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

        let storage_record = StorageRecord {
            id: encrypted_name,
            value: Some(encrypted_value),
            type_: Some(encrypted_type),
            tags: Some(encrypted_tags),
        };
        let decrypted_wallet_record = decrypt_storage_record(&storage_record, &keys).unwrap();

        assert_eq!(&decrypted_wallet_record.id, name);
        assert_eq!(&decrypted_wallet_record.value.unwrap(), value);
        assert_eq!(&decrypted_wallet_record.type_.unwrap(), type_);
        assert_eq!(&decrypted_wallet_record.tags.unwrap(), &tags);
    }

    #[test]
    fn test_decrypt_storage_record_fails_if_wrong_keys() {
        let keys = Keys::new();
        let keys2 = Keys::new();
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

        let storage_record = StorageRecord {
            id: encrypted_name,
            value: Some(encrypted_value),
            type_: Some(encrypted_type),
            tags: Some(encrypted_tags),
        };
        let res = decrypt_storage_record(&storage_record, &keys2);

        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }
}