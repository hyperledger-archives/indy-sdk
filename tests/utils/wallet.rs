use sovrin::api::ErrorCode;
use sovrin::api::wallet::{
    sovrin_create_wallet,
    sovrin_open_wallet,
    sovrin_wallet_set_seq_no_for_value
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

pub struct WalletUtils {}

impl WalletUtils {
    pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_wallet_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(wallet_name).unwrap();
        let xtype = CString::new(xtype).unwrap();

        let err =
            sovrin_create_wallet(command_handle,
                                 pool_name.as_ptr(),
                                 wallet_name.as_ptr(),
                                 xtype.as_ptr(),
                                 null(),
                                 null(),
                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn open_wallet(wallet_name: &str) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err, handle| {
            sender.send((err, handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_open_wallet_cb(cb);

        let wallet_name = CString::new(wallet_name).unwrap();

        let err =
            sovrin_open_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               null(),
                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, wallet_handle) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(wallet_handle)
    }

    pub fn wallet_set_seq_no_for_value(wallet_handle: i32, claim_def_uuid: &str, claim_def_seq_no: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_wallet_set_seq_no_for_value_cb(cb);

        let claim_def_uuid = CString::new(claim_def_uuid).unwrap();

        let err =
            sovrin_wallet_set_seq_no_for_value(command_handle,
                                               wallet_handle,
                                               claim_def_uuid.as_ptr(),
                                               claim_def_seq_no,
                                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }
}