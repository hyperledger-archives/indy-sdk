extern crate cxs;
extern crate tempfile;
extern crate libc;
extern crate mockito;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
mod utils;
use utils::demo::*;
use utils::timeout::TimeoutUtils;
use utils::claim_def_wallet;

use self::tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use cxs::api;
use std::sync::mpsc::channel;
use cxs::utils::wallet::get_wallet_handle;

#[allow(dead_code)]
static SERIALIZED_CONNECTION: &str = r#"{"source_id":"test_cxs_connection_connect","handle":2608616713,"pw_did":"62LeFLkN9ZeCr32j73PUyD","pw_verkey":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","did_endpoint":"","state":4,"uuid":"","endpoint":"","invite_detail":{"e":"34.210.228.152:80","rid":"6oHwpBN","sakdp":"key","sn":"enterprise","sD":"62LeFLkN9ZeCr32j73PUyD","lu":"https://s19.postimg.org/ykyz4x8jn/evernym.png","sVk":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","tn":"there"}}"#;
#[allow(dead_code)]
static SERIALIZED_CLAIM: &str = r#"{"source_id":"Claim For Driver's License","handle":3664805180,"claim_attributes":"{\"age\":[\"28\",\"28\"],\"height\":[\"175\",\"175\"],\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]}","msg_uid":"7TKyPLr","schema_seq_no":12,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f","issued_did":"62LeFLkN9ZeCr32j73PUyD","state":2,"claim_request":null}"#;
static CLAIM_DATA: &str = r#"{"sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"], "name":["Alex","1139481716457488690172217916278103335"], "height":["175","175"], "age":["28","28"] }"#;

static CLAIM_DEF_ISSUER_DID: &str = "V4SGRU86Z58d6TV7PBUe6f";
static CLAIM_DEF_SCHEMA_SEQ_NUM: u32 = 103;




#[ignore]
#[test]
fn test_demo(){
    let serialize_connection_fn = api::connection::cxs_connection_serialize;
    let serialize_claim_fn = api::issuer_claim::cxs_issuer_claim_serialize;

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

    // Create Claim Offer ***************************************************************
    let source_id = "Claim For Driver's License";
    let claim_name = "Driver's License";
    let claim_id = "cCanHnpFAD";
    let claim_data:serde_json::Value = serde_json::from_str(CLAIM_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did = "V4SGRU86Z58d6TV7PBUe6f";
    let ledger_schema_seq_num = 103;
    let (err, claim_handle) = create_claim_offer(claim_name, source_id, claim_data, ledger_issuer_did, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(claim_handle>0);

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
    let rc = api::connection::cxs_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new("{\"phone\":\"8017900625\"}").unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(utils::timeout::TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_cxs_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::cxs_connection_update_state);
    assert_eq!(connection_state, 4);

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

    receive_request_send_claim(connection_handle,claim_handle)
}

fn receive_request_send_claim(connection_handle: u32, claim_handle:u32){

    // update claim *******************************************************************
    let target_claim_state = 3;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::cxs_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);

    // Insert Claim Def ***************************************************************
    insert_claim_def();

    // Send claim *********************************************************************
    let err = utils::demo::send_claim(claim_handle, connection_handle);
    assert_eq!(err, 0);
}

fn insert_claim_def(){
    // init the wallet
    let claim_def_issuer_did= CLAIM_DEF_ISSUER_DID;
    let schema_string = claim_def_wallet::create_default_schema(CLAIM_DEF_SCHEMA_SEQ_NUM);
    claim_def_wallet::put_claim_def_in_wallet(get_wallet_handle(), claim_def_issuer_did, &schema_string); }

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



