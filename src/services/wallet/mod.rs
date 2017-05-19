mod default;

use self::default::DefaultWalletType;

use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;
use utils::json::{JsonDecodable, JsonEncodable};

pub trait Wallet {
    fn set(&self, key: &str, value: &str) -> Result<(), WalletError>;
    fn get(&self, key: &str) -> Result<String, WalletError>;
    fn list(&self, key_prefix: &str) -> Result<Vec<(String, String)>, WalletError>;
    fn get_not_expired(&self, key: &str) -> Result<String, WalletError>;
    fn get_pool_name(&self) -> String;
}

trait WalletType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError>;
    fn delete(&self, name: &str, credentials: Option<&str>) -> Result<(), WalletError>;
    fn open(&self, name: &str, pool_name: &str, config: Option<&str>, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError>;
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

pub struct WalletService {
    types: RefCell<HashMap<&'static str, Box<WalletType>>>,
    wallets: RefCell<HashMap<i32, Box<Wallet>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let mut types: HashMap<&str, Box<WalletType>> = HashMap::new();
        types.insert("default", Box::new(DefaultWalletType::new()));

        WalletService {
            types: RefCell::new(types),
            wallets: RefCell::new(HashMap::new())
        }
    }

    pub fn resiter_type(xtype: &str,
                        create: fn(name: &str,
                                   config: &str,
                                   credentials: &str) -> Result<(), WalletError>,
                        open: fn(name: &str,
                                 config: &str,
                                 credentials: &str) -> Result<i32, WalletError>,
                        set: extern fn(handle: i32,
                                       key: &str, sub_key: &str,
                                       value: &str) -> Result<(), WalletError>,
                        get: extern fn(handle: i32,
                                       key: &str, sub_key: &str) -> Result<(String, i32), WalletError>,
                        list: extern fn(handle: i32,
                                        key: &str, sub_key: &str) -> Result<(Vec<(String, String)>, i32), WalletError>,
                        get_not_expired: extern fn(handle: i32,
                                                   key: &str, sub_key: &str) -> Result<(String, i32), WalletError>,
                        close: extern fn(handle: i32) -> Result<(), WalletError>,
                        delete: extern fn(name: &str) -> Result<(), WalletError>) {
        unimplemented!();
    }

    pub fn create(&self, pool_name: &str, xtype: Option<&str>, name: &str, config: Option<&str>,
                  credentials: Option<&str>) -> Result<(), WalletError> {
        let xtype = xtype.unwrap_or("default");

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(xtype) {
            return Err(WalletError::UnknownType(xtype.to_string()))
        }

        let wallet_path = _wallet_path(name);
        if wallet_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()))
        }
        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        let wallet_type = wallet_types.get(xtype).unwrap();
        wallet_type.create(name, config, credentials)?;

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
        wallet_type.delete(name, credentials)?;

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

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
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

        // FIXME: Check for already opened walled!!!

        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();
        let wallet = wallet_type.open(name,
                                      descriptor.pool_name.as_str(),
                                      config.as_ref().map(String::as_str),
                                      runtime_config,
                                      credentials)?;

        let wallet_handle = SequenceUtils::get_next_id();
        self.wallets.borrow_mut().insert(wallet_handle, wallet);
        Ok(wallet_handle)
    }

    pub fn close(&self, handle: i32) -> Result<(), WalletError> {
        match self.wallets.borrow_mut().remove(&handle) {
            Some(wallet) => Ok(()),
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
    use utils::test::TestUtils;

    use std::time::Duration;
    use std::thread;

    #[test]
    fn new_works() {
        WalletService::new();
    }

    #[test]
    fn create_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn create_works_for_none_type() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn create_works_for_unknown_type() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        let res = wallet_service.create("pool1", Some("unknown"), "wallet1", None, None);
        assert_match!(Err(WalletError::UnknownType(_)), res);

        TestUtils::cleanup_sovrin_home();
    }


    #[test]
    fn create_works_for_twice() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let res = wallet_service.create("pool1", None, "wallet1", None, None);
        assert_match!(Err(WalletError::AlreadyExists(_)), res);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn delete_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.delete("wallet1", None).unwrap();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn open_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        wallet_service.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn close_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.close(wallet_handle).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn set_get_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn set_get_works_for_reopen() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        wallet_service.close(wallet_handle).unwrap();

        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn get_works_for_unknown() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let res = wallet_service.get(wallet_handle, "key1");
        assert_match!(Err(WalletError::NotFound(_)), res);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn set_get_works_for_update() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        wallet_service.set(wallet_handle, "key1", "value1").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value1", value);

        wallet_service.set(wallet_handle, "key1", "value2").unwrap();
        let value = wallet_service.get(wallet_handle, "key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn set_get_not_expired_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();
        wallet_service.set(wallet_handle, "key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(2, 0));

        let res = wallet_service.get_not_expired(wallet_handle, "key1");
        assert_match!(Err(WalletError::NotFound(_)), res);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn list_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", None, "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", Some("{\"freshness_time\": 1}"), None).unwrap();

        wallet_service.set(wallet_handle, "key1::subkey1", "value1").unwrap();
        wallet_service.set(wallet_handle, "key1::subkey2", "value2").unwrap();

        let mut key_values = wallet_service.list(wallet_handle, "key1::").unwrap();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn get_pool_name_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();
        let wallet_handle = wallet_service.open(wallet_name, None, None).unwrap();

        assert_eq!(wallet_service.get_pool_name(wallet_handle).unwrap(), pool_name);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn get_pool_name_works_for_incorrect_wallet_handle() {
        TestUtils::cleanup_sovrin_home();

        let wallet_service = WalletService::new();
        let wallet_name = "wallet1";
        let pool_name = "pool1";
        wallet_service.create(pool_name, None, wallet_name, None, None).unwrap();

        let get_pool_name_res = wallet_service.get_pool_name(1);
        assert_match!(Err(WalletError::InvalidHandle(_)), get_pool_name_res);

        TestUtils::cleanup_sovrin_home();
    }
}