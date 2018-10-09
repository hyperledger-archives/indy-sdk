use super::environment;
use super::logger;

use std::fs;

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
    logger::set_default_indy_logger();
}

pub fn tear_down() {
    cleanup_storage();
}