
use indy_common::errors::wallet::WalletStorageError;
use postgres_storage::PostgresStorage;

pub trait WalletStorageType {
    fn create_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>, metadata: &[u8]) -> Result<(), WalletStorageError>;
    fn open_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<PostgresStorage>, WalletStorageError>;
    fn delete_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletStorageError>;
}
