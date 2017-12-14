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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ClaimDef {
    #[serde(rename = "ref")]
    schema_seq_no: u32,
    origin: String,
    signature_type: String,
    data: ClaimData,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ClaimData {
    primary: Option<PrimaryData>,
    revocation: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct PrimaryData {
    n: String,
    s: String,
    rms: String,
    r: String,
    rctxt: String,
    z: String,
}

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

    let pool_handle = pool::open_pool_ledger(&settings::CONFIG_POOL_NAME, None).unwrap();

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
mod tests {
    use std::path::{Path};
    use super::*;

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"australia","client_ip":"52.64.96.160","client_port":"9702","node_ip":"52.64.96.160","node_port":"9701","services":["VALIDATOR"]},"dest":"UZH61eLH3JokEwjMWQoCMwB3PMD6zRBvG6NCv5yVwXz","identifier":"3U8HUen8WcgpbnEz1etnai","txnId":"c585f1decb986f7ff19b8d03deba346ab8a0494cc1e4d69ad9b8acb0dfbeab6f","type":"0"}"#,
            r#"{"data":{"alias":"brazil","client_ip":"54.233.203.241","client_port":"9702","node_ip":"54.233.203.241","node_port":"9701","services":["VALIDATOR"]},"dest":"2MHGDD2XpRJohQzsXu4FAANcmdypfNdpcqRbqnhkQsCq","identifier":"G3knUCmDrWd1FJrRryuKTw","txnId":"5c8f52ca28966103ff0aad98160bc8e978c9ca0285a2043a521481d11ed17506","type":"0"}"#,
            r#"{"data":{"alias":"canada","client_ip":"52.60.207.225","client_port":"9702","node_ip":"52.60.207.225","node_port":"9701","services":["VALIDATOR"]},"dest":"8NZ6tbcPN2NVvf2fVhZWqU11XModNudhbe15JSctCXab","identifier":"22QmMyTEAbaF4VfL7LameE","txnId":"408c7c5887a0f3905767754f424989b0089c14ac502d7f851d11b31ea2d1baa6","type":"0"}"#,
            r#"{"data":{"alias":"england","client_ip":"52.56.191.9","client_port":"9702","node_ip":"52.56.191.9","node_port":"9701","services":["VALIDATOR"]},"dest":"DNuLANU7f1QvW1esN3Sv9Eap9j14QuLiPeYzf28Nub4W","identifier":"NYh3bcUeSsJJcxBE6TTmEr","txnId":"d56d0ff69b62792a00a361fbf6e02e2a634a7a8da1c3e49d59e71e0f19c27875","type":"0"}"#,
            r#"{"data":{"alias":"korea","client_ip":"52.79.115.223","client_port":"9702","node_ip":"52.79.115.223","node_port":"9701","services":["VALIDATOR"]},"dest":"HCNuqUoXuK9GXGd2EULPaiMso2pJnxR6fCZpmRYbc7vM","identifier":"U38UHML5A1BQ1mYh7tYXeu","txnId":"76201e78aca720dbaf516d86d9342ad5b5d46f5badecf828eb9edfee8ab48a50","type":"0"}"#,
            r#"{"data":{"alias":"singapore","client_ip":"13.228.62.7","client_port":"9702","node_ip":"13.228.62.7","node_port":"9701","services":["VALIDATOR"]},"dest":"Dh99uW8jSNRBiRQ4JEMpGmJYvzmF35E6ibnmAAf7tbk8","identifier":"HfXThVwhJB4o1Q1Fjr4yrC","txnId":"51e2a46721d104d9148d85b617833e7745fdbd6795cb0b502a5b6ea31d33378e","type":"0"}"#,
            r#"{"data":{"alias":"virginia","client_ip":"34.225.215.131","client_port":"9702","node_ip":"34.225.215.131","node_port":"9701","services":["VALIDATOR"]},"dest":"EoGRm7eRADtHJRThMCrBXMUM2FpPRML19tNxDAG8YTP8","identifier":"SPdfHq6rGcySFVjDX4iyCo","txnId":"0a4992ea442b53e3dca861deac09a8d4987004a8483079b12861080ea4aa1b52","type":"0"}"#];
        let pool_name = settings::CONFIG_POOL_NAME;
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 7;
        let pool_name = settings::CONFIG_POOL_NAME;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool_name.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        assert_eq!(pool::pool_config_json(Path::new(txn_file_path)),config_string);
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));

    }

    fn open_sandbox_pool() -> u32 {
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
        sandbox_pool_setup();
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
        sandbox_pool_setup();
        assert!(init_wallet(&settings::CONFIG_WALLET_NAME).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let claim_def = get_claim_def_from_ledger(0,
                                                  "GGBDg1j8bsKmr4h5T9XqYf",
                                                  15,
                                                  "CL",
                                                  "4fUDR9R7fjwELRvH9JT6HH").unwrap();

        assert!(&claim_def.contains("\"identifier\":\"GGBDg1j8bsKmr4h5T9XqYf\",\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"ref\":15"));
    }


}