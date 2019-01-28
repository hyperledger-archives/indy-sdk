extern crate futures;

use indy::IndyError;
use indy::crypto;
use self::futures::Future;

pub fn create_key(wallet_handle: i32, seed: Option<&str>) -> Result<String, IndyError> {
    let key_json = json!({"seed": seed}).to_string();
    crypto::create_key(wallet_handle, Some(&key_json)).wait()
}

pub fn set_key_metadata(wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), IndyError> {
    crypto::set_key_metadata(wallet_handle, verkey, metadata).wait()
}

pub fn get_key_metadata(wallet_handle: i32, verkey: &str) -> Result<String, IndyError> {
    crypto::get_key_metadata(wallet_handle, verkey).wait()
}

pub fn sign(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, IndyError> {
    crypto::sign(wallet_handle, my_vk, msg).wait()
}

pub fn verify(their_vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, IndyError> {
    crypto::verify(their_vk, msg, signature).wait()
}

pub fn auth_crypt(wallet_handle: i32, my_vk: &str, their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, IndyError> {
    crypto::auth_crypt(wallet_handle, my_vk, their_vk, msg).wait()
}

pub fn auth_decrypt(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), IndyError> {
    crypto::auth_decrypt(wallet_handle, my_vk, msg).wait()
}

pub fn anon_crypt(their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, IndyError> {
    crypto::anon_crypt(their_vk, msg).wait()
}

pub fn anon_decrypt(wallet_handle: i32, my_vk: &str, encrypted_msg: &[u8]) -> Result<Vec<u8>, IndyError> {
    crypto::anon_decrypt(wallet_handle, my_vk, encrypted_msg).wait()
}

pub fn pack_message(wallet_handle: i32, message: &[u8], receiver_keys: &str, sender: Option<&str>) -> Result<Vec<u8>, IndyError> {
    crypto::pack_message(wallet_handle, message, receiver_keys, sender).wait()
}

pub fn unpack_message(wallet_handle: i32, jwe: &[u8]) -> Result<Vec<u8>, IndyError> {
    crypto::unpack_message(wallet_handle, jwe).wait()
}