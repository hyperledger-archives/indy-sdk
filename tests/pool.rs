extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use sovrin::api::ErrorCode;

use utils::pool::PoolUtils;
use utils::test::TestUtils;


#[test]
fn create_pool_ledger_config_works() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool_create");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
fn open_pool_ledger_works() {
    TestUtils::cleanup_storage();
    let name = "pool_open";
    let res = PoolUtils::create_pool_ledger_config(name);
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger(name);
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
fn open_pool_ledger_works_for_twice() {
    TestUtils::cleanup_storage();
    let pool_name = "pool_open_twice";

    let res = PoolUtils::create_pool_ledger_config(pool_name);
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger(pool_name);
    assert!(res.is_ok());
    let res = PoolUtils::open_pool_ledger(pool_name);
    assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

    TestUtils::cleanup_storage();
}