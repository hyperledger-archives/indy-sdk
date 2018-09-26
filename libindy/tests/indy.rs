extern crate indy;

#[test]
fn set_crypto_thread_pool_size() {
    indy::api::indy_set_crypto_thread_pool_size(2);
}