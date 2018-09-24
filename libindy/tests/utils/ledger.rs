extern crate time;
extern crate serde;
extern crate serde_json;

use indy::api::ErrorCode;
use indy::api::ledger::*;

use utils::{callback, timeout, anoncreds, blob_storage, did, wallet, pool, ctypes};
use utils::constants::*;

use std::ffi::CString;
use std::ptr::null;
use std::sync::{Once, ONCE_INIT};
use std::mem;

pub static mut SCHEMA_ID: &'static str = "";
pub static mut CRED_DEF_ID: &'static str = "";
pub static mut REV_REG_DEF_ID: &'static str = "";
pub const SCHEMA_DATA: &'static str = r#"{"id":"id","name":"gvt","version":"1.0","attr_names":["name", "age", "sex", "height"]}"#;

const SUBMIT_RETRY_CNT: usize = 3;

pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let request_json = CString::new(request_json).unwrap();

    let err =
        indy_sign_and_submit_request(command_handle,
                                     pool_handle,
                                     wallet_handle,
                                     submitter_did.as_ptr(),
                                     request_json.as_ptr(),
                                     cb);

    super::results::result_to_string(err, receiver)
}

pub fn submit_request_with_retries(pool_handle: i32, request_json: &str, previous_response: &str) -> Result<String, ErrorCode> {
    _submit_retry(_extract_seq_no_from_reply(previous_response).unwrap(), || {
        submit_request(pool_handle, request_json)
    })
}

pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let request_json = CString::new(request_json).unwrap();

    let err = indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb);

    super::results::result_to_string(err, receiver)
}

pub fn submit_action(pool_handle: i32, request_json: &str, nodes: Option<&str>, timeout: Option<i32>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let request_json = CString::new(request_json).unwrap();
    let nodes = nodes.map(ctypes::str_to_cstring);
    let timeout = timeout.unwrap_or(-1);

    let err = indy_submit_action(command_handle,
                                 pool_handle,
                                 request_json.as_ptr(),
                                 nodes.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                 timeout,
                                 cb);

    super::results::result_to_string(err, receiver)
}

pub fn sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let request_json = CString::new(request_json).unwrap();

    let err =
        indy_sign_request(command_handle,
                          wallet_handle,
                          submitter_did.as_ptr(),
                          request_json.as_ptr(),
                          cb);

    super::results::result_to_string(err, receiver)
}

pub fn multi_sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let request_json = CString::new(request_json).unwrap();

    let err =
        indy_multi_sign_request(command_handle,
                                wallet_handle,
                                submitter_did.as_ptr(),
                                request_json.as_ptr(),
                                cb);

    super::results::result_to_string(err, receiver)
}

fn _extract_seq_no_from_reply(reply: &str) -> Result<u64, &'static str> {
    ::serde_json::from_str::<::serde_json::Value>(reply).map_err(|_| "Reply isn't valid JSON")?
        ["result"]["txnMetadata"]["seqNo"]
        .as_u64().ok_or("Missed seqNo in reply")
}

fn _submit_retry<F>(minimal_timestamp: u64, submit_action: F) -> Result<String, ErrorCode>
    where F: Fn() -> Result<String, ErrorCode> {
    let mut i = 0;
    let action_result = loop {
        let action_result = submit_action()?;

        let retry = _extract_seq_no_from_reply(&action_result)
            .map(|received_timestamp| received_timestamp < minimal_timestamp)
            .unwrap_or(true);

        if retry && i < SUBMIT_RETRY_CNT {
            ::std::thread::sleep(timeout::short_timeout());
            i += 1;
        } else {
            break action_result;
        }
    };
    Ok(action_result)
}

