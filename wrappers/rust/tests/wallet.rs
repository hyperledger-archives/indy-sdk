extern crate rust_libindy_wrapper as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;

use indy::did::Did;
use indy::wallet::Wallet;

use indy::ErrorCode;

use std::path::{Path, PathBuf};
use std::panic;
use std::sync::mpsc::channel;
use std::time::Duration;

mod utils;

use utils::constants::{DEFAULT_CREDENTIALS, INVALID_HANDLE, METADATA};
use utils::file::{TempDir, TempFile};
use utils::rand;

const VALID_TIMEOUT: Duration = Duration::from_secs(5);
const INVALID_TIMEOUT: Duration = Duration::from_micros(1);
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

    #[test]
    fn create_default_wallet() {
        let config = wallet_config::with_storage("default");
        
        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_default_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);

        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    // #[test]
    // fn create_wallet_custom_storage_type() {
    //     unimplemented!();
    // }

    #[test]
    fn create_wallet_unknown_storage_type() {
        let config = wallet_config::with_storage("unknown");

        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!(ErrorCode::WalletUnknownTypeError, result.unwrap_err());
    }

    #[test]
    fn create_wallet_empty_storage_type() {
        let config = wallet_config::new();

        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_wallet_without_key() {
        let config = wallet_config::new();
        let credentials = "{}";

        let result = Wallet::create(&config, credentials);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn create_wallet_without_encryption() {
        let config = wallet_config::new();
        let credentials = json!({"key": ""}).to_string();

        let result = Wallet::create(&config, &credentials);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, &credentials).unwrap();
    }

    #[test]
    fn create_default_wallet_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::with_storage("default");

        Wallet::create_async(
            &config,
            CREDENTIALS,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        
        assert_eq!(ErrorCode::Success, ec);

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_wallet_unknown_storage_type_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::with_storage("unknown");

        Wallet::create_async(
            &config,
            CREDENTIALS,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletUnknownTypeError, ec);
    }

    #[test]
    fn create_default_wallet_timeout() {
        let config = wallet_config::with_storage("default");

        let result = Wallet::create_timeout(
            &config,
            CREDENTIALS,
            VALID_TIMEOUT
        );

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_wallet_unknown_storage_type_timeout() {
        let config = wallet_config::with_storage("unknown");

        let result = Wallet::create_timeout(
            &config,
            CREDENTIALS,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletUnknownTypeError, result.unwrap_err());
    }

    #[test]
    fn create_wallet_timeout_timeouts() {
        let config = wallet_config::with_storage("unknown");

        let result = Wallet::create_timeout(
            &config,
            CREDENTIALS,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}


#[cfg(test)]
mod test_wallet_delete {
    use super::*;

    #[inline]
    fn assert_wallet_deleted(config: &str, credentials: &str) {
        let result = Wallet::open(config, credentials);
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn delete_wallet() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);

        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);

        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_closed() {
        let config = wallet_config::new();

        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();
        Wallet::close(handle).unwrap();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);
        
        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_opened() {
        let config = wallet_config::new();

        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);
        
        assert_eq!(ErrorCode::CommonInvalidState, result.unwrap_err());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    // #[test]
    // fn delete_registered_wallet() {
    //     unimplemented!();
    // }

    #[test]
    fn delete_wallet_repeated_command() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn delete_wallet_invalid_credentials() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete(&config, r#"{"key": "badkey"}"#);

        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err());

        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn delete_wallet_uncreated() {
        let config = wallet_config::new();

        let result = Wallet::delete(&config, DEFAULT_CREDENTIALS);

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn delete_wallet_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        Wallet::delete_async(
            &config,
            DEFAULT_CREDENTIALS,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv().unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_uncreated_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::new();

        Wallet::delete_async(
            &config,
            DEFAULT_CREDENTIALS,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv().unwrap();

        assert_eq!(ErrorCode::WalletNotFoundError, ec);
    }

    #[test]
    fn delete_wallet_timeout() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::delete_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            VALID_TIMEOUT,
        );

        assert_eq!((), result.unwrap());
        assert_wallet_deleted(&config, DEFAULT_CREDENTIALS);
    }

    #[test]
    fn delete_wallet_uncreated_timeout() {
        let config = wallet_config::new();

        let result = Wallet::delete_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            VALID_TIMEOUT,
        );

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn delete_wallet_timeout_timeouts() {
        let config = wallet_config::new();

        let result = Wallet::delete_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            INVALID_TIMEOUT,
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_wallet_open {
    use super::*;
    
    #[test]
    fn open_wallet() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = wallet_config::with_custom_path(&dir);

        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    // #[test]
    // fn open_wallet_registered() {
    //     unimplemented!();
    // }

    #[test]
    fn open_wallet_not_created() {
        let config = wallet_config::new();

        let result = Wallet::open(&config, DEFAULT_CREDENTIALS);
        
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn open_wallet_repeated_command() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::open(&config, DEFAULT_CREDENTIALS);

        assert_eq!(ErrorCode::WalletAlreadyOpenedError, result.unwrap_err());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_two_same_time() {
        let config1 = wallet_config::new();
        let config2 = wallet_config::new();

        Wallet::create(&config1, DEFAULT_CREDENTIALS).unwrap();
        Wallet::create(&config2, DEFAULT_CREDENTIALS).unwrap();

        let handle1 = Wallet::open(&config1, DEFAULT_CREDENTIALS).unwrap();
        let handle2 = Wallet::open(&config2, DEFAULT_CREDENTIALS).unwrap();

        Wallet::close(handle1).unwrap();
        Wallet::close(handle2).unwrap();
        Wallet::delete(&config1, DEFAULT_CREDENTIALS).unwrap();
        Wallet::delete(&config2, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_invalid_credentials() {
        let config = wallet_config::new();
        let credentials = json!({"key": "xylophone rat"}).to_string();

        Wallet::create(&config, &credentials).unwrap();

        let result = Wallet::open(&config, DEFAULT_CREDENTIALS);

        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err());

        Wallet::delete(&config, &credentials).unwrap();
    }

    #[test]
    fn open_wallet_change_credentials() {
        let config = wallet_config::new();
        let credentials1 = json!({"key": "key_1"}).to_string();
        let credentials2 = json!({"key": "key_2"}).to_string();
        let rekey = json!({"key": "key_1", "rekey": "key_2"}).to_string();

        Wallet::create(&config, &credentials1).unwrap();

        let handle = Wallet::open(&config, &rekey).unwrap();
        Wallet::close(handle).unwrap();

        let result = Wallet::open(&config, &credentials1);
        assert_eq!(ErrorCode::WalletAccessFailed, result.unwrap_err());

        let handle = Wallet::open(&config, &credentials2).unwrap();
        Wallet::close(handle).unwrap();

        Wallet::delete(&config, &credentials2).unwrap();
    }

    #[test]
    fn open_wallet_invalid_config() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::open("{}", DEFAULT_CREDENTIALS);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());

        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        Wallet::open_async(
            &config,
            DEFAULT_CREDENTIALS,
            move |ec, handle| sender.send((ec, handle)).unwrap()
        );

        let (ec, handle) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_async_not_created() {
        let (sender, receiver) = channel();
        let config = wallet_config::new();

        Wallet::open_async(
            &config,
            DEFAULT_CREDENTIALS,
            move |ec, handle| sender.send((ec, handle)).unwrap()
        );

        let (ec, handle) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletNotFoundError, ec);
        assert_eq!(0, handle);
    }

    #[test]
    fn open_wallet_timeout() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let handle = Wallet::open_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            VALID_TIMEOUT
        ).unwrap();

        Wallet::close(handle).unwrap();
        Wallet::delete(&config, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn open_wallet_timeout_not_created() {
        let config = wallet_config::new();

        let result = Wallet::open_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn open_wallet_timeout_timeouts() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::open_timeout(
            &config,
            DEFAULT_CREDENTIALS,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_wallet_close {
    use super::*;
    
    #[test]
    fn close_wallet() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::close(handle);

        assert_eq!((), result.unwrap());
    }

    // #[test]
    // fn close_wallet_registered() {
    //     unimplemented!();
    // }

    #[test]
    fn close_wallet_invalid_handle() {
        let result = Wallet::close(INVALID_HANDLE);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn close_wallet_duplicate_command() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::close(handle);

        assert_eq!((), result.unwrap());

        let result = Wallet::close(handle);

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn close_wallet_async() {
        let (sender, receiver) = channel();
        let config = wallet_config::new();

        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        Wallet::close_async(handle, move |ec| sender.send(ec).unwrap());

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
    }

    #[test]
    fn close_wallet_async_invalid_handle() {
        let (sender, receiver) = channel();

        Wallet::close_async(INVALID_HANDLE, move |ec| sender.send(ec).unwrap());

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
    }

    #[test]
    fn close_wallet_timeout() {
        let config = wallet_config::new();
        Wallet::create(&config, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::close_timeout(handle, VALID_TIMEOUT);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn close_wallet_timeout_invalid_handle() {
        let result = Wallet::close_timeout(INVALID_HANDLE, VALID_TIMEOUT);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn close_wallet_timeout_timeouts() {
        let result = Wallet::close_timeout(INVALID_HANDLE, INVALID_TIMEOUT);
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_wallet_export {
    use super::*;

    #[test]
    fn export_wallet() {
        let config_wallet = wallet_config::new();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::export(handle, &config_export);

        assert_eq!((), result.unwrap());
        
        assert!(path.exists());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn export_wallet_path_already_exists() {
        let config_wallet = wallet_config::new();
        let file = TempFile::new(None).unwrap();
        let config_export = wallet_config::export::new(&file, EXPORT_KEY);

        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::export(handle, &config_export);

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        
        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn export_wallet_invalid_config() {
        let config_wallet = wallet_config::new();
        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::export(handle, "{}");

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn export_wallet_invalid_handle() {
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        let result = Wallet::export(INVALID_HANDLE, &config_export);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
        assert!(!path.exists());
    }

    #[test]
    fn export_wallet_async() {
        let (sender, receiver) = channel();
        let config_wallet = wallet_config::new();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        Wallet::export_async(
            handle,
            &config_export,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert!(path.exists());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn export_wallet_async_invalid_handle() {
        let (sender, receiver) = channel();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        Wallet::export_async(
            INVALID_HANDLE,
            &config_export,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
        assert!(!path.exists());
    }

    #[test]
    fn export_wallet_timeout() {
        let config_wallet = wallet_config::new();
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::export_timeout(
            handle,
            &config_export,
            VALID_TIMEOUT
        );

        assert_eq!((), result.unwrap());
        assert!(path.exists());

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn export_wallet_timeout_invalid_handle() {
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        let result = Wallet::export_timeout(
            INVALID_HANDLE,
            &config_export,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
        assert!(!path.exists());
    }

    #[test]
    fn export_wallet_timeout_timeouts() {
        let (config_export, path, _dir) = wallet_config::export::with_defaults();

        let result = Wallet::export_timeout(
            INVALID_HANDLE,
            &config_export,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
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
        Wallet::create(&config_wallet, credentials).unwrap();
        let handle = Wallet::open(&config_wallet, credentials).unwrap();

        let (did, _) = Did::new(handle, "{}").unwrap();
        Did::set_metadata(handle, &did, METADATA).unwrap();
        let did_with_metadata = Did::get_metadata(handle, &did).unwrap();

        Wallet::export(handle, &config_export).unwrap();

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

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

        let result = Wallet::import(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        assert_eq!((), result.unwrap());

        let handle = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let imported_did_with_metadata = Did::get_metadata(handle, &did).unwrap();

        assert_eq!(did_with_metadata, imported_did_with_metadata);

        Wallet::close(handle).unwrap();
        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }

    #[test]
    fn import_wallet_invalid_path() {
        let config_wallet = wallet_config::new();
        let non_existant_path = Path::new("PlaceWithoutWindOrWords");
        let config_export = wallet_config::export::new(
            &non_existant_path,
            EXPORT_KEY
        );

        let result = Wallet::import(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());

        let result = Wallet::open(&config_wallet, DEFAULT_CREDENTIALS);
        assert_eq!(ErrorCode::WalletNotFoundError, result.unwrap_err());
    }

    #[test]
    fn import_wallet_invalid_config() {
        let config_wallet = wallet_config::new();

        let result = Wallet::import(&config_wallet, DEFAULT_CREDENTIALS, "{}");

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
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

        let result = Wallet::import(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_import
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
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

        Wallet::create(&config_wallet, DEFAULT_CREDENTIALS).unwrap();

        let result = Wallet::import(
            &config_wallet,
            DEFAULT_CREDENTIALS,
            &config_export
        );

        assert_eq!(ErrorCode::WalletAlreadyExistsError, result.unwrap_err());

        Wallet::delete(&config_wallet, DEFAULT_CREDENTIALS).unwrap();
    }
}
