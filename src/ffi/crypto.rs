use super::*;

use {ErrorCode, IndyHandle};

use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_create_key(command_handle: IndyHandle,
                           wallet_handle: IndyHandle,
                           key_json: *const c_char,
                           cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_key_metadata(command_handle: IndyHandle,
                                 wallet_handle: IndyHandle,
                                 verkey: *const c_char,
                                 metadata: *const c_char,
                                 cb: Option<ResponseEmptyCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_get_key_metadata(command_handle: IndyHandle,
                                 wallet_handle: IndyHandle,
                                 verkey: *const c_char,
                                 cb: Option<ResponseStringCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_sign(command_handle: IndyHandle,
                            wallet_handle: IndyHandle,
                            signer_vk: *const c_char,
                            message_raw: *const u8,
                            message_len: u32,
                            cb: Option<ResponseSliceCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_verify(command_handle: IndyHandle,
                              signer_vk: *const c_char,
                              message_raw: *const u8,
                              message_len: u32,
                              signature_raw: *const u8,
                              signature_len: u32,
                              cb: Option<ResponseBoolCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_auth_crypt(command_handle: IndyHandle,
                                  wallet_handle: IndyHandle,
                                  sender_vk: *const c_char,
                                  recipient_vk: *const c_char,
                                  message_raw: *const u8,
                                  message_len: u32,
                                  cb: Option<ResponseSliceCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_auth_decrypt(command_handle: IndyHandle,
                                    wallet_handle: IndyHandle,
                                    recipient_vk: *const c_char,
                                    encrypted_msg_raw: *const u8,
                                    encrypted_msg_len: u32,
                                    cb: Option<ResponseStringSliceCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_anon_crypt(command_handle: IndyHandle,
                                  recipient_vk: *const c_char,
                                  message_raw: *const u8,
                                  message_len: u32,
                                  cb: Option<ResponseSliceCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_crypto_anon_decrypt(command_handle: IndyHandle,
                                    wallet_handle: IndyHandle,
                                    recipient_vk: *const c_char,
                                    encrypted_msg_raw: *const u8,
                                    encrypted_msg_len: u32,
                                    cb: Option<ResponseSliceCB>) -> ErrorCode;
}
