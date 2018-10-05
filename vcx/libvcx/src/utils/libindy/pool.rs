extern crate libc;

use utils::{ error, timeout::TimeoutUtils };
use std::sync::RwLock;
use settings;
use indy::pool::Pool;
use indy::ErrorCode;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;

lazy_static! {
    static ref POOL_HANDLE: RwLock<Option<i32>> = RwLock::new(None);
}

pub fn change_pool_handle(handle: Option<i32>){
    let mut h = POOL_HANDLE.write().unwrap();
    *h = handle;
}

pub fn set_protocol_version() -> u32 {
    match Pool::set_protocol_version(2) {
        Ok(_) => error::SUCCESS.code_num,
        Err(_) => error::UNKNOWN_LIBINDY_ERROR.code_num,
    }
}

pub fn create_pool_ledger_config(pool_name: &str, path: &str) -> Result<(), u32> {
    let pool_config = format!(r#"{{"genesis_txn":"{}"}}"#, path);

    match Pool::create_ledger_config(pool_name, Some(&pool_config)) {
        Ok(_) => Ok(()),
        Err(x) => if x != ErrorCode::PoolLedgerConfigAlreadyExistsError {
            Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        } else {
            Ok(())
        }
    }
}

pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<u32, u32> {

    set_protocol_version();

    match Pool::open_ledger_timeout(pool_name, config, TimeoutUtils::medium_timeout()).map_err(map_rust_indy_sdk_error_code) {
        Ok(x) => {
            change_pool_handle(Some(x));
            Ok(x as u32)
        },
        Err(_) => Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
    }
}

pub fn close() -> Result<(), u32> {
    let handle = get_pool_handle()?;
    change_pool_handle(None);
    Pool::close_timeout(handle, TimeoutUtils::medium_timeout()).map_err(map_rust_indy_sdk_error_code)
}

pub fn delete(pool_name: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        change_pool_handle(None);
        return Ok(())
    }

    Pool::delete(pool_name).map_err(map_rust_indy_sdk_error_code)
}

pub fn get_pool_handle() -> Result<i32, u32> {
    Ok(POOL_HANDLE.read().or(Err(error::NO_POOL_OPEN.code_num))?.ok_or(error::NO_POOL_OPEN.code_num)?)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use utils::constants::{POOL, GENESIS_PATH};

    pub fn delete_test_pool() {
        match delete(POOL) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    pub fn open_sandbox_pool() -> u32 {
        create_genesis_txn_file();
        create_pool_ledger_config(POOL, GENESIS_PATH).unwrap();
        open_pool_ledger(POOL, None).unwrap()
    }

    pub fn create_genesis_txn_file() {
        let test_pool_ip = "127.0.0.1".to_string();

        let node_txns = vec![
            format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
            format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
            format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
            format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

        let txn_file_data = node_txns[0..4].join("\n");

        let mut f = fs::File::create(GENESIS_PATH).unwrap();
        f.write_all(txn_file_data.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_open_close_pool() {
        use super::*;
        init!("ledger");
        assert!(get_pool_handle().unwrap() > 0);
    }
}
