extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::thread;
use std::ptr;
use claim_def;
use settings;

#[no_mangle]
pub extern fn cxs_claimdef_create(command_handle: u32,
                                  source_id: *const c_char,
                                  claimdef_name: *const c_char,
                                  schema_seq_no: u32,
                                  create_non_revoc: bool,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claimdef_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    let issuer_did = match settings::get_config_value(settings::CONFIG_ENTERPRISE_DID) {
        Ok(x) => x,
        Err(x) => return x
    };
    thread::spawn( move|| {
        let ( rc, handle) = match claim_def::create_new_claimdef(source_id,
                                                                 claimdef_name,
                                                                 schema_seq_no,
                                                                 issuer_did,
                                                                 create_non_revoc) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(x) => (x, 0),
        };

        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_claimdef_serialize(command_handle: u32,
                                     claimdef_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !claim_def::is_valid_handle(claimdef_handle) {
        return error::INVALID_CLAIM_DEF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match claim_def::to_string(claimdef_handle) {
            Ok(x) => {
                info!("serializing claimdef handle: {} with data: {}", claimdef_handle, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("could not serialize claimdef handle {}", claimdef_handle);
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_claimdef_deserialize(command_handle: u32,
                                       claimdef_data: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claimdef_data, error::INVALID_OPTION.code_num);

    thread::spawn( move|| {
        let (rc, handle) = match claim_def::from_string(&claimdef_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(x) => (x, 0),
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_claimdef_release(claimdef_handle: u32) -> u32 {
    claim_def::release(claimdef_handle)
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::libindy::pool;
    use std::path::{Path};
    use utils::wallet::{ init_wallet };

    extern "C" fn create_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn claim_def_on_ledger_err_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, error::CLAIM_DEF_ALREADY_CREATED.code_num);
        println!("successfully called claim_def_on_ledger_err_cb")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_claimdef_serialize(0,claimdef_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, claimdef_str: *const c_char) {
        assert_eq!(err, 0);
        if claimdef_str.is_null() {
            panic!("claimdef is null");
        }
        check_useful_c_str!(claimdef_str, ());
        println!("successfully called serialize_cb: {}", claimdef_str);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = "{\"claim_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"zip\":\"1\",\"address1\":\"7\",\"address2\":\"8\",\"city\":\"6\",\"state\":\"6\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"handle\":1378455216,\"name\":\"NAME\",\"source_id\":\"test id\"}";
        let new = claim_def::to_string(claimdef_handle).unwrap();
        let def1: claim_def::CreateClaimDef = serde_json::from_str(expected).unwrap();
        let def2: claim_def::CreateClaimDef = serde_json::from_str(&new).unwrap();
        assert_eq!(def1,def2);
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
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));
    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = "pool1";
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_cxs_create_claimdef_success() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       false,
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    #[test]
    fn test_cxs_create_claimdef_with_pool() {
        settings::set_defaults();
        open_sandbox_pool();
        let wallet_handle = init_wallet("wallet1").unwrap();
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("qqqqq").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       false,
                                       Some(claim_def_on_ledger_err_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_cxs_create_claimdef_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       0,
                                       false,
                                       Some(create_cb_err)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_claimdef_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       false,
                                       Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_claimdef_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"source_id\":\"test id\",\"claim_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"city\":\"6\",\"address2\":\"8\",\"address1\":\"7\",\"state\":\"6\",\"zip\":\"1\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"handle\":1378455216,\"name\":\"NAME\"}";
        cxs_claimdef_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}
