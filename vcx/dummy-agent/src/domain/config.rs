use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    pub agent: AgentConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct AgentConfig {
    // Forward Agent wallet id
    pub wallet_id: String,
    // Forward Agent wallet passphrase
    pub wallet_passphrase: String,
    // Forward Agent DID
    pub did: String,
    // Seed for deterministic generation of Forward Agent did key
    pub did_seed: Option<String>,
    // Storage type for Forward Agent and agents wallets
    pub storage_type: Option<String>,
    // Storage config for Forward Agent and agents wallets
    pub storage_config: Option<Value>,
    // Storage credentials for Forward Agent and agents wallets
    pub storage_credentials: Option<Value>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    // List of ip:port to bind
    pub addresses: Vec<String>
}