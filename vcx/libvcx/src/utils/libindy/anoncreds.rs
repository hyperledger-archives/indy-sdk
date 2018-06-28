extern crate libc;
use self::libc::c_char;
use settings;
use std::ffi::CString;
use std::ptr::null;
use utils::constants::{ LIBINDY_CRED_OFFER };
use utils::libindy::{indy_function_eval, check_str, mock_libindy_rc};
use utils::libindy::return_types::{
    Return_I32_STR,
    Return_I32_BOOL,
    Return_I32_STR_STR,
    Return_I32_STR_STR_STR,
};
use utils::libindy::SigTypes;
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use utils::libindy::wallet::get_wallet_handle;
use utils::timeout::TimeoutUtils;

extern {
    fn indy_issuer_create_and_store_credential_def(command_handle: i32,
                                                   wallet_handle: i32,
                                                   issuer_did: *const c_char,
                                                   schema_json: *const c_char,
                                                   tag: *const c_char,
                                                   type_: *const c_char,
                                                   config_json: *const c_char,
                                                   cb: Option<extern fn(xcommand_handle: i32,
                                                                        err: i32,
                                                                        cred_def_id: *const c_char,
                                                                        cred_def_json: *const c_char)>) -> i32;
    fn indy_issuer_create_credential_offer(command_handle: i32,
                                           wallet_handle: i32,
                                           cred_def_id: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32,
                                                                err: i32,
                                                                cred_offer_json: *const c_char)>) -> i32;


    fn indy_issuer_create_credential(command_handle: i32,
                                     wallet_handle: i32,
                                     cred_offer_json: *const c_char,
                                     cred_req_json: *const c_char,
                                     cred_values_json: *const c_char,
                                     rev_reg_id: *const c_char,
                                     blob_storage_reader_handle: i32,
                                     cb: Option<extern fn(xcommand_handle: i32,
                                                          err: i32,
                                                          cred_json: *const c_char,
                                                          cred_revoc_id: *const c_char,
                                                          revoc_reg_delta_json: *const c_char)>) -> i32;

    fn indy_prover_create_credential_req(command_handle: i32,
                                         wallet_handle: i32,
                                         prover_did: *const c_char,
                                         cred_offer_json: *const c_char,
                                         cred_def_json: *const c_char,
                                         master_secret_id: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32,
                                                              err: i32,
                                                              cred_req_json: *const c_char,
                                                              cred_req_metadata_json: *const c_char)>) -> i32;

    fn indy_prover_store_credential(command_handle: i32,
                                    wallet_handle: i32,
                                    cred_id: *const c_char,
                                    cred_req_metadata_json: *const c_char,
                                    cred_json: *const c_char,
                                    cred_def_json: *const c_char,
                                    rev_reg_def_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32,
                                                         err: i32,
                                                         out_cred_id: *const c_char)>) -> i32;

    fn indy_prover_get_credentials_for_proof_req(command_handle: i32,
                                                 wallet_handle: i32,
                                                 proof_request_json: *const c_char,
                                                 cb: Option<extern fn(xcommand_handle: i32,
                                                                      err: i32,
                                                                      credentials_json: *const c_char)>
    ) -> i32;

    fn indy_verifier_verify_proof(command_handle: i32,
                                  proof_request_json: *const c_char,
                                  proof_json: *const c_char,
                                  schemas_json: *const c_char,
                                  credential_defs_json: *const c_char,
                                  rev_reg_defs_json: *const c_char,
                                  rev_regs_json: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32,
                                                       err: i32,
                                                       valid: bool)>) -> i32;

    fn indy_prover_create_proof(command_handle: i32,
                                wallet_handle: i32,
                                proof_req_json: *const c_char,
                                requested_credentials_json: *const c_char,
                                master_secret_id: *const c_char,
                                schemas_json: *const c_char,
                                credential_defs_json: *const c_char,
                                rev_states_json: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32,
                                                     err: i32,
                                                     proof_json: *const c_char)>) -> i32;


    fn indy_prover_create_master_secret(command_handle: i32,
                                        wallet_handle: i32,
                                        master_secret_id: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32,
                                                             err: i32,
                                                             out_master_secret_id: *const c_char)>) -> i32;

    fn indy_issuer_create_schema(command_handle: i32,
                                 issuer_did: *const c_char,
                                 name: *const c_char,
                                 version: *const c_char,
                                 attrs: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                      schema_id: *const c_char,
                                                      schema_json: *const c_char)>) -> i32;
}

