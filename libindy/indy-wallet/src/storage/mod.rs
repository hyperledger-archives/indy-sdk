pub mod default;
pub mod plugged;

use indy_api_types::errors::prelude::*;
use crate::language;
use crate::wallet::EncryptedValue;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tag {
    Encrypted(Vec<u8>, Vec<u8>),
    PlainText(Vec<u8>, String)
}

#[derive(Debug)]
pub enum TagName {
    OfEncrypted(Vec<u8>),
    OfPlain(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct StorageRecord {
    pub id: Vec<u8>,
    pub value: Option<EncryptedValue>,
    pub type_: Option<Vec<u8>>,
    pub tags: Option<Vec<Tag>>,
}

impl StorageRecord {
    fn new(id: Vec<u8>, value: Option<EncryptedValue>, type_: Option<Vec<u8>>, tags: Option<Vec<Tag>>) -> Self {
        Self {
            id,
            value,
            type_,
            tags,
        }
    }
}

pub trait StorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, IndyError>;
    fn get_total_count(&self) -> Result<Option<usize>, IndyError>;
}

pub trait WalletStorage {
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, IndyError>;
    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), IndyError>;
    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), IndyError>;
    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError>;
    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError>;
    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), IndyError>;
    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), IndyError>;
    fn get_storage_metadata(&self) -> Result<Vec<u8>, IndyError>;
    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), IndyError>;
    fn get_all(&self) -> Result<Box<dyn StorageIterator>, IndyError>;
    fn search(&self, type_: &[u8], query: &language::Operator, options: Option<&str>) -> Result<Box<dyn StorageIterator>, IndyError>;
    fn close(&mut self) -> Result<(), IndyError>;
}

pub trait WalletStorageType {
    fn create_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>, metadata: &[u8]) -> Result<(), IndyError>;
    fn open_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<dyn WalletStorage>, IndyError>;
    fn delete_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), IndyError>;
}