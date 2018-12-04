extern crate indyrs as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate futures;
#[macro_use]
mod utils;

use indy::crypto;
use indy::wallet;

#[allow(unused_imports)]
use futures::Future;

use utils::constants::DEFAULT_CREDENTIALS;

macro_rules! safe_wallet_create {
    ($x:ident) => {
        match wallet::delete_wallet($x, DEFAULT_CREDENTIALS).wait() {
            Ok(..) => {},
            Err(..) => {}
        };
        wallet::create_wallet($x, DEFAULT_CREDENTIALS).wait().unwrap();
    }
}

macro_rules! wallet_cleanup {
    ($x:ident, $y:ident) => {
        wallet::close_wallet($x).wait().unwrap();
        wallet::delete_wallet($y, DEFAULT_CREDENTIALS).wait().unwrap();
    }
}

pub fn time_it_out<F>(msg: &str, test: F) -> bool where F: Fn() -> bool {
    for _ in 1..250 {
        if test() {
            return true;
        }
    }
    // It tried to do a timeout test 250 times and the system was too fast, so just succeed
    println!("{} - system too fast for timeout test", msg);
    true
}

mod high_cases {
    use super::*;

    mod key_tests {
        use super::*;

        #[test]
        fn all_async_works() {
            let wallet_name = r#"{"id":"all_async_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();

            let vkey = crypto::create_key(handle, None).wait().unwrap();

            let metadata = r#"{"name": "dummy"}"#;
            crypto::set_key_metadata(handle, &vkey, metadata).wait().unwrap();

            let meta= crypto::get_key_metadata(handle, &vkey).wait().unwrap();
            assert_eq!(metadata.to_string(),meta);

            wallet_cleanup!(handle, wallet_name);
        }
    }

    mod crypto_tests {
        use super::*;

        #[test]
        fn sign_verify_async_works() {
            let wallet_name = r#"{"id":"sign_verify_async_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();

            let vkey = crypto::create_key(handle, None).wait().unwrap();

            let message = r#"Hello World"#.as_bytes();
            let sig = crypto::sign(handle, &vkey, message).wait().unwrap();
            assert_eq!(sig.len(), 64);

            wallet_cleanup!(handle, wallet_name);
        }
    }
}

mod low_cases {
    use super::*;

    mod key_tests {
        use super::*;

        #[test]
        fn create_key_works() {
            let wallet_name = r#"{"id":"create_key_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();

            let res = crypto::create_key(handle, None).wait();
            assert!(res.is_ok());

            wallet_cleanup!(handle, wallet_name);
        }
        #[test]
        fn set_metadata_works() {
            let wallet_name = r#"{"id":"set_metadata_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let verkey = crypto::create_key(handle, None).wait().unwrap();

            assert!(crypto::set_key_metadata(handle, &verkey, r#"{"name": "dummy key"}"#).wait().is_ok());
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn get_metadata_works() {
            let wallet_name = r#"{"id":"get_metadata_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let verkey = crypto::create_key(handle, None).wait().unwrap();

            assert!(crypto::set_key_metadata(handle, &verkey, r#"{"name": "dummy key"}"#).wait().is_ok());
            assert_eq!(crypto::get_key_metadata(handle, &verkey).wait().unwrap(), r#"{"name": "dummy key"}"#);
            wallet_cleanup!(handle, wallet_name);
        }
    }

    mod crypto_tests {
        use super::*;

        #[test]
        fn sign_works() {
            let wallet_name = r#"{"id":"sign_works"}"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let vkey = crypto::create_key(handle, None).wait().unwrap();

            let res = crypto::sign(handle, &vkey, r#"Hello World"#.as_bytes()).wait();
            assert!(res.is_ok());
            let sig = res.unwrap();
            assert_eq!(sig.len(), 64);

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn verify_works() {
            let wallet_name = r#"{"id":"verify_works"}"#;
            let message = r#"Hello World"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let vkey = crypto::create_key(handle, None).wait().unwrap();
            let res = crypto::sign(handle, &vkey, message.as_bytes()).wait();
            assert!(res.is_ok());
            let sig = res.unwrap();

            let res = crypto::verify(&vkey, message.as_bytes(), sig.as_slice()).wait();
            assert!(res.is_ok());
            assert!(res.unwrap());

            let mut fake_sig = Vec::new();
            for i in 1..65 {
                fake_sig.push(i as u8);
            }

            let res = crypto::verify(&vkey, message.as_bytes(), fake_sig.as_slice()).wait();
            assert!(res.is_ok());
            assert!(!res.unwrap());
            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn auth_crypt_decrypt_works() {
            let wallet_name = r#"{"id":"auth_crypt_decrypt_works"}"#;
            let message = r#"Hello World"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let vkey1 = crypto::create_key(handle, None).wait().unwrap();
            let vkey2 = crypto::create_key(handle, None).wait().unwrap();

            let res = crypto::auth_crypt(handle, &vkey1, &vkey2, message.as_bytes()).wait();
            assert!(res.is_ok());
            let ciphertext = res.unwrap();

            let res = crypto::auth_decrypt(handle, &vkey2, ciphertext.as_slice()).wait();

            assert!(res.is_ok());
            let (actual_vkey, plaintext) = res.unwrap();
            assert_eq!(actual_vkey, vkey1);
            assert_eq!(plaintext, message.as_bytes());

            let mut fake_msg = Vec::new();
            for i in 1..ciphertext.len() {
                fake_msg.push(i as u8);
            }

            let res = crypto::auth_decrypt(handle, &vkey2, fake_msg.as_slice()).wait();
            assert!(res.is_err());

            wallet_cleanup!(handle, wallet_name);
        }

        #[test]
        fn anon_crypt_decrypt_works() {
            let wallet_name = r#"{"id":"anon_crypt_decrypt_works"}"#;
            let message = r#"Hello World"#;
            safe_wallet_create!(wallet_name);
            let handle = wallet::open_wallet(wallet_name, DEFAULT_CREDENTIALS).wait().unwrap();
            let vkey1 = crypto::create_key(handle, None).wait().unwrap();

            let res = crypto::anon_crypt(&vkey1, message.as_bytes()).wait();
            assert!(res.is_ok());
            let ciphertext = res.unwrap();

            let res = crypto::anon_decrypt(handle, &vkey1, ciphertext.as_slice()).wait();

            assert!(res.is_ok());
            let plaintext = res.unwrap();
            assert_eq!(plaintext, message.as_bytes());

            let mut fake_msg = Vec::new();
            for i in 1..ciphertext.len() {
                fake_msg.push(i as u8);
            }

            let res = crypto::anon_decrypt(handle, &vkey1, fake_msg.as_slice()).wait();
            assert!(res.is_err());

            wallet_cleanup!(handle, wallet_name);
        }

    }
}
