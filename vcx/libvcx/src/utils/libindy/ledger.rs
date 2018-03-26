extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::libindy::indy_function_eval;
use utils::libindy::SigTypes;
use utils::libindy::return_types::Return_I32_STR;
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
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

    fn indy_build_schema_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  data: *const c_char,
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

    fn indy_build_get_claim_def_txn(command_handle: i32,
                                    submitter_did: *const c_char,
                                    xref: i32,
                                    signature_type: *const c_char,
                                    origin: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         request_json: *const c_char)>) -> i32;

    fn indy_build_claim_def_txn(command_handle: i32,
                                submitter_did: *const c_char,
                                xref: i32,
                                signature_type: *const c_char,
                                data: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                     request_result_json: *const c_char)>) -> i32;

    pub fn indy_sign_and_submit_request(command_handle: i32,
                                        pool_handle: i32,
                                        wallet_handle: i32,
                                        submitter_did: *const c_char,
                                        request_json: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             request_result_json: *const c_char)>) -> i32;
}

pub fn libindy_sign_and_submit_request(pool_handle: i32,
                                       wallet_handle: i32,
                                       issuer_did: &str,
                                       request_json: &str) -> Result<String, u32>
{
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



pub fn libindy_submit_request(pool_handle: i32, request_json: &str) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let json = CString::new(request_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_submit_request(rtn_obj.command_handle,
                                pool_handle as i32,
                                json.as_ptr(),
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

pub fn libindy_build_get_claim_def_txn(submitter_did: &str,
                                       schema_sequence_num: i32,
                                       sig_type: Option<SigTypes>,
                                       issuer_did: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let sub_did = CString::new(submitter_did).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_claim_def_txn(rtn_obj.command_handle,
                                         sub_did.as_ptr(),
                                         schema_sequence_num,
                                         s_type.as_ptr(),
                                         i_did.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_create_claim_def_txn(submitter_did: &str,
                                          schema_sequence_num: i32,
                                          sig_type: Option<SigTypes>,
                                          claim_def_json: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let s_did = CString::new(submitter_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    let claim_def_json = CString::new(claim_def_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_claim_def_txn(rtn_obj.command_handle,
                                     s_did.as_ptr(),
                                     schema_sequence_num,
                                     s_type.as_ptr(),
                                     claim_def_json.as_ptr(),
                                     Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{CLAIM_DEF_DATA};
    #[test]
    fn simple_libindy_build_get_txn_request_test() {
        let result = libindy_build_get_txn_request("GGBDg1j8bsKmr4h5T9XqYf",15);
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }

    #[test]
    fn simple_libindy_build_get_claim_def_txn_test() {
        let result = libindy_build_get_claim_def_txn("GGBDg1j8bsKmr4h5T9XqYf",
                                                     15,
                                                     None,
                                                     "GGBDg1j8bsKmr4h5T9XqYf");
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }

    #[test]
    fn simple_libindy_build_create_txn_request_test() {
        let result = libindy_build_create_claim_def_txn("GGBDg1j8bsKmr4h5T9XqYf",15, None, CLAIM_DEF_DATA);
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }

    #[test]
    fn simple_libindy_build_schema_request_test() {
        let request = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let result = libindy_build_schema_request("GGBDg1j8bsKmr4h5T9XqYf",request);
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }
}