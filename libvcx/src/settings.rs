extern crate url;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::RwLock;
use utils::error;
use std::path::Path;
use url::Url;
use messages::validation;
use std::fs;
use std::io::prelude::*;
use serde_json::Value;


pub static CONFIG_POOL_NAME: &'static str = "pool_name";
pub static CONFIG_WALLET_NAME: &'static str = "wallet_name";
pub static CONFIG_WALLET_TYPE: &'static str = "wallet_type";
pub static CONFIG_AGENCY_ENDPOINT: &'static str = "agency_endpoint";
pub static CONFIG_AGENCY_DID: &'static str = "agency_did";
pub static CONFIG_AGENCY_VERKEY: &'static str = "agency_verkey";
pub static CONFIG_REMOTE_TO_SDK_DID: &'static str = "remote_to_sdk_did";
pub static CONFIG_REMOTE_TO_SDK_VERKEY: &'static str = "remote_to_sdk_verkey";
pub static CONFIG_SDK_TO_REMOTE_DID: &'static str = "sdk_to_remote_did"; // functionally not used
pub static CONFIG_SDK_TO_REMOTE_VERKEY: &'static str = "sdk_to_remote_verkey";
pub static CONFIG_INSTITUTION_DID: &'static str = "institution_did";
pub static CONFIG_INSTITUTION_VERKEY: &'static str = "institution_verkey"; // functionally not used
pub static CONFIG_INSTITUTION_NAME: &'static str = "institution_name";
pub static CONFIG_INSTITUTION_LOGO_URL: &'static str = "institution_logo_url";
pub static CONFIG_ENABLE_TEST_MODE: &'static str = "enable_test_mode";
pub static CONFIG_GENESIS_PATH: &str = "genesis_path";
pub static CONFIG_WALLET_KEY: &str = "wallet_key";
pub static CONFIG_LOG_CONFIG: &str = "log_config";
pub static CONFIG_LINK_SECRET_ALIAS: &str = "link_secret_alias";

pub static UNINITIALIZED_WALLET_KEY: &str = "<KEY_IS_NOT_SET>";
pub static DEFAULT_GENESIS_PATH: &str = "/tmp/genesis.txn";
pub static DEFAULT_WALLET_NAME: &str = "LIBVCX_SDK_WALLET";
pub static DEFAULT_POOL_NAME: &str = "pool1";
pub static DEFAULT_LINK_SECRET_ALIAS: &str = "main";
pub static DEFAULT_DEFAULT: &str = "default";
pub static DEFAULT_URL: &str = "http://127.0.0.1:8080";
pub static DEFAULT_DID: &str = "2hoqvcwupRTUNkXn6ArYzs";
pub static DEFAULT_VERKEY: &str = "FuN98eH2eZybECWkofW6A9BKJxxnTatBCopfUiNxo6ZB";
pub static DEFAULT_ENABLE_TEST_MODE: &str = "false";
pub static TEST_WALLET_KEY: &str = "key";

lazy_static! {
    static ref SETTINGS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub fn set_defaults() -> u32 {

    // if this fails the program should exit
    let mut settings = SETTINGS.write().unwrap();

    settings.insert(CONFIG_POOL_NAME.to_string(),DEFAULT_POOL_NAME.to_string());
    settings.insert(CONFIG_WALLET_NAME.to_string(),DEFAULT_WALLET_NAME.to_string());
    settings.insert(CONFIG_WALLET_TYPE.to_string(),DEFAULT_DEFAULT.to_string());
    settings.insert(CONFIG_AGENCY_ENDPOINT.to_string(),DEFAULT_URL.to_string());
    settings.insert(CONFIG_AGENCY_DID.to_string(),DEFAULT_DID.to_string());
    settings.insert(CONFIG_AGENCY_VERKEY.to_string(),DEFAULT_VERKEY.to_string());
    settings.insert(CONFIG_REMOTE_TO_SDK_DID.to_string(),DEFAULT_DID.to_string());
    settings.insert(CONFIG_REMOTE_TO_SDK_VERKEY.to_string(),DEFAULT_VERKEY.to_string());
    settings.insert(CONFIG_INSTITUTION_DID.to_string(),DEFAULT_DID.to_string());
    settings.insert(CONFIG_INSTITUTION_NAME.to_string(),DEFAULT_DEFAULT.to_string());
    settings.insert(CONFIG_INSTITUTION_LOGO_URL.to_string(),DEFAULT_URL.to_string());
//    settings.set(CONFIG_ENABLE_TEST_MODE,DEFAULT_ENABLE_TEST_MODE);
    settings.insert(CONFIG_SDK_TO_REMOTE_DID.to_string(),DEFAULT_DID.to_string());
    settings.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(),DEFAULT_VERKEY.to_string());
    settings.insert(CONFIG_GENESIS_PATH.to_string(), DEFAULT_GENESIS_PATH.to_string());
    settings.insert(CONFIG_WALLET_KEY.to_string(),TEST_WALLET_KEY.to_string());
    settings.insert(CONFIG_LINK_SECRET_ALIAS.to_string(), DEFAULT_LINK_SECRET_ALIAS.to_string());

    error::SUCCESS.code_num
}

