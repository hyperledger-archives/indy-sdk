extern crate libc;

use settings;
use utils::libindy::{
    pool::get_pool_handle,
    wallet::get_wallet_handle,
};
use utils::error;
use indy::ledger::Ledger;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use utils::timeout::TimeoutUtils;

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
    Ledger::build_get_txn_request(submitter_did, None, sequence_num)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> Result<String, u32> {
    Ledger::build_schema_request(submitter_did, data)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> Result<String, u32> {
    Ledger::build_get_schema_request(submitter_did, schema_id)
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
    Ledger::build_get_cred_def_request(&submitter_did, cred_def_id).map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               credential_def_json: &str)  -> Result<String, u32>{
    Ledger::build_cred_def_request(submitter_did, credential_def_json).map_err(map_rust_indy_sdk_error_code)
}
