#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::inmem_wallet::InmemWallet;
use utils::{environment, wallet, test, did};
use utils::constants::*;

use self::indy::ErrorCode;
use api::INVALID_WALLET_HANDLE;
use std::path::PathBuf;
use std::fs;

pub const CONFIG: &'static str = r#"{"freshness_time":1000}"#;

fn cleanup_file(path: &PathBuf) {
    if path.exists() {
        fs::remove_file(path).unwrap();
    }
}

use utils::test::cleanup_wallet;

mod high_cases {
    use super::*;

    mod register_wallet_storage {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_works() {
            utils::setup("indy_register_wallet_storage_works");

            test::cleanup_storage("indy_register_wallet_storage_works");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();

            InmemWallet::cleanup();
            utils::tear_down("indy_register_wallet_storage_works");
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            utils::setup("indy_create_wallet_works");

            wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_create_wallet_works");
        }

        #[test]
        fn indy_create_wallet_works_for_custom_path() {
            utils::setup("indy_create_wallet_works_for_custom_path");

            let config = json!({
                "id": "wallet_1_indy_create_wallet_works_for_custom_path",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path("indy_create_wallet_works_for_custom_path"),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_create_wallet_works_for_custom_path");
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            utils::setup("indy_create_wallet_works_for_plugged");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
            utils::tear_down("indy_create_wallet_works_for_plugged");
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            utils::setup("indy_create_wallet_works_for_unknown_type");

            let res = wallet::create_wallet(UNKNOWN_WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletUnknownTypeError, res);

            utils::tear_down("indy_create_wallet_works_for_unknown_type");
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_empty_type"}"#;
            utils::setup("indy_create_wallet_works_for_empty_type");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_create_wallet_works_for_empty_type");
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works"}"#;
            utils::setup("indy_delete_wallet_works");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_delete_wallet_works");
        }

        #[test]
        fn indy_delete_wallet_works_for_custom_path() {
            utils::setup("indy_delete_wallet_works_for_custom_path");

            let config = json!({
                "id": "wallet_1_indy_delete_wallet_works_for_custom_path",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path("indy_delete_wallet_works_for_custom_path"),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_delete_wallet_works_for_custom_path");
        }

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works_for_closed"}"#;
            utils::setup("indy_delete_wallet_works_for_closed");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_delete_wallet_works_for_closed");
        }

        #[test]
        fn indy_delete_wallet_works_for_opened() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works_for_opened"}"#;
            utils::setup("indy_delete_wallet_works_for_opened");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidState, res);

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down("indy_delete_wallet_works_for_opened");
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            utils::setup("indy_delete_wallet_works_for_plugged");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
            utils::tear_down("indy_delete_wallet_works_for_plugged");
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_open_wallet_works"}"#;
            utils::setup("indy_open_wallet_works");

            let config = json!({
                "id": "indy_open_wallet_works",
                "storage_type": "default"
            }).to_string();
            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_open_wallet_works", &config);
        }

        #[test]
        fn indy_open_wallet_works_for_custom_path() {
            utils::setup("indy_open_wallet_works_for_custom_path");

            let config = json!({
                "id": "indy_open_wallet_works_for_custom_path",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path("indy_open_wallet_works_for_custom_path"),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_open_wallet_works_for_custom_path", &config);
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            utils::setup("indy_open_wallet_works_for_plugged");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
            utils::tear_down("indy_open_wallet_works_for_plugged");
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            utils::setup("indy_close_wallet_works");

            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_close_wallet_works");

            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_close_wallet_works", &wallet_config);
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            utils::setup("indy_close_wallet_works_for_plugged");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
            utils::tear_down("indy_close_wallet_works_for_plugged");
        }
    }

    mod export_wallet {
        use super::*;

        #[test]
        fn indy_export_wallet_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_export_wallet_works");

            let path = wallet::export_wallet_path("indy_export_wallet_works");
            let config_json = wallet::prepare_export_wallet_config(&path);

            did::create_my_did(wallet_handle, "{}").unwrap();
            did::create_my_did(wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            assert!(path.exists());

            test::cleanup_files(&path, "indy_export_wallet_works_export_wallet");
            utils::tear_down_with_wallet(wallet_handle, "indy_export_wallet_works", &wallet_config);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_works() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_import_wallet_works"}"#;
            utils::setup("indy_import_wallet_works");

            let path = wallet::export_wallet_path("indy_import_wallet_works");
            let config_json = wallet::prepare_export_wallet_config(&path);

            let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet("indy_import_wallet_works").unwrap();

            let (did, _) = did::create_my_did(wallet_handle, "{}").unwrap();
            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let did_with_meta = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();

            wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let did_with_meta_after_import = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            assert_eq!(did_with_meta, did_with_meta_after_import);

            cleanup_file(&path);
            utils::tear_down_with_wallet(wallet_handle, "indy_import_wallet_works", WALLET_CONFIG);
        }
    }

    mod generate_wallet_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_generate_wallet_key_works() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_generate_wallet_key_works"}"#;
            test::cleanup_storage("indy_generate_wallet_key_works");

            let key = wallet::generate_wallet_key(None).unwrap();

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(WALLET_CONFIG, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, &credentials).unwrap();

            test::cleanup_storage("indy_generate_wallet_key_works");
        }

        #[test]
        fn indy_generate_wallet_key_works_for_seed() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_generate_wallet_key_works_for_seed"}"#;
            test::cleanup_storage("indy_generate_wallet_key_works_for_seed");

            let config = json!({"seed": MY1_SEED}).to_string();
            let key = wallet::generate_wallet_key(Some(config.as_str())).unwrap();
            assert_eq!(key.from_base58().unwrap(), vec![177, 92, 220, 199, 104, 203, 161, 4, 218, 78, 105, 13, 7, 50, 66, 107, 154, 155, 108, 133, 1, 30, 87, 149, 233, 76, 39, 156, 178, 46, 230, 124]);

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(WALLET_CONFIG, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, &credentials).unwrap();

            test::cleanup_storage("indy_generate_wallet_key_works_for_seed");
        }
    }
}

