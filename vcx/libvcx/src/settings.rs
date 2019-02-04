extern crate url;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::RwLock;
use utils::{get_temp_dir_path, error};
use std::path::Path;
use url::Url;
use messages::validation;
use std::fs;
use std::io::prelude::*;
use serde_json::Value;

pub static CONFIG_POOL_NAME: &'static str = "pool_name";
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
pub static CONFIG_LOG_CONFIG: &str = "log_config";
pub static CONFIG_LINK_SECRET_ALIAS: &str = "link_secret_alias";
pub static CONFIG_EXPORTED_WALLET_PATH: &str = "exported_wallet_path";
pub static CONFIG_WALLET_BACKUP_KEY: &str = "backup_key";
pub static CONFIG_WALLET_KEY: &str = "wallet_key";
pub static CONFIG_WALLET_NAME: &'static str = "wallet_name";
pub static CONFIG_WALLET_TYPE: &'static str = "wallet_type";
pub static CONFIG_WALLET_HANDLE: &'static str = "wallet_handle";
pub static CONFIG_THREADPOOL_SIZE: &'static str = "threadpool_size";
pub static CONFIG_WALLET_KEY_DERIVATION: &'static str = "wallet_key_derivation";
pub static CONFIG_PROTOCOL_VERSION: &'static str = "protocol_version";
pub static CONFIG_PAYMENT_METHOD: &'static str = "payment_method";

pub static DEFAULT_PROTOCOL_VERSION: usize = 2;
pub static MAX_SUPPORTED_PROTOCOL_VERSION: usize = 2;
pub static UNINITIALIZED_WALLET_KEY: &str = "<KEY_IS_NOT_SET>";
pub static DEFAULT_GENESIS_PATH: &str = "genesis.txn";
pub static DEFAULT_EXPORTED_WALLET_PATH: &str = "wallet.txn";
pub static DEFAULT_WALLET_NAME: &str = "LIBVCX_SDK_WALLET";
pub static DEFAULT_POOL_NAME: &str = "pool1";
pub static DEFAULT_LINK_SECRET_ALIAS: &str = "main";
pub static DEFAULT_DEFAULT: &str = "default";
pub static DEFAULT_URL: &str = "http://127.0.0.1:8080";
pub static DEFAULT_DID: &str = "2hoqvcwupRTUNkXn6ArYzs";
pub static DEFAULT_VERKEY: &str = "FuN98eH2eZybECWkofW6A9BKJxxnTatBCopfUiNxo6ZB";
pub static DEFAULT_ENABLE_TEST_MODE: &str = "false";
pub static DEFAULT_WALLET_BACKUP_KEY: &str = "backup_wallet_key";
pub static DEFAULT_WALLET_KEY: &str = "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY";
pub static DEFAULT_THREADPOOL_SIZE: usize = 8;
pub static MASK_VALUE: &str = "********";
pub static DEFAULT_WALLET_KEY_DERIVATION: &str = "RAW";
pub static DEFAULT_PAYMENT_PLUGIN: &str = "libnullpay.so";
pub static DEFAULT_PAYMENT_INIT_FUNCTION: &str = "nullpay_init";
pub static DEFAULT_PAYMENT_METHOD: &str = "null";
pub static MAX_THREADPOOL_SIZE: usize = 128;

lazy_static! {
    static ref SETTINGS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

trait ToString {
    fn to_string(&self) -> Self;
}

impl ToString for HashMap<String, String> {
    fn to_string(&self) -> Self {
        let mut v = self.clone();
        v.insert(CONFIG_WALLET_KEY.to_string(), "********".to_string());
        v
    }
}
pub fn set_defaults() -> u32 {
    trace!("set_defaults >>>");

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
    settings.insert(CONFIG_SDK_TO_REMOTE_DID.to_string(),DEFAULT_DID.to_string());
    settings.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(),DEFAULT_VERKEY.to_string());
    settings.insert(CONFIG_WALLET_KEY.to_string(),DEFAULT_WALLET_KEY.to_string());
    settings.insert(CONFIG_WALLET_KEY_DERIVATION.to_string(),DEFAULT_WALLET_KEY_DERIVATION.to_string());
    settings.insert(CONFIG_LINK_SECRET_ALIAS.to_string(), DEFAULT_LINK_SECRET_ALIAS.to_string());
    settings.insert(CONFIG_PROTOCOL_VERSION.to_string(), DEFAULT_PROTOCOL_VERSION.to_string());
    settings.insert(CONFIG_EXPORTED_WALLET_PATH.to_string(),
                    get_temp_dir_path(Some(DEFAULT_EXPORTED_WALLET_PATH)).to_str().unwrap_or("").to_string());
    settings.insert(CONFIG_WALLET_BACKUP_KEY.to_string(), DEFAULT_WALLET_BACKUP_KEY.to_string());
    settings.insert(CONFIG_THREADPOOL_SIZE.to_string(), DEFAULT_THREADPOOL_SIZE.to_string());
    settings.insert(CONFIG_PAYMENT_METHOD.to_string(), DEFAULT_PAYMENT_METHOD.to_string());

    error::SUCCESS.code_num
}

