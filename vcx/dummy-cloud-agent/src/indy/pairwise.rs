use futures::*;
use utils::futures::*;
use indyrs::{pairwise, IndyError};

#[derive(Deserialize, Debug)]
pub struct Pairwise {
    pub my_did: String,
    pub their_did: String,
    pub metadata: String,
}

#[derive(Deserialize, Debug)]
pub struct PairwiseInfo {
    pub my_did: String,
    pub metadata: String,
}


pub fn is_pairwise_exists(wallet_handle: i32, their_did: &str) -> Box<Future<Item=bool, Error=IndyError>> {
    pairwise::is_pairwise_exists(wallet_handle, their_did)
        .into_box()
}

pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: Option<&str>) -> Box<Future<Item=(), Error=IndyError>> {
    pairwise::create_pairwise(wallet_handle, their_did, my_did, metadata)
        .into_box()
}

pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    pairwise::get_pairwise(wallet_handle, their_did)
        .into_box()
}

pub fn list_pairwise(wallet_handle: i32) -> Box<Future<Item=String, Error=IndyError>> {
    pairwise::list_pairwise(wallet_handle)
        .into_box()
}

pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
    pairwise::set_pairwise_metadata(wallet_handle, their_did, Some(metadata))
        .into_box()
}