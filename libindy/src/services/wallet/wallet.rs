use std::collections::HashMap;
use std::io::{Write,Read};

use serde_json;
use sodiumoxide::crypto::aead::xchacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;

use errors::wallet::WalletError;

use super::storage;
use super::iterator::WalletIterator;
use super::encryption::*;
use super::query_encryption::encrypt_query;
use super::language;
use super::WalletRecord;


pub(super) type Tags = HashMap<String, String>;


#[derive(Debug, Default)]
pub(super) struct Keys {
   pub type__key: [u8; 32],
   pub name_key: [u8; 32],
   pub value_key: [u8; 32],
   pub item_hmac_key: [u8; 32],
   pub tag_name_key: [u8; 32],
   pub tag_value_key: [u8; 32],
   pub tags_hmac_key: [u8; 32]
}


impl Keys {
    pub fn new(keys_vector: Vec<u8>) -> Keys {
        let mut keys: Keys = Default::default();

        keys.type__key.clone_from_slice(&keys_vector[0..32]);
        keys.name_key.clone_from_slice(&keys_vector[32..64]);
        keys.value_key.clone_from_slice(&keys_vector[64..96]);
        keys.item_hmac_key.clone_from_slice(&keys_vector[96..128]);
        keys.tag_name_key.clone_from_slice(&keys_vector[128..160]);
        keys.tag_value_key.clone_from_slice(&keys_vector[160..192]);
        keys.tags_hmac_key.clone_from_slice(&keys_vector[192..224]);

        return keys;
    }

    pub fn gen_keys(master_key: [u8; 32]) -> Vec<u8>{
        let xchacha20poly1305_ietf::Key(type__key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(item_hmac_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tags_hmac_key) = xchacha20poly1305_ietf::gen_key();

        let mut keys: Vec<u8> = Vec::new();
        keys.extend_from_slice(&type__key);
        keys.extend_from_slice(&name_key);
        keys.extend_from_slice(&value_key);
        keys.extend_from_slice(&item_hmac_key);
        keys.extend_from_slice(&tag_name_key);
        keys.extend_from_slice(&tag_value_key);
        keys.extend_from_slice(&tags_hmac_key);

        return encrypt_as_not_searchable(&keys, master_key);
    }
}


#[derive(Deserialize,Debug)]
pub struct WalletRuntimeConfig {}

impl WalletRuntimeConfig {
    pub fn parse_from_json(json_str: &str) -> Result<WalletRuntimeConfig, WalletError> {
        let config: WalletRuntimeConfig = serde_json::from_str(json_str)?;
        Ok(config)
    }
}

impl Default for WalletRuntimeConfig {
    fn default() -> WalletRuntimeConfig {
        WalletRuntimeConfig {}
    }
}


pub(super) struct Wallet {
    name: String,
    pool_name: String,
    storage: Box<storage::WalletStorage>,
    keys: Keys,
}


impl Wallet {
    pub fn new(name: &str, pool_name: &str, storage: Box<storage::WalletStorage>, keys: Keys) -> Wallet {
        Wallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            storage: storage,
            keys: keys,
        }
    }

    pub fn add(&self, type_: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        let etype_ = encrypt_as_searchable(type_.as_bytes(), self.keys.type__key, self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);
        let xchacha20poly1305_ietf::Key(value_key) = xchacha20poly1305_ietf::gen_key();
        let evalue = encrypt_as_not_searchable(value.as_bytes(), value_key);
        let evalue_key = encrypt_as_not_searchable(&value_key, self.keys.value_key);

        let etags = encrypt_tags(tags, self.keys.tag_name_key, self.keys.tag_value_key, self.keys.tags_hmac_key);

        self.storage.add(&etype_, &ename, &evalue, &evalue_key, &etags)?;
        Ok(())
    }

