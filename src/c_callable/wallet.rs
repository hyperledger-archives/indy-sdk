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