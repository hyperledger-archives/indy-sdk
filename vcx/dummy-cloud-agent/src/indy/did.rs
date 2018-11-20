use futures::*;
use super::IndyError;
use utils::futures::*;
use indyrs::did::Did as did;

pub fn create_and_store_my_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    did::new(wallet_handle, did_info)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    did::get_ver_key_local(wallet_handle, did)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn store_their_did(wallet_handle: i32, did_info: &str) -> Box<Future<Item=(), Error=IndyError>> {
    did::store_their_did(wallet_handle, did_info)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn set_did_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
    did::set_metadata(wallet_handle, did, metadata)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn get_did_metadata(wallet_handle: i32, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    did::get_metadata(wallet_handle, did)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}