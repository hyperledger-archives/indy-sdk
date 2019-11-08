#[macro_use]
pub mod utils;
pub mod handlers;
pub mod messages;

#[cfg(feature = "aries")]
#[cfg(test)]
mod test {
    use rand;
    use rand::Rng;
    use utils::devsetup::tests::{init_plugin, config_with_wallet_handle};
    use messages::agent_utils::connect_register_provision;
    use utils::libindy::wallet::*;

    struct Faber {
        wallet_name: String,
        wallet_handle: i32,
        connection_handle: u32,
        config: String,
        schema_handle: u32,
        cred_def_handle: u32,
        credential_handle: u32,
        presentation_handle: u32,
    }

    impl Faber {
        fn setup() -> Faber {
            let wallet_name = "faber_wallet";

            let config = json!({
                "agency_url": "http://localhost:8080",
                "agency_did": "VsKV7grR1BUE29mG2Fm2kX",
                "agency_verkey": "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
                "wallet_name": wallet_name,
                "wallet_key": "123",
                "payment_method": "null",
                "enterprise_seed": "000000000000000000000000Trustee1",
                "protocol_type": "2.0",
                "communication_method": "aries"
            }).to_string();

            let config = connect_register_provision(&config).unwrap();

            let config = config_with_wallet_handle(wallet_name, &config);

            Faber {
                config,
                wallet_name: wallet_name.to_string(),
                schema_handle: 0,
                cred_def_handle: 0,
                connection_handle: 0,
                wallet_handle: get_wallet_handle(),
                credential_handle: 0,
                presentation_handle: 0
            }
        }

        fn activate(&self) {
            ::settings::clear_config();
            ::settings::process_config_string(&self.config, false).unwrap();
            set_wallet_handle(self.wallet_handle);
        }

        fn create_schema(&mut self) {
            self.activate();
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let data = r#"["name","date","degree"]"#.to_string();
            let name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
            let version: String = String::from("1.0");

            self.schema_handle = ::schema::create_and_publish_schema("test_schema", did.clone(), name, version, data).unwrap();
        }

        fn create_credential_definition(&mut self) {
            self.activate();

            let schema_id = ::schema::get_schema_id(self.schema_handle).unwrap();
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let name = String::from("degree");
            let tag = String::from("tag");

            self.cred_def_handle = ::credential_def::create_and_publish_credentialdef(String::from("test_cred_def"), name, did.clone(), schema_id, tag, String::from("{}")).unwrap();
        }

        fn create_presentation_request(&self) -> u32 {
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let requested_attrs = json!([
                {"name": "name"},
                {"name": "date"},
                {"name": "degree"}
            ]).to_string();

            ::proof::create_proof(String::from("alice_degree"),
                                  requested_attrs,
                                  json!([]).to_string(),
                                  json!({}).to_string(),
                                  String::from("proof_from_alice")).unwrap()
        }

        fn create_invite(&mut self) -> String {
            self.activate();
            self.connection_handle = ::connection::create_connection("alice").unwrap();
            ::connection::connect(self.connection_handle, None).unwrap();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(2, ::connection::get_state(self.connection_handle));

            ::connection::get_invite_details(self.connection_handle, false).unwrap()
        }

