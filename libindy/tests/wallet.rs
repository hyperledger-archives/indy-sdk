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

use utils::environment::EnvironmentUtils;
use utils::inmem_wallet::InmemWallet;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::did::DidUtils;
use utils::constants::*;

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

            WalletUtils::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_custom_path() {
            TestUtils::cleanup_storage();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            WalletUtils::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(UNKNOWN_WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletUnknownTypeError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_custom_path() {
            TestUtils::cleanup_storage();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            WalletUtils::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            WalletUtils::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            WalletUtils::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_opened() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::delete_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_custom_path() {
            TestUtils::cleanup_storage();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            WalletUtils::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_storage(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = WalletUtils::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod export_wallet {
        use super::*;

        #[test]
        fn indy_export_wallet_works() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            assert!(path.exists());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_works() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let (did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();
            DidUtils::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let did_with_meta = DidUtils::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            WalletUtils::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let did_with_meta_after_import = DidUtils::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            assert_eq!(did_with_meta, did_with_meta_after_import);

            WalletUtils::close_wallet(wallet_handle).unwrap();
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

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyExistsError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_missed_key() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(WALLET_CONFIG, r#"{}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(r#"{"id": ""}"#, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_wrong_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = WalletUtils::delete_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAccessFailed);

            TestUtils::cleanup_storage();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyOpenedError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            TestUtils::cleanup_storage();

            let wallet_config_1 = r#"{"id":"indy_open_wallet_works_for_two_wallets1"}"#;
            let wallet_config_2 = r#"{"id":"indy_open_wallet_works_for_two_wallets2"}"#;

            WalletUtils::create_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            WalletUtils::create_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            let wallet_handle_1 = WalletUtils::open_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            let wallet_handle_2 = WalletUtils::open_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            WalletUtils::close_wallet(wallet_handle_1).unwrap();
            WalletUtils::close_wallet(wallet_handle_2).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = WalletUtils::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, r#"{"key":"key", "rekey":"other_key"}"#).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore]
        fn indy_open_wallet_works_for_invalid_config() {
            TestUtils::cleanup_storage();

            let config = r#"{"field":"value"}"#;

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = WalletUtils::open_wallet(config, WALLET_CREDENTIALS);
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

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            let res = WalletUtils::close_wallet(wallet_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod export_wallet {
        use super::*;
        use std::fs;

        #[test]
        fn indy_export_wallet_returns_error_if_path_exists() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            fs::DirBuilder::new()
                .recursive(true)
                .create(path).unwrap();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = WalletUtils::export_wallet(wallet_handle, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_config() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = WalletUtils::export_wallet(wallet_handle, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_handle() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = WalletUtils::export_wallet(wallet_handle + 1, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_returns_error_if_path_doesnt_exist() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            let wallet_config = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist"}"#;
            let res = WalletUtils::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            let res = WalletUtils::open_wallet(wallet_config, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_import_wallet_returns_error_if_invalid_config() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_import_wallet_works_for_other_key() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let config_json = json!({
                "path": path.to_str().unwrap(),
                "key": "other_key",
            }).to_string();

            let res = WalletUtils::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_import_wallet_works_for_duplicate_name() {
            TestUtils::cleanup_storage();

            let path = WalletUtils::export_wallet_path();
            let config_json = WalletUtils::prepare_export_wallet_config(&path);

            WalletUtils::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            WalletUtils::export_wallet(wallet_handle, &config_json).unwrap();

            let res = WalletUtils::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(ErrorCode::WalletAlreadyExistsError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            TestUtils::cleanup_storage();
        }
    }
}

fn _custom_path() -> String {
    let mut path = EnvironmentUtils::tmp_path();
    path.push("custom_wallet_path");
    path.to_str().unwrap().to_owned()
}

