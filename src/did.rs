use super::{ErrorCode, IndyHandle};

use std::ffi::CString;
use utils;
use ffi::did;

pub struct Did {}

impl Did {
    pub fn new(wallet_handle: IndyHandle, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string_string();

        let my_did_json = CString::new(my_did_json).unwrap();

        let err = unsafe {
            did::indy_create_and_store_my_did(command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         cb)
        };

        utils::results::result_to_two(err, receiver)
    }

    pub fn replace_keys_start(wallet_handle: i32, tgt_did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let tgt_did = CString::new(tgt_did).unwrap();
        let identity_json = CString::new(identity_json).unwrap();

        let err = unsafe {
            did::indy_replace_keys_start(command_handle,
                                    wallet_handle,
                                    tgt_did.as_ptr(),
                                    identity_json.as_ptr(),
                                    cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn replace_keys_apply(wallet_handle: i32, tgt_did: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let tgt_did = CString::new(tgt_did).unwrap();

        let err = unsafe {
            did::indy_replace_keys_apply(command_handle,
                                    wallet_handle,
                                    tgt_did.as_ptr(),
                                    cb)
        };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn set_metadata(wallet_handle: i32, tgt_did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, callback) = utils::callbacks::_closure_to_cb_ec();

        let tgt_did = CString::new(tgt_did).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = unsafe {
            did::indy_set_did_metadata(command_handle,
                                  wallet_handle,
                                  tgt_did.as_ptr(),
                                  metadata.as_ptr(),
                                  callback)
        };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn get_did_with_meta(wallet_handle: i32, tgt_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let tgt_did = CString::new(tgt_did).unwrap();

        let err = unsafe {
            did::indy_get_my_did_with_meta(command_handle,
                                      wallet_handle,
                                      tgt_did.as_ptr(),
                                      cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn list_dids_with_meta(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let err = unsafe { did::indy_list_my_dids_with_meta(command_handle, wallet_handle, cb) };

        utils::results::result_to_one(err, receiver)
    }

    pub fn abbreviate_verkey(tgt_did: &str, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let tgt_did = CString::new(tgt_did).unwrap();
        let verkey = CString::new(verkey).unwrap();

        let err = unsafe {
            did::indy_abbreviate_verkey(command_handle,
                                   tgt_did.as_ptr(),
                                   verkey.as_ptr(),
                                   cb)
        };

        utils::results::result_to_one(err, receiver)
    }
}
