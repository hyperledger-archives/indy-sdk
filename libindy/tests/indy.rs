extern crate indy;

use std::ffi::CString;

#[test]
fn set_runtime_config_works() {
    let config = CString::new(r#"{"crypto_thread_pool_size": 2}"#).unwrap();
    indy::api::indy_set_runtime_config(config.as_ptr());
}