pub fn build_get_ddo_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let target_did = CString::new(target_did).unwrap();

    let err = indy_build_get_ddo_request(command_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         target_did.as_ptr(), cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                         alias: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let target_did = CString::new(target_did).unwrap();

    let verkey = verkey.map(ctypes::str_to_cstring);
    let alias = alias.map(ctypes::str_to_cstring);
    let role = role.map(ctypes::str_to_cstring);
    let err =
        indy_build_nym_request(command_handle,
                               submitter_did.as_ptr(),
                               target_did.as_ptr(),
                               verkey.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                               alias.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                               role.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                               cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let target_did = CString::new(target_did).unwrap();

    let hash = hash.map(ctypes::str_to_cstring);
    let raw = raw.map(ctypes::str_to_cstring);
    let enc = enc.map(ctypes::str_to_cstring);

    let err =
        indy_build_attrib_request(command_handle,
                                  submitter_did.as_ptr(),
                                  target_did.as_ptr(),
                                  hash.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                  raw.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                  enc.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                  cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_attrib_request(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let target_did = CString::new(target_did).unwrap();
    let raw = raw.map(ctypes::str_to_cstring);
    let hash = hash.map(ctypes::str_to_cstring);
    let enc = enc.map(ctypes::str_to_cstring);

    let err =
        indy_build_get_attrib_request(command_handle,
                                      submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                      target_did.as_ptr(),
                                      raw.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                      hash.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                      enc.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                      cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_nym_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let target_did = CString::new(target_did).unwrap();

    let err = indy_build_get_nym_request(command_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         target_did.as_ptr(), cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let data = CString::new(data).unwrap();

    let err = indy_build_schema_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_schema_request(submitter_did:  Option<&str>, id: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let id = CString::new(id).unwrap();

    let err =
        indy_build_get_schema_request(command_handle,
                                      submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                      id.as_ptr(),
                                      cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_cred_def_txn(submitter_did: &str, cred_def_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let cred_def_json = CString::new(cred_def_json).unwrap();

    let err =
        indy_build_cred_def_request(command_handle,
                                    submitter_did.as_ptr(),
                                    cred_def_json.as_ptr(),
                                    cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_cred_def_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let id = CString::new(id).unwrap();

    let err =
        indy_build_get_cred_def_request(command_handle,
                                        submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                        id.as_ptr(),
                                        cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let target_did = CString::new(target_did).unwrap();
    let data = CString::new(data).unwrap();

    let err =
        indy_build_node_request(command_handle,
                                submitter_did.as_ptr(),
                                target_did.as_ptr(),
                                data.as_ptr(),
                                cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_validator_info_request(submitter_did: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_get_validator_info_request(command_handle, submitter_did.as_ptr(), cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_txn_request(submitter_did: Option<&str>, data: i32, ledger_type: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let ledger_type = ledger_type.map(ctypes::str_to_cstring);

    let err = indy_build_get_txn_request(command_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         ledger_type.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         data, cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();

    let err = indy_build_pool_config_request(command_handle, submitter_did.as_ptr(), writes, force, cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_pool_restart_request(submitter_did: &str,
                                  action: &str,
                                  datetime: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let action = CString::new(action).unwrap();
    let datetime = datetime.map(ctypes::str_to_cstring);

    let err = indy_build_pool_restart_request(command_handle,
                                              submitter_did.as_ptr(),
                                              action.as_ptr(),
                                              datetime.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                              cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_pool_upgrade_request(submitter_did: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                                  justification: Option<&str>, reinstall: bool, force: bool, package: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let name = CString::new(name).unwrap();
    let version = CString::new(version).unwrap();
    let action = CString::new(action).unwrap();
    let sha256 = CString::new(sha256).unwrap();
    let timeout = timeout.map(|t| t as i32).unwrap_or(-1);

    let schedule = schedule.map(ctypes::str_to_cstring);
    let justification = justification.map(ctypes::str_to_cstring);
    let package = package.map(ctypes::str_to_cstring);

    let err =
        indy_build_pool_upgrade_request(command_handle,
                                        submitter_did.as_ptr(),
                                        name.as_ptr(),
                                        version.as_ptr(),
                                        action.as_ptr(),
                                        sha256.as_ptr(),
                                        timeout,
                                        schedule.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                        justification.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                        reinstall,
                                        force,
                                        package.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                        cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_revoc_reg_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let data = CString::new(data).unwrap();

    let err =
        indy_build_revoc_reg_def_request(command_handle,
                                         submitter_did.as_ptr(),
                                         data.as_ptr(),
                                         cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_revoc_reg_entry_request(submitter_did: &str, rev_reg_def_id: &str, rev_reg_type: &str, value: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = CString::new(submitter_did).unwrap();
    let rev_reg_def_id = CString::new(rev_reg_def_id).unwrap();
    let rev_reg_type = CString::new(rev_reg_type).unwrap();
    let value = CString::new(value).unwrap();

    let err =
        indy_build_revoc_reg_entry_request(command_handle,
                                           submitter_did.as_ptr(),
                                           rev_reg_def_id.as_ptr(),
                                           rev_reg_type.as_ptr(),
                                           value.as_ptr(),
                                           cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_revoc_reg_def_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let id = CString::new(id).unwrap();

    let err =
        indy_build_get_revoc_reg_def_request(command_handle,
                                             submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                             id.as_ptr(),
                                             cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_revoc_reg_request(submitter_did: Option<&str>, rev_reg_def_id: &str, timestamp: u64) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let rev_reg_def_id = CString::new(rev_reg_def_id).unwrap();

    let err =
        indy_build_get_revoc_reg_request(command_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         rev_reg_def_id.as_ptr(),
                                         timestamp as i64,
                                         cb);

    super::results::result_to_string(err, receiver)
}

pub fn build_get_revoc_reg_delta_request(submitter_did: Option<&str>, rev_reg_def_id: &str, from: Option<u64>, to: u64) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let submitter_did = submitter_did.map(ctypes::str_to_cstring);
    let rev_reg_def_id = CString::new(rev_reg_def_id).unwrap();

    let from = if from.is_some() { from.unwrap() as i64 } else { -1 };

    let err =
        indy_build_get_revoc_reg_delta_request(command_handle,
                                               submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                               rev_reg_def_id.as_ptr(),
                                               from,
                                               to as i64,
                                               cb);

    super::results::result_to_string(err, receiver)
}

pub fn parse_get_schema_response(get_schema_response: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string_string();

    let get_schema_response = CString::new(get_schema_response).unwrap();

    let err =
        indy_parse_get_schema_response(command_handle,
                                       get_schema_response.as_ptr(),
                                       cb);

    super::results::result_to_string_string(err, receiver)
}

pub fn parse_get_cred_def_response(get_cred_def_response: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string_string();

    let get_cred_def_response = CString::new(get_cred_def_response).unwrap();

    let err =
        indy_parse_get_cred_def_response(command_handle,
                                         get_cred_def_response.as_ptr(),
                                         cb);

    super::results::result_to_string_string(err, receiver)
}

pub fn parse_get_revoc_reg_def_response(get_revoc_reg_def_response: &str) -> Result<(String, String), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string_string();

    let get_revoc_reg_def_response = CString::new(get_revoc_reg_def_response).unwrap();

    let err =
        indy_parse_get_revoc_reg_def_response(command_handle,
                                              get_revoc_reg_def_response.as_ptr(),
                                              cb);

    super::results::result_to_string_string(err, receiver)
}

pub fn parse_get_revoc_reg_response(get_revoc_reg_response: &str) -> Result<(String, String, u64), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string_string_u64();

    let get_revoc_reg_response = CString::new(get_revoc_reg_response).unwrap();

    let err =
        indy_parse_get_revoc_reg_response(command_handle,
                                          get_revoc_reg_response.as_ptr(),
                                          cb);

    super::results::result_to_string_string_u64(err, receiver)
}

pub fn parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response: &str) -> Result<(String, String, u64), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string_string_u64();

    let get_revoc_reg_delta_response = CString::new(get_revoc_reg_delta_response).unwrap();

    let err =
        indy_parse_get_revoc_reg_delta_response(command_handle,
                                                get_revoc_reg_delta_response.as_ptr(),
                                                cb);

    super::results::result_to_string_string_u64(err, receiver)
}

pub fn register_transaction_parser_for_sp(txn_type: &str, parse: CustomTransactionParser, free: CustomFree) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let txn_type = CString::new(txn_type).unwrap();

    let err =
        indy_register_transaction_parser_for_sp(command_handle,
                                                txn_type.as_ptr(),
                                                Some(parse),
                                                Some(free),
                                                cb);

    super::results::result_to_empty(err, receiver)
}

pub fn post_entities() -> (&'static str, &'static str, &'static str) {
    lazy_static! {
                    static ref COMMON_ENTITIES_INIT: Once = ONCE_INIT;

                }

    unsafe {
        COMMON_ENTITIES_INIT.call_once(|| {
            let pool_name = "COMMON_ENTITIES_POOL";
            let pool_handle = pool::create_and_open_pool_ledger(pool_name).unwrap();

            let wallet_handle = wallet::create_and_open_default_wallet().unwrap();

            let (issuer_did, _) = did::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            let (schema_id, schema_json) = anoncreds::issuer_create_schema(&issuer_did,
                                                                           GVT_SCHEMA_NAME,
                                                                           SCHEMA_VERSION,
                                                                           GVT_SCHEMA_ATTRIBUTES).unwrap();

            let schema_request = build_schema_request(&issuer_did, &schema_json).unwrap();
            let schema_response = sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &schema_request).unwrap();

            let get_schema_request = build_get_schema_request(Some(&issuer_did), &schema_id).unwrap();
            let get_schema_response = submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
            let (schema_id, schema_json) = parse_get_schema_response(&get_schema_response).unwrap();

            let (cred_def_id, cred_def_json) = anoncreds::issuer_create_credential_definition(wallet_handle,
                                                                                              &issuer_did,
                                                                                              &schema_json,
                                                                                              TAG_1,
                                                                                              None,
                                                                                              Some(&anoncreds::revocation_cred_def_config())).unwrap();
            let cred_def_request = build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
            sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &cred_def_request).unwrap();

            let tails_writer_config = anoncreds::tails_writer_config();
            let tails_writer_handle = blob_storage::open_writer("default", &tails_writer_config).unwrap();

            let (rev_reg_id, revoc_reg_def_json, rev_reg_entry_json) =
                anoncreds::issuer_create_and_store_revoc_reg(wallet_handle,
                                                             &issuer_did,
                                                             None,
                                                             TAG_1,
                                                             &cred_def_id,
                                                             &anoncreds::issuance_on_demand_rev_reg_config(),
                                                             tails_writer_handle).unwrap();

            let rev_reg_def_request = build_revoc_reg_def_request(&issuer_did, &revoc_reg_def_json).unwrap();
            sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &rev_reg_def_request).unwrap();

            let rev_reg_entry_request = build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_entry_json).unwrap();
            sign_and_submit_request(pool_handle, wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

            let res = mem::transmute(&schema_id as &str);
            mem::forget(schema_id);
            SCHEMA_ID = res;

            let res = mem::transmute(&cred_def_id as &str);
            mem::forget(cred_def_id);
            CRED_DEF_ID = res;

            let res = mem::transmute(&rev_reg_id as &str);
            mem::forget(rev_reg_id);
            REV_REG_DEF_ID = res;

            wallet::close_wallet(wallet_handle).unwrap();
        });

        (SCHEMA_ID, CRED_DEF_ID, REV_REG_DEF_ID)
    }
}