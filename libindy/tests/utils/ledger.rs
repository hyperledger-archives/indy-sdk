extern crate time;
extern crate serde_json;

use indy::api::ErrorCode;
use indy::api::ledger::*;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::anoncreds::AnoncredsUtils;
use utils::blob_storage::BlobStorageUtils;
use utils::constants::*;
use utils::anoncreds_types::{CredentialDefinition, RevocationRegistryDefinition};

use std::ffi::CString;
use std::ptr::null;

pub struct LedgerUtils {}

impl LedgerUtils {
    const SUBMIT_RETRY_CNT: usize = 3;
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

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
        LedgerUtils::_submit_retry(LedgerUtils::_extract_seq_no_from_reply(previous_response).unwrap(), || {
            LedgerUtils::submit_request(pool_handle, request_json)
        })
    }

    pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let request_json = CString::new(request_json).unwrap();

        let err = indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

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

    fn _extract_seq_no_from_reply(reply: &str) -> Result<u64, &'static str> {
        ::serde_json::from_str::<::serde_json::Value>(reply).map_err(|_| "Reply isn't valid JSON")?
            ["result"]["seqNo"]
            .as_u64().ok_or("Missed seqNo in reply")
    }

    fn _submit_retry<F>(minimal_timestamp: u64, submit_action: F) -> Result<String, ErrorCode>
        where F: Fn() -> Result<String, ErrorCode> {
        let mut i = 0;
        let action_result = loop {
            let action_result = submit_action()?;

            let retry = LedgerUtils::_extract_seq_no_from_reply(&action_result)
                .map(|received_timestamp| received_timestamp < minimal_timestamp)
                .unwrap_or(true);

            if retry && i < LedgerUtils::SUBMIT_RETRY_CNT {
                ::std::thread::sleep(TimeoutUtils::short_timeout());
                i += 1;
            } else {
                break action_result;
            }
        };
        Ok(action_result)
    }

    pub fn build_get_ddo_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err = indy_build_get_ddo_request(command_handle, submitter_did.as_ptr(), target_did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let verkey_str = verkey.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let data_str = data.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let role_str = role.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let err =
            indy_build_nym_request(command_handle,
                                   submitter_did.as_ptr(),
                                   target_did.as_ptr(),
                                   if verkey.is_some() { verkey_str.as_ptr() } else { null() },
                                   if data.is_some() { data_str.as_ptr() } else { null() },
                                   if role.is_some() { role_str.as_ptr() } else { null() },
                                   cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_build_attrib_request(command_handle,
                                      submitter_did.as_ptr(),
                                      target_did.as_ptr(),
                                      if hash.is_some() { hash_str.as_ptr() } else { null() },
                                      if raw.is_some() { raw_str.as_ptr() } else { null() },
                                      if enc.is_some() { enc_str.as_ptr() } else { null() },
                                      cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_attrib_request(submitter_did: &str, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_build_get_attrib_request(command_handle,
                                          submitter_did.as_ptr(),
                                          target_did.as_ptr(),
                                          if raw.is_some() { raw_str.as_ptr() } else { null() },
                                          if hash.is_some() { hash_str.as_ptr() } else { null() },
                                          if enc.is_some() { enc_str.as_ptr() } else { null() },
                                          cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_nym_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err = indy_build_get_nym_request(command_handle, submitter_did.as_ptr(), target_did.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = indy_build_schema_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_schema_request(submitter_did: &str, dest: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let dest = CString::new(dest).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_get_schema_request(command_handle,
                                          submitter_did.as_ptr(),
                                          dest.as_ptr(),
                                          data.as_ptr(),
                                          cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, cred_def_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let cred_def_json = CString::new(cred_def_json).unwrap();

        let err =
            indy_build_claim_def_txn(command_handle,
                                     submitter_did.as_ptr(),
                                     xref,
                                     signature_type.as_ptr(),
                                     cred_def_json.as_ptr(),
                                     cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, origin: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let origin = CString::new(origin).unwrap();

        let err =
            indy_build_get_claim_def_txn(command_handle,
                                         submitter_did.as_ptr(),
                                         xref,
                                         signature_type.as_ptr(),
                                         origin.as_ptr(),
                                         cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

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

    pub fn build_get_txn_request(submitter_did: &str, data: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();

        let err = indy_build_get_txn_request(command_handle, submitter_did.as_ptr(), data, cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();

        let err = indy_build_pool_config_request(command_handle, submitter_did.as_ptr(), writes, force, cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_pool_upgrade_request(submitter_did: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                                      justification: Option<&str>, reinstall: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let name = CString::new(name).unwrap();
        let version = CString::new(version).unwrap();
        let action = CString::new(action).unwrap();
        let sha256 = CString::new(sha256).unwrap();
        let timeout = timeout.map(|t| t as i32).unwrap_or(-1);

        let schedule_str = schedule.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let justification_str = justification.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_build_pool_upgrade_request(command_handle,
                                            submitter_did.as_ptr(),
                                            name.as_ptr(),
                                            version.as_ptr(),
                                            action.as_ptr(),
                                            sha256.as_ptr(),
                                            timeout,
                                            if schedule.is_some() { schedule_str.as_ptr() } else { null() },
                                            if justification.is_some() { justification_str.as_ptr() } else { null() },
                                            reinstall,
                                            force,
                                            cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_revoc_reg_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

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
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

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

    pub fn build_get_revoc_reg_def_request(submitter_did: &str, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let id = CString::new(id).unwrap();

        let err =
            indy_build_get_revoc_reg_def_request(command_handle,
                                                 submitter_did.as_ptr(),
                                                 id.as_ptr(),
                                                 cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_revoc_reg_request(submitter_did: &str, rev_reg_def_id: &str, timestamp: i64) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let rev_reg_def_id = CString::new(rev_reg_def_id).unwrap();

        let err =
            indy_build_get_revoc_reg_request(command_handle,
                                             submitter_did.as_ptr(),
                                             rev_reg_def_id.as_ptr(),
                                             timestamp,
                                             cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_revoc_reg_delta_request(submitter_did: &str, rev_reg_def_id: &str, from: Option<i64>, to: i64) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let rev_reg_def_id = CString::new(rev_reg_def_id).unwrap();

        let from = if from.is_some() { from.unwrap() } else { -1 };

        let err =
            indy_build_get_revoc_reg_delta_request(command_handle,
                                                   submitter_did.as_ptr(),
                                                   rev_reg_def_id.as_ptr(),
                                                   from,
                                                   to,
                                                   cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn post_schema_to_ledger(pool_handle: i32, wallet_handle: i32, did: &str) -> (i32, String) {
        let (_schema_id, schema_json) = AnoncredsUtils::issuer_create_schema(&did,
                                                                            GVT_SCHEMA_NAME,
                                                                            SCHEMA_VERSION,
                                                                            GVT_SCHEMA_ATTRIBUTES).unwrap();

        let schema_request = LedgerUtils::build_schema_request(did, &schema_json).unwrap();
        let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &schema_request).unwrap();
        let schema_seq_no = serde_json::from_str::<serde_json::Value>(&schema_req_resp).unwrap()["result"]["seqNo"].as_i64().unwrap();
        (schema_seq_no as i32, schema_json)
    }

    pub fn prepare_claim_def(wallet_handle: i32, did: &str, schema_json: &str) -> (String, String) {
        let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(wallet_handle,
                                                                                               &did,
                                                                                               &schema_json,
                                                                                               TAG_1,
                                                                                               None,
                                                                                               &AnoncredsUtils::revocation_cred_def_config()).unwrap();

        let cred_def = serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap();
        let cred_def_value_json = serde_json::to_string(&&cred_def.value).unwrap();
        (cred_def_id, cred_def_value_json)
    }

    pub fn post_claim_def_to_ledger(pool_handle: i32, wallet_handle: i32, did: &str, schema_seq_no: i32, schema_json: &str) -> (String, String) {
        let (cred_def_id, cred_def_value_json) = LedgerUtils::prepare_claim_def(wallet_handle, did, schema_json);

        let claim_def_request = LedgerUtils::build_claim_def_txn(&did, schema_seq_no, SIGNATURE_TYPE, &cred_def_value_json).unwrap();
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &claim_def_request).unwrap();

        let cred_def_id_in_ledger = LedgerUtils::build_id(&did, "\x03", &schema_seq_no.to_string(), SIGNATURE_TYPE); // TODO FIXME
        (cred_def_id, cred_def_id_in_ledger)
    }

    pub fn prepare_rev_reg(wallet_handle: i32, did: &str, cred_def_id_in_wallet: &str, cred_def_id_in_ledger: &str) -> (String, String, String) {
        let tails_writer_config = AnoncredsUtils::tails_writer_config();
        let tails_writer_handle = BlobStorageUtils::open_writer("default", &tails_writer_config).unwrap();

        let (_, revoc_reg_def_json, rev_reg_entry_json) =
            AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(wallet_handle,
                                                                   &did,
                                                                   None,
                                                                   TAG_1,
                                                                   &cred_def_id_in_wallet,
                                                                   &AnoncredsUtils::default_rev_reg_config(),
                                                                   tails_writer_handle).unwrap();

        let mut revoc_reg_def = serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap();

        let rev_reg_id = AnoncredsUtils::build_id(&did, "\x04", Some(&cred_def_id_in_ledger), REVOC_REG_TYPE, TAG_1);
        revoc_reg_def.id = rev_reg_id.clone();
        revoc_reg_def.cred_def_id = cred_def_id_in_ledger.to_string();

        let revoc_reg_def_json = serde_json::to_string(&&revoc_reg_def).unwrap();

        (rev_reg_id, revoc_reg_def_json, rev_reg_entry_json)
    }

    pub fn post_rev_reg_def(pool_handle: i32, wallet_handle: i32, did: &str, cred_def_id_in_wallet: &str, cred_def_id_in_ledger: &str) -> (String, String, String) {
        let (rev_reg_id, revoc_reg_def_json, rev_reg_entry_json) = LedgerUtils::prepare_rev_reg(wallet_handle, did, cred_def_id_in_wallet, cred_def_id_in_ledger);

        let rev_reg_def_request = LedgerUtils::build_revoc_reg_def_request(&did, &revoc_reg_def_json).unwrap();
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &rev_reg_def_request).unwrap();

        (rev_reg_id, revoc_reg_def_json, rev_reg_entry_json)
    }

    pub fn post_rev_reg_entry(pool_handle: i32, wallet_handle: i32, did: &str, rev_reg_id: &str, _rev_reg_entry_json: &str) -> String {
        let rev_reg_entry_value = r#"{"accum":"123456789", "issued":[], "revoked":[]}"#; // TODO: Node crashes without issued or revoked

        let rev_reg_entry_request = LedgerUtils::build_revoc_reg_entry_request(&did, &rev_reg_id, REVOC_REG_TYPE, rev_reg_entry_value).unwrap();
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &did, &rev_reg_entry_request).unwrap()
    }

    pub fn build_id(identifier: &str, marker: &str, related_entity_id: &str, _type: &str) -> String {
        format!("{}:{}:{}:{}", identifier, marker, _type, related_entity_id)
    }
}
