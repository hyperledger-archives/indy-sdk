use super::{ErrorCode, IndyHandle};

use libc::c_char;
use std::ffi::CString;

pub struct Crypto {}

impl Crypto {
    pub fn create_key(wallet_handle: IndyHandle, my_key_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let my_key_json = CString::new(my_key_json).unwrap();

        let err = unsafe {
            indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb)
        };
        super::results::result_to_string(err, receiver)
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
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, valid: bool)>) -> ErrorCode; 

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
