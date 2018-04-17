extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::crypto::CryptoCommand;
use utils::cstring::CStringUtils;
use utils::byte_array::vec_to_pointer;

use self::libc::c_char;


/// Creates keys pair and stores in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// key_json: Key information as json. Example:
/// {
///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code.
/// - verkey: Ver key of generated key pair, also used as key identifier
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_create_key(command_handle: i32,
                               wallet_handle: i32,
                               key_json: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    verkey: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(key_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::CreateKey(
            wallet_handle,
            key_json,
            Box::new(move |result| {
                let (err, verkey) = result_to_err_code_1!(result, String::new());
                let verkey = CStringUtils::string_to_cstring(verkey);
                cb(command_handle, err, verkey.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Saves/replaces the meta information for the giving key in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// verkey - the key (verkey, key id) to store metadata.
/// metadata - the meta information that will be store with the key.
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
pub  extern fn indy_set_key_metadata(command_handle: i32,
                                     wallet_handle: i32,
                                     verkey: *const c_char,
                                     metadata: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32,
                                                          err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam3);
    check_useful_c_str_empty_accepted!(metadata, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::SetKeyMetadata(
            wallet_handle,
            verkey,
            metadata,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Retrieves the meta information for the giving key in the wallet.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// verkey - The key (verkey, key id) to retrieve metadata.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code.
/// - metadata - The meta information stored with the key; Can be null if no metadata was saved for this key.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_get_key_metadata(command_handle: i32,
                                     wallet_handle: i32,
                                     verkey: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32,
                                                          err: ErrorCode,
                                                          metadata: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::GetKeyMetadata(
            wallet_handle,
            verkey,
            Box::new(move |result| {
                let (err, metadata) = result_to_err_code_1!(result, String::new());
                let metadata = CStringUtils::string_to_cstring(metadata);
                cb(command_handle, err, metadata.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Signs a message with a key.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// signer_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
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
pub  extern fn indy_crypto_sign(command_handle: i32,
                                wallet_handle: i32,
                                signer_vk: *const c_char,
                                message_raw: *const u8,
                                message_len: u32,
                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                     signature_raw: *const u8, signature_len: u32)>) -> ErrorCode {
    check_useful_c_str!(signer_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::CryptoSign(
            wallet_handle,
            signer_vk,
            message_raw,
            Box::new(move |result| {
                let (err, signature) = result_to_err_code_1!(result, Vec::new());
                let (signature_raw, signature_len) = vec_to_pointer(&signature);
                cb(command_handle, err, signature_raw, signature_len)
            })
        )));

    result_to_err_code!(result)
}

/// Verify a signature with a verkey.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// signer_vk: verkey of signer of the message
/// message_raw: a pointer to first byte of message that has been signed
/// message_len: a message length
/// signature_raw: a pointer to first byte of signature to be verified
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
pub  extern fn indy_crypto_verify(command_handle: i32,
                                  signer_vk: *const c_char,
                                  message_raw: *const u8,
                                  message_len: u32,
                                  signature_raw: *const u8,
                                  signature_len: u32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                       valid: bool)>) -> ErrorCode {
    check_useful_c_str!(signer_vk, ErrorCode::CommonInvalidParam2);
    check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidParam3, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(signature_raw, signature_len, ErrorCode::CommonInvalidParam5, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::CryptoVerify(
            signer_vk,
            message_raw,
            signature_raw,
            Box::new(move |result| {
                let (err, valid) = result_to_err_code_1!(result, false);
                cb(command_handle, err, valid)
            })
        )));

    result_to_err_code!(result)
}

/// Encrypt a message by authenticated-encryption scheme.
///
/// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
/// Using Recipient's public key, Sender can compute a shared secret key.
/// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
/// That shared secret key can be used to verify that the encrypted message was not tampered with,
/// before eventually decrypting it.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handle (created by open_wallet).
/// sender_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
/// recipient_vk: id (verkey) of their key
/// message_raw: a pointer to first byte of message that to be encrypted
/// message_len: a message length
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// an encrypted message as a pointer to array of bytes.
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub  extern fn indy_crypto_auth_crypt(command_handle: i32,
                                      wallet_handle: i32,
                                      sender_vk: *const c_char,
                                      recipient_vk: *const c_char,
                                      msg_data: *const u8,
                                      msg_len: u32,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           encrypted_msg: *const u8,
                                                           encrypted_len: u32)>) -> ErrorCode {
    check_useful_c_str!(sender_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(msg_data, msg_len, ErrorCode::CommonInvalidParam5, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::AuthenticatedEncrypt(
            wallet_handle,
            sender_vk,
            recipient_vk,
            msg_data,
            Box::new(move |result| {
                let (err, encrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

/// Decrypt a message by authenticated-encryption scheme.
///
/// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
/// Using Recipient's public key, Sender can compute a shared secret key.
/// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
/// That shared secret key can be used to verify that the encrypted message was not tampered with,
/// before eventually decrypting it.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// recipient_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
/// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
/// encrypted_msg_len: a message length
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// sender verkey and decrypted message as a pointer to array of bytes
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_crypto_auth_decrypt(command_handle: i32,
                                        wallet_handle: i32,
                                        recipient_vk: *const c_char,
                                        encrypted_msg: *const u8,
                                        encrypted_len: u32,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             sender_vk: *const c_char,
                                                             msg_data: *const u8,
                                                             msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(encrypted_msg, encrypted_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::AuthenticatedDecrypt(
            wallet_handle,
            recipient_vk,
            encrypted_msg,
            Box::new(move |result| {
                let (err, sender_vk, msg) = result_to_err_code_2!(result, String::new(), Vec::new());
                let (msg_data, msg_len) = vec_to_pointer(&msg);
                let sender_vk = CStringUtils::string_to_cstring(sender_vk);
                cb(command_handle, err, sender_vk.as_ptr(), msg_data, msg_len)
            })
        )));

    result_to_err_code!(result)
}

/// Encrypts a message by anonymous-encryption scheme.
///
/// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
/// Only the Recipient can decrypt these messages, using its private key.
/// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// recipient_vk: verkey of message recipient
/// message_raw: a pointer to first byte of message that to be encrypted
/// message_len: a message length
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// an encrypted message as a pointer to array of bytes
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
#[no_mangle]
pub  extern fn indy_crypto_anon_crypt(command_handle: i32,
                                      recipient_vk: *const c_char,
                                      msg_data: *const u8,
                                      msg_len: u32,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           encrypted_msg: *const u8,
                                                           encrypted_len: u32)>) -> ErrorCode {
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam2);
    check_useful_c_byte_array!(msg_data, msg_len, ErrorCode::CommonInvalidParam3, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::AnonymousEncrypt(
            recipient_vk,
            msg_data,
            Box::new(move |result| {
                let (err, encrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

/// Decrypts a message by anonymous-encryption scheme.
///
/// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
/// Only the Recipient can decrypt these messages, using its private key.
/// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
///
/// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
/// for specific DID.
///
/// #Params
/// command_handle: command handle to map callback to user context.
/// wallet_handle: wallet handler (created by open_wallet).
/// recipient_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
/// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
/// encrypted_msg_len: a message length
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// decrypted message as a pointer to an array of bytes
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_crypto_anon_decrypt(command_handle: i32,
                                        wallet_handle: i32,
                                        recipient_vk: *const c_char,
                                        encrypted_msg: *const u8,
                                        encrypted_len: u32,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             msg_data: *const u8,
                                                             msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(encrypted_msg, encrypted_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Crypto(CryptoCommand::AnonymousDecrypt(
            wallet_handle,
            recipient_vk,
            encrypted_msg,
            Box::new(move |result| {
                let (err, msg) = result_to_err_code_1!(result, Vec::new());
                let (msg_data, msg_len) = vec_to_pointer(&msg);
                cb(command_handle, err, msg_data, msg_len)
            })
        )));

    result_to_err_code!(result)
}