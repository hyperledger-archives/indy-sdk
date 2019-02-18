#![allow(dead_code, unused_macros)]

pub mod callback;

#[macro_use]
#[path = "../../src/utils/memzeroize.rs"]
pub mod zeroize;

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
#[path = "../../src/utils/ctypes.rs"]
pub mod ctypes;

#[path = "../../src/utils/inmem_wallet.rs"]
pub mod inmem_wallet;

#[path = "../../src/domain/mod.rs"]
pub mod domain;

pub fn setup() {
    test::cleanup_storage();
    logger::set_default_logger();
}

pub fn tear_down() {
    test::cleanup_storage();
}

pub fn setup_with_wallet() -> i32 {
    setup();
    wallet::create_and_open_default_wallet().unwrap()
}

pub fn setup_with_plugged_wallet() -> i32 {
    setup();
    wallet::create_and_open_plugged_wallet().unwrap()
}

pub fn tear_down_with_wallet(wallet_handle: i32) {
    wallet::close_wallet(wallet_handle).unwrap();
    tear_down();
}

pub fn setup_with_pool() -> i32 {
    setup();
    pool::create_and_open_pool_ledger(constants::POOL).unwrap()
}

pub fn tear_down_with_pool(pool_handle: i32) {
    pool::close(pool_handle).unwrap();
    tear_down();
}

pub fn setup_with_wallet_and_pool() -> (i32, i32) {
    let wallet_handle = setup_with_wallet();
    let pool_handle = pool::create_and_open_pool_ledger(constants::POOL).unwrap();
    (wallet_handle, pool_handle)
}

pub fn tear_down_with_wallet_and_pool(wallet_handle: i32, pool_handle: i32) {
    pool::close(pool_handle).unwrap();
    tear_down_with_wallet(wallet_handle);
}

pub fn setup_trustee() -> (i32, i32, String) {
    let (wallet_handle, pool_handle) = setup_with_wallet_and_pool();
    let (did, _) = did::create_and_store_my_did(wallet_handle, Some(constants::TRUSTEE_SEED)).unwrap();
    (wallet_handle, pool_handle, did)
}

pub fn setup_steward() -> (i32, i32, String) {
    let (wallet_handle, pool_handle) = setup_with_wallet_and_pool();
    let (did, _) = did::create_and_store_my_did(wallet_handle, Some(constants::STEWARD_SEED)).unwrap();
    (wallet_handle, pool_handle, did)
}

pub fn setup_did() -> (i32, String) {
    let wallet_handle = setup_with_wallet();
    let (did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();
    (wallet_handle, did)
}

pub fn setup_new_identity() -> (i32, i32, String, String) {
    let (wallet_handle, pool_handle, trustee_did) = setup_trustee();

    let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
    let nym = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("TRUSTEE")).unwrap();
    let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym).unwrap();
    pool::check_response_type(&response, types::ResponseType::REPLY);

    (wallet_handle, pool_handle, my_did, my_vk)
}