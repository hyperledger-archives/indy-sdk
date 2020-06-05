#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as indy;
extern crate indyrs as api;
use crate::utils::wallet;
use crate::utils::test;
use crate::utils::logger;
use crate::utils::constants::*;

#[test]
fn indy_set_logger_works() {
    const DEFAULT_WALLET_CONFIG: &str = r#"{"id":"indy_set_logger_works","storage_type":"default"}"#;
    test::cleanup_storage("indy_set_logger_works");

    wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    log::set_boxed_logger(Box::new(logger::SimpleLogger {})).ok();

    logger::set_logger(log::logger());

    let wallet_handle = wallet::open_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    wallet::close_wallet(wallet_handle).unwrap();
    test::cleanup_storage("indy_set_logger_works");
}

#[test]
fn indy_set_default_logger_works() {
    const DEFAULT_WALLET_CONFIG: &str = r#"{"id":"indy_set_default_logger_works","storage_type":"default"}"#;
    test::cleanup_storage("indy_set_default_logger_works");

    wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    logger::set_default_logger();

    let wallet_handle = wallet::open_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    wallet::close_wallet(wallet_handle).unwrap();
    test::cleanup_storage("indy_set_default_logger_works");
}
