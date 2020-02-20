#![cfg_attr(feature = "fatal_warnings", deny(warnings))]
#![crate_name = "vcx"]
//this is needed for some large json macro invocations
#![recursion_limit = "128"]
extern crate serde;
extern crate rand;
extern crate reqwest;
extern crate url;
extern crate openssl;
extern crate indyrs as indy;
extern crate futures;

#[macro_use]
extern crate log;

extern crate libc;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate time;

extern crate regex;

extern crate uuid;

extern crate failure;

extern crate rmp_serde;
extern crate indy_sys;

extern crate base64;

extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate chrono;

#[macro_use]
pub mod utils;
pub mod settings;
#[macro_use]
pub mod messages;

pub mod api;
pub mod connection;
pub mod issuer_credential;
pub mod credential_request;
pub mod proof;
pub mod schema;
pub mod credential_def;
pub mod error;
pub mod credential;
pub mod object_cache;
pub mod disclosed_proof;

pub mod v3;

#[allow(unused_imports)]
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    use super::*;
    use settings;
    use connection;
    use credential;
    use issuer_credential;
    use disclosed_proof;
    use proof;
    use api::VcxStateType;
    use api::ProofStateType;
    use serde_json::Value;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;
    use utils::{
        devsetup::{set_institution, set_consumer},
        constants::{DEFAULT_SCHEMA_ATTRS, TEST_TAILS_FILE},
        get_temp_dir_path
    };
    use utils::devsetup::*;

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_delete_connection() {
        let _setup = SetupLibraryAgencyV1ZeroFees::init();

        let alice = connection::create_connection("alice").unwrap();
        connection::connect(alice, None).unwrap();
        connection::delete_connection(alice).unwrap();
        assert!(connection::release(alice).is_err());
    }



    fn attr_names() -> (String, String, String, String, String) {
        let address1 = "Address1".to_string();
        let address2 = "address2".to_string();
        let city = "CITY".to_string();
        let state = "State".to_string();
        let zip = "zip".to_string();
        (address1, address2, city, state, zip)
    }
    fn requested_attrs(did: &str, schema_id: &str, cred_def_id: &str, from: Option<u64>, to: Option<u64>) -> Value {
        let (address1, address2, city, state, zip) = attr_names();
        json!([
           {
              "name":address1,
               "non_revoked": {"from": from, "to": to},
              "restrictions": [{
                "issuer_did": did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,
               }]
           },
           {
              "name":address2,
               "non_revoked": {"from": from, "to": to},
              "restrictions": [{
                "issuer_did": did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,
               }],
           },
           {
              "name":city,
               "non_revoked": {"from": from, "to": to},
              "restrictions": [{
                "issuer_did": did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,
               }]
           },
           {
              "name":state,
               "non_revoked": {"from": from, "to": to},
              "restrictions": [{
                "issuer_did": did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,
               }]
           },
           {
              "name":zip,
               "non_revoked": {"from": from, "to": to},
              "restrictions": [{
                "issuer_did": did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,
               }]
           }
        ])

    }

    fn send_cred_offer(did: &str, cred_def_handle: u32, connection: u32, credential_data: &str) -> u32 {

        let credential_offer = issuer_credential::issuer_credential_create(cred_def_handle,
                                                                           "1".to_string(),
                                                                           did.to_string(),
                                                                           "credential_name".to_string(),
                                                                           credential_data.to_string(),
                                                                           1).unwrap();
        println!("sending credential offer");
        issuer_credential::send_credential_offer(credential_offer, connection).unwrap();
        thread::sleep(Duration::from_millis(2000));
        credential_offer
    }

    fn send_cred_req(connection: u32) -> u32 {
        set_consumer();
        let credential_offers = credential::get_credential_offer_messages(connection).unwrap();
        let offers: Value = serde_json::from_str(&credential_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        let credential = credential::credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, credential::get_state(credential).unwrap());
        println!("sending credential request");
        credential::send_credential_request(credential, connection).unwrap();
        thread::sleep(Duration::from_millis(2000));
        credential
    }

    fn send_credential(issuer_handle: u32, connection: u32, credential_handle: u32) {
        set_institution();
        issuer_credential::update_state(issuer_handle, None).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_credential::get_state(issuer_handle).unwrap());
        println!("sending credential");
        issuer_credential::send_credential(issuer_handle, connection).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CREDENTIAL
        ::utils::devsetup::set_consumer();
        credential::update_state(credential_handle, None).unwrap();
        thread::sleep(Duration::from_millis(2000));
        println!("storing credential");
        credential::get_credential_id(credential_handle).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, credential::get_state(credential_handle).unwrap());
    }

    fn send_proof_request(connection_handle: u32, requested_attrs: &str, requested_preds: &str, revocation_interval: &str, log_msg: &str) -> (u32, String) {
        let proof_req_handle = proof::create_proof("1".to_string(),
                                                   requested_attrs.to_string(),
                                                   requested_preds.to_string(),
                                                   revocation_interval.to_string(),
                                                   "name".to_string()).unwrap();
        println!("sending proof request {}", log_msg);
        proof::send_proof_request(proof_req_handle, connection_handle).unwrap();
        let req_uuid = proof::get_proof_uuid(proof_req_handle).unwrap();
        thread::sleep(Duration::from_millis(2000));
        (proof_req_handle, req_uuid)
    }

    fn create_proof(connection_handle: u32, msg_uid: &str) -> u32 {
        set_consumer();
        let requests = disclosed_proof::get_proof_request_messages(connection_handle, None).unwrap();
        let requests: Value = serde_json::from_str(&requests).unwrap();
        let mut req = None;
        for _req in requests.as_array().unwrap() {
            if _req["msg_ref_id"] == json!(msg_uid) { req = Some(_req); }
        }
        let requests = serde_json::to_string(req.unwrap()).unwrap();
        disclosed_proof::create_proof(::utils::constants::DEFAULT_PROOF_NAME, &requests).unwrap()
    }

    fn generate_and_send_proof(proof_handle: u32, connection_handle: u32, selected_credentials: Value) {
        set_consumer();
        disclosed_proof::generate_proof(proof_handle, selected_credentials.to_string(), "{}".to_string()).unwrap();
        println!("sending proof");
        disclosed_proof::send_proof(proof_handle, connection_handle).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, disclosed_proof::get_state(proof_handle).unwrap());
        thread::sleep(Duration::from_millis(5000));
    }

    fn default_selected_credentials(proof_handle: u32) -> Value {
        println!("retrieving matching credentials");
        let retrieved_credentials = disclosed_proof::retrieve_credentials(proof_handle).unwrap();
        let matching_credentials: Value = serde_json::from_str(&retrieved_credentials).unwrap();
        let (address1, address2, city, state, zip) = attr_names();
        json!({
               "attrs":{
                  address1.to_string():{"credential": matching_credentials["attrs"][address1][0], "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string()},
                  address2.to_string():{"credential": matching_credentials["attrs"][address2][0], "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string()},
                  city.to_string():{"credential": matching_credentials["attrs"][city][0], "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string()},
                  state.to_string():{"credential": matching_credentials["attrs"][state][0], "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string()},
                  zip.to_string():{"credential": matching_credentials["attrs"][zip][0], "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string()},
               },
               "predicates":{
               }
            })

    }

    fn revoke_credential(issuer_handle: u32, rev_reg_id: Option<String>) {
        // GET REV REG DELTA BEFORE REVOCATION
        let (_, delta, timestamp) = ::utils::libindy::anoncreds::get_rev_reg_delta_json(&rev_reg_id.clone().unwrap(), None, None).unwrap();
        println!("revoking credential");
        ::issuer_credential::revoke_credential(issuer_handle).unwrap();
        let (_, delta_after_revoke, _) = ::utils::libindy::anoncreds::get_rev_reg_delta_json(&rev_reg_id.unwrap(), Some(timestamp + 1), None).unwrap();
        assert_ne!(delta, delta_after_revoke);
    }

    fn _real_proof_demo() {
        let number_of_attributes = 10;

        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();

        // AS INSTITUTION SEND CREDENTIAL OFFER
        println!("creating schema/credential_def and paying fees");
        let mut attrs_list: Value = serde_json::Value::Array(vec![]);
        for i in 1..number_of_attributes {
            attrs_list.as_array_mut().unwrap().push(json!(format!("key{}",i)));
        }
        let attrs_list = attrs_list.to_string();
        let (schema_id, _schema_json, cred_def_id, _cred_def_json, cred_def_handle, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(&attrs_list, false);
        let mut credential_data = json!({});
        for i in 1..number_of_attributes {
            credential_data[format!("key{}", i)] = json!([format!("value{}",i)]);
        }
        let credential_data = credential_data.to_string();
        let credential_offer = send_cred_offer(&institution_did, cred_def_handle, alice, &credential_data);

        // AS CONSUMER SEND CREDENTIAL REQUEST
        let credential = send_cred_req(faber);

        // AS INSTITUTION SEND CREDENTIAL
        send_credential(credential_offer, alice, credential);

        // AS INSTITUTION SEND PROOF REQUEST
        ::utils::devsetup::set_institution();

        let restrictions = json!({ "issuer_did": institution_did, "schema_id": schema_id, "cred_def_id": cred_def_id, });
        let mut attrs: Value = serde_json::Value::Array(vec![]);
        for i in 1..number_of_attributes {
            attrs.as_array_mut().unwrap().push(json!({ "name":format!("key{}", i), "restrictions": [restrictions]}));
        }
        let (proof_req_handle, req_uuid) = send_proof_request(alice, &attrs.to_string(), "[]", "{}", "");

        let proof_handle = create_proof(faber, &req_uuid);
        println!("retrieving matching credentials");
        let retrieved_credentials = disclosed_proof::retrieve_credentials(proof_handle).unwrap();
        let matching_credentials: Value = serde_json::from_str(&retrieved_credentials).unwrap();
        let mut credentials: Value = json!({"attrs":{}, "predicates":{}});

        for i in 1..number_of_attributes {
            credentials["attrs"][format!("key{}", i)] = json!({
                "credential": matching_credentials["attrs"][format!("key{}",i)][0].clone(),
                "tails_file": get_temp_dir_path(TEST_TAILS_FILE).to_str().unwrap().to_string(),
            });
        };
        generate_and_send_proof(proof_handle, faber, credentials);

        // AS INSTITUTION VALIDATE PROOF
        set_institution();
        proof::update_state(proof_req_handle, None).unwrap();
        assert_eq!(proof::get_proof_state(proof_req_handle).unwrap(), ProofStateType::ProofValidated as u32);
        println!("proof validated!");
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_real_proof() {
        let _setup = SetupLibraryAgencyV1ZeroFees::init();

        _real_proof_demo();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_real_proof_with_revocation() {
        let _setup = SetupLibraryAgencyV1ZeroFees::init();

        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();

        // CREATE SCHEMA AND CRED DEF
        println!("creating schema/credential_def and paying fees");
        let attrs_list = json!(["address1", "address2", "city", "state", "zip"]).to_string();
        let (schema_id, _schema_json, cred_def_id, _cred_def_json, cred_def_handle, rev_reg_id) =
            ::utils::libindy::anoncreds::tests::create_and_store_credential_def(&attrs_list, true);

        // AS INSTITUTION SEND CREDENTIAL OFFER
        let (address1, address2, city, state, zip) = attr_names();
        let credential_data = json!({address1: ["123 Main St"], address2: ["Suite 3"], city: ["Draper"], state: ["UT"], zip: ["84000"]}).to_string();
        let credential_offer = send_cred_offer(&institution_did, cred_def_handle, alice, &credential_data);

        // AS CONSUMER SEND CREDENTIAL REQUEST
        let credential = send_cred_req(faber);

        // AS INSTITUTION SEND CREDENTIAL
        send_credential(credential_offer, alice, credential);

        // AS INSTITUTION SEND PROOF REQUEST
        ::utils::devsetup::set_institution();

        let time_before_revocation = time::get_time().sec as u64;
        let mut _requested_attrs = requested_attrs(&institution_did, &schema_id, &cred_def_id, None, Some(time_before_revocation));
        let (proof_req_handle, req_uuid) = send_proof_request(alice, &_requested_attrs.to_string(), "[]", "{}", "");

        //AS Consumer - (Prover) GET PROOF REQ AND ASSOCIATED CREDENTIALS, GENERATE AND SEND PROOF
        let proof_handle = create_proof(faber, &req_uuid);
        let _selected_credentials = default_selected_credentials(proof_handle);
        generate_and_send_proof(proof_handle, faber, _selected_credentials);

        // AS INSTITUTION VALIDATE PROOF
        set_institution();
        proof::update_state(proof_req_handle, None).unwrap();
        assert_eq!(proof::get_proof_state(proof_req_handle).unwrap(), ProofStateType::ProofValidated as u32);
        println!("proof validated!");
        let _wallet = ::utils::libindy::payments::get_wallet_token_info().unwrap();

        // AS INSTITUTION REVOKE CRED
        revoke_credential(credential_offer, rev_reg_id);

        // VERIFIER SEND NEW PROOF REQ, Expected revoked proof
        let requested_time = time::get_time().sec as u64;
        let mut _requested_attrs = requested_attrs(&institution_did, &schema_id, &cred_def_id, None, Some(requested_time));
        _requested_attrs[0]["non_revoked"] = json!({"from": requested_time+1});
        let interval = json!({"from": time::get_time().sec+1}).to_string();
        let (proof_req_handle2, req_uuid2) = send_proof_request(alice, &_requested_attrs.to_string(), "[]", &interval, "- revoked creds");

        //AS Consumer - (Prover) Generate Proof with revoked credentials
        let revoked_proof = create_proof(faber, &req_uuid2);
        let _selected_credentials = default_selected_credentials(revoked_proof);
        generate_and_send_proof(revoked_proof, faber, _selected_credentials);

        // AS INSTITUTION VALIDATE REVOKED PROOF
        set_institution();
        proof::update_state(proof_req_handle2, None).unwrap();
        assert_eq!(proof::get_proof_state(proof_req_handle2).unwrap(), ProofStateType::ProofInvalid as u32);
        println!("proof invalid - revoked!");

        // VERIFIER SENDS PROOF_REQ WITH INTERVAL BEFORE REVOCATION
        let _requested_attrs = requested_attrs(&institution_did, &schema_id, &cred_def_id, None, Some(time_before_revocation));
        let (proof_req_handle3, req_uuid3) = send_proof_request(alice, &_requested_attrs.to_string(), "[]", "{}", "");

        //AS Consumer - (Prover) Generate Proof with revoked credentials but valid interval
        let valid_interval_proof = create_proof(faber, &req_uuid3);
        let _selected_credentials = default_selected_credentials(valid_interval_proof);
        generate_and_send_proof(valid_interval_proof, faber, _selected_credentials);

        // AS INSTITUTION VALIDATE REVOKED PROOF - VALID
        set_institution();
        proof::update_state(proof_req_handle3, None).unwrap();
        assert_eq!(proof::get_proof_state(proof_req_handle3).unwrap(), ProofStateType::ProofValidated as u32);
        println!("proof valid for specified interval!");
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "agency_v2")]
    #[test]
    fn test_real_proof_for_protocol_type_v2() {
        let _setup = SetupLibraryAgencyV2::init();

        _real_proof_demo();
    }
}
