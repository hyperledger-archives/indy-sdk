use super::{ErrorCode, IndyHandle};
use libc::c_char;
use std::ffi::CString;
use utils;

pub struct Key {}

impl Key {
    pub fn create(wallet_handle: IndyHandle, my_key_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let my_key_json = CString::new(my_key_json).unwrap();

        let err = unsafe {
            indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb)
        };
        utils::results::result_to_one(err, receiver)
    }

    pub fn set_metadata(wallet_handle: IndyHandle, verkey: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let verkey = CString::new(verkey).unwrap();

        let err = unsafe {
            indy_set_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb)
        };
        utils::results::result_to_empty(err, receiver)
    }

    pub fn get_metadata(wallet_handle: IndyHandle, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let verkey = CString::new(verkey).unwrap();

        let err = unsafe {
            indy_get_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb)
        };

        utils::results::result_to_one(err, receiver)
    }
}

pub struct Crypto {}

impl Crypto {
    pub fn sign(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_u8();

        let signer_vk = CString::new(signer_vk).unwrap();
        let err = unsafe {
            indy_crypto_sign(command_handle, wallet_handle, signer_vk.as_ptr(),
                             message.as_ptr() as *const u8,
                             message.len() as u32,
                             cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn verify(wallet_handle: IndyHandle, message: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_bool();

        let err = unsafe {
            indy_crypto_verify(command_handle, wallet_handle,
                               message.as_ptr() as *const u8, message.len() as u32,
                               signature.as_ptr() as *const u8, signature.len() as u32, cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn auth_crypt(wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_u8();

        let sender_vk = CString::new(sender_vk).unwrap();
        let recipient_vk = CString::new(recipient_vk).unwrap();
        let err = unsafe {
            indy_crypto_auth_crypt(command_handle, wallet_handle,
                                   sender_vk.as_ptr(),
                                    recipient_vk.as_ptr(),
                                    message.as_ptr() as *const u8,
                                    message.len() as u32, cb)
        };
        utils::results::result_to_one(err, receiver)
    }

    pub fn auth_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string_u8();

        let recipient_vk = CString::new(recipient_vk).unwrap();
        let err = unsafe {
            indy_crypto_auth_decrypt(command_handle,
                                     wallet_handle,
                                     recipient_vk.as_ptr(),
                                     encrypted_message.as_ptr() as *const u8,
                                     encrypted_message.len() as u32, cb)
        };

        utils::results::result_to_two(err, receiver)
    }

    pub fn anon_crypt(wallet_handle: IndyHandle, recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_u8();

        let recipient_vk = CString::new(recipient_vk).unwrap();
        let err = unsafe {
            indy_crypto_anon_crypt(command_handle,
                                   wallet_handle,
                                   recipient_vk.as_ptr(),
                                   message.as_ptr() as *const u8,
                                    message.len() as u32,
                                    cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn anon_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string_u8();

        let recipient_vk = CString::new(recipient_vk).unwrap();
        let err = unsafe {
            indy_crypto_anon_decrypt(command_handle,
                                     wallet_handle,
                                     recipient_vk.as_ptr(),
                                     encrypted_message.as_ptr() as *const u8,
                                     encrypted_message.len() as u32, cb)
        };

        utils::results::result_to_two(err, receiver)
    }
}

extern {
    #[no_mangle]
    pub fn indy_create_key(command_handle: i32,
                           wallet_handle: i32,
                           key_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, vk: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_key_metadata(command_handle: i32,
                                 wallet_handle: i32,
                                 verkey: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;                                

    #[no_mangle]
    pub fn indy_get_key_metadata(command_handle: i32,
                                 wallet_handle: i32,
                                 verkey: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, metadata: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_sign(command_handle: i32,
                            wallet_handle: i32,
                            signer_vk: *const c_char,
                            message_raw: *const u8,
                            message_len: u32,
                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, signature_raw: *const u8, signature_len: u32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_verify(command_handle: i32,
                              wallet_handle: i32,
                              message_raw: *const u8,
                              message_len: u32,
                              signature_raw: *const u8,
                              signature_len: u32,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, valid: u8)>) -> ErrorCode; 

    #[no_mangle]
    pub fn indy_crypto_auth_crypt(command_handle: i32,
                                  wallet_handle: i32,
                                  sender_vk: *const c_char,
                                  recipient_vk: *const c_char,
                                  message_raw: *const u8,
                                  message_len: u32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_auth_decrypt(command_handle: i32,
                                    wallet_handle: i32,
                                    recipient_vk: *const c_char,
                                    encrypted_msg_raw: *const u8,
                                    encrypted_msg_len: u32,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, sender_vk: *const c_char, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_anon_crypt(command_handle: i32,
                                  wallet_handle: i32,
                                  recipient_vk: *const c_char,
                                  message_raw: *const u8,
                                  message_len: u32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_anon_decrypt(command_handle: i32,
                                    wallet_handle: i32,
                                    recipient_vk: *const c_char,
                                    encrypted_msg_raw: *const u8,
                                    encrypted_msg_len: u32,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, sender_vk: *const c_char, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) -> ErrorCode;
}