pub fn libindy_verifier_verify_proof(proof_req_json: &str,
                                     proof_json: &str,
                                     schemas_json: &str,
                                     credential_defs_json: &str,
                                     rev_reg_defs_json: &str,
                                     rev_regs_json: &str)  -> Result<bool, u32>{

    let rtn_obj = Return_I32_BOOL::new()?;
    let proof_req_json = CString::new(proof_req_json).map_err(map_string_error)?;
    let proof_json = CString::new(proof_json).map_err(map_string_error)?;
    let schemas_json = CString::new(schemas_json).map_err(map_string_error)?;
    let credential_defs_json = CString::new(credential_defs_json).map_err(map_string_error)?;
    let rev_reg_defs_json = CString::new(rev_reg_defs_json).map_err(map_string_error)?;
    let rev_regs_json = CString::new(rev_regs_json).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_verifier_verify_proof(rtn_obj.command_handle,
                                       proof_req_json.as_ptr(),
                                       proof_json.as_ptr(),
                                       schemas_json.as_ptr(),
                                       credential_defs_json.as_ptr(),
                                       rev_reg_defs_json.as_ptr(),
                                       rev_regs_json.as_ptr(),
                                       Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn libindy_create_and_store_credential_def(issuer_did: &str,
                                               schema_json: &str,
                                               tag: &str,
                                               sig_type: Option<SigTypes>,
                                               config_json: &str)  -> Result<(String, String), u32>{

    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR_STR::new()?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let schema_json = CString::new(schema_json).map_err(map_string_error)?;
    let tag = CString::new(tag).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    let config_json = CString::new(config_json).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_issuer_create_and_store_credential_def(rtn_obj.command_handle,
                                                   wallet_handle,
                                                   i_did.as_ptr(),
                                                   schema_json.as_ptr(),
                                                   tag.as_ptr(),
                                                   s_type.as_ptr(),
                                                   config_json.as_ptr(),
                                                   Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

pub fn libindy_issuer_create_credential_offer(cred_def_id: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(LIBINDY_CRED_OFFER.to_string());
    }
    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    let cred_def_id = CString::new(cred_def_id).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_issuer_create_credential_offer(rtn_obj.command_handle,
                                                wallet_handle,
                                                cred_def_id.as_ptr(),
                                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_issuer_create_credential(cred_offer_json: &str,
                                        cred_req_json: &str,
                                        cred_values_json: &str,
                                        rev_reg_id: Option<&str>,
                                        blob_storage_reader_handle: Option<i32>)
    -> Result<(String, String, String), u32>{
    let rtn_obj = Return_I32_STR_STR_STR::new()?;
    let wallet_handle = get_wallet_handle();
    let cred_offer_json = CString::new(cred_offer_json ).map_err(map_string_error)?;
    let cred_req_json = CString::new(cred_req_json ).map_err(map_string_error)?;
    let cred_values_json = CString::new(cred_values_json ).map_err(map_string_error)?;
    let rev_reg_id_str = CString::new(rev_reg_id.unwrap_or_default() ).map_err(map_string_error)?;
    let blob_storage_reader_handle = blob_storage_reader_handle.unwrap_or(-1);

    unsafe {
        indy_function_eval(
            indy_issuer_create_credential(rtn_obj.command_handle,
                                          wallet_handle,
                                          cred_offer_json.as_ptr(),
                                          cred_req_json.as_ptr(),
                                          cred_values_json.as_ptr(),
                                          if rev_reg_id.is_some() { rev_reg_id_str.as_ptr() } else { null() },
                                          blob_storage_reader_handle,
                                          Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2, opt_str3) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    //Todo: when we do revocation, we will need to check the two last strings. Currently, None is returned if no revocation info was provided
    //Not sure if this is sufficient
    let str2 = if rev_reg_id.is_some() { check_str(opt_str2)?} else {String::new()};
    let str3 = if rev_reg_id.is_some() { check_str(opt_str3)?} else {String::new()};
    Ok((str1, str2, str3))
}

pub fn libindy_prover_create_proof(proof_req_json: &str,
                                   requested_credentials_json: &str,
                                   master_secret_id: &str,
                                   schemas_json: &str,
                                   credential_defs_json: &str,
                                   revoc_states_json: Option<&str>) -> Result<String, u32> {
    let rtn_obj = Return_I32_STR::new()?;

    let wallet_handle = get_wallet_handle();
    let proof_req_json = CString::new(proof_req_json).map_err(map_string_error)?;
    let requested_credentials_json = CString::new(requested_credentials_json).map_err(map_string_error)?;
    let schemas_json = CString::new(schemas_json).map_err(map_string_error)?;
    let master_secret_name = CString::new(master_secret_id).map_err(map_string_error)?;
    let credential_defs_json = CString::new(credential_defs_json).map_err(map_string_error)?;
    let revoc_states_json = match revoc_states_json {
        Some(s) => CString::new(s).map_err(map_string_error)?,
        None => CString::new("{}").map_err(map_string_error)?,
    };

    unsafe {
        indy_function_eval(
            indy_prover_create_proof(rtn_obj.command_handle,
                                     wallet_handle,
                                     proof_req_json.as_ptr(),
                                     requested_credentials_json.as_ptr(),
                                     master_secret_name.as_ptr(),
                                     schemas_json.as_ptr(),
                                     credential_defs_json.as_ptr(),
                                     revoc_states_json.as_ptr(),
                                     Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_prover_get_credentials_for_proof_req(proof_req: &str) -> Result<String, u32> {

    let rtn_obj = Return_I32_STR::new()?;

    let wallet_handle = get_wallet_handle();
    let proof_req = CString::new(proof_req).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_get_credentials_for_proof_req(rtn_obj.command_handle,
                                                      wallet_handle,
                                                      proof_req.as_ptr(),
                                                      Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_medium()).and_then(check_str)
}

pub fn libindy_prover_create_credential_req(prover_did: &str,
                                            credential_offer_json: &str,
                                            credential_def_json: &str) -> Result<(String, String), u32>
{
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::CREDENTIAL_REQ_STRING.to_owned(), String::new())); }

    let rtn_obj = Return_I32_STR_STR::new()?;

    let wallet_handle = get_wallet_handle();
    let prover_did = CString::new(prover_did).map_err(map_string_error)?;
    let credential_offer_json = CString::new(credential_offer_json).map_err(map_string_error)?;
    let credential_def_json = CString::new(credential_def_json).map_err(map_string_error)?;
    let master_secret_name = CString::new(settings::DEFAULT_LINK_SECRET_ALIAS.to_string()).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_create_credential_req(rtn_obj.command_handle,
                                                   wallet_handle,
                                                   prover_did.as_ptr(),
                                                   credential_offer_json.as_ptr(),
                                                   credential_def_json.as_ptr(),
                                                   master_secret_name.as_ptr(),
                                                   Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

pub fn libindy_prover_store_credential(cred_id: Option<&str>,
                                       cred_req_meta: &str,
                                       cred_json: &str,
                                       cred_def_json: &str,
                                       rev_reg_def_json: Option<&str>) -> Result<String, u32>
{
    if settings::test_indy_mode_enabled() { return Ok("cred_id".to_string()); }

    let rtn_obj = Return_I32_STR::new()?;

    let wallet_handle = get_wallet_handle();
    let cred_id_str = CString::new(cred_id.unwrap_or_default() ).map_err(map_string_error)?;
    let cred_req_meta = CString::new(cred_req_meta).map_err(map_string_error)?;
    let cred_json = CString::new(cred_json).map_err(map_string_error)?;
    let cred_def_json = CString::new(cred_def_json).map_err(map_string_error)?;
    let rev_reg_def_str = CString::new(rev_reg_def_json.unwrap_or_default() ).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_store_credential(
                rtn_obj.command_handle,
                wallet_handle,
                if cred_id.is_some() { cred_id_str.as_ptr() } else { null() },
                cred_req_meta.as_ptr(),
                cred_json.as_ptr(),
                cred_def_json.as_ptr(),
                if rev_reg_def_json.is_some() { rev_reg_def_str.as_ptr() } else { null() },
                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

/*
pub fn libindy_prover_store_credential_offer(wallet_handle: i32,
                                             credential_offer_json: &str) -> Result<(), u32>
{
    if settings::test_indy_mode_enabled() { return Ok(()); }

    let rtn_obj = Return_I32::new()?;

    let credential_offer_json = CString::new(credential_offer_json).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_store_credential_offer(rtn_obj.command_handle,
                                          wallet_handle,
                                          credential_offer_json.as_ptr(),
                                          Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_medium())
}
*/

pub fn libindy_prover_create_master_secret(master_secret_id: &str) -> Result<String, u32>
{
    if settings::test_indy_mode_enabled() { return Ok(settings::DEFAULT_LINK_SECRET_ALIAS.to_string()); }

    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;

    let master_secret_id = CString::new(master_secret_id).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_create_master_secret(rtn_obj.command_handle,
                                             wallet_handle,
                                             master_secret_id.as_ptr(),
                                             Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_issuer_create_schema(issuer_did: &str,
                                    name: &str,
                                    version: &str,
                                    attrs: &str) -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let issuer_did = CString::new(issuer_did).map_err(map_string_error)?;
    let name = CString::new(name).map_err(map_string_error)?;
    let version = CString::new(version).map_err(map_string_error)?;
    let attrs = CString::new(attrs).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_issuer_create_schema(rtn_obj.command_handle,
                                      issuer_did.as_ptr(),
                                      name.as_ptr(),
                                      version.as_ptr(),
                                      attrs.as_ptr(),
                                     Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    extern crate serde_json;
    extern crate rand;
    use rand::Rng;
    use settings;
    use utils::constants::*;

    pub fn create_schema() -> (String, String) {
        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}", rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        libindy_issuer_create_schema(&institution_did, &schema_name, &schema_version, &data).unwrap()
    }

    pub fn create_schema_req(schema_json: &str) -> String {
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        ::utils::libindy::ledger::libindy_build_schema_request(&institution_did, schema_json).unwrap()
    }

    pub fn write_schema(request: &str) {
        let (payment_info, response) = ::utils::libindy::payments::pay_for_txn(&request, SCHEMA_TXN_TYPE).unwrap();
    }

    pub fn create_and_write_test_schema() -> (String, String) {
        let (schema_id, schema_json) = create_schema();
        let req = create_schema_req(&schema_json);
        write_schema(&req);
        (schema_id, schema_json)
    }

    pub fn create_and_store_credential_def() -> (String, String, String, String) {
        /* create schema */
        let (schema_id, schema_json) = create_and_write_test_schema();

        let name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        /* create cred-def */
        let handle = ::credential_def::create_new_credentialdef("1".to_string(),
                                                                name,
                                                                institution_did.clone(),
                                                                schema_id.clone(),
                                                                "tag1".to_string(),
                                                                r#"{"support_revocation":false}"#.to_string()).unwrap();

        let cred_def_id = ::credential_def::get_cred_def_id(handle).unwrap();
        let (_, cred_def_json) = ::credential_def::retrieve_credential_def(&cred_def_id).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json)
    }

    pub fn create_credential_offer() -> (String, String, String, String, String) {
        let (schema_id, schema_json, cred_def_id, cred_def_json) = create_and_store_credential_def();

        let offer = ::utils::libindy::anoncreds::libindy_issuer_create_credential_offer(&cred_def_id).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer)
    }

    pub fn create_credential_req() -> (String, String, String, String, String, String, String) {
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer) = create_credential_offer();
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (req, req_meta) = ::utils::libindy::anoncreds::libindy_prover_create_credential_req(&institution_did, &offer, &cred_def_json).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta)
    }

    pub fn create_and_store_credential() -> (String, String, String, String, String, String, String, String) {

        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta) = create_credential_req();

        /* create cred */
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let encoded_attributes = ::issuer_credential::encode_attributes(&credential_data).unwrap();
        let (cred, _, _) = ::utils::libindy::anoncreds::libindy_issuer_create_credential(&offer, &req, &encoded_attributes, None, None).unwrap();
        /* store cred */
        let cred_id = ::utils::libindy::anoncreds::libindy_prover_store_credential(None, &req_meta, &cred, &cred_def_json, None).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id)
    }

    pub fn create_proof() -> (String, String, String, String) {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id) = create_and_store_credential();

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "self_attest_3": json!({
                   "name":"self_attest",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();

        let requested_credentials_json = json!({
              "self_attested_attributes":{
                 "self_attest_3": "my_self_attested_val"
              },
              "requested_attributes":{
                 "address1_1": {"cred_id": cred_id, "revealed": true},
                 "zip_2": {"cred_id": cred_id, "revealed": true}
                },
              "requested_predicates":{}
        }).to_string();

        let schema_json: serde_json::Value = serde_json::from_str(&schema_json).unwrap();
        let schemas = json!({
            schema_id: schema_json,
        }).to_string();

        let cred_def_json: serde_json::Value = serde_json::from_str(&cred_def_json).unwrap();
        let cred_defs = json!({
            cred_def_id: cred_def_json,
        }).to_string();

       libindy_prover_get_credentials_for_proof_req(&proof_req).unwrap();

        let proof = libindy_prover_create_proof(
            &proof_req,
            &requested_credentials_json,
            "main",
            &schemas,
            &cred_defs,
            None).unwrap();
        (schemas, cred_defs, proof_req, proof)
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_prover_verify_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_verify_proof";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);

        let (schemas, cred_defs, proof_req, proof) = create_proof();

        let result = libindy_verifier_verify_proof(
            &proof_req,
            &proof,
            &schemas,
            &cred_defs,
            "{}",
            "{}",
        );

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
        assert!(result.is_ok());
        let proof_validation = result.unwrap();
        assert!(proof_validation, true);
    }
}
