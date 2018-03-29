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

use self::tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use vcx::api;
use vcx::utils::libindy::pool;
use vcx::utils::timeout::TimeoutUtils;
use std::sync::mpsc::channel;

#[allow(dead_code)]
static SERIALIZED_CONNECTION: &str = r#"{"source_id":"test_vcx_connection_connect","handle":2608616713,"pw_did":"62LeFLkN9ZeCr32j73PUyD","pw_verkey":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","did_endpoint":"","state":4,"uuid":"","endpoint":"","invite_detail":{"e":"34.210.228.152:80","rid":"6oHwpBN","sakdp":"key","sn":"enterprise","sD":"62LeFLkN9ZeCr32j73PUyD","lu":"https://s19.postimg.org/ykyz4x8jn/evernym.png","sVk":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","tn":"there"}}"#;
#[allow(dead_code)]
static SERIALIZED_CREDENTIAL: &str = r#"{"source_id":"Credential For Driver's License","handle":3664805180,"credential_attributes":"{\"age\":[\"28\",\"28\"],\"height\":[\"175\",\"175\"],\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]}","msg_uid":"7TKyPLr","schema_seq_no":12,"issuer_did":"Niaxv2v4mPr1HdTeJkQxuU","issued_did":"62LeFLkN9ZeCr32j73PUyD","state":2,"credential_request":null}"#;
static CREDENTIAL_DATA1: &str = r#"{"address1": ["Claim1 address1"], "address2": ["Claim1 address2"], "city": ["Claim1 New York"], "state": ["New York"], "zip": ["888888"]}"#;
static CREDENTIAL_DATA2: &str = r#"{"claim2": ["Claim2 Value"], "a2": ["Claim2 a2"], "b2": ["Claim2 b2"], "c2": ["Claim2 c2"], "d2": ["Claim2 d2"]}"#;
static CREDENTIAL_DATA3: &str = r#"{"claim3": ["Claim3 Value"], "a3": ["Claim3 a3"], "b3": ["Claim3 b3"], "c3": ["Claim3 c3"], "d3": ["Claim3 d3"]}"#;
static CREDENTIAL_DATA4: &str = r#"{"address1": ["Claim4 address1"], "address2": ["Claim4 address2"], "city": ["Claim4 SLC"], "state": ["UT"], "zip": ["222222"]}"#;
#[allow(dead_code)]
static CREDENTIAL_DATA5: &str = r#"{"NewClaim": ["New Claim-Claim5"], "claim5": ["Claim5 Val"], "a5": ["Claim5 a5"], "b5": ["Claim5 b5"], "c5": ["Claim5 c5"], "d5": ["Claim5 d5"]}"#;
static CREDENTIAL_DEF_ISSUER_DID1: &str = "DunkM3x1y7S4ECgSL4Wkru";
static CREDENTIAL_DEF_ISSUER_DID2: &str = "DunkM3x1y7S4ECgSL4Wkru";
static CREDENTIAL_DEF_ISSUER_DID3: &str = "EmapZ8H9S2qPp3JKyfr5z1";
static CREDENTIAL_DEF_ISSUER_DID4: &str = "2hoqvcwupRTUNkXn6ArYzs";
#[allow(dead_code)]
static CREDENTIAL_DEF_ISSUER_DID5: &str = "2hoqvcwupRTUNkXn6ArYzs";
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM1: u32 = 296;
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM2: u32 = 294;
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM3: u32 = 302;
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM4: u32 = 300;

#[test]
fn test_demo_full(){
    use std::env;
    match env::var("RUST_TEST_DEMO_FULL"){
        Ok(_) => demo_full(),
        Err(_) => {},
    }
}

fn demo_full(){
    let serialize_connection_fn = api::connection::vcx_connection_serialize;
    let serialize_credential_fn = api::issuer_credential::vcx_issuer_credential_serialize;
    let invite_details = api::connection::vcx_connection_invite_details;

    let random_int: u32 = rand::random();
    let logo_url = format!("https://robohash.org/{}?set=set3", random_int);

    // Init SDK  *********************************************************************
    let config_string: String = json!({
       "agency_endpoint": "https://enym-eagency.pdev.evernym.com",
       "institution_logo_url":logo_url,
       "sdk_to_remote_verkey": "By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy",
       "institution_did": "2hoqvcwupRTUNkXn6ArYzs",
       "remote_to_sdk_did": "NUHiPAuSi8XoPRPGnECPUo",
       "institution_name":"Evernym",
       "sdk_to_remote_did": "M7uZU89SUdsav7i4hVZtXp",
       "agency_verkey": "4hmBc54YanNhQHTD66u6XDp1NSgQm1BacPFbE7b5gtat",
       "wallet_name": "my_real_wallet",
       "agency_did": "7o2xT9Qtp83cJUJMUBTF3M",
       "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
       "remote_to_sdk_verkey": "Chj1oQYdmbTXKG96Fpo8C2sd6fRrt9UyCrbmuo4vzroK",
       "genesis_path":self::vcx::utils::constants::GENESIS_PATH
      }).to_string();

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    pool::open_sandbox_pool();
    self::vcx::utils::libindy::pool::close().unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::vcx::vcx_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));

    // Creating a Trustee DID -> sufficient permissions to create CredentialDef
