extern crate indy;
extern crate serde;
extern crate serde_json;
extern crate rand;

use std::path::Path;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

pub mod api;
pub mod error;
pub mod connection;

pub fn create_path(s:&str) -> &Path {
    Path::new(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txn_file_exists(){
        let p = String::from("/home/mark/genesis.txn");
        assert!(create_path(&p).exists());

    }

}
