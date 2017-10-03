extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::pairwise::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

pub struct PairwiseUtils {}

impl PairwiseUtils {
    pub fn pairwise_exists(wallet_handle: i32, their_did: &str) -> Result<bool, ErrorCode> {
        let (pairwise_exists_sender, pairwise_exists_receiver) = channel();
        let pairwise_exists_cb = Box::new(move |err, exists| {
            pairwise_exists_sender.send((err, exists)).unwrap();
        });
        let (pairwise_exists_command_handle, pairwise_exists_callback) = CallbackUtils::closure_to_pairwise_exists_cb(pairwise_exists_cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_is_pairwise_exists(pairwise_exists_command_handle,
                                    wallet_handle,
                                    their_did.as_ptr(),
                                    pairwise_exists_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, exists) = pairwise_exists_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(exists)
    }

    pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_create_cb(cb);

        let their_did = CString::new(their_did).unwrap();
        let my_did = CString::new(my_did).unwrap();

        let err =
            indy_create_pairwise(command_handle,
                                 wallet_handle,
                                 their_did.as_ptr(),
                                 my_did.as_ptr(),
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

    pub fn list_pairwise(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, pairwise_list| {
            sender.send((err, pairwise_list)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_list_cb(cb);

        let err =
            indy_list_pairwise(command_handle,
                               wallet_handle,
                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pairwise_list) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pairwise_list)
    }

    pub fn pairwise_get_my_did(wallet_handle: i32, their_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, pairwise_list| {
            sender.send((err, pairwise_list)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_get_my_did_cb(cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_pairwise_get_my_did(command_handle,
                                     wallet_handle,
                                     their_did.as_ptr(),
                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_did) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(my_did)
    }

    pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_set_metadata_cb(cb);

        let their_did = CString::new(their_did).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err =
            indy_set_pairwise_metadata(command_handle,
                                       wallet_handle,
                                       their_did.as_ptr(),
                                       metadata.as_ptr(),
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

    pub fn get_pairwise_metadata(wallet_handle: i32, their_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, pairwise_list| {
            sender.send((err, pairwise_list)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_get_metadata_cb(cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_get_pairwise_metadata(command_handle,
                                       wallet_handle,
                                       their_did.as_ptr(),
                                       cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_did) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(my_did)
    }
}