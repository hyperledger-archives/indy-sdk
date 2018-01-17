extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::thread;
use std::ptr;
use schema;
use settings;

#[no_mangle]
pub extern fn cxs_schema_create(command_handle: u32,
                                source_id: *const c_char,
                                schema_name: *const c_char,
                                schema_data: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);
    let issuer_did = match settings::get_config_value(settings::CONFIG_ENTERPRISE_DID) {
        Ok(x) => x,
        Err(x) => return x
    };
    thread::spawn( move|| {
        let ( rc, handle) = match schema::create_new_schema(source_id,
                                                                 schema_name,
                                                                 issuer_did,
                                                                 schema_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(x) => (x, 0),
        };

        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_schema_serialize(command_handle: u32,
                                   schema_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match schema::to_string(schema_handle) {
            Ok(x) => {
                info!("serializing schema handle: {} with data: {}", schema_handle, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("could not serialize schema handle {}", schema_handle);
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_schema_deserialize(command_handle: u32,
                                     schema_data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);

    thread::spawn( move|| {
        let (rc, handle) = match schema::from_string(&schema_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(x) => (x, 0),
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_schema_release(schema_handle: u32) -> u32 {
    schema::release(schema_handle)
}

#[no_mangle]
pub extern fn cxs_schema_get_sequence_no(command_handle: u32,
                                         schema_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, sequence_no: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let (schema_no, rc) = match schema::get_sequence_num(schema_handle) {
            Ok(x) => (x, error::SUCCESS.code_num),
            Err(x) => (0, x),
        };
        cb(command_handle, rc, schema_no);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_commit(schema_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_schema_get_data(schema_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use api::claim_def::cxs_claimdef_create;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::libindy::pool;
    use utils::signus::SignusUtils;
    use utils::constants::{ MY1_SEED };
    use std::path::{Path};
    use utils::wallet::{init_wallet};

    extern "C" fn create_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, schema_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_schema_serialize(0, schema_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_cb_get_seq_no(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb_get_seq_no");
        assert_eq!(cxs_schema_get_sequence_no(0, schema_handle, Some(get_seq_no_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_schema_and_claimdef_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_schema_and_claimdef_cb");
        let schema_seq_no = schema::get_sequence_num(schema_handle).unwrap();
        println!("created schema with schema_seq_no: {}", schema_seq_no);
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       schema_seq_no,
                                       false,
                                       Some(create_cb)),
                   error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(800));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, schema_str: *const c_char) {
        assert_eq!(err, 0);
        if schema_str.is_null() {
            panic!("schema_str is null");
        }
        check_useful_c_str!(schema_str, ());
        println!("successfully called serialize_cb: {}", schema_str);
    }

    extern "C" fn get_seq_no_cb(handle: u32, err: u32, schema_no: u32) {
        assert_eq!(err, 0);
        assert_eq!(schema_no, 299);
        println!("successfully called get_seq_no_cb: {}", schema_no);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"handle":1,"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        let new = schema::to_string(schema_handle).unwrap();
        assert_eq!(expected, new);
    }

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
            r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
            r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
            r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
        let pool_name = "pool1";
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 4;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool1.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        pool::pool_config_json(Path::new(txn_file_path));
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))), Ok(error::SUCCESS.code_num));
    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = "pool1";
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_cxs_create_schema_success() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_schema_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Schema").unwrap().into_raw(),
                                       CString::new("{}").unwrap().into_raw(),
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    #[test]
    fn test_cxs_create_schema_with_pool() {
        settings::set_defaults();
        open_sandbox_pool();
        let wallet_handle = init_wallet("wallet1").unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, &my_did);
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(cxs_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_schema_and_claimdef_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[ignore]
    #[test]
    fn test_cxs_create_schema_and_create_claimdef_with_pool() {
        settings::set_defaults();
        open_sandbox_pool();
        let wallet_handle = init_wallet("wallet1").unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, &my_did);
        let data = r#"{"name":"test","version":"1.0","attr_names":["name","male","test","test2"]}"#.to_string();
        assert_eq!(cxs_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_schema_and_claimdef_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn test_cxs_schema_serialize() {
        set_default_and_enable_test_mode();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(cxs_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_schema_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = r#"{"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"handle":1,"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        cxs_schema_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_schema_get_schema_no_succeeds() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new("{}").unwrap().into_raw(),
                                     Some(create_cb_get_seq_no)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

    }
}