use super::{IndyHandle};

use indy::did::{Did as IndyDid};
use indy::ErrorCode;
use indy::future::Future;

pub struct Did {}

impl Did {
    pub fn new(wallet_handle: IndyHandle, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        IndyDid::new(wallet_handle, my_did_json).wait()
    }

    pub fn replace_keys_start(wallet_handle: i32, did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        IndyDid::replace_keys_start(wallet_handle, did, identity_json).wait()
    }

    pub fn replace_keys_apply(wallet_handle: i32, did: &str) -> Result<(), ErrorCode> {
        IndyDid::replace_keys_apply(wallet_handle, did).wait()
    }

    pub fn set_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), ErrorCode> {
        IndyDid::set_metadata(wallet_handle, did, metadata).wait()
    }

    pub fn get_did_with_meta(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        IndyDid::get_my_metadata(wallet_handle, did).wait()
    }

    pub fn list_dids_with_meta(wallet_handle: i32) -> Result<String, ErrorCode> {
        IndyDid::list_with_metadata(wallet_handle).wait()
    }

    pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, ErrorCode> {
        IndyDid::abbreviate_verkey(did, verkey).wait()
    }
}