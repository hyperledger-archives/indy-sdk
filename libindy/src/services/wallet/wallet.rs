extern crate sodiumoxide;

use std::collections::HashMap;
use std::rc::Rc;

use utils::crypto::chacha20poly1305_ietf::{TAG_LENGTH, KEY_LENGTH, NONCE_LENGTH, ChaCha20Poly1305IETF,ChaCha20Poly1305IETFKey};
use utils::crypto::hmacsha256::{HMACSHA256, HMACSHA256Key};

use errors::wallet::WalletError;
use errors::common::CommonError;

use super::storage;
use super::iterator::WalletIterator;
use super::encryption::*;
use super::query_encryption::encrypt_query;
use super::language;
use super::WalletRecord;
use self::sodiumoxide::utils::memzero;


pub(super) struct Keys {
   pub type_key: ChaCha20Poly1305IETFKey,
   pub name_key: ChaCha20Poly1305IETFKey,
   pub value_key: ChaCha20Poly1305IETFKey,
   pub item_hmac_key: HMACSHA256Key,
   pub tag_name_key: ChaCha20Poly1305IETFKey,
   pub tag_value_key: ChaCha20Poly1305IETFKey,
   pub tags_hmac_key: HMACSHA256Key,
}

impl Keys {
    pub fn new(mut keys_bytes: Vec<u8>) -> Result<Keys, WalletError> {
        if keys_bytes.len() != KEY_LENGTH * 7 {
            return Err(WalletError::CommonError(
                CommonError::InvalidState(format!("Keys vector is of invalid length"))
            ));
        }
        let keys = Keys {
            type_key: ChaCha20Poly1305IETF::clone_key_from_slice(&keys_bytes[0..32]),
            name_key: ChaCha20Poly1305IETF::clone_key_from_slice(&keys_bytes[32..64]),
            value_key: ChaCha20Poly1305IETF::clone_key_from_slice(&keys_bytes[64..96]),
            item_hmac_key: HMACSHA256::clone_key_from_slice(&keys_bytes[96..128]),
            tag_name_key: ChaCha20Poly1305IETF::clone_key_from_slice(&keys_bytes[128..160]),
            tag_value_key: ChaCha20Poly1305IETF::clone_key_from_slice(&keys_bytes[160..192]),
            tags_hmac_key: HMACSHA256::clone_key_from_slice(&keys_bytes[192..224]),
        };
        memzero(&mut keys_bytes[..]);

        Ok(keys)
    }

    pub fn gen_keys(master_key: &ChaCha20Poly1305IETFKey) -> Vec<u8>{
        let type_key = ChaCha20Poly1305IETF::generate_key();
        let name_key = ChaCha20Poly1305IETF::generate_key();
        let value_key = ChaCha20Poly1305IETF::generate_key();
        let item_hmac_key = HMACSHA256::generate_key();
        let tag_name_key = ChaCha20Poly1305IETF::generate_key();
        let tag_value_key = ChaCha20Poly1305IETF::generate_key();
        let tags_hmac_key = HMACSHA256::generate_key();

        let mut keys: Vec<u8> = Vec::new();
        keys.extend_from_slice(&type_key.get_bytes());
        keys.extend_from_slice(&name_key.get_bytes());
        keys.extend_from_slice(&value_key.get_bytes());
        keys.extend_from_slice(&item_hmac_key.get_bytes());
        keys.extend_from_slice(&tag_name_key.get_bytes());
        keys.extend_from_slice(&tag_value_key.get_bytes());
        keys.extend_from_slice(&tags_hmac_key.get_bytes());

        let encrypted_keys = encrypt_as_not_searchable(&keys, master_key);
        memzero(&mut keys[..]);

        encrypted_keys
    }