mod medium_cases {
    extern crate libc;

    use super::*;
    use std::ffi::CString;

    mod register_wallet_type {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_does_not_work_twice_with_same_name() {
            utils::setup("indy_register_wallet_storage_does_not_work_twice_with_same_name");
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            let res = wallet::register_wallet_storage(INMEM_TYPE, true).unwrap_err();
            assert_eq!(ErrorCode::WalletTypeAlreadyRegisteredError, res);

            InmemWallet::cleanup();
            utils::tear_down("indy_register_wallet_storage_does_not_work_twice_with_same_name");
        }

        #[test]
        fn indy_register_wallet_storage_does_not_work_with_null_params() {
            utils::setup("indy_register_wallet_storage_does_not_work_with_null_params");
            InmemWallet::cleanup();

            let xtype = CString::new(INMEM_TYPE).unwrap();
            let res = unsafe {
                wallet::indy_register_wallet_storage(1, xtype.as_ptr(), None, None, None, None, None,
                                                     None, None, None, None, None,
                                                     None, None, None, None, None, None,
                                                     None, None, None, None,
                                                     None, None, None, None, None)
            };
            assert_eq!(ErrorCode::CommonInvalidParam3, res);

            InmemWallet::cleanup();
            utils::tear_down("indy_register_wallet_storage_does_not_work_with_null_params");
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_duplicate_name"}"#;
            utils::setup("indy_create_wallet_works_for_duplicate_name");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletAlreadyExistsError, res);

            test::cleanup_storage("indy_create_wallet_works_for_duplicate_name");
        }

        #[test]
        fn indy_create_wallet_works_for_missed_key() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_missed_key"}"#;
            utils::setup("indy_create_wallet_works_for_missed_key");

