use nullpay::ErrorCode;

use serde_json::to_string;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::ffi::CString;
use std::ptr::null;
use std::os::raw::c_char;

#[derive(Serialize, Deserialize)]
pub struct PoolConfig {
    pub genesis_txn: String
}

const PROTOCOL_VERSION: usize = 2;

pub fn create_and_open_pool_ledger(pool_name: &str) -> Result<i32, ErrorCode> {
    set_protocol_version(PROTOCOL_VERSION).unwrap();
    let txn_file_path = _create_genesis_txn_file_for_test_pool(pool_name, None, None);
    let pool_config = _pool_config_json(txn_file_path.as_path());
    _create_pool_ledger_config(pool_name, Some(pool_config.as_str()))?;
    _open_pool_ledger(pool_name, None)
}

pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

    let err = unsafe {
        indy_close_pool_ledger(command_handle, pool_handle, cb)
    };

    super::results::result_to_empty(err, receiver)
}

fn _pool_config_json(txn_file_path: &Path) -> String {
    to_string(&PoolConfig {
        genesis_txn: txn_file_path.to_string_lossy().to_string()
    }).unwrap()
}

fn _create_pool_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

    let pool_name = CString::new(pool_name).unwrap();
    let pool_config_str = pool_config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

    let err = unsafe {
        indy_create_pool_ledger_config(command_handle,
                                       pool_name.as_ptr(),
                                       if pool_config.is_some() { pool_config_str.as_ptr() } else { null() },
                                       cb)
    };

    super::results::result_to_empty(err, receiver)
}

fn _create_genesis_txn_file_for_test_pool(pool_name: &str,
                                          nodes_count: Option<u8>,
                                          txn_file_path: Option<&Path>) -> PathBuf {
    let nodes_count = nodes_count.unwrap_or(4);

    let test_pool_ip = super::environment::test_pool_ip();

    let node_txns = vec![
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

    let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");

    _create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

fn _create_genesis_txn_file(pool_name: &str,
                            txn_file_data: &str,
                            txn_file_path: Option<&Path>) -> PathBuf {
    let txn_file_path = txn_file_path.map_or(
        super::environment::tmp_file_path(format!("{}.txn", pool_name).as_str()),
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

fn _open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_i32();

    let pool_name = CString::new(pool_name).unwrap();
    let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

    let err = unsafe {
        indy_open_pool_ledger(command_handle,
                              pool_name.as_ptr(),
                              if config.is_some() { config_str.as_ptr() } else { null() },
                              cb)
    };

    super::results::result_to_int(err, receiver)
}


pub fn set_protocol_version(protocol_version: usize) -> Result<(), ErrorCode> {
    let (receiver, cmd_id, cb) = super::callbacks::_closure_to_cb_ec();

    let err = unsafe { indy_set_protocol_version(cmd_id, protocol_version, cb) };

    super::results::result_to_empty(err, receiver)
}


extern {
    #[no_mangle]
    fn indy_open_pool_ledger(command_handle: i32,
                             config_name: *const c_char,
                             config: *const c_char,
                             cb: Option<extern fn(xcommand_handle: i32,
                                                  err: ErrorCode,
                                                  pool_handle: i32)>) -> ErrorCode;

    #[no_mangle]
    fn indy_create_pool_ledger_config(command_handle: i32,
                                      config_name: *const c_char,
                                      config: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32,
                                                           err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_close_pool_ledger(command_handle: i32,
                                  handle: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_set_protocol_version(command_handle: i32,
                                     protocol_version: usize,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode;
}