//    let (trustee_did, trustee_verkey) = signus::SignusUtils::create_and_store_my_did(get_wallet_handle(), Some(r#"{"seed":"000000000000000000000000Trustee1"}"#))?;
//    let (issuer_did, issuer_verkey) = signus::SignusUtils::create_and_store_my_did(get_wallet_handle(), Some(r#"{"seed":"000000000000000000000000Issuer01"}"#))?;


    //Create New Schema And CredentialDef ******************************************************************
//    let schema_no = create_schema_and_credentialdef();


    // Create Credential Offer1 ***************************************************************
    let source_id = "Credential1";
    let credential_name = "Credential1";
    let credential_data:serde_json::Value = serde_json::from_str(CREDENTIAL_DATA1).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did = CREDENTIAL_DEF_ISSUER_DID1.clone();
    let ledger_schema_seq_num = CREDENTIAL_DEF_SCHEMA_SEQ_NUM1;
    let (err, credential_handle) = create_credential_offer(credential_name, source_id, credential_data, ledger_issuer_did, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(credential_handle>0);

    // Create Credential Offer2 ***************************************************************
    let source_id2 = "Credential2";
    let credential_name2 = "Credential2";
    let credential_data2 = serde_json::from_str(CREDENTIAL_DATA2).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did2 = CREDENTIAL_DEF_ISSUER_DID2.clone();
    let ledger_schema_seq_num2 = CREDENTIAL_DEF_SCHEMA_SEQ_NUM2;
    let (err2, credential_handle2) = create_credential_offer(credential_name2, source_id2, credential_data2, ledger_issuer_did2, ledger_schema_seq_num2);
    assert_eq!(err2, 0);
    assert!(credential_handle2>0);

    // Create Credential Offer3 ***************************************************************
    let source_id3 = "Credential3";
    let credential_name3 = "Credential3";
    let credential_data3 = serde_json::from_str(CREDENTIAL_DATA3).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did3 = CREDENTIAL_DEF_ISSUER_DID3.clone();
    let ledger_schema_seq_num3 = CREDENTIAL_DEF_SCHEMA_SEQ_NUM3;
    let (err3, credential_handle3) = create_credential_offer(credential_name3, source_id3, credential_data3, ledger_issuer_did3, ledger_schema_seq_num3);
    assert_eq!(err3, 0);
    assert!(credential_handle3>0);

    // Create Credential Offer4 ***************************************************************
    let source_id4 = "Credential4";
    let credential_name4 = "Credential4";
    let credential_data4 = serde_json::from_str(CREDENTIAL_DATA4).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did4 = CREDENTIAL_DEF_ISSUER_DID4.clone();
    let ledger_schema_seq_num4 = CREDENTIAL_DEF_SCHEMA_SEQ_NUM4;
    let (err4, credential_handle4) = create_credential_offer(credential_name4, source_id4, credential_data4, ledger_issuer_did4, ledger_schema_seq_num4);
    assert_eq!(err4, 0);
    assert!(credential_handle4>0);

    // Create Credential Offer5 Only if Created Schema and CredentialDef ***************************************************************
//    let source_id5 = "Credential5";
//    let credential_name5 = "Credential5";
//    let credential_data5 = serde_json::from_str(CREDENTIAL_DATA5).unwrap(); // this format will make it easier to modify in the futre
//    let ledger_issuer_did5 = CREDENTIAL_DEF_ISSUER_DID5.clone();
//    let ledger_schema_seq_num5 = schema_no;
//    let (err5, credential_handle5) = create_credential_offer(credential_name5, source_id5, credential_data5, ledger_issuer_did5, ledger_schema_seq_num5);
//    assert_eq!(err5, 0);
//    assert!(credential_handle5>0);

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
          "name":"zip",
          "issuer_did":ledger_issuer_did
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"city",
          "issuer_did":ledger_issuer_did
       },
       {
          "name":"credential2",
       },
       {
          "schema_seq_no":ledger_schema_seq_num2,
          "name":"d2",
          "issuer_did":ledger_issuer_did2
       },
       {
          "schema_seq_no":ledger_schema_seq_num2,
          "name":"b2",
          "issuer_did":ledger_issuer_did2
       },
       {
          "schema_seq_no":ledger_schema_seq_num3,
          "name":"credential3",
          "issuer_did":ledger_issuer_did3
       },
       {
          "schema_seq_no":ledger_schema_seq_num3,
          "name":"b3",
          "issuer_did":ledger_issuer_did3
       },{
          "schema_seq_no":ledger_schema_seq_num3,
          "name":"d3",
          "issuer_did":ledger_issuer_did3
       },
       {
          "name":"a3",
       },
       {
          "name":"c3",
       },
       {
          "schema_seq_no":ledger_schema_seq_num4,
          "name":"state",
          "issuer_did":ledger_issuer_did4
       },
