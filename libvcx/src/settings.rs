extern crate config;
extern crate url;

use std::collections::HashMap;
use config::Config;
use std::sync::RwLock;
use utils::error;
use std::path::Path;
use url::Url;


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
pub static DEFAULT_GENESIS_PATH: &str = "/tmp/genesis.txn";
pub static DEFAULT_WALLET_NAME: &str = "LIBVCX_SDK_WALLET";
pub static UNINITIALIZED_WALLET_KEY: &str = "<KEY_IS_NOT_SET>";

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn set_defaults() -> u32 {

    // if this fails the program should exit
    let mut settings = SETTINGS.write().unwrap();

    settings.set_default(CONFIG_POOL_NAME, "pool1");
    settings.set_default(CONFIG_WALLET_NAME, DEFAULT_WALLET_NAME);
    settings.set_default(CONFIG_WALLET_TYPE, "default");
    settings.set_default(CONFIG_AGENCY_ENDPOINT, "http://127.0.0.1:8080");
    settings.set_default(CONFIG_AGENCY_DID, "QRyASgXVV6Hoo6zkQTZCWm");
    settings.set_default(CONFIG_AGENCY_VERKEY, "3BVdD7SGNenA1NDK4Z8Kf9A33uVoGZcKrfJa5vELJZVm");
    settings.set_default(CONFIG_REMOTE_TO_SDK_DID, "8xUi3QNchFXzfhCgbALpBr");
    settings.set_default(CONFIG_REMOTE_TO_SDK_VERKEY, "5LXDnRUM7k651nBmhcRraKThVAZYqepaW99zCBYosuwX");
    settings.set_default(CONFIG_INSTITUTION_DID, "2hoqvcwupRTUNkXn6ArYzs");
    settings.set_default(CONFIG_INSTITUTION_NAME, "default");
    settings.set_default(CONFIG_INSTITUTION_LOGO_URL, "http://www.evernym.com");
    settings.set_default(CONFIG_ENABLE_TEST_MODE, "false");
    settings.set_default(CONFIG_SDK_TO_REMOTE_VERKEY, "2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm");
    settings.set_default(CONFIG_GENESIS_PATH, DEFAULT_GENESIS_PATH);
    settings.set_default(CONFIG_WALLET_KEY, UNINITIALIZED_WALLET_KEY);

    error::SUCCESS.code_num
}

pub fn set_to_defaults() -> u32 {

    // if this fails the program should exit
    let mut settings = SETTINGS.write().unwrap();

    settings.set(CONFIG_POOL_NAME,"pool1");
    settings.set(CONFIG_WALLET_NAME,DEFAULT_WALLET_NAME);
    settings.set(CONFIG_WALLET_TYPE,"default");
    settings.set(CONFIG_AGENCY_ENDPOINT,"http://127.0.0.1:8080");
    settings.set(CONFIG_AGENCY_DID,"QRyASgXVV6Hoo6zkQTZCWm");
    settings.set(CONFIG_AGENCY_VERKEY,"3BVdD7SGNenA1NDK4Z8Kf9A33uVoGZcKrfJa5vELJZVm");
    settings.set(CONFIG_REMOTE_TO_SDK_DID,"8xUi3QNchFXzfhCgbALpBr");
    settings.set(CONFIG_REMOTE_TO_SDK_VERKEY,"5LXDnRUM7k651nBmhcRraKThVAZYqepaW99zCBYosuwX");
    settings.set(CONFIG_INSTITUTION_DID,"2hoqvcwupRTUNkXn6ArYzs");
    settings.set(CONFIG_INSTITUTION_NAME,"default");
    settings.set(CONFIG_INSTITUTION_LOGO_URL,"http://www.evernym.com");
    settings.set(CONFIG_ENABLE_TEST_MODE,"false");
    settings.set(CONFIG_SDK_TO_REMOTE_VERKEY,"2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm");
    settings.set(CONFIG_GENESIS_PATH, DEFAULT_GENESIS_PATH);
    settings.set(CONFIG_WALLET_KEY,UNINITIALIZED_WALLET_KEY);

    error::SUCCESS.code_num
}

