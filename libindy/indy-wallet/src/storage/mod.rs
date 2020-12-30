use async_trait::async_trait;
use indy_api_types::errors::prelude::*;

use crate::{language, wallet::EncryptedValue};

pub mod default;
pub mod mysql;
//pub mod plugged; FIXME:!!!

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tag {
    Encrypted(Vec<u8>, Vec<u8>),
    PlainText(Vec<u8>, String),
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
    fn new(
        id: Vec<u8>,
        value: Option<EncryptedValue>,
        type_: Option<Vec<u8>>,
        tags: Option<Vec<Tag>>,
    ) -> Self {
        Self {
            id,
            value,
            type_,
            tags,
        }
    }
}

#[async_trait]
pub trait StorageIterator: Send + Sync {
    async fn next(&mut self) -> Result<Option<StorageRecord>, IndyError>;
    fn get_total_count(&self) -> Result<Option<usize>, IndyError>;
}

#[async_trait]
pub trait WalletStorage: Send + Sync {
    async fn get(&self, type_: &[u8], id: &[u8], options: &str)
        -> Result<StorageRecord, IndyError>;
    async fn add(
        &self,
        type_: &[u8],
        id: &[u8],
        value: &EncryptedValue,
        tags: &[Tag],
    ) -> Result<(), IndyError>;
    async fn update(
        &self,
        type_: &[u8],
        id: &[u8],
        value: &EncryptedValue,
    ) -> Result<(), IndyError>;
    async fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError>;
    async fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), IndyError>;
    async fn delete_tags(
        &self,
        type_: &[u8],
        id: &[u8],
        tag_names: &[TagName],
    ) -> Result<(), IndyError>;
    async fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), IndyError>;
    async fn get_storage_metadata(&self) -> Result<Vec<u8>, IndyError>;
    async fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), IndyError>;
    async fn get_all(&self) -> Result<Box<dyn StorageIterator>, IndyError>;

    // TODO:
    async fn search(
        &self,
        type_: &[u8],
        query: &language::Operator,
        options: Option<&str>,
    ) -> Result<Box<dyn StorageIterator>, IndyError>;
    fn close(&mut self) -> Result<(), IndyError>;
}

#[async_trait]
pub trait WalletStorageType: Send + Sync {
    async fn create_storage(
        &self,
        id: &str,
        config: Option<&str>,
        credentials: Option<&str>,
        metadata: &[u8],
    ) -> Result<(), IndyError>;
    async fn open_storage(
        &self,
        id: &str,
        config: Option<&str>,
        credentials: Option<&str>,
    ) -> Result<Box<dyn WalletStorage>, IndyError>;
    async fn delete_storage(
        &self,
        id: &str,
        config: Option<&str>,
        credentials: Option<&str>,
    ) -> Result<(), IndyError>;
}
