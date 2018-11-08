#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

#[macro_use]
extern crate log;

extern crate serde;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

//extern crate rmp_serde;

#[macro_use]
extern crate lazy_static;

extern crate indy_crypto;
extern crate libsqlite3_sys;
extern crate libc;
extern crate rand;
extern crate rusqlite;
extern crate postgres;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
pub mod utils;
pub mod errors;
pub mod api;
pub mod wallet_storage;

