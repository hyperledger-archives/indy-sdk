extern crate futures;

use indy::did;
use indy::IndyError;
use self::futures::Future;

use utils::{ledger, pool};
use utils::types::ResponseType;


pub fn create_store_and_publish_my_did_from_trustee(wallet_handle: i32, pool_handle: i32) -> Result<(String, String), IndyError> {
    let (trustee_did, _) = create_and_store_my_did(wallet_handle, Some(::utils::constants::TRUSTEE_SEED))?;
    let (my_did, my_vk) = create_and_store_my_did(wallet_handle, None)?;
    let nym = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("TRUSTEE"))?;
    let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym)?;
    pool::check_response_type(&response, ResponseType::REPLY);
    Ok((my_did, my_vk))
}

pub fn create_store_and_publish_my_did_from_steward(wallet_handle: i32, pool_handle: i32) -> Result<(String, String), IndyError> {
    let (trustee_did, _) = create_and_store_my_did(wallet_handle, Some(::utils::constants::TRUSTEE_SEED))?;
    let (my_did, my_vk) = create_and_store_my_did(wallet_handle, None)?;
    let nym = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, Some("STEWARD"))?;
    let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym)?;
    pool::check_response_type(&response, ResponseType::REPLY);
    Ok((my_did, my_vk))
}

pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), IndyError> {
    let my_did_json = json!({"seed": seed}).to_string();
    did::create_and_store_my_did(wallet_handle, &my_did_json).wait()
}

pub fn create_my_did(wallet_handle: i32, my_did_json: &str) -> Result<(String, String), IndyError> {
    did::create_and_store_my_did(wallet_handle, my_did_json).wait()
}

pub fn store_their_did(wallet_handle: i32, identity_json: &str) -> Result<(), IndyError> {
    did::store_their_did(wallet_handle, identity_json).wait()
}

pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_verkey: &str) -> Result<(), IndyError> {
    let their_identity_json = json!({"did": their_did, "verkey": their_verkey}).to_string();
    did::store_their_did(wallet_handle, &their_identity_json).wait()
}

pub fn replace_keys_start(wallet_handle: i32, did: &str, identity_json: &str) -> Result<String, IndyError> {
    did::replace_keys_start(wallet_handle, did, identity_json).wait()
}

pub fn replace_keys_apply(wallet_handle: i32, did: &str) -> Result<(), IndyError> {
    did::replace_keys_apply(wallet_handle, did).wait()
}

pub fn replace_keys(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, IndyError> {
    let verkey = did::replace_keys_start(wallet_handle, did, "{}").wait().unwrap();

    let nym_request = ledger::build_nym_request(did, did, Some(&verkey), None, None).unwrap();
    ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &nym_request).unwrap();

    replace_keys_apply(wallet_handle, did).unwrap();

    Ok(verkey)
}

pub fn key_for_did(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, IndyError> {
    did::key_for_did(pool_handle, wallet_handle, did).wait()
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Result<String, IndyError> {
    did::key_for_local_did(wallet_handle, did).wait()
}

pub fn set_endpoint_for_did(wallet_handle: i32, did: &str, address: &str, transport_key: &str) -> Result<(), IndyError> {
    did::set_endpoint_for_did(wallet_handle, did, address, transport_key).wait()
}

pub fn get_endpoint_for_did(wallet_handle: i32, pool_handle: i32, did: &str) -> Result<(String, Option<String>), IndyError> {
    did::get_endpoint_for_did(wallet_handle, pool_handle, did).wait()
}

pub fn set_did_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), IndyError> {
    did::set_did_metadata(wallet_handle, did, metadata).wait()
}

pub fn get_did_metadata(wallet_handle: i32, did: &str) -> Result<String, IndyError> {
    did::get_did_metadata(wallet_handle, did).wait()
}

pub fn get_my_did_with_metadata(wallet_handle: i32, did: &str) -> Result<String, IndyError> {
    did::get_my_did_with_metadata(wallet_handle, did).wait()
}

pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, IndyError> {
    did::abbreviate_verkey(did, verkey).wait()
}