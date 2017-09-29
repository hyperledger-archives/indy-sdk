extern crate libc;

use self::libc::c_char;
use api::Errorcode;
use api::Errorcode::Success;
use api::CxsStatus;

#[no_mangle]
pub extern fn cxs_init() -> Errorcode { Success }


/**
 * Schema object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_create(schema_data: *const c_char, schema_handle: *mut i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_commit(schema_handle: i32) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_schema_get_data(schema_handle: i32, data: *mut c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_get_sequence_no(schema_handle: i32, sequence_no: *mut i32) -> Errorcode { Success }


/**
 * claimdef object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_create(schema_handle: i32, claimdef_handle: *mut i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: i32, sequence_no: *mut i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: i32, data: *mut c_char) -> Errorcode { Success }


/**
 * connection object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_create(recipient_info: *const c_char, connection_handle: *mut i32) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_connection_connect(connection_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_get_data(connection_handle: i32, data: *mut c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_get_state(connection_handle: i32, status: *mut c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_list_state(status_array: *mut CxsStatus) -> Errorcode { Success }


/**
 * claim object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_create(claimdef_handle: i32, claim_data: *const c_char, claim_handle: *mut i32) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_claim_set_connection(claim_handle: i32, connection_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send_offer(claim_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_claim_request(claim_handle: i32, claim_request: *mut c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send(claim_handle: i32) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_claim_terminate(claim_handle: i32, termination_type: i32, msg: *const c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_list_state(status_array: *mut CxsStatus) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_state(claim_handle: i32, status: *mut c_char) -> Errorcode { Success }


/**
 * proof object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(proof_request_data: *mut c_char, proof_handle: *mut i32) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(proof_handle: i32, connection_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(proof_handle: i32) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: i32, response_data: *mut c_char) -> Errorcode { Success }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: i32, response_data: *const c_char) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> Errorcode { Success }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_state(proof_handle: i32, status: *mut c_char) -> Errorcode { Success }
