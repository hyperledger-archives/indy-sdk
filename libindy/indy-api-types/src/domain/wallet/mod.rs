use serde_json::value::Value;
use std::collections::HashMap;

use crate::validation::Validatable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub id: String,
    pub storage_type: Option<String>,
    pub storage_config: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeyDerivationMethod {
    RAW,
    ARGON2I_MOD,
    ARGON2I_INT
}

fn default_key_derivation_method() -> KeyDerivationMethod {
    KeyDerivationMethod::ARGON2I_MOD
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    pub key: String,
    pub path: String,
    #[serde(default = "default_key_derivation_method")]
    pub key_derivation_method: KeyDerivationMethod
}

#[derive(Debug, Deserialize)]
pub struct KeyConfig {
    pub seed: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    // Wallet record type
    #[serde(rename = "type")]
    pub type_: String,
    // Wallet record id
    pub id: String,
    // Wallet record value
    pub value: String,
    // Wallet record tags
    pub tags: HashMap<String, String>,
}

pub type Tags = HashMap<String, String>;

impl Validatable for Config {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Wallet id is empty".to_string());
        }
        Ok(())
    }
}

