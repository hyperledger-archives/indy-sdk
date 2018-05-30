extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use settings;
use utils::libindy::{
    indy_function_eval,
    return_types::{ Return_I32_STR_STR, Return_I32_STR },
    pool::get_pool_handle,
    wallet::get_wallet_handle,
    error_codes::{map_indy_error_code, map_string_error}
};
use utils::error;
use utils::timeout::TimeoutUtils;



extern {

    fn indy_build_get_txn_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  data: i32,
                                  cb: Option<extern fn(xcommand_handle: i32,
                                                       err: i32,
                                                       request_json: *const c_char)>
    ) -> i32;


    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32,
                                                err: i32,
                                                request_result_json: *const c_char)>
    ) -> i32;


    fn indy_build_get_cred_def_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                            request_json: *const c_char)>) -> i32;
//    fn indy_build_get_cred_def_request(command_handle: i32,
//                                    submitter_did: *const c_char,
//                                    xref: i32,
//                                    signature_type: *const c_char,
//                                    origin: *const c_char,
//                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                         request_json: *const c_char)>) -> i32;


    fn indy_build_cred_def_request(command_handle: i32,
                                   submitter_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32,
                                                        err: i32,
                                                        request_result_json: *const c_char)>) -> i32;
//    fn indy_build_cred_def_request(command_handle: i32,
//                                submitter_did: *const c_char,
//                                xref: i32,
//                                signature_type: *const c_char,
//                                data: *const c_char,
//                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                     request_result_json: *const c_char)>) -> i32;

    // Todo: Add to cred_def object
    fn indy_parse_get_cred_def_response(command_handle: i32,
                                        get_cred_def_response: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             cred_def_id: *const c_char,
                                                             cred_def_json: *const c_char)>) -> i32;
    pub fn indy_sign_and_submit_request(command_handle: i32,
                                        pool_handle: i32,
                                        wallet_handle: i32,
                                        submitter_did: *const c_char,
                                        request_json: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             request_result_json: *const c_char)>) -> i32;
    fn indy_build_get_schema_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                          request_json: *const c_char)>) -> i32;
//    pub fn indy_build_get_schema_request(command_handle: i32,
//                                         submitter_did: *const c_char,
//                                         dest: *const c_char,
//                                         data: *const c_char,
//                                         cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                              request_json: *const c_char)>) -> i32;
    fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: i32,
                                                      request_json: *const c_char)>
    ) -> i32;

    //Todo: Add to schema object
    fn indy_parse_get_schema_response(command_handle: i32,
                                      get_schema_response: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                           schema_id: *const c_char,
                                                           schema_json: *const c_char)>) -> i32;
}