    pub fn get(&self, type_: &str, name: &str, options: &str) -> Result<WalletRecord, WalletError> {
        let etype_ = encrypt_as_searchable(type_.as_bytes(), self.keys.type__key, self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);

        let result = self.storage.get(&etype_, &ename, options)?;

        let value = match result.value {
            None => None,
            Some(storage_value) => {
                let value_key = decrypt(&storage_value.key, self.keys.value_key)?;
                if value_key.len() != 32 {
                    return Err(WalletError::EncryptionError("Value key is not right size".to_string()));
                }
                let mut vkey: [u8; 32] = Default::default();
                vkey.copy_from_slice(&value_key);
                Some(String::from_utf8(decrypt(&storage_value.data, vkey)?)?)
            }
        };

        let tags = match decrypt_tags(&result.tags, self.keys.tag_name_key, self.keys.tag_value_key)? {
            None => None,
            Some(tags) => Some(serde_json::to_string(&tags)?)
        };

        Ok(WalletRecord::new(String::from(name), Some(type_.to_string()), value, tags))
    }

    pub fn delete(&self, type_: &str, name: &str) -> Result<(), WalletError> {
        let etype_ = encrypt_as_searchable(type_.as_bytes(), self.keys.type__key, self.keys.item_hmac_key);
        let ename = encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);

        self.storage.delete(&etype_, &ename)?;
        Ok(())
    }

    pub fn search<'a>(&'a self, type_: &str, query: &str, options: Option<&str>) -> Result<WalletIterator, WalletError> {
        let parsed_query = language::parse_from_json(query)?;
        let encrypted_query = encrypt_query(parsed_query, &self.keys);
        let encrypted_type_ = encrypt_as_searchable(type_.as_bytes(), self.keys.type__key, self.keys.item_hmac_key);
        let storage_iterator = self.storage.search(&encrypted_type_, &encrypted_query, options)?;
        let wallet_iterator = WalletIterator::new(storage_iterator, &self.keys);
        Ok(wallet_iterator)
    }

    pub fn close(&mut self) -> Result<(), WalletError> {
        self.storage.close()?;
        Ok(())
    }

    pub fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn export(&self, writer: Box<Write>, key: [u8; 32]) -> Result<(), WalletError> {
        unimplemented!()
    }

    fn import(&self, reader: Box<Read>, key: [u8; 32], clear_before: bool) -> Result<(), WalletError> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use std;
    use std::env;
    use errors::wallet::WalletError;
    use services::wallet::wallet::{WalletRecord,Wallet,WalletRuntimeConfig};
    use services::wallet::storage::{WalletStorage,WalletStorageType};
    use services::wallet::storage::default::{SQLiteStorageType};
    use services::wallet::language::*;
    use super::*;


    macro_rules! jsonise {
        ($($x:tt)+) => {
            serde_json::to_string(&json!($($x)+)).unwrap();
        }
    }


    fn _wallet_path() -> std::path::PathBuf {
        let mut path = env::home_dir().unwrap();
        path.push(".indy_client");
        path.push("wallet");
        path.push("test_wallet");
        path
    }


    fn _cleanup() {
        std::fs::remove_dir_all(_wallet_path()).unwrap();
        std::fs::create_dir(_wallet_path()).unwrap();
    }

    fn _credentials() -> String {
        r##"{"master_key": "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=\n", "storage_credentials": {}}}"##.to_string()
    }


    fn _create_wallet() -> Wallet {
        let name = "test_wallet";
        let pool_name = "test_pool";
        let storage_type = SQLiteStorageType::new();
        let master_key = _get_test_master_key();
        storage_type.create_storage("test_wallet", None, "", &Keys::gen_keys(master_key)).unwrap();
        let credentials = _credentials();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();
        Wallet::new(name, pool_name, storage, Keys::new(keys))
    }

    fn _get_test_master_key() -> [u8; 32] {
        return [
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }

    fn _get_test_keys() -> Vec<u8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }
