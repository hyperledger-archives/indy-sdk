extern crate config;

use config::Config;
use std::sync::RwLock;
use utils::error;
use std::path::Path;


pub static CONFIG_POOL_NAME: &'static str = "pool_name";
pub static CONFIG_POOL_CONFIG_NAME: &'static str = "config_name";
pub static CONFIG_WALLET_NAME: &'static str = "wallet_name";
pub static CONFIG_WALLET_TYPE: &'static str = "wallet_type";
pub static CONFIG_AGENT_ENDPOINT: &'static str = "agent_endpoint";

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn set_defaults() -> u32 {

    let mut settings = match SETTINGS.write() {
        Err(_) => return error::UNKNOWN_ERROR.code_num,
        Ok(y) => y,
    };

    settings.set_default(CONFIG_POOL_NAME,"pool1");
    settings.set_default(CONFIG_POOL_CONFIG_NAME,"config1");
    settings.set_default(CONFIG_WALLET_NAME,"wallet1");
    settings.set_default(CONFIG_WALLET_TYPE,"default");
    settings.set_default(CONFIG_AGENT_ENDPOINT,"http://127.0.0.1:8080");

    error::SUCCESS.code_num
}

pub fn process_config_file(path: &str) -> u32 {

    if !Path::new(path).is_file() {return error::UNKNOWN_ERROR.code_num}

    match SETTINGS.write() {
        Err(_) => return error::UNKNOWN_ERROR.code_num,
        Ok(mut y) => y.merge(config::File::with_name(path)).unwrap(),
    };

    //TODO: checks parameters

    error::SUCCESS.code_num
}

pub fn get_config_value(key: &str) -> Result<String, u32> {

    match SETTINGS.read() {
        Err(_) => Err(error::UNKNOWN_ERROR.code_num),
        Ok(y) => match y.get_str(key) {
            Err(_) => Err(error::UNKNOWN_ERROR.code_num),
            Ok(value) => Ok(value),
        },
    }
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
            Err(x) => assert_eq!(x, error::UNKNOWN_ERROR.code_num),
            Ok(v) => assert_eq!(v,"totalgarbage"), //if test gets here it will fail
        };
    }

    #[test]
    fn test_bad_path() {
        //test bad path
        let tmp_string = "garbage.txt";
        let rc = process_config_file(&tmp_string);
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    fn test_process_config_file() {
        let a = "a";
        let b = "b";
        let c = "c";
        let d = "d";

        let config_path = "/tmp/test_settings.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"a\" : \"a\", \"b\":\"b\", \"c\":\"c\", \"d\":\"d\" }";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        let rc = process_config_file(&config_path);
        assert_eq!(rc, error::SUCCESS.code_num);

        match get_config_value(&a) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,a),
        };

        match get_config_value(&b) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,b),
        };

        match get_config_value(&c) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,c),
        };

        match get_config_value(&d) {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) => assert_eq!(v,d),
        };

        // Leave file around or other concurrent tests will fail
        //fs::remove_file(config_path).unwrap();
    }
}
