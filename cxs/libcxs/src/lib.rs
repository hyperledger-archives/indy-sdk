use std::path::Path;

extern crate indy;

#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod utils;

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
