extern crate cxs;
extern crate tempfile;
extern crate libc;
extern crate rand;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
mod utils;
use utils::demo::*;
use utils::timeout::TimeoutUtils;

use self::tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use cxs::api;
use cxs::utils::libindy::pool;
use std::sync::mpsc::channel;
use std::path::Path;
use cxs::utils::error;

#[allow(dead_code)]
static SERIALIZED_CONNECTION: &str = r#"{"source_id":"test_cxs_connection_connect","handle":2608616713,"pw_did":"62LeFLkN9ZeCr32j73PUyD","pw_verkey":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","did_endpoint":"","state":4,"uuid":"","endpoint":"","invite_detail":{"e":"34.210.228.152:80","rid":"6oHwpBN","sakdp":"key","sn":"enterprise","sD":"62LeFLkN9ZeCr32j73PUyD","lu":"https://s19.postimg.org/ykyz4x8jn/evernym.png","sVk":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","tn":"there"}}"#;
#[allow(dead_code)]
static SERIALIZED_CLAIM: &str = r#"{"source_id":"Claim For Driver's License","handle":3664805180,"claim_attributes":"{\"age\":[\"28\",\"28\"],\"height\":[\"175\",\"175\"],\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]}","msg_uid":"7TKyPLr","schema_seq_no":12,"issuer_did":"Niaxv2v4mPr1HdTeJkQxuU","issued_did":"62LeFLkN9ZeCr32j73PUyD","state":2,"claim_request":null}"#;
static CLAIM_DATA: &str = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
static CLAIM_DEF_ISSUER_DID: &str = "2hoqvcwupRTUNkXn6ArYzs";
static CLAIM_DEF_SCHEMA_SEQ_NUM: u32 = 15;

fn sandbox_pool_setup() {
    let node_txns = vec![
        r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
        r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
        r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
        r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
    let pool_name = "PoolForDemo";
    let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
    let nodes_count = 4;
    let pool_name = "PoolForDemo";
    let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
    let txn_file_path = "/tmp/PoolForDemo.txn";
    pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
    assert_eq!(pool::pool_config_json(Path::new(txn_file_path)),config_string);
    assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));

}

pub fn open_sandbox_pool() {
    sandbox_pool_setup();
}

#[test]
fn test_demo(){
    use std::env;
    match env::var("RUST_TEST_DEMO"){
        Ok(_) => demo(),
        Err(_) => {},
    }
}

fn demo(){
    let serialize_connection_fn = api::connection::cxs_connection_serialize;
    let serialize_claim_fn = api::issuer_claim::cxs_issuer_claim_serialize;
    let invite_details = api::connection::cxs_connection_invite_details;

    let random_int: u32 = rand::random();
    let log_url = format!("https://robohash.org/{}?set=set3", random_int);

    // Init SDK  *********************************************************************
    let config_string: String = json!({"agent_endpoint":"https://enym-eagency.pdev.evernym.com",
    "agency_pairwise_did":"Ab8TvZa3Q19VNkQVzAWVL7",
    "agency_pairwise_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf",
    "agent_pairwise_did":"9f9juYT9NHdnewYZwbY9ra",
    "agent_pairwise_verkey":"5igySWcXEuiDrArBbVFWgFMp1uYuNUngnSMBD3aNWdWw",
    "enterprise_did_agent":"5bJqPo8aCWyBwLQosZkJcB",
    "agent_enterprise_verkey":"3W9WGtRowAanh5q6giQrGncZVMvRwPedB9fJAJkAN5Gk",
    "enterprise_did_agent":"5bJqPo8aCWyBwLQosZkJcB",
    "enterprise_name":"enterprise",
    "wallet_name":"my_real_wallet",
    "genesis_path":"/tmp/PoolForDemo.txn",
    "logo_url":log_url
    }).to_string();

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    open_sandbox_pool();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));

    // Creating a Trustee DID -> sufficient permissions to create ClaimDef
