#![allow(dead_code)]
/*
We allow dead code because this module is imported for every integration test.
It expects all code to be used in each integration test.
Without this, we are warned of all unused code in each integration test.
*/

extern crate indyrs as indy;

pub mod b58;
pub mod constants;
pub mod did;
pub mod environment;
pub mod file;
pub mod pool;
pub mod rand;
pub mod setup;
pub mod wallet;

#[allow(unused_macros)]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    }
}
