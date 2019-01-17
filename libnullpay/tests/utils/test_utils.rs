use super::environment;

use std::fs;
use indy::logger::set_default_logger;

pub fn cleanup_indy_home() {
    let path = environment::indy_home_path();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

pub fn cleanup_temp() {
    let path = environment::tmp_path();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

pub fn cleanup_storage() {
    cleanup_indy_home();
    cleanup_temp();
}

pub fn setup() {
    cleanup_storage();
    set_default_logger(None).ok();
}

pub fn tear_down() {
    cleanup_storage();
}