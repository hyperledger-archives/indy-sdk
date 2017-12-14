extern crate serde_json;
extern crate libc;

use settings;
use std::sync::mpsc::channel;
use self::libc::c_char;
use std::ffi::CString;
use utils::callback::CallbackUtils;
use utils::pool;
use utils::error;
use utils::timeout::TimeoutUtils;
use utils::wallet::{ init_wallet, get_wallet_handle };

extern {

    fn indy_build_get_claim_def_txn(command_handle: i32,
                                    submitter_did: *const c_char,
                                    xref: i32,
                                    signature_type: *const c_char,
                                    origin: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         request_json: *const c_char)>) -> i32;

    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                           request_result_json: *const c_char)>) -> i32;

}

//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
//struct ClaimDef {
//    #[serde(rename = "ref")]
//    schema_seq_no: u32,
//    origin: String,
//    signature_type: String,
//    data: ClaimData,
//}

//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
//struct ClaimData {
//    primary: Option<PrimaryData>,
//    revocation: Option<String>,
//}
//
//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
//struct PrimaryData {
//    n: String,
//    s: String,
//    rms: String,
//    r: String,
//    rctxt: String,
//    z: String,
//}

pub fn get_claim_def_from_ledger(command_handle: u32,
                                 submitter_did: &str,
                                 schema_num:u32,
                                 signature_type:&str,
                                 issuer_did:&str) -> Result<String, u32> {

    let claim_def_request = build_claim_def_request(command_handle,
                                                    submitter_did,
                                                    schema_num,
                                                    signature_type,
                                                    issuer_did)?;

    let claim_def:serde_json::Value = match send_request_to_ledger(command_handle, &claim_def_request) {
        Ok(x) => {
            match serde_json::from_str(&x) {
                Ok(y) => y,
                Err(_) => return Err(error::INVALID_JSON.code_num)
            }
        },
        Err(y) => {
            warn!("Indy send request for claim_def failed");
            return Err(y)
        },
    };

    info!("Retrieved claim_def from the ledger");
    match serde_json::to_string(&claim_def["result"]) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

fn send_request_to_ledger(command_handle: u32,
                          claim_def_req: &str) -> Result<String, u32> {

//    let pool_handle = pool::open_pool_ledger(&settings::CONFIG_POOL_NAME, None).unwrap();
    let pool_handle = pool::get_pool_handle();

    let (sender, receiver) = channel();
    let cb = Box::new(move |err, valid | {
        sender.send((err, valid)).unwrap();
    });

    let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);
    unsafe {
        let indy_err = indy_submit_request(command_handle,
                                                    pool_handle as i32,
                                                    CString::new(claim_def_req).unwrap().as_ptr(),
                                                    cb);
        if indy_err != 0 {
            return Err(error::INDY_SUBMIT_REQUEST_ERR.code_num)
        }
    }

    let (err, claim_def) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0{
        return Err(error::INDY_SUBMIT_REQUEST_ERR.code_num)
    }
    Ok(claim_def)
}

fn build_claim_def_request(command_handle: u32,
                           submitter_did: &str,
                           schema_num:u32,
                           signature_type:&str,
                           issuer_did:&str) -> Result<String, u32> {
    let (sender, receiver) = channel();
    let cb = Box::new(move |err, valid | {
        sender.send((err, valid)).unwrap();
    });

    let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);
    unsafe {
        let indy_err = indy_build_get_claim_def_txn(command_handle,
                                                  CString::new(submitter_did).unwrap().as_ptr(),
                                                  schema_num as i32,
                                                  CString::new(signature_type).unwrap().as_ptr(),
                                                  CString::new(issuer_did).unwrap().as_ptr(),
                                                  cb);
        if indy_err != 0 {
            return Err(error::BUILD_CLAIM_DEF_REQ_ERR.code_num)
        }
    }

    let (err, claim_def_req) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0{
        return Err(error::BUILD_CLAIM_DEF_REQ_ERR.code_num)
    }
    info!("Created claim_def request");
    Ok(claim_def_req)
}

#[cfg(test)]
pub mod tests {
    use std::path::{Path};
    use super::*;

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
            r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
            r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
            r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
        let pool_name = settings::CONFIG_POOL_NAME;
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 4;
        let pool_name = settings::CONFIG_POOL_NAME;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool_name.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        assert_eq!(pool::pool_config_json(Path::new(txn_file_path)),config_string);
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));

    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = settings::CONFIG_POOL_NAME;
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }

    #[test]
    fn test_open_sandbox_pool() {
        assert_ne!(open_sandbox_pool(), 0);
    }

    #[test]
    fn test_get_claim_def_request() {
        settings::set_defaults();
        let pool_handle = open_sandbox_pool();
        assert!(pool_handle > 0);
        let wallet_name = String::from("wallet1");
        assert!(init_wallet(&wallet_name).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        assert!(wallet_handle > 0);

        let claim_def_req = build_claim_def_request(0,
                                                    "GGBDg1j8bsKmr4h5T9XqYf",
                                                    15,
                                                    "CL",
                                                    "4fUDR9R7fjwELRvH9JT6HH").unwrap();
        assert!(claim_def_req.contains("\"identifier\":\"GGBDg1j8bsKmr4h5T9XqYf\",\"operation\":{\"type\":\"108\",\"ref\":15,\"signature_type\":\"CL\",\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\"}"));
    }

    #[test]
    fn test_get_claim_def_by_send_request_to_ledger() {
        settings::set_defaults();
        open_sandbox_pool();
        assert!(init_wallet(&settings::CONFIG_WALLET_NAME).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let claim_def_req = build_claim_def_request(0,
                                                    "GGBDg1j8bsKmr4h5T9XqYf",
                                                    15,
                                                    "CL",
                                                    "4fUDR9R7fjwELRvH9JT6HH").unwrap();

        let claim_def = send_request_to_ledger(0, &claim_def_req);
        let claim_def_obj: serde_json::Value = serde_json::from_str(&claim_def.unwrap()).unwrap();
        assert_eq!(claim_def_obj["result"]["identifier"], json!("GGBDg1j8bsKmr4h5T9XqYf"));
    }

    #[test]
    fn test_get_claim_def_by_send_request_fails() {
        assert_eq!(send_request_to_ledger(0,""), Err(error::INDY_SUBMIT_REQUEST_ERR.code_num));
    }

    #[test]
    fn test_build_claim_def_req_fails() {
        assert_eq!(build_claim_def_request(0,
                                           "GGBDg1j8bsKmr4h5T9XqYf",
                                           15,
                                           "",
                                           "4fUDR9R7fjwELRvH9JT6HH"), Err(error::BUILD_CLAIM_DEF_REQ_ERR.code_num));
    }

    #[test]
    fn test_get_claim_def() {
        let claim_def_ex = "{\"data\":null,\"identifier\":\"GGBDg1j8bsKmr4h5T9XqYf\",\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"ref\":15,\"reqId\":1513286218510271542,\"seqNo\":null,\"signature_type\":\"CL\",\"type\":\"108\"}";
        settings::set_defaults();
        open_sandbox_pool();
        assert!(init_wallet(&settings::CONFIG_WALLET_NAME).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let claim_def = get_claim_def_from_ledger(0,
                                                  "GGBDg1j8bsKmr4h5T9XqYf",
                                                  15,
                                                  "CL",
                                                  "4fUDR9R7fjwELRvH9JT6HH").unwrap();

        assert!(&claim_def.contains("\"ref\":15"));
        assert!(&claim_def.contains("\"seqNo\":20,\"signature_type\":\"CL\""));
        assert!(&claim_def.contains("\"type\":\"108\""));
    }


}