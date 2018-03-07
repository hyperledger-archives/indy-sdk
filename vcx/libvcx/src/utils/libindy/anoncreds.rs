extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::error;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32_STR, Return_I32_BOOL, Return_I32_STR_STR };
use utils::libindy::SigTypes;
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use utils::timeout::TimeoutUtils;

extern {
    fn indy_issuer_create_and_store_claim_def(command_handle: i32,
                                              wallet_handle: i32,
                                              issuer_did: *const c_char,
                                              schema_json: *const c_char,
                                              signature_type: *const c_char,
                                              create_non_revoc: bool,
                                              cb: Option<extern fn(xcommand_handle: i32,
                                                                   err: i32,
                                                                   claim_def_json: *const c_char)>) -> i32;

    fn indy_verifier_verify_proof(command_handle: i32,
                                  proof_request_json: *const c_char,
                                  proof_json: *const c_char,
                                  schemas_json: *const c_char,
                                  claim_defs_jsons: *const c_char,
                                  revoc_regs_json: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                       valid: bool)>) -> i32;

    fn indy_issuer_create_claim(command_handle: i32,
                                wallet_handle: i32,
                                claim_req_json: *const c_char,
                                claim_json: *const c_char,
                                user_revoc_index: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                     revoc_reg_update_json: *const c_char, //TODO must be OPTIONAL
                                                     xclaim_json: *const c_char
                                )>)-> i32;
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(str) => Ok(str),
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}


pub fn libindy_verifier_verify_proof(proof_req_json: &str,
                                     proof_json: &str,
                                     schemas_json: &str,
                                     claim_defs_json: &str,
                                     revoc_regs_json: &str)  -> Result<bool, u32>{

    let rtn_obj = Return_I32_BOOL::new()?;
    let proof_req_json = CString::new(proof_req_json.to_string()).map_err(map_string_error)?;
    let proof_json = CString::new(proof_json.to_string()).map_err(map_string_error)?;
    let schemas_json = CString::new(schemas_json.to_string()).map_err(map_string_error)?;
    let claim_defs_json = CString::new(claim_defs_json.to_string()).map_err(map_string_error)?;
    let revoc_regs_json = CString::new(revoc_regs_json.to_string()).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
        indy_verifier_verify_proof(rtn_obj.command_handle,
                                   proof_req_json.as_ptr(),
                                   proof_json.as_ptr(),
                                   schemas_json.as_ptr(),
                                   claim_defs_json.as_ptr(),
                                   revoc_regs_json.as_ptr(),
                                   Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn libindy_create_and_store_claim_def(wallet_handle: i32,
                                          issuer_did: String,
                                          schema_json: String,
                                          sig_type: Option<SigTypes>,
                                          create_non_revoc: bool)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let schema_json = CString::new(schema_json).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
        indy_issuer_create_and_store_claim_def(rtn_obj.command_handle,
                                                   wallet_handle,
                                                   i_did.as_ptr(),
                                                   schema_json.as_ptr(),
                                                   s_type.as_ptr(),
                                                   create_non_revoc,
                                                   Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_issuer_create_claim(wallet_handle: i32,
                                   claim_req_json: String,
                                   claim_json: String,
                                   user_revoc_index: i32)  -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let claim_req_json = CString::new(claim_req_json).map_err(map_string_error)?;
    let claim_json = CString::new(claim_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_issuer_create_claim(rtn_obj.command_handle,
                                     wallet_handle,
                                     claim_req_json.as_ptr(),
                                     claim_json.as_ptr(),
                                     user_revoc_index,
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
    use settings;
    use utils::libindy::signus::SignusUtils;
    use serde_json;
    use issuer_claim::tests::{ create_standard_issuer_claim, util_put_claim_def_in_issuer_wallet };
    use utils::libindy::wallet::{ init_wallet, get_wallet_handle, delete_wallet};
    use utils::constants::{ INDY_PROOF_REQ_JSON,
                            INDY_PROOF_JSON,
                            INDY_SCHEMAS_JSON,
                            INDY_CLAIM_DEFS_JSON,
                            INDY_REVOC_REGS_JSON,
                            SCHEMAS_JSON,
    };

    #[test]
    fn simple_libindy_create_and_store_claim_def_test() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        init_wallet("wallet_simple").unwrap();
        let result = libindy_create_and_store_claim_def(get_wallet_handle(),
                                                        "GGBDg1j8bsKmr4h5T9XqYf".to_string(),
                                                        SCHEMAS_JSON.to_string(),
                                                        None,
                                                        false);
        delete_wallet("wallet_simple").unwrap();
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_issuer_create_claim_test() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let mut issuer_claim = create_standard_issuer_claim();
        issuer_claim.claim_id = String::from("id");
        let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e".to_owned();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &issuer_did);
        init_wallet("test_wallet").unwrap();
        let wallet_handle = get_wallet_handle();
        SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
        let mut claim_req = issuer_claim.claim_request.clone().unwrap();
        claim_req.issuer_did = issuer_did.to_owned();
        issuer_claim.claim_request = Some(claim_req.clone());
        let encoded_claim_data = issuer_claim.create_attributes_encodings().unwrap();
        util_put_claim_def_in_issuer_wallet(15, wallet_handle);
        let result = libindy_issuer_create_claim(get_wallet_handle(),
                                                 serde_json::to_string(&claim_req).unwrap(),
                                                 encoded_claim_data.to_string(),
                                                        -1);
        delete_wallet("test_wallet").unwrap();
        assert!(result.is_ok());
        let (str1, str2) = result.unwrap();
        println!("{}, {}", str1, str2);
    }

    #[test]
    fn simple_libindy_verifier_verify_proof() {
        settings::set_defaults();
        init_wallet("wallet_simple").unwrap();
        let result = libindy_verifier_verify_proof(INDY_PROOF_REQ_JSON,
                                                   INDY_PROOF_JSON,
                                                   INDY_SCHEMAS_JSON,
                                                   INDY_CLAIM_DEFS_JSON,
                                                   INDY_REVOC_REGS_JSON);
        delete_wallet("wallet_simple").unwrap();
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}