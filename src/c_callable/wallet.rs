extern crate libc;

use self::libc::{c_char, c_uchar};

/// Creates and saves in secured Wallet identity that can be used to
/// issue and verify Identity Ledger transaction.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// identity_json: Identity information as json. For current moment it is NYM transaction
///   data with optional information for creation of sign key (DID, Verkey, Role, Alias, Seed and etc...).
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// String identifier of this Identity.
///
/// #Errors
/// No method specific errors.
/// See `SovrinError` docs for common errors description.
#[no_mangle]
pub  extern fn wallet_sovrin_create_identity(client_id: i32, command_id: i32,
                                             identity_json: *const c_char,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           identity_id: *const c_char)) {
    unimplemented!();
}

/// Returns public information for Identity stored in secured Wallet.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// identity_id: Id of Identity stored in secured Wallet.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Public Identity information as json (DID, Verkey, Role, Alias and etc...).
///
/// #Errors
/// No method specific errors.
/// See `SovrinError` docs for common errors description.
#[no_mangle]
pub  extern fn wallet_sovrin_get_identity(client_id: i32, command_id: i32,
                                          identity_id: *const c_char,
                                          cb: extern fn(xcommand_id: i32, err: i32,
                                                        identity_json: *const c_char)) {
    unimplemented!();
}

/// Returns list of ids for Identities stored in secured Wallet.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// List of stored identity ids.
///
/// #Errors
/// No method specific errors.
/// See `SovrinError` docs for common errors description.
#[no_mangle]
pub  extern fn wallet_sovrin_get_identities(client_id: i32, command_id: i32,
                                            cb: extern fn(xcommand_id: i32, err: i32,
                                                          identity_ids: [*const c_char])) {
    unimplemented!();
}

/// Creates all necessary keys and objects depends on received schema and return schema_id.
///
/// #Params
/// client_id: id of sovrin client instance.
/// command_id: command id to map of callback to user context.
/// schema_json: Schema as a json. Includes name, version, attributes, keys, accumulator and etc.
///     Every empty field in the schema will be filled with the right value.
///     For example: if schema have an empty public key value, function will generate it. If it's
///         not necessary, value for public key field should be None.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Returns id of schema.
///
/// #Errors
/// No method specific errors.
/// See `AnoncredsError` docs for common errors description.
#[no_mangle]
pub extern fn wallet_anoncreds_create_schema(client_id: i32, command_id: i32,
                                             schema_json: *const c_char,
                                             cb: extern fn(xcommand_id: i32, err: i32,
                                                           schema_id: *const c_char)) {
    unimplemented!();
}