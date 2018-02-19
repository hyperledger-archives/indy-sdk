extern crate libc;

use self::libc::c_char;
use utils::libindy::callback::{closure_cb_i32, closure_cb_i32_i32, closure_cb_i32_str,
                               closure_cb_i32_str_str, closure_cb_i32_bool, closure_cb_i32_bin,
                               closure_cb_i32_str_bin, closure_cb_i32_bin_bin};

pub struct CallbackUtils {}

impl CallbackUtils {

    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_open_pool_ledger_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pool_handle: i32)>) {
        closure_cb_i32_i32(closure)
    }

    pub fn closure_to_refresh_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_close_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_delete_pool_ledger_config_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_send_tx_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_claim_definition_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_def_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_register_wallet_type_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_create_wallet_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_open_wallet_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, handle: i32)>) {
        closure_cb_i32_i32(closure)
    }

    pub fn closure_to_prover_create_master_secret_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_prover_create_claim_req_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_req_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_claim_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  revoc_reg_update_json: *const c_char,
                                  xclaim_json: *const c_char)>) {
        closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_prover_store_claim_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_prover_get_claims_for_proof_req_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32, claims_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_prover_get_claims(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32, claims_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_prover_create_proof_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, proof_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_verifier_verify_proof_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        closure_cb_i32_bool(closure)
    }

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  did: *const c_char,
                                  verkey: *const c_char)>) {
        closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_store_their_did_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_sign_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signature_raw: *const u8, signature_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_crypto_sign_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signature_raw: *const u8, signature_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_verify_signature_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        closure_cb_i32_bool(closure)
    }

    pub fn closure_to_crypto_verify_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        closure_cb_i32_bool(closure)
    }

    pub fn closure_to_claim_offer_json_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_prover_get_claim_offers_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_offers_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_agent_connect_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pool_handle: i32)>) {
        closure_cb_i32_i32(closure)
    }

    pub fn closure_to_agent_add_identity_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_agent_rm_identity_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_agent_send_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_agent_close_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_sign_and_submit_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_submit_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_build_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_delete_wallet_cb(closure: Box<FnMut(i32) + Send>) -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_replace_keys_start_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_replace_keys_apply_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_encrypt_cb(closure: Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  encrypted_msg_raw: *const u8,
                                  encrypted_msg_len: u32,
                                  nonce_raw: *const u8,
                                  nonce_len: u32)>) {
        closure_cb_i32_bin_bin(closure)
    }

    pub fn closure_to_decrypt_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_sign_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signed_request_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }


    pub fn closure_to_issuer_revoke_claim_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, revoc_reg_update_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_and_store_revoc_reg_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, revoc_reg_update_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_encrypt_sealed_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_decrypt_sealed_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_pairwise_exists_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        closure_cb_i32_bool(closure)
    }

    pub fn closure_to_pairwise_create_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_pairwise_list_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pairwise_list: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_get_pairwise_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pairwise_info_json: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_set_pairwise_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_create_key_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_store_key_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_get_key_metadata_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, metadata: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_prep_msg_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_prep_anonymous_msg_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        closure_cb_i32_bin(closure)
    }

    pub fn closure_to_parse_msg_cb(closure: Box<FnMut(i32, Option<String>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char, msg_raw: *const u8, msg_len: u32)>) {
        closure_cb_i32_str_bin(closure)
    }

    pub fn closure_to_key_for_did_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_key_for_local_did_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        closure_cb_i32_str(closure)
    }

    pub fn closure_to_set_endpoint_for_did_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_get_endpoint_for_did_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  endpoint: *const c_char,
                                  transport_vk: *const c_char)>) {
        closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_store_did_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        closure_cb_i32(closure)
    }

    pub fn closure_to_get_did_metadata_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, metadata: *const c_char)>) {
        closure_cb_i32_str(closure)

    }
}
