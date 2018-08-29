#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use utils::constants;
    use utils::libindy::wallet;
    use utils::libindy::pool;
    use settings;
    use object_cache::ObjectCache;

    static mut INSTITUTION_CONFIG: u32 = 0;
    static mut CONSUMER_CONFIG: u32 = 0;

    lazy_static! {
        static ref CONFIG_STRING: ObjectCache<String> = Default::default();
    }

    pub const TRUSTEE: &str = "000000000000000000000000Trustee1";
    pub const ENTERPRISE_PREFIX: &'static str = "enterprise";
    pub const CONSUMER_PREFIX: &'static str = "consumer";

    /* dev */
    /*
    pub const AGENCY_ENDPOINT: &'static str = "https://enym-eagency.pdev.evernym.com";
    pub const AGENCY_DID: &'static str = "YRuVCckY6vfZfX9kcQZe3u";
    pub const AGENCY_VERKEY: &'static str = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";

    pub const C_AGENCY_ENDPOINT: &'static str = "https://cagency.pdev.evernym.com";
    pub const C_AGENCY_DID: &'static str = "dTLdJqRZLwMuWSogcKfBT";
    pub const C_AGENCY_VERKEY: &'static str = "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH";
    */

    /* sandbox */
    pub const AGENCY_ENDPOINT: &'static str = "https://agency-ea-sandbox.evernym.com";
    pub const AGENCY_DID: &'static str = "HB7qFQyFxx4ptjKqioEtd8";
    pub const AGENCY_VERKEY: &'static str = "9pJkfHyfJMZjUjS7EZ2q2HX55CbFQPKpQ9eTjSAUMLU8";

    pub const C_AGENCY_ENDPOINT: &'static str = "https://agency-sandbox.evernym.com";
    pub const C_AGENCY_DID: &'static str = "Nv9oqGX57gy15kPSJzo2i4";
    pub const C_AGENCY_VERKEY: &'static str = "CwpcjCc6MtVNdQgwoonNMFoR6dhzmRXHHaUCRSrjh8gj";

    pub fn set_trustee_did() {
        let (my_did, _) = ::utils::libindy::signus::create_and_store_my_did(wallet::get_wallet_handle(), Some(TRUSTEE)).unwrap();
        let did = settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
    }

    pub fn setup_ledger_env(wallet_name: &str) {
        match pool::get_pool_handle() {
            Ok(x) => pool::close().unwrap(),
            Err(x) => (),
        };

        pool::tests::delete_test_pool();

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_WALLET_KEY,settings::TEST_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_GENESIS_PATH, settings::DEFAULT_GENESIS_PATH);
        pool::tests::open_sandbox_pool();

        wallet::init_wallet(wallet_name).unwrap();
        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        set_trustee_did();
        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    pub fn cleanup_dev_env(wallet_name: &str) {
        _delete_wallet(wallet_name);
        _delete_pool();
    }

    pub fn cleanup_local_env(wallet_name: &str) {
        set_institution();
        _delete_wallet(&format!("{}_{}", ENTERPRISE_PREFIX, wallet_name));
        set_consumer();
        _delete_wallet(&format!("{}_{}", CONSUMER_PREFIX, wallet_name));
        _delete_pool();
    }

    fn _delete_wallet(wallet_name: &str) {
        wallet::close_wallet().unwrap();
        wallet::delete_wallet(wallet_name).unwrap();
    }

    fn _delete_pool() {
        if pool::get_pool_handle().is_ok() {
            pool::close().unwrap();
            pool::delete(::utils::constants::POOL).unwrap();
        }
    }

    pub fn set_institution() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(INSTITUTION_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
        change_wallet_handle();
    }

    pub fn set_consumer() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(CONSUMER_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
        change_wallet_handle();
    }

    fn change_wallet_handle() {
        let wallet_handle = settings::get_config_value(settings::CONFIG_WALLET_HANDLE).unwrap();
        unsafe { wallet::WALLET_HANDLE = wallet_handle.parse::<i32>().unwrap() }
    }

    pub fn setup_local_env(wallet_name: &str) {
        settings::clear_config();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::TEST_WALLET_KEY);

        let enterprise_wallet_name = format!("{}_{}", ENTERPRISE_PREFIX, wallet_name);
        let enterprise_config = ::messages::agent_utils::connect_register_provision(AGENCY_ENDPOINT,
                                                                                    AGENCY_DID,
                                                                                    AGENCY_VERKEY,
                                                                                    Some(enterprise_wallet_name.clone()),
                                                                                    None,
                                                                                    Some(TRUSTEE.to_string()),
                                                                                    settings::TEST_WALLET_KEY,
                                                                                    Some("institution".to_string()),
                                                                                    Some("http://www.logo.com".to_string()),
                                                                                    Some(constants::GENESIS_PATH.to_string())).unwrap();

        ::api::vcx::vcx_shutdown(false);

        let consumer_wallet_name = format!("{}_{}", CONSUMER_PREFIX, wallet_name);
        let consumer_config = ::messages::agent_utils::connect_register_provision(C_AGENCY_ENDPOINT,
                                                                         C_AGENCY_DID,
                                                                         C_AGENCY_VERKEY,
                                                                         Some(consumer_wallet_name.clone()),
                                                                         None,
                                                                         Some(TRUSTEE.to_string()),
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("consumer".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();


        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&enterprise_wallet_name, &enterprise_config)).unwrap();
        }

        unsafe {
            CONSUMER_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&consumer_wallet_name, &consumer_config)).unwrap();
        }

        pool::tests::open_sandbox_pool();

        set_consumer();
        ::utils::libindy::payments::tests::token_setup(None, None);

        set_institution();
        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    fn _config_with_wallet_handle(wallet_n: &str, config: &str) -> String {
        let wallet_handle = wallet::open_wallet(wallet_n).unwrap();
        let mut config: serde_json::Value = serde_json::from_str(config).unwrap();
        config[settings::CONFIG_WALLET_HANDLE] = json!(wallet_handle.to_string());
        config.to_string()
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_local_env() {
        let wallet_name = "test_local_env";
        setup_local_env(wallet_name);
        ::utils::libindy::anoncreds::tests::create_and_store_credential();
        cleanup_local_env(wallet_name);
    }

    pub fn setup_wallet_env(test_name: &str) -> Result<i32, String> {
        use utils::libindy::wallet::init_wallet;
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        init_wallet(test_name).map_err(|e| format!("Unable to init_wallet in tests: {}", e))
    }

    pub fn cleanup_wallet_env(test_name: &str) -> Result<(), String> {
        use utils::libindy::wallet::delete_wallet;
        println!("Deleting Wallet");
        delete_wallet(test_name).or(Err(format!("Unable to delete wallet: {}", test_name)))
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    pub fn test_two_enterprise_connections() {
        use connection::*;
        use std::thread;
        use std::time::Duration;
        use api::VcxStateType;

        let wallet_name = "two";
        setup_local_env(wallet_name);

        let (faber, alice) = ::connection::tests::create_connected_connections();
        ::api::vcx::vcx_shutdown(false);

        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::TEST_WALLET_KEY);

        let config = ::messages::agent_utils::connect_register_provision(AGENCY_ENDPOINT,
                                                                         AGENCY_DID,
                                                                         AGENCY_VERKEY,
                                                                         Some(wallet_name.to_string()),
                                                                         None,
                                                                         None,
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("another_institution".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();

        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(_config_with_wallet_handle(&wallet_name, &config)).unwrap();
        }

        pool::tests::open_sandbox_pool();

        //wallet::open_wallet(wallet_name).unwrap();
        set_institution();

        let alice = build_connection("alice").unwrap();
        connect(alice, Some("{}".to_string())).unwrap();
        let details = get_invite_details(alice, false).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        ::utils::devsetup::tests::set_consumer();
        let faber = build_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(faber));
        connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        ::utils::devsetup::tests::set_institution();
        thread::sleep(Duration::from_millis(2000));
        update_state(alice).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(alice));

        cleanup_local_env(wallet_name);
    }
}