pub fn validate_config(config: &HashMap<String, String>) -> Result<u32, u32> {

    // If values are provided, validate they're in the correct format
    validate_optional_config_val(config.get(CONFIG_INSTITUTION_DID), error::INVALID_DID.code_num, validation::validate_did)?;
    validate_optional_config_val(config.get(CONFIG_INSTITUTION_VERKEY), error::INVALID_VERKEY.code_num, validation::validate_verkey)?;

    validate_optional_config_val(config.get(CONFIG_AGENCY_DID), error::INVALID_DID.code_num, validation::validate_did)?;
    validate_optional_config_val(config.get(CONFIG_AGENCY_VERKEY), error::INVALID_VERKEY.code_num, validation::validate_verkey)?;

    validate_optional_config_val(config.get(CONFIG_SDK_TO_REMOTE_DID), error::INVALID_DID.code_num, validation::validate_did)?;
    validate_optional_config_val(config.get(CONFIG_SDK_TO_REMOTE_VERKEY), error::INVALID_VERKEY.code_num, validation::validate_verkey)?;

    validate_optional_config_val(config.get(CONFIG_REMOTE_TO_SDK_DID), error::INVALID_DID.code_num, validation::validate_did)?;
    validate_optional_config_val(config.get(CONFIG_REMOTE_TO_SDK_VERKEY), error::INVALID_VERKEY.code_num, validation::validate_verkey)?;

    validate_optional_config_val(config.get(CONFIG_AGENCY_ENDPOINT), error::INVALID_URL.code_num, Url::parse)?;
    validate_optional_config_val(config.get(CONFIG_INSTITUTION_LOGO_URL), error::INVALID_URL.code_num, Url::parse)?;
    validate_optional_config_val(config.get(CONFIG_POOL_NAME), error::INVALID_POOL_NAME.code_num, validate_pool_name)?;

    validate_optional_config_val(config.get(CONFIG_WALLET_KEY), error::MISSING_WALLET_KEY.code_num, validate_wallet_key)?;

    Ok(error::SUCCESS.code_num)
}

fn validate_pool_name(value: &str) -> Result<u32, u32> {
    for c in value.chars() {
        if !c.is_alphanumeric() && c != '_' { return Err(error::INVALID_POOL_NAME.code_num);}
    }
    Ok(error::SUCCESS.code_num)
}

fn validate_wallet_key(key: &str) -> Result<u32, u32> {
    if key == UNINITIALIZED_WALLET_KEY { return Err(error::MISSING_WALLET_KEY.code_num); }
    Ok(error::SUCCESS.code_num)
}

fn validate_optional_config_val<F, S, E>(val: Option<&String>, err: u32, closure: F) -> Result<u32, u32>
    where F: Fn(&str) -> Result<S, E> {

    if val.is_none() {return Ok(error::SUCCESS.code_num)}

    closure(val.as_ref().ok_or(error::INVALID_CONFIGURATION.code_num)?)
        .or(Err(err))?;

    Ok(error::SUCCESS.code_num)

}

