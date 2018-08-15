#![allow(unused_variables)]
#![allow(dead_code)]
#![crate_name = "vcx"]
extern crate serde;
extern crate rand;
extern crate reqwest;
extern crate url;
extern crate openssl;
extern crate rust_indy_sdk as indy;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod utils;
pub mod settings;
pub mod messages;

pub mod api;
pub mod connection;
pub mod issuer_credential;
pub mod credential_request;
pub mod proof;
pub mod schema;
pub mod credential_def;
pub mod proof_compliance;
pub mod error;
pub mod credential;
pub mod object_cache;
pub mod disclosed_proof;

#[allow(unused_imports)]
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
    use ::utils::devsetup::tests::{setup_local_env, set_institution, set_consumer, cleanup_dev_env};

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_delete_connection() {
        let test_name = "test_delete_connection";
        settings::set_defaults();
        ::utils::devsetup::tests::setup_local_env(test_name);
        let alice = connection::build_connection("alice").unwrap();
        connection::delete_connection(alice).unwrap();
        assert!(connection::release(alice).is_err());
        ::utils::devsetup::tests::cleanup_dev_env(test_name);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_real_proof() {
        settings::set_defaults();
	    setup_local_env("test_real_proof");
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();
        // AS INSTITUTION SEND CREDENTIAL OFFER
        println!("creating schema/credential_def and paying fees");
        let (schema_id, _, cred_def_id, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def();
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let credential_offer = issuer_credential::issuer_credential_create(cred_def_id.clone(),
                                                            "1".to_string(),
                                                            institution_did.clone(),
                                                            "credential_name".to_string(),
                                                            credential_data.to_owned(),
                                                            1).unwrap();
        println!("sending credential offer");
        issuer_credential::send_credential_offer(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND CREDENTIAL REQUEST
        set_consumer();
        let credential_offers = credential::get_credential_offer_messages(faber, None).unwrap();
        let offers: Value = serde_json::from_str(&credential_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        let credential = credential::credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, credential::get_state(credential).unwrap());
        println!("sending credential request");
        credential::send_credential_request(credential, faber).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS INSTITUTION SEND CREDENTIAL
        set_institution();
        issuer_credential::update_state(credential_offer).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_credential::get_state(credential_offer).unwrap());
        println!("sending credential");
        issuer_credential::send_credential(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CREDENTIAL
        tests::set_consumer();
        credential::update_state(credential).unwrap();
        println!("storing credential");
        let cred_id = credential::get_credential_id(credential).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, credential::get_state(credential).unwrap());
        // AS INSTITUTION SEND PROOF REQUEST
        tests::set_institution();
        let requested_attrs = json!([
           {
              "name":"address1",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": institution_did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,

              }]
           },
           {
              "name":"address2",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": institution_did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,

              }]
           },
           {
              "name":"city",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": institution_did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,

              }]
           },
           {
              "name":"state",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": institution_did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,

              }]
           },
           {
              "name":"zip",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": institution_did,
                "schema_id": schema_id,
                "cred_def_id": cred_def_id,

              }]
           }
        ]).to_string();

        let proof_req_handle = proof::create_proof("1".to_string(), requested_attrs, "[]".to_string(), "name".to_string()).unwrap();
        println!("sending proof request");
        proof::send_proof_request(proof_req_handle, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND PROOF
        set_consumer();
        let requests = disclosed_proof::get_proof_request_messages(faber, None).unwrap();
        let requests: Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();
        let proof_handle = disclosed_proof::create_proof(::utils::constants::DEFAULT_PROOF_NAME.to_string(), requests).unwrap();
        let selected_credentials : Value = json!({
               "attrs":{
                  "address1":{
                    "cred_info":{
                       "referent": cred_id,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id": schema_id,
                       "cred_def_id": cred_def_id,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "address2":{
                    "cred_info":{
                       "referent": cred_id,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id": schema_id,
                       "cred_def_id": cred_def_id,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "city":{
                    "cred_info":{
                       "referent": cred_id,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id": schema_id,
                       "cred_def_id": cred_def_id,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "state":{
                    "cred_info":{
                       "referent": cred_id,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id": schema_id,
                       "cred_def_id": cred_def_id,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "zip":{
                    "cred_info":{
                       "referent": cred_id,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id": schema_id,
                       "cred_def_id": cred_def_id,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 }
               },
               "predicates":{
               }
            });

        disclosed_proof::generate_proof(proof_handle, selected_credentials.to_string(), "{}".to_string()).unwrap();
        println!("sending proof");
        disclosed_proof::send_proof(proof_handle, faber).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, disclosed_proof::get_state(proof_handle).unwrap());
        thread::sleep(Duration::from_millis(5000));
        // AS INSTITUTION VALIDATE PROOF
        set_institution();
        proof::update_state(proof_req_handle).unwrap();
        assert_eq!(proof::get_proof_state(proof_req_handle).unwrap(), ProofStateType::ProofValidated as u32);
        println!("proof validated!");
        let wallet = ::utils::libindy::payments::get_wallet_token_info().unwrap();
        cleanup_dev_env("test_real_proof");
    }
}
