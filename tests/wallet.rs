extern crate rust_libindy_wrapper as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;

use indy::did::Did;
use indy::wallet::Wallet;

use indy::ErrorCode;

use std::path::Path;
use std::panic;
use std::sync::mpsc::channel;
use std::time::Duration;

mod utils;

use utils::{export_config_json, export_path};
use utils::constants::DEFAULT_CREDENTIALS;
use utils::file::TempDir;
use utils::rand;

const VALID_TIMEOUT: Duration = Duration::from_secs(5);
const INVALID_TIMEOUT: Duration = Duration::from_micros(1);



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
}

#[cfg(test)]
mod wallet_register {

}

#[cfg(test)]
mod wallet_tests {
    use super::*;

    #[test]
    fn create_delete_wallet_works() {
        let wallet_name = r#"{"id":"create_delete_wallet_works"}"#;
        match Wallet::create(wallet_name, DEFAULT_CREDENTIALS) {
            Ok(..) => assert!(Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).is_ok()),
            Err(e) => match e {
                ErrorCode::WalletAlreadyExistsError => {
                    //This is ok, just delete
                    assert!(Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).is_ok())
                }
                _ => {
                    panic!("{:#?}", e)
                }
            }
        }
    }

    #[test]
    fn open_close_wallet_works() {
        let wallet_name = r#"{"id":"open_wallet_works"}"#;
        let open_closure = || {
            match Wallet::open(wallet_name, DEFAULT_CREDENTIALS) {
                Ok(handle) => {
                    Wallet::close(handle).unwrap();
                    Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).unwrap();
                },
                Err(e) => {
                    Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).unwrap();
                    panic!("{:#?}", e);
                }
            }
        };

        match Wallet::create(wallet_name, DEFAULT_CREDENTIALS) {
            Err(e) => match e {
                ErrorCode::WalletAlreadyExistsError => {
                    open_closure()
                }
                _ => panic!("{:#?}", e)
            }
            _ => open_closure()
        };
    }

    #[test]
    fn export_import_wallet_works() {
        let wallet_name = r#"{"id":"export_import_wallet_works"}"#;

        let open_closure = || {
            match Wallet::open(wallet_name, DEFAULT_CREDENTIALS) {
                Ok(handle) => {
                    Did::new(handle, "{}").unwrap();

                    Wallet::export(handle, &export_config_json(wallet_name)).unwrap();

                    assert!(Path::new(&export_path(wallet_name)).exists());

                    Wallet::close(handle).unwrap();
                    Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).unwrap();
                },
                Err(e) => {
                    Wallet::delete(wallet_name, DEFAULT_CREDENTIALS).unwrap();
                    panic!("{:#?}", e);
                }
            }
        };

        match Wallet::create(wallet_name, DEFAULT_CREDENTIALS) {
            Err(e) => match e {
                ErrorCode::WalletAlreadyExistsError => {
                    open_closure()
                }
                _ => panic!("{:#?}", e)
            }
            _ => open_closure()
        };
    }
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
mod wallet_delete {
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
        Wallet::create(&config, DEFAULT_CREDENTIALS);

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