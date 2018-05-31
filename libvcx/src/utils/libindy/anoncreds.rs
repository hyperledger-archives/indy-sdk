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
                                            credential_def_json: &str,
                                            master_secret_id: Option<String>) -> Result<(String, String), u32>
{
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::CREDENTIAL_REQ_STRING.to_owned(), String::new())); }

    let rtn_obj = Return_I32_STR_STR::new()?;

    let wallet_handle = get_wallet_handle();
    let prover_did = CString::new(prover_did).map_err(map_string_error)?;
    let credential_offer_json = CString::new(credential_offer_json).map_err(map_string_error)?;
    let credential_def_json = CString::new(credential_def_json).map_err(map_string_error)?;
    let master_secret_name = CString::new(master_secret_id.unwrap_or(settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap())).map_err(map_string_error)?;

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
mod tests {
    use super::*;
    extern crate serde_json;
    use settings;
    use utils::libindy::wallet::{ init_wallet, delete_wallet, open_wallet};
    use utils::constants::*;

    fn setup_non_pool_tests(wallet_name: &str) {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        ::utils::devsetup::tests::set_institution_dev_config(wallet_name);
        open_wallet(wallet_name, None).unwrap();
    }

    fn cleanup_non_pool_tests(wallet_name: &str) {
        delete_wallet(wallet_name).unwrap();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn simple_libindy_create_credential_offer_test() {
        let wallet_name = "test_libindy_create_cred_offer";
        setup_non_pool_tests(wallet_name);

        let result = libindy_issuer_create_credential_offer(CRED_DEF_ID);

        cleanup_non_pool_tests(wallet_name);
        assert!(result.is_ok());
    }

    #[test]
    fn simple_libindy_issuer_create_credential_offer_req_and_cred() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_libindy_create_credential";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let libindy_offer = libindy_issuer_create_credential_offer(CRED_DEF_ID).unwrap();

        let (libindy_cred_req, cred_req_meta) = libindy_prover_create_credential_req(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &libindy_offer,
            CRED_DEF_JSON,
            None).unwrap();

        let encoded_cred_data = r#"{"age":["111","111"],"height":["4'11","25730877424947290072821310314181366395232879096832067784637233452620527354832"],"name":["Bob","93006290325627508022776103386395994712401809437930957652111221015872244345185"],"sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"]}"#;

        let result = libindy_issuer_create_credential(
            &libindy_offer,
            &libindy_cred_req,
            encoded_cred_data,
            None,
            None);
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let (str1, str2, str3) = result.unwrap();
    }

    #[test]
    fn simple_libindy_create_schema() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let result = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            "schema_nam",
            "2.2.2",
            schema_data);
        delete_wallet("test_create_schema").unwrap();
        assert!(result.is_ok());
        let (id, schema) = result.unwrap();
    }

    #[test]
    fn simple_libindy_create_cred_def() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_cred_def";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let result = libindy_create_and_store_credential_def(
            "AQ2EZRY9JQ4ssjmZPL5MiU",
            SCHEMA_JSON,
           "tag_1",
            Some(SigTypes::CL),
            r#"{"support_revocation":false}"#
        );
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let (id, cred) = result.unwrap();
    }

    #[test]
    //Todo: Get working. Works individually but fails during cargo test
    fn simple_libindy_create_master_secret() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_ms";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let rc = libindy_prover_create_master_secret("random_ms");
        delete_wallet(wallet_name).unwrap();
        assert!(rc.is_ok());
    }

    #[test]
    fn simple_libindy_create_cred_req() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_cred_req";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let result = libindy_prover_create_credential_req(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            CRED_OFFER,
            CRED_DEF_JSON,
            None,
        );
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let (cred_req, cred_req_meta) = result.unwrap();
    }

    #[test]
    fn simple_libindy_prover_store_cred() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_store_cred2";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let libindy_offer = libindy_issuer_create_credential_offer(CRED_DEF_ID).unwrap();

        let (libindy_cred_req, cred_req_meta) = libindy_prover_create_credential_req(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &libindy_offer,
            CRED_DEF_JSON,
            None,
            ).unwrap();

        let encoded_cred_data = r#"{"age":["111","111"],"height":["4'11","25730877424947290072821310314181366395232879096832067784637233452620527354832"],"name":["Bob","93006290325627508022776103386395994712401809437930957652111221015872244345185"],"sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"]}"#;

        let (cred, _, _) = libindy_issuer_create_credential(
            &libindy_offer,
            &libindy_cred_req,
            encoded_cred_data,
            None,
            None).unwrap();

        let result = libindy_prover_store_credential(
            None,
            &cred_req_meta,
            &cred,
            CRED_DEF_JSON,
            None);

        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let cred_id = result.unwrap();
    }

    #[test]
    fn simple_libindy_prover_get_creds_from_req() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_get_creds_from_req";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "height_1": json!({
                   "name":"height",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        let result = libindy_prover_get_credentials_for_proof_req(&proof_req);

        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let creds = result.unwrap();
    }

    #[test]
    fn test_prover_create_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_prover_create_proof";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "height_1": json!({
                   "name":"height",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
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
                 "height_1": {"cred_id": LICENCE_CRED_ID, "revealed": true},
                 "zip_2": {"cred_id": ADDRESS_CRED_ID, "revealed": true}
                },
              "requested_predicates":{}
        }).to_string();

        let schema_json: serde_json::Value = serde_json::from_str(SCHEMA_JSON).unwrap();
        let address_schema_json: serde_json::Value = serde_json::from_str(ADDRESS_SCHEMA_JSON).unwrap();
        let schemas = json!({
            SCHEMA_ID: schema_json,
            ADDRESS_SCHEMA_ID: address_schema_json,
        }).to_string();

        let cred_def_json: serde_json::Value = serde_json::from_str(CRED_DEF_JSON).unwrap();
        let address_cred_def_json: serde_json::Value = serde_json::from_str(ADDRESS_CRED_DEF_JSON).unwrap();
        let cred_defs = json!({
            ADDRESS_CRED_DEF_ID: address_cred_def_json,
            CRED_DEF_ID: cred_def_json,
        }).to_string();

        let result = libindy_prover_create_proof(
            &proof_req,
            &requested_credentials_json,
            "main",
            &schemas,
            &cred_defs,
            None);
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let proof = result.unwrap();
    }

    #[test]
    fn test_prover_verify_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_verify_proof";
        ::utils::devsetup::tests::setup_wallet(wallet_name);
        open_wallet(wallet_name, None).unwrap();

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "height_1": json!({
                   "name":"height",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": "2hoqvcwupRTUNkXn6ArYzs" })]
               }),
               "self_attest_3": json!({
                   "name":"self_attest",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();

        let schemas = json!({
            SCHEMA_ID: serde_json::from_str::<serde_json::Value>(SCHEMA_JSON).unwrap(),
            ADDRESS_SCHEMA_ID: serde_json::from_str::<serde_json::Value>(ADDRESS_SCHEMA_JSON).unwrap(),
        }).to_string();

        let cred_defs = json!({
            CRED_DEF_ID: serde_json::from_str::<serde_json::Value>(CRED_DEF_JSON).unwrap(),
            ADDRESS_CRED_DEF_ID: serde_json::from_str::<serde_json::Value>(ADDRESS_CRED_DEF_JSON).unwrap(),
        }).to_string();


        let result = libindy_verifier_verify_proof(
            &proof_req,
            PROOF_JSON,
            &schemas,
            &cred_defs,
            "{}",
            "{}",
        );

        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let proof_validation = result.unwrap();
        assert!(proof_validation, true);
    }
}
