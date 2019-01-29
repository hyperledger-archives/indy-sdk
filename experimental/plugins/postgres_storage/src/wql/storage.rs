
use errors::wallet::WalletStorageError;
use language;

use errors::common::CommonError;
use utils::crypto::chacha20poly1305_ietf;


#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedValue {
    pub data: Vec<u8>,
    pub key: Vec<u8>,
}

#[allow(dead_code)]
pub const ENCRYPTED_KEY_LEN: usize = chacha20poly1305_ietf::TAGBYTES + chacha20poly1305_ietf::NONCEBYTES + chacha20poly1305_ietf::KEYBYTES;

#[allow(dead_code)]
impl EncryptedValue {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        Self { data, key }
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

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tag {
    Encrypted(Vec<u8>, Vec<u8>),
    PlainText(Vec<u8>, String)
}

#[derive(Debug)]
#[allow(dead_code)]
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
    pub fn new(id: Vec<u8>, value: Option<EncryptedValue>, type_: Option<Vec<u8>>, tags: Option<Vec<Tag>>) -> Self {
        Self {
            id,
            value,
            type_,
            tags,
        }
    }
}

pub trait StorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, WalletStorageError>;
    fn get_total_count(&self) -> Result<Option<usize>, WalletStorageError>;
}

pub trait WalletStorage {
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, WalletStorageError>;
    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), WalletStorageError>;
    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), WalletStorageError>;
    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), WalletStorageError>;
    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), WalletStorageError>;
    fn get_storage_metadata(&self) -> Result<Vec<u8>, WalletStorageError>;
    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), WalletStorageError>;
    fn get_all(&self) -> Result<Box<StorageIterator>, WalletStorageError>;
    fn search(&self, type_: &[u8], query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, WalletStorageError>;
    fn close(&mut self) -> Result<(), WalletStorageError>;
}
