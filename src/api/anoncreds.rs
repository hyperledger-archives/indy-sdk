extern crate libc;

use self::libc::{c_char, c_uchar};

/// Issues claim.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// schema_id: id of schema.
/// attributes:  all names of attributes as a byte array.
/// claim_request:
///     A claim request containing prover ID and prover-generated values as a byte array.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Returns Claim.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_create_claim(client_id: i32, command_id: i32,
                                     schema_id: *const c_char, attributes: *const c_char,
                                     claim_request: *const c_char,
                                     cb: extern fn(xcommand_id: i32, err: i32, claim: *const c_char)) {
    unimplemented!();
}

/// Creates claim request.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// schema_id: id of schema.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Claim request.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_create_claim_request(client_id: i32, command_id: i32,
                                             schema_id: *const c_char,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           claim_req: *const c_char)) {
    unimplemented!();
}

/// Creates proof.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// proof_input: description of a proof to be presented
///     (revealed attributes, predicates, timestamps for non-revocation).
///     For example: {
///         names:['name', 'height'],
///         predicates: [{predicate: 'gt', value: 18, attr: age}]
///     }
/// nonce: verifier's nonce.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Proof.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_create_proof(client_id: i32, command_id: i32,
                                     proof_input: *const c_char,
                                     nonce: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32,
                                                   proof: *const c_char, attrs: *const c_char)) {
    unimplemented!();
}

/// Verifies proof.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// proof_input: description of a proof to be presented
///     (revealed attributes, predicates, timestamps for non-revocation).
///     For example: {
///         names:['name', 'height'],
///         predicates: [{predicate: 'gt', value: 18, attr: age}]
///     }
/// proof: a proof.
/// revealed_attributes:
///     values of revealed attributes (initial values, non-encoded).
/// nonce: verifier's nonce.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// True if verified successfully and false otherwise.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn anoncreds_verify_proof(client_id: i32, command_id: i32,
                                     proof_input: *const c_char,
                                     proof: *const c_uchar,
                                     revealed_attributes: *const c_uchar,
                                     nonce: *const c_uchar,
                                     cb: extern fn(xcommand_id: i32, err: i32)) {
    unimplemented!();
}