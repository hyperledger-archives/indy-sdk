extern crate libc;
extern crate byteorder;
extern crate serde_json;
extern crate rmp_serde;
extern crate time;
extern crate futures;
extern crate indyrs as indy;

use self::indy::ErrorCode;
use self::indy::pairwise::Pairwise;
use self::futures::Future;

pub fn pairwise_exists(wallet_handle: i32, their_did: &str) -> Result<bool, ErrorCode> {
    Pairwise::does_exist(wallet_handle, their_did).wait()
}

pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
    Pairwise::create(wallet_handle, their_did, my_did, metadata).wait()
}

pub fn list_pairwise(wallet_handle: i32) -> Result<String, ErrorCode> {
    Pairwise::list(wallet_handle).wait()
}

pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Result<String, ErrorCode> {
    Pairwise::get(wallet_handle, their_did).wait()
}

pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
    Pairwise::set_metadata(wallet_handle, their_did, metadata).wait()
}