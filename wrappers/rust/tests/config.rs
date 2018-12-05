extern crate indyrs as indy;

mod tests {
    use super::*;

    #[test]
    fn set_runtime_config_works () {
        indy::set_runtime_config(r#"{"crypto_thread_pool_size": 2}"#);
    }
}
