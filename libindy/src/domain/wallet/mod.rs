use serde_json::value::Value;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id: String,
    pub storage_type: Option<String>,
    pub storage_config: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub rekey: Option<String>,
    pub storage_credentials: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportConfig {
    pub key: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub keys: Vec<u8>,
    pub master_key_salt: Vec<u8>,
}