fn is_valid(value: &str) -> bool {
    for c in value.chars() {
        if !c.is_alphanumeric() && c != '_' { return false;}
    }
    true
}

pub fn validate_config() -> Result<u32, String> {
    let mut error = String::new();

    //if this fails the program should exit
    let config: HashMap<String, String> = SETTINGS.read().unwrap().deserialize::<HashMap<String, String>>().unwrap();

    for setting in config.iter() {
        let mut valid = true;
        if setting.0 == CONFIG_POOL_NAME && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_WALLET_NAME && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENCY_ENDPOINT {
            match Url::parse(setting.1) {
                Err(x) => valid = false,
                Ok(_) => valid = true,
            }
        } else if setting.0 == CONFIG_LOG_CONFIG {
            println!("log_config set to {}", setting.1);
        } else if setting.0 == CONFIG_INSTITUTION_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENCY_VERKEY && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_REMOTE_TO_SDK_VERKEY && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENCY_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_REMOTE_TO_SDK_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_INSTITUTION_NAME {
            valid = true;
        } else if setting.0 == CONFIG_INSTITUTION_LOGO_URL {
            match Url::parse(setting.1) {
                Err(x) => valid = false,
                Ok(_) => valid = true,
            }
        } else if setting.0 == CONFIG_GENESIS_PATH && !is_valid(setting.1){
            // test that the file actually exists (do not worry about the contents of the file)
            if Path::new(setting.1).exists() {
                valid = true;
            } else {
                error!("Genesis file pointed to by vcx config file does not exists");
                valid = false;
            }
        } else {
            //TODO: determine whether we should ignore invalid parameters
            //error.push_str(setting.0);
            //error.push_str("is invalid\n");
        }

        if !valid {
            error.push_str(setting.0);
            error.push_str(" has invalid setting: ");
            error.push_str(setting.1);
        }
    };

    if !error.is_empty() {
        Err(error.to_owned())
    } else {
        let settings = SETTINGS.read().unwrap();
        info!("loaded settings: {:?}", settings.deserialize::<HashMap<String, String>>().unwrap());
        Ok(error::SUCCESS.code_num)
    }
}

pub fn test_indy_mode_enabled() -> bool {
     let config = SETTINGS.read().unwrap();

    match config.get_str(CONFIG_ENABLE_TEST_MODE) {
        Err(_) => false,
        Ok(value) => if value == "true" { true } else { if value == "indy" { true } else {false }},
    }
}

pub fn test_agency_mode_enabled() -> bool {
    let config = SETTINGS.read().unwrap();

    match config.get_str(CONFIG_ENABLE_TEST_MODE) {
        Err(_) => false,
        Ok(value) => if value == "true" { true } else { if value == "agency" { true } else {false }},
    }
}

pub fn process_config_file(path: &str) -> Result<u32, String> {
    if !Path::new(path).is_file() {
        Err("could not find configuration file".to_owned())
    } else {
        // if this fails the program should exit
        SETTINGS.write().unwrap().merge(config::File::with_name(path)).unwrap();

        match validate_config() {
            Err(x) => Err(x),
            Ok(_) => Ok(error::SUCCESS.code_num),
        }
    }
}

pub fn get_config_value(key: &str) -> Result<String, u32> {
    // if this fails the program should exit
    match SETTINGS.read().unwrap().get_str(key) {
        Err(_) => Err(error::INVALID_CONFIGURATION.code_num),
        Ok(value) => Ok(value),
    }
}

pub fn set_config_value(key: &str, value: &str) {
    SETTINGS.write().unwrap().set(key, value).unwrap();
}

pub fn get_wallet_credentials() -> Option<String> {
    let key = get_config_value(CONFIG_WALLET_KEY).unwrap();

    if key == UNINITIALIZED_WALLET_KEY { None } else { Some(format!("{{\"key\":\"{}\"}}", key)) }
}


#[cfg(test)]
pub mod tests {
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs;
    use super::*;

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

