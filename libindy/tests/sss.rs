extern crate indy;
extern crate indy_crypto;

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
use utils::sss::SSSUtils;
use utils::test::TestUtils;
use utils::crypto::CryptoUtils;
use utils::constants::*;

use indy::api::ErrorCode;

use serde_json::{Value, Error};

use std::str;

mod high_cases {
    use super::*;
    use rust_base58::FromBase58;

    mod shard_creation {
        use super::*;

        // TODO: Tests contain duplicated setup code, fix it

        #[test]
        fn indy_secret_sharding_basic() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            let vk = SSSUtils::shard_msg_with_secret_and_store_shards(wallet_handle, 3, 5, None, &verkey).unwrap();
            println!("verkey={:?}, vk={:?}", verkey, vk);

            let shards_json = SSSUtils::get_shards_of_verkey(wallet_handle,
                                                                  &vk).unwrap();
            println!("{:?}", &shards_json);

            let secret = SSSUtils::get_recover_secret_from_shards(wallet_handle, &shards_json).unwrap();
            println!("{:?}", &secret);

//            let secret_json = str::from_utf8(Base58::decode(*secret)).unwrap();
            let secret_json_str = &secret.from_base58().unwrap();
            let secret_json = str::from_utf8(secret_json_str).unwrap();
            println!("{:?}", &secret_json);

            let v: Value = serde_json::from_str(secret_json).unwrap();
            let s = v[format!("__key__::{}", verkey)].as_str().unwrap();
            assert_eq!(MY1_SEED, str::from_utf8(&s.from_base58().unwrap()).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}