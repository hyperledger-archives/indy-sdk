#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

//#[macro_use]
//extern crate log;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

//extern crate rmp_serde;

#[macro_use]
extern crate lazy_static;

//extern crate named_type;

//#[macro_use]
//extern crate named_type_derive;

extern crate indy;
extern crate indy_crypto;
extern crate libc;
extern crate rand;
extern crate postgres;


// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
pub mod utils;

pub mod postgres_wallet;
pub mod postgres_storage;

//pub mod api;
//mod errors;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn dummy() {
        assert!(true, "Dummy check!");
    }
}

