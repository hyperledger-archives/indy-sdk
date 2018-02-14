extern crate libc;

use indy::api::ErrorCode;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::slice;


lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

lazy_static! {
    static ref CLOSURE_CB_MAP: Mutex<HashMap<i32, i32>> = Default::default();
}

pub struct CallbackUtils {}

impl CallbackUtils {
    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref CREATE_POOL_LEDGER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn create_pool_ledger_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CREATE_POOL_LEDGER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_pool_ledger_callback))
    }

    pub fn closure_to_open_pool_ledger_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                          -> (i32,
                                              Option<extern fn(command_handle: i32, err: ErrorCode,
                                                               pool_handle: i32)>) {
        lazy_static! {
            static ref OPEN_POOL_LEDGER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        extern "C" fn open_pool_ledger_callback(command_handle: i32, err: ErrorCode, pool_handle: i32) {
            let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, pool_handle)
        }

        let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(open_pool_ledger_callback))
    }

    pub fn closure_to_refresh_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                        Option<extern fn(command_handle: i32,
                                                                                                         err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_close_pool_ledger_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                      Option<extern fn(command_handle: i32,
                                                                                                       err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_delete_pool_ledger_config_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                              Option<extern fn(command_handle: i32,
                                                                                                               err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_send_tx_cb(closure: Box<FnMut(ErrorCode, String) + Send>)
                                 -> (i32,
                                     Option<extern fn(command_handle: i32, err: ErrorCode,
                                                      request_result_json: *const c_char)>) {
        lazy_static! {
            static ref OPEN_POOL_LEDGER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        extern "C" fn send_tx_callback(command_handle: i32, err: ErrorCode, request_result_json: *const c_char) {
            let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str: &CStr =
                unsafe {
                    CStr::from_ptr(request_result_json)
                };
            cb(err, str.to_str().unwrap().to_string());
        }

        let mut callbacks = OPEN_POOL_LEDGER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(send_tx_callback))
    }

    pub fn closure_to_issuer_create_claim_definition_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                           Option<extern fn(command_handle: i32,
                                                                                                                            err: ErrorCode,
                                                                                                                            claim_def_json: *const c_char)>) {
        lazy_static! {
            static ref CREATE_CLAIM_DEFINITION_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn create_claim_definition_callback(command_handle: i32, err: ErrorCode, claim_def_json: *const c_char) {
            let mut callbacks = CREATE_CLAIM_DEFINITION_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claim_def_json = unsafe { CStr::from_ptr(claim_def_json).to_str().unwrap().to_string() };
            cb(err, claim_def_json)
        }

        let mut callbacks = CREATE_CLAIM_DEFINITION_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_claim_definition_callback))
    }

    pub fn closure_to_register_wallet_type_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                         Option<extern fn(command_handle: i32,
                                                                                                          err: ErrorCode)>) {
        lazy_static! {
            static ref REFISTER_WALLET_TYPE_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn register_wallet_type_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = REFISTER_WALLET_TYPE_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = REFISTER_WALLET_TYPE_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(register_wallet_type_callback))
    }

    pub fn closure_to_issuer_create_claim_offer_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                      Option<extern fn(command_handle: i32,
                                                                                                                       err: ErrorCode,
                                                                                                                       claim_offer_json: *const c_char)>) {
        lazy_static! {
            static ref CREATE_CLAIM_OFFER_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn create_and_store_claim_offer_callback(command_handle: i32, err: ErrorCode, claim_offer_json: *const c_char) {
            let mut callbacks = CREATE_CLAIM_OFFER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claim_offer_json = unsafe { CStr::from_ptr(claim_offer_json).to_str().unwrap().to_string() };
            cb(err, claim_offer_json)
        }

        let mut callbacks = CREATE_CLAIM_OFFER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_and_store_claim_offer_callback))
    }

    pub fn closure_to_create_wallet_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                  Option<extern fn(command_handle: i32,
                                                                                                   err: ErrorCode)>) {
        lazy_static! {
            static ref CREATE_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn create_wallet_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CREATE_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CREATE_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_wallet_callback))
    }

    pub fn closure_to_open_wallet_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                     -> (i32,
                                         Option<extern fn(command_handle: i32, err: ErrorCode,
                                                          handle: i32)>) {
        lazy_static! {
            static ref OPEN_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        extern "C" fn open_wallet_callback(command_handle: i32, err: ErrorCode, handle: i32) {
            let mut callbacks = OPEN_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, handle)
        }

        let mut callbacks = OPEN_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(open_wallet_callback))
    }

    pub fn closure_to_prover_create_master_secret_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode)>) {
        lazy_static! {
            static ref PROVER_CREATE_MASTER_SECRET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn closure_to_prover_create_master_secret_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = PROVER_CREATE_MASTER_SECRET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = PROVER_CREATE_MASTER_SECRET_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_prover_create_master_secret_callback))
    }

    pub fn closure_to_prover_create_claim_req_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                    Option<extern fn(command_handle: i32,
                                                                                                                     err: ErrorCode,
                                                                                                                     claim_req_json: *const c_char)>) {
        lazy_static! {
            static ref PROVER_CREATE_CLAIM_REQ_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn prover_create_claim_req_callback(command_handle: i32, err: ErrorCode, claim_req_json: *const c_char) {
            let mut callbacks = PROVER_CREATE_CLAIM_REQ_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claim_req_json = unsafe { CStr::from_ptr(claim_req_json).to_str().unwrap().to_string() };
            cb(err, claim_req_json)
        }

        let mut callbacks = PROVER_CREATE_CLAIM_REQ_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_create_claim_req_callback))
    }

    pub fn closure_to_issuer_create_claim_cb(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                        Option<extern fn(command_handle: i32,
                                                                                                                         err: ErrorCode,
                                                                                                                         revoc_reg_update_json: *const c_char,
                                                                                                                         xclaim_json: *const c_char)>) {
        lazy_static! {
            static ref CREATE_CLAIM_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
        }

        extern "C" fn create_claim_callback(command_handle: i32, err: ErrorCode, revoc_reg_update_json: *const c_char, xclaim_json: *const c_char) {
            let mut callbacks = CREATE_CLAIM_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let revoc_reg_update_json = unsafe { CStr::from_ptr(revoc_reg_update_json).to_str().unwrap().to_string() };
            let xclaim_json = unsafe { CStr::from_ptr(xclaim_json).to_str().unwrap().to_string() };
            cb(err, revoc_reg_update_json, xclaim_json)
        }

        let mut callbacks = CREATE_CLAIM_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_claim_callback))
    }

    pub fn closure_to_prover_store_claim_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref PROVER_STORE_CLAIM_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn prover_store_claim_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = PROVER_STORE_CLAIM_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = PROVER_STORE_CLAIM_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_store_claim_callback))
    }

    pub fn closure_to_prover_get_claims_for_proof_req_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                            Option<extern fn(command_handle: i32,
                                                                                                                             err: ErrorCode,
                                                                                                                             claims_json: *const c_char)>) {
        lazy_static! {
            static ref PROVER_GET_CLAIMS_FOR_PROOF_REQ_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn prover_get_claims_for_proof_req_callback(command_handle: i32, err: ErrorCode, claims_json: *const c_char) {
            let mut callbacks = PROVER_GET_CLAIMS_FOR_PROOF_REQ_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claims_json = unsafe { CStr::from_ptr(claims_json).to_str().unwrap().to_string() };
            cb(err, claims_json)
        }

        let mut callbacks = PROVER_GET_CLAIMS_FOR_PROOF_REQ_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_get_claims_for_proof_req_callback))
    }

    pub fn closure_to_prover_get_claims(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                           Option<extern fn(command_handle: i32,
                                                                                                            err: ErrorCode,
                                                                                                            claims_json: *const c_char)>) {
        lazy_static! {
            static ref PROVER_GET_CLAIMS_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn prover_get_claims_callback(command_handle: i32, err: ErrorCode, claims_json: *const c_char) {
            let mut callbacks = PROVER_GET_CLAIMS_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claims_json = unsafe { CStr::from_ptr(claims_json).to_str().unwrap().to_string() };
            cb(err, claims_json)
        }

        let mut callbacks = PROVER_GET_CLAIMS_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_get_claims_callback))
    }

    pub fn closure_to_prover_create_proof_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode,
                                                                                                                 proof_json: *const c_char)>) {
        lazy_static! {
            static ref PROVER_CREATE_PROOF_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn prover_create_proof_callback(command_handle: i32, err: ErrorCode, proof_json: *const c_char) {
            let mut callbacks = PROVER_CREATE_PROOF_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let proof_json = unsafe { CStr::from_ptr(proof_json).to_str().unwrap().to_string() };
            cb(err, proof_json)
        }

        let mut callbacks = PROVER_CREATE_PROOF_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_create_proof_callback))
    }

    pub fn closure_to_verifier_verify_proof_cb(closure: Box<FnMut(ErrorCode, bool) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode,
                                                                                                                 valid: bool)>) {
        lazy_static! {
            static ref VERIFIER_VERIFY_PROOF_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, bool) + Send > >> = Default::default();
        }

        extern "C" fn verifier_verify_proof_callback(command_handle: i32, err: ErrorCode, valid: bool) {
            let mut callbacks = VERIFIER_VERIFY_PROOF_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, valid)
        }

        let mut callbacks = VERIFIER_VERIFY_PROOF_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(verifier_verify_proof_callback))
    }

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                            Option<extern fn(command_handle: i32,
                                                                                                                             err: ErrorCode,
                                                                                                                             did: *const c_char,
                                                                                                                             verkey: *const c_char)>) {
        lazy_static! {
            static ref CREATE_AND_STORE_MY_DID_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
        }

        extern "C" fn create_and_store_my_did_callback(command_handle: i32, err: ErrorCode, did: *const c_char, verkey: *const c_char) {
            let mut callbacks = CREATE_AND_STORE_MY_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let did = unsafe { CStr::from_ptr(did).to_str().unwrap().to_string() };
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, did, verkey)
        }

        let mut callbacks = CREATE_AND_STORE_MY_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_and_store_my_did_callback))
    }

    pub fn closure_to_store_their_did_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                    Option<extern fn(command_handle: i32,
                                                                                                     err: ErrorCode)>) {
        lazy_static! {
            static ref STORE_THEIR_DID_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn store_their_did_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = STORE_THEIR_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = STORE_THEIR_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(store_their_did_callback))
    }

    pub fn closure_to_sign_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>)
                              -> (i32,
                                  Option<extern fn(command_handle: i32, err: ErrorCode,
                                                   signature_raw: *const u8, signature_len: u32)>) {
        lazy_static! {
            static ref SIGN_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, Vec<u8>) + Send>>> = Default::default();
        }

        extern "C" fn sign_callback(command_handle: i32, err: ErrorCode, signature_raw: *const u8, signature_len: u32) {
            let mut callbacks = SIGN_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let signature = unsafe { slice::from_raw_parts(signature_raw, signature_len as usize) };
            cb(err, signature.to_vec());
        }

        let mut callbacks = SIGN_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(sign_callback))
    }

    pub fn closure_to_crypto_sign_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>)
                                     -> (i32,
                                         Option<extern fn(command_handle: i32, err: ErrorCode,
                                                          signature_raw: *const u8, signature_len: u32)>) {
        lazy_static! {
            static ref CRYPTO_SIGN_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, Vec<u8>) + Send>>> = Default::default();
        }

        extern "C" fn crypto_sign_callback(command_handle: i32, err: ErrorCode, signature_raw: *const u8, signature_len: u32) {
            let mut callbacks = CRYPTO_SIGN_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let signature = unsafe { slice::from_raw_parts(signature_raw, signature_len as usize) };
            cb(err, signature.to_vec());
        }

        let mut callbacks = CRYPTO_SIGN_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(crypto_sign_callback))
    }

    pub fn closure_to_verify_signature_cb(closure: Box<FnMut(ErrorCode, bool) + Send>) -> (i32,
                                                                                           Option<extern fn(command_handle: i32,
                                                                                                            err: ErrorCode,
                                                                                                            valid: bool)>) {
        lazy_static! {
            static ref VERIFY_SIGNATURE_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, bool) + Send > >> = Default::default();
        }

        extern "C" fn closure_to_verify_signature_callback(command_handle: i32, err: ErrorCode, valid: bool) {
            let mut callbacks = VERIFY_SIGNATURE_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, valid)
        }

        let mut callbacks = VERIFY_SIGNATURE_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_verify_signature_callback))
    }

    pub fn closure_to_crypto_verify_cb(closure: Box<FnMut(ErrorCode, bool) + Send>) -> (i32,
                                                                                        Option<extern fn(command_handle: i32,
                                                                                                         err: ErrorCode,
                                                                                                         valid: bool)>) {
        lazy_static! {
            static ref CRYPTO_VERIFY_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, bool) + Send > >> = Default::default();
        }

        extern "C" fn closure_to_crypto_verify(command_handle: i32, err: ErrorCode, valid: bool) {
            let mut callbacks = CRYPTO_VERIFY_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, valid)
        }

        let mut callbacks = CRYPTO_VERIFY_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_crypto_verify))
    }

    pub fn closure_to_claim_offer_json_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                     Option<extern fn(command_handle: i32,
                                                                                                      err: ErrorCode)>) {
        lazy_static! {
            static ref PROVER_STORE_CLAIM_OFFER_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn closure_to_claim_offer_json_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = PROVER_STORE_CLAIM_OFFER_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = PROVER_STORE_CLAIM_OFFER_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_claim_offer_json_callback))
    }

    pub fn closure_to_prover_get_claim_offers_cb(closure: Box<FnMut(ErrorCode, String) + Send>)
                                                 -> (i32,
                                                     Option<extern fn(command_handle: i32, err: ErrorCode,
                                                                      claim_offers_json: *const c_char)>) {
        lazy_static! {
            static ref GET_CLAIM_OFFERS_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        extern "C" fn prover_get_claim_offers_callback(command_handle: i32, err: ErrorCode, claim_offers_json: *const c_char) {
            let mut callbacks = GET_CLAIM_OFFERS_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claim_offers_json = unsafe { CStr::from_ptr(claim_offers_json).to_str().unwrap().to_string() };
            cb(err, claim_offers_json);
        }

        let mut callbacks = GET_CLAIM_OFFERS_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prover_get_claim_offers_callback))
    }

    pub fn closure_map_ids(cb_id: i32, param_id: i32) {
        let mut map = CLOSURE_CB_MAP.lock().unwrap();
        map.insert(param_id, cb_id);
    }

    pub fn closure_to_agent_listen_cb(closure: Box<FnMut(ErrorCode, i32) + Send>)
                                      -> (i32,
                                          Option<extern fn(command_handle: i32, err: ErrorCode,
                                                           pool_handle: i32)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        extern "C" fn agent_listen_callback(command_handle: i32, err: ErrorCode, pool_handle: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, pool_handle)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(agent_listen_callback))
    }

    pub fn closure_to_agent_connected_cb(closure: Box<FnMut(i32, ErrorCode, i32, String, String) + Send>)
                                         -> (i32, Option<extern fn(listener_handle: i32,
                                                                   err: ErrorCode,
                                                                   conn_handle: i32,
                                                                   sender_did: *const c_char,
                                                                   receiver_did: *const c_char)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(i32, ErrorCode, i32, String, String) + Send>>> = Default::default();
        }

        extern "C" fn callback(listener_handle: i32,
                               err: ErrorCode,
                               conn_handle: i32,
                               sender_did: *const c_char,
                               receiver_did: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let sender_did = unsafe { CStr::from_ptr(sender_did).to_str().unwrap().to_string() };
            let receiver_did = unsafe { CStr::from_ptr(receiver_did).to_str().unwrap().to_string() };
            let cb_id: i32 = *CLOSURE_CB_MAP.lock().unwrap().get(&listener_handle).unwrap();
            callbacks.get_mut(&cb_id).unwrap()(listener_handle, err, conn_handle, sender_did.clone(), receiver_did.clone());
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_agent_add_identity_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_agent_rm_identity_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                      Option<extern fn(command_handle: i32,
                                                                                                       err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_agent_send_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                               Option<extern fn(command_handle: i32,
                                                                                                err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }

    pub fn closure_to_agent_close_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                Option<extern fn(command_handle: i32,
                                                                                                 err: ErrorCode)>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn agent_close_connection_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(agent_close_connection_callback))
    }


    pub fn closure_to_sign_and_submit_request_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                    Option<extern fn(command_handle: i32,
                                                                                                                     err: ErrorCode,
                                                                                                                     request_result_json: *const c_char)>) {
        lazy_static! {
            static ref SIGN_AND_SUBMIT_REQUEST_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn closure_to_sign_and_submit_request_callback(command_handle: i32, err: ErrorCode, request_result_json: *const c_char) {
            let mut callbacks = SIGN_AND_SUBMIT_REQUEST_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let request_result_json = unsafe { CStr::from_ptr(request_result_json).to_str().unwrap().to_string() };
            cb(err, request_result_json)
        }

        let mut callbacks = SIGN_AND_SUBMIT_REQUEST_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_sign_and_submit_request_callback))
    }

    pub fn closure_to_submit_request_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                           Option<extern fn(command_handle: i32,
                                                                                                            err: ErrorCode,
                                                                                                            request_result_json: *const c_char)>) {
        lazy_static! {
            static ref SUBMIT_REQUEST_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn closure_to_submit_request_callback(command_handle: i32, err: ErrorCode, request_result_json: *const c_char) {
            let mut callbacks = SUBMIT_REQUEST_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let request_result_json = unsafe { CStr::from_ptr(request_result_json).to_str().unwrap().to_string() };
            cb(err, request_result_json)
        }

        let mut callbacks = SUBMIT_REQUEST_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_submit_request_callback))
    }

    pub fn closure_to_build_request_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                          Option<extern fn(command_handle: i32,
                                                                                                           err: ErrorCode,
                                                                                                           request_json: *const c_char)>) {
        lazy_static! {
            static ref BUILD_REQUEST_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn closure_to_build_request_callback(command_handle: i32, err: ErrorCode, request_json: *const c_char) {
            let mut callbacks = BUILD_REQUEST_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let request_json = unsafe { CStr::from_ptr(request_json).to_str().unwrap().to_string() };
            cb(err, request_json)
        }

        let mut callbacks = BUILD_REQUEST_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_build_request_callback))
    }

    pub fn closure_to_delete_wallet_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                  Option<extern fn(command_handle: i32,
                                                                                                   err: ErrorCode)>) {
        lazy_static! {
            static ref DELETE_WALLET_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn delete_wallet_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = DELETE_WALLET_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = DELETE_WALLET_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(delete_wallet_callback))
    }

    pub fn closure_to_replace_keys_start_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                               Option<extern fn(command_handle: i32,
                                                                                                                err: ErrorCode,
                                                                                                                verkey: *const c_char)>) {
        lazy_static! {
            static ref REPLACE_KEYS_START_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn replace_keys_start_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char) {
            let mut callbacks = REPLACE_KEYS_START_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, verkey)
        }

        let mut callbacks = REPLACE_KEYS_START_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(replace_keys_start_callback))
    }

    pub fn closure_to_replace_keys_apply_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref REPLACE_KEYS_APPLY_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn replace_keys_apply_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = REPLACE_KEYS_APPLY_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = REPLACE_KEYS_APPLY_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(replace_keys_apply_callback))
    }

    pub fn closure_to_encrypt_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>) -> (i32,
                                                                                     Option<extern fn(command_handle: i32,
                                                                                                      err: ErrorCode,
                                                                                                      encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        lazy_static! {
            static ref ENCRYPT_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn encrypt_callback(command_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32) {
            let mut callbacks = ENCRYPT_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let encrypted_msg = unsafe { slice::from_raw_parts(encrypted_msg_raw, encrypted_msg_len as usize) };
            cb(err, encrypted_msg.to_vec());
        }

        let mut callbacks = ENCRYPT_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(encrypt_callback))
    }

    pub fn closure_to_decrypt_cb(closure: Box<FnMut(ErrorCode, String, Vec<u8>) + Send>) -> (i32,
                                                                                             Option<extern fn(command_handle: i32,
                                                                                                              err: ErrorCode,
                                                                                                              sender_vk: *const c_char,
                                                                                                              decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        lazy_static! {
            static ref DECRYPT_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn decrypt_callback(command_handle: i32, err: ErrorCode, sender_vk: *const c_char, decrypted_msg_raw: *const u8, decrypted_msg_len: u32) {
            let mut callbacks = DECRYPT_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let sender_vk = unsafe { CStr::from_ptr(sender_vk).to_str().unwrap().to_string() };
            let decrypted_msg = unsafe { slice::from_raw_parts(decrypted_msg_raw, decrypted_msg_len as usize) };
            cb(err, sender_vk, decrypted_msg.to_vec())
        }

        let mut callbacks = DECRYPT_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(decrypt_callback))
    }

    pub fn closure_to_sign_request_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                         Option<extern fn(command_handle: i32,
                                                                                                          err: ErrorCode,
                                                                                                          signed_request_json: *const c_char)>) {
        lazy_static! {
            static ref SIGN_REQUEST_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn sign_request_callback(command_handle: i32, err: ErrorCode, signed_request_json: *const c_char) {
            let mut callbacks = SIGN_REQUEST_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let signed_request_json = unsafe { CStr::from_ptr(signed_request_json).to_str().unwrap().to_string() };
            cb(err, signed_request_json)
        }

        let mut callbacks = SIGN_REQUEST_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(sign_request_callback))
    }


    pub fn closure_to_issuer_revoke_claim_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode,
                                                                                                                 revoc_reg_update_json: *const c_char)>) {
        lazy_static! {
            static ref ISSUER_REVOKE_CLAIM_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn issuer_revoke_claim_callback(command_handle: i32, err: ErrorCode, revoc_reg_update_json: *const c_char) {
            let mut callbacks = ISSUER_REVOKE_CLAIM_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let revoc_reg_update_json = unsafe { CStr::from_ptr(revoc_reg_update_json).to_str().unwrap().to_string() };
            cb(err, revoc_reg_update_json)
        }

        let mut callbacks = ISSUER_REVOKE_CLAIM_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(issuer_revoke_claim_callback))
    }

    pub fn closure_to_issuer_create_and_store_revoc_reg_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                                              Option<extern fn(command_handle: i32,
                                                                                                                               err: ErrorCode,
                                                                                                                               revoc_reg_update_json: *const c_char)>) {
        lazy_static! {
            static ref ISSUER_CREATE_REVOC_REG_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn issuer_create_and_store_revoc_reg_callback(command_handle: i32, err: ErrorCode, revoc_reg_json: *const c_char) {
            let mut callbacks = ISSUER_CREATE_REVOC_REG_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let revoc_reg_json = unsafe { CStr::from_ptr(revoc_reg_json).to_str().unwrap().to_string() };
            cb(err, revoc_reg_json)
        }

        let mut callbacks = ISSUER_CREATE_REVOC_REG_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(issuer_create_and_store_revoc_reg_callback))
    }

    pub fn closure_to_encrypt_sealed_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>) -> (i32,
                                                                                            Option<extern fn(command_handle: i32,
                                                                                                             err: ErrorCode,
                                                                                                             encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        lazy_static! {
            static ref ENCRYPT_SEALED_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn encrypt_sealed_callback(command_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32) {
            let mut callbacks = ENCRYPT_SEALED_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let encrypted_msg = unsafe { slice::from_raw_parts(encrypted_msg_raw, encrypted_msg_len as usize) };
            cb(err, encrypted_msg.to_vec())
        }

        let mut callbacks = ENCRYPT_SEALED_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(encrypt_sealed_callback))
    }

    pub fn closure_to_decrypt_sealed_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>) -> (i32,
                                                                                            Option<extern fn(command_handle: i32,
                                                                                                             err: ErrorCode,
                                                                                                             decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        lazy_static! {
            static ref DECRYPT_SEALED_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn encrypt_sealed_callback(command_handle: i32, err: ErrorCode, decrypted_msg_raw: *const u8, decrypted_msg_len: u32) {
            let mut callbacks = DECRYPT_SEALED_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let decrypted_msg = unsafe { slice::from_raw_parts(decrypted_msg_raw, decrypted_msg_len as usize) };
            cb(err, decrypted_msg.to_vec())
        }

        let mut callbacks = DECRYPT_SEALED_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(encrypt_sealed_callback))
    }

    pub fn closure_to_pairwise_exists_cb(closure: Box<FnMut(ErrorCode, bool) + Send>) -> (i32,
                                                                                          Option<extern fn(command_handle: i32,
                                                                                                           err: ErrorCode,
                                                                                                           valid: bool)>) {
        lazy_static! {
            static ref PAIRWISE_EXISTS_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, bool) + Send > >> = Default::default();
        }

        extern "C" fn pairwise_exists_callback(command_handle: i32, err: ErrorCode, exists: bool) {
            let mut callbacks = PAIRWISE_EXISTS_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, exists)
        }

        let mut callbacks = PAIRWISE_EXISTS_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(pairwise_exists_callback))
    }

    pub fn closure_to_pairwise_create_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                    Option<extern fn(command_handle: i32,
                                                                                                     err: ErrorCode)>) {
        lazy_static! {
            static ref PAIRWISE_CREATE_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn pairwise_create_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = PAIRWISE_CREATE_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = PAIRWISE_CREATE_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(pairwise_create_callback))
    }

    pub fn closure_to_pairwise_list_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                          Option<extern fn(command_handle: i32,
                                                                                                           err: ErrorCode,
                                                                                                           pairwise_list: *const c_char)>) {
        lazy_static! {
            static ref PAIRWISE_LIST_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn pairwise_list_callback(command_handle: i32, err: ErrorCode, pairwise_list: *const c_char) {
            let mut callbacks = PAIRWISE_LIST_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let pairwise_list = unsafe { CStr::from_ptr(pairwise_list).to_str().unwrap().to_string() };
            cb(err, pairwise_list)
        }

        let mut callbacks = PAIRWISE_LIST_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(pairwise_list_callback))
    }

    pub fn closure_to_get_pairwise_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                         Option<extern fn(command_handle: i32,
                                                                                                          err: ErrorCode,
                                                                                                          pairwise_info_json: *const c_char)>) {
        lazy_static! {
            static ref GET_PAIRWISE_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn get_pairwise_callback(command_handle: i32, err: ErrorCode, pairwise_info_json: *const c_char) {
            let mut callbacks = GET_PAIRWISE_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let pairwise_info_json = unsafe { CStr::from_ptr(pairwise_info_json).to_str().unwrap().to_string() };
            cb(err, pairwise_info_json)
        }

        let mut callbacks = GET_PAIRWISE_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(get_pairwise_callback))
    }

    pub fn closure_to_set_pairwise_metadata_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                          Option<extern fn(command_handle: i32,
                                                                                                           err: ErrorCode)>) {
        lazy_static! {
            static ref SET_PAIRWISE_METADATA_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn set_pairwise_metadata_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = SET_PAIRWISE_METADATA_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = SET_PAIRWISE_METADATA_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(set_pairwise_metadata_callback))
    }

    pub fn closure_to_create_key_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode,
                                                                                                        verkey: *const c_char)>) {
        lazy_static! {
            static ref CREATE_KEY_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn create_key_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char) {
            let mut callbacks = CREATE_KEY_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, verkey)
        }

        let mut callbacks = CREATE_KEY_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_key_callback))
    }

    pub fn closure_to_store_key_metadata_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref STORE_KEY_METADATA_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn store_key_metadata_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = STORE_KEY_METADATA_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = STORE_KEY_METADATA_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(store_key_metadata_callback))
    }

    pub fn closure_to_get_key_metadata_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                             Option<extern fn(command_handle: i32,
                                                                                                              err: ErrorCode,
                                                                                                              metadata: *const c_char)>) {
        lazy_static! {
            static ref GET_KEY_META_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn get_key_metadata_callback(command_handle: i32, err: ErrorCode, metadata: *const c_char) {
            let mut callbacks = GET_KEY_META_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = unsafe { CStr::from_ptr(metadata).to_str().unwrap().to_string() };
            cb(err, metadata)
        }

        let mut callbacks = GET_KEY_META_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(get_key_metadata_callback))
    }

    pub fn closure_to_prep_msg_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>) -> (i32,
                                                                                      Option<extern fn(command_handle: i32,
                                                                                                       err: ErrorCode,
                                                                                                       encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        lazy_static! {
            static ref PREP_MSG_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn prep_msg_callback(command_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32) {
            let mut callbacks = PREP_MSG_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let encrypted_msg = unsafe { slice::from_raw_parts(encrypted_msg_raw, encrypted_msg_len as usize) };
            cb(err, encrypted_msg.to_vec())
        }

        let mut callbacks = PREP_MSG_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prep_msg_callback))
    }

    pub fn closure_to_prep_anonymous_msg_cb(closure: Box<FnMut(ErrorCode, Vec<u8>) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode,
                                                                                                                 encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        lazy_static! {
            static ref PREP_ANONYMOUS_MSG_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn prep_anonymous_msg_callback(command_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32) {
            let mut callbacks = PREP_ANONYMOUS_MSG_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let encrypted_msg = unsafe { slice::from_raw_parts(encrypted_msg_raw, encrypted_msg_len as usize) };
            cb(err, encrypted_msg.to_vec())
        }

        let mut callbacks = PREP_ANONYMOUS_MSG_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(prep_anonymous_msg_callback))
    }

    pub fn closure_to_parse_msg_cb(closure: Box<FnMut(ErrorCode, Option<String>, Vec<u8>) + Send>) -> (i32,
                                                                                                       Option<extern fn(command_handle: i32,
                                                                                                                        err: ErrorCode,
                                                                                                                        verkey: *const c_char,
                                                                                                                        msg_raw: *const u8, msg_len: u32)>) {
        lazy_static! {
            static ref PARSE_MSG_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Option<String>, Vec<u8>) + Send > >> = Default::default();
        }

        extern "C" fn parse_msg_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char, msg_raw: *const u8, msg_len: u32) {
            let mut callbacks = PARSE_MSG_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let msg = unsafe { slice::from_raw_parts(msg_raw, msg_len as usize) };

            let verkey =
                if verkey.is_null() { None } else {
                    unsafe { Some(CStr::from_ptr(verkey).to_str().unwrap().to_string()) }
                };

            cb(err, verkey, msg.to_vec())
        }

        let mut callbacks = PARSE_MSG_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(parse_msg_callback))
    }

    pub fn closure_to_key_for_did_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                        Option<extern fn(command_handle: i32,
                                                                                                         err: ErrorCode,
                                                                                                         verkey: *const c_char)>) {
        lazy_static! {
            static ref KEY_FOR_DID_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn key_for_did_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char) {
            let mut callbacks = KEY_FOR_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, verkey)
        }

        let mut callbacks = KEY_FOR_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(key_for_did_callback))
    }

    pub fn closure_to_key_for_local_did_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                              Option<extern fn(command_handle: i32,
                                                                                                               err: ErrorCode,
                                                                                                               verkey: *const c_char)>) {
        lazy_static! {
            static ref KEY_FOR_LOCAL_DID_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn key_for_local_did_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char) {
            let mut callbacks = KEY_FOR_LOCAL_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, verkey)
        }

        let mut callbacks = KEY_FOR_LOCAL_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(key_for_local_did_callback))
    }

    pub fn closure_to_set_endpoint_for_did_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                         Option<extern fn(command_handle: i32,
                                                                                                          err: ErrorCode)>) {
        lazy_static! {
            static ref SET_ENDPOINT_FOR_DID_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn set_endpoint_for_did_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = SET_ENDPOINT_FOR_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = SET_ENDPOINT_FOR_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(set_endpoint_for_did_callback))
    }

    pub fn closure_to_get_endpoint_for_did_cb(closure: Box<FnMut(ErrorCode, String, Option<String>) + Send>) -> (i32,
                                                                                                                 Option<extern fn(command_handle: i32,
                                                                                                                                  err: ErrorCode,
                                                                                                                                  endpoint: *const c_char,
                                                                                                                                  transport_vk: *const c_char)>) {
        lazy_static! {
            static ref ENDPOINT_FOR_DID_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Option<String>) + Send > >> = Default::default();
        }

        extern "C" fn endpoint_for_did_callback(command_handle: i32, err: ErrorCode, endpoint: *const c_char, transport_vk: *const c_char) {
            let mut callbacks = ENDPOINT_FOR_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let endpoint = unsafe { CStr::from_ptr(endpoint).to_str().unwrap().to_string() };
            let transport_vk = if !transport_vk.is_null() {
                unsafe { Some(CStr::from_ptr(transport_vk).to_str().unwrap().to_string()) }
            } else { None };
            cb(err, endpoint, transport_vk)
        }

        let mut callbacks = ENDPOINT_FOR_DID_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(endpoint_for_did_callback))
    }

    pub fn closure_to_store_did_metadata_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                       Option<extern fn(command_handle: i32,
                                                                                                        err: ErrorCode)>) {
        lazy_static! {
            static ref STORE_KEY_METADATA_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn store_key_metadata_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = STORE_KEY_METADATA_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = STORE_KEY_METADATA_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(store_key_metadata_callback))
    }

    pub fn closure_to_get_did_metadata_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                             Option<extern fn(command_handle: i32,
                                                                                                              err: ErrorCode,
                                                                                                              metadata: *const c_char)>) {
        lazy_static! {
            static ref GET_KEY_META_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn get_key_metadata_callback(command_handle: i32, err: ErrorCode, metadata: *const c_char) {
            let mut callbacks = GET_KEY_META_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = unsafe { CStr::from_ptr(metadata).to_str().unwrap().to_string() };
            cb(err, metadata)
        }

        let mut callbacks = GET_KEY_META_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(get_key_metadata_callback))
    }

    pub fn closure_to_get_abbr_verkey_cb(closure: Box<FnMut(ErrorCode, String) + Send>) -> (i32,
                                                                                            Option<extern fn(command_handle: i32,
                                                                                                             err: ErrorCode,
                                                                                                             verkey: *const c_char)>) {
        lazy_static! {
            static ref GET_ABBR_VERKEY_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        extern "C" fn get_abbr_verkey_callback(command_handle: i32, err: ErrorCode, verkey: *const c_char) {
            let mut callbacks = GET_ABBR_VERKEY_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            cb(err, verkey)
        }

        let mut callbacks = GET_ABBR_VERKEY_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(get_abbr_verkey_callback))
    }
}
