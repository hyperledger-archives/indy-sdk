#[cfg(test)]
pub mod tests {
    extern crate rand;

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

    /* INSTITUTION/ENTERPRISE settings */
    pub const AGENCY_ENDPOINT: &'static str = "https://enym-eagency.pdev.evernym.com";
    pub const AGENCY_DID: &'static str = "YRuVCckY6vfZfX9kcQZe3u";
    pub const AGENCY_VERKEY: &'static str = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";

    /* CONSUMER/USER settings */
    pub const C_AGENCY_ENDPOINT: &'static str = "https://cagency.pdev.evernym.com";
    pub const C_AGENCY_DID: &'static str = "dTLdJqRZLwMuWSogcKfBT";
    pub const C_AGENCY_VERKEY: &'static str = "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH";

    pub fn setup_ledger_env(wallet_name: &str) {
        match pool::get_pool_handle() {
            Ok(x) => pool::close().unwrap(),
            Err(x) => (),
        };

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_WALLET_KEY,settings::TEST_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_GENESIS_PATH, settings::DEFAULT_GENESIS_PATH);
        pool::tests::open_sandbox_pool();

        wallet::init_wallet(wallet_name).unwrap();
        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        let (my_did, _) = ::utils::libindy::signus::create_and_store_my_did(wallet::get_wallet_handle(), Some(TRUSTEE)).unwrap();
        let did = settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    pub fn cleanup_dev_env(wallet_name: &str) {
        //settings::set_defaults();
        wallet::close_wallet().unwrap();
        wallet::delete_wallet(wallet_name).unwrap();
        pool::close().unwrap();
        pool::delete(::utils::constants::POOL).unwrap();
    }

    pub fn set_institution() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(INSTITUTION_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
    }

    pub fn set_consumer() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(CONSUMER_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
    }

    pub fn setup_local_env(wallet_name: &str) {
        settings::clear_config();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::TEST_WALLET_KEY);

        let config = ::messages::agent_utils::connect_register_provision(AGENCY_ENDPOINT,
                                                                         AGENCY_DID,
                                                                         AGENCY_VERKEY,
                                                                         Some(wallet_name.to_string()),
                                                                         None,
                                                                         Some(TRUSTEE.to_string()),
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("institution".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();

        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(config).unwrap();
        }

        ::api::vcx::vcx_shutdown(false);

        let config = ::messages::agent_utils::connect_register_provision(C_AGENCY_ENDPOINT,
                                                                         C_AGENCY_DID,
                                                                         C_AGENCY_VERKEY,
                                                                         Some(wallet_name.to_string()),
                                                                         None,
                                                                         None,
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("consumer".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();

        unsafe {
            CONSUMER_CONFIG = CONFIG_STRING.add(config).unwrap();
        }

        pool::tests::open_sandbox_pool();

        wallet::open_wallet(wallet_name).unwrap();
        set_institution();

        ::utils::libindy::payments::tests::token_setup(None, None);

    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_local_env() {
        let wallet_name = "test_local_env";
        setup_local_env(wallet_name);
        ::utils::libindy::anoncreds::tests::create_and_store_credential();
        cleanup_dev_env(wallet_name);
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
            INSTITUTION_CONFIG = CONFIG_STRING.add(config).unwrap();
        }

        pool::tests::open_sandbox_pool();

        wallet::open_wallet(wallet_name).unwrap();
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

        cleanup_dev_env(wallet_name);
    }
}
