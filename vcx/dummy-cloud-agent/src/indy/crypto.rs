use indyrs::crypto::Crypto as crypto;
use futures::*;
use super::IndyError;
use utils::futures::*;

pub fn auth_crypt(wallet_handle: i32,
                  sender_vk: &str,
                  recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::auth_crypt(wallet_handle, sender_vk, recipient_vk, message)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn auth_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=(String, Vec<u8>), Error=IndyError>> {
    crypto::auth_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn anon_crypt(recipient_vk: &str,
                  message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::anon_crypt(recipient_vk, message)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn anon_decrypt(wallet_handle: i32,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::anon_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

#[cfg(test)]
pub fn sign(wallet_handle: i32,
            signer_vk: &str,
            message: &[u8]) -> Box<Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::sign(wallet_handle, signer_vk, message)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn verify(signer_vk: &str,
              message: &[u8],
              signature: &[u8]) -> Box<Future<Item=bool, Error=IndyError>> {
    crypto::verify(signer_vk, message, signature)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}