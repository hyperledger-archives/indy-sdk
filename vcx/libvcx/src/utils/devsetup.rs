#[allow(unused_macros)]
macro_rules! init {
    ($x:expr) => (
    ::utils::threadpool::init();
    ::settings::clear_config();
    ::settings::set_config_value(::settings::CONFIG_WALLET_KEY,::settings::DEFAULT_WALLET_KEY);
    ::settings::set_config_value(::settings::CONFIG_WALLET_KEY_DERIVATION,::settings::DEFAULT_WALLET_KEY_DERIVATION);
    ::utils::libindy::wallet::tests::delete_test_wallet(::settings::DEFAULT_WALLET_NAME);

    match $x {
        "true" => {
            ::settings::set_defaults();
            ::settings::set_config_value(::settings::CONFIG_ENABLE_TEST_MODE,"true");
            ::utils::libindy::wallet::init_wallet(::settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
        },
        "false" => {
            ::settings::set_defaults();
            ::settings::set_config_value(::settings::CONFIG_ENABLE_TEST_MODE,"false");
            ::utils::libindy::wallet::init_wallet(::settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
        },
        "indy" => {
            ::settings::set_defaults();
            ::settings::set_config_value(::settings::CONFIG_ENABLE_TEST_MODE,"indy");
            ::utils::libindy::wallet::init_wallet(::settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
        },
        "ledger" => {
            ::settings::set_config_value(::settings::CONFIG_ENABLE_TEST_MODE,"false");
            ::utils::devsetup::tests::init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);
            ::utils::devsetup::tests::setup_ledger_env();
        },
        "agency" => {
            ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, ::settings::DEFAULT_WALLET_NAME));
            ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::CONSUMER_PREFIX, ::settings::DEFAULT_WALLET_NAME));
            ::utils::libindy::pool::tests::delete_test_pool();
            ::utils::devsetup::tests::init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);
            ::utils::devsetup::tests::setup_local_env("1.0");
        },
        "agency_2_0" => {
            ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::ENTERPRISE_PREFIX, ::settings::DEFAULT_WALLET_NAME));
            ::utils::libindy::wallet::tests::delete_test_wallet(&format!("{}_{}", ::utils::constants::CONSUMER_PREFIX, ::settings::DEFAULT_WALLET_NAME));
            ::utils::libindy::pool::tests::delete_test_pool();
            ::utils::devsetup::tests::init_plugin(::settings::DEFAULT_PAYMENT_PLUGIN, ::settings::DEFAULT_PAYMENT_INIT_FUNCTION);
            ::utils::devsetup::tests::setup_local_env("2.0");
        },
        _ => {panic!("Invalid test mode");},
    };
    )
}

#[allow(unused_macros)]
macro_rules! teardown {
    ($x:expr) => (

    match $x {
        "agency" => { ::utils::devsetup::tests::cleanup_local_env(); },
        "false" => {
            ::utils::libindy::wallet::tests::delete_test_wallet(::settings::DEFAULT_WALLET_NAME);
            ::utils::libindy::pool::tests::delete_test_pool();
        },
        _ => { panic!("Invalid test mode"); },
    };
    )
}

#[cfg(test)]
pub mod tests {
    extern crate rand;
    extern crate serde_json;
    extern crate libloading;
    extern crate libc;

    use utils::constants;
    use utils::libindy::wallet;
    use utils::libindy::pool;
    use settings;
    use std;
    use utils;
    use object_cache::ObjectCache;
    static mut INSTITUTION_CONFIG: u32 = 0;
    static mut CONSUMER_CONFIG: u32 = 0;
    use indy::ErrorCode;

    static INIT_PLUGIN: std::sync::Once = std::sync::ONCE_INIT;

    lazy_static! {
        static ref CONFIG_STRING: ObjectCache<String> = Default::default();
    }

    pub const TRUSTEE: &str = "000000000000000000000000Trustee1";

