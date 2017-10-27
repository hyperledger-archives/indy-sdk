extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::crypto::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::ledger::LedgerUtils;

pub struct CryptoUtils {}

impl CryptoUtils {

    pub fn create_key(wallet_handle: i32, seed: Option<&str>) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, verkey| {
            sender.send((err, verkey)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_create_key_cb(cb);

        let key_json = seed.map_or("{}".to_string(), |seed| format!(r#"{{"seed":"{}"}}"#, seed));
        let key_json = CString::new(key_json).unwrap();

        let err = indy_create_key(command_handle,
                                  wallet_handle,
                                  key_json.as_ptr(),
                                  callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(verkey)
    }

    pub fn set_key_metadata(wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_store_key_metadata_cb(cb);

        let verkey = CString::new(verkey).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = indy_set_key_metadata(command_handle,
                                        wallet_handle,
                                        verkey.as_ptr(),
                                        metadata.as_ptr(),
                                        callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn get_key_metadata(wallet_handle: i32, verkey: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, metadata| {
            sender.send((err, metadata)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_get_key_metadata_cb(cb);

        let verkey = CString::new(verkey).unwrap();

        let err = indy_get_key_metadata(command_handle,
                                        wallet_handle,
                                        verkey.as_ptr(),
                                        callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, metadata) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(metadata)
    }

    //    pub fn sign_vk(wallet_handle: i32, verkey: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    //        let (sender, receiver) = channel();
    //
    //        let cb = Box::new(move |err, signature| {
    //            sender.send((err, signature)).unwrap();
    //        });
    //
    //        let (command_handle, cb) = CallbackUtils::closure_to_sign_cb(cb);
    //
    //        let verkey = CString::new(verkey).unwrap();
    //
    //        let err =
    //            indy_sign_vk(command_handle,
    //                         wallet_handle,
    //                         verkey.as_ptr(),
    //                         msg.as_ptr() as *const u8,
    //                         msg.len() as u32,
    //                         cb);
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        let (err, signature) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        Ok(signature)
    //    }
    //
    //    pub fn verify_vk(wallet_handle: i32, verkey: &str, msg: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
    //        let (sender, receiver) = channel();
    //
    //        let cb = Box::new(move |err, valid| {
    //            sender.send((err, valid)).unwrap();
    //        });
    //
    //        let (command_handle, cb) = CallbackUtils::closure_to_verify_signature_cb(cb);
    //
    //        let verkey = CString::new(verkey).unwrap();
    //
    //        let err =
    //            indy_verify_vk_signature(command_handle,
    //                                     wallet_handle,
    //                                     verkey.as_ptr(),
    //                                     msg.as_ptr() as *const u8,
    //                                     msg.len() as u32,
    //                                     signature.as_ptr() as *const u8,
    //                                     signature.len() as u32,
    //                                     cb);
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        let (err, valid) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        Ok(valid)
    //    }
    //
    //    pub fn encrypt_vk(wallet_handle: i32, sender_vk: &str, recepient_vk: &str, msg: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ErrorCode> {
    //        let (sender, receiver) = channel();
    //
    //        let cb = Box::new(move |err, encrypted_msg, nonce| {
    //            sender.send((err, encrypted_msg, nonce)).unwrap();
    //        });
    //
    //        let (command_handle, cb) = CallbackUtils::closure_to_encrypt_cb(cb);
    //
    //        let my_did = CString::new(my_did).unwrap();
    //        let did = CString::new(did).unwrap();
    //
    //        let err =
    //            indy_encrypt_vk(command_handle,
    //                            wallet_handle,
    //                            my_did.as_ptr(),
    //                            did.as_ptr(),
    //                            msg.as_ptr() as *const u8,
    //                            msg.len() as u32,
    //                            cb);
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        let (err, encrypted_msg, nonce) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    //
    //        if err != ErrorCode::Success {
    //            return Err(err);
    //        }
    //
    //        Ok((encrypted_msg, nonce))
    //    }
}