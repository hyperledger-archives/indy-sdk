#![allow(unused_variables)]
#![allow(dead_code)]
#![crate_name = "vcx"]
extern crate serde;
extern crate rand;
extern crate reqwest;
extern crate config;
extern crate url;
extern crate openssl;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod utils;
pub mod settings;
pub mod messages;

use std::path::Path;

pub mod api;
pub mod connection;
pub mod issuer_claim;
pub mod claim_request;
pub mod proof;
pub mod schema;
pub mod claim_def;
pub mod proof_compliance;
pub mod claim;
pub mod object_cache;
pub mod disclosed_proof;

pub fn create_path(s:&str) -> &Path {
    Path::new(s)
}

