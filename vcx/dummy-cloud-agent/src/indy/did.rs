use futures::*;
use indyrs::{did, IndyError, WalletHandle};

use crate::utils::futures::*;

pub fn create_and_store_my_did(wallet_handle: WalletHandle, did_info: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    did::create_and_store_my_did(wallet_handle, did_info)
        .into_box()
}

pub fn key_for_local_did(wallet_handle: WalletHandle, did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    did::key_for_local_did(wallet_handle, did)
        .into_box()
}

pub fn store_their_did(wallet_handle: WalletHandle, did_info: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    did::store_their_did(wallet_handle, did_info)
        .into_box()
}

pub fn set_did_metadata(wallet_handle: WalletHandle, did: &str, metadata: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    did::set_did_metadata(wallet_handle, did, metadata)
        .into_box()
}

pub fn get_did_metadata(wallet_handle: WalletHandle, did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    did::get_did_metadata(wallet_handle, did)
        .into_box()
}