    /* dev */
    /*
    pub const AGENCY_ENDPOINT: &'static str = "http://int-eas.pdev.evernym.com";
    pub const AGENCY_DID: &'static str = "YRuVCckY6vfZfX9kcQZe3u";
    pub const AGENCY_VERKEY: &'static str = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";

    pub const C_AGENCY_ENDPOINT: &'static str = "http://int-agency.pdev.evernym.com";
    pub const C_AGENCY_DID: &'static str = "dTLdJqRZLwMuWSogcKfBT";
    pub const C_AGENCY_VERKEY: &'static str = "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH";
    */

    /* sandbox */
    /*pub const AGENCY_ENDPOINT: &'static str = "http://sbx-eas.pdev.evernym.com";
    pub const AGENCY_DID: &'static str = "HB7qFQyFxx4ptjKqioEtd8";
    pub const AGENCY_VERKEY: &'static str = "9pJkfHyfJMZjUjS7EZ2q2HX55CbFQPKpQ9eTjSAUMLU8";

    pub const C_AGENCY_ENDPOINT: &'static str = "http://sbx-agency.pdev.evernym.com";
    pub const C_AGENCY_DID: &'static str = "Nv9oqGX57gy15kPSJzo2i4";
    pub const C_AGENCY_VERKEY: &'static str = "CwpcjCc6MtVNdQgwoonNMFoR6dhzmRXHHaUCRSrjh8gj";*/

    /* dummy */
    pub const AGENCY_ENDPOINT: &'static str = "http://localhost:8080";
    pub const AGENCY_DID: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
    pub const AGENCY_VERKEY: &'static str = "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR";

    pub const C_AGENCY_ENDPOINT: &'static str = "http://localhost:8080";
    pub const C_AGENCY_DID: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
    pub const C_AGENCY_VERKEY: &'static str = "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR";

    pub fn set_trustee_did() {
        let (my_did, my_vk) = ::utils::libindy::signus::create_and_store_my_did(Some(TRUSTEE)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        settings::set_config_value(settings::CONFIG_INSTITUTION_VERKEY, &my_vk);
    }

    pub fn create_new_seed() -> String {

        let x = rand::random::<u32>();
        format!("{:032}", x)
    }

    pub fn init_plugin(library: &str, initializer: &str) {
        settings::set_config_value(settings::CONFIG_PAYMENT_METHOD, settings::DEFAULT_PAYMENT_METHOD);

        INIT_PLUGIN.call_once(|| {
            if let Ok(lib) = _load_lib(library) {
                unsafe {
                    if let Ok(init_func) = lib.get(initializer.as_bytes()) {
                        let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = init_func;

                        match init_func() {
                            ErrorCode::Success => {
                                debug!("Plugin has been loaded: {:?}", library);
                            },
                            _ => {
                                error!("Plugin has not been loaded: {:?}", library);
                                std::process::exit(123);
                            }
                        }
                    } else {
                        error!("Init function not found: {:?}", initializer);
                        std::process::exit(123);
                    }
                }
            } else {
                error!("Plugin not found: {:?}", library);
                std::process::exit(123);
            }
        });
    }

    #[cfg(all(unix, test))]
    fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
        libloading::os::unix::Library::open(Some(library), libc::RTLD_NOW | libc::RTLD_NODELETE)
            .map(libloading::Library::from)
    }

