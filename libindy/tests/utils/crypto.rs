extern crate futures;

use indy::ErrorCode;
use indy::crypto;
use self::futures::Future;

pub fn create_key(wallet_handle: i32, seed: Option<&str>) -> Result<String, ErrorCode> {
    let key_json = json!({"seed": seed}).to_string();
    crypto::create_key(wallet_handle, Some(&key_json)).wait()
}

pub fn set_key_metadata(wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), ErrorCode> {
    crypto::set_key_metadata(wallet_handle, verkey, metadata).wait()
}

pub fn get_key_metadata(wallet_handle: i32, verkey: &str) -> Result<String, ErrorCode> {
    crypto::get_key_metadata(wallet_handle, verkey).wait()
}

pub fn sign(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    crypto::sign(wallet_handle, my_vk, msg).wait()
}

pub fn verify(their_vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
    crypto::verify(their_vk, msg, signature).wait()
}

pub fn auth_crypt(wallet_handle: i32, my_vk: &str, their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    crypto::auth_crypt(wallet_handle, my_vk, their_vk, msg).wait()
}

pub fn auth_decrypt(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
    crypto::auth_decrypt(wallet_handle, my_vk, msg).wait()
}

pub fn anon_crypt(their_vk: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    crypto::anon_crypt(their_vk, msg).wait()
}

pub fn anon_decrypt(wallet_handle: i32, my_vk: &str, encrypted_msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    crypto::anon_decrypt(wallet_handle, my_vk, encrypted_msg).wait()
}
