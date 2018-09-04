use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use native::crypto;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseSliceCB,
          ResponseBoolCB,
          ResponseStringSliceCB};

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Key {}

impl Key {
    /// Creates key pair in wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `my_key_json` - Optional key information as json. If none then defaults are used.
    ///
    /// # Example
    /// my_key_json
    /// {
    ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// # Returns
    /// verkey of generated key pair, also used as key identifier
    pub fn create(wallet_handle: IndyHandle, my_key_json: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Key::_create(command_handle, wallet_handle, my_key_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Creates key pair in wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `my_key_json` - key information as json
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Example
    /// my_key_json
    /// {
    ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// # Returns
    /// verkey of generated key pair, also used as key identifier
    pub fn create_timeout(wallet_handle: IndyHandle, my_key_json: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Key::_create(command_handle, wallet_handle, my_key_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Creates key pair in wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `my_key_json` - Optional key information as json. If none then defaults are used.
    /// * `closure` - The closure that is called when finished
    ///
    /// # Example
    /// my_key_json
    /// {
    ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// # Returns
    /// errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(wallet_handle: IndyHandle, my_key_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Key::_create(command_handle, wallet_handle, my_key_json, cb)
    }

    fn _create(command_handle: IndyHandle, wallet_handle: IndyHandle, my_key_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let my_key_json = opt_c_str_json!(my_key_json);

        ErrorCode::from(unsafe { crypto::indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb) })
    }

    /// Saves/replaces the metadata for the `verkey` in the wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `verkey` - the public key or key id where to store the metadata
    /// * `metadata` - the metadata that will be stored with the key, can be empty string
    pub fn set_metadata(wallet_handle: IndyHandle, verkey: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Key::_set_metadata(command_handle, wallet_handle, verkey, metadata, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Saves/replaces the metadata for the `verkey` in the wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `verkey` - the public key or key id where to store the metadata
    /// * `metadata` - the metadata that will be stored with the key, can be empty string
    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_metadata_timeout(wallet_handle: IndyHandle, verkey: &str, metadata: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Key::_set_metadata(command_handle, wallet_handle, verkey, metadata, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Saves/replaces the metadata for the `verkey` in the wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `verkey` - the public key or key id where to store the metadata
    /// * `metadata` - the metadata that will be stored with the key, can be empty string
    /// * `closure` - The closure that is called when finished
    pub fn set_metadata_async<F: 'static>(wallet_handle: IndyHandle, verkey: &str, metadata: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Key::_set_metadata(command_handle, wallet_handle, verkey, metadata, cb)
    }

    fn _set_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, verkey: &str, metadata: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
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
    pub fn get_metadata(wallet_handle: IndyHandle, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Key::_get_metadata(command_handle, wallet_handle, verkey, cb);

        ResultHandler::one(err, receiver)
    }

    /// Retrieves the metadata for the `verkey` in the wallet
    /// # Argument
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `verkey` - the public key or key id to retrieve metadata
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// metadata currently stored with the key; Can be empty if no metadata was saved for this key
    pub fn get_metadata_timeout(wallet_handle: IndyHandle, verkey: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Key::_get_metadata(command_handle, wallet_handle, verkey, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Retrieves the metadata for the `verkey` in the wallet
    /// # Argument
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `verkey` - the public key or key id to retrieve metadata
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn get_metadata_async<F: 'static>(wallet_handle: IndyHandle, verkey: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Key::_get_metadata(command_handle, wallet_handle, verkey, cb)
    }

    fn _get_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, verkey: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let verkey = c_str!(verkey);

        ErrorCode::from(unsafe { crypto::indy_get_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb) })
    }
}

pub struct Crypto {}

impl Crypto {
    /// Signs a message with a key
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data to be signed
    /// # Returns
    /// the signature
    pub fn sign(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_sign(command_handle, wallet_handle, signer_vk, message, cb);

        ResultHandler::one(err, receiver)
    }

    /// Signs a message with a key
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data to be signed
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// the signature
    pub fn sign_timeout(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8], timeout: Duration) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_sign(command_handle, wallet_handle, signer_vk, message, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Signs a message with a key
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data to be signed
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn sign_async<F: 'static>(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(Box::new(closure));

        Crypto::_sign(command_handle, wallet_handle, signer_vk, message, cb)
    }

    fn _sign(command_handle: IndyHandle, wallet_handle: IndyHandle, signer_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
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
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data that was signed
    /// * `signature` - the signature to verify
    /// # Returns
    /// true if signature is valid, false otherwise
    pub fn verify(signer_vk: &str, message: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Crypto::_verify(command_handle, signer_vk, message, signature, cb);

        ResultHandler::one(err, receiver)
    }

     /// Verify a signature with a verkey
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data that was signed
    /// * `signature` - the signature to verify
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// true if signature is valid, false otherwise
    pub fn verify_timeout(signer_vk: &str, message: &[u8], signature: &[u8], timeout: Duration) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Crypto::_verify(command_handle, signer_vk, message, signature, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Verify a signature with a verkey
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data that was signed
    /// * `signature` - the signature to verify
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn verify_async<F: 'static>(signer_vk: &str, message: &[u8], signature: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, bool) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_bool(Box::new(closure));

        Crypto::_verify(command_handle, signer_vk, message, signature, cb)
    }

    fn _verify(command_handle: IndyHandle, signer_vk: &str, message: &[u8], signature: &[u8], cb: Option<ResponseBoolCB>) -> ErrorCode {
        let signer_vk = c_str!(signer_vk);

        ErrorCode::from(unsafe {
            crypto::indy_crypto_verify(command_handle, signer_vk.as_ptr(),
                               message.as_ptr() as *const u8, message.len() as u32,
                               signature.as_ptr() as *const u8, signature.len() as u32, cb)
        })
    }

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
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `recipient_vk` - key id or verkey of the other party's key
    /// * `message` - the data to be encrypted
    /// # Returns
    /// the encrypted message
    pub fn auth_crypt(wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_auth_crypt(command_handle, wallet_handle, sender_vk, recipient_vk, message, cb);

        ResultHandler::one(err, receiver)
    }

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
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `recipient_vk` - key id or verkey of the other party's key
    /// * `message` - the data to be encrypted
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// the encrypted message
    pub fn auth_crypt_timeout(wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8], timeout: Duration) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_auth_crypt(command_handle, wallet_handle, sender_vk, recipient_vk, message, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

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
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `recipient_vk` - key id or verkey of the other party's key
    /// * `message` - the data to be encrypted
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn auth_crypt_async<F: 'static>(wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(Box::new(closure));

        Crypto::_auth_crypt(command_handle, wallet_handle, sender_vk, recipient_vk, message, cb)
    }

    fn _auth_crypt(command_handle: IndyHandle, wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
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
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: the message to be decrypted
    /// # Returns
    /// sender's verkey and decrypted message
    pub fn auth_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_slice();

        let err = Crypto::_auth_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

        ResultHandler::two(err, receiver)
    }

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
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: the message to be decrypted
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// sender's verkey and decrypted message
    pub fn auth_decrypt_timeout(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], timeout: Duration) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_slice();

        let err = Crypto::_auth_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

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
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: the message to be decrypted
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn auth_decrypt_async<F: 'static>(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_slice(Box::new(closure));

        Crypto::_auth_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb)
    }

