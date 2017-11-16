#![allow(unused_variables)]
#![allow(dead_code)]
extern crate serde;
extern crate rand;
extern crate reqwest;
extern crate config;
extern crate url;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;
mod settings;
mod messages;

use std::path::Path;

pub mod api;
pub mod connection;
pub mod issuer_claim;
pub mod claim_request;
pub mod proof;

pub fn create_path(s:&str) -> &Path {
    Path::new(s)
}

