use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    pub agency: AgencyConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct AgencyConfig {
    // Agency wallet id
    pub wallet_id: String,
    // Agency wallet passphrase
    pub wallet_passphrase: String,
    // Agency DID
    pub did: String,
    // Seed for deterministic generation of agency did key
    pub did_seed: Option<String>,
    // Storage type for agency and agents wallets
    pub storage_type: Option<String>,
    // Storage config for agency and agents wallets
    pub storage_config: Option<Value>,
    // Storage credentials for agency and agents wallets
    pub storage_credentials: Option<Value>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    // List of ip:port to bind
    pub addresses: Vec<String>
}