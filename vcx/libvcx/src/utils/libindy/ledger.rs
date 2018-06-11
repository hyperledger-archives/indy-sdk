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
                                                       request_json: *const c_char)> ) -> i32;


    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32,
                                                err: i32,
                                                request_result_json: *const c_char)> ) -> i32;


    fn indy_build_get_cred_def_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                            request_json: *const c_char)>) -> i32;

    fn indy_build_cred_def_request(command_handle: i32,
                                   submitter_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32,
                                                        err: i32,
                                                        request_result_json: *const c_char)>) -> i32;

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
    fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: i32,
                                                      request_json: *const c_char)> ) -> i32;

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
mod tests { /* see anoncreds for full testing of all these ledger actions */ }
