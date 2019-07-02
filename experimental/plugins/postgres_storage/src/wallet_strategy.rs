
#[derive(Deserialize, Debug)]
#[derive(Copy, Clone)]
enum WalletScheme {
    DatabasePerWallet,
    MultiWalletSingleTable,
    MultiWalletMultiTable
}

trait WalletStrategy {
    // initialize storage based on wallet storage strategy
    fn init_storage(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError>;
    // initialize a single wallet based on wallet storage strategy
    fn init_wallet(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError>;
    // determine phyisical table name based on wallet strategy
    fn table_name(&self, config: &PostgresConfig, base_name: &str) -> str;
    // determine additional query parameters based on wallet strategy
    fn query_qualifier(&self, config: &PostgresConfig) -> str;
}

pub struct DatabasePerWalletStrategy {}
pub struct MultiWalletSingleTableStrategy {}
pub struct MultiWalletMultiTableStrategy {}

impl WalletStrategy for DatabasePerWalletStrategy {
    // initialize storage based on wallet storage strategy
    fn init_storage(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // no-op
        Ok(())
    }
    // initialize a single wallet based on wallet storage strategy
    fn init_wallet(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // create database for wallet
    }
    // determine phyisical table name based on wallet strategy
    fn table_name(&self, config: &PostgresConfig, base_name: &str) -> str {
    }
    // determine additional query parameters based on wallet strategy
    fn query_qualifier(&self, config: &PostgresConfig) -> str {
    }
}

impl WalletStrategy for MultiWalletSingleTableStrategy {
    // initialize storage based on wallet storage strategy
    fn init_storage(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // create database and tables for storage
    }
    // initialize a single wallet based on wallet storage strategy
    fn init_wallet(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // no-op
        Ok(())
    }
    // determine phyisical table name based on wallet strategy
    fn table_name(&self, config: &PostgresConfig, base_name: &str) -> str {
    }
    // determine additional query parameters based on wallet strategy
    fn query_qualifier(&self, config: &PostgresConfig) -> str {
    }
}

impl WalletStrategy for MultiWalletMultiTableStrategy {
    // initialize storage based on wallet storage strategy
    fn init_storage(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // create database for storage
    }
    // initialize a single wallet based on wallet storage strategy
    fn init_wallet(&self, config: &PostgresConfig, credentials: &PostgresCredentials) -> Result<(), WalletStorageError> {
        // create tables for wallet storage
    }
    // determine phyisical table name based on wallet strategy
    fn table_name(&self, config: &PostgresConfig, base_name: &str) -> str {
    }
    // determine additional query parameters based on wallet strategy
    fn query_qualifier(&self, config: &PostgresConfig) -> str {
    }
}
