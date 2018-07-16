extern crate sodiumoxide;

use self::sodiumoxide::utils::memzero;
use std::collections::HashMap;
use std::rc::Rc;

use utils::crypto::{hmacsha256, chacha20poly1305_ietf};

use errors::wallet::WalletError;
use errors::common::CommonError;

use super::storage;
use super::iterator::WalletIterator;
use super::encryption::*;
use super::query_encryption::encrypt_query;
use super::language;
use super::WalletRecord;

#[derive(Serialize, Deserialize)]
pub(super) struct Keys {
    pub type_key: chacha20poly1305_ietf::Key,
    pub name_key: chacha20poly1305_ietf::Key,
    pub value_key: chacha20poly1305_ietf::Key,
    pub item_hmac_key: hmacsha256::Key,
    pub tag_name_key: chacha20poly1305_ietf::Key,
    pub tag_value_key: chacha20poly1305_ietf::Key,
    pub tags_hmac_key: hmacsha256::Key,
}

impl Keys {
    pub fn new() -> Keys {
        Keys {
            type_key: chacha20poly1305_ietf::gen_key(),
            name_key: chacha20poly1305_ietf::gen_key(),
            value_key: chacha20poly1305_ietf::gen_key(),
            item_hmac_key: hmacsha256::gen_key(),
            tag_name_key: chacha20poly1305_ietf::gen_key(),
            tag_value_key: chacha20poly1305_ietf::gen_key(),
            tags_hmac_key: hmacsha256::gen_key(),
        }
    }

    pub fn serialize_encrypted(&self, master_key: &chacha20poly1305_ietf::Key) -> Result<Vec<u8>, WalletError> {
        extern crate rmp_serde;

        let mut serialized = rmp_serde::to_vec(self)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize keys: {:?}", err)))?;

        let encrypted = encrypt_as_not_searchable(&serialized, master_key);

        memzero(&mut serialized[..]);
        Ok(encrypted)
    }

    pub fn deserialize_encrypted(bytes: &[u8], master_key: &chacha20poly1305_ietf::Key) -> Result<Keys, WalletError> {
        extern crate rmp_serde;

        let mut decrypted = decrypt_merged(bytes, master_key)
            .map_err(|_| WalletError::AccessFailed("Invalid master key provided".to_string()))?;

        let keys: Keys = rmp_serde::from_slice(&decrypted)
            .map_err(|_| WalletError::AccessFailed("Invalid master key provided".to_string()))?;

        memzero(&mut decrypted[..]);
        Ok(keys)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedValue {
    pub data: Vec<u8>,
    pub key: Vec<u8>,
}

const ENCRYPTED_KEY_LEN: usize = chacha20poly1305_ietf::TAGBYTES + chacha20poly1305_ietf::NONCEBYTES + chacha20poly1305_ietf::KEYBYTES;

impl EncryptedValue {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        Self { data, key }
    }

    pub fn encrypt(data: &str, key: &chacha20poly1305_ietf::Key) -> Self {
        let value_key = chacha20poly1305_ietf::gen_key();
        EncryptedValue::new(
            encrypt_as_not_searchable(data.as_bytes(), &value_key),
            encrypt_as_not_searchable(&value_key[..], key)
        )
    }

    pub fn decrypt(&self, key: &chacha20poly1305_ietf::Key) -> Result<String, WalletError> {
        let mut value_key_bytes = decrypt_merged(&self.key, key)?;

        let value_key = chacha20poly1305_ietf::Key::from_slice(&value_key_bytes)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid value key: {:?}", err)))?;

        memzero(&mut value_key_bytes[..]);

        let res = String::from_utf8(decrypt_merged(&self.data, &value_key)?)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid UTF8 string inside of value: {:?}", err)))?;

        Ok(res)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.key.clone();
        result.extend_from_slice(self.data.as_slice());
        result
    }

    pub fn from_bytes(joined_data: &[u8]) -> Result<Self, CommonError> {
        // value_key is stored as NONCE || CYPHERTEXT. Lenth of CYPHERTHEXT is length of DATA + length of TAG.
        if joined_data.len() < ENCRYPTED_KEY_LEN {
            return Err(CommonError::InvalidStructure(format!("Unable to split value_key from value: value too short")));
        }

        let value_key = joined_data[..ENCRYPTED_KEY_LEN].to_owned();
        let value = joined_data[ENCRYPTED_KEY_LEN..].to_owned();
        Ok(EncryptedValue { data: value, key: value_key })
    }
}

pub(super) struct Wallet {
    id: String,
    storage: Box<storage::WalletStorage>,
    keys: Rc<Keys>,
}

impl Wallet {
    pub fn new(id: String, storage: Box<storage::WalletStorage>, keys: Rc<Keys>) -> Wallet {
        Wallet { id, storage, keys }
    }