//
//    fn _create_valid_wallet_config_str() -> &'static str {
//        r##"{"storage": {"base": "/tmp"}}"##
//    }
//
//    fn _create_storage_type() -> Box<StorageType> {
//        Box::new(SQLiteStorageType::new())
//    }
//
//    fn _create_storage() -> Box<Storage> {
//        let storage_type = _create_storage_type();
//        let storage = storage_type.create()
//    }
//
//
//    fn _bad_configs_list() -> Vec<&'static str> {
//       return vec![
//        "{}", // empty config
//        "{\"foo\": \"bar\"}", // not a storage config
//        "{\"storage\": {\"foo\": \"bar\"}}", // no base
//        "{\"storage\": {\"base\": \"tmp}}", // wrong format for json
//        // "{\"storage\": {\"base\": \":$%:^&:*`\"}}", // base is not a path
//        // "{\"storage\": {\"base\": \"\"}}", // empty base
//        // "{\"storage\": {\"base\": \"/tmp/../tmp\"}}", // base is a path traversal
//        ]
//    }
//
//
    fn _search_iterator_to_map<'a>(mut iterator: WalletIterator) -> HashMap<String, String> {
        let mut map = HashMap::new();
        loop {
            let res = iterator.next().unwrap();
            if let Some(entity) = res {
                map.insert(entity.name, entity.value.unwrap());
            } else {
                break;
            }
        }

        map
    }

    fn _search_iterator_to_vector<'a>(mut iterator: WalletIterator) -> Vec<(String,String)> {
        let mut v = Vec::new();

        loop {
            let res = iterator.next().unwrap();
            if let Some(entity) = res {
                v.push((entity.name, (entity.value.unwrap())));
            } else {
                break;
            }
        }

        v
    }

