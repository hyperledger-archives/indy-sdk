use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config, sovrin_open_pool_ledger};

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use std::fs;
use std::ffi::CString;
use std::ptr::null;
use std::path::PathBuf;
use std::sync::mpsc::channel;

pub struct PoolUtils {}

impl PoolUtils {
    pub fn create_pool_ledger_config(pool_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_pool_ledger_cb(cb);

        PoolUtils::create_genesis_txn_file(pool_name);
        let pool_config = CString::new(PoolUtils::create_pool_config(pool_name)).unwrap();
        let pool_name = CString::new(pool_name).unwrap();

        let err = sovrin_create_pool_ledger_config(command_handle,
                                                   pool_name.as_ptr(),
                                                   pool_config.as_ptr(),
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

    pub fn open_pool_ledger(pool_name: &str) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err, pool_handle| {
            sender.send((err, pool_handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_open_pool_ledger_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();

        let err = sovrin_open_pool_ledger(command_handle,
                                          pool_name.as_ptr(),
                                          null(),
                                          cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pool_handle) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pool_handle)
    }

    pub fn create_genesis_txn_file(pool_name: &str) -> PathBuf {
        let path = EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name).as_str());

        if !path.parent().unwrap().exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(path.parent().unwrap()).unwrap();
        }

        fs::File::create(path.clone()).unwrap().sync_all().unwrap();
        path
    }

    pub fn create_pool_config(pool_name: &str) -> String {
        let txn_file_path = EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name).as_str());
        format!("{{\"genesis_txn\": \"{}\"}}", txn_file_path.to_string_lossy())
    }
}