extern crate libc;
extern crate indy_crypto;

mod storage;
mod encryption;

use self::default::DefaultWalletType;
use self::plugged::PluggedWalletType;

use api::ErrorCode;
use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use self::libc::c_char;

use self::storage::StorageType;

#[derive(Debug, Default)]
struct Keys {
   class_key: [u8; 32],
   name_key: [u8; 32],
   value_key: [u8; 32],
   item_hmac_key: [u8; 32],
   tag_name_key: [u8; 32],
   tag_value_key: [u8; 32],
   tags_hmac_key: [u8; 32]
}

impl Keys {
    fn new(keys_vector: Vec<u8>) -> Keys {
        let mut keys: Keys = Default::default();

        keys.class_key.clone_from_slice(&keys_vector[0..32]);
        keys.name_key.clone_from_slice(&keys_vector[32..64]);
        keys.value_key.clone_from_slice(&keys_vector[64..96]);
        keys.item_hmac_key.clone_from_slice(&keys_vector[96..128]);
        keys.tag_name_key.clone_from_slice(&keys_vector[128..160]);
        keys.tag_value_key.clone_from_slice(&keys_vector[160..192]);
        keys.tags_hmac_key.clone_from_slice(&keys_vector[192..224]);

        return keys;
    }

    fn gen_keys(master_key: [u8; 32]) -> Vec<u8>{
        let xchacha20poly1305_ietf::Key(class_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(item_hmac_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_name_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tag_value_key) = xchacha20poly1305_ietf::gen_key();
        let xchacha20poly1305_ietf::Key(tags_hmac_key) = xchacha20poly1305_ietf::gen_key();

        let mut keys: Vec<u8> = Vec::new();
        keys.extend_from_slice(&class_key);
        keys.extend_from_slice(&name_key);
        keys.extend_from_slice(&value_key);
        keys.extend_from_slice(&item_hmac_key);
        keys.extend_from_slice(&tag_name_key);
        keys.extend_from_slice(&tag_value_key);
        keys.extend_from_slice(&tags_hmac_key);

        return _encrypt_as_not_searchable(&keys, master_key);
    }
}

pub struct WalletEntity {
    name: String,
    value: Option<String>,
    tags: Option<HashMap<String, String>>,
}

impl WalletEntity {
    fn new(name: &str, value: Option<String>, tags: Option<HashMap<String, String>>) -> WalletEntity {
        WalletEntity {
            name: name.to_string(),
            value: value,
            tags: tags,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_value(&self) -> Result<&str, WalletError> {
        match self.value {
            Some(ref value) => Ok(&value),
            None => Err(WalletError::NotFound)
        }
    }

    fn get_tags(&self) -> Result<&HashMap<String, String>, WalletError> {
        match self.tags {
            Some(ref tags) => Ok(&tags),
            None => Err(WalletError::NotFound)
        }
    }
}

struct Wallet {
    name: String,
    pool_name: String,
    storage: Box<storage::Storage>,
    keys: Keys,
}

impl Wallet {
    fn new(name: &str, pool_name: &str, storage: Box<storage::Storage>, keys: Keys, master_key: [u8; 32]) -> Result<GenericWallet, WalletError> {
        let keys = _decrypt(&encrypted_keys, master_key)?;
        GenericWallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            storage: storage,
            keys: keys,
        }
    }

    fn add(&self, class: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> Result<(), WalletError> {
        let eclass = _encrypt_as_searchable(class.as_bytes(), self.keys.class_key, self.keys.item_hmac_key);
        let ename = _encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);
        let xchacha20poly1305_ietf::Key(value_key) = xchacha20poly1305_ietf::gen_key();
        let evalue = _encrypt_as_not_searchable(value.as_bytes(), value_key);
        let evalue_key = _encrypt_as_not_searchable(&value_key, self.keys.value_key);

        let etags = _encrypt_tags(tags, self.keys.tag_name_key, self.keys.tag_value_key, self.keys.tags_hmac_key);

        self.storage.add(&eclass, &ename, &evalue, &evalue_key, &etags)?;
        Ok(())
    }

    fn get(&self, class: &str, name: &str, options: &str) -> Result<WalletEntity, WalletError> {
        let eclass = _encrypt_as_searchable(class.as_bytes(), self.keys.class_key, self.keys.item_hmac_key);
        let ename = _encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);

        let result = self.storage.get(&eclass, &ename, options)?;

        let value = match result.value {
            None => None,
            Some(storage_value) => {
                let value_key = _decrypt(&storage_value.key, self.keys.value_key)?;
                if value_key.len() != 32 {
                    return Err(WalletError::EncryptionError("Value key is not right size".to_string()));
                }
                let mut vkey: [u8; 32] = Default::default();
                vkey.copy_from_slice(&value_key);
                Some(String::from_utf8(_decrypt(&storage_value.data, vkey)?)?)
            }
        };

        Ok(WalletEntity::new(name, value,_decrypt_tags(&result.tags, self.keys.tag_name_key, self.keys.tag_value_key)?))
    }

