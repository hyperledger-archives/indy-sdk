// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config, sovrin_open_pool_ledger};

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::pool::PoolUtils;
use utils::test::TestUtils;

use std::fs;
use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

#[test]
fn create_pool_ledger_config_works() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool1");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
fn open_pool_ledger_works() {
    let res = PoolUtils::create_pool_ledger_config("pool1");
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger("pool1");
    assert!(res.is_ok());

    TestUtils::cleanup_storage();
}

#[test]
fn open_pool_ledger_works_for_twice() {
    TestUtils::cleanup_storage();

    let res = PoolUtils::create_pool_ledger_config("pool1");
    assert!(res.is_ok());

    let res = PoolUtils::open_pool_ledger("pool1");
    assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

    TestUtils::cleanup_storage();
}
