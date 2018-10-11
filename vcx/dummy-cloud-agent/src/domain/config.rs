use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub forward_agent: ForwardAgentConfig,
    pub server: ServerConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ForwardAgentConfig {
    // Forward Agent DID
    pub did: String,
    // Seed for deterministic generation of Forward Agent did key
    pub did_seed: Option<String>,
    // Forward Agent wallet id
    pub wallet_id: String,
    // Forward Agent wallet passphrase
    pub wallet_passphrase: String,
    // Wallet storage type for Forward Agent and agents wallets
    pub wallet_storage_type: Option<String>,
    // Wallet storage config for Forward Agent and agents wallets
    pub wallet_storage_config: Option<Value>,
    // Wallet storage credentials for Forward Agent and agents wallets
    pub wallet_storage_credentials: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    // Http application prefix
    pub prefix: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    // List of ip:port to bind
    pub addresses: Vec<String>,
    // Amount of http workers (instances of app). By default amount of logical CPU cores.
    pub workers: Option<usize>,
}