//

    #[test]
    fn wallet_add_get_works() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag_value_1".to_string());

        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        let retrieved_tags: Tags = serde_json::from_str(&entity.tags.unwrap()).unwrap();
        assert_eq!(retrieved_tags, tags);
    }
    #[test]
    fn wallet_set_get_works_for_reopen() {
        _cleanup();
        let mut wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag_value_1".to_string());

        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        let retrieved_tags: Tags = serde_json::from_str(&entity.tags.unwrap()).unwrap();
        assert_eq!(retrieved_tags, tags);

        wallet.close().unwrap();

        let storage_type = SQLiteStorageType::new();
        let credentials = _credentials();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();
        let wallet = Wallet::new("test_wallet", "test_pool", storage, Keys::new(keys));

        let entity = wallet.get(type_, name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        let retrieved_tags: Tags = serde_json::from_str(&entity.tags.unwrap()).unwrap();
        assert_eq!(retrieved_tags, tags);
    }

    #[test]
    fn wallet_get_returns_item_not_found_error_for_unknown() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";

        let res = wallet.get(type_, "wrong_name", r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##);

        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_cannot_add_twice_the_same_key() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag_value_1".to_string());

        wallet.add(type_, name, value, &tags).unwrap();
        let res = wallet.add(type_, name, "different_value", &tags);

        assert_match!(Err(WalletError::ItemAlreadyExists), res);
    }

    #[test]
    fn wallet_delete_works() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag_value_1".to_string());

        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        let retrieved_tags: Tags = serde_json::from_str(&entity.tags.unwrap()).unwrap();
        assert_eq!(retrieved_tags, tags);

        wallet.delete(type_, name).unwrap();
        let res = wallet.get(type_, name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_delete_returns_item_not_found_if_no_such_item() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";

        let res = wallet.delete(type_, "nonexistant_name");
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_get_pool_name_works() {
        _cleanup();
        let wallet = _create_wallet();

        assert_eq!(wallet.get_pool_name(), "test_pool");
    }

    #[test]
    fn wallet_get_name_works() {
        _cleanup();
        let wallet = _create_wallet();

        assert_eq!(wallet.get_name(), "test_wallet");
    }

    // query encryption tests
    #[test]
    fn wallet_query_parsing() {
        _cleanup();
        let test_query = json!({
            "k1": "v1",
            "$or": [
                {
                    "k2": {"$like": "like_target"},
                    "k3": {"$gte": "100"},
                    "$not": {
                        "k4": "v4",
                        "k5": {
                            "$regex": "regex_string"
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
        let master_key = _get_test_master_key();
        let column_keys = Keys::gen_keys(master_key);
        let keys = Keys::new(column_keys);
        let raw_query = serde_json::to_string(&test_query).unwrap();
        let query = language::parse_from_json(&raw_query).unwrap();
        let encrypted_query = encrypt_query(query, &keys);

        assert_match!(Operator::And(_), encrypted_query);
    }

    /// Search testing ///
    // eq tests //
    #[test]
    fn wallet_search_single_item_eqencrypted() {
        _cleanup();
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag2".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";

        // successful encrypted search
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag3": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag1": "tag3"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_single_item_eq_plain() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag1".to_string(), "tag2".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful plain search
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different tag name
        let query_json = jsonise!({
            "~tag3": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different tag value
        let query_json = jsonise!({
            "~tag1": "tag3"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different type_
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search equal name value
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    // neq tests //
    #[test]
    fn wallet_search_single_item_neqencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name".to_string(), "tag_value".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$neq": "different_tag_value"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with matching value
        let query_json = jsonise!({
            "tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with neq value but eq name
        let query_json = jsonise!({
            "different_tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": {"$neq": "target_tag_value"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search
        let query_json = jsonise!({
            "~tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

    }

    #[test]
    fn wallet_search_single_item_neq_plain() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "tag_value".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful plain search
        let query_json = jsonise!({
            "~tag_name": {"$neq": "different_tag_value"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with eq value and eq name
        let query_json = jsonise!({
            "~tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with neq value but neq name
        let query_json = jsonise!({
            "~different_tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$neq": "target_tag_value"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

    }

    // gt tests //
    #[test]
    fn wallet_search_single_item_gt_unencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$gt": "1"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$gt": "4"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$gt": "1"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    // gte tests //
    #[test]
    fn wallet_search_single_item_gte_unencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$gte": "2"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$gte": "4"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$gte": "1"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }


    // lt tests //
    #[test]
    fn wallet_search_single_item_lt_unencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$lt": "3"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$lt": "1"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$lt": "2"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$lt": "2"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }


    // lte tests //
    #[test]
    fn wallet_search_single_item_lte_unencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$lte": "2"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$lte": "0"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$lte": "2"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$lte": "2"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }


    // in tests //
    #[test]
    fn wallet_search_single_item_in_unencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("~tag_name".to_string(), "tag_value_1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "tag_value_2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("~tag_name".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful unencrypted search
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful unencrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_4", "tag_value_5"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexistant_tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search wrong type_
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_single_item_inencrypted() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name".to_string(), "tag_value_1".to_string());
        wallet.add("test_type_", "foo1", "bar1", &tags).unwrap();
        tags.insert("tag_name".to_string(), "tag_value_2".to_string());
        wallet.add("test_type_", "foo2", "bar2", &tags).unwrap();
        tags.insert("tag_name".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "foo3", "bar3", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_4", "tag_value_5"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "nonexistant_tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search wrong type_
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }


    // and tests
    #[test]
    fn wallet_search_and_with_eqs() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("~tag_name_2".to_string(), "tag_value_2".to_string());
        tags.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        tags.insert("~tag_name_2".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "spam", "eggs", &tags).unwrap();

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_2",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&("foo".to_string(), "bar".to_string())));

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&("spam".to_string(), "eggs".to_string())));

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_3": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("spam".to_string(), "eggs".to_string())));
        assert!(results.contains(&("foo".to_string(), "bar".to_string())));

        // no matches
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_3": "tag_value_3",
            "tag_name_4": "tag_value_4",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong type_
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_2",
        });
        let iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong tag type
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong tag value
        let query_json = jsonise!({
            "tag_name_1": "tag_value_0",
            "~tag_name_2": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);
    }

    // or tests
    #[test]
    fn wallet_search_or_with_eqs() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("~tag_name_2".to_string(), "tag_value_21".to_string());
        tags.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        tags.insert("~tag_name_2".to_string(), "tag_value_22".to_string());
        wallet.add("test_type_", "spam", "eggs", &tags).unwrap();
        tags.insert("~tag_name_4".to_string(), "tag_value_4".to_string());
        tags.remove("~tag_name_2");
        wallet.add("test_type_", "ping", "pong", &tags).unwrap();

        // All 3
        let query_json = jsonise!({
            "$or": [
                {"tag_name_1": "tag_value_1"},
                {"~tag_name_2": "tag_value_22"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String,String>::new();
        expected_values.insert("foo".to_string(), "bar".to_string());
        expected_values.insert("spam".to_string(), "eggs".to_string());
        expected_values.insert("ping".to_string(), "pong".to_string());
        assert_eq!(values, expected_values);

        // 1 and 3 matching
        let query_json = jsonise!({
            "$or": [
                {"~tag_name_2": "tag_value_21"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String,String>::new();
        expected_values.insert("foo".to_string(), "bar".to_string());
        expected_values.insert("ping".to_string(), "pong".to_string());
        assert_eq!(values, expected_values);

        // 3 matching, 1 not because wrong tag type
        let query_json = jsonise!({
            "$or": [
                {"tag_name_2": "tag_value_21"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String,String>::new();
        expected_values.insert("ping".to_string(), "pong".to_string());
        assert_eq!(values, expected_values);

        // no matching
        let query_json = jsonise!({
            "$or": [
                {"tag_name_2": "tag_value_21"},
                {"~tag_name_4": "tag_value_5"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String,String>::new();
        assert_eq!(values, expected_values);

        // no matching - wrong type_
        let query_json = jsonise!({
            "$or": [
                {"tag_name_1": "tag_value_1"},
                {"~tag_name_2": "tag_value_22"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String,String>::new();
        assert_eq!(values, expected_values);
    }

    // not tests
    #[test]
    fn wallet_search_not_simple() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("~tag_name_2".to_string(), "tag_value_21".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        wallet.add("test_type_", "foo", "bar", &tags1).unwrap();
        let mut tags2 = HashMap::new();
        tags2.insert("~tag_name_2".to_string(), "tag_value_22".to_string());
        wallet.add("test_type_", "spam", "eggs", &tags2).unwrap();
        let mut tags3 = HashMap::new();
        tags3.insert("~tag_name_4".to_string(), "tag_value_4".to_string());
        wallet.add("test_type_", "ping", "pong", &tags3).unwrap();

        let query_json = jsonise!({
            "$not": {"tag_name_1": "tag_value_1_different"}
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 1);
        let expected_values = HashMap::<String,String>::new();
        assert_eq!(values.get("foo").unwrap(), "bar");

        let query_json = jsonise!({
            "$not": {"~tag_name_2": "tag_value_22"}
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 2);
        let expected_values = HashMap::<String,String>::new();
        assert_eq!(values.get("foo").unwrap(), "bar");
        assert_eq!(values.get("ping").unwrap(), "pong");

        let query_json = jsonise!({
            "$not": {
                "$or": [
                    {"tag_name_1": "tag_value_1"},
                    {"~tag_name_2": "tag_value_22"},
                    {"~tag_name_4": "tag_value_4"},
                ]
            }
        });
        let iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 0);
    }

     #[test]
    fn wallet_search_without_value() {
        _cleanup();
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name".to_string(), "tag_value".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": false, \"fetch_tags\": false}";

        // successful encrypted searchF
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert!(res.value.is_none());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag_name_2": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag_name": "tag_value_2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

     #[test]
    fn wallet_search_with_tags() {
        _cleanup();
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags.insert("~tag_name_1".to_string(), "tag_value_1".to_string());
        tags.insert("*tag_name_2".to_string(), "tag_value_2".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": true}";

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1"
        });
        let mut iterator = wallet.search ("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar");
        let mut expected_tags = HashMap::new();
        expected_tags.insert(String::from("tag_name_1"), String::from("tag_value_1"));
        expected_tags.insert(String::from("tag_name_2"), String::from("tag_value_2"));
        expected_tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        expected_tags.insert(String::from("*tag_name_2"), String::from("tag_value_2"));
        let retrieved_tags: Tags = serde_json::from_str(&res.tags.unwrap()).unwrap();
        assert_eq!(retrieved_tags, expected_tags);
        let res = iterator.next().unwrap();
        assert!(res.is_none());

         // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag_name_2": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag_name": "tag_value_2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_nested_query() {
        _cleanup();
        let search_config = "{\"fetch_type\": false, \"fetch_value\": true, \"fetch_tags\": false}";
        let wallet = _create_wallet();
        let mut tags = HashMap::new();
        tags.insert("tag1".to_string(), "tag2".to_string());
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let query_json = jsonise!({
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
        let mut iterator = wallet.search("test_type_", &query_json, Some(search_config)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string())
    }
}