        fn update_state(&self, expected_state: u32) {
            self.activate();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));
        }

        fn offer_credential(&mut self) {
            self.activate();

            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let credential_data = json!({
                "name": "alice",
                "date": "05-2018",
                "degree": "maths",
            }).to_string();

            self.credential_handle = ::issuer_credential::issuer_credential_create(self.cred_def_handle,
                                                                                   String::from("alice_degree"),
                                                                                   did,
                                                                                   String::from("cred"),
                                                                                   credential_data,
                                                                                   0).unwrap();

            ::issuer_credential::send_credential_offer(self.credential_handle, self.connection_handle).unwrap();
            ::issuer_credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(2, ::issuer_credential::get_state(self.credential_handle).unwrap());
        }

        fn send_credential(&self) {
            self.activate();
            ::issuer_credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(4, ::connection::get_state(self.connection_handle)); // TODO: WHY it already sends credential ????

            ::issuer_credential::send_credential(self.credential_handle, self.connection_handle).unwrap();
            ::issuer_credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(4, ::connection::get_state(self.connection_handle));
            assert_eq!(::v3::messages::status::Status::Success.code(), ::v3::handlers::issuance::get_issuer_credential_status(self.credential_handle).unwrap());
        }

        fn request_presentation(&mut self) {
            self.activate();
            self.presentation_handle = self.create_presentation_request();
            assert_eq!(1, ::proof::get_state(self.presentation_handle).unwrap());

            ::proof::send_proof_request(self.presentation_handle, self.connection_handle).unwrap();
            ::proof::update_state(self.presentation_handle, None).unwrap();

            assert_eq!(2, ::proof::get_state(self.presentation_handle).unwrap());
        }

        fn verify_presentation(&self) {
            self.activate();

            ::proof::update_state(self.presentation_handle, None).unwrap();
            assert_eq!(4, ::proof::get_state(self.presentation_handle).unwrap());
            assert_eq!(::v3::messages::status::Status::Success.code(), ::proof::get_proof_state(self.presentation_handle).unwrap());
        }

        fn teardown(&self) {
            self.activate();
            close_wallet().unwrap();
            delete_wallet(&self.wallet_name, None, None, None).unwrap();
        }
    }

    struct Alice {
        wallet_name: String,
        wallet_handle: i32,
        connection_handle: u32,
        config: String,
        credential_handle: u32,
        presentation_handle: u32,
    }

    impl Alice {
        fn setup() -> Alice {
            let wallet_name = "alice_wallet";

            let config = json!({
                "agency_url": "http://localhost:8080",
                "agency_did": "VsKV7grR1BUE29mG2Fm2kX",
                "agency_verkey": "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
                "wallet_name": wallet_name,
                "wallet_key": "123",
                "payment_method": "null",
                "protocol_type": "2.0",
                "communication_method": "aries"
            }).to_string();

            let config = connect_register_provision(&config).unwrap();

            let config = config_with_wallet_handle(&wallet_name, &config);

            Alice {
                config,
                wallet_name: wallet_name.to_string(),
                wallet_handle: get_wallet_handle(),
                connection_handle: 0,
                credential_handle: 0,
                presentation_handle: 0
            }
        }

        fn activate(&self) {
            ::settings::clear_config();
            ::settings::process_config_string(&self.config, false).unwrap();
            set_wallet_handle(self.wallet_handle);
        }

        fn accept_invite(&mut self, invite: &str) {
            self.activate();
            self.connection_handle = ::connection::create_connection_with_invite("faber", invite).unwrap();
            ::connection::connect(self.connection_handle, None).unwrap();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(3, ::connection::get_state(self.connection_handle));
        }

        fn update_state(&self, expected_state: u32) {
            self.activate();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));
        }

        fn accept_offer(&mut self) {
            self.activate();
            let offers = ::credential::get_credential_offer_messages(self.connection_handle).unwrap();
            let offer = ::serde_json::from_str::<Vec<::serde_json::Value>>(&offers).unwrap()[0].clone();
            let offer_json = ::serde_json::to_string(&offer).unwrap();

            self.credential_handle = ::credential::credential_create_with_offer("degree", &offer_json).unwrap();
            assert_eq!(3, ::credential::get_state(self.credential_handle).unwrap());

            ::credential::send_credential_request(self.credential_handle, self.connection_handle).unwrap();
            assert_eq!(2, ::credential::get_state(self.credential_handle).unwrap());
        }

        fn accept_credential(&self) {
            self.activate();
            ::credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(4, ::connection::get_state(self.connection_handle));
            assert_eq!(::v3::messages::status::Status::Success.code(), ::v3::handlers::issuance::get_holder_credential_status(self.credential_handle).unwrap());
        }

        fn send_presentation(&mut self) {
            self.activate();
            let presentation_requests = ::disclosed_proof::get_proof_request_messages(self.connection_handle, None).unwrap();
            let presentation_request = ::serde_json::from_str::<Vec<::serde_json::Value>>(&presentation_requests).unwrap()[0].clone();
            let presentation_request_json = ::serde_json::to_string(&presentation_request).unwrap();

            self.presentation_handle = ::disclosed_proof::create_proof("degree", &presentation_request_json).unwrap();


            let credentials = ::disclosed_proof::retrieve_credentials(self.presentation_handle).unwrap();
            let credentials: ::std::collections::HashMap<String, ::serde_json::Value> = ::serde_json::from_str(&credentials).unwrap();

            let mut use_credentials = json!({});

            for (referent, credentials) in credentials["attrs"].as_object().unwrap().iter() {
                use_credentials["attrs"][referent] = json!({
                    "credential": credentials[0]
                })
            }

            ::disclosed_proof::generate_proof(self.presentation_handle, use_credentials.to_string(), String::from("{}")).unwrap();
            assert_eq!(1, ::disclosed_proof::get_state(self.presentation_handle).unwrap());

            ::disclosed_proof::send_proof(self.presentation_handle, self.connection_handle).unwrap();
            assert_eq!(2, ::disclosed_proof::get_state(self.presentation_handle).unwrap());
        }

        fn ensure_presentation_verified(&self) {
            self.activate();
            ::disclosed_proof::update_state(self.presentation_handle, None).unwrap();
            assert_eq!(::v3::messages::status::Status::Success.code(), ::v3::handlers::proof_presentation::prover::get_presentation_status(self.presentation_handle).unwrap());
        }

        fn teardown(&self) {
            self.activate();
            close_wallet().unwrap();
            delete_wallet(&self.wallet_name, None, None, None).unwrap();
        }
    }

    #[test]
    fn aries_demo() {
        init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);
        ::utils::libindy::pool::tests::open_sandbox_pool();

        let mut faber = Faber::setup();
        let mut alice = Alice::setup();

        // Publish Schema and Credential Definition
        faber.create_schema();

        ::std::thread::sleep(::std::time::Duration::from_secs(2));

        faber.create_credential_definition();

        // Connection
        let invite = faber.create_invite();
        alice.accept_invite(&invite);

        faber.update_state(5);
        alice.update_state(4);
        faber.update_state(4);

        // Credential issuance
        faber.offer_credential();
        alice.accept_offer();
        faber.send_credential();
        alice.accept_credential();

        // Credential Presentation
        faber.request_presentation();
        alice.send_presentation();
        faber.verify_presentation();
        alice.ensure_presentation_verified();

        faber.teardown();
        alice.teardown();

        ::utils::libindy::pool::tests::delete_test_pool();
    }
}

