extern crate vcx;
extern crate indy;
#[macro_use]
extern crate log;
use vcx::api::logger::*;
use indy::wallet;
/// These tests can only be run individually as initing the log crate can happen
/// only once.
mod log_tests {
    use super::*;
    #[test]
    fn test_logging_default_info() {
        assert_eq!(vcx_set_default_logger(None), 0);
        unsafe { assert_eq!(LOGGER_STATE, LoggerState::Default); }
        info!("testing info");
    }

    #[test]
    #[ignore]
    fn test_logging_default_debug() {
        use vcx::api::vcx::vcx_error_c_message;
        let pattern = Some("debug".to_string());
        assert_eq!(vcx_set_default_logger(pattern), 0);
        debug!("testing debug");
        vcx_error_c_message(1000);
    }

    #[test]
    #[ignore]
    fn test_logging_default_is_warn() {
        assert_eq!(vcx_set_default_logger(None), 0);
        warn!("testing warning");
    }

    #[test]
    #[ignore]
    fn test_logging_env_var(){
        use std::env::set_var;
        set_var("RUST_LOG", "info");
        assert_eq!(vcx_set_default_logger(None), 0);
        info!("testing info");
    }

/// This test depends on some modifications to the indy code.
/// By adding a indy_set_default_logger(null()) to the indy_create_wallet function,
/// it tests that both calls to log::init an occur and not conflict
    #[test]
    #[ignore]
    fn test_works_with_libindy(){
        pub const DEFAULT_WALLET_CONFIG: &'static str = r#"{"id":"wallet_1","storage_type":"default"}"#;
        pub const WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;
        wallet::Wallet::create(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
        assert_eq!(vcx_set_default_logger(Some("debug".to_string())), 0);
        debug!("testing debug");
        trace!("testing trace");
    }
}
