pub mod export_import;

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
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod,
    #[serde(default = "default_key_derivation_method")]
    pub rekey_key_derivation_method: KeyDerivationMethod
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyDerivationMethod {
    ARAGON2I_MOD,
    ARAGON2I_INT
}

fn default_key_derivation_method() -> KeyDerivationMethod {
    KeyDerivationMethod::ARAGON2I_MOD
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportConfig {
    pub key: String,
    pub path: String,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub keys: Vec<u8>,
    pub master_key_salt: Vec<u8>,
}
