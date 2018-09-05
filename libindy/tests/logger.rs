#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indy;
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
mod utils;

use utils::wallet;
use utils::test;
use utils::logger;
use utils::constants::*;

#[test]
fn indy_set_logger_works() {
    test::cleanup_storage();

    wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    logger::set_logger();

    let wallet_handle = wallet::open_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    wallet::close_wallet(wallet_handle).unwrap();
    test::cleanup_storage();
}

#[test]
fn indy_set_default_logger_works() {
    test::cleanup_storage();

    wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    logger::set_default_logger();

    let wallet_handle = wallet::open_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

    wallet::close_wallet(wallet_handle).unwrap();
    test::cleanup_storage();
}

