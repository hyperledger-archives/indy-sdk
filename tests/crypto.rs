extern crate rust_indy_sdk as indy;
use indy::crypto::Key;
use indy::wallet::Wallet;
use indy::ErrorCode;

use std::time::Duration;
use std::sync::mpsc::channel;

mod high_cases {
    use super::*;

    mod key_tests {
        use super::*;

        macro_rules! safe_wallet_create {
            ($x:ident) => {
                match Wallet::delete($x) {
                    Ok(..) => {},
                    Err(..) => {}
                };
                Wallet::create("pool1", $x, None, None, None).unwrap();
            }
        }

        #[test]
        fn create_key_works() {
            let wallet_name = "create_key_works_wallet";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();

            let res = Key::create(handle, None);
            assert!(res.is_ok());

            Wallet::close(handle).unwrap();
            Wallet::delete(wallet_name).unwrap();
        }

        #[test]
        fn create_key_timeout_works() {
            let wallet_name = "create_key_timeout_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();

            let res = Key::create_timeout(handle, None, Duration::from_millis(500));
            assert!(res.is_ok());

            Wallet::close(handle).unwrap();
            Wallet::delete(wallet_name).unwrap();
        }

        #[test]
        fn create_key_async_works() {
            let wallet_name = "create_key_async_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let (sender, receiver) = channel();

            let closure = move |_error, result| {
                sender.send(result).unwrap();
            };

            let res = Key::create_async(handle, None, closure);
            assert_eq!(res, ErrorCode::Success);
            receiver.recv().unwrap();
            Wallet::close(handle).unwrap();
            Wallet::delete(wallet_name).unwrap();
        }
    }

    mod crypto_tests {

    }
}
