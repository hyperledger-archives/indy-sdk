extern crate indyrs as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate futures;
extern crate indy_sys;

use indy::did;
use indy::wallet;

use indy::ErrorCode;

use std::path::{Path, PathBuf};

mod utils;

use utils::constants::{DEFAULT_CREDENTIALS, METADATA};
use utils::file::{TempDir, TempFile};
use utils::rand;
#[allow(unused_imports)]
use futures::Future;

const EXPORT_KEY: &str = "TheScythesHangInTheAppleTrees";


mod wallet_config {
    use super::*;
    
    #[inline]
    pub fn new() -> String {
        json!({
            "id": rand::random_string(20)
        }).to_string()
    }

    #[inline]
    pub fn with_storage(storage: &str) -> String {
        json!({
            "id": rand::random_string(20),
            "storage_type": storage,
        }).to_string()
    }

    #[inline]
    pub fn with_custom_path<P: AsRef<Path>>(path: P) -> String {
        json!({
            "id": rand::random_string(20),
            "storage_type": "default",
            "storage_config": {
                "path": path.as_ref().to_str()
            }
        }).to_string()
    }

    pub mod export {
        use super::*;
        
        #[inline]
        pub fn new<P: AsRef<Path>>(path: P, key: &str) -> String {
            json!({
                "path": path.as_ref(),
                "key": key
            }).to_string()
        }

        pub fn with_defaults() -> (String, PathBuf, TempDir) {
            let dir = TempDir::new(None).unwrap();
            let path = dir.as_ref().join("wallet_export");
            let config = wallet_config::export::new(&path, EXPORT_KEY);

            (config, path, dir)
        }
    }
}



#[cfg(test)]
mod test_wallet_register {
    // Future work
}

#[cfg(test)]
mod test_wallet_create {
    use super::*;
    const CREDENTIALS: &str = r#"{"key":"9DXvkIMD7iSgD&RT$XYjHo0t"}"#;
    use futures::Future;

    #[test]
    fn create_default_wallet() {
        let config = wallet_config::with_storage("default");
        
        let result = wallet::create_wallet(&config, CREDENTIALS).wait();

        assert_eq!((), result.unwrap());

        wallet::delete_wallet(&config, CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn create_default_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);

        let result = wallet::create_wallet(&config, CREDENTIALS).wait();

        assert_eq!((), result.unwrap());

        wallet::delete_wallet(&config, CREDENTIALS).wait().unwrap();
    }

    // #[test]
    // fn create_wallet_custom_storage_type() {
    //     unimplemented!();
    // }

    #[test]
    fn create_wallet_unknown_storage_type() {
        let config = wallet_config::with_storage("unknown");

        let result = wallet::create_wallet(&config, CREDENTIALS).wait();

        assert_eq!(ErrorCode::WalletUnknownTypeError, result.unwrap_err().error_code);
    }

    #[test]
    fn create_wallet_empty_storage_type() {
        let config = wallet_config::new();

        let result = wallet::create_wallet(&config, CREDENTIALS).wait();

        assert_eq!((), result.unwrap());

        wallet::delete_wallet(&config, CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn create_wallet_without_key() {
        let config = wallet_config::new();
        let credentials = "{}";

        let result = wallet::create_wallet(&config, credentials).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn create_wallet_without_encryption() {
        let config = wallet_config::new();
        let credentials = json!({"key": ""}).to_string();

        let result = wallet::create_wallet(&config, &credentials).wait();

        assert_eq!((), result.unwrap());

        wallet::delete_wallet(&config, &credentials).wait().unwrap();
    }
}


#[cfg(test)]
mod test_wallet_delete {
    use super::*;
    use futures::Future;

    #[inline]
    fn assert_wallet_deleted(config: &str, credentials: &str) {
        let result = wallet::open_wallet(config, credentials).wait();
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err().error_code);
    }

    #[test]
    fn delete_wallet() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_closed() {
        let config = wallet_config::new();

        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        wallet::close_wallet(handle).wait().unwrap();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();
        
        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_opened() {
        let config = wallet_config::new();

        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();
        
        assert_eq!(ErrorCode::CommonInvalidState, result.unwrap_err().error_code);

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    // #[test]
    // fn delete_registered_wallet() {
    //     unimplemented!();
    // }

    #[test]
    fn delete_wallet_repeated_command() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err().error_code);
    }

    #[test]
    fn delete_wallet_invalid_credentials() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::delete_wallet(&config, r#"{"key": "badkey"}"#).wait();

        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err().error_code);

        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn delete_wallet_uncreated() {
        let config = wallet_config::new();

        let result = wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err().error_code);
    }

}

#[cfg(test)]
mod test_wallet_open {
    use super::*;
    
