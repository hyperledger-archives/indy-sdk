pub mod default;
pub mod plugged;

use errors::wallet::WalletStorageError;
use services::wallet::language;
use services::wallet::wallet::EncryptedValue;

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
pub struct StorageEntity {
    pub name: Vec<u8>,
    pub value: Option<EncryptedValue>,
    pub type_: Option<Vec<u8>>,
    pub tags: Option<Vec<Tag>>,
}

impl StorageEntity {
    fn new(name: Vec<u8>, value: Option<EncryptedValue>, type_: Option<Vec<u8>>, tags: Option<Vec<Tag>>) -> Self {
        Self {
            name: name,
            value: value,
            type_: type_,
            tags: tags,
        }
    }
}


pub trait StorageIterator {
    fn next(&mut self) -> Result<Option<StorageEntity>, WalletStorageError>;
    fn get_total_count(&self) -> Result<Option<usize>, WalletStorageError>;
}


pub trait WalletStorage {
    fn get(&self, type_: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, WalletStorageError>;
    fn add(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &EncryptedValue, tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn update(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &EncryptedValue) -> Result<(), WalletStorageError>;
    fn add_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn update_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn delete_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tag_names: &[TagName]) -> Result<(), WalletStorageError>;
    fn delete(&self, type_: &Vec<u8>, name: &Vec<u8>) -> Result<(), WalletStorageError>;
    fn get_storage_metadata(&self) -> Result<Vec<u8>, WalletStorageError>;
    fn set_storage_metadata(&self, metadata: &Vec<u8>) -> Result<(), WalletStorageError>;
    fn get_all(&self) -> Result<Box<StorageIterator>, WalletStorageError>;
    fn search(&self, type_: &Vec<u8>, query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, WalletStorageError>;
    fn close(&mut self) -> Result<(), WalletStorageError>;
}


pub trait WalletStorageType {
    fn create_storage(&self, name: &str, config: Option<&str>, credentials: &str, metadata: &Vec<u8>) -> Result<(), WalletStorageError>;
    fn open_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<Box<WalletStorage>, WalletStorageError>;
    fn delete_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(), WalletStorageError>;
}