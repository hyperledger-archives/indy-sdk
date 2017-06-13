//// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use sovrin::api::ErrorCode;

use utils::pool::PoolUtils;
use utils::test::TestUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::logger::LoggerUtils;


mod high_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            PoolUtils::create_pool_ledger_config("pool_create").unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works() {
            TestUtils::cleanup_storage();
            let name = "pool_open";

            PoolUtils::create_pool_ledger_config(name).unwrap();

            PoolUtils::open_pool_ledger(name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_twice() {
            TestUtils::cleanup_storage();
            LoggerUtils::init();
            let pool_name = "pool_open_twice";

            PoolUtils::create_pool_ledger_config(pool_name).unwrap();

            PoolUtils::open_pool_ledger(pool_name).unwrap();
            let res = PoolUtils::open_pool_ledger(pool_name);
            assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

            TestUtils::cleanup_storage();
        }
    }
}