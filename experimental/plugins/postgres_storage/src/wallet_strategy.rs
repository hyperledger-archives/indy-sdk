
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
    // determine phyisical table name based on wallet strategy
    fn table_name(&self, config: &PostgresConfig, base_name: &str) -> str;
    // determine additional query parameters based on wallet strategy
    fn query_qualifier(&self, config: &PostgresConfig) -> str;
}
