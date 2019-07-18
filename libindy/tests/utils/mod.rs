#![allow(dead_code, unused_macros)]

extern crate libc;

use utils::constants::WALLET_CREDENTIALS;

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
pub mod cache;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../src/utils/test.rs"]
pub mod test;

pub mod timeout;

#[path = "../../src/utils/sequence.rs"]
pub mod sequence;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../src/utils/ctypes.rs"]
pub mod ctypes;

#[path = "../../src/utils/inmem_wallet.rs"]
pub mod inmem_wallet;

#[path = "../../src/domain/mod.rs"]
pub mod domain;

pub fn setup(name: &str) {
    test::cleanup_storage(name);
    logger::set_default_logger();
}

pub fn tear_down_delete_wallet(wallet_config: &str) {
    wallet::delete_wallet(wallet_config, WALLET_CREDENTIALS).unwrap();
}

pub fn tear_down_delete_wallet_with_credentials(wallet_config: &str, wallet_credentials: &str) {
    wallet::delete_wallet(wallet_config, wallet_credentials).unwrap();
}

pub fn tear_down(name: &str) {
    test::cleanup_storage(name);
}

pub fn setup_with_wallet(name: &str) -> (i32, String) {
    setup(name);
    wallet::create_and_open_default_wallet(name).unwrap()
}

pub fn setup_with_plugged_wallet(name: &str) -> (i32, String) {
    setup(name);
    wallet::create_and_open_plugged_wallet().unwrap()
}

pub fn tear_down_with_wallet(wallet_handle: i32, name: &str, wallet_config: &str) {
    wallet::close_wallet(wallet_handle).unwrap();
    wallet::delete_wallet(wallet_config, WALLET_CREDENTIALS).unwrap();
    tear_down(name);
}

pub fn setup_with_pool(name: &str) -> i32 {
    setup(name);
    pool::create_and_open_pool_ledger(name).unwrap()
}

pub fn tear_down_with_pool(pool_handle: i32, name: &str) {
    pool::close(pool_handle).unwrap();
    tear_down(name);
}

pub fn setup_with_wallet_and_pool(name: &str) -> (i32, i32, String) {
    let (wallet_handle, config) = setup_with_wallet(name);
    let pool_handle = pool::create_and_open_pool_ledger(name).unwrap();
    (wallet_handle, pool_handle, config)
}

pub fn tear_down_with_wallet_and_pool(wallet_handle: i32, pool_handle: i32, name: &str, wallet_config: &str) {
    pool::close(pool_handle).unwrap();
    tear_down_with_wallet(wallet_handle, name, wallet_config);
}

pub fn setup_trustee(name: &str) -> (i32, i32, String, String) {
    let (wallet_handle, pool_handle, config) = setup_with_wallet_and_pool(name);
    let (did, _) = did::create_and_store_my_did(wallet_handle, Some(constants::TRUSTEE_SEED)).unwrap();
    (wallet_handle, pool_handle, did, config)
}

pub fn setup_steward(name: &str) -> (i32, i32, String, String) {
    let (wallet_handle, pool_handle, config) = setup_with_wallet_and_pool(name);
    let (did, _) = did::create_and_store_my_did(wallet_handle, Some(constants::STEWARD_SEED)).unwrap();
    (wallet_handle, pool_handle, did, config)
}

pub fn setup_did(name: &str) -> (i32, String, String) {
    let (wallet_handle, config) = setup_with_wallet(name);
    let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();
    (wallet_handle, did, config)
}

pub fn setup_new_identity(name: &str) -> (i32, i32, String, String, String) {
    let (wallet_handle, pool_handle, trustee_did, config) = setup_trustee(name);

    let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
    let nym = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("TRUSTEE")).unwrap();
    let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym).unwrap();
    pool::check_response_type(&response, types::ResponseType::REPLY);

    (wallet_handle, pool_handle, my_did, my_vk, config)
}