use super::{ErrorCode, IndyHandle};

use std::ffi::CString;

use ffi::did;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

pub struct Did {}

impl Did {
    pub fn new(wallet_handle: IndyHandle, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let my_did_json = c_str!(my_did_json);

        let err = unsafe {
            did::indy_create_and_store_my_did(command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn replace_keys_start(wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let tgt_did = c_str!(tgt_did);
        let identity_json = c_str!(identity_json);

        let err = unsafe {
            did::indy_replace_keys_start(command_handle,
                                    wallet_handle,
                                    tgt_did.as_ptr(),
                                    identity_json.as_ptr(),
                                    cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn replace_keys_apply(wallet_handle: IndyHandle, tgt_did: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let tgt_did = c_str!(tgt_did);

        let err = unsafe {
            did::indy_replace_keys_apply(command_handle,
                                    wallet_handle,
                                    tgt_did.as_ptr(),
                                    cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn store_their_did(wallet_handle: IndyHandle, identity_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let identity_json = c_str!(identity_json);

        let err = unsafe {
            did::indy_store_their_did(command_handle,
                                    wallet_handle,
                                    identity_json.as_ptr(),
                                    cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn get_ver_key(pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let did = c_str!(did);

        let err = unsafe {
            did::indy_key_for_did(command_handle,
                                  pool_handle,
                                  wallet_handle,
                                  did.as_ptr(),
                                  cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn get_ver_key_local(wallet_handle: IndyHandle, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let did = c_str!(did);

        let err = unsafe {
            did::indy_key_for_local_did(command_handle,
                                  wallet_handle,
                                  did.as_ptr(),
                                  cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn set_endpoint(wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let did = c_str!(did);
        let address = c_str!(address);
        let transport_key = c_str!(transport_key);

        let err = unsafe {
            did::indy_set_endpoint_for_did(command_handle,
                                           wallet_handle,
                                           did.as_ptr(),
                                           address.as_ptr(),
                                           transport_key.as_ptr(),
                                           cb)
        };
        ResultHandler::empty(err, receiver)
    }

    pub fn get_endpoint(wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let did = c_str!(did);
        let err = unsafe {
            did::indy_get_endpoint_for_did(command_handle,
                                           wallet_handle,
                                           pool_handle,
                                           did.as_ptr(),
                                           cb)
        };
        ResultHandler::two(err, receiver)
    }

    pub fn set_metadata(wallet_handle: IndyHandle, tgt_did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let tgt_did = c_str!(tgt_did);
        let metadata = c_str!(metadata);

        let err = unsafe {
            did::indy_set_did_metadata(command_handle,
                                  wallet_handle,
                                  tgt_did.as_ptr(),
                                  metadata.as_ptr(),
                                  cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn get_metadata(wallet_handle: IndyHandle, tgt_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let tgt_did = c_str!(tgt_did);

        let err = unsafe {
            did::indy_get_did_metadata(command_handle,
                                      wallet_handle,
                                      tgt_did.as_ptr(),
                                      cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn get_my_metadata(wallet_handle: IndyHandle, my_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let my_did = c_str!(my_did);

        let err = unsafe {
            did::indy_get_my_did_with_meta(command_handle,
                                      wallet_handle,
                                      my_did.as_ptr(),
                                      cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn list_with_metadata(wallet_handle: IndyHandle) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = unsafe { did::indy_list_my_dids_with_meta(command_handle, wallet_handle, cb) };

        ResultHandler::one(err, receiver)
    }

    pub fn abbreviate_verkey(tgt_did: &str, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let tgt_did = c_str!(tgt_did);
        let verkey = c_str!(verkey);

        let err = unsafe {
            did::indy_abbreviate_verkey(command_handle,
                                   tgt_did.as_ptr(),
                                   verkey.as_ptr(),
                                        cb)
        };

        ResultHandler::one(err, receiver)
    }
}
