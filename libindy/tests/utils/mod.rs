#![allow(dead_code, unused_macros)]

extern crate libc;

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

#[path = "../../src/utils/validation.rs"]
pub mod validation;

#[path = "../../src/utils/inmem_wallet.rs"]
pub mod inmem_wallet;

#[path = "../../src/domain/mod.rs"]
pub mod domain;

fn setup() -> String {
    let name = ::utils::rand_utils::get_rand_string(10);
    test::cleanup_storage(&name);
    logger::set_default_logger();
    name
}

fn tear_down(name: &str) {
    test::cleanup_storage(name);
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
        let name = setup();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn wallet() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn plugged_wallet() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_plugged_wallet().unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn pool() -> Setup {
        let name = setup();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle, did: String::new(), verkey: String::new() }
    }

    pub fn wallet_and_pool() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did: String::new(), verkey: String::new() }
    }

    pub fn trustee() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(constants::TRUSTEE_SEED)).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn steward() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(constants::STEWARD_SEED)).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn endorser() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_store_and_publish_did(wallet_handle, pool_handle, "ENDORSER").unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn new_identity() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        let (did, verkey) = did::create_store_and_publish_did(wallet_handle, pool_handle, "TRUSTEE").unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle, did, verkey }
    }

    pub fn did() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did, verkey }
    }

    pub fn key() -> Setup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let verkey = crypto::create_key(wallet_handle, None).unwrap();
        Setup { name, wallet_config, wallet_handle, pool_handle: 0, did: String::new(), verkey }
    }

    pub fn payment() -> Setup {
        let name = setup();
        payments::mock_method::init();
        Setup { name, wallet_config: String::new(), wallet_handle: 0, pool_handle: 0, did: String::new(), verkey: String::new() }
    }

    pub fn payment_wallet() -> Setup {
        let name = setup();
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