    pub fn encrypt(&self, master_key: &ChaCha20Poly1305IETFKey) -> Vec<u8> {
        let mut keys = Vec::new();
        keys.extend_from_slice(self.type_key.get_bytes());
        keys.extend_from_slice(self.name_key.get_bytes());
        keys.extend_from_slice(self.value_key.get_bytes());
        keys.extend_from_slice(self.item_hmac_key.get_bytes());
        keys.extend_from_slice(self.tag_name_key.get_bytes());
        keys.extend_from_slice(self.tag_value_key.get_bytes());
        keys.extend_from_slice(self.tags_hmac_key.get_bytes());

        let encrypted_keys = encrypt_as_not_searchable(&keys, master_key);
        memzero(&mut keys[..]);

        encrypted_keys
    }
}


#[derive(Deserialize, Debug)]
pub struct WalletRuntimeConfig {}

pub(super) struct Wallet {
    name: String,
    pool_name: String,
    storage: Box<storage::WalletStorage>,
    keys: Rc<Keys>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedValue {
    pub data: Vec<u8>,
    pub key: Vec<u8>
}


const ENCRYPTED_KEY_LEN: usize = TAG_LENGTH + NONCE_LENGTH + KEY_LENGTH;

impl EncryptedValue {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        Self {
            data: data,
            key: key,
        }
    }

    pub fn encrypt(data: &str, key: &ChaCha20Poly1305IETFKey) -> Self {
        let value_key = ChaCha20Poly1305IETF::generate_key();
        EncryptedValue::new(
            encrypt_as_not_searchable(data.as_bytes(), &value_key),
            encrypt_as_not_searchable(value_key.get_bytes(), key)
        )
    }

    pub fn decrypt(&self, key: &ChaCha20Poly1305IETFKey) -> Result<String, WalletError> {
        let mut value_key_bytes = decrypt_merged(&self.key, key)?;
        if value_key_bytes.len() != KEY_LENGTH {
            return Err(WalletError::EncryptionError("Value key is not right size".to_string()));
        }
        let value_key = ChaCha20Poly1305IETF::clone_key_from_slice(&value_key_bytes[..]);
        memzero(&mut value_key_bytes[..]);
        String::from_utf8(decrypt_merged(&self.data, &value_key)?)
            .map_err(|_| WalletError::CommonError(CommonError::InvalidStructure("Invalid UTF8 string inside of value".to_string())))
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
        Ok(EncryptedValue{data: value, key: value_key})
    }
}

impl Wallet {
    pub fn new(name: &str, pool_name: &str, storage: Box<storage::WalletStorage>, keys: Rc<Keys>) -> Wallet {
        Wallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            storage: storage,
            keys: keys,
        }
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

    pub(super) fn rotate_key(&self, new_master_key: &ChaCha20Poly1305IETFKey) -> Result<(), WalletError> {
        let new_metadata = self.keys.encrypt(new_master_key);
        self.storage.set_storage_metadata(&new_metadata)?;
        Ok(())
    }

    pub fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub (super) fn get_all(&self) -> Result<WalletIterator, WalletError> {
        let all_items = self.storage.get_all()?;
        Ok(WalletIterator::new(all_items, Rc::clone(&self.keys)))
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use std;
    use std::env;
    use std::rc::Rc;
    use errors::wallet::WalletError;
    use services::wallet::wallet::Wallet;
    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::language::*;
    use std::collections::HashMap;
    use super::*;

    type Tags = HashMap<String, String>;

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
        std::fs::remove_dir_all(_wallet_path()).ok();
        std::fs::create_dir(_wallet_path()).ok();
    }

    fn _credentials() -> String {
        r##"{"master_key": "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=\n", "storage_credentials": {}}}"##.to_string()
    }

    fn _create_wallet() -> Wallet {
        let name = "test_wallet";
        let pool_name = "test_pool";
        let storage_type = SQLiteStorageType::new();
        let master_key = _get_test_master_key();
        storage_type.create_storage("test_wallet", None, "", &Keys::gen_keys(&master_key)).unwrap();
        let credentials = _credentials();
        let storage = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();

        let keys = Keys::new(
            decrypt_merged(
                &storage.get_storage_metadata().unwrap(),
                &master_key
            ).unwrap()
        ).unwrap();

        Wallet::new(name, pool_name, storage, Rc::new(keys))
    }