    #[test]
    fn open_wallet() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn open_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);

        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    // #[test]
    // fn open_wallet_registered() {
    //     unimplemented!();
    // }

    #[test]
    fn open_wallet_not_created() {
        let config = wallet_config::new();

        let result = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait();
        
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err().error_code);
    }

    #[test]
    fn open_wallet_repeated_command() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!(ErrorCode::WalletAlreadyOpenedError, result.unwrap_err().error_code);

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn open_wallet_two_same_time() {
        let config1 = wallet_config::new();
        let config2 = wallet_config::new();

        wallet::create_wallet(&config1, DEFAULT_CREDENTIALS).wait().unwrap();
        wallet::create_wallet(&config2, DEFAULT_CREDENTIALS).wait().unwrap();

        let handle1 = wallet::open_wallet(&config1, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle2 = wallet::open_wallet(&config2, DEFAULT_CREDENTIALS).wait().unwrap();

        wallet::close_wallet(handle1).wait().unwrap();
        wallet::close_wallet(handle2).wait().unwrap();
        wallet::delete_wallet(&config1, DEFAULT_CREDENTIALS).wait().unwrap();
        wallet::delete_wallet(&config2, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn open_wallet_invalid_credentials() {
        let config = wallet_config::new();
        let credentials = json!({"key": "xylophone rat"}).to_string();

        wallet::create_wallet(&config, &credentials).wait().unwrap();

        let result = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait();

        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err().error_code);

        wallet::delete_wallet(&config, &credentials).wait().unwrap();
    }

    #[test]
    fn open_wallet_change_credentials() {
        let config = wallet_config::new();
        let credentials1 = json!({"key": "key_1"}).to_string();
        let credentials2 = json!({"key": "key_2"}).to_string();
        let rekey = json!({"key": "key_1", "rekey": "key_2"}).to_string();

        wallet::create_wallet(&config, &credentials1).wait().unwrap();

        let handle = wallet::open_wallet(&config, &rekey).wait().unwrap();
        wallet::close_wallet(handle).wait().unwrap();

        let result = wallet::open_wallet(&config, &credentials1).wait();
        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err().error_code);

        let handle = wallet::open_wallet(&config, &credentials2).wait().unwrap();
        wallet::close_wallet(handle).wait().unwrap();

        wallet::delete_wallet(&config, &credentials2).wait().unwrap();
    }

    #[test]
    fn open_wallet_invalid_config() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::open_wallet("{}", DEFAULT_CREDENTIALS).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);

        wallet::delete_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
    }
}

#[cfg(test)]
mod test_wallet_close {
    use super::*;
    use indy::INVALID_WALLET_HANDLE;

    #[test]
    fn close_wallet() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::close_wallet(handle).wait();

        assert_eq!((), result.unwrap());
    }

    // #[test]
    // fn close_wallet_registered() {
    //     unimplemented!();
    // }

    #[test]
    fn close_wallet_invalid_handle() {
        let result = wallet::close_wallet(INVALID_WALLET_HANDLE).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }

    #[test]
    fn close_wallet_duplicate_command() {
        let config = wallet_config::new();
        wallet::create_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::close_wallet(handle).wait();

        assert_eq!((), result.unwrap());

        let result = wallet::close_wallet(handle).wait();

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }

}

