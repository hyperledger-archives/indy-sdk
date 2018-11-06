extern crate libc;
extern crate time;
extern crate serde_json;

use settings;
use serde_json::Value;
use utils::libindy::{
    pool::get_pool_handle,
    wallet::get_wallet_handle,
};
use utils::error;
use indy::ledger::Ledger;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use utils::timeout::TimeoutUtils;
use utils::libindy::payments::{pay_for_txn, PaymentTxn};
use utils::constants::{ SCHEMA_ID, SCHEMA_JSON, SCHEMA_TXN_TYPE, CRED_DEF_ID, CRED_DEF_JSON, CRED_DEF_TXN_TYPE, REV_REG_DEF_TXN_TYPE, REV_REG_DELTA_TXN_TYPE, REVOC_REG_TYPE, REV_DEF_JSON, REV_REG_ID, REV_REG_DELTA_JSON, REV_REG_JSON};
use utils::libindy::anoncreds::{ libindy_create_and_store_credential_def, libindy_create_and_store_revoc_reg, libindy_issuer_create_schema, libindy_issuer_revoke_credential };

pub fn multisign_request(did: &str, request: &str) -> Result<String, u32> {
   Ledger::multi_sign_request(get_wallet_handle(), did, request)
       .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_sign_request(did: &str, request: &str) -> Result<String,u32> {
    Ledger::sign_request(get_wallet_handle(), did, request)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_sign_and_submit_request(issuer_did: &str, request_json: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(r#"{"rc":"success"}"#.to_string()); }
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    let wallet_handle = get_wallet_handle();

    Ledger::sign_and_submit_request(pool_handle, wallet_handle, issuer_did, request_json)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_submit_request(request_json: &str) -> Result<String, u32> {
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    Ledger::submit_request_timeout(pool_handle, request_json, TimeoutUtils::long_timeout()).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_txn_request(submitter_did: &str, sequence_num: i32) -> Result<String, u32> {
    Ledger::build_get_txn_request(Some(submitter_did), None, sequence_num)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> Result<String, u32> {
    Ledger::build_schema_request(submitter_did, data)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> Result<String, u32> {
    Ledger::build_get_schema_request(Some(submitter_did), schema_id)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_parse_get_schema_response(get_schema_response: &str) -> Result<(String, String), u32> {
    Ledger::parse_get_schema_response(get_schema_response).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_parse_get_cred_def_response(get_cred_def_response: &str) -> Result<(String, String), u32> {
    Ledger::parse_get_cred_def_response(get_cred_def_response).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_credential_def_txn(cred_def_id: &str)  -> Result<String, u32>{
    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    Ledger::build_get_cred_def_request(Some(&submitter_did), cred_def_id).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               credential_def_json: &str)  -> Result<String, u32>{
    Ledger::build_cred_def_request(submitter_did, credential_def_json).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_revoc_reg_def_request(submitter_did: &str,
                                           rev_reg_def_json: &str) -> Result<String, u32> {
   Ledger::build_revoc_reg_def_request(submitter_did, rev_reg_def_json).map_err(map_rust_indy_sdk_error_code)
}


pub fn libindy_build_revoc_reg_entry_request(submitter_did: &str,
                                             rev_reg_id: &str,
                                             rev_def_type: &str,
                                             value: &str) -> Result<String, u32> {
    Ledger::build_revoc_reg_entry_request(submitter_did, rev_reg_id, rev_def_type, value)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_revoc_reg_def_request(submitter_did: &str, rev_reg_id: &str) -> Result<String, u32> {
    Ledger::build_get_revoc_reg_def_request(Some(submitter_did), rev_reg_id).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_parse_get_revoc_reg_def_response(rev_reg_def_json: &str) -> Result<(String, String), u32> {
    Ledger::parse_get_revoc_reg_def_response(rev_reg_def_json)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_revoc_reg_delta_request(submitter_did: &str,
                                                 rev_reg_id: &str,
                                                 from: i64,
                                                 to: i64) -> Result<String, u32> {
    Ledger::build_get_revoc_reg_delta_request(Some(submitter_did),
                                              rev_reg_id,
                                              from,
                                              to).map_err(map_rust_indy_sdk_error_code)
}

fn libindy_build_get_revoc_reg_request(submitter_did: &str, rev_reg_id: &str, timestamp: u64)
    -> Result<String, u32> {
    Ledger::build_get_revoc_reg_request(Some(submitter_did),
                                              rev_reg_id,
                                              timestamp as i64)
        .map_err(map_rust_indy_sdk_error_code)
}

fn libindy_parse_get_revoc_reg_response(get_rev_reg_resp: &str) -> Result<(String, String, u64), u32> {
    Ledger::parse_get_revoc_reg_response(get_rev_reg_resp)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_parse_get_revoc_reg_delta_response(get_rev_reg_delta_response: &str)
    -> Result<(String, String, u64), u32> {
    Ledger::parse_get_revoc_reg_delta_response(get_rev_reg_delta_response)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn create_schema(name: &str, version: &str, data: &str) -> Result<(String, Option<PaymentTxn>), u32> {
    if settings::test_indy_mode_enabled() {
        return Ok((SCHEMA_ID.to_string(), Some(PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"recipient":"pay:null:xkIsxem0YNtHrRO"}]"#,1, false).unwrap(), )));
    }

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    let (id, create_schema) = libindy_issuer_create_schema(&submitter_did, name, version, data)?;

    let request = libindy_build_schema_request(&submitter_did, &create_schema)?;

    let (payment, response) = pay_for_txn(&request, SCHEMA_TXN_TYPE)?;

    _check_create_schema_response(&response)?;

    Ok((id, payment))
}

fn _check_create_schema_response(response: &str) -> Result<(), u32> {
    let response: Value = serde_json::from_str(response).or(Err(error::INVALID_JSON.code_num))?;

    if let Some(_) = response.get("result") { return Ok(()) };

    warn!("No result found in ledger txn. Must be Rejected");

    if response["op"] == json!("REJECT") {
        match response.get("reason") {
            Some(r) => return Err(error::DUPLICATE_SCHEMA.code_num),
            None => return Err(error::UNKNOWN_SCHEMA_REJECTION.code_num),
        }
    }

    Err(error::UNKNOWN_SCHEMA_REJECTION.code_num)
}

pub fn get_schema_json(schema_id: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((SCHEMA_ID.to_string(), SCHEMA_JSON.to_string()))}

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    libindy_build_get_schema_request(&submitter_did, schema_id)
        .and_then(|req| libindy_submit_request(&req))
        .and_then(|response| libindy_parse_get_schema_response(&response))
}

pub fn create_cred_def(issuer_did: &str,
                       schema_json: &str,
                       tag: &str,
                       sig_type: Option<&str>,
                       support_revocation: Option<bool>) -> Result<(String, Option<PaymentTxn>), u32> {
    if settings::test_indy_mode_enabled() {
        return Ok((CRED_DEF_ID.to_string(), Some(PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"recipient":"pay:null:xkIsxem0YNtHrRO"}]"#,1, false).unwrap())));
    }

    let config_json = json!({"support_revocation": support_revocation.unwrap_or(false)}).to_string();

    let (id, cred_def_json) = libindy_create_and_store_credential_def(issuer_did,
                                                                      schema_json,
                                                                      tag,
                                                                      sig_type,
                                                                      &config_json)?;

    let cred_def_req = libindy_build_create_credential_def_txn(issuer_did, &cred_def_json)?;

    let (payment, response) = pay_for_txn(&cred_def_req, CRED_DEF_TXN_TYPE)?;

    Ok((id, payment))
}

pub fn get_cred_def_json(cred_def_id: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((CRED_DEF_ID.to_string(), CRED_DEF_JSON.to_string())); }

    libindy_build_get_credential_def_txn(cred_def_id)
        .and_then(|req| libindy_submit_request(&req))
        .and_then(|response| libindy_parse_get_cred_def_response(&response))
}

pub fn create_rev_reg_def(issuer_did: &str, cred_def_id: &str, tails_file: &str, max_creds: u32)
    -> Result<(String, String, String, Option<PaymentTxn>), u32> {
    debug!("creating revocation registry definition with issuer_did: {}, cred_def_id: {}, tails_file_path: {}, max_creds: {}",
           issuer_did, cred_def_id, tails_file, max_creds);
    if settings::test_indy_mode_enabled() { return Ok((REV_REG_ID.to_string(), REV_DEF_JSON.to_string(), "".to_string(), None)); }

    let (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) = libindy_create_and_store_revoc_reg(
        issuer_did,
        cred_def_id,
        tails_file,
        max_creds
    )?;

    let rev_reg_def_req = libindy_build_revoc_reg_def_request(issuer_did, &rev_reg_def_json)?;

    let (payment, _) = pay_for_txn(&rev_reg_def_req, REV_REG_DEF_TXN_TYPE)?;

    Ok((rev_reg_id, rev_reg_def_json, rev_reg_entry_json, payment))
}

pub fn get_rev_reg_def_json(rev_reg_id: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((REV_REG_ID.to_string(), REV_DEF_JSON.to_string())); }

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    libindy_build_get_revoc_reg_def_request(&submitter_did, rev_reg_id)
        .and_then(|req| libindy_submit_request(&req))
        .and_then(|response| libindy_parse_get_revoc_reg_def_response(&response))
}

pub fn post_rev_reg_delta(issuer_did: &str, rev_reg_id: &str, rev_reg_entry_json: &str)
                          -> Result<(Option<PaymentTxn>, String), u32> {
    libindy_build_revoc_reg_entry_request(issuer_did, rev_reg_id, REVOC_REG_TYPE, rev_reg_entry_json)
        .and_then(|req| pay_for_txn(&req, REV_REG_DELTA_TXN_TYPE))
}

pub fn get_rev_reg_delta_json(rev_reg_id: &str, from: Option<u64>, to: Option<u64>)
    -> Result<(String, String, u64), u32> {
    if settings::test_indy_mode_enabled() { return Ok((REV_REG_ID.to_string(), REV_REG_DELTA_JSON.to_string(), 1)); }

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    let from: i64 = if let Some(_from) = from { _from as i64 } else { -1 };
    let to = if let Some(_to) = to { _to as i64 } else { time::get_time().sec };

    libindy_build_get_revoc_reg_delta_request(&submitter_did, rev_reg_id, from, to)
        .and_then(|req| libindy_submit_request(&req))
        .and_then(|response| libindy_parse_get_revoc_reg_delta_response(&response))
}

pub fn get_rev_reg(rev_reg_id: &str, timestamp: u64) -> Result<(String, String, u64), u32> {
    if settings::test_indy_mode_enabled() { return Ok((REV_REG_ID.to_string(), REV_REG_JSON.to_string(), 1)); }
    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    libindy_build_get_revoc_reg_request(&submitter_did, rev_reg_id, timestamp)
        .and_then(|req| libindy_submit_request(&req))
        .and_then(|response| libindy_parse_get_revoc_reg_response(&response))
}

pub fn revoke_credential(tails_file: &str, rev_reg_id: &str, cred_rev_id: &str)
    -> Result<(Option<PaymentTxn>, String), u32> {
    if settings::test_indy_mode_enabled() {
        return Ok((Some(PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"recipient":"pay:null:xkIsxem0YNtHrRO"}]"#,1, false).unwrap()), REV_REG_DELTA_JSON.to_string()));
    }

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    let delta = libindy_issuer_revoke_credential(tails_file, rev_reg_id, cred_rev_id)?;
    let (payment, _) = post_rev_reg_delta(&submitter_did, rev_reg_id, &delta)?;

    Ok((payment, delta))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::constants::{SCHEMAS_JSON};
    #[cfg(feature = "pool_tests")]
    use utils::constants::{TEST_TAILS_FILE};

    #[test]
    fn test_create_cred_def() {
        init!("true");
        let (id, _) = create_cred_def("did", SCHEMAS_JSON,  "tag_1", None, Some(false)).unwrap();
        assert_eq!(id, CRED_DEF_ID);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_cred_def_real() {
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let (_, schema_json) = get_schema_json(&schema_id).unwrap();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let revocation_details = json!({"support_revocation": true, "tails_file": "/tmp/tails.txt", "max_creds": 2}).to_string();
        create_cred_def(&did, &schema_json, "tag_1", None, Some(true)).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_rev_reg_def_fails_for_cred_def_created_without_revocation() {
        let wallet_name = "test_create_revocable_fails_with_no_tails_file";
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        // Cred def is created with support_revocation=false,
        // revoc_reg_def will fail in libindy because cred_Def doesn't have revocation keys
        let (_, _, cred_def_id, _, _, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let rc = create_rev_reg_def(&did, &cred_def_id, "/tmp/path.txt", 2);

        assert_eq!(rc, Err(error::LIBINDY_INVALID_STRUCTURE.code_num));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_rev_reg_def() {
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let (_, schema_json) = get_schema_json(&schema_id).unwrap();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let revocation_details = json!({"support_revocation": true, "tails_file": "/tmp/tails.txt", "max_creds": 2}).to_string();
        let (id, payment) = create_cred_def(&did, &schema_json, "tag_1", None, Some(true)).unwrap();
        create_rev_reg_def(&did, &id, "/tmp/tails.txt", 2).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_rev_reg_def_json() {
        init!("ledger");
        let attrs = r#"["address1","address2","city","state","zip"]"#;
        let (_, _, _, _, _, rev_reg_id) =
            ::utils::libindy::anoncreds::tests::create_and_store_credential_def(attrs, true);

        let rev_reg_id = rev_reg_id.unwrap();
        let (id, json) = get_rev_reg_def_json(&rev_reg_id).unwrap();
        assert_eq!(id, rev_reg_id);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_rev_reg_delta_json() {
        init!("ledger");
        let attrs = r#"["address1","address2","city","state","zip"]"#;
        let (_, _, _, _, _, rev_reg_id) =
            ::utils::libindy::anoncreds::tests::create_and_store_credential_def(attrs, true);
        let rev_reg_id = rev_reg_id.unwrap();

        let (id, delta, timestamp) = get_rev_reg_delta_json(&rev_reg_id, None, None).unwrap();
        assert_eq!(id, rev_reg_id);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_rev_reg() {
        init!("ledger");
        let attrs = r#"["address1","address2","city","state","zip"]"#;
        let (_, _, _, _, _, rev_reg_id) =
            ::utils::libindy::anoncreds::tests::create_and_store_credential_def(attrs, true);
        let rev_reg_id = rev_reg_id.unwrap();

        let (id, rev_reg, timestamp) = get_rev_reg(&rev_reg_id, time::get_time().sec as u64).unwrap();
        assert_eq!(id, rev_reg_id);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn from_pool_ledger_with_id(){
        use settings;
        init!("ledger");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, schema_json) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);

        let rc = get_schema_json(&schema_id);

        let (id, retrieved_schema) = rc.unwrap();
        assert!(retrieved_schema.contains(&schema_id));

    }

    #[test]
    fn from_ledger_schema_id(){
        init!("true");
        let (id, retrieved_schema) = get_schema_json(SCHEMA_ID).unwrap();
        assert_eq!(&retrieved_schema, SCHEMA_JSON);
        assert_eq!(&id, SCHEMA_ID);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_revoke_credential(){
        init!("ledger");
        let (_, _, cred_def_id, _, _, _, _, cred_id, rev_reg_id, cred_rev_id)
        = ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS, true);

        let rev_reg_id = rev_reg_id.unwrap();
        let (_, first_rev_reg_delta, _) = get_rev_reg_delta_json(&rev_reg_id, None, None).unwrap();
        let (_, test_same_delta, _) = get_rev_reg_delta_json(&rev_reg_id, None, None).unwrap();

        assert_eq!(first_rev_reg_delta,  test_same_delta);
        let (payment, revoked_rev_reg_delta) = revoke_credential(TEST_TAILS_FILE, &rev_reg_id, cred_rev_id.unwrap().as_str()).unwrap();

        // Delta should change after revocation
        let (_, second_rev_reg_delta, _) = get_rev_reg_delta_json(&rev_reg_id, None, None).unwrap();

        assert!(payment.is_some());
        assert_ne!(first_rev_reg_delta, second_rev_reg_delta);
    }
}
