extern crate libc;

use sovrin::api::ErrorCode;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;


lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
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

    pub fn closure_to_issuer_create_claim_definition_cb(closure: Box<FnMut(ErrorCode, String, String) + Send>) -> (i32,
                                                                                                                   Option<extern fn(command_handle: i32,
                                                                                                                                    err: ErrorCode,
                                                                                                                                    claim_def_json: *const c_char,
                                                                                                                                    claim_def_uuid: *const c_char)>) {
        lazy_static! {
            static ref CREATE_CLAIM_DEFINITION_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
        }

        extern "C" fn create_claim_definition_callback(command_handle: i32, err: ErrorCode, claim_def_json: *const c_char, claim_def_uuid: *const c_char) {
            let mut callbacks = CREATE_CLAIM_DEFINITION_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let claim_def_json = unsafe { CStr::from_ptr(claim_def_json).to_str().unwrap().to_string() };
            let claim_def_uuid = unsafe { CStr::from_ptr(claim_def_uuid).to_str().unwrap().to_string() };
            cb(err, claim_def_json, claim_def_uuid)
        }

        let mut callbacks = CREATE_CLAIM_DEFINITION_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(create_claim_definition_callback))
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

    pub fn closure_to_wallet_set_seq_no_for_value_cb(closure: Box<FnMut(ErrorCode) + Send>) -> (i32,
                                                                                                Option<extern fn(command_handle: i32,
                                                                                                                 err: ErrorCode)>) {
        lazy_static! {
            static ref WALLET_SET_SEQ_NO_FOR_VALUE_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        extern "C" fn closure_to_wallet_set_seq_no_for_value_callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = WALLET_SET_SEQ_NO_FOR_VALUE_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = WALLET_SET_SEQ_NO_FOR_VALUE_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(closure_to_wallet_set_seq_no_for_value_callback))
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

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(ErrorCode, String, String, String) + Send>) -> (i32,
                                                                                                                    Option<extern fn(command_handle: i32,
                                                                                                                                     err: ErrorCode,
                                                                                                                                     did: *const c_char,
                                                                                                                                     verkey: *const c_char,
                                                                                                                                     pk: *const c_char)>) {
        lazy_static! {
            static ref CREATE_AND_STORE_MY_DID_CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String, String) + Send > >> = Default::default();
        }

        extern "C" fn create_and_store_my_did_callback(command_handle: i32, err: ErrorCode, did: *const c_char, verkey: *const c_char, pk: *const c_char) {
            let mut callbacks = CREATE_AND_STORE_MY_DID_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let did = unsafe { CStr::from_ptr(did).to_str().unwrap().to_string() };
            let verkey = unsafe { CStr::from_ptr(verkey).to_str().unwrap().to_string() };
            let pk = unsafe { CStr::from_ptr(pk).to_str().unwrap().to_string() };
            cb(err, did, verkey, pk)
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

    pub fn closure_to_sign_cb(closure: Box<FnMut(ErrorCode, String) + Send>)
                              -> (i32,
                                  Option<extern fn(command_handle: i32, err: ErrorCode,
                                                   signature: *const c_char)>) {
        lazy_static! {
            static ref SIGN_CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        extern "C" fn sign_callback(command_handle: i32, err: ErrorCode, signature: *const c_char) {
            let mut callbacks = SIGN_CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let signature = unsafe { CStr::from_ptr(signature).to_str().unwrap().to_string() };
            cb(err, signature);
        }

        let mut callbacks = SIGN_CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(sign_callback))
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
}
