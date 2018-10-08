#![cfg_attr(feature = "fatal_warnings", deny(warnings))]
#![allow(unused_variables)]
#![allow(dead_code)]
#![crate_name = "vcx"]
extern crate serde;
extern crate rand;
extern crate reqwest;
extern crate url;
extern crate openssl;
extern crate rust_libindy_wrapper as indy;

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
    use ::utils::devsetup::tests::{set_institution, set_consumer};
    use utils::constants::DEFAULT_SCHEMA_ATTRS;

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_delete_connection() {
        init!("agency");
        let alice = connection::build_connection("alice").unwrap();
        connection::delete_connection(alice).unwrap();
        assert!(connection::release(alice).is_err());
        teardown!("agency");
    }


    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "sovtoken")]
    #[test]
    fn test_real_proof() {
        let number_of_attributes = 50;
        init!("agency");
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();
        // AS INSTITUTION SEND CREDENTIAL OFFER
        println!("creating schema/credential_def and paying fees");
        let mut attrs_list:Value = serde_json::Value::Array(vec![]);
        for i in 1..number_of_attributes {
            attrs_list.as_array_mut().unwrap().push(json!(format!("key{}",i)));
        }
        let attrs_list = attrs_list.to_string();
        let (schema_id, _, cred_def_id, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(&attrs_list);
        let mut credential_data = json!({});
        for i in 1..number_of_attributes {
            credential_data[format!("key{}",i)] = json!([format!("value{}",i)]);
        }
        let credential_data = credential_data.to_string();
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
        let credential_offers = credential::get_credential_offer_messages(faber).unwrap();
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
        thread::sleep(Duration::from_millis(2000));
        println!("storing credential");
        let cred_id = credential::get_credential_id(credential).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, credential::get_state(credential).unwrap());
        // AS INSTITUTION SEND PROOF REQUEST
        tests::set_institution();

        let address1 = "Address1";
        let address2 = "address2";
        let city = "CITY";
        let state = "State";
        let zip = "zip";
        let restrictions = json!({ "issuer_did": institution_did, "schema_id": schema_id, "cred_def_id": cred_def_id, });
        let mut attrs:Value = serde_json::Value::Array(vec![]);
        for i in 1..number_of_attributes {
            attrs.as_array_mut().unwrap().push(json!({ "name":format!("key{}", i), "restrictions": [restrictions]}));
        }
        let requested_attrs = attrs.to_string();
        let proof_req_handle = proof::create_proof("1".to_string(), requested_attrs, "[]".to_string(), "name".to_string()).unwrap();
        println!("sending proof request");
        proof::send_proof_request(proof_req_handle, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        set_consumer();
        let requests = disclosed_proof::get_proof_request_messages(faber, None).unwrap();
        let requests: Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();
        let proof_handle = disclosed_proof::create_proof(::utils::constants::DEFAULT_PROOF_NAME, &requests).unwrap();
        println!("retrieving matching credentials");
        let retrieved_credentials = disclosed_proof::retrieve_credentials(proof_handle).unwrap();
        let matching_credentials: Value = serde_json::from_str(&retrieved_credentials).unwrap();
        let mut credentials: Value = json!({"attrs":{}, "predicates":{}});
        for i in 1..number_of_attributes {
            credentials["attrs"][format!("key{}",i)] = matching_credentials["attrs"][format!("key{}",i)][0].clone();
        }
        let selected_credentials = credentials.to_string();
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
        teardown!("agency");
    }
}
