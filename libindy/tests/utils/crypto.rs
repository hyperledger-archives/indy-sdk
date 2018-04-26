extern crate libc;

use std::ffi::CString;

use indy::api::crypto::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;

pub struct CryptoUtils {}

impl CryptoUtils {
    pub fn create_key(wallet_handle: i32, seed: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let key_json = seed.map_or("{}".to_string(), |seed| format!(r#"{{"seed":"{}"}}"#, seed));
        let key_json = CString::new(key_json).unwrap();

        let err = indy_create_key(command_handle,
                                  wallet_handle,
                                  key_json.as_ptr(),
                                  cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn set_key_metadata(wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let verkey = CString::new(verkey).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = indy_set_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), metadata.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn get_key_metadata(wallet_handle: i32, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let verkey = CString::new(verkey).unwrap();

        let err = indy_get_key_metadata(command_handle,
                                        wallet_handle,
                                        verkey.as_ptr(),
                                        cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn sign(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_vec_u8();

        let my_vk = CString::new(my_vk).unwrap();

        let err =
            indy_crypto_sign(command_handle,
                             wallet_handle,
                             my_vk.as_ptr(),
                             msg.as_ptr() as *const u8,
                             msg.len() as u32,
                             cb);

        super::results::result_to_vec_u8(err, receiver)
    }

    pub fn verify(their_vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_bool();

        let their_vk = CString::new(their_vk).unwrap();

        let err = indy_crypto_verify(command_handle,
                                     their_vk.as_ptr(),
                                     msg.as_ptr() as *const u8,
                                     msg.len() as u32,
                                     signature.as_ptr() as *const u8,
                                     signature.len() as u32,
                                     cb);

        super::results::result_to_bool(err, receiver)
    }

    pub fn auth_crypt(wallet_handle: i32, my_vk: &str, their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_vec_u8();

        let my_vk = CString::new(my_vk).unwrap();
        let their_vk = CString::new(their_vk).unwrap();

        let err = indy_crypto_auth_crypt(command_handle,
                                         wallet_handle,
                                         my_vk.as_ptr(),
                                         their_vk.as_ptr(),
                                         msg.as_ptr() as *const u8,
                                         msg.len() as u32,
                                         cb);

        super::results::result_to_vec_u8(err, receiver)
    }

    pub fn auth_decrypt(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_vec_u8();

        let my_vk = CString::new(my_vk).unwrap();

        let err =
            indy_crypto_auth_decrypt(command_handle,
                                     wallet_handle,
                                     my_vk.as_ptr(),
                                     msg.as_ptr() as *const u8,
                                     msg.len() as u32,
                                     cb);

        super::results::result_to_string_vec_u8(err, receiver)
    }

    pub fn anon_crypt(their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_vec_u8();

        let their_vk = CString::new(their_vk).unwrap();

        let err =
            indy_crypto_anon_crypt(command_handle,
                                   their_vk.as_ptr(),
                                   msg.as_ptr() as *const u8,
                                   msg.len() as u32,
                                   cb);

        super::results::result_to_vec_u8(err, receiver)
    }

    pub fn anon_decrypt(wallet_handle: i32, my_vk: &str, encrypted_msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_vec_u8();

        let my_vk = CString::new(my_vk).unwrap();

        let err =
            indy_crypto_anon_decrypt(command_handle,
                                     wallet_handle,
                                     my_vk.as_ptr(),
                                     encrypted_msg.as_ptr() as *const u8,
                                     encrypted_msg.len() as u32,
                                     cb);

        super::results::result_to_vec_u8(err, receiver)
    }
}