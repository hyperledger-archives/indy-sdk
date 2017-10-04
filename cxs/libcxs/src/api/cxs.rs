extern crate libc;

use self::libc::c_char;
use api::CxsStatus;
use utils::cstring::CStringUtils;
use utils::{pool, wallet};
use utils::error;
use connection::build_connection;
use connection::connect;
use connection::to_string;
use connection::get_state;
use connection::release;
use std::ffi::CString;

#[no_mangle]
<<<<<<< HEAD
pub extern fn cxs_init (pool_name:*const c_char,
                           config_name:*const c_char,
                           wallet_name:*const c_char,
                           wallet_type:*const c_char) -> u32 {
    check_useful_c_str!(pool_name,1002);
    check_useful_c_str!(config_name,1003);
    check_useful_c_str!(wallet_name,1004);
    check_useful_c_str!(wallet_type,1005);
    match pool::create_pool_config(&pool_name, &config_name) {
        0 => 0,
        x => return x,
    };

    match wallet::create_wallet(&pool_name, &wallet_name, &wallet_type) {
        0 => 0,
        x => return x,
    };

    return 0
}




/**
 * Schema object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_create(schema_data: *const c_char, schema_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_commit(schema_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_schema_get_data(schema_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_get_sequence_no(schema_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }


/**
 * claimdef object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_create(schema_handle: u32, claimdef_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }


/**
 * connection object
 */

#[no_mangle]
pub extern fn cxs_connection_create(recipient_info: *const c_char, connection_handle: *mut u32) -> u32 {
    check_useful_c_str!(recipient_info, error::UNKNOWN_ERROR.code_num);

    if connection_handle.is_null() {return error::UNKNOWN_ERROR.code_num}

    let handle = build_connection("Whatever.".to_owned());

    unsafe { *connection_handle = handle }

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_connect(connection_handle: u32) -> u32 {
    connect(connection_handle)
}

#[no_mangle]
pub extern fn cxs_connection_get_data(connection_handle: u32) -> *mut c_char {

    let json_string = to_string(connection_handle);
    let msg = CStringUtils::string_to_cstring(json_string);

    msg.into_raw()
}

#[no_mangle]
pub extern fn cxs_connection_get_state(connection_handle: u32, status: *mut u32) -> u32 {

    if status.is_null() {return error::UNKNOWN_ERROR.code_num}

    let state = get_state(connection_handle);

    unsafe { *status = state }

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_release(connection_handle: u32) -> u32 {
    release(connection_handle)
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }


/**
 * claim object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_create(claimdef_handle: u32, claim_data: *const c_char, claim_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_claim_set_connection(claim_handle: u32, connection_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send_offer(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_claim_terminate(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_state(claim_handle: u32, status: *mut c_char) -> u32 { error::SUCCESS.code_num }


/**
 * proof object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(proof_request_data: *mut c_char, proof_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(proof_handle: u32, connection_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(proof_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_state(proof_handle: u32, status: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let pool_name = CString::new("pool1").unwrap().into_raw();
        let config_name = CString::new("config1").unwrap().into_raw();
        let wallet_name = CString::new("wallet1").unwrap().into_raw();
        let wallet_type = CString::new("default").unwrap().into_raw();
        let result = cxs_init(pool_name, config_name, wallet_name, wallet_type);
        assert_eq!(result,0);
    }
}