    fn delete(&self, class: &str, name: &str) -> Result<(), WalletError> {
        let eclass = _encrypt_as_searchable(class.as_bytes(), self.keys.class_key, self.keys.item_hmac_key);
        let ename = _encrypt_as_searchable(name.as_bytes(), self.keys.name_key, self.keys.item_hmac_key);

        self.storage.delete(&eclass, &ename)?;
        Ok(())
    }

    fn search<'a>(&'a self, class: &str, query: &str, options: Option<&str>) -> Result<Box<WalletIterator + 'a>, WalletError> {
        let parsed_query = language::parse_from_json(query)?;
        let encrypted_query = encrypt_query(parsed_query, &self.keys);
        let encrypted_class = _encrypt_as_searchable(class.as_bytes(), self.keys.class_key, self.keys.item_hmac_key);
        let storage_iterator = self.storage.search(&encrypted_class, &encrypted_query, options)?;
        let wallet_iterator = GenericWalletIterator::new(storage_iterator, &self.keys);
        Ok(Box::new(wallet_iterator))
    }

    fn close(&mut self) -> Result<(), WalletError> {
        self.storage.close()?;
        Ok(())
    }

    fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn export(&self, writer: Box<Write>, key: [u8; 32]) -> Result<(), WalletError> {
        Err(WalletError::NotImplemented)
    }

    fn import(&self, reader: Box<Read>, key: [u8; 32], clear_before: bool) -> Result<(), WalletError> {
        Err(WalletError::NotImplemented)
    }
}

#[derive(Serialize, Deserialize)]
struct WalletDescriptor {
    pool_name: String,
    xtype: String,
    name: String
}

impl WalletDescriptor {
    pub fn new(pool_name: &str, xtype: &str, name: &str) -> WalletDescriptor {
        WalletDescriptor {
            pool_name: pool_name.to_string(),
            xtype: xtype.to_string(),
            name: name.to_string()
        }
    }
}

impl JsonEncodable for WalletDescriptor {}

impl<'a> JsonDecodable<'a> for WalletDescriptor {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct WalletMetadata {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    associated_pool_name: String,
}

impl From<WalletDescriptor> for WalletMetadata {
    fn from(internal: WalletDescriptor) -> Self {
        WalletMetadata {
            name: internal.name,
            type_: internal.xtype,
            associated_pool_name: internal.pool_name,
        }
    }
}

impl JsonEncodable for WalletMetadata {}

impl<'a> JsonDecodable<'a> for WalletMetadata {}

pub struct WalletService {
    types: RefCell<HashMap<String, Box<StorageType>>>,
    wallets: RefCell<HashMap<i32, Box<Wallet>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let mut types: HashMap<String, Box<WalletType>> = HashMap::new();
        types.insert("default".to_string(), Box::new(DefaultWalletType::new()));

        WalletService {
            types: RefCell::new(types),
            wallets: RefCell::new(HashMap::new())
        }
    }

    pub fn register_type(&self,
                         xtype: &str,
                         create: extern fn(name: *const c_char,
                                           config: *const c_char,
                                           credentials: *const c_char) -> ErrorCode,
                         open: extern fn(name: *const c_char,
                                         config: *const c_char,
                                         runtime_config: *const c_char,
                                         credentials: *const c_char,
                                         handle: *mut i32) -> ErrorCode,
                         set: extern fn(handle: i32,
                                        key: *const c_char,
                                        value: *const c_char) -> ErrorCode,
                         get: extern fn(handle: i32,
                                        key: *const c_char,
                                        value_ptr: *mut *const c_char) -> ErrorCode,
                         get_not_expired: extern fn(handle: i32,
                                                    key: *const c_char,
                                                    value_ptr: *mut *const c_char) -> ErrorCode,
                         list: extern fn(handle: i32,
                                         key_prefix: *const c_char,
                                         values_json_ptr: *mut *const c_char) -> ErrorCode,
                         close: extern fn(handle: i32) -> ErrorCode,
                         delete: extern fn(name: *const c_char,
                                           config: *const c_char,
                                           credentials: *const c_char) -> ErrorCode,
                         free: extern fn(wallet_handle: i32,
                                         value: *const c_char) -> ErrorCode) -> Result<(), WalletError> {
        let mut wallet_types = self.types.borrow_mut();

        if wallet_types.contains_key(xtype) {
            return Err(WalletError::TypeAlreadyRegistered(xtype.to_string()));
        }

        wallet_types.insert(xtype.to_string(),
                            Box::new(
                                PluggedWalletType::new(create, open, set, get,
                                                       get_not_expired, list, close, delete, free)));
        Ok(())
    }

