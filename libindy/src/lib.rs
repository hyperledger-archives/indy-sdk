extern crate base64;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate openssl;

extern crate named_type;
#[macro_use]
extern crate named_type_derive;

extern crate rusqlite;
extern crate sodiumoxide;
extern crate libsqlite3_sys;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
mod utils;

pub mod api;
mod commands;
mod errors;
mod services;
mod domain;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn dummy() {
        assert!(true, "Dummy check!");
    }
}