pub fn log_settings() {
    let settings = SETTINGS.read().unwrap();
    info!("loaded settings: {:?}", settings);
}

pub fn test_indy_mode_enabled() -> bool {
    let config = SETTINGS.read().unwrap();

    match config.get(CONFIG_ENABLE_TEST_MODE) {
        None => false,
        Some(value) => if value == "true" { true } else { if value == "indy" { true } else {false }},
    }
}

pub fn test_agency_mode_enabled() -> bool {
    let config = SETTINGS.read().unwrap();

    match config.get(CONFIG_ENABLE_TEST_MODE) {
        None => false,
        Some(value) => if value == "true" { true } else { if value == "agency" { true } else {false }},
    }
}

pub fn process_config_string(config: &str) -> Result<u32, u32> {
    let configuration: Value = serde_json::from_str(config)
        .or(Err(error::INVALID_JSON.code_num))?;

    if let Value::Object(ref map) = configuration {
        for (key, value) in map {
            if value.is_string() {
                set_config_value(key, value.as_str().unwrap());
            }
        }
    }

    let config = SETTINGS.read().unwrap();
    validate_config(&config.clone())
}

pub fn process_config_file(path: &str) -> Result<u32, u32> {
    if !Path::new(path).is_file() {
        error!("Configuration path was invalid");
        Err(error::INVALID_CONFIGURATION.code_num)
    } else {
        process_config_string(&read_config_file(path)?)
    }
}

pub fn get_config_value(key: &str) -> Result<String, u32> {
    match SETTINGS.read().unwrap().get(key) {
        None => Err(error::INVALID_CONFIGURATION.code_num),
        Some(value) => Ok(value.to_string()),
    }
}

pub fn set_config_value(key: &str, value: &str) {
    SETTINGS.write().unwrap().insert(key.to_string(), value.to_string());
}

pub fn get_wallet_credentials() -> String {
    let key = get_config_value(CONFIG_WALLET_KEY).unwrap_or(UNINITIALIZED_WALLET_KEY.to_string());

    format!("{{\"key\":\"{}\"}}", key)
}

pub fn write_config_to_file(config: &str, path_string: &str) -> Result<(), u32> {
    let mut file = fs::File::create(Path::new(path_string))
        .or(Err(error::UNKNOWN_ERROR.code_num))?;

    file.write_all(config.as_bytes()).or(Err(error::UNKNOWN_ERROR.code_num))?;

    Ok(())
}

pub fn read_config_file(path: &str) -> Result<String, u32> {
    let mut file = fs::File::open(path).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let mut config = String::new();
    file.read_to_string(&mut config).or(Err(error::UNKNOWN_ERROR.code_num))?;
    Ok(config)
}

pub fn remove_default_genesis_file(){
    remove_file_if_exists(DEFAULT_GENESIS_PATH);
}

pub fn remove_file_if_exists(filename: &str){
    if Path::new(filename).exists() {
        println!("{}", format!("Removing file for testing: {}.", &filename));
        match fs::remove_file(filename) {
            Ok(t) => (),
            Err(e) => println!("Unable to remove file: {:?}", e)
        }
    }
}

pub fn clear_config() {
    let mut config = SETTINGS.write().unwrap();
    config.clear();
}