    fn _get_test_master_key() -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETF::clone_key_from_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ][..])
    }

    fn _get_test_new_master_key() -> ChaCha20Poly1305IETFKey {
        ChaCha20Poly1305IETF::clone_key_from_slice(&[
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8,
            2, 2, 3, 4, 5, 6, 7, 8
        ][..])
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        let mut map = HashMap::new();
        map.insert("retrieveType", type_);
        map.insert("retrieveValue", value);
        map.insert("retrieveTags", tags);
        serde_json::to_string(&map).unwrap()
    }

    fn _search_options(records: bool, total_count: bool, type_: bool, value: bool, tags: bool) -> String {
        let mut map = HashMap::new();
        map.insert("retrieveRecords", records);
        map.insert("retrieveTotalCount", total_count);
        map.insert("retrieveType", type_);
        map.insert("retrieveValue", value);
        map.insert("retrieveTags", tags);
        serde_json::to_string(&map).unwrap()
    }

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

    fn _search_iterator_to_vector<'a>(mut iterator: WalletIterator) -> Vec<(String, String)> {
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
        let tags: HashMap<String, String>= serde_json::from_str(r#"{"tag1": "tag_value_1"}"#).unwrap();
        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        assert_eq!(entity.tags.unwrap(), tags);
    }

    #[test]
    fn wallet_set_get_works_for_reopen() {
        _cleanup();
        let mut wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let tags: HashMap<String, String>= serde_json::from_str(r#"{"tag1": "tag_value_1"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        assert_eq!(entity.tags.unwrap(), tags);

        wallet.close().unwrap();

        let storage_type = SQLiteStorageType::new();
        let credentials = _credentials();
        let storage = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();
        let keys = Keys::new(
            decrypt_merged(// DARKO
                         &storage.get_storage_metadata().unwrap(),
                    &_get_test_master_key()
            ).unwrap()
        ).unwrap();
        let wallet = Wallet::new("test_wallet", "test_pool", storage, Rc::new(keys));

        let entity = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        assert_eq!(entity.tags.unwrap(), tags);
    }

    #[test]
    fn wallet_get_returns_item_not_found_error_for_unknown() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";

        let res = wallet.get(type_, "wrong_name", &_fetch_options(false, true, true));

        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_cannot_add_twice_the_same_key() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let tags: HashMap<String, String>= serde_json::from_str(r#"{"tag1": "tag_value_1"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        let res = wallet.add(type_, name, "different_value", &tags);

        assert_match!(Err(WalletError::ItemAlreadyExists), res);
    }

    /**

     * Update tests
    */
    #[test]
    fn wallet_update() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags: HashMap<String, String>= serde_json::from_str(r#"{}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        wallet.get(type_, name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": false}"##).unwrap();
        wallet.update(type_, name, new_value).unwrap();
        let item = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();
        assert_eq!(item.name, String::from(name));
        assert_eq!(item.value.unwrap(), String::from(new_value));
    }

    #[test]
    fn wallet_update_returns_error_if_wrong_name() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name";
        let wrong_name = "wrong_name";
        let value = "value";
        let new_value = "new_value";
        let tags: HashMap<String, String>= serde_json::from_str(r#"{"tag1":"value1", "tag2":"value2", "~tag3":"value3"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        wallet.get(type_, name, &_fetch_options(false, true, false)).unwrap();
        let res = wallet.update(type_, wrong_name, new_value);
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_update_returns_error_if_wrong_type() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let wrong_type = "wrong_type";
        let name = "name";
        let value = "value";
        let new_value = "new_value";
        let tags: HashMap<String, String>= serde_json::from_str(r#"{"tag1":"value1", "tag2":"value2", "~tag3":"value3"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        wallet.get(type_, name, &_fetch_options(false, true, false)).unwrap();
        let res = wallet.update(wrong_type, name, new_value);
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    /**
     * Add tags tests
     */
    #[test]
    fn wallet_add_tags_() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name";
        let value = "value";
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();

        let new_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_2": "tag_value_2", "~tag_name_3": "~tag_value_3"}"#).unwrap();
        wallet.add_tags(type_, name, &new_tags).unwrap();

        let item = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();
        let tags = item.tags.unwrap();
        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_3": "~tag_value_3"}"#).unwrap();

        assert_eq!(expected_tags, tags);
    }

    /**
     * Update tags tests
     */
    #[test]
    fn wallet_update_tags() {
        _cleanup();

        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name";
        let value = "value";
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_3": "~tag_value_3"}"#).unwrap();
        wallet.add(type_, name, value, &tags).unwrap();

        let updated_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "new_tag_value_1", "tag_name_2": "new_tag_value_2"}"#).unwrap();
        wallet.update_tags(type_, name, &updated_tags).unwrap();

        let item = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();
        let retrieved_tags = item.tags.unwrap();

        assert_eq!(updated_tags, retrieved_tags);
    }

    /**
     * Delete tags tests
     */
    #[test]
    fn wallet_delete_tags() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name";
        let value = "value";
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_3": "~tag_value_3"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();

        let tag_names = vec!["tag_name_1", "~tag_name_3"];
        wallet.delete_tags(type_, name, &tag_names[..]).unwrap();

        let item = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();
        let retrieved_tags = item.tags.unwrap();
        let expected_tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_2": "tag_value_2"}"#).unwrap();

        assert_eq!(expected_tags, retrieved_tags);
        wallet.delete_tags(type_, name, &tag_names).unwrap();
    }

    #[test]
    fn wallet_delete_works() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";
        let name = "name1";
        let value = "value1";
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1"}"#).unwrap();

        wallet.add(type_, name, value, &tags).unwrap();
        let entity = wallet.get(type_, name, &_fetch_options(false, true, true)).unwrap();

        assert_eq!(entity.name, name);
        assert_eq!(entity.value.unwrap(), value);
        assert_eq!(entity.tags.unwrap(), tags);

        wallet.delete(type_, name).unwrap();
        let res = wallet.get(type_, name, &_fetch_options(false, true, true));
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn wallet_delete_returns_item_not_found_if_no_such_item() {
        _cleanup();
        let wallet = _create_wallet();
        let type_ = "test";

        let res = wallet.delete(type_, "nonexistent_name");
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
        let master_key = _get_test_master_key();
        let column_keys = Keys::gen_keys(&master_key);
        let name = "test_wallet";
        let pool_name = "test_pool";
        let storage_type = SQLiteStorageType::new();
        let master_key = _get_test_master_key();
        storage_type.create_storage("test_wallet", None, "", &Keys::gen_keys(&master_key)).unwrap();
        let credentials = _credentials();
        let storage = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();

        let keys = Keys::new(
            decrypt_merged(
                &storage.get_storage_metadata().unwrap(),
                &master_key
            ).unwrap()
        ).unwrap();
        let raw_query = serde_json::to_string(&test_query).unwrap();
        let query = language::parse_from_json(&raw_query).unwrap();
        let encrypted_query = encrypt_query(query, &keys).unwrap();

        assert_match!(Operator::And(_), encrypted_query);
    }

    /// Search testing ///
    // eq tests //
    #[test]
    fn wallet_search_empty_query() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tag2"}"#).unwrap();

        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, true, false);

        // successful encrypted search
        let query_json = "{}";
        let mut iterator = wallet.search("test_type_", query_json, Some(fetch_options)).unwrap();

        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());

        let res = iterator.next().unwrap();
        assert!(res.is_none());

        let total_count = iterator.get_total_count().unwrap();
        assert_eq!(total_count, None); // because it is not requested.
    }

    #[test]
    fn wallet_search_empty_query_with_count() {
        _cleanup();

        let type_ = "test_type_";
        let name = "foo";
        let value = "bar";
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tag_value1"}"#).unwrap();

        let wallet = _create_wallet();

        wallet.add(type_, name, value, &tags).unwrap();
        let fetch_options = &_search_options(true, true, true, true, true);

        // successful encrypted search
        let query_json = "{}";
        let mut iterator = wallet.search("test_type_", query_json, Some(fetch_options)).unwrap();

        let res = iterator.next().unwrap().unwrap();

        let expected = WalletRecord{
            name: name.to_string(),
            value: Some(value.to_string()),
            tags: Some(tags.clone()),
            type_: Some(type_.to_string()),
        };
        assert_eq!(res, expected);

        let res = iterator.next().unwrap();
        assert!(res.is_none());

        let total_count = iterator.get_total_count().unwrap();
        assert_eq!(total_count, Some(1));
    }

    #[test]
    fn wallet_search_empty_query_only_count() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tags2"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(false, true, false, true, false);

        // successful encrypted search
        let query_json = "{}";
        let mut iterator = wallet.search("test_type_", query_json, Some(fetch_options)).unwrap();

        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // repeated next call should return None again
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        let total_count = iterator.get_total_count().unwrap();
        assert_eq!(total_count, Some(1));
    }

    #[test]
    fn wallet_search_single_item_eqencrypted() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tag2"}"#).unwrap();

        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, true, false);

        // successful encrypted search
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag3": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag1": "tag3"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_unencrypted_tag_name_empty() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tag2"}"#).unwrap();

        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, true, false);

        // successful encrypted search
        let query_json = jsonise!({
            "tag1": "tag2",
            "~": "tag3",
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));
        assert_match!(Err(WalletError::QueryError(_)), res)
    }

    #[test]
    fn wallet_search_returns_error_if_encrypted_tag_name_empty() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag1":"tag2"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, true, false);

        // successful encrypted search
        let query_json = jsonise!({
            "tag1": "tag2",
            "": "tag3",
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));
        assert_match!(Err(WalletError::QueryError(_)), res)
    }

    #[test]
    fn wallet_search_single_item_eq_plain() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"~tag1":"tag2"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful plain search
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different tag name
        let query_json = jsonise!({
            "~tag3": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different tag value
        let query_json = jsonise!({
            "~tag1": "tag3"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different type_
        let query_json = jsonise!({
            "~tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search equal name value
        let query_json = jsonise!({
            "tag1": "tag2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    // neq tests //
    #[test]
    fn wallet_search_single_item_neq_encrypted() {
        _cleanup();
        let fetch_options = &_fetch_options(false, true, false);
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name":"tag_value"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$neq": "different_tag_value"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with matching value
        let query_json = jsonise!({
            "tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with neq value but eq name
        let query_json = jsonise!({
            "different_tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": {"$neq": "target_tag_value"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search
        let query_json = jsonise!({
            "~tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_single_item_neq_plain() {
        _cleanup();
        let fetch_options = &_search_options(true, false,false, true, false);
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name":"tag_value"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();

        // successful plain search
        let query_json = jsonise!({
            "~tag_name": {"$neq": "different_tag_value"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with eq value and eq name
        let query_json = jsonise!({
            "~tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with neq value but neq name
        let query_json = jsonise!({
            "~different_tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$neq": "target_tag_value"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$neq": "different_tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    // gt tests //
    #[test]
    fn wallet_search_single_item_gt_unencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name":"1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name":"2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name":"3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$gt": "1"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$gt": "4"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$gt": "1"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_gt_used_with_encrypted_tag() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$gt": "1"},
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    // gte tests //
    #[test]
    fn wallet_search_single_item_gte_unencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$gte": "2"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$gte": "4"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$neq": "tag_value"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$gte": "1"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_gte_used_with_encrypted_tag() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$gte": "1"},
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }


    // lt tests //
    #[test]
    fn wallet_search_single_item_lt_unencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$lt": "3"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$lt": "1"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$lt": "2"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$lt": "2"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_lt_used_with_encrypted_tag() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$lt": "1"},
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    // lte tests //
    #[test]
    fn wallet_search_single_item_lte_unencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "~tag_name": {"$lte": "2"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$lte": "0"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexisting_tag_name": {"$lte": "2"}
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "~tag_name": {"$lte": "2"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_lte_used_with_encrypted_tag() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$lte": "1"},
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    // like tests //
    #[test]
    fn wallet_search_like() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "tag_value_1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "tag_value_2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "not_matching"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful unencrypted search
        let query_json = jsonise!({
            "~tag_name": {"$like": "tag_value_%"},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo2".to_string(), "bar2".to_string())));

        // unsuccessful unencrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$like": "tag_value_no_match%"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexistent_tag_name": {"$like": "tag_value_%"},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful search wrong type_
        let query_json = jsonise!({
            "~tag_name": {"$like": "tag_value_no_match%"},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_returns_error_if_like_used_with_encrypted_tag() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$like": "1"},
        });
        let res = wallet.search("test_type_", &query_json, Some(fetch_options));

        assert_match!(Err(WalletError::QueryError(_)), res);
    }

    // in tests //
    #[test]
    fn wallet_search_single_item_in_unencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "tag_value_1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "tag_value_2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"~tag_name": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful unencrypted search
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful unencrypted search with no matches
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_4", "tag_value_5"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search with nonexisting value
        let query_json = jsonise!({
            "~nonexistent_tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search wrong type_
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_single_item_inencrypted() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"tag_name": "tag_value_1"}"#).unwrap();
        wallet.add("test_type_", "foo1", "bar1", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"tag_name": "tag_value_2"}"#).unwrap();
        wallet.add("test_type_", "foo2", "bar2", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"tag_name": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "foo3", "bar3", &tags_3).unwrap();

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("foo1".to_string(), "bar1".to_string())));
        assert!(results.contains(&("foo3".to_string(), "bar3".to_string())));

        // unsuccessful encrypted search with no matches
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_4", "tag_value_5"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with nonexisting value
        let query_json = jsonise!({
            "nonexistent_tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful unencrypted search
        let query_json = jsonise!({
            "~tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search wrong type_
        let query_json = jsonise!({
            "tag_name": {"$in": ["tag_value_1", "tag_value_3"]},
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }


    // and tests
    #[test]
    fn wallet_search_and_with_eqs() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_2": "tag_value_2", "~tag_name_3": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_2": "tag_value_3", "~tag_name_3": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "spam", "eggs", &tags_2).unwrap();

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "tag_name_2": "tag_value_2",
            "~tag_name_2": "tag_value_2",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&("foo".to_string(), "bar".to_string())));

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&("spam".to_string(), "eggs".to_string())));

        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_3": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
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
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong type
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "~tag_name_2": "tag_value_2",
        });
        let iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong tag name
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1",
            "tag_name_3": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);

        // wrong tag value
        let query_json = jsonise!({
            "tag_name_1": "tag_value_0",
            "~tag_name_2": "tag_value_3",
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let results = _search_iterator_to_vector(iterator);
        assert_eq!(results.len(), 0);
    }

    // or tests
    #[test]
    fn wallet_search_or_with_eqs() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "~tag_name_2": "tag_value_21", "~tag_name_3": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "~tag_name_2": "tag_value_22", "~tag_name_3": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "spam", "eggs", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "~tag_name_3": "tag_value_3", "~tag_name_4": "tag_value_4"}"#).unwrap();
        wallet.add("test_type_", "ping", "pong", &tags_3).unwrap();

        // All 3
        let query_json = jsonise!({
            "$or": [
                {"tag_name_1": "tag_value_1"},
                {"~tag_name_2": "tag_value_22"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String, String>::new();
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
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String, String>::new();
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
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let mut expected_values = HashMap::<String, String>::new();
        expected_values.insert("ping".to_string(), "pong".to_string());
        assert_eq!(values, expected_values);

        // no matching
        let query_json = jsonise!({
            "$or": [
                {"tag_name_2": "tag_value_21"},
                {"~tag_name_4": "tag_value_5"}
            ]
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let expected_values = HashMap::<String, String>::new();
        assert_eq!(values, expected_values);

        // no matching - wrong type_
        let query_json = jsonise!({
            "$or": [
                {"tag_name_1": "tag_value_1"},
                {"~tag_name_2": "tag_value_22"},
                {"~tag_name_4": "tag_value_4"}
            ]
        });
        let iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        let expected_values = HashMap::<String, String>::new();
        assert_eq!(values, expected_values);
    }

    // not tests
    #[test]
    fn wallet_search_not_simple() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();

        let tags_1: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "~tag_name_2": "tag_value_21", "~tag_name_3": "tag_value_3"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags_1).unwrap();

        let tags_2: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_12": "tag_value_12", "~tag_name_2": "tag_value_22"}"#).unwrap();
        wallet.add("test_type_", "spam", "eggs", &tags_2).unwrap();

        let tags_3: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_13": "tag_value_13", "~tag_name_4": "tag_value_4"}"#).unwrap();
        wallet.add("test_type_", "ping", "pong", &tags_3).unwrap();

        let query_json = jsonise!({
            "$not": {"tag_name_1": "tag_value_1_different"}
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 3);
        let expected_values = HashMap::<String, String>::new();
        assert_eq!(values.get("foo").unwrap(), "bar");

        let query_json = jsonise!({
            "$not": {"~tag_name_2": "tag_value_22"}
        });
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 2);
        let expected_values = HashMap::<String, String>::new();
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
        let iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let values = _search_iterator_to_map(iterator);
        assert_eq!(values.len(), 0);
    }

    #[test]
    fn wallet_search_without_value() {
        _cleanup();
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name": "tag_value"}"#).unwrap();
        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, false, false);

        // successful encrypted searchF
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert!(res.value.is_none());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag_name_2": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag_name": "tag_value_2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_with_tags() {
        _cleanup();
        let wallet = _create_wallet();

        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tag_name_1": "tag_value_1", "tag_name_2": "tag_value_2", "~tag_name_1": "tag_value_1", "*tag_name_2": "tag_value_2"}"#).unwrap();

        wallet.add("test_type_", "foo", "bar", &tags).unwrap();
        let fetch_options = &_search_options(true, false, false, true, true);

        // successful encrypted search
        let query_json = jsonise!({
            "tag_name_1": "tag_value_1"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar");
        assert_eq!(res.tags.unwrap(), tags.clone());
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag name
        let query_json = jsonise!({
            "tag_name_2": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different tag value
        let query_json = jsonise!({
            "tag_name": "tag_value_2"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful encrypted search with different type_
        let query_json = jsonise!({
            "tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type__wrong", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());

        // unsuccessful plain search equal name value
        let query_json = jsonise!({
            "~tag_name": "tag_value"
        });
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn wallet_search_nested_query() {
        _cleanup();
        let fetch_options = &_search_options(true, false, false, true, false);
        let wallet = _create_wallet();
        let tags: HashMap<String, String> = serde_json::from_str(r#"{"tags1": "tags2"}"#).unwrap();
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
        let mut iterator = wallet.search("test_type_", &query_json, Some(fetch_options)).unwrap();
        let res = iterator.next().unwrap().unwrap();
        assert_eq!(res.name, "foo".to_string());
        assert_eq!(res.value.unwrap(), "bar".to_string())
    }
}
