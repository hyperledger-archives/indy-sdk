extern crate libc;

use std::ffi::CString;

use indy::api::pairwise::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use std::ptr::null;

pub struct PairwiseUtils {}

impl PairwiseUtils {
    pub fn pairwise_exists(wallet_handle: i32, their_did: &str) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_bool();

        let their_did = CString::new(their_did).unwrap();

        let err = indy_is_pairwise_exists(command_handle, wallet_handle, their_did.as_ptr(), cb);

        super::results::result_to_bool(err, receiver)
    }

    pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let their_did = CString::new(their_did).unwrap();
        let my_did = CString::new(my_did).unwrap();
        let metadata_str = metadata.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_create_pairwise(command_handle,
                                 wallet_handle,
                                 their_did.as_ptr(),
                                 my_did.as_ptr(),
                                 if metadata.is_some() { metadata_str.as_ptr() } else { null() },
                                 cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn list_pairwise(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let err = indy_list_pairwise(command_handle, wallet_handle, cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let their_did = CString::new(their_did).unwrap();

        let err = indy_get_pairwise(command_handle, wallet_handle, their_did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let their_did = CString::new(their_did).unwrap();
        let metadata_str = metadata.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_set_pairwise_metadata(command_handle,
                                       wallet_handle,
                                       their_did.as_ptr(),
                                       if metadata.is_some() { metadata_str.as_ptr() } else { null() },
                                       cb);

        super::results::result_to_empty(err, receiver)
    }
}