    pub fn add(&self, type_: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        let etype = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);
        let evalue = EncryptedValue::encrypt(value, &self.keys.value_key);
        let etags = encrypt_tags(tags, &self.keys.tag_name_key, &self.keys.tag_value_key, &self.keys.tags_hmac_key);
        self.storage.add(&etype, &ename, &evalue, &etags)?;
        Ok(())
    }

    pub fn add_tags(&self, type_: &str, name: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);
        let encrypted_tags = encrypt_tags(tags, &self.keys.tag_name_key, &self.keys.tag_value_key, &self.keys.tags_hmac_key);
        self.storage.add_tags(&encrypted_type, &encrypted_name, &encrypted_tags)?;
        Ok(())
    }

    pub fn update_tags(&self, type_: &str, name: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);
        let encrypted_tags = encrypt_tags(tags, &self.keys.tag_name_key, &self.keys.tag_value_key, &self.keys.tags_hmac_key);
        self.storage.update_tags(&encrypted_type, &encrypted_name, &encrypted_tags)?;
        Ok(())
    }

    pub fn delete_tags(&self, type_: &str, name: &str, tag_names: &[&str]) -> Result<(), WalletError> {
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);
        let encrypted_tag_names = encrypt_tag_names(tag_names, &self.keys.tag_name_key, &self.keys.tags_hmac_key);
        self.storage.delete_tags(&encrypted_type, &encrypted_name, &encrypted_tag_names[..])?;
        Ok(())
    }

    pub fn update(&self, type_: &str, name: &str, new_value: &str) -> Result<(), WalletError> {
        let encrypted_type = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let encrypted_name = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);
        let encrypted_value = EncryptedValue::encrypt(new_value, &self.keys.value_key);
        self.storage.update(&encrypted_type, &encrypted_name, &encrypted_value)?;
        Ok(())
    }

    pub fn get(&self, type_: &str, name: &str, options: &str) -> Result<WalletRecord, WalletError> {
        let etype = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);

        let result = self.storage.get(&etype, &ename, options)?;

        let value = match result.value {
            None => None,
            Some(encrypted_value) => Some(encrypted_value.decrypt(&self.keys.value_key)?)
        };

        let tags = decrypt_tags(&result.tags, &self.keys.tag_name_key, &self.keys.tag_value_key)?;

        Ok(WalletRecord::new(String::from(name), result.type_.map(|_| type_.to_string()), value, tags))
    }

    pub fn delete(&self, type_: &str, name: &str) -> Result<(), WalletError> {
        let etype = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), &self.keys.name_key, &self.keys.item_hmac_key);

        self.storage.delete(&etype, &ename)?;
        Ok(())
    }

    pub fn search<'a>(&'a self, type_: &str, query: &str, options: Option<&str>) -> Result<WalletIterator, WalletError> {
        let parsed_query = language::parse_from_json(query)?;
        let encrypted_query = encrypt_query(parsed_query, &self.keys)?;
        let encrypted_type_ = encrypt_as_searchable(type_.as_bytes(), &self.keys.type_key, &self.keys.item_hmac_key);
        let storage_iterator = self.storage.search(&encrypted_type_, &encrypted_query, options)?;
        let wallet_iterator = WalletIterator::new(storage_iterator, Rc::clone(&self.keys));
        Ok(wallet_iterator)
    }

    pub fn close(&mut self) -> Result<(), WalletError> {
        self.storage.close()?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<WalletIterator, WalletError> {
        let all_items = self.storage.get_all()?;
        Ok(WalletIterator::new(all_items, Rc::clone(&self.keys)))
    }

    pub fn get_id<'a>(&'a self) -> &'a str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use std::rc::Rc;
    use std::collections::HashMap;

    use domain::wallet::Metadata;
    use errors::wallet::WalletError;
    use services::wallet::encryption;
    use services::wallet::wallet::Wallet;
    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::language::*;
    use utils::test::TestUtils;

    macro_rules! jsonstr {
        ($($x:tt)+) => {
            json!($($x)+).to_string()
        }
    }

    macro_rules! jsonmap {
        ($($x:tt)+) => {
            {
                let map: ::std::collections::HashMap<String, String> = serde_json::from_value(json!($($x)+)).unwrap();
                map
            }
        }
    }

    #[test]
    fn wallet_get_id_works() {
        _cleanup();

        let wallet = _wallet();
        assert_eq!(wallet.get_id(), _wallet_id());
    }

    #[test]
    fn wallet_add_get_works() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.name, _id1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(record.tags.unwrap(), _tags());
    }

    #[test]
    fn wallet_add_get_works_for_reopen() {
        _cleanup();

        let mut wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();
        wallet.close().unwrap();

        let wallet = _exists_wallet();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.name, _id1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(record.tags.unwrap(), _tags());
    }

    #[test]
    fn wallet_get_works_for_non_existing() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.get(_type1(), _id2(), &_fetch_options(false, true, true));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_add_works_for_already_existing() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.add(_type1(), _id1(), _value2(), &_tags());
        assert_match!(Err(WalletError::ItemAlreadyExists), res);
    }

    #[test]
    fn wallet_update_works() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.name, _id1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(record.tags.unwrap(), _tags());

        wallet.update(_type1(), _id1(), _value2()).unwrap();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.name, _id1());
        assert_eq!(record.value.unwrap(), _value2());
        assert_eq!(record.tags.unwrap(), _tags());
    }

    #[test]
    fn wallet_update_works_for_non_existing_id() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.update(_type1(), _id2(), _value2());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_update_works_for_non_existing_type() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.update(_type2(), _id1(), _value2());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    /**
     * Add tags tests
     */
    #[test]
    fn wallet_add_tags_works() {
        _cleanup();

        let tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
         });

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let new_tags = jsonmap!({
            "tag_name_2": "tag_value_2",
            "~tag_name_3": "~tag_value_3",
        });

        wallet.add_tags(_type1(), _id1(), &new_tags).unwrap();
        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();

        let expected_tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
            "~tag_name_3": "~tag_value_3",
         });

        assert_eq!(record.tags.unwrap(), expected_tags);
    }

    #[test]
    fn wallet_update_tags_works() {
        _cleanup();

        let tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
         });

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let new_tags = jsonmap!({
            "tag_name_2": "tag_value_2",
            "~tag_name_3": "~tag_value_3",
        });

        wallet.update_tags(_type1(), _id1(), &new_tags).unwrap();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.tags.unwrap(), new_tags);
    }

    #[test]
    fn wallet_delete_tags_works() {
        _cleanup();

        let tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
            "~tag_name_3": "~tag_value_3",
            "~tag_name_4": "~tag_value_4",
         });

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        wallet.delete_tags(_type1(), _id1(), &vec!["tag_name_1", "~tag_name_3", "tag_name_5", "~tag_name_6"]).unwrap();

        let expected_tags = jsonmap!({
            "tag_name_2": "tag_value_2",
            "~tag_name_4": "~tag_value_4",
         });

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.tags.unwrap(), expected_tags);
    }

    #[test]
    fn wallet_delete_works() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let record = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true)).unwrap();
        assert_eq!(record.name, _id1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(record.tags.unwrap(), _tags());

        wallet.delete(_type1(), _id1()).unwrap();

        let res = wallet.get(_type1(), _id1(), &_fetch_options(false, true, true));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_delete_works_for_non_existing_id() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.delete(_type1(), _id2());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_delete_works_for_non_existing_type() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let res = wallet.delete(_type2(), _id1());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn language_parse_from_json_ecrypt_query_works() {
        _cleanup();

        let query = jsonstr!({
            "k1": "v1",
            "$or": [
                {
                    "~k2": {"$like": "like_target"},
                    "~k3": {"$gte": "100"},
                    "$not": {
                        "k4": "v4",
                        "~k5": {
                            "$like": "like_string"
                        },
                    },
                },
                {
                    "k6": {"$in": ["in_string_1", "in_string_2"]},
                }
            ],
            "$not": {
                "$not": {
                    "$not": {
                        "$not": {
                            "k7": "v7"
                        }
                    }
                }
            },
            "$not": {
                "k8": "v8"
            }
        });

        let query = language::parse_from_json(&query).unwrap();
        let encrypted_query = encrypt_query(query, &Keys::new()).unwrap();

        assert_match!(Operator::And(_), encrypted_query);
    }

    #[test]
    fn wallet_search_works_for_empty_query() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &_tags()).unwrap();

        let mut iterator = wallet.search(
            _type1(),
            "{}",
            Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
                type_: None,
            },
            WalletRecord {
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
                type_: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_empty_query_with_count() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &_tags()).unwrap();

        let mut iterator = wallet.search(
            _type1(),
            "{}",
            Some(&_search_options(true, true, true, true, true))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: Some(_tags()),
                type_: Some(_type1().to_string()),
            },
            WalletRecord {
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: Some(_tags()),
                type_: Some(_type1().to_string()),
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert_eq!(iterator.get_total_count().unwrap().unwrap(), 2);
    }

    #[test]
    fn wallet_search_works_for_empty_query_with_only_count() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &_tags()).unwrap();

        let mut iterator = wallet.search(
            _type1(),
            "{}",
            Some(&_search_options(false, true, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert_eq!(iterator.get_total_count().unwrap().unwrap(), 2);
    }

    #[test]
    fn wallet_search_works_for_eq_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_2",
         });

        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with different tag name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_3": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with different tag value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": "tag_value_2"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search equal name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_empty_tag_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _type1(), &_tags()).unwrap();

        let res = wallet.search(_type1(),
                                &jsonstr!({
                                    "tag1": "tag2",
                                    "~": "tag3",
                                }),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res)
    }

    #[test]
    fn wallet_search_works_for_empty_tag_encrypted() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _type1(), &_tags()).unwrap();

        let res = wallet.search(_type1(),
                                &jsonstr!({
                                    "tag1": "tag2",
                                    "": "tag3",
                                }),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res)
    }

    #[test]
    fn wallet_search_works_for_eq_plan() {
        _cleanup();

        let wallet = _wallet();

        let tags = jsonmap!({
            "~tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
         });

        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with different tag name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_3": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with different tag value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": "tag_value_2"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search equal name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": "tag_value_1"}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    // neq tests //
    #[test]
    fn wallet_search_works_for_neq_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let tags = jsonmap!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_2",
         });

        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with matching value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": {"$neq": "tag_value_1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with neq value and neq name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_different": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search with different type
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_neq_plain() {
        _cleanup();

        let wallet = _wallet();

        let tags = jsonmap!({
            "~tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
         });

        wallet.add(_type1(), _id1(), _value1(), &tags).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with matching value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_1": {"$neq": "tag_value_1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with neq value and neq name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search with different type
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_1": {"$neq": "tag_value_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_gt_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name":"1"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name":"2"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name":"3"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$gt": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$gt": "4"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$gt": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$gt": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_gt_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let res = wallet.search(_type1(),
                                &jsonstr!({"tag_name": {"$gt": "1"}}),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    #[test]
    fn wallet_search_works_for_gte_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name":"1"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name":"2"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name":"3"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$gte": "2"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$gte": "4"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$gte": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$gte": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_gte_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let res = wallet.search(_type1(),
                                &jsonstr!({"tag_name": {"$gte": "1"}}),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    #[test]
    fn wallet_search_works_for_lt_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name":"2"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name":"3"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name":"4"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$lt": "4"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$lt": "2"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$lt": "4"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$lt": "4"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_lt_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let res = wallet.search(_type1(),
                                &jsonstr!({"tag_name": {"$lt": "4"}}),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    #[test]
    fn wallet_search_works_for_lte_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name":"2"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name":"3"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name":"4"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$lte": "3"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$lte": "1"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$lte": "3"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with different type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$lte": "3"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_lte_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let res = wallet.search(_type1(),
                                &jsonstr!({"tag_name": {"$lte": "3"}}),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    #[test]
    fn wallet_search_works_for_like_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name": "tag_value_1"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name": "tag_value_2"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name": "not_matching"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$like": "tag_value_%"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$like": "tag_value_no_match%"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$like": "tag_value_%"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search wrong type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$like": "tag_value_%"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_like_encrypted() {
        _cleanup();

        let wallet = _wallet();

        let res = wallet.search(_type1(),
                                &jsonstr!({"tag_name": {"$like": "1"}}),
                                Some(&_search_options(true, false, false, true, false)));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    #[test]
    fn wallet_search_works_for_in_plain() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"~tag_name": "tag_value_1"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"~tag_name": "tag_value_2"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"~tag_name": "tag_value_3"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$in": ["tag_value_4", "tag_value_5"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting tag
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name_different": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful encrypted search
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search wrong type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"~tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_in_encrypted() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &jsonmap!({"tag_name": "tag_value_1"})).unwrap();
        wallet.add(_type1(), _id2(), _value2(), &jsonmap!({"tag_name": "tag_value_2"})).unwrap();
        wallet.add(_type1(), _id3(), _value3(), &jsonmap!({"tag_name": "tag_value_3"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name": {"$in": ["tag_value_4", "tag_value_5"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search with nonexisting tag
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"tag_name_different": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful plain search
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"~tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // unsuccessful search wrong type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({"tag_name": {"$in": ["tag_value_1", "tag_value_3"]}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }


    #[test]
    fn wallet_search_works_for_and() {
        _cleanup();

        let wallet = _wallet();

        wallet.add(_type1(),
                   _id1(),
                   _value1(),
                   &jsonmap!({
                        "tag_name_1": "tag_value_1",
                        "tag_name_2": "tag_value_2",
                        "~tag_name_2": "tag_value_2",
                        "~tag_name_3": "tag_value_3"})).unwrap();

        wallet.add(_type1(),
                   _id2(),
                   _value2(),
                   &jsonmap!({
                        "tag_name_1": "tag_value_1",
                        "tag_name_2": "tag_value_2",
                        "~tag_name_2": "tag_value_3",
                        "~tag_name_3": "tag_value_3"})).unwrap();

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                            "tag_name_1": "tag_value_1",
                                            "tag_name_2": "tag_value_2",
                                            "~tag_name_2": "tag_value_2",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                            "tag_name_1": "tag_value_1",
                                            "~tag_name_2": "tag_value_3",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            }
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                            "tag_name_1": "tag_value_1",
                                            "~tag_name_3": "tag_value_3",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // no matches
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                             "tag_name_1": "tag_value_1",
                                             "~tag_name_3": "tag_value_3",
                                             "tag_name_4": "tag_value_4",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // wrong type
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({
                                              "tag_name_1": "tag_value_1",
                                              "~tag_name_2": "tag_value_2",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // wrong tag name
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                               "tag_name_1": "tag_value_1",
                                               "tag_name_3": "tag_value_3",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // wrong tag value
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                                 "tag_name_1": "tag_value_0",
                                                  "~tag_name_2": "tag_value_3",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_or() {
        _cleanup();

        let wallet = _wallet();

        wallet.add(_type1(),
                   _id1(),
                   _value1(),
                   &jsonmap!({
                       "tag_name_1": "tag_value_1",
                       "~tag_name_2": "tag_value_21",
                       "~tag_name_3": "tag_value_3"})).unwrap();

        wallet.add(_type1(),
                   _id2(),
                   _value2(),
                   &jsonmap!({
                       "tag_name_1": "tag_value_1",
                       "~tag_name_2": "tag_value_22",
                       "~tag_name_3": "tag_value_3"})).unwrap();

        wallet.add(_type1(),
                   _id3(),
                   _value3(),
                   &jsonmap!({
                       "tag_name_1": "tag_value_1",
                       "~tag_name_3": "tag_value_3",
                       "~tag_name_4": "tag_value_4"})).unwrap();

        // All 3
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                              "$or": [
                                                  {"tag_name_1": "tag_value_1"},
                                                  {"~tag_name_2": "tag_value_22"},
                                                  {"~tag_name_4": "tag_value_4"}
                                             ]
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());


        // 1 and 3 matching
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                              "$or": [
                                                   {"~tag_name_2": "tag_value_21"},
                                                   {"~tag_name_4": "tag_value_4"}
                                             ]
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // 3 matching, 1 not because wrong tag type
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                              "$or": [
                                                   {"tag_name_2": "tag_value_21"},
                                                   {"~tag_name_4": "tag_value_4"}
                                             ]
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        // no matching
        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                                 "tag_name_1": "tag_value_0",
                                                  "~tag_name_2": "tag_value_3",
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());

        // no matching - wrong type_
        let mut iterator = wallet.search(_type2(),
                                         &jsonstr!({
                                              "$or": [
                                                  {"tag_name_1": "tag_value_1"},
                                                  {"~tag_name_2": "tag_value_22"},
                                                  {"~tag_name_4": "tag_value_4"}
                                             ]
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_not() {
        _cleanup();

        let wallet = _wallet();

        wallet.add(_type1(),
                   _id1(),
                   _value1(),
                   &jsonmap!({
                       "tag_name_1": "tag_value_1",
                       "~tag_name_2": "tag_value_21",
                       "~tag_name_3": "tag_value_3"})).unwrap();

        wallet.add(_type1(),
                   _id2(),
                   _value2(),
                   &jsonmap!({
                       "tag_name_12": "tag_value_12",
                       "~tag_name_2": "tag_value_22"})).unwrap();

        wallet.add(_type1(),
                   _id3(),
                   _value3(),
                   &jsonmap!({
                       "tag_name_13": "tag_value_13",
                       "~tag_name_4": "tag_value_4"})).unwrap();


        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"$not": {"tag_name_1": "tag_value_1_different"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id2().to_string(),
                value: Some(_value2().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({"$not": {"~tag_name_2": "tag_value_22"}}),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = _sort(vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
            WalletRecord {
                type_: None,
                name: _id3().to_string(),
                value: Some(_value3().to_string()),
                tags: None,
            },
        ]);

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());

        let mut iterator = wallet.search(_type1(),
                                         &jsonstr!({
                                             "$not": {
                                                  "$or": [
                                                     {"tag_name_1": "tag_value_1"},
                                                     {"~tag_name_2": "tag_value_22"},
                                                     {"~tag_name_4": "tag_value_4"},
                                                  ]
                                              }
                                         }),
                                         Some(&_search_options(true, false, false, true, false))).unwrap();

        assert!(iterator.next().unwrap().is_none());
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    #[test]
    fn wallet_search_works_for_nested() {
        _cleanup();

        let wallet = _wallet();
        wallet.add(_type1(), _id1(), _value1(), &_tags()).unwrap();

        let query = jsonstr!({
            "$or": [
                    {"foo": "bar"},
                    {"$not": {
                        "$not": {
                            "$not": {
                                "$not": {
                                    "k7": "v7"
                                }
                            }
                        }
                    },
                        "$not": {
                            "k8": "v8"
                        }
                    }
            ]
        });

        let mut iterator = wallet.search(_type1(), &query, Some(&_search_options(true, false, false, true, false))).unwrap();

        let expected_records = vec![
            WalletRecord {
                type_: None,
                name: _id1().to_string(),
                value: Some(_value1().to_string()),
                tags: None,
            },
        ];

        assert_eq!(_fetch_all(&mut iterator), expected_records);
        assert!(iterator.get_total_count().unwrap().is_none());
    }

    fn _cleanup() {
        TestUtils::cleanup_storage();
    }

    fn _type1() -> &'static str {
        "type1"
    }

    fn _type2() -> &'static str {
        "type2"
    }

    fn _id1() -> &'static str {
        "id1"
    }

    fn _id2() -> &'static str {
        "id2"
    }

    fn _id3() -> &'static str {
        "id3"
    }

    fn _value1() -> &'static str {
        "value1"
    }

    fn _value2() -> &'static str {
        "value2"
    }

    fn _value3() -> &'static str {
        "value3"
    }

    fn _tags() -> HashMap<String, String> {
        jsonmap!({"tag1": "tag_value_1"})
    }

    fn _wallet_id() -> &'static str {
        "w1"
    }

    fn _wallet() -> Wallet {
        let storage_type = SQLiteStorageType::new();
        let master_key = _master_key();

        let keys = Keys::new();

        let metadata = {
            let master_key_salt = encryption::gen_master_key_salt().unwrap();

            let metadata = Metadata {
                master_key_salt: master_key_salt[..].to_vec(),
                keys: keys.serialize_encrypted(&master_key).unwrap(),
            };

            serde_json::to_vec(&metadata)
                .map_err(|err| CommonError::InvalidState(format!("Cannot serialize wallet metadata: {:?}", err))).unwrap()
        };

        storage_type.create_storage(_wallet_id(),
                                    None,
                                    None,
                                    &metadata).unwrap();

        let storage = storage_type.open_storage(_wallet_id(), None, None).unwrap();

        Wallet::new(_wallet_id().to_string(), storage, Rc::new(keys))
    }

    fn _exists_wallet() -> Wallet {
        let storage_type = SQLiteStorageType::new();
        let storage = storage_type.open_storage(_wallet_id(), None, None).unwrap();

        let metadata: Metadata = {
            let metadata = storage.get_storage_metadata().unwrap();
            serde_json::from_slice(&metadata)
                .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize metadata: {:?}", err))).unwrap()
        };

        let master_key = _master_key();
        let keys = Keys::deserialize_encrypted(&metadata.keys, &master_key).unwrap();

        Wallet::new(_wallet_id().to_string(), storage, Rc::new(keys))
    }

    fn _master_key() -> chacha20poly1305_ietf::Key {
        chacha20poly1305_ietf::Key::new([
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ])
    }

    fn _new_master_key() -> chacha20poly1305_ietf::Key {
        chacha20poly1305_ietf::Key::new([
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8
        ])
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
            "retrieveType": type_,
            "retrieveValue": value,
            "retrieveTags": tags,
        }).to_string()
    }

    fn _search_options(records: bool, total_count: bool, type_: bool, value: bool, tags: bool) -> String {
        json!({
            "retrieveRecords": records,
            "retrieveTotalCount": total_count,
            "retrieveType": type_,
            "retrieveValue": value,
            "retrieveTags": tags,
        }).to_string()
    }

    fn _fetch_all<'a>(iterator: &mut WalletIterator) -> Vec<WalletRecord> {
        let mut v = Vec::new();

        loop {
            if let Some(record) = iterator.next().unwrap() {
                v.push(record);
            } else {
                break;
            }
        }

        _sort(v)
    }

    fn _sort(mut v: Vec<WalletRecord>) -> Vec<WalletRecord> {
        v.sort();
        v
    }
}