    fn _auth_decrypt(command_handle: IndyHandle, wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], cb: Option<ResponseStringSliceCB>) -> ErrorCode {
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open)
    /// * `recipient_vk`: verkey of message recipient
    /// * `message`: a pointer to first byte of message that to be encrypted
    ///
    /// # Returns
    /// the encrypted message
    pub fn anon_crypt(recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_anon_crypt(command_handle, recipient_vk, message, cb);

        ResultHandler::one(err, receiver)
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open)
    /// * `recipient_vk`: verkey of message recipient
    /// * `message`: a pointer to first byte of message that to be encrypted
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// the encrypted message
    pub fn anon_crypt_timeout(recipient_vk: &str, message: &[u8], timeout: Duration) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_anon_crypt(command_handle, recipient_vk, message, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open)
    /// * `recipient_vk`: verkey of message recipient
    /// * `message`: a pointer to first byte of message that to be encrypted
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn anon_crypt_async<F: 'static>(recipient_vk: &str, message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(Box::new(closure));

        Crypto::_anon_crypt(command_handle, recipient_vk, message, cb)
    }

    fn _anon_crypt(command_handle: IndyHandle, recipient_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open).
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: a pointer to first byte of message that to be decrypted
    ///
    /// # Returns
    /// decrypted message
    pub fn anon_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_anon_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

        ResultHandler::one(err, receiver)
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open).
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: a pointer to first byte of message that to be decrypted
    /// * `timeout` - the maximum time this function waits for a response
    /// # Returns
    /// decrypted message
    pub fn anon_decrypt_timeout(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], timeout: Duration) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_anon_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
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
    /// # Arguments
    /// * `wallet_handle`: wallet handle (created by Wallet::open).
    /// * `recipient_vk`: key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `encrypted_message`: a pointer to first byte of message that to be decrypted
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// decrypted message
    pub fn anon_decrypt_async<F: 'static>(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(Box::new(closure));

        Crypto::_anon_decrypt(command_handle, wallet_handle, recipient_vk, encrypted_message, cb)
    }

    fn _anon_decrypt(command_handle: IndyHandle, wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
        let recipient_vk = c_str!(recipient_vk);
        ErrorCode::from(unsafe {
            crypto::indy_crypto_anon_decrypt(command_handle,
                                     wallet_handle,
                                     recipient_vk.as_ptr(),
                                     encrypted_message.as_ptr() as *const u8,
                                     encrypted_message.len() as u32, cb)
        })
    }
}

