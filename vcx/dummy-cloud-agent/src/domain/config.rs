use serde_json::Value;

use crate::domain::protocol_type::ProtocolTypes;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub forward_agent: ForwardAgentConfig,
    pub server: ServerConfig,
    pub server_admin: Option<ServerAdminConfig>,
    pub wallet_storage: WalletStorageConfig,
    pub protocol_type: Option<ProtocolTypes>,
    pub indy_runtime: Option<IndyRuntimeConfig>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ForwardAgentConfig {
    // Forward Agent DID
    pub did: String,
    // Seed for deterministic generation of Forward Agent did key
    pub did_seed: Option<String>,
    // Forward Agent Endpoint
    pub endpoint: String,
    // Forward Agent wallet id
    pub wallet_id: String,
    // Forward Agent wallet passphrase
    pub wallet_passphrase: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    // Http application prefix
    pub prefix: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    // List of ip:port to bind
    pub addresses: Vec<String>,
    // Amount of http workers (instances of app). By default amount of logical CPU cores.
    pub workers: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerAdminConfig {
    // enable or disable http api for fetching information about agency status
    pub enabled: bool,
    // List of ip:port to bind
    pub addresses: Vec<String>,
}


//#[derive(Clone, Debug, Deserialize)]
//pub struct AgentWallet {
//    // enable or disable http api for fetching information about agency status
//    pub enabled: bool,
//    // List of ip:port to bind
//    pub addresses: Vec<String>,
//}

#[derive(Clone, Debug, Deserialize)]
pub struct WalletStorageConfig {
    // Wallet storage type for agents wallets
    #[serde(rename = "type")]
    pub xtype: Option<String>,
    // Optional to override default library path. Default value is determined based on value of
    // xtype and OS
    pub plugin_library_path: Option<String>,
    // Optional to override default storage initialization function. Default value is  determined
    // based on value of xtype and OS
    pub plugin_init_function: Option<String>,
    // Wallet storage config for agents wallets
    pub config: Option<Value>,
    // Wallet storage credentials for agents wallets
    pub credentials: Option<Value>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndyRuntimeConfig {
    // size of thread pool for the most expensive crypto operations
    pub crypto_thread_pool_size: usize
}