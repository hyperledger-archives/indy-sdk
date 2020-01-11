#[macro_use]
pub mod utils;
pub mod handlers;
pub mod messages;

pub const SERIALIZE_VERSION: &'static str = "2.0";

#[cfg(test)]
pub mod test {
    use rand;
    use rand::Rng;
    use utils::devsetup::tests::{init_plugin, config_with_wallet_handle};
    use messages::agent_utils::connect_register_provision;
    use utils::libindy::wallet::*;
    use v3::messages::a2a::A2AMessage;

    pub fn source_id() -> String {
        String::from("test source id")
    }

    pub mod setup {
        use settings::{CONFIG_WALLET_KEY_DERIVATION, DEFAULT_WALLET_KEY};

        pub fn base_config() -> ::serde_json::Value {
            json!({
                "agency_did":"VsKV7grR1BUE29mG2Fm2kX",
                "agency_endpoint":"http://localhost:8080",
                "agency_verkey":"Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
                "genesis_path":"<CHANGE_ME>",
                "institution_did":"V4SGRU86Z58d6TV7PBUe6f",
                "institution_logo_url":"<CHANGE_ME>",
                "institution_name":"<CHANGE_ME>",
                "institution_verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "protocol_type":"2.0",
                "remote_to_sdk_did":"LjC6xZPeYPeL5AjuRByMDA",
                "remote_to_sdk_verkey":"Bkd9WFmCydMCvLKL8x47qyQTN1nbyQ8rUK8JTsQRtLGE",
                "sdk_to_remote_did":"Mi3bbeWQDVpQCmGFBqWeYa",
                "sdk_to_remote_verkey":"CHcPnSn48wfrUhekmcFZAmx8NvhHCh72J73WToNiK9EX",
                "wallet_key":DEFAULT_WALLET_KEY,
                "wallet_name":"test_wallet",
                CONFIG_WALLET_KEY_DERIVATION:"RAW",
                "communication_method":"aries",
            })
        }

        pub struct TestModeSetup {}

        impl TestModeSetup {
            pub fn init() -> TestModeSetup {
                let mut config = base_config();
                config["enable_test_mode"] = json!("true");
                ::settings::process_config_string(&config.to_string(), false).unwrap();
                TestModeSetup {}
            }
        }

        pub struct AgencyModeSetup {
            pub wallet_name: String,
            pub wallet_handle: i32,
        }

        impl AgencyModeSetup {
            pub fn init() -> AgencyModeSetup {
                let wallet_name = "wallet_name";

                let mut config = base_config();
                config["wallet_name"] = json!(wallet_name);
                config["enable_test_mode"] = json!("agency");

                ::settings::process_config_string(&config.to_string(), false).unwrap();

                ::utils::libindy::wallet::create_wallet(wallet_name, None, None, None).unwrap();
                let config = ::utils::devsetup::tests::config_with_wallet_handle(wallet_name, &config.to_string());

                ::settings::process_config_string(&config.to_string(), false).unwrap();

                AgencyModeSetup {
                    wallet_name: wallet_name.to_string(),
                    wallet_handle: ::utils::libindy::wallet::get_wallet_handle(),
                }
            }
        }

        impl Drop for AgencyModeSetup {
            fn drop(&mut self) {
                ::utils::libindy::wallet::delete_wallet(&self.wallet_name, None, None, None).unwrap();
            }
        }
    }

    pub struct PaymentPlugin {}

