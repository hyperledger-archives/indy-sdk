use futures::*;
use utils::futures::*;
use indyrs::{did, ErrorCode};

pub fn create_and_store_my_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(String, String), Error=ErrorCode>> {
    did::create_and_store_my_did(wallet_handle, did_info)
        .into_box()
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=ErrorCode>> {
    did::key_for_local_did(wallet_handle, did)
        .into_box()
}

pub fn store_their_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(), Error=ErrorCode>> {
    did::store_their_did(wallet_handle, did_info)
        .into_box()
}

pub fn set_did_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Box<Future<Item=(), Error=ErrorCode>> {
    did::set_did_metadata(wallet_handle, did, metadata)
        .into_box()
}

pub fn get_did_metadata(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=ErrorCode>> {
    did::get_did_metadata(wallet_handle, did)
        .into_box()
}