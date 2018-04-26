extern crate libc;

use std::ffi::CString;

use indy::api::did::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::ledger::LedgerUtils;

pub struct DidUtils {}

impl DidUtils {
    pub fn create_store_and_publish_my_did_from_trustee(wallet_handle: i32, pool_handle: i32) -> Result<(String, String), ErrorCode> {
        let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(::utils::constants::TRUSTEE_SEED))?;
        let (my_did, my_vk) = DidUtils::create_and_store_my_did(wallet_handle, None)?;
        let nym = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("TRUSTEE"))?;
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym)?; //TODO check response type
        Ok((my_did, my_vk))
    }

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
        let my_did_json = CString::new(my_did_json).unwrap();

        let err = indy_create_and_store_my_did(command_handle, wallet_handle, my_did_json.as_ptr(), cb);

        super::results::result_to_string_string(err, receiver)
    }

    pub fn create_my_did(wallet_handle: i32, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let my_did_json = CString::new(my_did_json).unwrap();

        let err = indy_create_and_store_my_did(command_handle, wallet_handle, my_did_json.as_ptr(), cb);

        super::results::result_to_string_string(err, receiver)
    }

    pub fn store_their_did(wallet_handle: i32, identity_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let identity_json = CString::new(identity_json).unwrap();

        let err = indy_store_their_did(command_handle, wallet_handle, identity_json.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_verkey: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let their_identity_json = format!("{{\"did\":\"{}\",\"verkey\":\"{}\"}}", their_did, their_verkey);
        let their_identity_json = CString::new(their_identity_json).unwrap();

        let err = indy_store_their_did(command_handle, wallet_handle, their_identity_json.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn replace_keys_start(wallet_handle: i32, did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let did = CString::new(did).unwrap();
        let identity_json = CString::new(identity_json).unwrap();

        let err = indy_replace_keys_start(command_handle, wallet_handle, did.as_ptr(), identity_json.as_ptr(),
                                          cb);
        super::results::result_to_string(err, receiver)
    }

    pub fn replace_keys_apply(wallet_handle: i32, did: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let did = CString::new(did).unwrap();

        let err = indy_replace_keys_apply(command_handle, wallet_handle, did.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn replace_keys(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let verkey = DidUtils::replace_keys_start(wallet_handle, did, "{}").unwrap();

        let nym_request = LedgerUtils::build_nym_request(did, did, Some(&verkey), None, None).unwrap();
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, did, &nym_request).unwrap();

        DidUtils::replace_keys_apply(wallet_handle, did).unwrap();

        Ok(verkey)
    }

    pub fn key_for_did(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let did = CString::new(did).unwrap();

        let err = indy_key_for_did(command_handle, pool_handle, wallet_handle, did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let did = CString::new(did).unwrap();

        let err = indy_key_for_local_did(command_handle, wallet_handle, did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn set_endpoint_for_did(wallet_handle: i32, did: &str, address: &str, transport_key: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let did = CString::new(did).unwrap();
        let address = CString::new(address).unwrap();
        let transport_key = CString::new(transport_key).unwrap();

        let err = indy_set_endpoint_for_did(command_handle,
                                            wallet_handle,
                                            did.as_ptr(),
                                            address.as_ptr(),
                                            transport_key.as_ptr(),
                                            cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn get_endpoint_for_did(wallet_handle: i32, pool_handle: i32, did: &str) -> Result<(String, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_opt_string();

        let did = CString::new(did).unwrap();

        let err = indy_get_endpoint_for_did(command_handle, wallet_handle, pool_handle, did.as_ptr(), cb);

        super::results::result_to_string_opt_string(err, receiver)
    }

    pub fn set_did_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let did = CString::new(did).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = indy_set_did_metadata(command_handle, wallet_handle, did.as_ptr(), metadata.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn get_did_metadata(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let did = CString::new(did).unwrap();

        let err = indy_get_did_metadata(command_handle, wallet_handle, did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let did = CString::new(did).unwrap();
        let verkey = CString::new(verkey).unwrap();

        let err = indy_abbreviate_verkey(command_handle, did.as_ptr(), verkey.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }
}