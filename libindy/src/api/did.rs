extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::did::DidCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;
use std::ptr;


/// Creates keys (signing and encryption keys) for a new
/// DID (owned by the caller of the library).
/// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
/// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
/// and encrypt transactions.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// did_json: Identity information as json. Example:
/// {
///     "did": string, (optional;
///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
///             if not provided and cid is true then the full verkey will be used as a new DID;
///             if provided, then keys will be replaced - key rotation use case)
///     "seed": string, (optional; if not provide then a random one will be created)
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
///     "cid": bool, (optional; if not set then false is used;)
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///   did: DID generated and stored in the wallet
///   verkey: The DIDs verification key
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_create_and_store_my_did(command_handle: i32,
                                            wallet_handle: i32,
                                            did_json: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 did: *const c_char,
                                                                 verkey: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::CreateAndStoreMyDid(
            wallet_handle,
            did_json,
            Box::new(move |result| {
                let (err, did, verkey) = result_to_err_code_2!(result, String::new(), String::new());
                let did = CStringUtils::string_to_cstring(did);
                let verkey = CStringUtils::string_to_cstring(verkey);
                cb(command_handle, err, did.as_ptr(), verkey.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Generated temporary keys (signing and encryption keys) for an existing
/// DID (owned by the caller of the library).
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// identity_json: Identity information as json. Example:
/// {
///     "seed": string, (optional; if not provide then a random one will be created)
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///   verkey: The DIDs verification key
///
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_replace_keys_start(command_handle: i32,
                                       wallet_handle: i32,
                                       did: *const c_char,
                                       identity_json: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                            verkey: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(identity_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::ReplaceKeysStart(
            wallet_handle,
            identity_json,
            did,
            Box::new(move |result| {
                let (err, verkey) = result_to_err_code_1!(result, String::new());
                let verkey = CStringUtils::string_to_cstring(verkey);
                cb(command_handle, err, verkey.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Apply temporary keys as main for an existing DID (owned by the caller of the library).
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// did: DID stored in the wallet
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_replace_keys_apply(command_handle: i32,
                                       wallet_handle: i32,
                                       did: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::ReplaceKeysApply(
            wallet_handle,
            did,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Saves their DID for a pairwise connection in a secured Wallet,
/// so that it can be used to verify transaction.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// identity_json: Identity information as json. Example:
///     {
///        "did": string, (required)
///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_store_their_did(command_handle: i32,
                                    wallet_handle: i32,
                                    identity_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(identity_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::StoreTheirDid(
            wallet_handle,
            identity_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Returns ver key (key id) for the given DID.
///
/// "indy_key_for_did" call follow the idea that we resolve information about their DID from
/// the ledger with cache in the local wallet. The "indy_open_wallet" call has freshness parameter
/// that is used for checking the freshness of cached pool value.
///
/// Note if you don't want to resolve their DID info from the ledger you can use
/// "indy_key_for_local_did" call instead that will look only to the local wallet and skip
/// freshness checking.
///
/// Note that "indy_create_and_store_my_did" makes similar wallet record as "indy_create_key".
/// As result we can use returned ver key in all generic crypto and messaging functions.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// pool_handle:   Pool handle (created by open_pool).
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to resolve key.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
/// - key - The DIDs ver key (key id).
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_key_for_did(command_handle: i32,
                               pool_handle: i32,
                               wallet_handle: i32,
                               did: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32,
                                                    err: ErrorCode,
                                                    key: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::KeyForDid(
            pool_handle,
            wallet_handle,
            did,
            Box::new(move |result| {
                let (err, key) = result_to_err_code_1!(result, String::new());
                let key = CStringUtils::string_to_cstring(key);
                cb(command_handle, err, key.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Returns ver key (key id) for the given DID.
///
/// "indy_key_for_local_did" call looks data stored in the local wallet only and skips freshness
/// checking.
///
/// Note if you want to get fresh data from the ledger you can use "indy_key_for_did" call
/// instead.
///
/// Note that "indy_create_and_store_my_did" makes similar wallet record as "indy_create_key".
/// As result we can use returned ver key in all generic crypto and messaging functions.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to resolve key.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
/// - key - The DIDs ver key (key id).
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_key_for_local_did(command_handle: i32,
                                     wallet_handle: i32,
                                     did: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32,
                                                          err: ErrorCode,
                                                          key: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::KeyForLocalDid(
            wallet_handle,
            did,
            Box::new(move |result| {
                let (err, key) = result_to_err_code_1!(result, String::new());
                let key = CStringUtils::string_to_cstring(key);
                cb(command_handle, err, key.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Set/replaces endpoint information for the given DID.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to resolve endpoint.
/// address -  The DIDs endpoint address.
/// transport_key - The DIDs transport key (ver key, key id).
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_set_endpoint_for_did(command_handle: i32,
                                        wallet_handle: i32,
                                        did: *const c_char,
                                        address: *const c_char,
                                        transport_key: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(address, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(transport_key, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::SetEndpointForDid(
            wallet_handle,
            did,
            address,
            transport_key,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Returns endpoint information for the given DID.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to resolve endpoint.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
/// - endpoint - The DIDs endpoint.
/// - transport_vk - The DIDs transport key (ver key, key id).
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_get_endpoint_for_did(command_handle: i32,
                                        wallet_handle: i32,
                                        pool_handle: i32,
                                        did: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             address: *const c_char,
                                                             transport_vk: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::GetEndpointForDid(
            wallet_handle,
            pool_handle,
            did,
            Box::new(move |result| {
                let (err, address, transport_vk) = result_to_err_code_2!(result, String::new(), None);
                let address = CStringUtils::string_to_cstring(address);
                let transport_vk = transport_vk.map(CStringUtils::string_to_cstring);
                cb(command_handle, err, address.as_ptr(),
                   transport_vk.as_ref().map(|vk| vk.as_ptr()).unwrap_or(ptr::null()));
            })
        )));

    result_to_err_code!(result)
}

/// Saves/replaces the meta information for the giving DID in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - the DID to store metadata.
/// metadata - the meta information that will be store with the DID.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_set_did_metadata(command_handle: i32,
                                    wallet_handle: i32,
                                    did: *const c_char,
                                    metadata: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str_empty_accepted!(metadata, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::SetDidMetadata(
            wallet_handle,
            did,
            metadata,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Retrieves the meta information for the giving DID in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to retrieve metadata.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
/// - metadata - The meta information stored with the DID; Can be null if no metadata was saved for this DID.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_get_did_metadata(command_handle: i32,
                                    wallet_handle: i32,
                                    did: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         metadata: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::GetDidMetadata(
            wallet_handle,
            did,
            Box::new(move |result| {
                let (err, metadata) = result_to_err_code_1!(result, String::new());
                let metadata = CStringUtils::string_to_cstring(metadata);
                cb(command_handle, err, metadata.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Retrieves the information about the giving DID in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// did - The DID to retrieve information.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///   did_with_meta:  {
///     "did": string - DID stored in the wallet,
///     "verkey": string - The DIDs transport key (ver key, key id),
///     "metadata": string - The meta information stored with the DID
///   }
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_get_my_did_with_meta(command_handle: i32,
                                        wallet_handle: i32,
                                        my_did: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                             did_with_meta: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::GetMyDidWithMeta(
            wallet_handle,
            my_did,
            Box::new(move |result| {
                let (err, did_with_meta) = result_to_err_code_1!(result, String::new());
                let did_with_meta = CStringUtils::string_to_cstring(did_with_meta);
                cb(command_handle, err, did_with_meta.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Retrieves the information about all DIDs stored in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///   dids:  [{
///     "did": string - DID stored in the wallet,
///     "verkey": string - The DIDs transport key (ver key, key id).,
///     "metadata": string - The meta information stored with the DID
///   }]
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub extern fn indy_list_my_dids_with_meta(command_handle: i32,
                                          wallet_handle: i32,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                               dids: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::ListMyDidsWithMeta(
            wallet_handle,
            Box::new(move |result| {
                let (err, dids) = result_to_err_code_1!(result, String::new());
                let dids = CStringUtils::string_to_cstring(dids);
                cb(command_handle, err, dids.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// did: DID.
/// full_verkey: The DIDs verification key,
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
///   verkey: The DIDs verification key in either abbreviated or full form
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_abbreviate_verkey(command_handle: i32,
                                      did: *const c_char,
                                      full_verkey: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           verkey: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(full_verkey, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Did(DidCommand::AbbreviateVerkey(
            did,
            full_verkey,
            Box::new(move |result| {
                let (err, verkey) = result_to_err_code_1!(result, String::new());
                let verkey = CStringUtils::string_to_cstring(verkey);
                cb(command_handle, err, verkey.as_ptr())
            })
        )));

    result_to_err_code!(result)
}