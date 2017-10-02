extern crate time;

use indy::api::ErrorCode;
use indy::api::pool::{indy_create_pool_ledger_config, indy_delete_pool_ledger_config};
#[cfg(feature = "local_nodes_pool")]
use indy::api::pool::{indy_close_pool_ledger, indy_open_pool_ledger, indy_refresh_pool_ledger};
use indy::api::ledger::indy_submit_request;

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::json::JsonEncodable;
use utils::timeout::TimeoutUtils;


use std::fs;
use std::ffi::CString;
use std::io::Write;
#[cfg(feature = "local_nodes_pool")]
use std::ptr::null;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;

#[derive(Serialize, Deserialize)]
struct PoolConfig {
    pub genesis_txn: String
}

impl JsonEncodable for PoolConfig {}

pub struct PoolUtils {}

impl PoolUtils {
    pub fn create_genesis_txn_file(pool_name: &str,
                                   txn_file_data: &str,
                                   txn_file_path: Option<&Path>) -> PathBuf {
        let txn_file_path = txn_file_path.map_or(
            EnvironmentUtils::tmp_file_path(format!("{}.txn", pool_name).as_str()),
            |path| path.to_path_buf());

        if !txn_file_path.parent().unwrap().exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(txn_file_path.parent().unwrap()).unwrap();
        }

        let mut f = fs::File::create(txn_file_path.as_path()).unwrap();
        f.write_all(txn_file_data.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        txn_file_path
    }

    pub fn create_genesis_txn_file_for_test_pool(pool_name: &str,
                                                 nodes_count: Option<u8>,
                                                 txn_file_path: Option<&Path>) -> PathBuf {
        let nodes_count = nodes_count.unwrap_or(4);
        assert!(nodes_count > 0 && nodes_count <= 4);

        let test_pool_ip = EnvironmentUtils::test_pool_ip();

        let node_txns = vec![
            format!("{{\"data\":{{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"{}\",\"client_port\":9702,\"node_ip\":\"{}\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]}},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"{}\",\"client_port\":9704,\"node_ip\":\"{}\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]}},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"{}\",\"client_port\":9706,\"node_ip\":\"{}\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]}},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"{}\",\"client_port\":9708,\"node_ip\":\"{}\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]}},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip)];

        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");

        PoolUtils::create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
    }

    pub fn create_genesis_txn_file_for_test_pool_with_invalid_nodes(pool_name: &str,
                                                                    txn_file_path: Option<&Path>) -> PathBuf {
        let test_pool_ip = EnvironmentUtils::test_pool_ip();

        let node_txns = vec![
            format!("{{\"data\":{{\"client_ip\":\"{}\",\"client_port\":9702,\"node_ip\":\"{}\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]}},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"client_ip\":\"{}\",\"client_port\":9704,\"node_ip\":\"{}\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]}},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"client_ip\":\"{}\",\"client_port\":9706,\"node_ip\":\"{}\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]}},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"client_ip\":\"{}\",\"client_port\":9708,\"node_ip\":\"{}\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]}},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip)];

        let txn_file_data = node_txns.join("\n");
        PoolUtils::create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
    }

    pub fn create_genesis_txn_file_for_test_pool_with_wrong_alias(pool_name: &str,
                                                                  txn_file_path: Option<&Path>) -> PathBuf {
        let test_pool_ip = EnvironmentUtils::test_pool_ip();

        let node_txns = vec![
            format!("{{\"data\":{{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"{}\",\"client_port\":9702,\"node_ip\":\"{}\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]}},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"{}\",\"client_port\":9704,\"node_ip\":\"{}\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]}},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"{}\",\"client_port\":9706,\"node_ip\":\"{}\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]}},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip),
            format!("{{\"data\":{{\"alias\":\"ALIAS_NODE\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"{}\",\"client_port\":9708,\"node_ip\":\"{}\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]}},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}}", test_pool_ip, test_pool_ip)];

        let txn_file_data = node_txns.join("\n");
        PoolUtils::create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
    }

    // Note that to be config valid it assumes genesis txt file is already exists
    pub fn pool_config_json(txn_file_path: &Path) -> String {
        PoolConfig {
            genesis_txn: txn_file_path.to_string_lossy().to_string()
        }
            .to_json()
            .unwrap()
    }

    pub fn create_pool_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_pool_ledger_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let pool_config_str = pool_config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_create_pool_ledger_config(command_handle,
                                                 pool_name.as_ptr(),
                                                 if pool_config.is_some() { pool_config_str.as_ptr() } else { null() },
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

    #[cfg(feature = "local_nodes_pool")]
    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();


        let cb = Box::new(move |err, pool_handle| {
            sender.send((err, pool_handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_open_pool_ledger_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_open_pool_ledger(command_handle,
                                        pool_name.as_ptr(),
                                        if config.is_some() { config_str.as_ptr() } else { null() },
                                        cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, pool_handle) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(pool_handle)
    }

    pub fn create_and_open_pool_ledger(pool_name: &str) -> Result<i32, ErrorCode> {
        let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
        let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
        PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str()))?;
        PoolUtils::open_pool_ledger(pool_name, None)
    }

    pub fn refresh(pool_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (command_handle, cb) = CallbackUtils::closure_to_refresh_pool_ledger_cb(
            Box::new(move |res| sender.send(res).unwrap()));

        let res = indy_refresh_pool_ledger(command_handle, pool_handle, cb);
        if res != ErrorCode::Success {
            return Err(res);
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }

        Ok(())
    }

    pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (command_handle, cb) = CallbackUtils::closure_to_close_pool_ledger_cb(
            Box::new(move |res| sender.send(res).unwrap()));

        let res = indy_close_pool_ledger(command_handle, pool_handle, cb);
        if res != ErrorCode::Success {
            return Err(res);
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res);
        }

        Ok(())
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let (cmd_id, cb) = CallbackUtils::closure_to_delete_pool_ledger_config_cb(Box::new(
            move |res| sender.send(res).unwrap()));

        let pool_name = CString::new(pool_name).unwrap();

        let res = indy_delete_pool_ledger_config(cmd_id, pool_name.as_ptr(), cb);
        if res != ErrorCode::Success {
            return Err(res)
        }
        let res = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
        if res != ErrorCode::Success {
            return Err(res)
        }
        Ok(())
    }

    pub fn send_request(pool_handle: i32, request: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb_send = Box::new(move |err, resp| {
            sender.send((err, resp)).unwrap();
        });
        let req = CString::new(request).unwrap();
        let (command_handle, callback) = CallbackUtils::closure_to_send_tx_cb(cb_send);

        let err = indy_submit_request(command_handle,
                                      pool_handle,
                                      req.as_ptr(),
                                      callback);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, resp) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(resp)
    }

    pub fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }
}