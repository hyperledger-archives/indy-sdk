extern crate indyrs as indy;
use std::env;

#[test]
fn set_runtime_config_works() {
    indy::set_runtime_config(r#"{"crypto_thread_pool_size": 2}"#);
}

#[test]
fn set_runtime_config_works_for_freshness_threshold() {
    indy::set_runtime_config(r#"{"freshness_threshold": 352}"#);
    assert_eq!(env::var("FRESHNESS_THRESHOLD").unwrap(), "352");
}