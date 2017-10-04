extern crate indy;
extern crate serde;
extern crate serde_json;
extern crate rand;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

use std::ffi::CString;
use indy::api::ErrorCode;
use indy::api::pool::indy_create_pool_ledger_config;

pub mod api;
pub mod error;
pub mod connection;

pub fn config()-> ErrorCode {
    let pool_name = "pool1";
    let config_name = "config1";
    let c_pool_name = CString::new(pool_name).unwrap();
    let c_config_name = CString::new(config_name).unwrap();
    let command_handle: i32 = 1;
    extern "C" fn f(_handle: i32, _err: ErrorCode) { }

    indy_create_pool_ledger_config(command_handle,
                                    c_pool_name.as_ptr(),
                                    c_config_name.as_ptr(),
                                    Some(f))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config(){
        assert_eq!(ErrorCode::Success,config());
    }

}
