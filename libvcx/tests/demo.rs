extern crate vcx;
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
use vcx::api;
use std::sync::mpsc::channel;
use vcx::utils::libindy::pool::open_sandbox_pool;

static CLAIM_DATA: &str = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
static CLAIM_DEF_ISSUER_DID: &str = "2hoqvcwupRTUNkXn6ArYzs";
// STAGING is 245, SANDBOX is 36, DEV is 22
static CLAIM_DEF_SCHEMA_SEQ_NUM: u32 = 22;

#[test]
fn test_demo(){
    use std::env;
    match env::var("RUST_TEST_DEMO"){
        Ok(_) => demo(),
        Err(_) => {},
    }
}

fn demo(){
    let serialize_connection_fn = api::connection::vcx_connection_serialize;
    let serialize_claim_fn = api::issuer_claim::vcx_issuer_claim_serialize;
    let invite_details = api::connection::vcx_connection_invite_details;

    let random_int: u32 = rand::random();
    let logo_url = format!("https://robohash.org/{}?set=set3", random_int);

    /*
    // Init STAGING ENV
    let config_string: String = json!({
        "enterprise_verkey": "3W9WGtRowAanh5q6giQrGncZVMvRwPedB9fJAJkAN5Gk",
        "enterprise_name": "Evernym",
        "agency_pairwise_verkey": "z8bokfTeuSGjygosTZmjo9XnHGsRhiTcnhV5Dp6xxsL",
        "agent_endpoint": "https://eas.pstg.evernym.com",
        "agent_pairwise_verkey": "FqBMwDobDQNjHCMyVBJG8hUY4Fq5XcgqqRdj89dTBZwL",
        "agency_pairwise_did": "2opd29fJoE7UJwtVn6QDhd",
        "wallet_name": "my_real_wallet",
        "genesis_path":self::vcx::utils::constants::GENESIS_PATH,
        "logo_url":logo_url,
        "agent_pairwise_did": "UDDEoTTzUG7vmcEq6meesq",
        "enterprise_did": "5bJqPo8aCWyBwLQosZkJcB",
        "agent_enterprise_verkey": "EA7bVoYwFs8Y8aA5oCR5aEbVWZDxUfYmMqHjSLf1D16t",
        "enterprise_did_agent": "R9835umcd7E8sMa9qqxpmq"
    }).to_string();

    // Init SANDBOX ENV  *********************************************************************
    let config_string: String = json!({
        "enterprise_verkey": "D2RTm2HZFBLMhrCp43ZowDMuxVHGUVe49pgM6cHhkj6L",
        "enterprise_name": "Evernym",
        "agency_pairwise_verkey": "BtGUVLVKQGbiLDPz8kg56vjHWjzMmcMsr65cMa2zwCZm",
        "agent_endpoint": "https://agency-ea-sandbox.evernym.com",
        "agent_pairwise_verkey": "En9myzhuLRqLAw6zPwVFaSmiNtVwZDDYazvuBLHD6Hin",
        "agency_pairwise_did": "LyDVoajwQ6skcGSf6DKay9",
        "genesis_path":self::vcx::utils::constants::GENESIS_PATH,
        "logo_url":logo_url,
        "wallet_name":"my_real_wallet",
        "agent_pairwise_did": "SHEzUMX56jxo9BPU8Rsmnc",
        "enterprise_did": "2hoqvcwupRTUNkXn6ArYzs",
        "agent_enterprise_verkey": "9YAUcPGbRgu8GGnsMacT4xwHZJTVQAe1g9xUUj7t1iZh",
        "enterprise_did_agent": "GfW11C5VZT2nddzf7fECVU"
        }).to_string();
    */

    // Init DEV ENV  *********************************************************************
    let config_string: String = json!({
       "agent_endpoint": "https://enym-eagency.pdev.evernym.com",
       "logo_url":logo_url,
       "agent_enterprise_verkey": "By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy",
       "enterprise_did": "2hoqvcwupRTUNkXn6ArYzs",
       "agent_pairwise_did": "NUHiPAuSi8XoPRPGnECPUo",
       "enterprise_name":"Evernym",
       "enterprise_did_agent": "M7uZU89SUdsav7i4hVZtXp",
       "agency_pairwise_verkey": "4hmBc54YanNhQHTD66u6XDp1NSgQm1BacPFbE7b5gtat",
       "wallet_name": "my_real_wallet",
       "agency_pairwise_did": "7o2xT9Qtp83cJUJMUBTF3M",
       "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
       "agent_pairwise_verkey": "Chj1oQYdmbTXKG96Fpo8C2sd6fRrt9UyCrbmuo4vzroK",
       "genesis_path":self::vcx::utils::constants::GENESIS_PATH
      }).to_string();

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    open_sandbox_pool();
    self::vcx::utils::libindy::pool::close().unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::vcx::vcx_init(0,path.as_ptr(),Some(generic_cb));
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
    let rc = api::connection::vcx_connection_create(
        command_handle,CString::new("test_vcx_connection_connect").unwrap().into_raw(),create_connection_cb);
    assert_eq!(rc,0);
    let (err, connection_handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("Connection Handle: {}", connection_handle);
    assert_eq!(err, 0);
    assert!(connection_handle > 0);
    // Connect ************************************************************************
    let (sender, receiver) = channel();
    let (command_handle, cb) = closure_to_connect_cb(Box::new(move|err|{sender.send(err).unwrap();}));
    let phone_number = "2053863441";
    let connection_opt = json!({"phone":phone_number});
//    let connection_opt = String::from("");
    let rc = api::connection::vcx_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new(connection_opt.to_string()).unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(utils::timeout::TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    let err = invite_details_vcx_object(connection_handle, invite_details);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::vcx_connection_update_state);
    assert_eq!(connection_state, 4);

    // update claim *******************************************************************
    let target_claim_state = 1;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::vcx_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);



    // Send Claim Offer ***************************************************************
    println!("ABOUT TO SEND CLAIM OFFER");
    std::thread::sleep(Duration::from_millis(5000));
    let err = send_claim_offer(claim_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize claim ****************************************************************
    let err = serialize_vcx_object(claim_handle, serialize_claim_fn);
    assert_eq!(err,0);

    receive_request_send_claim(connection_handle,claim_handle);

    send_proof_request_and_receive_proof(connection_handle, proof_handle);
}

fn receive_request_send_claim(connection_handle: u32, claim_handle:u32){

    // update claim *******************************************************************
    let target_claim_state = 3;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::vcx_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);


    // Send claim *********************************************************************
    let err = utils::demo::send_claim(claim_handle, connection_handle);
    assert_eq!(err, 0);
}

fn send_proof_request_and_receive_proof(connection_handle: u32, proof_handle:u32){
    let target_proof_state = 1;
    let state = wait_for_updated_state(proof_handle, target_proof_state, api::proof::vcx_proof_update_state);
    assert_eq!(target_proof_state, state);
    let target_state = 4;

    // Send Proof Request *************************************************************
    let err = utils::demo::send_proof_request(proof_handle, connection_handle);
    assert_eq!(err, 0);

    let state = wait_for_updated_state(proof_handle, target_state, api::proof::vcx_proof_update_state);

    assert_eq!(state, target_state);

    // Receive Proof
    let err = utils::demo::get_proof(proof_handle, connection_handle);
    assert_eq!(err, 0);
}