//       {
//          "schema_seq_no":ledger_schema_seq_num5,
//          "name":"a5",
//          "issuer_did":ledger_issuer_did5
//       },
//       {
//          "name":"credential5",
//       },
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
    let credential_data = CString::new("{\"credential\":\"attributes\"}").unwrap();
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
//    let pphone_number = "8014710072";
//    let lphone_number = "8017900625";
//    let phone_number = "3858814106";
    let phone_number = "2053863441";
//    let phone_number = "2182578533";
    let connection_opt = json!({"phone":phone_number});
    //let connection_opt = String::from("");
    let rc = api::connection::vcx_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new(connection_opt.to_string()).unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    let err = invite_details_vcx_object(connection_handle, invite_details);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::vcx_connection_update_state);
    assert_eq!(connection_state, 4);

    // update credential1 *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer1 ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER1");
    std::thread::sleep(Duration::from_millis(1000));
    let err = send_credential_offer(credential_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle, serialize_credential_fn);
    assert_eq!(err,0);

    receive_request_send_credential(connection_handle,credential_handle);
//    std::thread::sleep(Duration::from_millis(3));



    // update credential2 *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle2, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer2 ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER2");
    std::thread::sleep(Duration::from_millis(1000));
    let err = send_credential_offer(credential_handle2, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle2, serialize_credential_fn);
    assert_eq!(err,0);
    receive_request_send_credential(connection_handle,credential_handle2);
//    std::thread::sleep(Duration::from_millis(3));



    // update credential3 *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle3, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer3 ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER3");
    std::thread::sleep(Duration::from_millis(1000));
    let err = send_credential_offer(credential_handle3, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle3, serialize_credential_fn);
    assert_eq!(err,0);
    receive_request_send_credential(connection_handle,credential_handle3);
    std::thread::sleep(Duration::from_millis(3));



    // update credential4 *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle4, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer4 ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER4");
    std::thread::sleep(Duration::from_millis(1000));
    let err = send_credential_offer(credential_handle4, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle4, serialize_credential_fn);
    assert_eq!(err,0);

    receive_request_send_credential(connection_handle,credential_handle4);


    // **** Do only when creating new schema and credentialdef
    // update credential5 *******************************************************************
//    let target_credential_state = 1;
//    let credential_state = wait_for_updated_state(credential_handle5, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
//    assert_eq!(credential_state, target_credential_state);
//
//    // Send Credential Offer5 ***************************************************************
//    println!("ABOUT TO SEND CREDENTIAL OFFER5");
//    std::thread::sleep(Duration::from_millis(1000));
//    let err = send_credential_offer(credential_handle5, connection_handle);
//    assert_eq!(err,0);
//
//    // Serialize again ****************************************************************
//    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
//    assert_eq!(err,0);
//
//    // Serialize credential ****************************************************************
//    let err = serialize_vcx_object(credential_handle5, serialize_credential_fn);
//    assert_eq!(err,0);
//
//    receive_request_send_credential(connection_handle,credential_handle5);


    // Send Proof Request
    send_proof_request_and_receive_proof(connection_handle, proof_handle);
}

fn receive_request_send_credential(connection_handle: u32, credential_handle:u32){

    // update credential *******************************************************************
    let target_credential_state = 3;
    let credential_state = wait_for_updated_state(credential_handle, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);


    // Send credential *********************************************************************
    let err = utils::demo::send_credential(credential_handle, connection_handle);
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

#[allow(dead_code)]
fn create_schema_and_credentialdef() -> u32 {
    let new_schema_data = r#"{"name":"New Credential - Credential5","version":"1.0","attr_names":["NewCredential", "credential5", "a5","b5","c5","d5"]}"#.to_string();

    let source_id = "New Credential - Credential5";
    let credential_name = "New Credential - Credential5";
    let (err, schema_handle, schema_no) = create_schema(source_id, credential_name, &new_schema_data);
    assert_eq!(err, 0);
    assert!(schema_handle > 0);
    assert!(schema_no > 0);
    println!("\nSchema Created with SeqNO: {}\n", schema_no);

    //Create New CredentialDef ******************************************************************
    let (err, schema_handle) = create_credentialdef(source_id, credential_name, schema_no);
    assert_eq!(err, 0);
    assert!(schema_handle > 0);
    println!("\nCredentialDef Created\n");
    schema_handle
}

#[allow(dead_code)]
fn init_sdk(){
    // Init SDK  *********************************************************************
    let config_string = format!("{{\"agency_endpoint\":\"{}\",\
        \"agency_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
        \"remote_to_sdk_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
        \"sdk_to_remote_did\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
        \"institution_name\":\"enterprise\",\
        \"institution_logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
        \"agency_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
        \"remote_to_sdk_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", "https://agency-ea-sandbox.evernym.com");
    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();
    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::vcx::vcx_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));

    // deserialize connection *********************************************************
    let serialized_connection = SERIALIZED_CONNECTION;
    let connection_handle = deserialize_vcx_object(serialized_connection, api::connection::vcx_connection_deserialize);
    assert!(connection_handle>0);

    // deserialize credential **************************************************************
    let credential_handle = deserialize_vcx_object(SERIALIZED_CREDENTIAL, api::issuer_credential::vcx_issuer_credential_deserialize);
    assert!(credential_handle>0);
}
