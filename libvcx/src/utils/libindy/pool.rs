extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use std::env;
use std::fs;
use std::io::Write;
use std::ptr::null;
use std::path::{Path, PathBuf};
use utils::error;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{Return_I32, Return_I32_I32, receive};
use utils::json::JsonEncodable;
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use std::sync::RwLock;
use std::time::Duration;
use utils::timeout::TimeoutUtils;
use utils::constants::{POOL, GENESIS_PATH};
use settings;

lazy_static! {
    static ref POOL_HANDLE: RwLock<Option<i32>> = RwLock::new(None);
}

pub fn change_pool_handle(handle: Option<i32>){
    let mut h = POOL_HANDLE.write().unwrap();
    *h = handle;
}


#[derive(Serialize, Deserialize)]
struct PoolConfig {
    pub genesis_txn: String
}
impl JsonEncodable for PoolConfig {}

extern {
    fn indy_create_pool_ledger_config(command_handle: i32,
                                      config_name: *const c_char,
                                      config: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_delete_pool_ledger_config(command_handle: i32,
                                      config_name: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_open_pool_ledger(command_handle: i32,
                             config_name: *const c_char,
                             config: *const c_char,
                             cb: Option<extern fn(xcommand_handle: i32, err: i32, pool_handle: i32)>) -> i32;

    fn indy_refresh_pool_ledger(command_handle: i32,
                                handle: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_close_pool_ledger(command_handle: i32,
                              handle: i32,
                              cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_set_protocol_version(command_handle: i32,
                                 version: usize,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}

fn test_pool_ip() -> String { env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string()) }

fn tmp_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("indy_client");
    path
}

fn tmp_file_path(file_name: &str) -> PathBuf {
    let mut path = tmp_path();
    path.push(file_name);
    path
}

pub fn create_genesis_txn_file(pool_name: &str,
                               txn_file_data: &str,
                               txn_file_path: Option<&Path>) -> PathBuf {
    let txn_file_path = txn_file_path.map_or(
        tmp_file_path(format!("/tmp/{}.txn", pool_name).as_str()),
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


// Note that to be a valid config, it assumes that the genesis txt file already exists
pub fn pool_config_json(txn_file_path: &Path) -> String {
    PoolConfig {
        genesis_txn: txn_file_path.to_string_lossy().to_string()
    }
        .to_json()
        .unwrap()
}

pub fn set_protocol_version() -> u32 {
    let rtn_obj = match Return_I32::new() {
        Ok(x) => x,
        Err(x) => return x as u32,
    };

    unsafe {
        let rc = indy_set_protocol_version(rtn_obj.command_handle,
                                           2,
                                           Some(rtn_obj.get_callback()));

        if rc != 0 {
            error!("indy_set_protocol_version returned: {}", rc);
            return error::UNKNOWN_LIBINDY_ERROR.code_num;
        }
        match receive(&rtn_obj.receiver, TimeoutUtils::some_long()) {
            Ok(_) => {
                if rc != 0 {
                    println!("indy_set_protocol_version returned: {}", rc);
                    error::UNKNOWN_LIBINDY_ERROR.code_num
                } else {
                    0
                }
            },
            Err(_) => error::UNKNOWN_LIBINDY_ERROR.code_num,
        }
    }
}

pub fn create_pool_ledger_config(pool_name: &str, path: Option<&Path>) -> Result<u32, u32> {
    let pool_config = match path {
        Some(c) => pool_config_json(c),
        None => return Err(error::INVALID_GENESIS_TXN_PATH.code_num)
    };

    let pool_name = CString::new(pool_name).map_err(map_string_error)?;
    let pool_config = CString::new(pool_config).map_err(map_string_error)?;

    let rtn_obj = Return_I32::new()?;

    unsafe {
        let rc = indy_create_pool_ledger_config(rtn_obj.command_handle,
                                                 pool_name.as_ptr(),
                                                 pool_config.as_ptr(),
                                                 Some(rtn_obj.get_callback()));

        if rc != 306 && rc != 0 {
            println!("libindy create pool returned: {}", rc);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }
        match receive(&rtn_obj.receiver, TimeoutUtils::some_long()) {
            Ok(_) => {
                if rc != 306 && rc != 0 {
                    println!("libindy create pool returned: {}", rc);
                    return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
                }
                Ok(0)
            }
            Err(rc) => return Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
        }
    }
}

pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<u32, u32> {

    set_protocol_version();

    let pool_name = CString::new(pool_name).map_err(map_string_error)?;
    let pool_config = match config {
        Some(str) => Some(CString::new(str).map_err(map_string_error)?),
        None => None
    };
    let rtn_obj = Return_I32_I32::new()?;

    unsafe {
        indy_function_eval(indy_open_pool_ledger(rtn_obj.command_handle,
                                pool_name.as_ptr(),
                                match pool_config {
                                    Some(str) => str.as_ptr(),
                                    None => null()
                                },
                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(|handle|{
        change_pool_handle(Some(handle));
        Ok(handle as u32)
    })
}

pub fn call(pool_handle: i32, timeout: Option<Duration>, func: unsafe extern "C" fn(i32, i32, Option<extern "C" fn(i32, i32)>) -> i32) -> Result<(), u32> {
    let rtn_obj = Return_I32::new()?;
    unsafe {
        indy_function_eval(func(rtn_obj.command_handle,
                                pool_handle,
                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(timeout)
}

pub fn refresh(pool_handle: i32) -> Result<(), u32> {
    call(pool_handle,
         TimeoutUtils::some_long(),
         indy_refresh_pool_ledger)
}

pub fn close() -> Result<(), u32> {
    let handle = get_pool_handle()?;
    change_pool_handle(None);
    call(handle,
         TimeoutUtils::some_long(),
         indy_close_pool_ledger)
}

pub fn delete(pool_name: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        change_pool_handle(None);
        return Ok(())
    }

    let pool_name = CString::new(pool_name).map_err(map_string_error)?;

    let rtn_obj = Return_I32::new()?;

    unsafe {
        indy_function_eval(
            indy_delete_pool_ledger_config(rtn_obj.command_handle,
                                           pool_name.as_ptr(),
                                           Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None)
}

pub fn get_pool_handle() -> Result<i32, u32> {
    let h = POOL_HANDLE.read().unwrap();
    if h.is_none() {
        Err(error::NO_POOL_OPEN.code_num)
    }
    else {
        Ok(h.unwrap())
    }
}

fn sandbox_pool_setup() {
    let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", POOL);
    let test_pool_ip = "127.0.0.1".to_string();

    let node_txns = vec![
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

    let txn_file_data = node_txns[0..4].join("\n");
    create_genesis_txn_file(POOL, &txn_file_data, Some(Path::new(GENESIS_PATH)));
    pool_config_json(Path::new(GENESIS_PATH));
    assert_eq!(create_pool_ledger_config(POOL, Some(Path::new(GENESIS_PATH))),Ok(error::SUCCESS.code_num));
}

pub fn open_sandbox_pool() -> u32 {
    sandbox_pool_setup();
    let config = r#"{"refresh_on_open": true}"#;
    open_pool_ledger(POOL, Some(config)).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_open_close_pool() {
        let wallet_name = "test_open_close_pool";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        assert!(get_pool_handle().unwrap() > 0);
        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }
}