    pub fn create_default_genesis_file(){
        fs::File::create(DEFAULT_GENESIS_PATH).unwrap();
    }
    #[test]
    fn test_default_values() {
        remove_file_if_exists(DEFAULT_GENESIS_PATH);

        // test invalid config value
        match get_config_value("garbage") {
            Err(x) => assert_eq!(x, error::INVALID_CONFIGURATION.code_num),
            Ok(v) => assert_eq!(v,"totalgarbage"), //if test gets here it will fail
        };

        // set defaults
        set_defaults();
        set_to_defaults();
        assert_eq!(get_config_value(CONFIG_GENESIS_PATH).unwrap(), DEFAULT_GENESIS_PATH);

        // validate the default config.
        // should error with error::INVALID_GENEISIS string message
        // should error because genesis file should not exist
        match validate_config() {
            Ok(_) => { error!("Validating config should fail");
                       panic!("Validating config should fail");},
            Err(e) => assert_eq!(e, format!("{}{}", "genesis_path has invalid setting: ", DEFAULT_GENESIS_PATH)),
        }

        // add the genesis.txn file
        create_default_genesis_file();

        // validate and should pass this time.
        match validate_config() {
            Ok(i) => assert_eq!(i, error::SUCCESS.code_num),
            Err(e) => panic!(format!("error thrown: {}", e)),
        }

        // cleanup
        remove_file_if_exists(DEFAULT_GENESIS_PATH);
    }

    #[test]
    fn test_bad_path() {
        //test bad path
        let tmp_string = "garbage.txt";
        match process_config_file(&tmp_string) {
            Err(x) => assert_eq!(x,"could not find configuration file"),
            Ok(x) => assert_eq!(x,error::INVALID_CONFIGURATION.code_num),  //if test gets here it will fail
        }
    }

    #[test]
    fn test_process_config_file() {
        let a = "a";
        let b = "b";

        let config_path = "/tmp/test_settings.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        //throw in some invalid content to test the validation code
        let content = "{ \"a\" : \"a\", \"b\":\"b\", \"pool_name\":\"*98*\" }";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        match process_config_file(&config_path) {
            Err(_) => println!("expected invalid setting"),
            Ok(v) => assert_eq!(v, error::INVALID_CONFIGURATION.code_num), //fail if we get here
        }

        match get_config_value(&a) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,a),
        };

        match get_config_value(&b) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,b),
        };

        // Leave file around or other concurrent tests will fail
        //fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_invalid_url() {
        let a = "institution_logo_url";

        remove_file_if_exists(DEFAULT_GENESIS_PATH);

        // add the genesis.txn file
        fs::File::create(DEFAULT_GENESIS_PATH).unwrap();

        let config_path = "/tmp/test_settings.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        //throw in some invalid content to test the validation code
        let content = "{ \"institution_logo_url\" : \"wrong_url\" }";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        match process_config_file(&config_path) {
            Err(v) => assert_eq!(v, "institution_logo_url has invalid setting: wrong_url"),
            Ok(_) => println!("expected invalid URL"), //fail if we get here
        }
        //        assert!(process_config_file(&config_path) == Err(ParseError::InvalidIpv6Address));

        remove_file_if_exists(DEFAULT_GENESIS_PATH);
    }

    #[test]
    fn test_process_file_with_pairwise_configs() {
        set_defaults();
        let a = "agency_did";
        let a_rtn = "72x8p4HubxzUK1dwxcc5FU";
        let b = "remote_to_sdk_verkey";
        let b_rtn = "U22jM6Cea2YVixjWwHN9wq";
        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"agency_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"remote_to_sdk_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"sdk_to_remote_did\" : \"AB3JM851T4EQmhh8CdagSP\", \"institution_name\" : \"enterprise\",\
        \"institution_logo_url\" : \"https://s19.postimg.org/ykyz4x8jn/evernym.png\", \"agency_verkey\" : \"7118p4HubxzUK1dwxcc5FU\",\
        \"remote_to_sdk_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\"}";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        match process_config_file(&config_path) {
            Err(_) => println!("expected invalid setting"),
            Ok(v) => assert_eq!(v, error::SUCCESS.code_num), //fail if we get here
        }
    }
}
