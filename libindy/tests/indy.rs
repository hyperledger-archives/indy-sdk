extern crate named_type;
extern crate indy_crypto;
extern crate time;
#[macro_use]
extern crate lazy_static;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate indy_sys;

extern crate serde;
#[macro_use]
extern crate named_type_derive;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

#[test]
fn set_runtime_config_works() {
    indy::Indy::set_runtime_config(r#"{"crypto_thread_pool_size": 2}"#);
}