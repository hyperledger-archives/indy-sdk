extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::signus::SignusCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

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
///     "did": string, (optional; if not provided then the first 16 bit of the verkey will be used
///             as a new DID; if provided, then keys will be replaced - key rotation use case)
///     "seed": string, (optional; if not provide then a random one will be created)
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// DID, verkey (for verification of signature) and public_key (for decryption)
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_create_and_store_my_did(command_handle: i32,
                                              wallet_handle: i32,
                                              did_json: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                   did: *const c_char,
                                                                   verkey: *const c_char,
                                                                   pk: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::CreateAndStoreMyDid(
            wallet_handle,
            did_json,
            Box::new(move |result| {
                let (err, did, verkey, pk) = result_to_err_code_3!(result, String::new(), String::new(), String::new());
                let did = CStringUtils::string_to_cstring(did);
                let verkey = CStringUtils::string_to_cstring(verkey);
                let pk = CStringUtils::string_to_cstring(pk);
                cb(command_handle, err, did.as_ptr(), verkey.as_ptr(), pk.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Generated new keys (signing and encryption keys) for an existing
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
/// verkey (for verification of signature) and public_key (for decryption)
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_replace_keys(command_handle: i32,
                                   wallet_handle: i32,
                                   did: *const c_char,
                                   identity_json: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        verkey: *const c_char,
                                                        pk: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(identity_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::ReplaceKeys(
            wallet_handle,
            identity_json,
            did,
            Box::new(move |result| {
                let (err, verkey, pk) = result_to_err_code_2!(result, String::new(), String::new());
                let verkey = CStringUtils::string_to_cstring(verkey);
                let pk = CStringUtils::string_to_cstring(pk);
                cb(command_handle, err, verkey.as_ptr(), pk.as_ptr())
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
///        "verkey": string (optional, if only pk is provided),
///        "pk": string (optional, if only verification key is provided),
///        "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_store_their_did(command_handle: i32,
                                      wallet_handle: i32,
                                      identity_json: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(identity_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::StoreTheirDid(
            wallet_handle,
            identity_json,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Signs a message by a signing key associated with my DID. The DID with a signing key
/// must be already created and stored in a secured wallet (see create_and_store_my_identity)
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// did: signing DID
/// msg: a message to be signed
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// a signature string
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_sign(command_handle: i32,
                           wallet_handle: i32,
                           did: *const c_char,
                           msg: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                signature: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(msg, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Sign(
            wallet_handle,
            did,
            msg,
            Box::new(move |result| {
                let (err, signed_msg) = result_to_err_code_1!(result, String::new());
                let signed_msg = CStringUtils::string_to_cstring(signed_msg);
                cb(command_handle, err, signed_msg.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Verify a signature created by a key associated with a DID.
/// If a secure wallet doesn't contain a verkey associated with the given DID,
/// then verkey is read from the Ledger.
/// Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether verkey is still the same and updates verkey for the DID if needed.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// pool_handle: pool handle.
/// did: DID that signed the message
/// msg: message
/// signature: a signature to be verified
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// valid: true - if signature is valid, false - otherwise
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_verify_signature(command_handle: i32,
                                       wallet_handle: i32,
                                       pool_handle: i32,
                                       did: *const c_char,
                                       signed_msg: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                            valid: bool)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(signed_msg, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::VerifySignature(
            wallet_handle,
            pool_handle,
            did,
            signed_msg,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                cb(command_handle, err, valid)
            })
        )));

    result_to_err_code!(result)
}

/// Encrypts a message by a public key associated with a DID.
/// If a secure wallet doesn't contain a public key associated with the given DID,
/// then the public key is read from the Ledger.
/// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether public key is still the same and updates public key for the DID if needed.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// my_did: encrypting DID
/// did: encrypting DID
/// msg: a message to be signed
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// an encrypted message and nonce
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_encrypt(command_handle: i32,
                              wallet_handle: i32,
                              pool_handle: i32,
                              my_did: *const c_char,
                              did: *const c_char,
                              msg: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                   encrypted_msg: *const c_char,
                                                   nonce: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(msg, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Encrypt(
            wallet_handle,
            pool_handle,
            my_did,
            did,
            msg,
            Box::new(move |result| {
                let (err, encrypted_msg, nonce) = result_to_err_code_2!(result, String::new(), String::new());
                let encrypted_msg = CStringUtils::string_to_cstring(encrypted_msg);
                let nonce = CStringUtils::string_to_cstring(nonce);
                cb(command_handle, err, encrypted_msg.as_ptr(), nonce.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Decrypts a message encrypted by a public key associated with my DID.
/// The DID with a secret key must be already created and
/// stored in a secured wallet (see wallet_create_and_store_my_identity)
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// my_did: DID
/// did: DID that signed the message
/// encrypted_msg: encrypted message
/// nonce: nonce that encrypted message
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// decrypted message
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn sovrin_decrypt(command_handle: i32,
                              wallet_handle: i32,
                              my_did: *const c_char,
                              did: *const c_char,
                              encrypted_msg: *const c_char,
                              nonce: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                   decrypted_msg: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(encrypted_msg, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(nonce, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Decrypt(
            wallet_handle,
            did,
            my_did,
            encrypted_msg,
            nonce,
            Box::new(move |result| {
                let (err, decrypted_msg) = result_to_err_code_1!(result, String::new());
                let decrypted_msg = CStringUtils::string_to_cstring(decrypted_msg);
                cb(command_handle, err, decrypted_msg.as_ptr())
            })
        )));

    result_to_err_code!(result)
}