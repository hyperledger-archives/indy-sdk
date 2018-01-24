extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::error;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32_STR, Return_I32_BOOL };
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

#[cfg(test)]
mod tests {
    use super::*;
    use settings;
    use utils::wallet::{ init_wallet, get_wallet_handle, delete_wallet };
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