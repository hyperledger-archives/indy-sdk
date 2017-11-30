extern crate config;
extern crate url;

use std::collections::HashMap;
use config::Config;
use std::sync::RwLock;
use utils::error;
use std::path::Path;
use url::Url;


pub static CONFIG_POOL_NAME: &'static str = "pool_name";
pub static CONFIG_POOL_CONFIG_NAME: &'static str = "config_name";
pub static CONFIG_WALLET_NAME: &'static str = "wallet_name";
pub static CONFIG_WALLET_TYPE: &'static str = "wallet_type";
pub static CONFIG_AGENT_ENDPOINT: &'static str = "agent_endpoint";
pub static CONFIG_AGENCY_PAIRWISE_DID: &'static str = "agency_pairwise_did";
pub static CONFIG_AGENCY_PAIRWISE_VERKEY: &'static str = "agency_pairwise_verkey";
pub static CONFIG_AGENT_PAIRWISE_DID: &'static str = "agent_pairwise_did";
pub static CONFIG_AGENT_PAIRWISE_VERKEY: &'static str = "agent_pairwise_verkey";
pub static CONFIG_ENTERPRISE_DID: &'static str = "enterprise_did";
pub static CONFIG_ENTERPRISE_DID_AGENCY: &'static str = "enterprise_did_agency";
pub static CONFIG_ENTERPRISE_DID_AGENT: &'static str = "enterprise_did_agent";
pub static CONFIG_ENTERPRISE_NAME: &'static str = "enterprise_name";
pub static CONFIG_LOGO_URL: &'static str = "logo_url";
pub static CONFIG_ENABLE_TEST_MODE: &'static str = "enable_test_mode";
pub static CONFIG_ENTERPRISE_VERKEY: &'static str = "agent_enterprise_verkey";

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn set_defaults() -> u32 {
    // if this fails the program should exit
    let mut settings = SETTINGS.write().unwrap();

    settings.set_default(CONFIG_POOL_NAME,"pool1");
    settings.set_default(CONFIG_POOL_CONFIG_NAME,"");
    settings.set_default(CONFIG_WALLET_NAME,"wallet1");
    settings.set_default(CONFIG_WALLET_TYPE,"default");
    settings.set_default(CONFIG_AGENT_ENDPOINT,"http://127.0.0.1:8080");
    settings.set_default(CONFIG_AGENCY_PAIRWISE_DID,"QRyASgXVV6Hoo6zkQTZCWm");
    settings.set_default(CONFIG_AGENCY_PAIRWISE_VERKEY,"3BVdD7SGNenA1NDK4Z8Kf9A33uVoGZcKrfJa5vELJZVm");
    settings.set_default(CONFIG_AGENT_PAIRWISE_DID,"8xUi3QNchFXzfhCgbALpBr");
    settings.set_default(CONFIG_AGENT_PAIRWISE_VERKEY,"5LXDnRUM7k651nBmhcRraKThVAZYqepaW99zCBYosuwX");
    settings.set_default(CONFIG_ENTERPRISE_DID,"4fUDR9R7fjwELRvH9JT6HH");
    settings.set_default(CONFIG_ENTERPRISE_DID_AGENCY,"KkTVEE7RGg7z2d2hrfM2Hj");
    settings.set_default(CONFIG_ENTERPRISE_DID_AGENT,"515rg5tehHwzJK5ZKwtkBb");
    settings.set_default(CONFIG_ENTERPRISE_NAME,"default");
    settings.set_default(CONFIG_LOGO_URL,"http://www.evernym.com");
    settings.set_default(CONFIG_ENABLE_TEST_MODE,"false");
    settings.set_default(CONFIG_ENTERPRISE_VERKEY,"2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm");

    error::SUCCESS.code_num
}

fn is_valid(value: &str) -> bool {
    for c in value.chars() {
        if !c.is_alphanumeric() && c != '_' { return false;}
    }
    true
}

