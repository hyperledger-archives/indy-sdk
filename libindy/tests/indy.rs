extern crate indyrs as indy;

#[test]
fn set_runtime_config_works() {
    indy::Indy::set_runtime_config(r#"{"crypto_thread_pool_size": 2}"#);
}