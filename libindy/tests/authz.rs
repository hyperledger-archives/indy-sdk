extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::authz::AuthzUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::crypto::CryptoUtils;
use utils::constants::*;

use indy::api::ErrorCode;

use serde_json::{Value, Error};


#[cfg(feature = "local_nodes_pool")]
use std::thread;

mod high_cases {
    use super::*;

    mod policy_creation {
        use super::*;

        // TODO: Tests contain duplicateed setup code, fix it

        #[test]
        fn indy_policy_creation_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let policy_json = AuthzUtils::create_and_store_policy_address(wallet_handle).unwrap();
            println!("{:?}", policy_json);

            let policy: Value = serde_json::from_str(&policy_json).unwrap();
            println!("{:?}", policy);

            let policy_json1 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  policy["address"].as_str().unwrap()).unwrap();
            println!("{:?}", policy_json1);

            assert_eq!(policy_json, policy_json1);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_new_agent_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let policy_json = AuthzUtils::create_and_store_policy_address(wallet_handle).unwrap();
            println!("{:?}", policy_json);

            let policy: Value = serde_json::from_str(&policy_json).unwrap();
            println!("{:?}", policy);

            let policy_address = policy["address"].as_str().unwrap();
            println!("{:?}", &policy_address);

            let vk1 = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let verkey1 = AuthzUtils::add_agent_to_policy_in_wallet(wallet_handle, &policy_address, &vk1, false).unwrap();

            let policy_json1 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  &policy_address).unwrap();
            println!("{:?}", policy_json1);

            let policy1: Value = serde_json::from_str(&policy_json1).unwrap();
            println!("{:?}", policy1);

            let agents = &policy1["agents"];
            println!("{:?}", agents);

            let agent1 = &agents[verkey1];
            println!("{:?}", agent1);

            assert_eq!(agent1["secret"], Value::Null);
            assert_eq!(agent1["blinding_factor"], Value::Null);
            assert_eq!(agent1["double_commitment"], Value::Null);
            assert_eq!(agent1["witness"], Value::Null);

            let vk2 = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let verkey2 = AuthzUtils::add_agent_to_policy_in_wallet(wallet_handle, &policy_address,
                                                                    &vk2, true).unwrap();

            let policy_json2 = AuthzUtils::get_policy_from_wallet(wallet_handle,
                                                                  &policy_address).unwrap();
            println!("{:?}", policy_json2);

            let policy2: Value = serde_json::from_str(&policy_json2).unwrap();
            println!("{:?}", policy2);

            let agents = &policy2["agents"];
            println!("{:?}", agents);

            let agent2 = &agents[verkey2];
            println!("{:?}", agent2);
            assert_ne!(agent2["secret"], Value::Null);
            assert_ne!(agent2["blinding_factor"], Value::Null);
            assert_ne!(agent2["double_commitment"], Value::Null);
            assert_eq!(agent2["witness"], Value::Null);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}