fn validate_config() -> Result<u32, String> {
    let mut error = String::new();

    //if this fails the program should exit
    let config: HashMap<String, String> = SETTINGS.read().unwrap().deserialize::<HashMap<String, String>>().unwrap();

    for setting in config.iter() {
        let mut valid = true;
        if setting.0 == CONFIG_POOL_NAME && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_POOL_CONFIG_NAME {
            valid = true;
        } else if setting.0 == CONFIG_WALLET_NAME && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENT_ENDPOINT {
            match Url::parse(setting.1) {
                Err(x) => valid = false,
                Ok(_) => valid = true,
            }
        } else if setting.0 == CONFIG_ENTERPRISE_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_ENTERPRISE_DID_AGENCY && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_ENTERPRISE_DID_AGENT && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENCY_PAIRWISE_VERKEY && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENT_PAIRWISE_VERKEY && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENCY_PAIRWISE_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_AGENT_PAIRWISE_DID && !is_valid(setting.1) {
            valid = false;
        } else if setting.0 == CONFIG_ENTERPRISE_NAME {
            valid = true;
        } else if setting.0 == CONFIG_LOGO_URL {
            match Url::parse(setting.1) {
                Err(x) => valid = false,
                Ok(_) => valid = true,
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
        Ok(error::SUCCESS.code_num)
    }
}

pub fn test_mode_enabled() -> bool {
    let config = SETTINGS.read().unwrap();

    match config.get_str(CONFIG_ENABLE_TEST_MODE) {
        Err(_) => false,
        Ok(value) => if value == "true" { true } else { false },
    }
}

pub fn process_config_file(path: &str) -> Result<u32, String> {
    if !Path::new(path).is_file() {
        Err("could not find configuration file".to_owned())
    } else {
        // if this fails the program should exit
        SETTINGS.write().unwrap().merge(config::File::with_name(path)).unwrap();
        info!("path: {}", path);

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


#[cfg(test)]
pub mod tests {
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs;
    use super::*;

    #[test]
    fn test_invalid_config_value() {
        match get_config_value("garbage") {
            Err(x) => assert_eq!(x, error::INVALID_CONFIGURATION.code_num),
            Ok(v) => assert_eq!(v,"totalgarbage"), //if test gets here it will fail
        };
    }

    #[test]
    fn test_bad_path() {
        //test bad path
        let tmp_string = "garbage.txt";
        match process_config_file(&tmp_string) {
            Err(x) => assert_eq!(x,"could not find configuration file"),
            Ok(x) => assert_eq!(x,error::UNKNOWN_ERROR.code_num),  //if test gets here it will fail
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
            Ok(v) => assert_eq!(v, error::UNKNOWN_ERROR.code_num), //fail if we get here
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
        let a = "logo_url";

        let config_path = "/tmp/test_settings.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        //throw in some invalid content to test the validation code
        let content = "{ \"logo_url\" : \"wrong_url\" }";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        match process_config_file(&config_path) {
            Err(v) => assert_eq!(v, "logo_url has invalid setting: wrong_url"),
            Ok(_) => println!("expected invalid URL"), //fail if we get here
        }
        //        assert!(process_config_file(&config_path) == Err(ParseError::InvalidIpv6Address));
    }

    #[test]
    fn test_process_file_with_pairwise_configs() {
        set_defaults();
        let a = "agency_pairwise_did";
        let a_rtn = "72x8p4HubxzUK1dwxcc5FU";
        let b = "agent_pairwise_verkey";
        let b_rtn = "U22jM6Cea2YVixjWwHN9wq";
        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"enterprise\",\
        \"logo_url\" : \"https://s19.postimg.org/ykyz4x8jn/evernym.png\", \"agency_pairwise_verkey\" : \"7118p4HubxzUK1dwxcc5FU\",\
        \"agent_pairwise_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\"}";

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
