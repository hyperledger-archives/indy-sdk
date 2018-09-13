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

    fn wallet_config(storage_type: Option<&str>) -> String {
        let name = rand::random_string(20);

        if let Some(storage) = storage_type {
            json!({
                "id": name,
                "storage_type": storage
            }).to_string()
        } else {
            json!({"id": name}).to_string()
        }
    }

    #[test]
    fn create_default_wallet() {
        let config = wallet_config(Some("default"));
        
        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_default_wallet_custom_path() {
        let dir = TempDir::new(None).unwrap();
        let config = json!({
            "id": rand::random_string(20),
            "storage_type": "default",
            "storage_config": {
                "path": dir.as_ref().to_str()
            }
        }).to_string();

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
        let config = wallet_config(Some("unknown"));

        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!(ErrorCode::WalletUnknownTypeError, result.unwrap_err());
    }

    #[test]
    fn create_wallet_empty_storage_type() {
        let config = wallet_config(None);

        let result = Wallet::create(&config, CREDENTIALS);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, CREDENTIALS).unwrap();
    }

    #[test]
    fn create_wallet_without_key() {
        let config = wallet_config(None);
        let credentials = "{}";

        let result = Wallet::create(&config, credentials);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn create_wallet_without_encryption() {
        let config = wallet_config(None);
        let credentials = json!({"key": ""}).to_string();

        let result = Wallet::create(&config, &credentials);

        assert_eq!((), result.unwrap());

        Wallet::delete(&config, &credentials).unwrap();
    }

    #[test]
    fn create_default_wallet_async() {
        let (sender, receiver) = channel();
        let config = wallet_config(Some("default"));

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
        let config = wallet_config(Some("unknown"));

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
        let config = wallet_config(Some("default"));

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
        let config = wallet_config(Some("unknown"));

        let result = Wallet::create_timeout(
            &config,
            CREDENTIALS,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletUnknownTypeError, result.unwrap_err());
    }

    #[test]
    fn create_wallet_timeout_timeouts() {
        let config = wallet_config(Some("unknown"));

        let result = Wallet::create_timeout(
            &config,
            CREDENTIALS,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}