pub fn libindy_sign_and_submit_request(issuer_did: &str, request_json: &str) -> Result<String, u32>
{
    if settings::test_indy_mode_enabled() { return Ok(r#"{"rc":"success"}"#.to_string()); }
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    let json = CString::new(request_json).map_err(map_string_error)?;
    let issuer_did = CString::new(issuer_did).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_sign_and_submit_request(rtn_obj.command_handle,
                                         pool_handle as i32,
                                         wallet_handle as i32,
                                         issuer_did.as_ptr(),
                                         json.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(x) => Ok(x),
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}


//Todo: take out pool_handle param
pub fn libindy_submit_request(request_json: &str) -> Result<String, u32>
{
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    let rtn_obj = Return_I32_STR::new()?;
    let request_json = CString::new(request_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_submit_request(rtn_obj.command_handle,
                                pool_handle,
                                request_json.as_ptr(),
                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_build_get_txn_request(submitter_did: &str, sequence_num: i32) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let did = CString::new(submitter_did).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_txn_request(rtn_obj.command_handle,
                                       did.as_ptr(),
                                       sequence_num,
                                       Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let did = CString::new(submitter_did).map_err(map_string_error)?;
    let data = CString::new(data).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_schema_request(rtn_obj.command_handle,
                                      did.as_ptr(),
                                      data.as_ptr(),
                                      Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> Result<String, u32> {
    let rtn_obj = Return_I32_STR::new()?;
    let sub_did = CString::new(submitter_did).map_err(map_string_error)?;
    let schema_id = CString::new(schema_id).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_schema_request(rtn_obj.command_handle,
                                         sub_did.as_ptr(),
                                         schema_id.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_parse_get_schema_response(get_schema_response: &str) -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let get_schema_response = CString::new(get_schema_response).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_parse_get_schema_response(rtn_obj.command_handle,
                                           get_schema_response.as_ptr(),
                                           Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

pub fn libindy_parse_get_cred_def_response(get_cred_def_response: &str) -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let get_cred_def_response = CString::new(get_cred_def_response).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_parse_get_cred_def_response(rtn_obj.command_handle,
                                           get_cred_def_response.as_ptr(),
                                           Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}
pub fn libindy_build_get_credential_def_txn(cred_def_id: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let institution_did  = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;
    let sub_did = CString::new(institution_did).map_err(map_string_error)?;
    let cred_def_id = CString::new(cred_def_id).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_cred_def_request(rtn_obj.command_handle,
                                         sub_did.as_ptr(),
                                         cred_def_id.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               credential_def_json: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let s_did = CString::new(submitter_did).map_err(map_string_error)?;
    let credential_def_json = CString::new(credential_def_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(

            indy_build_cred_def_request(rtn_obj.command_handle,
                                     s_did.as_ptr(),
                                     credential_def_json.as_ptr(),
                                     Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}


#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;

    #[allow(unused_imports)]
    use rand::Rng;
    use utils::constants::{SCHEMA_ID, CRED_DEF_ID};
    use utils::libindy::{
        SigTypes,
        wallet::{delete_wallet, init_wallet},
        anoncreds::{ libindy_issuer_create_schema, libindy_create_and_store_credential_def },

    };

    #[test]
    fn simple_libindy_build_get_txn_request_test() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let result = libindy_build_get_txn_request("GGBDg1j8bsKmr4h5T9XqYf", 15);
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_build_get_credential_def_txn_test() {
        settings::set_defaults();
        let result = libindy_build_get_credential_def_txn(CRED_DEF_ID);
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_libindy_build_get_schema_request() {
        let did = "GGBDg1j8bsKmr4h5T9XqYf";
        assert!(libindy_build_get_schema_request(did, SCHEMA_ID).is_ok())
    }

    #[test]
    fn test_schema_request_from_created_schema() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let (id, create_schema_json) = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            "schema_name",
            "1.0",
            schema_data).unwrap();

        let schema_request = libindy_build_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_schema_json);

        delete_wallet(wallet_name).unwrap();
        assert!(schema_request.is_ok());
        println!("{}", schema_request.unwrap());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_build_get_schema_and_parse_response() {
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_dev_env(wallet_name);

        let get_schema_req = libindy_build_get_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
           SCHEMA_ID).unwrap();
        println!("get_schema_req: {}", get_schema_req);

        let get_schema_response = libindy_submit_request(&get_schema_req).unwrap();
        println!("get_schema_response: {}", get_schema_response);

        ::utils::devsetup::cleanup_dev_env(wallet_name);

        let (id, schema_json) = libindy_parse_get_schema_response(&get_schema_response).unwrap();
        println!("schema_id: {}", id);
        println!("schema_json: {}", schema_json);

        assert_eq!(id, SCHEMA_ID);
    }

    #[test]
    fn test_cred_def_request_from_created_cred_def() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_cred_def_req";
        let schema_json = r#"{"ver":"1.0","id":"2hoqvcwupRTUNkXn6ArYzs:2:unique_schema_name:0.0.1","name":"unique_schema_name","version":"0.0.1","attrNames":["age","height","name","sex"],"seqNo":1699}"#;
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let (id, create_cred_def_json) = libindy_create_and_store_credential_def(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            schema_json,
            "tag_1",
            Some(SigTypes::CL),
            r#"{"support_revocation":false}"#
        ).unwrap();

        println!("id: \n{}", id);
        println!("create_cred_def: \n{}", create_cred_def_json);
        let cred_def_req = libindy_build_create_credential_def_txn(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_cred_def_json);

        delete_wallet(wallet_name).unwrap();
        assert!(cred_def_req.is_ok());
        println!("{}", cred_def_req.unwrap());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_schema_and_cred_def_on_ledger() {
        ::utils::logger::LoggerUtils::init_test_logging();
        let wallet_name = "create_schema_and_cred_def";
        ::utils::devsetup::setup_dev_env(wallet_name);

        //Create Schema-------------
        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}",rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());

        let (schema_id, create_schema_json) = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &schema_name,
            &schema_version,
            schema_data).unwrap();

        let schema_request = libindy_build_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_schema_json).unwrap();


        let schema_response = libindy_sign_and_submit_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &schema_request).unwrap();

        println!("schema_response: {}", schema_response);


        // Get Schema Json
        let get_schema_req = libindy_build_get_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &schema_id).unwrap();
        println!("get_schema_req: {}", get_schema_req);

        let get_schema_response = libindy_submit_request(&get_schema_req).unwrap();
        println!("get_schema_response: {}", get_schema_response);

        let (schema_id, schema_json) = libindy_parse_get_schema_response(&get_schema_response).unwrap();
        println!("schema_id: {}", schema_id);
        println!("schema_json: {}", schema_json);


        //Create CredDef ---------------
        let (cred_id, create_cred_def_json) = libindy_create_and_store_credential_def(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &schema_json,
            "tag_1",
            Some(SigTypes::CL),
            r#"{"support_revocation":false}"#
        ).unwrap();

        println!("cred_id: \n{}", cred_id);
        println!("create_cred_def: \n{}", create_cred_def_json);
        let cred_def_req = libindy_build_create_credential_def_txn(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_cred_def_json).unwrap();

        let submit_cred_def_response = libindy_sign_and_submit_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &cred_def_req,
        ).unwrap();
        println!("{}", submit_cred_def_response);


        ::utils::devsetup::cleanup_dev_env(wallet_name);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_build_get_cred_def_req_and_parse_response() {
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_dev_env(wallet_name);

        let get_cred_def_req = libindy_build_get_credential_def_txn(CRED_DEF_ID).unwrap();
        println!("get_cred_def_req: {}", get_cred_def_req);

        let get_cred_def_response = libindy_submit_request(&get_cred_def_req ).unwrap();
        println!("get_cred_def_response : {}", get_cred_def_response);

        let (id, cred_def_json) = libindy_parse_get_cred_def_response(&get_cred_def_response).unwrap();
        println!("cred_def_id: {}", id);
        println!("cred_def_json: {}", cred_def_json);

        ::utils::devsetup::cleanup_dev_env(wallet_name);
        assert_eq!(id, CRED_DEF_ID);
    }
}
