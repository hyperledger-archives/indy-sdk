use futures::*;
use indyrs::{crypto, IndyError, WalletHandle};

use crate::utils::futures::*;

pub fn auth_crypt(wallet_handle: WalletHandle,
                  sender_vk: &str,
                  recipient_vk: &str,
                  message: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::auth_crypt(wallet_handle, sender_vk, recipient_vk, message)
        .into_box()
}

pub fn auth_decrypt(wallet_handle: WalletHandle,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<dyn Future<Item=(String, Vec<u8>), Error=IndyError>> {
    crypto::auth_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .into_box()
}

pub fn anon_crypt(recipient_vk: &str,
                  message: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::anon_crypt(recipient_vk, message)
        .into_box()
}

pub fn anon_decrypt(wallet_handle: WalletHandle,
                    recipient_vk: &str,
                    encrypted_message: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::anon_decrypt(wallet_handle, recipient_vk, encrypted_message)
        .into_box()
}

#[cfg(test)]
pub fn sign(wallet_handle: WalletHandle,
            signer_vk: &str,
            message: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::sign(wallet_handle, signer_vk, message)
        .into_box()
}

pub fn verify(signer_vk: &str,
              message: &[u8],
              signature: &[u8]) -> Box<dyn Future<Item=bool, Error=IndyError>> {
    crypto::verify(signer_vk, message, signature)
        .into_box()
}

pub fn pack_message(wallet_handle: WalletHandle, sender_vk: Option<&str>, receiver_keys: &str, msg: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::pack_message(wallet_handle, msg, receiver_keys, sender_vk)
        .into_box()
}

pub fn unpack_message(wallet_handle: WalletHandle, msg: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    crypto::unpack_message(wallet_handle, msg)
        .into_box()
}