    impl PaymentPlugin {
        pub fn load() {
            init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);
        }
    }

    pub struct Pool {}

    impl Pool {
        pub fn open() -> Pool {
            ::utils::libindy::pool::tests::open_sandbox_pool();
            Pool {}
        }
    }

    impl Drop for Pool {
        fn drop(&mut self) {
            ::utils::libindy::pool::close().unwrap();
            ::utils::libindy::pool::tests::delete_test_pool();
        }
    }

    fn download_message(did: String) -> ::messages::get_message::Message {
        let mut messages = ::messages::get_message::download_messages(Some(vec![did]), Some(vec![String::from("MS-103")]), None).unwrap();
        assert_eq!(1, messages.len());
        let mut messages = messages.pop().unwrap();
        assert_eq!(1, messages.msgs.len());
        let message = messages.msgs.pop().unwrap();
        message
    }

    pub struct Faber {
        pub wallet_name: String,
        pub wallet_handle: i32,
        pub connection_handle: u32,
        pub config: String,
        pub schema_handle: u32,
        pub cred_def_handle: u32,
        pub credential_handle: u32,
        pub presentation_handle: u32,
    }

    impl Faber {
        pub fn setup() -> Faber {
            ::settings::clear_config();
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

        pub fn setup_local() -> Faber {
            let wallet_name = "faber_wallet";

            let config = json!({
                "agency_did":"VsKV7grR1BUE29mG2Fm2kX",
                "agency_endpoint":"http://localhost:8080",
                "agency_verkey":"Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
                "communication_method":"aries",
                "genesis_path":"<CHANGE_ME>",
                "institution_did":"V4SGRU86Z58d6TV7PBUe6f",
                "institution_logo_url":"<CHANGE_ME>",
                "institution_name":"<CHANGE_ME>",
                "institution_verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "protocol_type":"2.0",
                "remote_to_sdk_did":"LjC6xZPeYPeL5AjuRByMDA",
                "remote_to_sdk_verkey":"Bkd9WFmCydMCvLKL8x47qyQTN1nbyQ8rUK8JTsQRtLGE",
                "sdk_to_remote_did":"Mi3bbeWQDVpQCmGFBqWeYa",
                "sdk_to_remote_verkey":"CHcPnSn48wfrUhekmcFZAmx8NvhHCh72J73WToNiK9EX",
                "wallet_key":"123",
                "wallet_name":"faber_wallet"
            }).to_string();

            ::settings::process_config_string(&config, false).unwrap();
            ::utils::libindy::wallet::create_wallet(wallet_name, None, None, None).unwrap();
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

        pub fn activate(&self) {
            ::settings::clear_config();
            ::settings::process_config_string(&self.config, false).unwrap();
            set_wallet_handle(self.wallet_handle);
        }

        pub fn create_schema(&mut self) {
            self.activate();
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let data = r#"["name","date","degree", "empty_param"]"#.to_string();
            let name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
            let version: String = String::from("1.0");

            self.schema_handle = ::schema::create_and_publish_schema("test_schema", did.clone(), name, version, data).unwrap();
        }

        pub fn create_credential_definition(&mut self) {
            self.activate();

            let schema_id = ::schema::get_schema_id(self.schema_handle).unwrap();
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let name = String::from("degree");
            let tag = String::from("tag");

            self.cred_def_handle = ::credential_def::create_and_publish_credentialdef(String::from("test_cred_def"), name, did.clone(), schema_id, tag, String::from("{}")).unwrap();
        }

        pub fn create_presentation_request(&self) -> u32 {
            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let requested_attrs = json!([
                {"name": "name"},
                {"name": "date"},
                {"name": "degree"},
                {"name": "empty_param", "restrictions": {"attr::empty_param::value": ""}}
            ]).to_string();

            ::proof::create_proof(String::from("alice_degree"),
                                  requested_attrs,
                                  json!([]).to_string(),
                                  json!({}).to_string(),
                                  String::from("proof_from_alice")).unwrap()
        }

        pub fn create_invite(&mut self) -> String {
            self.activate();
            self.connection_handle = ::connection::create_connection("alice").unwrap();
            ::connection::connect(self.connection_handle, None).unwrap();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(2, ::connection::get_state(self.connection_handle));

            ::connection::get_invite_details(self.connection_handle, false).unwrap()
        }

        pub fn update_state(&self, expected_state: u32) {
            self.activate();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));
        }

        pub fn update_state_with_message(&self, expected_state: u32) -> A2AMessage {
            self.activate();
            let did = ::connection::get_pw_did(self.connection_handle).unwrap();
            let message = download_message(did);

            let a2a_message = ::connection::decode_message(self.connection_handle, message.clone()).unwrap();

            ::connection::update_state_with_message(self.connection_handle, message).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));

            a2a_message
        }

        pub fn ping(&self) {
            self.activate();
            ::connection::send_ping(self.connection_handle, None).unwrap();
        }

        pub fn discovery_features(&self) {
            self.activate();
            ::connection::send_discovery_features(self.connection_handle, None, None).unwrap();
        }

        pub fn connection_info(&self) -> ::serde_json::Value {
            self.activate();
            let details = ::connection::get_invite_details(self.connection_handle, false).unwrap();
            ::serde_json::from_str(&details).unwrap()
        }

        pub fn offer_credential(&mut self) {
            self.activate();

            let did = String::from("V4SGRU86Z58d6TV7PBUe6f");
            let credential_data = json!({
                "name": "alice",
                "date": "05-2018",
                "degree": "maths",
                "empty_param": ""
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

        pub fn send_credential(&self) {
            self.activate();
            ::issuer_credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(3, ::issuer_credential::get_state(self.credential_handle).unwrap());

            ::issuer_credential::send_credential(self.credential_handle, self.connection_handle).unwrap();
            ::issuer_credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(4, ::issuer_credential::get_state(self.credential_handle).unwrap());
            assert_eq!(::v3::messages::status::Status::Success.code(), ::issuer_credential::get_credential_status(self.credential_handle).unwrap());
        }

        pub fn request_presentation(&mut self) {
            self.activate();
            self.presentation_handle = self.create_presentation_request();
            assert_eq!(1, ::proof::get_state(self.presentation_handle).unwrap());

            ::proof::send_proof_request(self.presentation_handle, self.connection_handle).unwrap();
            ::proof::update_state(self.presentation_handle, None).unwrap();

            assert_eq!(2, ::proof::get_state(self.presentation_handle).unwrap());
        }

        pub fn verify_presentation(&self) {
            self.activate();
            self.update_proof_state(4, ::v3::messages::status::Status::Success.code())
        }

        pub fn update_proof_state(&self, expected_state: u32, expected_status: u32) {
            self.activate();

            ::proof::update_state(self.presentation_handle, None).unwrap();
            assert_eq!(expected_state, ::proof::get_state(self.presentation_handle).unwrap());
            assert_eq!(expected_status, ::proof::get_proof_state(self.presentation_handle).unwrap());
        }

        pub fn teardown(&self) {
            self.activate();
            close_wallet().unwrap();
            delete_wallet(&self.wallet_name, None, None, None).unwrap();
        }
    }

    pub struct Alice {
        pub wallet_name: String,
        pub wallet_handle: i32,
        pub connection_handle: u32,
        pub config: String,
        pub credential_handle: u32,
        pub presentation_handle: u32,
    }

    impl Alice {
        pub fn setup() -> Alice {
            ::settings::clear_config();
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
                presentation_handle: 0,
            }
        }

        pub fn activate(&self) {
            ::settings::clear_config();
            ::settings::process_config_string(&self.config, false).unwrap();
            set_wallet_handle(self.wallet_handle);
        }

        pub fn accept_invite(&mut self, invite: &str) {
            self.activate();
            self.connection_handle = ::connection::create_connection_with_invite("faber", invite).unwrap();
            ::connection::connect(self.connection_handle, None).unwrap();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(3, ::connection::get_state(self.connection_handle));
        }

        pub fn update_state(&self, expected_state: u32) {
            self.activate();
            ::connection::update_state(self.connection_handle, None).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));
        }

        pub fn update_state_with_message(&self, expected_state: u32) -> A2AMessage {
            self.activate();
            let did = ::connection::get_pw_did(self.connection_handle).unwrap();
            let message = download_message(did);

            let a2a_message = ::connection::decode_message(self.connection_handle, message.clone()).unwrap();

            ::connection::update_state_with_message(self.connection_handle, message).unwrap();
            assert_eq!(expected_state, ::connection::get_state(self.connection_handle));

            a2a_message
        }

        pub fn accept_offer(&mut self) {
            self.activate();
            let offers = ::credential::get_credential_offer_messages(self.connection_handle).unwrap();
            let offer = ::serde_json::from_str::<Vec<::serde_json::Value>>(&offers).unwrap()[0].clone();
            let offer_json = ::serde_json::to_string(&offer).unwrap();

            self.credential_handle = ::credential::credential_create_with_offer("degree", &offer_json).unwrap();
            assert_eq!(3, ::credential::get_state(self.credential_handle).unwrap());

            ::credential::send_credential_request(self.credential_handle, self.connection_handle).unwrap();
            assert_eq!(2, ::credential::get_state(self.credential_handle).unwrap());
        }

        pub fn accept_credential(&self) {
            self.activate();
            ::credential::update_state(self.credential_handle, None).unwrap();
            assert_eq!(4, ::credential::get_state(self.credential_handle).unwrap());
            assert_eq!(::v3::messages::status::Status::Success.code(), ::credential::get_credential_status(self.credential_handle).unwrap());
        }

        pub fn get_proof_request_messages(&self) -> String {
            self.activate();
            let presentation_requests = ::disclosed_proof::get_proof_request_messages(self.connection_handle, None).unwrap();
            let presentation_request = ::serde_json::from_str::<Vec<::serde_json::Value>>(&presentation_requests).unwrap()[0].clone();
            let presentation_request_json = ::serde_json::to_string(&presentation_request).unwrap();
            presentation_request_json
        }

        pub fn send_presentation(&mut self) {
            self.activate();
            let presentation_request_json = self.get_proof_request_messages();

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

        pub fn decline_presentation_request(&mut self) {
            self.activate();
            let presentation_request_json = self.get_proof_request_messages();

            self.presentation_handle = ::disclosed_proof::create_proof("degree", &presentation_request_json).unwrap();
            ::disclosed_proof::decline_presentation_request(self.presentation_handle, self.connection_handle, Some(String::from("reason")), None).unwrap();
        }

        pub fn propose_presentation(&mut self) {
            self.activate();
            let presentation_request_json = self.get_proof_request_messages();

            self.presentation_handle = ::disclosed_proof::create_proof("degree", &presentation_request_json).unwrap();
            let proposal_data = json!({
                "attributes": [
                    {
                        "name": "first name"
                    }
                ],
                "predicates": [
                    {
                        "name": "age",
                        "predicate": ">",
                        "threshold": 18
                    }
                ]
            });
            ::disclosed_proof::decline_presentation_request(self.presentation_handle, self.connection_handle, None, Some(proposal_data.to_string())).unwrap();
        }

        pub fn ensure_presentation_verified(&self) {
            self.activate();
            ::disclosed_proof::update_state(self.presentation_handle, None).unwrap();
            assert_eq!(::v3::messages::status::Status::Success.code(), ::disclosed_proof::get_presentation_status(self.presentation_handle).unwrap());
        }
    }

    impl Drop for Faber {
        fn drop(&mut self) {
            self.activate();
            close_wallet().unwrap();
            delete_wallet(&self.wallet_name, None, None, None).unwrap();
        }
    }

    impl Drop for Alice {
        fn drop(&mut self) {
            self.activate();
            close_wallet().unwrap();
            delete_wallet(&self.wallet_name, None, None, None).unwrap();
        }
    }

    #[cfg(feature = "aries")]
    #[test]
    fn aries_demo() {
        PaymentPlugin::load();
        let _pool = Pool::open();

        let mut faber = Faber::setup();
        let mut alice = Alice::setup();

        // Publish Schema and Credential Definition
        faber.create_schema();

        ::std::thread::sleep(::std::time::Duration::from_secs(2));

        faber.create_credential_definition();

        // Connection
        let invite = faber.create_invite();
        alice.accept_invite(&invite);

        faber.update_state(3);
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

        // Decline Presentation
        faber.request_presentation();
        alice.decline_presentation_request();
        faber.update_proof_state(4, 2);

        // Propose Presentation
        faber.request_presentation();
        alice.propose_presentation();
        faber.update_proof_state(4, 2);
    }

    #[cfg(feature = "aries")]
    #[test]
    fn aries_demo_update_state_with_message_flow() {
        PaymentPlugin::load();
        let _pool = Pool::open();

        let mut faber = Faber::setup();
        let mut alice = Alice::setup();

        // Connection
        let invite = faber.create_invite();
        alice.accept_invite(&invite);

        faber.update_state_with_message(3);
        alice.update_state_with_message(4);
        faber.update_state_with_message(4);

        // Ping
        faber.ping();

        let message = alice.update_state_with_message(4);
        assert_match!(A2AMessage::Ping(_), message);

        let message = faber.update_state_with_message(4);
        assert_match!(A2AMessage::PingResponse(_), message);

        // Discovery Features
        faber.discovery_features();

        let message = alice.update_state_with_message(4);
        assert_match!(A2AMessage::Query(_), message);

        let message = faber.update_state_with_message(4);
        assert_match!(A2AMessage::Disclose(_), message);

        let faber_connection_info = faber.connection_info();
        assert!(faber_connection_info["protocols"].as_array().unwrap().len() > 0);
    }
}

