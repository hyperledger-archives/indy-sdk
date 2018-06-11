extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate named_type;
#[macro_use]
extern crate named_type_derive;

#[macro_use]
mod utils;

use utils::inmem_wallet::InmemWallet;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::constants::*;
use utils::environment::EnvironmentUtils;

use indy::api::ErrorCode;

pub const CONFIG: &'static str = r#"{"freshness_time":1000}"#;

mod high_cases {
    use super::*;

    mod register_wallet_storage {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_works() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, Some(TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(POOL, WALLET, Some("unknown_type"), None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletUnknownTypeError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, Some(TYPE), Some(CONFIG), None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            WalletUtils::delete_wallet(WALLET, None).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(WALLET, None).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore]
        fn indy_delete_wallet_works_for_opened() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let res = WalletUtils::delete_wallet(WALLET, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();
            WalletUtils::delete_wallet(WALLET, None).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_open_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, Some(CONFIG), None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod export_wallet {
        use super::*;
        use std::path::PathBuf;
        use std::fs;


        fn _prepare_export_wallet_path() -> PathBuf {
            let export_dir = EnvironmentUtils::tmp_path();
            if !export_dir.exists() {
                fs::create_dir_all(export_dir).unwrap();
            }

            EnvironmentUtils::tmp_file_path("export_file")
        }

        #[test]
        fn indy_export_wallet_works() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_export_wallet_works";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(wallet_name, None, None).unwrap();
            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            assert!(path.exists());
            TestUtils::cleanup_storage();
        }
    }

    mod import_wallet {
        use super::*;
        use std::path::PathBuf;
        use std::fs;


        fn _prepare_export_wallet_path() -> PathBuf {
            let export_dir = EnvironmentUtils::tmp_path();
            if !export_dir.exists() {
                fs::create_dir_all(export_dir).unwrap();
            }

            EnvironmentUtils::tmp_file_path("export_file")
        }

        #[test]
        fn indy_import_wallet_works() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_import_wallet_works";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(wallet_name, None, None).unwrap();
            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(wallet_name, None).unwrap();

            WalletUtils::import_wallet(POOL, wallet_name, None, None, None, &config_json).unwrap();
            WalletUtils::open_wallet(wallet_name, None, None).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    extern crate libc;

    use super::*;
    use std::ffi::CString;

    mod register_wallet_type {
        use super::*;
        use indy::api::wallet::indy_register_wallet_storage;

        #[test]
        fn indy_register_wallet_storage_does_not_work_twice_with_same_name() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            let res = WalletUtils::register_wallet_storage(INMEM_TYPE, true);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletTypeAlreadyRegisteredError);

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_register_wallet_storage_does_not_work_with_null_params() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let xtype = CString::new(INMEM_TYPE).unwrap();
            let res = indy_register_wallet_storage(1, xtype.as_ptr(), None, None, None, None, None,
                                                   None, None, None, None, None,
                                                   None, None, None, None, None, None,
                                                   None, None, None, None,
                                                   None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam3);

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let res = WalletUtils::create_wallet(POOL, WALLET, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyExistsError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_missed_key() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(POOL, WALLET, None, None, Some(r#"{}"#));
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInputError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            TestUtils::cleanup_storage();

            let wallet_name = "";
            let res = WalletUtils::create_wallet(POOL, wallet_name, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidParam3);

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::delete_wallet(WALLET, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            WalletUtils::delete_wallet(WALLET, None).unwrap();
            let res = WalletUtils::delete_wallet(WALLET, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_wrong_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, Some(r#"{"key":"key"}"#)).unwrap();
            let res = WalletUtils::delete_wallet(WALLET, Some(r#"{"key":"other_key"}"#));
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAccessFailed);

            TestUtils::cleanup_storage();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::open_wallet(WALLET, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            let res = WalletUtils::open_wallet(WALLET, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyOpenedError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            TestUtils::cleanup_storage();

            let wallet_name_1 = "indy_open_wallet_works_for_two_wallets1";
            let wallet_name_2 = "indy_open_wallet_works_for_two_wallets2";

            WalletUtils::create_wallet(POOL, wallet_name_1, None, None, None).unwrap();
            WalletUtils::create_wallet(POOL, wallet_name_2, None, None, None).unwrap();

            let wallet_handle_1 = WalletUtils::open_wallet(wallet_name_1, None, None).unwrap();
            let wallet_handle_2 = WalletUtils::open_wallet(wallet_name_2, None, None).unwrap();

            WalletUtils::close_wallet(wallet_handle_1).unwrap();
            WalletUtils::close_wallet(wallet_handle_2).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, Some(r#"{"key":"key"}"#)).unwrap();
            let res = WalletUtils::open_wallet(WALLET, None, Some(r#"{"key":"other_key"}"#));
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, Some(r#"{"key":"key"}"#)).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, Some(r#"{"key":"key", "rekey":"other_key"}"#)).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, Some(r#"{"key":"other_key"}"#)).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore]
        fn indy_open_wallet_works_for_invalid_config() {
            TestUtils::cleanup_storage();

            let config = r#"{"field":"value"}"#;

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let res = WalletUtils::open_wallet(WALLET, Some(config), None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::close_wallet(1);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            let res = WalletUtils::close_wallet(wallet_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod export_wallet {
        use super::*;
        use std::path::PathBuf;
        use std::fs;


        fn _prepare_export_wallet_path() -> PathBuf {
            let export_dir = EnvironmentUtils::tmp_path();
            if !export_dir.exists() {
                fs::create_dir_all(export_dir).unwrap();
            }

            EnvironmentUtils::tmp_file_path("export_file")
        }

        #[test]
        fn indy_export_wallet_returns_error_if_path_exists() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            fs::File::create(&path).unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_export_wallet_returns_error_if_path_exists";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(wallet_name, None, None).unwrap();
            let res = WalletUtils::export_wallet(wallet_handle, &config_json);

            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);
            assert!(path.exists());
            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_config() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            fs::File::create(&path).unwrap();

            let wallet_name = "indy_export_wallet_returns_error_if_invalid_config";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(wallet_name, None, None).unwrap();
            let res = WalletUtils::export_wallet(wallet_handle, "{}");

            // TODO - maybe introduce WalletConfigurationError
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
            assert!(path.exists());
            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_handle() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            fs::File::create(&path).unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_export_wallet_returns_error_if_invalid_handle";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(wallet_name, None, None).unwrap();
            let res = WalletUtils::export_wallet(wallet_handle + 1, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
            assert!(path.exists());
            TestUtils::cleanup_storage();
        }
    }

    mod import_wallet {
        use super::*;
        use std::path::PathBuf;
        use std::fs;


        fn _prepare_export_wallet_path() -> PathBuf {
            let export_dir = EnvironmentUtils::tmp_path();
            if !export_dir.exists() {
                fs::create_dir_all(export_dir).unwrap();
            }

            EnvironmentUtils::tmp_file_path("export_file")
        }

        #[test]
        fn indy_import_wallet_returns_error_if_path_doesnt_exist() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_import_wallet_returns_error_if_path_doesnt_exist";
            let res = WalletUtils::import_wallet(POOL, wallet_name, None, None, None, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);
            let res = WalletUtils::open_wallet(wallet_name, None, None);
            assert_match!(Err(_), res);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_import_wallet_returns_error_if_invalid_config() {
            TestUtils::cleanup_storage();
            let path = _prepare_export_wallet_path();
            let path_str = path.to_str().unwrap();
            let config_json = WalletUtils::prepare_export_wallet_config(path_str);

            let wallet_name = "indy_import_wallet_returns_error_if_invalid_config";
            let res = WalletUtils::import_wallet(POOL, wallet_name, None, None, None, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }
}
