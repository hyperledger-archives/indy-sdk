extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::signus::SignusCommand;
use utils::cstring::CStringUtils;
use utils::byte_array::vec_to_pointer;

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
/// DID, verkey (for verification of signature) and public_key (for decryption)
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
        .send(Command::Signus(SignusCommand::CreateAndStoreMyDid(
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
/// verkey (for verification of signature) and public_key (for decryption)
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
        .send(Command::Signus(SignusCommand::ReplaceKeysStart(
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
/// did
/// cb: Callback that takes command result as parameter.
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
        .send(Command::Signus(SignusCommand::ReplaceKeysApply(
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
/// None
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
        .send(Command::Signus(SignusCommand::KeyForDid(
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
        .send(Command::Signus(SignusCommand::KeyForLocalDid(
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
        .send(Command::Signus(SignusCommand::SetEndpointForDid(
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
        .send(Command::Signus(SignusCommand::GetEndpointForDid(
            wallet_handle,
            pool_handle,
            did,
            Box::new(move |result| {
                let (err, address, transport_vk) = result_to_err_code_2!(result, String::new(), String::new());
                let address = CStringUtils::string_to_cstring(address);
                let transport_vk = CStringUtils::string_to_cstring(transport_vk);
                cb(command_handle, err, address.as_ptr(), transport_vk.as_ptr())
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
        .send(Command::Signus(SignusCommand::SetDidMetadata(
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
        .send(Command::Signus(SignusCommand::GetDidMetadata(
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

/// Signs a message by a signing key associated with my DID. The DID with a signing key
/// must be already created and stored in a secured wallet (see create_and_store_my_identity)
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// did: signing DID
/// message_raw: a pointer to first byte of message to be signed
/// message_len: a message length
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
pub  extern fn indy_sign(command_handle: i32,
                         wallet_handle: i32,
                         did: *const c_char,
                         message_raw: *const u8,
                         message_len: u32,
                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                              signature_raw: *const u8, signature_len: u32)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Sign(
            wallet_handle,
            did,
            message_raw,
            Box::new(move |result| {
                let (err, signature) = result_to_err_code_1!(result, Vec::new());
                let (signature_raw, signature_len) = vec_to_pointer(&signature);
                cb(command_handle, err, signature_raw, signature_len)
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
/// their_did: DID that signed the message
/// message_raw: a pointer to first byte of message to be signed
/// message_len: a message length
/// signature_raw: a a pointer to first byte of signature to be verified
/// signature_len: a signature length
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
pub  extern fn indy_verify_signature(command_handle: i32,
                                     wallet_handle: i32,
                                     pool_handle: i32,
                                     did: *const c_char,
                                     message_raw: *const u8,
                                     message_len: u32,
                                     signature_raw: *const u8,
                                     signature_len: u32,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                          valid: bool)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam5, ErrorCode::CommonInvalidParam6);
    check_useful_c_byte_array!(signature_raw, signature_len, ErrorCode::CommonInvalidParam7, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::VerifySignature(
            wallet_handle,
            pool_handle,
            did,
            message_raw,
            signature_raw,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                cb(command_handle, err, valid)
            })
        )));

    result_to_err_code!(result)
}

/// Encrypts a message by public-key (associated with their did) authenticated-encryption scheme
/// using nonce.
/// If a secure wallet doesn't contain a public key associated with the given DID,
/// then the public key is read from the Ledger.
/// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether public key is still the same and updates public key for the DID if needed.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// pool_handle: pool handle.
/// my_did: encrypted DID
/// their_did: encrypted DID
/// message_raw: a pointer to first byte of message that to be encrypted
/// message_len: a message length
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
pub  extern fn indy_encrypt(command_handle: i32,
                            wallet_handle: i32,
                            pool_handle: i32,
                            my_did: *const c_char,
                            their_did: *const c_char,
                            message_raw: *const u8,
                            message_len: u32,
                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 encrypted_msg_raw: *const u8, encrypted_msg_len: u32,
                                                 nonce_raw: *const u8, nonce_len: u32)>) -> ErrorCode {
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam5);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam6, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Encrypt(
            wallet_handle,
            pool_handle,
            my_did,
            their_did,
            message_raw,
            Box::new(move |result| {
                let (err, encrypted_msg, nonce) = result_to_err_code_2!(result, Vec::new(), Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                let (nonce_raw, nonce_len) = vec_to_pointer(&nonce);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len, nonce_raw, nonce_len)
            })
        )));

    result_to_err_code!(result)
}

/// Decrypts a message by public-key authenticated-encryption scheme using nonce.
/// The DID with a secret key must be already created and
/// stored in a secured wallet (see wallet_create_and_store_my_identity)
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// pool_handle: pool handle.
/// my_did: encrypted DID
/// their_did: encrypted DID that signed the message
/// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
/// encrypted_msg_len: a message length
/// nonce_raw: a pointer to first byte of nonce that encrypted message
/// nonce_len: a nonce length
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
pub  extern fn indy_decrypt(command_handle: i32,
                            wallet_handle: i32,
                            pool_handle: i32,
                            my_did: *const c_char,
                            their_did: *const c_char,
                            encrypted_msg_raw: *const u8,
                            encrypted_msg_len: u32,
                            nonce_raw: *const u8,
                            nonce_len: u32,
                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                 decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(encrypted_msg_raw, encrypted_msg_len, ErrorCode::CommonInvalidParam5, ErrorCode::CommonInvalidParam6);
    check_useful_c_byte_array!(nonce_raw, nonce_len, ErrorCode::CommonInvalidParam7, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam9);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::Decrypt(
            wallet_handle,
            pool_handle,
            my_did,
            their_did,
            encrypted_msg_raw,
            nonce_raw,
            Box::new(move |result| {
                let (err, decrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (decrypted_msg_raw, decrypted_msg_len) = vec_to_pointer(&decrypted_msg);
                cb(command_handle, err, decrypted_msg_raw, decrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

/// Encrypts a message by public-key (associated with did) anonymous-encryption scheme.
/// If a secure wallet doesn't contain a public key associated with the given DID,
/// then the public key is read from the Ledger.
/// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether public key is still the same and updates public key for the DID if needed.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// pool_handle: pool handle.
/// command_handle: command handle to map callback to user context.
/// did: encrypted DID
/// message_raw: a pointer to first byte of message that to be encrypted
/// message_len: a message length
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// an encrypted message
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub  extern fn indy_encrypt_sealed(command_handle: i32,
                                   wallet_handle: i32,
                                   pool_handle: i32,
                                   did: *const c_char,
                                   message_raw: *const u8,
                                   message_len: u32,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::EncryptSealed(
            wallet_handle,
            pool_handle,
            did,
            message_raw,
            Box::new(move |result| {
                let (err, encrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

/// Decrypts a message by public-key anonymous-encryption scheme.
/// The DID with a secret key must be already created and
/// stored in a secured wallet (see wallet_create_and_store_my_identity)
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// did: DID that signed the message
/// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
/// encrypted_msg_len: a message length
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
pub  extern fn indy_decrypt_sealed(command_handle: i32,
                                   wallet_handle: i32,
                                   did: *const c_char,
                                   encrypted_msg_raw: *const u8,
                                   encrypted_msg_len: u32,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                        decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(encrypted_msg_raw, encrypted_msg_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Signus(SignusCommand::DecryptSealed(
            wallet_handle,
            did,
            encrypted_msg_raw,
            Box::new(move |result| {
                let (err, decrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (decrypted_msg_raw, decrypted_msg_len) = vec_to_pointer(&decrypted_msg);
                cb(command_handle, err, decrypted_msg_raw, decrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}
