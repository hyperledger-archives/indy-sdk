extern crate rust_indy_sdk as indy;
use indy::crypto::{Key, Crypto};
use indy::wallet::Wallet;
use indy::ErrorCode;

use std::time::Duration;
use std::sync::mpsc::channel;

macro_rules! safe_wallet_create {
    ($x:ident) => {
        match Wallet::delete($x) {
            Ok(..) => {},
            Err(..) => {}
        };
        Wallet::create("pool1", $x, None, None, None).unwrap();
    }
}

macro_rules! wallet_cleanup {
    ($x:ident, $y:ident) => {
        Wallet::close($x).unwrap();
        Wallet::delete($y).unwrap();
    }
}

mod high_cases {
    use super::*;

    mod key_tests {
        use super::*;

        #[test]
        fn create_key_works() {
            let wallet_name = "create_key_works_wallet";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();

            let res = Key::create(handle, None);
            assert!(res.is_ok());

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn create_key_timeout_works() {
            let wallet_name = "create_key_timeout_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();

            let res = Key::create_timeout(handle, None, Duration::from_millis(500));
            assert!(res.is_ok());

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn create_key_async_works() {
            let wallet_name = "create_key_async_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let (sender, receiver) = channel();

            let closure = move |error, result| {
                sender.send((error, result)).unwrap();
            };

            let res = Key::create_async(handle, None, closure);
            assert!(res.is_ok());
            receiver.recv().unwrap();
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn set_metadata_works() {
            let wallet_name = "set_metadata_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let verkey = Key::create(handle, None).unwrap();

            assert!(Key::set_metadata(handle, &verkey, r#"{"name": "dummy key"}"#).is_ok());
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn set_metadata_timeout_works() {
            let wallet_name = "set_metadata_timeout_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let verkey = Key::create(handle, None).unwrap();

            assert!(Key::set_metadata_timeout(handle, &verkey, r#"{"name": "dummy key"}"#, Duration::from_millis(5000)).is_ok());
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn set_metadata_async_works() {
            let wallet_name = "set_metadata_async_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let (sender, receiver) = channel();

            let closure = move |error| {
                sender.send(error).unwrap();
            };
            let verkey = Key::create(handle, None).unwrap();
            let res = Key::set_metadata_async(handle, &verkey, r#"{"name": "dummy key"}"#, closure);
            assert!(res.is_ok());
            receiver.recv().unwrap();
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn get_metadata_works() {
            let wallet_name = "get_metadata_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let verkey = Key::create(handle, None).unwrap();

            assert!(Key::set_metadata(handle, &verkey, r#"{"name": "dummy key"}"#).is_ok());
            assert_eq!(Key::get_metadata(handle, &verkey).unwrap(), r#"{"name": "dummy key"}"#);
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn get_metadata_timeout_works() {
            let wallet_name = "get_metadata_timeout_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let verkey = Key::create(handle, None).unwrap();

            assert!(Key::set_metadata_timeout(handle, &verkey, r#"{"name": "dummy key"}"#, Duration::from_millis(5000)).is_ok());
            assert_eq!(Key::get_metadata_timeout(handle, &verkey, Duration::from_millis(5000)).unwrap(), r#"{"name": "dummy key"}"#);
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn get_metadata_async_works() {
            let wallet_name = "get_metadata_async_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let (sender, receiver) = channel();

            let closure = move |error| {
                sender.send(error).unwrap();
            };
            let verkey = Key::create(handle, None).unwrap();
            let res = Key::set_metadata_async(handle, &verkey, r#"{"name": "dummy key"}"#, closure);
            assert_eq!(res, ErrorCode::Success);
            receiver.recv().unwrap();

            let (sender, receiver) = channel();

            let closure = move |error, result| {
                sender.send((error, result)).unwrap();
            };
            let res = Key::get_metadata_async(handle, &verkey, closure);
            assert!(res.is_ok());
            let (e, r) = receiver.recv().unwrap();
            assert!(e.is_ok());
            assert_eq!(r, r#"{"name": "dummy key"}"#);
            wallet_cleanup!(handle, wallet_name);
        }
    }

    mod crypto_tests {
        use super::*;

        #[test]
        fn sign_works() {
            let wallet_name = "sign_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let vkey = Key::create(handle, None).unwrap();

            let res = Crypto::sign(handle, &vkey, r#"Hello World"#.as_bytes());
            assert!(res.is_ok());
            let sig = res.unwrap();
            assert_eq!(sig.len(), 64);

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn sign_timeout_works() {
            let wallet_name = "sign_timeout_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let vkey = Key::create(handle, None).unwrap();

            let res = Crypto::sign_timeout(handle, &vkey, r#"Hello World"#.as_bytes(), Duration::from_millis(5000));
            assert!(res.is_ok());
            let sig = res.unwrap();
            assert_eq!(sig.len(), 64);

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn sign_async_works() {
            let wallet_name = "sign_async_works";
            safe_wallet_create!(wallet_name);
            let handle = Wallet::open(wallet_name, None, None).unwrap();
            let vkey = Key::create(handle, None).unwrap();

            let (sender, receiver) = channel();
            let res = Crypto::sign_async(handle, &vkey, r#"Hello World"#.as_bytes(), move |err, sig| {
                sender.send((err, sig)).unwrap();
            });
            assert!(res.is_ok());
            let (e, sig) = receiver.recv().unwrap();
            assert!(e.is_ok());
            assert_eq!(sig.len(), 64);

            wallet_cleanup!(handle, wallet_name);
        }
    }
}
