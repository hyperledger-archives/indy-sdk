extern crate libc;

use self::libc::c_char;
use api::Errorcode;
use api::CxsStatus;
use utils::cstring::CStringUtils;
use connection::build_connection;
use connection::connect;
use connection::to_string;
use connection::get_state;
use connection::release;

#[no_mangle]
pub extern fn cxs_init() -> Errorcode { Errorcode::Success }

/**
 * Schema object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_create(schema_data: *const c_char, schema_handle: *mut u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_commit(schema_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables)]
pub extern fn cxs_schema_get_data(schema_handle: u32, data: *mut c_char) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_get_sequence_no(schema_handle: u32, sequence_no: *mut u32) -> Errorcode { Errorcode::Success }


/**
 * claimdef object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_create(schema_handle: u32, claimdef_handle: *mut u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> Errorcode { Errorcode::Success }


/**
 * connection object
 */

#[no_mangle]
pub extern fn cxs_connection_create(recipient_info: *const c_char, connection_handle: *mut u32) -> Errorcode {
    check_useful_c_str!(recipient_info, Errorcode::Failure);

    if connection_handle.is_null() {return Errorcode::Failure}

    let handle = build_connection("Whatever.".to_owned());

    unsafe { *connection_handle = handle }

    Errorcode::Success
}

#[no_mangle]
pub extern fn cxs_connection_connect(connection_handle: u32) -> Errorcode {
    connect(connection_handle)
}

#[no_mangle]
pub extern fn cxs_connection_get_data(connection_handle: u32) -> *mut c_char {

    let json_string = to_string(connection_handle);
    let msg = CStringUtils::string_to_cstring(json_string);

    msg.into_raw()
}

#[no_mangle]
pub extern fn cxs_connection_get_state(connection_handle: u32, status: *mut u32) -> Errorcode {

    if status.is_null() {return Errorcode::Failure}

    let state = get_state(connection_handle);

    unsafe { *status = state }

    Errorcode::Success
}

#[no_mangle]
pub extern fn cxs_connection_release(connection_handle: u32) -> Errorcode {
    release(connection_handle)
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_list_state(status_array: *mut CxsStatus) -> Errorcode { Errorcode::Success }


/**
 * claim object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_create(claimdef_handle: u32, claim_data: *const c_char, claim_handle: *mut u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables)]
pub extern fn cxs_claim_set_connection(claim_handle: u32, connection_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send_offer(claim_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send(claim_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables)]
pub extern fn cxs_claim_terminate(claim_handle: u32, termination_type: u32, msg: *const c_char) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_list_state(status_array: *mut CxsStatus) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_state(claim_handle: u32, status: *mut c_char) -> Errorcode { Errorcode::Success }


/**
 * proof object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(proof_request_data: *mut c_char, proof_handle: *mut u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(proof_handle: u32, connection_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(proof_handle: u32) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> Errorcode { Errorcode::Success }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> Errorcode { Errorcode::Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_state(proof_handle: u32, status: *mut c_char) -> Errorcode { Errorcode::Success }
