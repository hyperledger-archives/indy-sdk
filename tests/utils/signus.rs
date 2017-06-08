use std::sync::mpsc::{channel};
use std::ffi::{CString};

use sovrin::api::signus::{
    sovrin_sign,
    sovrin_create_and_store_my_did,
    sovrin_store_their_did,
    sovrin_replace_keys,
    sovrin_verify_signature
};
use sovrin::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct SignusUtils {}

impl SignusUtils {
    pub fn sign(wallet_handle: i32, their_did: &str, msg: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, signature| {
            sender.send((err, signature)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_sign_cb(cb);

        let their_did = CString::new(their_did).unwrap();
        let msg = CString::new(msg).unwrap();

        let err =
            sovrin_sign(command_handle,
                        wallet_handle,
                        their_did.as_ptr(),
                        msg.as_ptr(),
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

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<String>) -> Result<(String, String, String), ErrorCode> {
        let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
        let create_and_store_my_did_cb = Box::new(move |err, did, verkey, public_key| {
            create_and_store_my_did_sender.send((err, did, verkey, public_key)).unwrap();
        });
        let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
        let err =
            sovrin_create_and_store_my_did(create_and_store_my_did_command_handle,
                                           wallet_handle,
                                           CString::new(my_did_json).unwrap().as_ptr(),
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
            sovrin_create_and_store_my_did(command_handle,
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
            sovrin_store_their_did(command_handle,
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
        let err =
            sovrin_store_their_did(store_their_did_command_handle,
                                   wallet_handle,
                                   CString::new(their_identity_json).unwrap().as_ptr(),
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
            sovrin_replace_keys(command_handle,
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

    pub fn verify(wallet_handle: i32, pool_handle: i32, did: &str, signed_msg: &str) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid| {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verify_signature_cb(cb);

        let did = CString::new(did).unwrap();
        let signed_msg = CString::new(signed_msg).unwrap();

        let err =
            sovrin_verify_signature(command_handle,
                                    wallet_handle,
                                    pool_handle,
                                    did.as_ptr(),
                                    signed_msg.as_ptr(),
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
}