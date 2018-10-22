use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub forward_agent: ForwardAgentConfig,
    pub server: ServerConfig,
    pub wallet_storage: WalletStorageConfig,
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

#[derive(Clone, Debug, Deserialize)]
pub struct CloudAgentConfig {
    // Cloud Agent DID
    pub did: String,
    // Cloud Agent Owner DID
    pub owner_did: String,
    // Cloud Agent wallet id
    pub wallet_id: String,
    // Cloud Agent wallet passphrase
    pub wallet_passphrase: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WalletStorageConfig {
    // Wallet storage type for agents wallets
    #[serde(rename = "type")]
    pub xtype: Option<String>,
    // Wallet storage config for agents wallets
    pub config: Option<Value>,
    // Wallet storage credentials for agents wallets
    pub credentials: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CloudAgentConnectionConfig {
    // Cloud Agent wallet handle
    pub wallet_handle: i32,
    // Cloud Agent Owner DID
    pub owner_did: String,
    // Cloud Agent DID
    pub agent_did: String,
    // User pairwise DID
    pub from_did: String,
    // User pairwise DID Verkey
    pub from_did_verkey: String,
    // Cloud Agent pairwise DID
    pub pairwise_did: String,
    // Cloud Agent pairwise DID Verkey
    pub pairwise_did_verkey: String,
}