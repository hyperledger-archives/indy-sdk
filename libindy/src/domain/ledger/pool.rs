extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use super::constants::{POOL_CONFIG, POOL_UPGRADE, POOL_RESTART};

use self::indy_crypto::utils::json::JsonEncodable;

use std::collections::HashMap;

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolConfigOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub writes: bool,
    pub force: bool
}

impl PoolConfigOperation {
    pub fn new(writes: bool, force: bool) -> PoolConfigOperation {
        PoolConfigOperation {
            _type: POOL_CONFIG.to_string(),
            writes,
            force
        }
    }
}

impl JsonEncodable for PoolConfigOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolRestartOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub action: String,
    //start, cancel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
}

impl PoolRestartOperation {
    pub fn new(action: &str, datetime: Option<String>) -> PoolRestartOperation {
        PoolRestartOperation {
            _type: POOL_RESTART.to_string(),
            action: action.to_string(),
            datetime,
        }
    }
}

impl JsonEncodable for PoolRestartOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolUpgradeOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub version: String,
    pub action: String,
    //start, cancel
    pub sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    pub reinstall: bool,
    pub force: bool
}

impl PoolUpgradeOperation {
    pub fn new(name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<HashMap<String, String>>,
               justification: Option<&str>, reinstall: bool, force: bool) -> PoolUpgradeOperation {
        PoolUpgradeOperation {
            _type: POOL_UPGRADE.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            action: action.to_string(),
            sha256: sha256.to_string(),
            timeout,
            schedule,
            justification: justification.map(String::from),
            reinstall,
            force
        }
    }
}

impl JsonEncodable for PoolUpgradeOperation {}
