extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::pairwise::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use std::ptr::null;

pub struct PairwiseUtils {}

impl PairwiseUtils {
    pub fn pairwise_exists(wallet_handle: i32, their_did: &str) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, exists| {
            sender.send((err, exists)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_pairwise_exists_cb(cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_is_pairwise_exists(command_handle,
                                    wallet_handle,
                                    their_did.as_ptr(),
                                    callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, exists) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(exists)
    }

    pub fn create_pairwise(wallet_handle: i32, their_did: &str, my_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_pairwise_create_cb(cb);

        let their_did = CString::new(their_did).unwrap();
        let my_did = CString::new(my_did).unwrap();
        let metadata_str = metadata.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_create_pairwise(command_handle,
                                 wallet_handle,
                                 their_did.as_ptr(),
                                 my_did.as_ptr(),
                                 if metadata.is_some() { metadata_str.as_ptr() } else { null() },
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

    pub fn get_pairwise(wallet_handle: i32, their_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, pairwise_list| {
            sender.send((err, pairwise_list)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_get_pairwise_cb(cb);

        let their_did = CString::new(their_did).unwrap();

        let err =
            indy_get_pairwise(command_handle,
                              wallet_handle,
                              their_did.as_ptr(),
                              cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pairwise_info_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pairwise_info_json)
    }

    pub fn set_pairwise_metadata(wallet_handle: i32, their_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_set_pairwise_metadata_cb(cb);

        let their_did = CString::new(their_did).unwrap();
        let metadata_str = metadata.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_set_pairwise_metadata(command_handle,
                                       wallet_handle,
                                       their_did.as_ptr(),
                                       if metadata.is_some() { metadata_str.as_ptr() } else { null() },
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