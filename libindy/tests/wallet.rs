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
extern crate indy;
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;
extern crate os_type;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
mod utils;

use utils::inmem_wallet::InmemWallet;
use utils::{environment, wallet, test, did};
use utils::constants::*;

use indy::api::ErrorCode;

pub const CONFIG: &'static str = r#"{"freshness_time":1000}"#;

mod high_cases {
    use super::*;

    mod register_wallet_storage {
        use super::*;

        #[test]
        fn indy_register_wallet_storage_works() {
            utils::setup();

            test::cleanup_storage();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();

            InmemWallet::cleanup();
            utils::tear_down();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            utils::setup();

            wallet::create_wallet(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_custom_path() {
            utils::setup();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            utils::setup();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            utils::setup();

            let res = wallet::create_wallet(UNKNOWN_WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletUnknownTypeError);

            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_custom_path() {
            utils::setup();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(&config, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_opened() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            utils::setup();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            InmemWallet::cleanup();
            utils::tear_down();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_custom_path() {
            utils::setup();

            let config = json!({
                "id": "wallet_1",
                "storage_type": "default",
                "storage_config": {
                    "path": _custom_path(),
                }
            }).to_string();

            wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(&config, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            utils::setup();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
            utils::tear_down();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            utils::setup();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            wallet::create_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(INMEM_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            InmemWallet::cleanup();
            utils::tear_down();
        }
    }

    mod export_wallet {
        use super::*;

        #[test]
        fn indy_export_wallet_works() {
            let wallet_handle = utils::setup_with_wallet();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            did::create_my_did(wallet_handle, "{}").unwrap();
            did::create_my_did(wallet_handle, "{}").unwrap();

            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            assert!(path.exists());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_works() {
            utils::setup();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let (did, _) = did::create_my_did(wallet_handle, "{}").unwrap();
            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let did_with_meta = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let did_with_meta_after_import = did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            assert_eq!(did_with_meta, did_with_meta_after_import);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod generate_wallet_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_generate_wallet_key_works() {
            test::cleanup_storage();

            let key = wallet::generate_wallet_key(None).unwrap();

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(WALLET_CONFIG, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, &credentials).unwrap();

            test::cleanup_storage();
        }

        #[test]
        fn indy_generate_wallet_key_works_for_seed() {
            test::cleanup_storage();

            let config = json!({"seed": MY1_SEED}).to_string();
            let key = wallet::generate_wallet_key(Some(config.as_str())).unwrap();
            assert_eq!(key.from_base58().unwrap(), vec![177, 92, 220, 199, 104, 203, 161, 4, 218, 78, 105, 13, 7, 50, 66, 107, 154, 155, 108, 133, 1, 30, 87, 149, 233, 76, 39, 156, 178, 46, 230, 124]);

            let credentials = json!({"key": key, "key_derivation_method": "RAW"}).to_string();
            wallet::create_wallet(WALLET_CONFIG, &credentials).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, &credentials).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, &credentials).unwrap();

            test::cleanup_storage();
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
            utils::setup();
            InmemWallet::cleanup();

            wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();
            let res = wallet::register_wallet_storage(INMEM_TYPE, true);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletTypeAlreadyRegisteredError);

            InmemWallet::cleanup();
            utils::tear_down();
        }

        #[test]
        fn indy_register_wallet_storage_does_not_work_with_null_params() {
            utils::setup();
            InmemWallet::cleanup();

            let xtype = CString::new(INMEM_TYPE).unwrap();
            let res = indy_register_wallet_storage(1, xtype.as_ptr(), None, None, None, None, None,
                                                   None, None, None, None, None,
                                                   None, None, None, None, None, None,
                                                   None, None, None, None,
                                                   None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam3);

            InmemWallet::cleanup();
            utils::tear_down();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyExistsError);

            test::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_missed_key() {
            utils::setup();

            let res = wallet::create_wallet(WALLET_CONFIG, r#"{}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            utils::setup();

            let res = wallet::create_wallet(r#"{"id": ""}"#, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            utils::tear_down();
        }

        #[test]
        fn indy_create_wallet_works_for_raw_key_invalid_length() {
            test::cleanup_storage();

            let credentials = json!({"key": "key", "key_derivation_method": "RAW"}).to_string();
            let res = wallet::create_wallet(WALLET_CONFIG, &credentials);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            test::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            utils::setup();

            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            utils::tear_down();
        }

        #[test]
        fn indy_delete_wallet_works_for_wrong_credentials() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = wallet::delete_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAccessFailed);

            utils::tear_down();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            utils::setup();

            let res = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyOpenedError);

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            utils::setup();

            let wallet_config_1 = r#"{"id":"indy_open_wallet_works_for_two_wallets1"}"#;
            let wallet_config_2 = r#"{"id":"indy_open_wallet_works_for_two_wallets2"}"#;

            wallet::create_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            wallet::create_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            let wallet_handle_1 = wallet::open_wallet(wallet_config_1, WALLET_CREDENTIALS).unwrap();
            let wallet_handle_2 = wallet::open_wallet(wallet_config_2, WALLET_CREDENTIALS).unwrap();

            wallet::close_wallet(wallet_handle_1).unwrap();
            wallet::close_wallet(wallet_handle_2).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_credentials() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let res = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            utils::setup();

            wallet::create_wallet(WALLET_CONFIG, r#"{"key":"key"}"#).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"key", "rekey":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, r#"{"key":"other_key"}"#).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_config() {
            utils::setup();

            let config = r#"{"field":"value"}"#;

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let res = wallet::open_wallet(config, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            utils::tear_down();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works_for_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let res = wallet::close_wallet(wallet_handle + 1);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_close_wallet_works_for_twice() {
            let wallet_handle = utils::setup_with_wallet();

            wallet::close_wallet(wallet_handle).unwrap();
            let res = wallet::close_wallet(wallet_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            utils::tear_down();
        }
    }

    mod export_wallet {
        use super::*;
        use std::fs;

        #[test]
        fn indy_export_wallet_returns_error_if_path_exists() {
            let wallet_handle = utils::setup_with_wallet();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            fs::DirBuilder::new()
                .recursive(true)
                .create(path).unwrap();

            let res = wallet::export_wallet(wallet_handle, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_config() {
            let wallet_handle = utils::setup_with_wallet();

            let res = wallet::export_wallet(wallet_handle, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_export_wallet_returns_error_if_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            let res = wallet::export_wallet(wallet_handle + 1, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod import_wallet {
        use super::*;

        #[test]
        fn indy_import_wallet_returns_error_if_path_doesnt_exist() {
            utils::setup();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            let wallet_config = r#"{"id":"indy_import_wallet_returns_error_if_path_doesnt_exist"}"#;
            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            let res = wallet::open_wallet(wallet_config, WALLET_CREDENTIALS);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            utils::tear_down();
        }

        #[test]
        fn indy_import_wallet_returns_error_if_invalid_config() {
            utils::setup();

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            utils::tear_down();
        }

        #[test]
        fn indy_import_wallet_works_for_other_key() {
            utils::setup();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let config_json = json!({
                "path": path.to_str().unwrap(),
                "key": "other_key",
            }).to_string();

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down();
        }

        #[test]
        fn indy_import_wallet_works_for_duplicate_name() {
            utils::setup();

            let path = wallet::export_wallet_path();
            let config_json = wallet::prepare_export_wallet_config(&path);

            wallet::create_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            did::create_my_did(wallet_handle, "{}").unwrap();

            wallet::export_wallet(wallet_handle, &config_json).unwrap();

            let res = wallet::import_wallet(WALLET_CONFIG, WALLET_CREDENTIALS, &config_json);
            assert_eq!(ErrorCode::WalletAlreadyExistsError, res.unwrap_err());

            wallet::close_wallet(wallet_handle).unwrap();
            utils::tear_down();
        }
    }
}

mod dynamic_storage_cases {
    extern crate libc;

    use super::*;
    use std::env;
    use std::collections::HashMap;
    use utils::domain::wallet::{Config, Credentials};
    use serde_json::Value;

    mod configure_wallet_storage {
        use super::*;

        fn save_config_overrides() -> HashMap<String, Option<String>> {
            // save (and clear) existing vars
            let hs_keep = utils::wallet::wallet_storage_overrides();
            for (key, _val) in hs_keep.iter() {
                env::remove_var(key);
            }
            hs_keep
        }

        fn restore_config_overrides(hs_keep: HashMap<String, Option<String>>) {
            // restore original vars
            for (key, val) in hs_keep.iter() {
                match val {
                    Some(hval) => env::set_var(key, hval),
                    None => ()
                }
            }
        }

        fn some_test_overrides() -> HashMap<String, Option<String>> {
            let mut storage_config = HashMap::new();
            let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];
            storage_config.insert(env_vars[0].to_string(), Some(r#"{"conf1":"key1", "conf2":"key2"}"#.to_string()));
            storage_config.insert(env_vars[1].to_string(), Some(r#"{"account":"key1", "password":"key2"}"#.to_string()));
            storage_config.insert(env_vars[2].to_string(), Some("type1".to_string()));
            storage_config.insert(env_vars[3].to_string(), Some("./a/library.so".to_string()));
            storage_config.insert(env_vars[4].to_string(), Some("my_fn_".to_string()));
            storage_config
        }

        fn inmem_lib_test_overrides() -> HashMap<String, Option<String>> {
            let os = os_type::current_platform();
            let f_ext = match os.os_type {
                os_type::OSType::OSX => "dylib",
                _ => "so"
            };

            let mut storage_config = HashMap::new();
            let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];
            storage_config.insert(env_vars[0].to_string(), None);
            storage_config.insert(env_vars[1].to_string(), None);
            storage_config.insert(env_vars[2].to_string(), Some("inmem_custom".to_string()));
            storage_config.insert(env_vars[3].to_string(), Some(format!("../samples/storage/storage-inmem/target/debug/libindystrginmem.{}", f_ext)));
            storage_config.insert(env_vars[4].to_string(), Some("inmemwallet_fn_".to_string()));
            storage_config
        }

        #[test]
        fn configure_wallet_works_for_case() {
            let hs_keep = save_config_overrides();

            // test we correctly pick up the values
            env::set_var("STG_TYPE", "inmem_mylocaltest");
            let hs = utils::wallet::wallet_storage_overrides();
            env::remove_var("STG_TYPE");

            restore_config_overrides(hs_keep);

            // finally assert our test
            assert_eq!(hs.get("STG_TYPE").unwrap(), &Some("inmem_mylocaltest".to_string()));
        }

        #[test]
        fn override_wallet_configuration_works() {
            let hs_vars = some_test_overrides();
            let new_config = utils::wallet::override_wallet_configuration(&UNKNOWN_WALLET_CONFIG, &hs_vars);

            let old_config: Config = serde_json::from_str(UNKNOWN_WALLET_CONFIG).unwrap();
            let new_config: Config = serde_json::from_str(&new_config).unwrap();
            let wconf = hs_vars.get("STG_CONFIG").as_ref().unwrap().as_ref().unwrap();
            let v: Value = serde_json::from_str(&wconf[..]).unwrap();

            assert_eq!(old_config.id, new_config.id);
            assert_eq!(new_config.storage_config, Some(v));
        }

        #[test]
        fn override_wallet_credentials_works() {
            let hs_vars = some_test_overrides();
            let new_creds = utils::wallet::override_wallet_credentials(&WALLET_CREDENTIALS_RAW, &hs_vars);

            let old_creds: Credentials = serde_json::from_str(WALLET_CREDENTIALS_RAW).unwrap();
            let new_creds: Credentials = serde_json::from_str(&new_creds).unwrap();
            let wcreds = hs_vars.get("STG_CREDS").as_ref().unwrap().as_ref().unwrap();
            let v: Value = serde_json::from_str(&wcreds[..]).unwrap();

            assert_eq!(old_creds.key, new_creds.key);
            assert_eq!(new_creds.storage_credentials, Some(v));
        }

        #[test]
        fn dynaload_wallet_storage_plugin_works() {
            utils::setup();

            // load dynamic lib and set config/creds based on overrides
            let hs_vars = inmem_lib_test_overrides();
            let new_config = utils::wallet::override_wallet_configuration(&UNKNOWN_WALLET_CONFIG, &hs_vars);
            let new_creds = utils::wallet::override_wallet_credentials(&WALLET_CREDENTIALS_RAW, &hs_vars);
            let res = utils::wallet::load_storage_library_config(&hs_vars);

            // confirm dynamic lib loaded ok
            assert_eq!(res, Ok(()));

            // verify wallet CCOD
            wallet::create_wallet(&new_config, &new_creds).unwrap();
            let wallet_handle = wallet::open_wallet(&new_config, &new_creds).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::delete_wallet(&new_config, &new_creds).unwrap();

            utils::tear_down();
        }
    }
}

fn _custom_path() -> String {
    let mut path = environment::tmp_path();
    path.push("custom_wallet_path");
    path.to_str().unwrap().to_owned()
}
