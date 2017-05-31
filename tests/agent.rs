// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]
extern crate sovrin;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::agent::AgentUtils;
use utils::logger::LoggerUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::wallet::WalletUtils;

#[test]
fn sovrin_agent_connect_works_for_all_data_in_wallet_present() {
    LoggerUtils::init();
    TestUtils::cleanup_storage();

    let wallet_handle = WalletUtils::create_wallet("pool1", "wallet1", "default").expect("create wallet");

    let (did, ver_key, pub_key) = SignusUtils::create_and_store_my_did(wallet_handle).unwrap();
    SignusUtils::store_their_did(wallet_handle, did.as_str(), ver_key.as_str(), pub_key.as_str(), "endpoint").unwrap();

    let connect_handle = AgentUtils::connect(wallet_handle, did.as_str(), did.as_str()).unwrap();

    TestUtils::cleanup_storage();
}
