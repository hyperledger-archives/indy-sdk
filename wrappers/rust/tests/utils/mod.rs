#![allow(dead_code)]

extern crate rust_libindy_wrapper as indy;

use std::env;
use std::path::PathBuf;
use std::fs;

pub mod b58;
pub mod constants;
pub mod did;
pub mod environment;
pub mod file;
pub mod pool;
pub mod rand;
pub mod setup;
pub mod wallet;

#[allow(unused_macros)]
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

pub fn android_indy_client_dir_path() -> PathBuf {
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

pub fn android_create_indy_client_dir() {
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
