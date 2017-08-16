extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::signus::{
    indy_sign,
    indy_create_and_store_my_did,
    indy_store_their_did,
    indy_replace_keys,
    indy_verify_signature,
    indy_encrypt,
    indy_decrypt
};
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct SignusUtils {}

impl SignusUtils {
    pub fn sign(wallet_handle: i32, their_did: &str, msg: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, signature| {
            sender.send((err, signature)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_sign_cb(cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_sign(command_handle,
                      wallet_handle,
                      their_did.as_ptr(),
                      msg.as_ptr() as *const u8,
                      msg.len() as u32,
                      cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, signature) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(signature)
    }

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String, String), ErrorCode> {
        let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
        let create_and_store_my_did_cb = Box::new(move |err, did, verkey, public_key| {
            create_and_store_my_did_sender.send((err, did, verkey, public_key)).unwrap();
        });
        let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));

        let my_did_json = CString::new(my_did_json).unwrap();

        let err =
            indy_create_and_store_my_did(create_and_store_my_did_command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         create_and_store_my_did_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, my_did, my_verkey, my_pk) = create_and_store_my_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok((my_did, my_verkey, my_pk))
    }

    pub fn create_my_did(wallet_handle: i32, my_did_json: &str) -> Result<(String, String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, did, verkey, public_key| {
            sender.send((err, did, verkey, public_key)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_and_store_my_did_cb(cb);

        let my_did_json = CString::new(my_did_json).unwrap();

        let err =
            indy_create_and_store_my_did(command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_did, my_verkey, my_pk) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((my_did, my_verkey, my_pk))
    }

    pub fn store_their_did(wallet_handle: i32, identity_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_store_their_did_cb(cb);

        let identity_json = CString::new(identity_json).unwrap();


        let err =
            indy_store_their_did(command_handle,
                                 wallet_handle,
                                 identity_json.as_ptr(),
                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_pk: &str, their_verkey: &str, endpoint: &str) -> Result<(), ErrorCode> {
        let (store_their_did_sender, store_their_did_receiver) = channel();
        let store_their_did_cb = Box::new(move |err| { store_their_did_sender.send((err)).unwrap(); });
        let (store_their_did_command_handle, store_their_did_callback) = CallbackUtils::closure_to_store_their_did_cb(store_their_did_cb);

        let their_identity_json = format!("{{\"did\":\"{}\",\
                                            \"pk\":\"{}\",\
                                            \"verkey\":\"{}\",\
                                            \"endpoint\":\"{}\"\
                                           }}",
                                          their_did, their_pk, their_verkey, endpoint);

        let their_identity_json = CString::new(their_identity_json).unwrap();

        let err =
            indy_store_their_did(store_their_did_command_handle,
                                 wallet_handle,
                                 their_identity_json.as_ptr(),
                                 store_their_did_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = store_their_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn replace_keys(wallet_handle: i32, did: &str, identity_json: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, verkey, public_key| {
            sender.send((err, verkey, public_key)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_replace_keys_cb(cb);

        let did = CString::new(did).unwrap();
        let identity_json = CString::new(identity_json).unwrap();

        let err =
            indy_replace_keys(command_handle,
                              wallet_handle,
                              did.as_ptr(),
                              identity_json.as_ptr(),
                              cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_verkey, my_pk) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((my_verkey, my_pk))
    }

    pub fn verify(wallet_handle: i32, pool_handle: i32, did: &str, msg: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid| {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verify_signature_cb(cb);

        let did = CString::new(did).unwrap();

        let err =
            indy_verify_signature(command_handle,
                                  wallet_handle,
                                  pool_handle,
                                  did.as_ptr(),
                                  msg.as_ptr() as *const u8,
                                  msg.len() as u32,
                                  signature.as_ptr() as *const u8,
                                  signature.len() as u32,
                                  cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, valid) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(valid)
    }

    pub fn encrypt(wallet_handle: i32, pool_handle: i32, my_did: &str, did: &str, msg: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, encrypted_msg, nonce| {
            sender.send((err, encrypted_msg, nonce)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_encrypt_cb(cb);

        let my_did = CString::new(my_did).unwrap();
        let did = CString::new(did).unwrap();

        let err =
            indy_encrypt(command_handle,
                         wallet_handle,
                         pool_handle,
                         my_did.as_ptr(),
                         did.as_ptr(),
                         msg.as_ptr() as *const u8,
                         msg.len() as u32,
                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, encrypted_msg, nonce) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((encrypted_msg, nonce))
    }

    pub fn decrypt(wallet_handle: i32, my_did: &str, did: &str, encrypted_msg: &[u8], nonce: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, decrypted_msg| {
            sender.send((err, decrypted_msg)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_decrypt_cb(cb);

        let my_did = CString::new(my_did).unwrap();
        let did = CString::new(did).unwrap();

        let err =
            indy_decrypt(command_handle,
                         wallet_handle,
                         my_did.as_ptr(),
                         did.as_ptr(),
                         encrypted_msg.as_ptr() as *const u8,
                         encrypted_msg.len() as u32,
                         nonce.as_ptr() as *const u8,
                         nonce.len() as u32,
                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, decrypted_msg) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(decrypted_msg)
    }
}