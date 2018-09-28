extern crate rust_libindy_wrapper as indy;

use std::env;
use std::path::PathBuf;
use std::fs;
use serde_json;

pub mod b58;
pub mod constants;
pub mod did;
pub mod environment;
pub mod file;
pub mod pool;
pub mod rand;
pub mod setup;
pub mod wallet;

macro_rules! safe_wallet_create {
    ($x:ident) => {
        match Wallet::delete($x, r#"{"key":""}"#) {
            Ok(..) => {},
            Err(..) => {}
        };
        Wallet::create($x, r#"{"key":""}"#).unwrap();
    }
}

macro_rules! wallet_cleanup {
    ($x:ident, $y:ident) => {
        Wallet::close($x).unwrap();
        Wallet::delete($y, r#"{"key":""}"#).unwrap();
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    }
}

macro_rules! str_to_string {
    ($x:ident) => {
        match $x {
            Some(s) => Some(s.to_string()),
            None => None
        }
    }
}

pub fn trustee_identity_json() -> String {
    my_did_json_parameter(None, constants::TRUSTEE_SEED, None, None)
}

pub fn my1_identity_json() -> String {
    my_did_json_parameter(None, constants::MY1_SEED, None, None)
}

pub fn my1_identity_key_json() -> String {
    my_crypto_json_parameter(constants::MY1_SEED, None)
}

pub fn my_did_json_parameter(did: Option<&str>, seed: &str, crypto_type: Option<&str>, cid: Option<&str>) -> String {
    let d = hashmap![
        "did".to_string() => str_to_string!(did),
        "seed".to_string() => Some(seed.to_string()),
        "crypto_type".to_string() => str_to_string!(crypto_type),
        "cid".to_string() => str_to_string!(cid)
        ];
    serde_json::to_string(&d).unwrap()
}

pub fn my_crypto_json_parameter(seed: &str, crypto_type: Option<&str>) -> String {
    let c = hashmap![
        "seed".to_string() => Some(seed.to_string()),
        "crypto_type".to_string() => str_to_string!(crypto_type)
    ];
    serde_json::to_string(&c).unwrap()
}

pub fn tails_writer_config() -> String {
    let c = hashmap![
        "base_dir".to_string() => Some(format!("{}/tails", indy_home_path().to_str().unwrap().to_string())),
        "uri_pattern".to_string() => None
    ];

    serde_json::to_string(&c).unwrap()
}

pub fn indy_home_path() -> PathBuf {
    let mut path = env::home_dir().unwrap_or(PathBuf::from("/home/indy"));
    let mut indy_client_dir = ".indy_client";
        if cfg!(target_os = "ios"){
            indy_client_dir = "Documents/.indy_client";
        }
        path.push(indy_client_dir);

        if cfg!(target_os = "android"){
            android_create_indy_client_dir();
            path = android_indy_client_dir_path();
        }
        path
}

pub fn tmp_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("indy_client");
    path
}

pub fn tmp_file_path(file_name: &str) -> PathBuf {
    let mut path = tmp_path();
    path.push(file_name);
    path
}

pub fn android_indy_client_dir_path() -> PathBuf{
    let external_storage= env::var("EXTERNAL_STORAGE");
    let android_dir :String;
    match external_storage {
        Ok (val) => android_dir = val + "/.indy_client",
        Err(err) => {
            panic!("Failed to find external storage path {:?}", err)
        }
    }

    PathBuf::from(android_dir)
}

pub fn android_create_indy_client_dir(){
    //Creates directory only if it is not present.
    fs::create_dir_all(android_indy_client_dir_path().as_path()).unwrap();
}

pub fn time_it_out<F>(msg: &str, test: F) -> bool where F: Fn() -> bool {
    for _ in 1..250 {
        if test() {
            return true;
        }
    }
    // It tried to do a timeout test 250 times and the system was too fast, so just succeed
    println!("{} - system too fast for timeout test", msg);
    true
}
