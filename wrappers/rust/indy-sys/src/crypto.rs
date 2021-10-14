use super::*;

use {BString, CString, Error, CommandHandle, WalletHandle};

extern {

    pub fn indy_create_key(command_handle: CommandHandle,
                           wallet_handle: WalletHandle,
                           key_json: CString,
                           cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_set_key_metadata(command_handle: CommandHandle,
                                 wallet_handle: WalletHandle,
                                 verkey: CString,
                                 metadata: CString,
                                 cb: Option<ResponseEmptyCB>) -> Error;

    pub fn indy_get_key_metadata(command_handle: CommandHandle,
                                 wallet_handle: WalletHandle,
                                 verkey: CString,
                                 cb: Option<ResponseStringCB>) -> Error;

    pub fn indy_crypto_sign(command_handle: CommandHandle,
                            wallet_handle: WalletHandle,
                            signer_vk: CString,
                            message_raw: BString,
                            message_len: u32,
                            cb: Option<ResponseSliceCB>) -> Error;

    pub fn indy_crypto_verify(command_handle: CommandHandle,
                              signer_vk: CString,
                              message_raw: BString,
                              message_len: u32,
                              signature_raw: BString,
                              signature_len: u32,
                              cb: Option<ResponseBoolCB>) -> Error;

    pub fn indy_crypto_auth_crypt(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  sender_vk: CString,
                                  recipient_vk: CString,
                                  msg_data: BString,
                                  msg_len: u32,
                                  cb: Option<ResponseSliceCB>) -> Error;

    pub fn indy_crypto_auth_decrypt(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    recipient_vk: CString,
                                    encrypted_msg: BString,
                                    encrypted_len: u32,
                                    cb: Option<ResponseStringSliceCB>) -> Error;

    pub fn indy_crypto_anon_crypt(command_handle: CommandHandle,
                                  recipient_vk: CString,
                                  msg_data: BString,
                                  msg_len: u32,
                                  cb: Option<ResponseSliceCB>) -> Error;

    pub fn indy_crypto_anon_decrypt(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    recipient_vk: CString,
                                    encrypted_msg: BString,
                                    encrypted_len: u32,
                                    cb: Option<ResponseSliceCB>) -> Error;

    pub fn indy_pack_message(command_handle: CommandHandle,
                             wallet_handle: WalletHandle,
                             message: BString,
                             message_len: u32,
                             receiver_keys: CString,
                             sender: CString,
                             cb: Option<ResponseSliceCB>) -> Error;

    pub fn indy_unpack_message(command_handle: CommandHandle,
                               wallet_handle: WalletHandle,
                               jwe_msg: BString,
                               jwe_len: u32,
                               cb: Option<ResponseSliceCB>) -> Error;
}

