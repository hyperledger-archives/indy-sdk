extern crate vcx;
extern crate libc;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
mod utils;
use utils::demo::*;

use ::vcx::error::base::BaseError;
use ::vcx::utils::libindy::{ pool, wallet, signus, anoncreds, ledger};
use ::vcx::settings;
use ::vcx::utils::logger::LoggerUtils;
use std::time::Duration;
use std::ffi::CString;
use vcx::api;
use vcx::utils::timeout::TimeoutUtils;
use std::sync::mpsc::channel;
use ::vcx::schema::SchemaData;
use vcx::issuer_credential;

static CREDENTIAL_DATA: &str = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
// STAGING is 245, SANDBOX is 36, DEV is 22
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM: u32 = 22;

fn create_and_open_pool(pool_name:&str, config_file_path: &str) -> Result<u32, BaseError> {
    let config_file_path = std::path::Path::new(config_file_path);
    assert_eq!(pool::create_pool_ledger_config(pool_name, Some(config_file_path)).unwrap(), 0);
    pool::open_pool_ledger(pool_name, Some(pool_name)).or(Err(BaseError::GeneralError()))
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

    let wallet_name = "test_demo";
    let serialize_connection_fn = api::connection::vcx_connection_serialize;
    let serialize_credential_fn = api::issuer_credential::vcx_issuer_credential_serialize;
    let invite_details = api::connection::vcx_connection_invite_details;

    self::vcx::utils::logger::LoggerUtils::init();
    // Init DEV ENV  *********************************************************************
    self::vcx::utils::devsetup::setup_dev_env(wallet_name);

    // Create Credential Offer ***************************************************************
    let source_id = "Name and Sex";
    let credential_name = "Name and Sex";
    let credential_data:serde_json::Value = serde_json::from_str(CREDENTIAL_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_schema_seq_num = CREDENTIAL_DEF_SCHEMA_SEQ_NUM;
    //Todo: find out correct cred_def_id we want to use. Will default CRED_DEF_ID work?
    let (err, credential_handle) = create_credential_offer(credential_name, source_id, credential_data, self::vcx::utils::devsetup::INSTITUTION_DID, ::vcx::utils::constants::CRED_DEF_ID);
    assert_eq!(err, 0);
    assert!(credential_handle>0);

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
    //let phone_number = "2053863441";
    //let connection_opt = json!({"phone":phone_number});
    let connection_opt = String::from("");
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

    // update credential *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER");
    std::thread::sleep(Duration::from_millis(5000));
    let err = send_credential_offer(credential_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle, serialize_credential_fn);
    assert_eq!(err,0);

    receive_request_send_credential(connection_handle,credential_handle);

    send_proof_request_and_receive_proof(connection_handle, proof_handle);
    self::vcx::utils::devsetup::cleanup_dev_env(wallet_name);
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

fn create_and_open_wallet(wallet_name:&str, pool_name: &str) -> Result<i32, BaseError>{
    use ::vcx::utils::libindy::wallet;
    wallet::create_wallet(wallet_name, pool_name, None)
        .or(Err(BaseError::WalletError("Creating Wallet".to_string())))?;
    wallet::open_wallet(wallet_name, None).or(Err(BaseError::WalletError("Opening".to_string())))
}

#[allow(dead_code)]
fn get_and_update_version() -> String {
    let version = format!("{}.0",read_version("/home/mark/version.txt") as u32);
    version
}

// have to use this one because the member attr_names of schema::SchemaData is not an Option, and
// fails on one of the uses of this Schema struct.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Schema {
    #[serde(rename = "seqNo")]
    seq_no: i32,
    dest: String,
    data: SchemaData,
}

#[ignore]
#[test]
fn test_update_version(){
    read_version("/home/mark/version.txt");
}

#[allow(dead_code)]
#[ignore]
#[test]
fn test_libindy_direct(){
    LoggerUtils::init();
    let did_seed = "000000000000000000000000Trustee1";
    //    let did_seed = "000000000000000000000000Issuer02";
    let wallet_key = "libindy";
    let wallet_name = "issuer";
    let wallet_name2 = "prover";
    let pool_name = "libindy_pool";

    let master_secret_alias = "foobar";

    let version = format!("{}.0",read_version("/home/mark/version.txt") as u32);
    let schema_name = "unknown_schema_name";
    let schema_attrs = r#"["name","gpa"]"#;
    let truncated_schema_data = format!(r#"{{"name":"{}", "version":"{}"}}"#, schema_name, version);
    println!("truncated_schema_data: {}", truncated_schema_data);
    //    let config_file_path = std::path::Path::new("/var/lib/indy/verity-dev/pool_transactions_genesis");

    settings::set_config_value("wallet_name", wallet_name);
    settings::set_config_value("wallet_key", wallet_key);
    settings::set_config_value(settings::CONFIG_LINK_SECRET_ALIAS, master_secret_alias);

    let _pool_handle = create_and_open_pool(pool_name, "/home/mark/pool_1.txn").unwrap();
    let wallet_handle = create_and_open_wallet(wallet_name, pool_name).unwrap();

    assert!(wallet_handle > 0);
    let (expected_did, _) = signus::SignusUtils::create_and_store_my_did(wallet_handle, Some(did_seed)).unwrap();
    let schema_id= create_schema_on_ledger(&expected_did, schema_name, &version, schema_attrs).unwrap();

    println!("schema_id: {}", schema_id);

    // get the same schema from the ledger
    let schema_json_from_ledger_request = ::vcx::utils::libindy::ledger::libindy_build_get_schema_request(&expected_did, &schema_id).unwrap();
    println!("schema_json_from_ledger_request: {}", schema_json_from_ledger_request);
    let get_schema_response = ledger::libindy_submit_request(&schema_json_from_ledger_request).unwrap();
    let (schema_id, schema_json) = ledger::libindy_parse_get_schema_response(&get_schema_response).unwrap();
    println!("schema_id {}\nschema_json: {}", schema_id, schema_json);
    // get the same schema back.

    let cred_def_id = create_credential_def(&expected_did,
                                            &schema_json).unwrap();
    // get the cred_def_json
    let (_, cred_def_json) = get_cred_def_json(&cred_def_id);

    println!("cred_def_id: {}", cred_def_id);
    println!("cred_def_json: {}", cred_def_json);
    let credential_offer_string = anoncreds::libindy_issuer_create_credential_offer(&cred_def_id).unwrap();
    println!("credential_offer_string: {}", credential_offer_string);

    assert!(anoncreds::libindy_prover_create_master_secret(&settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap()).is_ok());
    let (credential_request_string, req_meta) = anoncreds::libindy_prover_create_credential_req(&expected_did,
                                                                                              &credential_offer_string,
                                                                                              &cred_def_json,
                                                                                    None).unwrap();
    println!("credential_request_string: {}", credential_request_string);

    let prepped_data = r#"{"name":["frank"],"gpa":["4.0"]}"#.to_string();
    let issuer_credential_handle = issuer_credential::issuer_credential_create(cred_def_id.to_string(),
                                                                               "IssuerCredentialName".to_string(),
                                                                               expected_did.to_string(),
                                                                               "CredentialNameHere".to_string(),
                                                                                prepped_data,
										1).unwrap();

    println!("issuer credential attributes: {}", issuer_credential::get_credential_attributes(issuer_credential_handle).unwrap());

    let encoded_attributes = issuer_credential::get_encoded_attributes(issuer_credential_handle).unwrap();
    println!("Encoded Attributes: {}", encoded_attributes);
    let (issuer_credential, _, _) = anoncreds::libindy_issuer_create_credential(&credential_offer_string, &credential_request_string, &encoded_attributes, None, None).unwrap();
//    let credential = issuer_credential::create_credential_payload_using_wallet("SomeID", &credential_request_string, encoded_attributes, wallet_handle).unwrap();
    println!("issuer_credential: {}", issuer_credential);

    assert!(anoncreds::libindy_prover_store_credential(None, &req_meta, &issuer_credential, &cred_def_json, None).is_ok());

    let proof_req_json = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "name_1": json!({
                   "name":"name",
               }),
               "gpa_2": json!({
                   "name":"gpa",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();

    let prover_credentials = anoncreds::libindy_prover_get_credentials_for_proof_req(&proof_req_json).unwrap();
    let value_of_prover_credential:serde_json::Value = serde_json::from_str(&prover_credentials).unwrap();
    println!("value_of_prover_credential: {}", value_of_prover_credential);
    println!("attrs: {:?}", &value_of_prover_credential.get("attrs").unwrap());
    println!("attr1_referent: {:?}", &value_of_prover_credential.get("attrs").unwrap().get("attr1_referent").unwrap());
    println!("prover_credentials: {}", prover_credentials);
//    let attr1_referent = &value_of_prover_credential.get("attrs").unwrap().get("attr1_referent").unwrap()[0];
//    let referent = &value_of_prover_credential["attrs"]["attr1_referent"];

//    let encoded_attributes = issuer_credential::get_encoded_attributes(issuer_credential_handle).unwrap();
//    println!("Encoded Attributes: {}", encoded_attributes);
//    let (_, issuer_credential) = anoncreds::libindy_issuer_create_credential(wallet_handle, &credential_request_string, &encoded_attributes, -1).unwrap();
////    let credential = issuer_credential::create_credential_payload_using_wallet("SomeID", &credential_request_string, encoded_attributes, wallet_handle).unwrap();
//    println!("issuer_credential: {}", issuer_credential);
//
//    assert!(anoncreds::libindy_prover_store_credential(wallet_handle2, &issuer_credential).is_ok());
//
//    let proof_req_json = format!(r#"{{
//                                   "nonce":"123432421212",
//                                   "name":"proof_req_1",
//                                   "version": "0.1",
//                                   "requested_attrs":{{
//                                        "attr1_referent":{{
//                                            "name":"name",
//                                            "restrictions":[{{"issuer_did":"{}",
//                                                            "schema_key":{{
//                                                                "name":"Faber Student Info",
//                                                                "version":"{}",
//                                                                "did":"{}"
//                                                            }}
//                                            }}]
//                                        }}
//                                   }},
//                                   "requested_predicates":{{}}
//                               }}"#, expected_did, version, expected_did );
//
//    let prover_credentials = anoncreds::libindy_prover_get_credentials(wallet_handle2, &proof_req_json).unwrap();
//    let value_of_prover_credential:serde_json::Value = serde_json::from_str(&prover_credentials).unwrap();
//    println!("value_of_prover_credential: {}", value_of_prover_credential);
//    println!("attrs: {:?}", &value_of_prover_credential.get("attrs").unwrap());
//    println!("attr1_referent: {:?}", &value_of_prover_credential.get("attrs").unwrap().get("attr1_referent").unwrap());
//    println!("prover_credentials: {}", prover_credentials);
//    let attr1_referent = &value_of_prover_credential.get("attrs").unwrap().get("attr1_referent").unwrap()[0];
//    let referent = &value_of_prover_credential.get("attrs").unwrap().get("attr1_referent").unwrap()[0].get("referent").unwrap();
    /*
    println!("referent: {:?}", referent);
    let schema_json = format!(r#"{{{}:{}}}"#, referent.to_string(), serde_json::to_string(&schema).unwrap());
    let credential_def_string = format!(r#"{{{}:{}}}"#, referent, cred_def_string);
    let requested_claims = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":[{},true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, referent);

    let proof = anoncreds::libindy_prover_create_proof(&proof_req_json, &requested_claims, &schema_json, &settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap(), &credential_def_string, Some("{}")).unwrap();



    assert!(anoncreds::libindy_verifier_verify_proof(&proof_req_json, &proof, &schema_json, &credential_def_string,"{}", "{}").unwrap());
    println!("proof: {}", proof);
    */
    assert!(wallet::delete_wallet(wallet_name).is_ok());
    assert!(wallet::delete_wallet(wallet_name2).is_ok());
}

#[allow(dead_code)]
fn read_version(filename:&str)-> i32{
    use std::fs::File;
    use std::io::prelude::*;
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents.pop();
    let mut my_int = contents.parse::<i32>().unwrap();
    my_int = my_int + 1;
    let mut f = File::create(filename).expect("File error");
    f.write_all(my_int.to_string().as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    my_int
}


fn create_credential_def(expected_did: &str, schema_json: &str) -> Result<String, BaseError>{
    // create cred def on ledger
    let (cred_def_id, credential_def_string) = anoncreds::libindy_create_and_store_credential_def(&expected_did, &schema_json, "tag1", None, "{}").unwrap();
    // take this value and...
    use ::vcx::utils::libindy::ledger::libindy_build_create_credential_def_txn;

    // send to create credential def txn
    let create_credential_def_request = libindy_build_create_credential_def_txn(&expected_did,
                                                                                &credential_def_string).unwrap();
    // send the txn
    ledger::libindy_sign_and_submit_request(&expected_did,
                                            &create_credential_def_request)
        .or(Err(BaseError::GeneralError()))?;
    Ok(cred_def_id)
}

#[allow(dead_code)]
fn create_schema_on_ledger(did: &str, name: &str, version: &str, attrs: &str) -> Result<String, u32>{
    let (schema_id, create_schema_json) = ::vcx::utils::libindy::anoncreds::libindy_issuer_create_schema(did, name, version, attrs).unwrap();
    let schema_request = ::vcx::utils::libindy::ledger::libindy_build_schema_request(did, &create_schema_json)?;
    ::vcx::utils::libindy::ledger::libindy_sign_and_submit_request(did, &schema_request).unwrap();
    Ok(schema_id)
}

#[allow(dead_code)]
fn get_cred_def_json(cred_def_id: &str) -> (String, String) {
    let get_cred_def_req = ledger::libindy_build_get_credential_def_txn(cred_def_id).unwrap();
    let get_cred_def_response = ledger::libindy_submit_request(&get_cred_def_req ).unwrap();
    ledger::libindy_parse_get_cred_def_response(&get_cred_def_response).unwrap()
}
