pub mod default;

use std::collections::HashMap;
use std::string;

use language;
use config::{StorageCredentials,WalletRuntimeConfig,StorageConfig};

use self::error::StorageError;


#[derive(Clone, Debug, PartialEq)]
pub enum TagValue {
    Encrypted(Vec<u8>),
    Plain(String),
    Meta(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StorageValue {
    pub data: Vec<u8>,
    pub key: Vec<u8>
}

#[derive(Clone, Debug, PartialEq)]
pub struct StorageEntity {
    pub name: Vec<u8>,
    pub value: Option<StorageValue>,
    pub tags: Option<HashMap<Vec<u8>, TagValue>>,
}

impl StorageValue {
    fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        Self {
            data: data,
            key: key,
        }
    }
}

impl StorageEntity {
    fn new(name: Vec<u8>, value: Option<StorageValue>, tags: Option<HashMap<Vec<u8>, TagValue>>) -> Self {
        Self {
            name: name,
            value: value,
            tags: tags,
        }
    }
}


pub trait StorageIterator {
    fn next(&mut self) -> Result<Option<StorageEntity>, StorageError>;
}


pub trait Storage {
    fn get(&self, class: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, StorageError>;
    fn add(&self, class: &Vec<u8>, name: &Vec<u8>, value: &Vec<u8>, value_key: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>) -> Result<(), StorageError>;
    fn delete(&self, class: &Vec<u8>, name: &Vec<u8>) -> Result<(), StorageError>;
    fn get_all(&self) -> Result<Box<StorageIterator>, StorageError>;
    fn search(&self, class: &Vec<u8>, query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, StorageError>;
    fn clear(&self) -> Result<(), StorageError>;
    fn close(&mut self) -> Result<(), StorageError>;
}


pub trait StorageType {
    fn create(&self, name: &str, storage_config: &StorageConfig, storage_credentials: &StorageCredentials, keys: &Vec<u8>) -> Result<(), StorageError>;
    fn delete(&self, name: &str, storage_config: &StorageConfig, storage_credentials: &StorageCredentials) -> Result<(), StorageError >;
    fn open(&self, name: &str, storage_config: &StorageConfig, runtime_config: &WalletRuntimeConfig, storage_credentials: &StorageCredentials) -> Result<(Box<Storage>, Vec<u8>), StorageError>;
}