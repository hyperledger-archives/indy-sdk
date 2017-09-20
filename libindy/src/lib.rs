// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
mod utils;

pub mod api;
mod commands;
mod errors;
mod services;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn dummy() {
        assert! (true, "Dummy check!");
    }
}
