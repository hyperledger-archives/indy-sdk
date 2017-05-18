extern crate time;

use sovrin::api::ErrorCode;
use sovrin::api::signus::{
    sovrin_sign,
    sovrin_create_and_store_my_did,
    sovrin_store_their_did
};

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::timeout::TimeoutUtils;

use std::fs;
use std::ffi::CString;
use std::io::Write;
use std::ptr::null;
use std::path::PathBuf;
use std::sync::mpsc::channel;

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
}