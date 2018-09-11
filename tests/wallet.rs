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

mod utils;

use utils::{export_config_json, export_path};
use utils::constants::DEFAULT_CREDENTIALS;
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
