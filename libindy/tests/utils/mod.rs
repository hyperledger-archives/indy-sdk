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

pub fn setup_() -> String {
    let name = ::utils::rand_utils::get_rand_string(10);
    test::cleanup_storage(&name);
    logger::set_default_logger();
    name
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

pub struct Setup {
    pub name: String,
    pub wallet_config: String,
    pub wallet_handle: i32,
    pub pool_handle: i32,
    pub did: String,
    pub verkey: String
}

impl Setup {
    pub fn empty() -> Setup {
        let name = setup_();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn wallet() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn plugged_wallet() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_plugged_wallet().unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn pool() -> Setup {
        let name = setup_();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle, did: String::new(), verkey: String::new() }
    }

    pub fn wallet_and_pool() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did: String::new(), verkey: String::new() }
    }

    pub fn trustee() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(constants::TRUSTEE_SEED)).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn steward() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(constants::STEWARD_SEED)).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn endorser() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_store_and_publish_did(wallet_handle, pool_handle, "ENDORSER").unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn new_identity() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_store_and_publish_did(wallet_handle, pool_handle, "TRUSTEE").unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn did() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did, verkey }
    }

    pub fn key() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let verkey = crypto::create_key(wallet_handle, None).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey }
    }

    pub fn payment() -> Setup {
        let name = setup_();
        payments::mock_method::init();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn payment_wallet() -> Setup {
        let name = setup_();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        payments::mock_method::init();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey: String::new() }
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        if self.wallet_handle != 0 {
            wallet::close_and_delete_wallet(self.wallet_handle, &self.wallet_config).unwrap();
        }
        if self.pool_handle != 0 {
            pool::close(self.pool_handle).unwrap();
        }
        tear_down(&self.name)
    }
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

pub fn setup_new_endorser(name: &str) -> (i32, i32, String, String, String) {
    let (wallet_handle, pool_handle, trustee_did, config) = setup_trustee(name);

    let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
    let nym = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("ENDORSER")).unwrap();
    let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym).unwrap();
    pool::check_response_type(&response, types::ResponseType::REPLY);

    (wallet_handle, pool_handle, my_did, my_vk, config)
}