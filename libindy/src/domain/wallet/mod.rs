pub mod export_import;

use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub id: String,
    pub storage_type: Option<String>,
    pub storage_config: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub rekey: Option<String>,
    pub storage_credentials: Option<Value>,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod,
    #[serde(default = "default_key_derivation_method")]
    pub rekey_derivation_method: KeyDerivationMethod
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyDerivationMethod {
    RAW,
    ARGON2I_MOD,
    ARGON2I_INT
}

fn default_key_derivation_method() -> KeyDerivationMethod {
    KeyDerivationMethod::ARGON2I_MOD
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportConfig {
    pub key: String,
    pub path: String,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Metadata {
    MetadataArgon(MetadataArgon),
    MetadataRaw(MetadataRaw),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataArgon {
    pub keys: Vec<u8>,
    pub master_key_salt: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataRaw {
    pub keys: Vec<u8>
}

#[derive(Debug, Deserialize)]
pub struct KeyConfig {
    pub seed: Option<String>
}

pub type Tags = HashMap<String, String>;
