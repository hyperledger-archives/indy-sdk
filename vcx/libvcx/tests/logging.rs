#[macro_use]
extern crate vcx;
extern crate indy;
extern crate libc;
#[macro_use]
extern crate log;



use self::libc::{c_void, c_char};
use std::ptr::null;
use vcx::api::logger::*;
use vcx::utils::logger::{LOGGER_STATE, LoggerState};
use indy::wallet;

use vcx::api::logger::vcx_set_logger;
/// These tests can only be run individually as initing the log crate can happen
/// only once.
///
/// These tests usually need to be run manually to verify that the standard
/// logging is outputting to stdout.
mod log_tests {
    use super::*;
    use vcx::utils::cstring::CStringUtils;
    use vcx::api::vcx::vcx_error_c_message;

    static mut COUNT: u32 = 0;
    extern fn custom_log(_context: *const c_void,
                         _level: u32,
                         _target: *const c_char,
                         message: *const c_char,
                         _module_path: *const c_char,
                         _file: *const c_char,
                         _line: u32) {
        let _message = CStringUtils::c_str_to_string(message).unwrap();
        unsafe { COUNT = COUNT + 1 }
    }
    #[test]
    fn test_logging_default_debug() {
        // this test should output a single debug line
        // and a single info line (from the vcx_error_c_message call)

        let pattern = CStringUtils::string_to_cstring("debug".to_string());
        assert_eq!(vcx_set_default_logger(pattern.as_ptr()), 0);
        debug!("testing debug");
//        vcx_error_c_message(1000);

    }

    #[ignore]
    #[test]
    fn test_logging_default_is_warn() {
        // this test should output a single warning line
        assert_eq!(vcx_set_default_logger(null()), 0);
        unsafe { assert_eq!(LOGGER_STATE, LoggerState::Default); }
        warn!("testing warning");
    }

    #[ignore]
    #[test]
    fn test_logging_env_var() {
        // this test should output a single info line
        use std::env::set_var;
        set_var("RUST_LOG", "info");
        assert_eq!(vcx_set_default_logger(null()), 0);
        info!("testing info");
    }

    /// This test depends on some modifications to the indy code.
    /// By adding a indy_set_default_logger(null()) to the indy_create_wallet function,
    /// it tests that both calls to log::init an occur and not conflict
    #[ignore]
    #[test]
    fn test_works_with_libindy() {
        pub const DEFAULT_WALLET_CONFIG: &'static str = r#"{"id":"wallet_1","storage_type":"default"}"#;
        pub const WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;
        wallet::Wallet::create(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
        let pattern = CStringUtils::string_to_cstring("debug".to_string());
        assert_eq!(vcx_set_default_logger(pattern.as_ptr()), 0);
        debug!("testing debug");
        trace!("testing trace");
    }

    #[ignore]
    #[test]
    fn test_set_logger() {
        unsafe { assert_eq!(COUNT, 0);}
        let _err = vcx_set_logger(null(), None, Some(custom_log), None);
        debug!("testing debug");
        unsafe { assert!(COUNT > 1); }

    }

    #[test]
    fn test_error_code() {
        use vcx::api::vcx::vcx_error_c_message;
        use vcx::utils::error;
        use std::ffi::CString;
        let mut buf:Vec<u8> = vec![0x0; 256];
        let error_code = 1035;
        let err = vcx_error_c_message(error_code, buf.as_mut_ptr(), 256);
        assert_eq!(err, error::SUCCESS.code_num);
        let mut s = String::from_utf8(buf).unwrap().shrink_to_fit();
        println!("{:?}", &s);
        println!("size: {}", s.len())
//        unsafe {
//            let c_s = CString::from_vec_unchecked(buf);
//            println!("after: {:?}", c_s);
//        }
    }


}



