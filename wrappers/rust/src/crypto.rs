use ffi::crypto;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseSliceCB,
          ResponseBoolCB,
          ResponseStringSliceCB};

use futures::Future;

use std::ffi::CString;
use std::ptr::null;

use {ErrorCode, IndyError};
use utils::callbacks::{ClosureHandler, ResultHandler};
use {WalletHandle, CommandHandle};

/// Creates key pair in wallet
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `my_key_json` - Optional key information as json. If none then defaults are used.
///
/// # Example
/// my_key_json
/// {
///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
///                                Can be UTF-8, base64 or hex string.
///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
/// }
/// # Returns
/// verkey of generated key pair, also used as key identifier
pub fn create_key(wallet_handle: WalletHandle, my_key_json: Option<&str>) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _create_key(command_handle, wallet_handle, my_key_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _create_key(command_handle: CommandHandle, wallet_handle: WalletHandle, my_key_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
    let my_key_json = opt_c_str_json!(my_key_json);

    ErrorCode::from(unsafe { crypto::indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb) })
}

/// Saves/replaces the metadata for the `verkey` in the wallet
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `verkey` - the public key or key id where to store the metadata
/// * `metadata` - the metadata that will be stored with the key, can be empty string
pub fn set_key_metadata(wallet_handle: WalletHandle, verkey: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _set_key_metadata(command_handle, wallet_handle, verkey, metadata, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _set_key_metadata(command_handle: CommandHandle, wallet_handle: WalletHandle, verkey: &str, metadata: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let verkey = c_str!(verkey);
    let metadata = c_str!(metadata);

    ErrorCode::from(unsafe { crypto::indy_set_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), metadata.as_ptr(), cb) })
}

