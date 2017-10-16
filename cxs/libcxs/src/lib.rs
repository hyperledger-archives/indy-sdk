#![allow(unused_variables)]
#![allow(dead_code)]
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate reqwest;
extern crate mockito;
extern crate config;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;
mod settings;

use std::path::Path;

pub mod api;
pub mod connection;

pub fn create_path(s:&str) -> &Path {
    Path::new(s)
}

