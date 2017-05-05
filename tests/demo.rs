// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::test::TestUtils;

#[test]
fn anoncreds_demo_works() {
    TestUtils::cleanup_storage();

    // FIXME: Implement me!!!

    TestUtils::cleanup_storage();
}

#[test]
fn ledger_demo_works() {
    TestUtils::cleanup_storage();

    // FIXME: Implement me!!!

    TestUtils::cleanup_storage();
}