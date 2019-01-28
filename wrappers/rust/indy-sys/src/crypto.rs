use super::*;

use {BString, CString, Error, Handle};

extern {

    #[no_mangle]
    pub fn indy_create_key(command_handle: Handle,
                           wallet_handle: Handle,
                           key_json: CString,
                           cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_set_key_metadata(command_handle: Handle,
                                 wallet_handle: Handle,
                                 verkey: CString,
                                 metadata: CString,
                                 cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_key_metadata(command_handle: Handle,
                                 wallet_handle: Handle,
                                 verkey: CString,
                                 cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_sign(command_handle: Handle,
                            wallet_handle: Handle,
                            signer_vk: CString,
                            message_raw: BString,
                            message_len: u32,
                            cb: Option<ResponseSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_verify(command_handle: Handle,
                              signer_vk: CString,
                              message_raw: BString,
                              message_len: u32,
                              signature_raw: BString,
                              signature_len: u32,
                              cb: Option<ResponseBoolCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_auth_crypt(command_handle: Handle,
                                  wallet_handle: Handle,
                                  sender_vk: CString,
                                  recipient_vk: CString,
                                  msg_data: BString,
                                  msg_len: u32,
                                  cb: Option<ResponseSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_auth_decrypt(command_handle: Handle,
                                    wallet_handle: Handle,
                                    recipient_vk: CString,
                                    encrypted_msg: BString,
                                    encrypted_len: u32,
                                    cb: Option<ResponseStringSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_anon_crypt(command_handle: Handle,
                                  recipient_vk: CString,
                                  msg_data: BString,
                                  msg_len: u32,
                                  cb: Option<ResponseSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_crypto_anon_decrypt(command_handle: Handle,
                                    wallet_handle: Handle,
                                    recipient_vk: CString,
                                    encrypted_msg: BString,
                                    encrypted_len: u32,
                                    cb: Option<ResponseSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_pack_message(command_handle: Handle,
                             wallet_handle: Handle,
                             message: BString,
                             message_len: u32,
                             receiver_keys: CString,
                             sender: CString,
                             cb: Option<ResponseSliceCB>) -> Error;

    #[no_mangle]
    pub fn indy_unpack_message(command_handle: Handle,
                               wallet_handle: Handle,
                               jwe_msg: BString,
                               jwe_len: u32,
                               cb: Option<ResponseSliceCB>) -> Error;
}