    pub fn create(&self, pool_name: &str, xtype: Option<&str>, name: &str, config: Option<&str>,
                  credentials: Option<&str>) -> Result<(), WalletError> {
        let xtype = xtype.unwrap_or("default");

        let storage_types = self.types.borrow();
        if !storage_types.contains_key(xtype) {
            return Err(WalletError::UnknownType(xtype.to_string()));
        }

        let wallet_path = _wallet_path(name);
        if wallet_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()));
        }
        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        let storage_type = storage_types.get(xtype).unwrap();
        storage_type.create(name, config, credentials, &Keys::gen_keys(master_key))?;

        let mut descriptor_file = File::create(_wallet_descriptor_path(name))?;
        descriptor_file
            .write_all({
                WalletDescriptor::new(pool_name, xtype, name)
                    .to_json()?
                    .as_bytes()
            })?;
        descriptor_file.sync_all()?;

        if config.is_some() {
            let mut config_file = File::create(_wallet_config_path(name))?;
            config_file.write_all(config.unwrap().as_bytes())?;
            config_file.sync_all()?;
        }

        Ok(())
    }

    pub fn delete(&self, name: &str, credentials: Option<&str>) -> Result<(), WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }

        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();

        let config = {
            let config_path = _wallet_config_path(name);

            if config_path.exists() {
                let mut config_json = String::new();
                let mut file = File::open(config_path)?;
                file.read_to_string(&mut config_json)?;
                Some(config_json)
            } else {
                None
            }
        };

        wallet_type.delete(name,
                           config.as_ref().map(String::as_str),
                           credentials)?;

        fs::remove_dir_all(_wallet_path(name))?;
        Ok(())
    }

    pub fn open(&self, name: &str, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<i32, WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = WalletDescriptor::from_json({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let storage_types = self.types.borrow();
        if !storage_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }
        let storage_type = storage_types.get(descriptor.xtype.as_str()).unwrap();

        let mut wallets = self.wallets.borrow_mut();
        if wallets.values().any(|ref wallet| wallet.get_name() == name) {
            return Err(WalletError::AlreadyOpened(name.to_string()));
        }

        let config = {
            let config_path = _wallet_config_path(name);

            if config_path.exists() {
                let mut config_json = String::new();
                let mut file = File::open(config_path)?;
                file.read_to_string(&mut config_json)?;
                Some(config_json)
            } else {
                None
            }
        };

        let storage, enc_keys = storage_type.open(name,
                                      descriptor.pool_name.as_str(),
                                      config.as_ref().map(String::as_str),
                                      runtime_config,
                                      credentials)?;

        let wallet = Wallet::new(name, pool_name, storage, enc_keys)?;

        let wallet_handle = SequenceUtils::get_next_id();
        wallets.insert(wallet_handle, wallet);
        Ok(wallet_handle)
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletMetadata>, WalletError> {
        let mut descriptors = Vec::new();
        let wallet_home_path = EnvironmentUtils::wallet_home_path();

        for entry in fs::read_dir(wallet_home_path)? {
            let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue };
            if let Some(wallet_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                let mut descriptor_json = String::new();
                File::open(_wallet_descriptor_path(wallet_name)).ok()
                    .and_then(|mut f| f.read_to_string(&mut descriptor_json).ok())
                    .and_then(|_| WalletDescriptor::from_json(descriptor_json.as_str()).ok())
                    .map(|descriptor| descriptors.push(descriptor.into()));
            }
        }

        Ok(descriptors)
    }

    pub fn close(&self, handle: i32) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().remove(&handle) {
            Some(wallet) => wallet.close(),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn set(&self, handle: i32, key: &str, value: &str) -> Result<(), WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.set(key, value),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn get(&self, handle: i32, key: &str) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.get(key),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn list(&self, handle: i32, key_prefix: &str) -> Result<Vec<(String, String)>, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.list(key_prefix),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn get_not_expired(&self, handle: i32, key: &str) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.get_not_expired(key),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => Ok(wallet.get_pool_name()),
            None => Err(WalletError::InvalidHandle(handle.to_string()))
        }
    }
}