/// Retrieves the metadata for the `verkey` in the wallet
/// # Argument
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `verkey` - the public key or key id to retrieve metadata
/// # Returns
/// metadata currently stored with the key; Can be empty if no metadata was saved for this key
pub fn get_key_metadata(wallet_handle: WalletHandle, verkey: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_key_metadata(command_handle, wallet_handle, verkey, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_key_metadata(command_handle: CommandHandle, wallet_handle: WalletHandle, verkey: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let verkey = c_str!(verkey);

    ErrorCode::from(unsafe { crypto::indy_get_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb) })
}

/// Signs a message with a key
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `signer_vk` - key id or verkey of my key. The key must be created by calling create_key or Did::new
/// * `message` - the data to be signed
/// # Returns
/// the signature
pub fn sign(wallet_handle: WalletHandle, signer_vk: &str, message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _sign(command_handle, wallet_handle, signer_vk, message, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _sign(command_handle: CommandHandle, wallet_handle: WalletHandle, signer_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    let signer_vk = c_str!(signer_vk);
    ErrorCode::from(unsafe {
        crypto::indy_crypto_sign(command_handle, wallet_handle, signer_vk.as_ptr(),
                         message.as_ptr() as *const u8,
                         message.len() as u32,
                         cb)
    })
}

/// Verify a signature with a verkey
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `signer_vk` - key id or verkey of my key. The key must be created by calling create_key or Did::new
/// * `message` - the data that was signed
/// * `signature` - the signature to verify
/// # Returns
/// true if signature is valid, false otherwise
pub fn verify(signer_vk: &str, message: &[u8], signature: &[u8]) -> Box<Future<Item=bool, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

    let err = _verify(command_handle, signer_vk, message, signature, cb);

    ResultHandler::bool(command_handle, err, receiver)
}

fn _verify(command_handle: CommandHandle, signer_vk: &str, message: &[u8], signature: &[u8], cb: Option<ResponseBoolCB>) -> ErrorCode {
    let signer_vk = c_str!(signer_vk);

    ErrorCode::from(unsafe {
        crypto::indy_crypto_verify(command_handle, signer_vk.as_ptr(),
                           message.as_ptr() as *const u8, message.len() as u32,
                           signature.as_ptr() as *const u8, signature.len() as u32, cb)
    })
}

/// **** THIS FUNCTION WILL BE DEPRECATED USE pack_message INSTEAD ****
/// Encrypt a message by authenticated-encryption scheme.
///
/// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
/// Using Recipient's public key, Sender can compute a shared secret key.
/// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
/// That shared secret key can be used to verify that the encrypted message was not tampered with,
/// before eventually decrypting it.
///
/// Note to use DID keys with this function you can call Did::get_ver_key to get key id (verkey)
/// for specific DID.
/// # Arguments
/// * `wallet_handle` - wallet handle (created by Wallet::open)
/// * `signer_vk` - key id or verkey of my key. The key must be created by calling create_key or Did::new
/// * `recipient_vk` - key id or verkey of the other party's key
/// * `message` - the data to be encrypted
/// # Returns
/// the encrypted message
pub fn auth_crypt(wallet_handle: WalletHandle, sender_vk: &str, recipient_vk: &str, message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _auth_crypt(command_handle, wallet_handle, sender_vk, recipient_vk, message, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _auth_crypt(command_handle: CommandHandle, wallet_handle: WalletHandle, sender_vk: &str, recipient_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    let sender_vk = c_str!(sender_vk);
    let recipient_vk = c_str!(recipient_vk);
    ErrorCode::from(unsafe {
        crypto::indy_crypto_auth_crypt(command_handle, wallet_handle,
                               sender_vk.as_ptr(),
                                recipient_vk.as_ptr(),
                                message.as_ptr() as *const u8,
                                message.len() as u32, cb)
    })
}

/// **** THIS FUNCTION WILL BE DEPRECATED USE unpack_message INSTEAD ****
/// Decrypt a message by authenticated-encryption scheme.
///
/// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
/// Using Recipient's public key, Sender can compute a shared secret key.
/// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
/// That shared secret key can be used to verify that the encrypted message was not tampered with,
/// before eventually decrypting it.
///
/// Note to use DID keys with this function you can call Did::get_ver_key to get key id (verkey)
/// for specific DID.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open)
/// * `recipient_vk`: key id or verkey of my key. The key must be created by calling create_key or Did::new
/// * `encrypted_message`: the message to be decrypted
/// # Returns
/// sender's verkey and decrypted message
pub fn auth_decrypt(wallet_handle: WalletHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Box<Future<Item=(String, Vec<u8>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_slice();

    let err = _auth_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

    ResultHandler::str_slice(command_handle, err, receiver)
}

fn _auth_decrypt(command_handle: CommandHandle, wallet_handle: WalletHandle, recipient_vk: &str, encrypted_message: &[u8], cb: Option<ResponseStringSliceCB>) -> ErrorCode {
    let recipient_vk = c_str!(recipient_vk);
    ErrorCode::from(unsafe {
        crypto::indy_crypto_auth_decrypt(command_handle,
                                 wallet_handle,
                                 recipient_vk.as_ptr(),
                                 encrypted_message.as_ptr() as *const u8,
                                 encrypted_message.len() as u32, cb)
    })
}

/// Encrypts a message by anonymous-encryption scheme.
///
/// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
/// Only the Recipient can decrypt these messages, using its private key.
/// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
///
/// Note to use DID keys with this function you can call Did::get_ver_key to get key id (verkey)
/// for specific DID.
///
/// Note: use pack_message function for A2A goals.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open)
/// * `recipient_vk`: verkey of message recipient
/// * `message`: a pointer to first byte of message that to be encrypted
///
/// # Returns
/// the encrypted message
pub fn anon_crypt(recipient_vk: &str, message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _anon_crypt(command_handle, recipient_vk, message, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _anon_crypt(command_handle: CommandHandle, recipient_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    let recipient_vk = c_str!(recipient_vk);
    ErrorCode::from(unsafe {
        crypto::indy_crypto_anon_crypt(command_handle,
                               recipient_vk.as_ptr(),
                               message.as_ptr() as *const u8,
                                message.len() as u32,
                                cb)
    })
}

/// Decrypts a message by anonymous-encryption scheme.
///
/// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
/// Only the Recipient can decrypt these messages, using its private key.
/// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
///
/// Note to use DID keys with this function you can call Did::get_ver_key to get key id (verkey)
/// for specific DID.
///
/// Note: use unpack_message function for A2A goals.
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open).
/// * `recipient_vk`: key id or verkey of my key. The key must be created by calling create_key or Did::new
/// * `encrypted_message`: a pointer to first byte of message that to be decrypted
///
/// # Returns
/// decrypted message
pub fn anon_decrypt(wallet_handle: WalletHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _anon_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _anon_decrypt(command_handle: CommandHandle, wallet_handle: WalletHandle, recipient_vk: &str, encrypted_message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    let recipient_vk = c_str!(recipient_vk);
    ErrorCode::from(unsafe {
        crypto::indy_crypto_anon_decrypt(command_handle,
                                 wallet_handle,
                                 recipient_vk.as_ptr(),
                                 encrypted_message.as_ptr() as *const u8,
                                 encrypted_message.len() as u32, cb)
    })
}

/// Unpacks a message packed using indy_pack_message which follows the wire message format HIPE
/// (Experimental)
///
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open).
/// * `message`: a pointer to the first byte of the message to be encrypted
/// * `receiver_keys`: a JSON array as a string containing a list of the receivers verkey's
/// * `sender` : a string of the sender's verkey When None is used in this parameter, anoncrypt is used
/// # Returns
/// a json structure in the form of a JWE that contains the encrypted message and associated metadata
pub fn pack_message(wallet_handle: WalletHandle, message: &[u8], receiver_keys: &str, sender: Option<&str>) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err= _pack_message(command_handle, wallet_handle, message, receiver_keys, sender, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _pack_message(command_handle: CommandHandle, wallet_handle: WalletHandle, message: &[u8], receiver_keys: &str, sender: Option<&str>, cb: Option<ResponseSliceCB>) -> ErrorCode {
    let receiver_keys = c_str!(receiver_keys);
    let sender_str = opt_c_str!(sender);

    ErrorCode::from(unsafe {
        crypto::indy_pack_message(command_handle,
                                  wallet_handle,
                                  message.as_ptr() as *const u8,
                                  message.len() as u32,
                                  receiver_keys.as_ptr(),
                                  opt_c_ptr!(sender, sender_str),
                                  cb)
    })

}

/// Unpacks a message packed using indy_pack_message which follows the wire message format HIPE
/// (Experimental)
///
///
/// # Arguments
/// * `wallet_handle`: wallet handle (created by Wallet::open).
/// * `jwe`: a pointer to the first byte of the JWE string
/// # Returns
/// a json structure that contains a decrypted message and a sender_verkey if packed with authcrypt
pub fn unpack_message(wallet_handle: WalletHandle, jwe: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err= _unpack_message(command_handle, wallet_handle, jwe, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _unpack_message(command_handle: CommandHandle, wallet_handle: WalletHandle, jwe: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        crypto::indy_unpack_message(command_handle,
                                    wallet_handle,
                                    jwe.as_ptr() as *const u8,
                                    jwe.len() as u32,
                                    cb)
    })
}