    #[cfg(any(not(unix), not(test)))]
    fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
        libloading::Library::new(library)
    }

    pub fn setup_ledger_env() {
        match pool::get_pool_handle() {
            Ok(x) => pool::close().unwrap(),
            Err(x) => (),
        };

        pool::tests::delete_test_pool();

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_WALLET_KEY,settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION,settings::DEFAULT_WALLET_KEY_DERIVATION);
        settings::set_config_value(settings::CONFIG_WALLET_NAME, settings::DEFAULT_WALLET_NAME);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_GENESIS_PATH,
                                   utils::get_temp_dir_path(Some(settings::DEFAULT_GENESIS_PATH)).to_str().unwrap_or(""));

        pool::tests::open_sandbox_pool();

        wallet::init_wallet(settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();

        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        set_trustee_did();

        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    pub fn cleanup_local_env() {
        set_institution();
        wallet::tests::delete_test_wallet(&format!("{}_{}", constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME));
        set_consumer();
        wallet::tests::delete_test_wallet(&format!("{}_{}", constants::CONSUMER_PREFIX, settings::DEFAULT_WALLET_NAME));
        pool::tests::delete_test_pool();
    }

    pub fn set_institution() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(INSTITUTION_CONFIG, |t| {
                settings::set_config_value(settings::CONFIG_PAYMENT_METHOD, settings::DEFAULT_PAYMENT_METHOD);
                settings::process_config_string(&t)
            }).unwrap();
        }
        change_wallet_handle();
    }

    pub fn set_consumer() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(CONSUMER_CONFIG, |t| {
                settings::set_config_value(settings::CONFIG_PAYMENT_METHOD, settings::DEFAULT_PAYMENT_METHOD);
                settings::process_config_string(&t)
            }).unwrap();
        }
        change_wallet_handle();
    }

    fn change_wallet_handle() {
        let wallet_handle = settings::get_config_value(settings::CONFIG_WALLET_HANDLE).unwrap();
        unsafe { wallet::WALLET_HANDLE = wallet_handle.parse::<i32>().unwrap() }
    }

    pub fn setup_local_env(protocol_type: &str) {
        use indy::ledger;
        use futures::Future;

        settings::clear_config();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, settings::DEFAULT_WALLET_KEY_DERIVATION);
        let enterprise_wallet_name = format!("{}_{}", constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME);
        let seed1 = create_new_seed();
        let config = json!({
            "agency_url": AGENCY_ENDPOINT.to_string(),
            "agency_did": AGENCY_DID.to_string(),
            "agency_verkey": AGENCY_VERKEY.to_string(),
            "wallet_name": enterprise_wallet_name,
            "wallet_key": settings::DEFAULT_WALLET_KEY.to_string(),
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
            "enterprise_seed": seed1,
            "agent_seed": seed1,
            "name": "institution".to_string(),
            "logo": "http://www.logo.com".to_string(),
            "path": constants::GENESIS_PATH.to_string(),
            "protocol_type": protocol_type,
        }).to_string();
        let enterprise_config = ::messages::agent_utils::connect_register_provision(&config).unwrap();

        ::api::vcx::vcx_shutdown(false);

        let consumer_wallet_name = format!("{}_{}", constants::CONSUMER_PREFIX, settings::DEFAULT_WALLET_NAME);
        let seed2 = create_new_seed();
        let config = json!({
            "agency_url": C_AGENCY_ENDPOINT.to_string(),
            "agency_did": C_AGENCY_DID.to_string(),
            "agency_verkey": C_AGENCY_VERKEY.to_string(),
            "wallet_name": consumer_wallet_name,
            "wallet_key": settings::DEFAULT_WALLET_KEY.to_string(),
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION.to_string(),
            "enterprise_seed": seed2,
            "agent_seed": seed2,
            "name": "consumer".to_string(),
            "logo": "http://www.logo.com".to_string(),
            "path": constants::GENESIS_PATH.to_string(),
            "protocol_type": protocol_type,
        }).to_string();
        let consumer_config = ::messages::agent_utils::connect_register_provision(&config).unwrap();

        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&enterprise_wallet_name, &enterprise_config)).unwrap();
        }
        unsafe {
            CONSUMER_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&consumer_wallet_name, &consumer_config)).unwrap();
        }
        pool::tests::open_sandbox_pool();

        // grab the generated did and vk from the consumer and enterprise
        set_consumer();
        let did2 = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let vk2 = settings::get_config_value(settings::CONFIG_INSTITUTION_VERKEY).unwrap();
        set_institution();
        let did1 = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let vk1 = settings::get_config_value(settings::CONFIG_INSTITUTION_VERKEY).unwrap();
        settings::clear_config();

        // make enterprise and consumer trustees on the ledger
        wallet::init_wallet(settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
        let (trustee_did, _) = ::utils::libindy::signus::create_and_store_my_did(Some(TRUSTEE)).unwrap();
        let req_nym = ledger::build_nym_request(&trustee_did, &did1, Some(&vk1), None, Some("TRUSTEE")).wait().unwrap();
        ::utils::libindy::ledger::libindy_sign_and_submit_request(&trustee_did, &req_nym).unwrap();
        let req_nym = ledger::build_nym_request(&trustee_did, &did2, Some(&vk2), None, Some("TRUSTEE")).wait().unwrap();
        ::utils::libindy::ledger::libindy_sign_and_submit_request(&trustee_did, &req_nym).unwrap();
        wallet::delete_wallet(settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();

        // as trustees, mint tokens into each wallet
        set_consumer();
        ::utils::libindy::payments::tests::token_setup(None, None);

        set_institution();
        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    fn _config_with_wallet_handle(wallet_n: &str, config: &str) -> String {
        let wallet_handle = wallet::open_wallet(wallet_n, None, None, None).unwrap();
        let mut config: serde_json::Value = serde_json::from_str(config).unwrap();
        config[settings::CONFIG_WALLET_HANDLE] = json!(wallet_handle.to_string());
        config.to_string()
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_local_env() {
        init!("ledger");
        ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);
    }

    pub fn setup_wallet_env(test_name: &str) -> Result<i32, String> {
        use utils::libindy::wallet::init_wallet;
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        init_wallet(test_name, None, None, None).map_err(|e| format!("Unable to init_wallet in tests: {}", e))
    }

    pub fn cleanup_wallet_env(test_name: &str) -> Result<(), String> {
        use utils::libindy::wallet::delete_wallet;
        println!("Deleting Wallet");
        delete_wallet(test_name, None, None, None).or(Err(format!("Unable to delete wallet: {}", test_name)))
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    pub fn test_two_enterprise_connections() {
        use connection::*;
        use std::thread;
        use std::time::Duration;
        use api::VcxStateType;

        init!("agency");

        let (faber, alice) = ::connection::tests::create_connected_connections();
        set_institution();
        wallet::tests::delete_test_wallet(&format!("{}_{}", constants::ENTERPRISE_PREFIX, settings::DEFAULT_WALLET_NAME));
        pool::close().unwrap();
        settings::set_defaults();

        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);

        let config = json!({
            "agency_url": AGENCY_ENDPOINT.to_string(),
            "agency_did": AGENCY_DID.to_string(),
            "agency_verkey": AGENCY_VERKEY.to_string(),
            "wallet_key": settings::DEFAULT_WALLET_KEY.to_string(),
            "name": "another_institution".to_string(),
            "logo": "http://www.logo.com".to_string(),
            "path": constants::GENESIS_PATH.to_string()
        }).to_string();
        let config = ::messages::agent_utils::connect_register_provision(&config).unwrap();

        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&settings::DEFAULT_WALLET_NAME, &config)).unwrap();
        }

        pool::tests::open_sandbox_pool();

        //wallet::open_wallet(wallet_name).unwrap();
        set_institution();

        let alice = create_connection("alice").unwrap();
        connect(alice, Some("{}".to_string())).unwrap();
        let details = get_invite_details(alice, false).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        ::utils::devsetup::tests::set_consumer();
        let faber = create_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(faber));
        connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        ::utils::devsetup::tests::set_institution();
        thread::sleep(Duration::from_millis(2000));
        update_state(alice, None).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(alice));

        teardown!("agency");
    }
}