fn _wallet_path(name: &str) -> PathBuf {
    EnvironmentUtils::wallet_path(name)
}

fn _wallet_descriptor_path(name: &str) -> PathBuf {
    _wallet_path(name).join("wallet.json")
}

fn _wallet_config_path(name: &str) -> PathBuf {
    _wallet_path(name).join("config.json")
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;

    use std::time::Duration;
    use std::thread;

    #[test]
    fn wallet_service_new_works() {
        WalletService::new();
    }

    #[test]
    fn wallet_service_register_type_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_create_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_create_works_for_none_type() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_unknown_type() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let res = wallet_service.create("pool1", Some("unknown"), "wallet1", None, None);
        assert_match!(Err(WalletError::UnknownType(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_create_works_for_twice() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let res = wallet_service.create("pool1", None, "wallet1", None, None);
        assert_match!(Err(WalletError::AlreadyExists(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_delete_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.delete("wallet1", None).unwrap();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_delete_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        wallet_service.delete("wallet1", None).unwrap();
        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_open_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_open_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        wallet_service.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_list_wallets_works() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();
        let w1_meta = WalletMetadata {
            name: "w1".to_string(),
            associated_pool_name: "p1".to_string(),
            type_: "default".to_string(),
        };
        let w2_meta = WalletMetadata {
            name: "w2".to_string(),
            associated_pool_name: "p2".to_string(),
            type_: "inmem".to_string(),
        };
        let w3_meta = WalletMetadata {
            name: "w3".to_string(),
            associated_pool_name: "p1".to_string(),
            type_: "default".to_string(),
        };
        wallet_service.create(&w1_meta.associated_pool_name,
                              Some(&w1_meta.type_),
                              &w1_meta.name,
                              None, None).unwrap();
        wallet_service.create(&w2_meta.associated_pool_name,
                              Some(&w2_meta.type_),
                              &w2_meta.name,
                              None, None).unwrap();
        wallet_service.create(&w3_meta.associated_pool_name,
                              None,
                              &w3_meta.name,
                              None, None).unwrap();

        let wallets = wallet_service.list_wallets().unwrap();

        assert!(wallets.contains(&w1_meta));
        assert!(wallets.contains(&w2_meta));
        assert!(wallets.contains(&w3_meta));

        InmemWallet::cleanup();
        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_close_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.close(wallet_handle).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_close_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.close(wallet_handle).unwrap();

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works_for_reopen() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        wallet_service.close(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_works_for_unknown() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let res = wallet_service.get(wallet_handle, "key1");
        assert_match!(Err(WalletError::NotFound(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_works_for_plugged_and_unknown() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let res = wallet_service.get(wallet_handle, "key1");
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), res);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_works_for_update() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_works_for_plugged_and_update() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_not_expired_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 10}"), None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();


        let value = wallet_service.get_not_expired(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_not_expired_works_for_expired() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(2, 0));

        let res = wallet_service.get_not_expired(wallet_handle, "key1");
        assert_match!(Err(WalletError::NotFound(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_set_get_not_expired_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 10}"), None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();

        let value = wallet_service.get_not_expired(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_set_get_not_expired_works_for_plugged_and_expired() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(2, 0));

        let res = wallet_service.get_not_expired(wallet_handle, "key1");
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), res);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_list_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();

        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();

        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
        key_values.sort();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_list_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();

        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();

        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
        key_values.sort();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_get_pool_name_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();
        let wallet_handle = wallet_service.open(wallet_name, None, None).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), pool_name);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_plugged() {
        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();

        let wallet_service = WalletService::new();

        wallet_service
            .register_type(
                "inmem",
                InmemWallet::create,
                InmemWallet::open,
                InmemWallet::set,
                InmemWallet::get,
                InmemWallet::get_not_expired,
                InmemWallet::list,
                InmemWallet::close,
                InmemWallet::delete,
                InmemWallet::free
            )
            .unwrap();

        wallet_service.create("pool1", Some("inmem"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), "pool1");

        TestUtils::cleanup_indy_home();
        InmemWallet::cleanup();
    }

    #[test]
    fn wallet_service_get_pool_name_works_for_incorrect_wallet_handle() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();

        let get_pool_name_res = wallet_service.get_pool_name(1);
        assert_match!(Err(WalletError::InvalidHandle(_)), get_pool_name_res);

        TestUtils::cleanup_indy_home();
    }
}