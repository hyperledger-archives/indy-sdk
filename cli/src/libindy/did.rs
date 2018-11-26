use super::{IndyHandle};

use indy::did;
use indy::ErrorCode;
use indy::future::Future;

pub struct Did {}

impl Did {
    pub fn new(wallet_handle: IndyHandle, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        did::create_and_store_my_did(wallet_handle, my_did_json).wait()
    }

    pub fn replace_keys_start(wallet_handle: i32, did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        did::replace_keys_start(wallet_handle, did, identity_json).wait()
    }

    pub fn replace_keys_apply(wallet_handle: i32, did: &str) -> Result<(), ErrorCode> {
        did::replace_keys_apply(wallet_handle, did).wait()
    }

    pub fn set_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), ErrorCode> {
        did::set_did_metadata(wallet_handle, did, metadata).wait()
    }

    pub fn get_did_with_meta(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        did::get_my_did_with_metadata(wallet_handle, did).wait()
    }

    pub fn list_dids_with_meta(wallet_handle: i32) -> Result<String, ErrorCode> {
        did::list_my_dids_with_metadata(wallet_handle).wait()
    }

    pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, ErrorCode> {
        did::abbreviate_verkey(did, verkey).wait()
    }
}