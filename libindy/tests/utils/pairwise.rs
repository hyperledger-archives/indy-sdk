extern crate futures;

use indy::IndyError;
use indy::pairwise;
use self::futures::Future;

pub fn pairwise_exists(wallet_handle: i32, their_did: &str) -> Result<bool, IndyError> {
    pairwise::is_pairwise_exists(wallet_handle, their_did).wait()
}

pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: Option<&str>) -> Result<(), IndyError> {
    pairwise::create_pairwise(wallet_handle, their_did, my_did, metadata).wait()
}

pub fn list_pairwise(wallet_handle: i32) -> Result<String, IndyError> {
    pairwise::list_pairwise(wallet_handle).wait()
}

pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Result<String, IndyError> {
    pairwise::get_pairwise(wallet_handle, their_did).wait()
}

pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: Option<&str>) -> Result<(), IndyError> {
    pairwise::set_pairwise_metadata(wallet_handle, their_did, metadata).wait()
}