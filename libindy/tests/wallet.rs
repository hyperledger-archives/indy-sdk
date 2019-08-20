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
use utils::Setup;

use self::indy::ErrorCode;
use std::path::PathBuf;
use std::fs;

fn cleanup_file(path: &PathBuf) {
    if path.exists() {
        fs::remove_file(path).unwrap();
    }
}

fn config(name: &str) -> String {
    json!({"id": name}).to_string()
}

mod high_cases {
    use super::*;

    mod register_wallet_storage {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_works() {
            let setup = Setup::empty();

            test::cleanup_storage(&setup.name);
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();

            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            let setup = Setup::empty();
            let config = config(&setup.name);
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_create_wallet_works_for_custom_path() {
            let setup = Setup::empty();

            let config = json!({
                "id": &setup.name,
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(&setup.name),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            Setup::empty();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            Setup::empty();

            let res = wallet::create_wallet(UNKNOWN_WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletUnknownTypeError, res);
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_delete_wallet_works_for_custom_path() {
            let setup = Setup::empty();

            let config = json!({
                "id": &setup.name,
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(&setup.name),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_delete_wallet_works_for_opened() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidState, res);

            wallet::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            Setup::empty();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            Setup::wallet();
        }

        #[test]
        fn indy_open_wallet_works_for_custom_path() {
            let setup = Setup::empty();

            let config = json!({
                "id": &setup.name,
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(&setup.name),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            Setup::empty();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            Setup::empty();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
        }
    }

    mod export_wallet {
        use super::*;

        #[test]
        fn indy_export_wallet_works() {
            let setup = Setup::wallet();

            let path = wallet::export_wallet_path(&setup.name);
            let config_json = wallet::prepare_export_wallet_config(&path);

            did::create_my_did(setup.wallet_handle, "{}").unwrap();
            did::create_my_did(setup.wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(setup.wallet_handle, &config_json).unwrap();

            assert!(path.exists());

            test::cleanup_files(&path, &setup.name);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_works() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let path = wallet::export_wallet_path(&setup.name);
            let config_json = wallet::prepare_export_wallet_config(&path);

            let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&setup.name).unwrap();

            let (did, _) = did::create_my_did(wallet_handle, "{}").unwrap();
            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let did_with_meta = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();

            wallet::import_wallet(&config, WALLET_CREDENTIALS, &config_json).unwrap();

            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            let did_with_meta_after_import = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            assert_eq!(did_with_meta, did_with_meta_after_import);

            wallet::close_and_delete_wallet(wallet_handle, &config).unwrap();
            cleanup_file(&path);
        }
    }

    mod generate_wallet_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_generate_wallet_key_works() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let key = wallet::generate_wallet_key(None).unwrap();

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(&config, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(&config, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&config, &credentials).unwrap();
        }

        #[test]
        fn indy_generate_wallet_key_works_for_seed() {
            let setup = Setup::empty();
            let wallet_config = config(&setup.name);

            let config = json!({"seed": MY1_SEED}).to_string();
            let key = wallet::generate_wallet_key(Some(config.as_str())).unwrap();
            assert_eq!(key.from_base58().unwrap(), vec![177, 92, 220, 199, 104, 203, 161, 4, 218, 78, 105, 13, 7, 50, 66, 107, 154, 155, 108, 133, 1, 30, 87, 149, 233, 76, 39, 156, 178, 46, 230, 124]);

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(&wallet_config, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(&wallet_config, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&wallet_config, &credentials).unwrap();
        }
    }
}

#[cfg(not(feature="only_high_cases"))]
mod medium_cases {
    extern crate libc;

    use super::*;
    use std::ffi::CString;

    use api::INVALID_WALLET_HANDLE;
    use utils::test::cleanup_wallet;

    mod register_wallet_type {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_does_not_work_twice_with_same_name() {
            Setup::empty();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            let res = wallet::register_wallet_storage(INMEM_TYPE, true).unwrap_err();
            assert_eq!(ErrorCode::WalletTypeAlreadyRegisteredError, res);

            InmemWallet::cleanup();
        }

        #[test]
        fn indy_register_wallet_storage_does_not_work_with_null_params() {
            Setup::empty();
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
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let res = wallet::create_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletAlreadyExistsError, res);
        }

        #[test]
        fn indy_create_wallet_works_for_missed_key() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let res = wallet::create_wallet(&config, r#"{}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            Setup::empty();
            let config = config("");

            let res = wallet::create_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_create_wallet_works_for_raw_key_invalid_length() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let credentials = json!({"key": "key", "key_derivation_method": "RAW"}).to_string();
            let res = wallet::create_wallet(&config, &credentials);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let res = wallet::delete_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);
        }

        #[test]
        fn indy_delete_wallet_works_for_wrong_credentials() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, r#"{"key":"key"}"#).unwrap();
            let res = wallet::delete_wallet(&config, r#"{"key":"other_key"}"#);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let res = wallet::open_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let res = wallet::open_wallet(&config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletAlreadyOpenedError, res);

            wallet::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            Setup::empty();

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
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets_with_same_ids_but_different_paths() {
            Setup::empty();

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
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_credentials() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, r#"{"key":"key"}"#).unwrap();
            let res = wallet::open_wallet(&config, r#"{"key":"other_key"}"#);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }

        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            wallet::create_wallet(&config, r#"{"key":"key"}"#).unwrap();
            let wallet_handle = wallet::open_wallet(&config, r#"{"key":"key", "rekey":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(&config, r#"{"key":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_config() {
            Setup::empty();

            let config = r#"{"field":"value"}"#;
            let res = wallet::open_wallet(config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works_for_invalid_handle() {
            Setup::empty();

            let res = wallet::close_wallet(INVALID_WALLET_HANDLE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_close_wallet_works_for_twice() {
            let setup = Setup::empty();

            let (wallet_handle, config) = wallet::create_and_open_default_wallet(&setup.name).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            let res = wallet::close_wallet(wallet_handle);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
        }
    }

    mod export_wallet {
        use super::*;
        use std::fs;

        #[test]
        fn indy_export_wallet_returns_error_if_path_exists() {
            let setup= Setup::wallet();

            let path = wallet::export_wallet_path(&setup.name);
            let config_json = wallet::prepare_export_wallet_config(&path);

            fs::DirBuilder::new()
                .recursive(true)
                .create(path.clone()).unwrap();

            let res = wallet::export_wallet(setup.wallet_handle, &config_json);
            assert_code!(ErrorCode::CommonIOError, res);

            fs::remove_dir_all(path).unwrap();
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_config() {
            let setup= Setup::wallet();

            let res = wallet::export_wallet(setup.wallet_handle, "{}");
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_handle() {
            let setup= Setup::empty();

            let path = wallet::export_wallet_path(&setup.name);
            let config_json = wallet::prepare_export_wallet_config(&path);

            let res = wallet::export_wallet(INVALID_WALLET_HANDLE, &config_json);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_returns_error_if_path_doesnt_exist() {
            let setup= Setup::empty();

            let import_config = json!({"id": &setup.name}).to_string();

            let path = wallet::export_wallet_path(&setup.name);
            let config_json = wallet::prepare_export_wallet_config(&path);

            let wallet_config = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist2"}"#;
            let res = wallet::import_wallet(&import_config, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::CommonIOError, res);

            let res = wallet::open_wallet(wallet_config, WALLET_CREDENTIALS);
            assert_code!(ErrorCode::WalletNotFoundError, res);
        }

        #[test]
        fn indy_import_wallet_returns_error_if_invalid_config() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let res = wallet::import_wallet(&config, WALLET_CREDENTIALS, "{}");
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_import_wallet_works_for_other_key() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let path = wallet::export_wallet_path("indy_import_wallet_works_for_other_key_export_wallet");
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();

            let config_json = json!({
                "path": path.to_str().unwrap(),
                "key": "other_key",
            }).to_string();

            let res = wallet::import_wallet(&config, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            cleanup_file(&path);
        }

        #[test]
        fn indy_import_wallet_works_for_duplicate_name() {
            let setup = Setup::empty();
            let config = config(&setup.name);

            let path = wallet::export_wallet_path("indy_import_wallet_works_for_duplicate_name_export_wallet");
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            cleanup_file(&path);
            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            let res = wallet::import_wallet(&config, WALLET_CREDENTIALS, &config_json);
            assert_code!(ErrorCode::WalletAlreadyExistsError, res);

            wallet::close_wallet(wallet_handle).unwrap();

            cleanup_file(&path);
        }
    }
}

fn _custom_path(name: &str) -> String {
    let mut path = environment::tmp_path();
    path.push(name);
    path.to_str().unwrap().to_owned()
}

