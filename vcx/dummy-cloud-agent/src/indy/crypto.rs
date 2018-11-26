use indyrs::{crypto, ErrorCode};
use futures::*;
use utils::futures::*;

pub fn auth_crypt(wallet_handle: i32,
                  sender_vk: &str,
                  recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=ErrorCode>> {
    crypto::auth_crypt(wallet_handle, sender_vk, recipient_vk, message)
        .into_box()
}

pub fn auth_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=(String, Vec<u8>), Error=ErrorCode>> {
    crypto::auth_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .into_box()
}

pub fn anon_crypt(recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=ErrorCode>> {
    crypto::anon_crypt(recipient_vk, message)
        .into_box()
}

pub fn anon_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=ErrorCode>> {
    crypto::anon_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .into_box()
}

#[cfg(test)]
pub fn sign(wallet_handle: i32,
            signer_vk: &str,
            message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=ErrorCode>> {
    crypto::sign(wallet_handle, signer_vk, message)
        .into_box()
}

pub fn verify(signer_vk: &str,
              message: &[u8],
              signature: &[u8]) -> Box<Future<Item=bool, Error=ErrorCode>> {
    crypto::verify(signer_vk, message, signature)
        .into_box()
}