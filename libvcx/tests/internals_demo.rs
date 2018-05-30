#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    extern crate vcx;

    use super::*;
    use self::vcx::settings;
    use self::vcx::connection;
    use self::vcx::credential;
    use self::vcx::issuer_credential;
    use self::vcx::disclosed_proof;
    use self::vcx::proof;
    use self::vcx::api::VcxStateType;
    use self::vcx::api::ProofStateType;
    use serde_json::Value;
    use std::thread;
    use std::time::Duration;

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_delete_connection() {
        self::vcx::utils::logger::LoggerUtils::init();
        let test_name = "test_delete_connection";
        settings::set_defaults();
        self::vcx::utils::devsetup::setup_dev_env(test_name);
        let alice = connection::build_connection("alice").unwrap();
        connection::delete_connection(alice).unwrap();
        assert!(connection::release(alice).is_err());
        self::vcx::utils::devsetup::cleanup_dev_env(test_name);
    }

    // Ignoring until Dev Agency is updated to libindy 1.4
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_real_proof() {
        self::vcx::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let cred_def_id = vcx::utils::constants::ADDRESS_CRED_DEF_ID.to_string();
        //BE INSTITUTION AND GENERATE INVITE FOR CONSUMER
        self::vcx::utils::devsetup::setup_dev_env("test_real_proof");
        let alice = connection::build_connection("alice").unwrap();
        connection::connect(alice, Some("{}".to_string())).unwrap();
        let details = connection::get_invite_details(alice, true).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        self::vcx::utils::devsetup::be_consumer();
        let faber = connection::build_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, connection::get_state(faber));
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, connection::get_state(alice));
        connection::connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        self::vcx::utils::devsetup::be_institution();
        thread::sleep(Duration::from_millis(2000));
        connection::update_state(alice).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, connection::get_state(alice));
        // AS INSTITUTION SEND CREDENTIAL OFFER
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let credential_offer = issuer_credential::issuer_credential_create(cred_def_id.clone(),
                                                            "1".to_string(),
                                                            settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                            "credential_name".to_string(),
                                                            credential_data.to_owned()).unwrap();
        issuer_credential::send_credential_offer(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND CREDENTIAL REQUEST
        self::vcx::utils::devsetup::be_consumer();
        let credential_offers = credential::get_credential_offer_messages(faber, None).unwrap();
        let offers: Value = serde_json::from_str(&credential_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        let credential = credential::credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, credential::get_state(credential).unwrap());
        credential::send_credential_request(credential, faber).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS INSTITUTION SEND CREDENTIAL
        self::vcx::utils::devsetup::be_institution();
        issuer_credential::update_state(credential_offer);
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_credential::get_state(credential_offer));
        issuer_credential::send_credential(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CREDENTIAL
        self::vcx::utils::devsetup::be_consumer();
        credential::update_state(credential).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, credential::get_state(credential).unwrap());
        // AS INSTITUTION SEND PROOF REQUEST
        self::vcx::utils::devsetup::be_institution();
        let requested_attrs = json!([
           {
              "name":"address1",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID,
                "schema_id": self::vcx::utils::constants::ADDRESS_SCHEMA_ID,
                "cred_def_id": self::vcx::utils::constants::ADDRESS_CRED_DEF_ID,

              }]
           },
           {
              "name":"address2",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID,
                "schema_id": self::vcx::utils::constants::ADDRESS_SCHEMA_ID,
                "cred_def_id": self::vcx::utils::constants::ADDRESS_CRED_DEF_ID,

              }]
           },
           {
              "name":"city",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID,
                "schema_id": self::vcx::utils::constants::ADDRESS_SCHEMA_ID,
                "cred_def_id": self::vcx::utils::constants::ADDRESS_CRED_DEF_ID,

              }]
           },
           {
              "name":"state",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID,
                "schema_id": self::vcx::utils::constants::ADDRESS_SCHEMA_ID,
                "cred_def_id": self::vcx::utils::constants::ADDRESS_CRED_DEF_ID,

              }]
           },
           {
              "name":"zip",
              "restrictions": [{
                "schema_name":"Home Address",
                "issuer_did": self::vcx::utils::devsetup::INSTITUTION_DID,
                "schema_id": self::vcx::utils::constants::ADDRESS_SCHEMA_ID,
                "cred_def_id": self::vcx::utils::constants::ADDRESS_CRED_DEF_ID,

              }]
           }
        ]).to_string();

        let proof_req_handle = proof::create_proof("1".to_string(), requested_attrs, "[]".to_string(), "name".to_string()).unwrap();
        proof::send_proof_request(proof_req_handle, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND PROOF
        self::vcx::utils::devsetup::be_consumer();
        let requests = disclosed_proof::get_proof_request_messages(faber, None).unwrap();
        let requests: Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();
        let proof_handle = disclosed_proof::create_proof(self::vcx::utils::constants::DEFAULT_PROOF_NAME.to_string(), requests).unwrap();
        let selected_credentials : Value = json!({
               "attrs":{
                  "address1_1":{
                    "cred_info":{
                       "referent":vcx::utils::constants::ADDRESS_CRED_ID,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id":vcx::utils::constants::ADDRESS_SCHEMA_ID,
                       "cred_def_id":vcx::utils::constants::ADDRESS_CRED_DEF_ID,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "address2_2":{
                    "cred_info":{
                       "referent":vcx::utils::constants::ADDRESS_CRED_ID,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id":vcx::utils::constants::ADDRESS_SCHEMA_ID,
                       "cred_def_id":vcx::utils::constants::ADDRESS_CRED_DEF_ID,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "city_3":{
                    "cred_info":{
                       "referent":vcx::utils::constants::ADDRESS_CRED_ID,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id":vcx::utils::constants::ADDRESS_SCHEMA_ID,
                       "cred_def_id":vcx::utils::constants::ADDRESS_CRED_DEF_ID,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "state_4":{
                    "cred_info":{
                       "referent":vcx::utils::constants::ADDRESS_CRED_ID,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id":vcx::utils::constants::ADDRESS_SCHEMA_ID,
                       "cred_def_id":vcx::utils::constants::ADDRESS_CRED_DEF_ID,
                       "rev_reg_id":null,
                       "cred_rev_id":null
                    },
                    "interval":null
                 },
                  "zip_5":{
                    "cred_info":{
                       "referent":vcx::utils::constants::ADDRESS_CRED_ID,
                       "attrs":{
                          "address1":"101 Tela Lane",
                          "address2":"101 Wilson Lane",
                          "zip":"87121",
                          "state":"UT",
                          "city":"SLC"
                       },
                       "schema_id":vcx::utils::constants::ADDRESS_SCHEMA_ID,
                       "cred_def_id":vcx::utils::constants::ADDRESS_CRED_DEF_ID,
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
        disclosed_proof::send_proof(proof_handle, faber).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, disclosed_proof::get_state(proof_handle).unwrap());
        thread::sleep(Duration::from_millis(5000));
        // AS INSTITUTION VALIDATE PROOF
        self::vcx::utils::devsetup::be_institution();
        proof::update_state(proof_req_handle);
        assert_eq!(proof::get_proof_state(proof_req_handle), ProofStateType::ProofValidated as u32);
        println!("proof validated!");
        self::vcx::utils::devsetup::cleanup_dev_env("test_real_proof");
    }
}
