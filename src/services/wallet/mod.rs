extern crate serde_json;

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
    fn get_not_expired(&self, key: &str) -> Result<String, WalletError>;
}

trait WalletType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError>;
    fn delete(&self, name: &str) -> Result<(), WalletError>;
    fn open(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError>;
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

impl <'a>JsonDecodable<'a> for WalletDescriptor {}

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
                serde_json::to_string(&WalletDescriptor::new(pool_name, xtype, name))?
                    .as_str()
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

    pub fn delete(&self, name: &str) -> Result<(), WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = serde_json::from_str({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(descriptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(descriptor.xtype));
        }

        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();
        wallet_type.delete(name)?;

        fs::remove_dir_all(_wallet_path(name))?;
        Ok(())
    }

    pub fn open(&self, pool_name: &str, name: &str, credentials: Option<&str>) -> Result<i32, WalletError> {
        let mut descriptor_json = String::new();
        let descriptor: WalletDescriptor = serde_json::from_str({
            let mut file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
            file.read_to_string(&mut descriptor_json)?;
            descriptor_json.as_str()
        })?;

        if descriptor.pool_name != pool_name {
            return Err(WalletError::IncorrectPool(pool_name.to_string()));
        }

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

        let wallet_type = wallet_types.get(descriptor.xtype.as_str()).unwrap();
        let wallet = wallet_type.open(name, config.as_ref().map(String::as_str), credentials)?;

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

    pub fn get_not_expired(&self, handle: i32, key: &str) -> Result<String, WalletError> {
        match self.wallets.borrow().get(&handle) {
            Some(wallet) => wallet.get_not_expired(key),
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

//
//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn json_from_str_works() {
//        let json = "{key1: \"value1\", key2: \"value2\"}";
//
//        json::from_str(json).unwrap();
//    }
//}