//    let (trustee_did, trustee_verkey) = signus::SignusUtils::create_and_store_my_did(get_wallet_handle(), Some(r#"{"seed":"000000000000000000000000Trustee1"}"#))?;
//    let (issuer_did, issuer_verkey) = signus::SignusUtils::create_and_store_my_did(get_wallet_handle(), Some(r#"{"seed":"000000000000000000000000Issuer01"}"#))?;

    // Create Claim Offer ***************************************************************
    let source_id = "Name and Sex";
    let claim_name = "Name and Sex";
    let claim_data:serde_json::Value = serde_json::from_str(CLAIM_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did = CLAIM_DEF_ISSUER_DID.clone();
    let ledger_schema_seq_num = CLAIM_DEF_SCHEMA_SEQ_NUM;
    let (err, claim_handle) = create_claim_offer(claim_name, source_id, claim_data, ledger_issuer_did, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(claim_handle>0);

    // Create Proof **************************************************************
    let requested_attrs = json!([
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address1",
          "issuer_did":ledger_issuer_did
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address2",
          "issuer_did":ledger_issuer_did
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"city",
          "issuer_did":ledger_issuer_did
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"state",
          "issuer_did":ledger_issuer_did
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"zip",
          "issuer_did":ledger_issuer_did
       }
    ]).to_string();
    let (err, proof_handle) = create_proof_request(source_id, requested_attrs.as_str());
    assert_eq!(err, 0);
    assert!(proof_handle>0);

    // Create Connection **************************************************************
    let (sender, receiver) = channel();
    let cb = Box::new(move | err, con_hand| {
        sender.send((err, con_hand)).unwrap();
    });
    let (command_handle, create_connection_cb) = closure_to_create_connection_cb(cb);
    #[allow(unused_variables)]
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    #[allow(unused_variables)]
    let claim_data = CString::new("{\"claim\":\"attributes\"}").unwrap();
    //    let issuer_did_cstring = CString::new(issuer_did).unwrap();
    let rc = api::connection::cxs_connection_create(
        command_handle,CString::new("test_cxs_connection_connect").unwrap().into_raw(),create_connection_cb);
    assert_eq!(rc,0);
    let (err, connection_handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("Connection Handle: {}", connection_handle);
    assert_eq!(err, 0);
    assert!(connection_handle > 0);
    // Connect ************************************************************************
    let (sender, receiver) = channel();
    let (command_handle, cb) = closure_to_connect_cb(Box::new(move|err|{sender.send(err).unwrap();}));
//    let pphone_number = "8014710072";
//    let lphone_number = "8017900625";
//    let phone_number = "8017170266";
//    let connection_opt = json!({"phone":phone_number});
    let connection_opt = String::from("");
    let rc = api::connection::cxs_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new(connection_opt.to_string()).unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(utils::timeout::TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_cxs_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    let err = invite_details_cxs_object(connection_handle, invite_details);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::cxs_connection_update_state);
    assert_eq!(connection_state, 4);

    // update claim *******************************************************************
    let target_claim_state = 1;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::cxs_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);



    // Send Claim Offer ***************************************************************
    println!("ABOUT TO SEND CLAIM OFFER");
    std::thread::sleep(Duration::from_millis(5000));
    let err = send_claim_offer(claim_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_cxs_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize claim ****************************************************************
    let err = serialize_cxs_object(claim_handle, serialize_claim_fn);
    assert_eq!(err,0);

    receive_request_send_claim(connection_handle,claim_handle);

    send_proof_request_and_receive_proof(connection_handle, proof_handle);
}

fn receive_request_send_claim(connection_handle: u32, claim_handle:u32){

    // update claim *******************************************************************
    let target_claim_state = 3;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::cxs_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);


    // Send claim *********************************************************************
    let err = utils::demo::send_claim(claim_handle, connection_handle);
    assert_eq!(err, 0);
}

fn send_proof_request_and_receive_proof(connection_handle: u32, proof_handle:u32){
    let target_proof_state = 1;
    let state = wait_for_updated_state(proof_handle, target_proof_state, api::proof::cxs_proof_update_state);
    assert_eq!(target_proof_state, state);
    let target_state = 4;

    // Send Proof Request *************************************************************
    let err = utils::demo::send_proof_request(proof_handle, connection_handle);
    assert_eq!(err, 0);

    let state = wait_for_updated_state(proof_handle, target_state, api::proof::cxs_proof_update_state);

    assert_eq!(state, target_state);

    // Receive Proof
    let err = utils::demo::get_proof(proof_handle, connection_handle);
    assert_eq!(err, 0);
}

#[allow(dead_code)]
fn init_sdk(){
    // Init SDK  *********************************************************************
    let issuer_did = "TCwEv4tiAuA5DfC7VTdu83";
    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
        \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
        \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
        \"enterprise_did_agency\":\"{}\",\
        \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
        \"enterprise_name\":\"enterprise\",\
        \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
        \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
        \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", "https://agency-ea-sandbox.evernym.com",
                                issuer_did);
    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();
    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));

    // deserialize connection *********************************************************
    let serialized_connection = SERIALIZED_CONNECTION;
    let connection_handle = deserialize_cxs_object(serialized_connection, api::connection::cxs_connection_deserialize);
    assert!(connection_handle>0);

    // deserialize claim **************************************************************
    let claim_handle = deserialize_cxs_object(SERIALIZED_CLAIM, api::issuer_claim::cxs_issuer_claim_deserialize);
    assert!(claim_handle>0);
}