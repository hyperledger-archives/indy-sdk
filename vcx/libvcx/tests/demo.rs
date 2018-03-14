extern crate vcx;
extern crate libc;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
mod utils;
use utils::demo::*;
use utils::timeout::TimeoutUtils;

use std::time::Duration;
use std::ffi::CString;
use vcx::api;
use std::sync::mpsc::channel;

static CLAIM_DATA: &str = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
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
    let wallet_name = "test_demo";
    let serialize_connection_fn = api::connection::vcx_connection_serialize;
    let serialize_claim_fn = api::issuer_claim::vcx_issuer_claim_serialize;
    let invite_details = api::connection::vcx_connection_invite_details;

    self::vcx::utils::logger::LoggerUtils::init();
    // Init DEV ENV  *********************************************************************
    self::vcx::utils::devsetup::setup_dev_env(wallet_name);

    // Create Claim Offer ***************************************************************
    let source_id = "Name and Sex";
    let claim_name = "Name and Sex";
    let claim_data:serde_json::Value = serde_json::from_str(CLAIM_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_schema_seq_num = CLAIM_DEF_SCHEMA_SEQ_NUM;
    let (err, claim_handle) = create_claim_offer(claim_name, source_id, claim_data, self::vcx::utils::devsetup::INSTITUTION_DID, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(claim_handle>0);

    // Create Proof **************************************************************
    let requested_attrs = json!([
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address1",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address2",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"city",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"state",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"zip",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
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
    //let phone_number = "2053863441";
    //let connection_opt = json!({"phone":phone_number});
    let connection_opt = String::from("");
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
    self::vcx::utils::devsetup::cleanup_dev_env(wallet_name);
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