pub fn create_default_genesis_file(){
    fs::File::create(DEFAULT_GENESIS_PATH).unwrap();
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_bad_path() {
        let path = "garbage.txt";
        assert_eq!(process_config_file(&path), Err(error::INVALID_CONFIGURATION.code_num));
    }

    #[test]
    fn test_read_config_file() {
        let config_path = "/tmp/test_init.json";

        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_read_config_file",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key"
        }).to_string();
        write_config_to_file(&content, config_path).unwrap();

        assert_eq!(read_config_file(config_path), Ok(content));
    }

    #[test]
    fn test_process_file() {
        let config_path = "/tmp/test_init.json";

        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_process_file",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key"
        }).to_string();
        write_config_to_file(&content, config_path).unwrap();

        assert_eq!(process_config_file(config_path), Ok(error::SUCCESS.code_num));
        assert_eq!(get_config_value("institution_name").unwrap(), "evernym enterprise".to_string());
    }

    #[test]
    fn test_process_config_str() {
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_process_config_str",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key"
        }).to_string();

        assert_eq!(process_config_string(&content), Ok(error::SUCCESS.code_num));
        assert_eq!(get_config_value("institution_name").unwrap(), "evernym enterprise".to_string());
    }

    #[test]
    fn test_validate_config() {
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_validate_config",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key",
            "institution_did": "44x8p4HubxzUK1dwxcc5FU",
            "institution_verkey": "444MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
        }).to_string();
        let config: HashMap<String, String> = serde_json::from_str(&content).unwrap();
        assert_eq!(validate_config(&config), Ok(error::SUCCESS.code_num))
    }

    #[test]
    fn test_validate_config_failures() {
        let invalid = "invalid";
        let valid_did = DEFAULT_DID;
        let valid_ver = DEFAULT_VERKEY;

        let mut config: HashMap<String, String> = HashMap::new();
        assert_eq!(validate_config(&config), Ok(error::SUCCESS.code_num));

        config.insert(CONFIG_INSTITUTION_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_INSTITUTION_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_AGENCY_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_AGENCY_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_SDK_TO_REMOTE_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_REMOTE_TO_SDK_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_INSTITUTION_LOGO_URL.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_URL.code_num));
        config.drain();
    }

    #[test]
    fn test_validate_optional_config_val() {
        let closure = Url::parse;
        let mut config: HashMap<String, String> = HashMap::new();
        config.insert("valid".to_string(), DEFAULT_URL.to_string());
        config.insert("invalid".to_string(), "invalid_url".to_string());

        //Success
        assert_eq!(validate_optional_config_val(config.get("valid"), error::INVALID_URL.code_num, closure),
                   Ok(error::SUCCESS.code_num));

        // Success with No config
        assert_eq!(validate_optional_config_val(config.get("unknown"), error::INVALID_URL.code_num, closure),
                   Ok(error::SUCCESS.code_num));

        // Fail with failed fn call
        assert_eq!(validate_optional_config_val(config.get("invalid"),
                                                error::INVALID_URL.code_num,
                                                closure), Err(error::INVALID_URL.code_num));
    }

    #[test]
    fn test_validate_pool_name() {
        assert_eq!(validate_pool_name("pool1"), Ok(error::SUCCESS.code_num));

        assert_eq!(validate_pool_name("**pool_name**"), Err(error::INVALID_POOL_NAME.code_num));
    }

    #[test]
    fn test_get_and_set_values() {
        let key = "key1".to_string();
        let value1 = "value1".to_string();

        // Fails with invalid key
        assert_eq!(get_config_value(&key), Err(error::INVALID_CONFIGURATION.code_num));

        set_config_value(&key, &value1);
        assert_eq!(get_config_value(&key).unwrap(), value1)
    }

    #[test]
    fn test_clear_config() {
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_clear_config",
            "institution_name" : "evernym enterprise",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key"
        }).to_string();

        assert_eq!(process_config_string(&content), Ok(error::SUCCESS.code_num));

        assert_eq!(get_config_value("pool_name").unwrap(), "pool1".to_string());
        assert_eq!(get_config_value("config_name").unwrap(), "config1".to_string());
        assert_eq!(get_config_value("wallet_name").unwrap(), "test_clear_config".to_string());
        assert_eq!(get_config_value("institution_name").unwrap(), "evernym enterprise".to_string());
        assert_eq!(get_config_value("genesis_path").unwrap(), "/tmp/pool1.txn".to_string());
        assert_eq!(get_config_value("wallet_key").unwrap(), "key".to_string());

        clear_config();

        // Fails after  config is cleared
        assert_eq!(get_config_value("pool_name"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(get_config_value("config_name"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(get_config_value("wallet_name"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(get_config_value("institution_name"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(get_config_value("genesis_path"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(get_config_value("wallet_key"), Err(error::INVALID_CONFIGURATION.code_num));
    }
}
