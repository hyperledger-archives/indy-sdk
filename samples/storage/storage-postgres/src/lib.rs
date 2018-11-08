#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

extern crate log;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// Note that to use macroses from indy_common::util inside of other modules it must me loaded first!
#[macro_use]
extern crate indy_common;
extern crate indy_crypto;
extern crate libc;
extern crate rand;
extern crate postgres;

pub mod postgres_wallet;
pub mod postgres_storage;
