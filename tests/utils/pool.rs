extern crate time;

use sovrin::api::ErrorCode;
use sovrin::api::pool::{sovrin_create_pool_ledger_config};
#[cfg(feature = "local_nodes_pool")]
use sovrin::api::pool::sovrin_open_pool_ledger;
use sovrin::api::ledger::sovrin_submit_request;

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::timeout::TimeoutUtils;

use std::fs;
use std::ffi::CString;
use std::io::Write;
#[cfg(feature = "local_nodes_pool")]
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

    #[cfg(feature="local_nodes_pool")]
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

    pub fn send_request(pool_handle: i32, request: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb_send = Box::new(move |err, resp| {
            sender.send((err, resp)).unwrap();
        });
        let req = CString::new(request).unwrap();
        let (command_handle, callback) = CallbackUtils::closure_to_send_tx_cb(cb_send);

        let err = sovrin_submit_request(command_handle,
                                        pool_handle,
                                        req.as_ptr(),
                                        callback);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, resp) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(resp)
    }

    pub fn create_genesis_txn_file(pool_name: &str) -> PathBuf {
        let path = EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name).as_str());

        if !path.parent().unwrap().exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(path.parent().unwrap()).unwrap();
        }

        let mut f = fs::File::create(path.clone()).unwrap();
        let data = format!("{}\n{}\n{}\n{}\n",
                           "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                           "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.3\",\"client_port\":9704,\"node_ip\":\"10.0.0.3\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                           "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.4\",\"client_port\":9706,\"node_ip\":\"10.0.0.4\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                           "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"10.0.0.5\",\"client_port\":9708,\"node_ip\":\"10.0.0.5\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
        f.write_all(data.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();
        path
    }

    pub fn create_pool_config(pool_name: &str) -> String {
        let txn_file_path = EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name).as_str());
        format!("{{\"genesis_txn\": \"{}\"}}", txn_file_path.to_string_lossy())
    }

    pub fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }
}