            let res = wallet::create_wallet(WALLET_CONFIG, r#"{}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_create_wallet_works_for_missed_key");
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            utils::setup("indy_create_wallet_works_for_empty_name");

            let res = wallet::create_wallet(r#"{"id": ""}"#, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_create_wallet_works_for_empty_name");
        }

        #[test]
        fn indy_create_wallet_works_for_raw_key_invalid_length() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_create_wallet_works_for_raw_key_invalid_length"}"#;
            test::cleanup_storage("indy_create_wallet_works_for_raw_key_invalid_length");

            let credentials = json!({"key": "key", "key_derivation_method": "RAW"}).to_string();
            let res = wallet::create_wallet(WALLET_CONFIG, &credentials);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            test::cleanup_storage("indy_create_wallet_works_for_raw_key_invalid_length");
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works_for_not_created"}"#;
            utils::setup("indy_delete_wallet_works_for_not_created");

            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);

            utils::tear_down("indy_delete_wallet_works_for_not_created");
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works_for_twice"}"#;
            utils::setup("indy_delete_wallet_works_for_twice");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);

            utils::tear_down("indy_delete_wallet_works_for_twice");
        }

        #[test]
        fn indy_delete_wallet_works_for_wrong_credentials() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_delete_wallet_works_for_wrong_credentials"}"#;
            utils::setup("indy_delete_wallet_works_for_wrong_credentials");

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_code!(ErrorCode::WalletAccessFailed, res);

            utils::tear_down("indy_delete_wallet_works_for_wrong_credentials");
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            const WALLET_CONFIG: &'static str = r#"{"id":"indy_open_wallet_works_for_not_created_wallet"}"#;
            utils::setup("indy_open_wallet_works_for_not_created_wallet");

            let res = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);

            utils::tear_down("indy_open_wallet_works_for_not_created_wallet");
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_open_wallet_works_for_twice"}"#;
            utils::setup("indy_open_wallet_works_for_twice");

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletAlreadyOpenedError, res);

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down("indy_open_wallet_works_for_twice");
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            utils::setup("indy_open_wallet_works_for_two_wallets");

            let wallet_config_1 = r#"{"id":"indy_open_wallet_works_for_two_wallets1"}"#;
            let wallet_config_2 = r#"{"id":"indy_open_wallet_works_for_two_wallets2"}"#;

            wallet::create_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            let wallet_handle_1 = wallet::open_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            let wallet_handle_2 = wallet::open_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle_1).unwrap();
            wallet::close_wallet(wallet_handle_2).unwrap();

            cleanup_wallet("indy_open_wallet_works_for_two_wallets1");
            cleanup_wallet("indy_open_wallet_works_for_two_wallets2");
            utils::tear_down("indy_open_wallet_works_for_two_wallets");
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths() {
            utils::setup("indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths");

            let wallet_config_1 = json!({
                "id": "indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths",
            }).to_string();

            let wallet_config_2 = json!({
                "id": "indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path("indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths"),
                }
            }).to_string();

            wallet::create_wallet(&wallet_config_1, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&wallet_config_2, WALLET_CREDENTIALS).unwrap();

            let wallet_handle_1 = wallet::open_wallet(&wallet_config_1, WALLET_CREDENTIALS).unwrap();
            let wallet_handle_2 = wallet::open_wallet(&wallet_config_2, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle_1).unwrap();
            wallet::close_wallet(wallet_handle_2).unwrap();

            wallet::delete_wallet(&wallet_config_1, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&wallet_config_2, WALLET_CREDENTIALS).unwrap();

            utils::tear_down("indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths");
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_credentials() {
            const WALLET_CONFIG: &'static str = r#"{"id":"indy_open_wallet_works_for_invalid_credentials"}"#;
            utils::setup("indy_open_wallet_works_for_invalid_credentials");

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_code!(ErrorCode::WalletAccessFailed, res);

            utils::tear_down("indy_open_wallet_works_for_invalid_credentials");
        }

        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            const WALLET_CONFIG: &'static str = r#"{"id":"indy_open_wallet_works_for_changing_credentials"}"#;
            utils::setup("indy_open_wallet_works_for_changing_credentials");

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"key", "rekey":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down("indy_open_wallet_works_for_changing_credentials");
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_config() {
            const WALLET_CONFIG: &'static str = r#"{"id":"indy_open_wallet_works_for_invalid_config"}"#;
            utils::setup("indy_open_wallet_works_for_invalid_config");

            let config = r#"{"field":"value"}"#;

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::open_wallet(config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_open_wallet_works_for_invalid_config");
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works_for_invalid_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_close_wallet_works_for_invalid_handle");

            let res = wallet::close_wallet(INVALID_WALLET_HANDLE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_close_wallet_works_for_invalid_handle", &wallet_config);
        }

        #[test]
        fn indy_close_wallet_works_for_twice() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_close_wallet_works_for_twice");

            wallet::close_wallet(wallet_handle).unwrap();
            let res = wallet::close_wallet(wallet_handle);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();
        }
    }

    mod export_wallet {
        use super::*;
        use std::fs;

        #[test]
        fn indy_export_wallet_returns_error_if_path_exists() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_export_wallet_returns_error_if_path_exists");

            let path = wallet::export_wallet_path("indy_export_wallet_returns_error_if_path_exists_export_wallet");
            let config_json = wallet::prepare_export_wallet_config(&path);

            fs::DirBuilder::new()
                .recursive(true)
                .create(path.clone()).unwrap();

            let res = wallet::export_wallet(wallet_handle, &config_json);
            assert_code!(ErrorCode::CommonIOError, res);

            fs::remove_dir_all(path).unwrap();
            utils::tear_down_with_wallet(wallet_handle, "indy_export_wallet_returns_error_if_path_exists", &wallet_config);
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_config() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_export_wallet_returns_error_if_invalid_config");

            let res = wallet::export_wallet(wallet_handle, "{}");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_export_wallet_returns_error_if_invalid_config", &wallet_config);
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_export_wallet_returns_error_if_invalid_handle");

            let path = wallet::export_wallet_path("indy_export_wallet_returns_error_if_invalid_handle");
            let config_json = wallet::prepare_export_wallet_config(&path);

            let res = wallet::export_wallet(INVALID_WALLET_HANDLE, &config_json);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_export_wallet_returns_error_if_invalid_handle", &wallet_config);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_returns_error_if_path_doesnt_exist() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist"}"#;
            utils::setup("indy_import_wallet_returns_error_if_path_doesnt_exist");

            let path = wallet::export_wallet_path("indy_import_wallet_returns_error_if_path_doesnt_exist");
            let config_json = wallet::prepare_export_wallet_config(&path);

            let wallet_config = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist"}"#;
            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::CommonIOError, res);

            let res = wallet::open_wallet(wallet_config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);

            utils::tear_down("indy_import_wallet_returns_error_if_path_doesnt_exist");
        }

        #[test]
        fn indy_import_wallet_returns_error_if_invalid_config() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist"}"#;
            utils::setup("indy_import_wallet_returns_error_if_path_doesnt_exist");

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, "{}");
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_import_wallet_returns_error_if_path_doesnt_exist");
        }

        #[test]
        fn indy_import_wallet_works_for_other_key() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_import_wallet_works_for_other_key"}"#;
            utils::setup("indy_import_wallet_works_for_other_key");

            let path = wallet::export_wallet_path("indy_import_wallet_works_for_other_key_export_wallet");
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let config_json = json!({
                "path": path.to_str().unwrap(),
                "key": "other_key",
            }).to_string();

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            cleanup_file(&path);
            utils::tear_down("indy_import_wallet_works_for_other_key");
        }

        #[test]
        fn indy_import_wallet_works_for_duplicate_name() {
            const WALLET_CONFIG: &str = r#"{"id":"indy_import_wallet_works_for_duplicate_name"}"#;
            utils::setup("indy_import_wallet_works_for_duplicate_name");

            let path = wallet::export_wallet_path("indy_import_wallet_works_for_duplicate_name_export_wallet");
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::WalletAlreadyExistsError, res);

            wallet::close_wallet(wallet_handle).unwrap();

            cleanup_file(&path);
            utils::tear_down("indy_import_wallet_works_for_duplicate_name");
        }
    }
}

fn _custom_path(name: &str) -> String {
    let mut path = environment::tmp_path();
    path.push(name);
    path.to_str().unwrap().to_owned()
}