pub fn validate_config(config: &HashMap<String, String>) -> Result<u32, u32> {
    trace!("validate_config >>> config: {:?}", config);

    //Mandatory parameters
    if config.get(CONFIG_WALLET_KEY).is_none() {
        return Err(error::MISSING_WALLET_KEY.code_num);
    }

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
    trace!("loaded settings: {:?}", settings.to_string());
}

pub fn test_indy_mode_enabled() -> bool {
    let config = SETTINGS.read().unwrap();

    match config.get(CONFIG_ENABLE_TEST_MODE) {
        None => false,
        Some(value) => if value == "true" { true } else { if value == "indy" { true } else {false }},
    }
}

pub fn get_threadpool_size() -> usize {
    let size = match get_config_value(CONFIG_THREADPOOL_SIZE) {
        Ok(x) => x.parse::<usize>().unwrap_or(DEFAULT_THREADPOOL_SIZE),
        Err(x) => DEFAULT_THREADPOOL_SIZE,
    };

    if size > MAX_THREADPOOL_SIZE {
        MAX_THREADPOOL_SIZE
    } else {
        size
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
    trace!("process_config_string >>> config {}", config);

    let configuration: Value = serde_json::from_str(config).or(Err(error::INVALID_JSON.code_num))?;
    if let Value::Object(ref map) = configuration {
        for (key, value) in map {
            set_config_value(key, value.as_str().ok_or(error::INVALID_JSON.code_num)?);
        }
    }

    validate_config(
        &SETTINGS.read().or(Err(error::INVALID_CONFIGURATION.code_num))?.clone()
    )
}

pub fn process_config_file(path: &str) -> Result<u32, u32> {
    trace!("process_config_file >>> path: {}", path);

    if !Path::new(path).is_file() {
        error!("Configuration path was invalid");
        Err(error::INVALID_CONFIGURATION.code_num)
    } else {
        process_config_string(&read_config_file(path)?)
    }
}

pub fn get_protocol_version() -> usize {
    let protocol_version = match get_config_value(CONFIG_PROTOCOL_VERSION) {
        Ok(ver) => ver.parse::<usize>().unwrap_or_else(|err| {
            warn!("Can't parse value of protocol version from config ({}), use default one ({})", err, DEFAULT_PROTOCOL_VERSION);
            DEFAULT_PROTOCOL_VERSION
        }),
        Err(err) => {
            info!("Can't fetch protocol version from config ({}), use default one ({})", err, DEFAULT_PROTOCOL_VERSION);
            DEFAULT_PROTOCOL_VERSION
        },
    };
    if protocol_version > MAX_SUPPORTED_PROTOCOL_VERSION {
        error!("Protocol version from config {}, greater then maximal supported {}, use maximum one",
               protocol_version, MAX_SUPPORTED_PROTOCOL_VERSION);
        MAX_SUPPORTED_PROTOCOL_VERSION
    } else {
        protocol_version
    }
}

pub fn get_config_value(key: &str) -> Result<String, u32> {
    trace!("get_config_value >>> key: {}", key);

    SETTINGS
        .read()
        .or(Err(error::INVALID_CONFIGURATION.code_num))?
        .get(key)
        .map_or(Err(error::INVALID_CONFIGURATION.code_num), |v| Ok(v.to_string()))
}

pub fn set_config_value(key: &str, value: &str) {
    trace!("set_config_value >>> key: {}, value: {}", key, value);
    SETTINGS.write().unwrap().insert(key.to_string(), value.to_string());
}

pub fn get_wallet_credentials() -> String {
    let key = get_config_value(CONFIG_WALLET_KEY).unwrap_or(UNINITIALIZED_WALLET_KEY.to_string());
    let mut credentials = json!({"key": key});

    let key_derivation = get_config_value(CONFIG_WALLET_KEY_DERIVATION).ok();
    if let Some(_key) = key_derivation { credentials["key_derivation_method"] = json!(_key); }

    credentials.to_string()
}

pub fn validate_payment_method() -> Result<(), u32> {
    let config = SETTINGS.read().unwrap();
    if let Some(method) = config.get(CONFIG_PAYMENT_METHOD) {
        if !method.to_string().is_empty() {
            return Ok(());
        }
    }
    return Err(error::MISSING_PAYMENT_METHOD.code_num);
}

pub fn get_payment_method() -> String {

    let payment_method = get_config_value(CONFIG_PAYMENT_METHOD).unwrap_or(DEFAULT_PAYMENT_METHOD.to_string());

    payment_method
}

pub fn write_config_to_file(config: &str, path_string: &str) -> Result<(), u32> {
    trace!("write_config_to_file >>> config: {}, path_string: {}", config, path_string);

    let mut file = fs::File::create(Path::new(path_string))
        .or(Err(error::UNKNOWN_ERROR.code_num))?;

    file.write_all(config.as_bytes()).or(Err(error::UNKNOWN_ERROR.code_num))?;

    Ok(())
}

pub fn read_config_file(path: &str) -> Result<String, u32> {
    trace!("read_config_file >>> path: {}", path);
    let mut file = fs::File::open(path).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let mut config = String::new();
    file.read_to_string(&mut config).or(Err(error::UNKNOWN_ERROR.code_num))?;
    Ok(config)
}

pub fn remove_file_if_exists(filename: &str){
    trace!("remove_file_if_exists >>> filename: {}", filename);
    if Path::new(filename).exists() {
        match fs::remove_file(filename) {
            Ok(t) => (),
            Err(e) => println!("Unable to remove file: {:?}", e)
        }
    }
}

pub fn clear_config() {
    trace!("clear_config >>>");
    let mut config = SETTINGS.write().unwrap();
    config.clear();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::get_temp_dir_path;

    #[test]
    fn test_bad_path() {
        let path = "garbage.txt";
        assert_eq!(process_config_file(&path), Err(error::INVALID_CONFIGURATION.code_num));
    }

    #[test]
    fn test_read_config_file() {
        let config_path_buf = get_temp_dir_path(Some("test_init.json"));
        let config_path = config_path_buf.to_str().unwrap();

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
        let config_path_buf = get_temp_dir_path(Some("test_init.json"));
        let config_path = config_path_buf.to_str().unwrap();

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
        assert_eq!(validate_config(&config), Ok(error::SUCCESS.code_num));
    }

    #[test]
    fn test_validate_config_failures() {
        let invalid = "invalid";
        let valid_did = DEFAULT_DID;
        let valid_ver = DEFAULT_VERKEY;

        let mut config: HashMap<String, String> = HashMap::new();
        assert_eq!(validate_config(&config), Err(error::MISSING_WALLET_KEY.code_num));

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_INSTITUTION_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_INSTITUTION_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_AGENCY_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_AGENCY_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_SDK_TO_REMOTE_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_REMOTE_TO_SDK_DID.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_DID.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
        config.insert(CONFIG_SDK_TO_REMOTE_VERKEY.to_string(), invalid.to_string());
        assert_eq!(validate_config(&config), Err(error::INVALID_VERKEY.code_num));
        config.drain();

        config.insert(CONFIG_WALLET_KEY.to_string(), "password".to_string());
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
    fn test_get_and_set_values() {
        let key = "key1".to_string();
        let value1 = "value1".to_string();

        // Fails with invalid key
        assert_eq!(get_config_value(&key), Err(error::INVALID_CONFIGURATION.code_num));

        set_config_value(&key, &value1);
        assert_eq!(get_config_value(&key).unwrap(), value1);
    }

    #[test]
    fn test_payment_plugin_validation() {
        clear_config();
        set_config_value(CONFIG_PAYMENT_METHOD, "null");
        assert_eq!(validate_payment_method(), Ok(()));
    }

    #[test]
    fn test_payment_plugin_validation_empty_string() {
        clear_config();
        set_config_value(CONFIG_PAYMENT_METHOD, "");
        assert_eq!(validate_payment_method(), Err(error::MISSING_PAYMENT_METHOD.code_num));
    }

    #[test]
    fn test_payment_plugin_validation_missing_option() {
        clear_config();
        assert_eq!(validate_payment_method(), Err(error::MISSING_PAYMENT_METHOD.code_num));
    }

    #[test]
    fn test_clear_config() {
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_clear_config",
            "institution_name" : "evernym enterprise",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"key",
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
