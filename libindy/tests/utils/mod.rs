#![allow(dead_code)]

pub mod callback;

#[path = "../../src/utils/environment.rs"]
pub mod environment;

pub mod pool;
pub mod crypto;
pub mod did;
pub mod wallet;
pub mod ledger;
pub mod anoncreds;
pub mod types;
pub mod pairwise;
pub mod constants;
pub mod blob_storage;
pub mod non_secrets;
pub mod results;
pub mod payments;
pub mod rand_utils;
pub mod logger;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../src/utils/test.rs"]
pub mod test;

pub mod timeout;

#[path = "../../src/utils/sequence.rs"]
pub mod sequence;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../src/utils/cstring.rs"]
pub mod cstring;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../src/utils/byte_array.rs"]
pub mod byte_array;

#[path = "../../src/utils/inmem_wallet.rs"]
pub mod inmem_wallet;

#[path = "../../src/domain/mod.rs"]
pub mod domain;

pub fn setup() {
    test::TestUtils::cleanup_storage();
    logger::LoggerUtils::set_default_logger();
}

pub fn tear_down() {
    test::TestUtils::cleanup_storage();
}

pub fn setup_with_wallet() -> i32 {
    setup();
    wallet::WalletUtils::create_and_open_default_wallet().unwrap()
}

pub fn setup_with_plugged_wallet() -> i32 {
    setup();
    wallet::WalletUtils::create_and_open_plugged_wallet().unwrap()
}

pub fn tear_down_with_wallet(wallet_handle: i32) {
    wallet::WalletUtils::close_wallet(wallet_handle).unwrap();
    tear_down();
}

pub fn setup_with_wallet_and_pool() -> (i32, i32) {
    let wallet_handle = setup_with_wallet();
    let pool_handle = pool::PoolUtils::create_and_open_pool_ledger(constants::POOL).unwrap();
    (wallet_handle, pool_handle)
}

pub fn tear_down_with_wallet_and_pool(wallet_handle: i32, pool_handle: i32) {
    pool::PoolUtils::close(pool_handle).unwrap();
    tear_down_with_wallet(wallet_handle);
}