#[cfg(test)]
mod test_wallet_export {
    use super::*;
    use indy::INVALID_WALLET_HANDLE;

    #[test]
    fn export_wallet() {
        let config_wallet = wallet_config::new();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        wallet::create_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::export_wallet(handle, &config_export).wait();

        assert_eq!((), result.unwrap());
        
        assert!(path.exists());

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn export_wallet_path_already_exists() {
        let config_wallet = wallet_config::new();
        let file = TempFile::new(None).unwrap();
        let config_export = wallet_config::export::new(&file, EXPORT_KEY);

        wallet::create_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::export_wallet(handle, &config_export).wait();

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err().error_code);
        
        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn export_wallet_invalid_config() {
        let config_wallet = wallet_config::new();
        wallet::create_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
        let handle = wallet::open_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::export_wallet(handle, "{}").wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn export_wallet_invalid_handle() {
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        let result = wallet::export_wallet(INVALID_WALLET_HANDLE, &config_export).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
        assert!(!path.exists());
    }
}

#[cfg(test)]
mod test_wallet_import {
    use super::*;

    fn setup_exported_wallet(
        config_wallet: &str,
        credentials: &str,
        config_export: &str
    ) -> (String, String) {
        wallet::create_wallet(&config_wallet, credentials).wait().unwrap();
        let handle = wallet::open_wallet(&config_wallet, credentials).wait().unwrap();

        let (did, _) = did::create_and_store_my_did(handle, "{}").wait().unwrap();
        did::set_did_metadata(handle, &did, METADATA).wait().unwrap();
        let did_with_metadata = did::get_did_metadata(handle, &did).wait().unwrap();

        wallet::export_wallet(handle, &config_export).wait().unwrap();

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        (did, did_with_metadata)
    }

    #[test]
    fn import_wallet() {
        let config_wallet = wallet_config::new();
        let (config_export, _path, _dir) = wallet_config::export::with_defaults();
        let (did, did_with_metadata) = setup_exported_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        let result = wallet::import_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        ).wait();

        assert_eq!((), result.unwrap());

        let handle = wallet::open_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        let imported_did_with_metadata = did::get_did_metadata(handle, &did).wait().unwrap();

        assert_eq!(did_with_metadata, imported_did_with_metadata);

        wallet::close_wallet(handle).wait().unwrap();
        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
    }

    #[test]
    fn import_wallet_invalid_path() {
        let config_wallet = wallet_config::new();
        let non_existant_path = Path::new("PlaceWithoutWindOrWords");
        let config_export = wallet_config::export::new(
            &non_existant_path,
            EXPORT_KEY
        );

        let result = wallet::import_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        ).wait();

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err().error_code);

        let result = wallet::open_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait();
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err().error_code);
    }

    #[test]
    fn import_wallet_invalid_config() {
        let config_wallet = wallet_config::new();

        let result = wallet::import_wallet(&config_wallet, DEFAULT_CREDENTIALS, "{}").wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn import_wallet_invalid_key() {
        let config_wallet = wallet_config::new();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();
        setup_exported_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        let config_import = wallet_config::export::new(path, "bad_key");

        let result = wallet::import_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_import
        ).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn import_wallet_duplicate_name() {
        let config_wallet = wallet_config::new();
        let (config_export, _path, _dir) = wallet_config::export::with_defaults();
        setup_exported_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        wallet::create_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();

        let result = wallet::import_wallet(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        ).wait();

        assert_eq!(ErrorCode::WalletAlreadyExistsError, result.unwrap_err().error_code);

        wallet::delete_wallet(&config_wallet, DEFAULT_CREDENTIALS).wait().unwrap();
    }
}
