#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
mod utils;

use indy::did::Did;
use indy::ErrorCode;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::b58::{FromBase58, IntoBase58};
use utils::constants::{DID_1, SEED_1, VERKEY_1};
use utils::wallet::Wallet;

const VALID_TIMEOUT: Duration = Duration::from_secs(5);
const INVALID_TIMEOUT: Duration = Duration::from_micros(1);
const INVALID_HANDLE: i32 = 583741;

#[inline]
fn assert_verkey_len(verkey: &str) {
    assert_eq!(32, verkey.from_base58().unwrap().len());
}


#[cfg(test)]
mod create_new_did {
    use super::*;

    #[inline]
    fn assert_did_length(did: &str) {
        assert_eq!(16, did.from_base58().unwrap().len());
    }

    #[test]
    fn create_did_with_empty_json() {
        let wallet = Wallet::new();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        assert_did_length(&did);
        assert_verkey_len(&verkey);
    }

    #[test]
    fn create_did_with_seed() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        assert_eq!(DID_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_cid() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1,
            "cid": true,
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        assert_eq!(VERKEY_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_did() {
        let wallet = Wallet::new();

        let config = json!({
            "did": DID_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        assert_eq!(DID_1, did);
        assert_ne!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_crypto_type() {
        let wallet = Wallet::new();

        let config = json!({
            "crypto_type": "ed25519"
        }).to_string();

        let result = Did::new(wallet.handle, &config);

        assert!(result.is_ok());

    }

    #[test]
    fn create_did_with_invalid_wallet_handle() {
        let result = Did::new(INVALID_HANDLE, "{}");
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn create_wallet_empty_config() {
        let wallet = Wallet::new();
        
        let result = Did::new(wallet.handle, "");

        assert!(result.is_err());
    }

    #[test]
    fn create_did_async_no_config() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();

        Did::new_async(
            wallet.handle,
            "{}",
            move |ec, did, verkey| { sender.send((ec, did, verkey)).unwrap(); }
        );

        let (ec, did, verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        
        assert_eq!(ErrorCode::Success, ec);
        assert_did_length(&did);
        assert_verkey_len(&verkey);
    }
    
    #[test]
    fn create_did_async_with_seed() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let config = json!({
            "seed": SEED_1
        }).to_string();

        Did::new_async(
            wallet.handle,
            &config,
            move |ec, did, key| { sender.send((ec, did, key)).unwrap(); }
        );

        let (ec, did, verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(DID_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::new_async(
            INVALID_HANDLE,
            "{}",
            move |ec, did, key| sender.send((ec, did, key)).unwrap()
        );

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        let expected = (ErrorCode::WalletInvalidHandle, String::new(), String::new());
        assert_eq!(expected, result);
    }

    #[test]
    fn create_did_timeout_no_config() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new_timeout(
            wallet.handle,
            "{}",
            VALID_TIMEOUT
        ).unwrap();

        assert_did_length(&did);
        assert_verkey_len(&verkey);
    }

    #[test]
    fn create_did_timeout_with_seed() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = Did::new_timeout(
            wallet.handle,
            &config,
            VALID_TIMEOUT
        ).unwrap();

        assert_eq!(DID_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_timeout_invalid_wallet() {
        let result = Did::new_timeout(INVALID_HANDLE, "{}", VALID_TIMEOUT);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn create_did_timeout_timeouts() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let result = Did::new_timeout(
            wallet.handle,
            &config,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod replace_keys_start {
    use super::*;

    #[test]
    fn replace_keys_start() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let new_verkey = Did::replace_keys_start(wallet.handle, &did, "{}").unwrap();

        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_invalid_wallet() {
        let wallet = Wallet::new();

        let result = Did::replace_keys_start(INVALID_HANDLE, DID_1, "{}");

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn replace_keys_start_with_seed() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let config = json!({"seed": SEED_1}).to_string();

        let new_verkey = Did::replace_keys_start(wallet.handle, &did, &config).unwrap();

        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_valid_crypto_type() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let config = json!({"crypto_type": "ed25519"}).to_string();

        let new_verkey = Did::replace_keys_start(wallet.handle, &did, &config).unwrap();

        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_invalid_crypto_type() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let config = json!({"crypto_type": "ed25518"}).to_string();

        let result = Did::replace_keys_start(wallet.handle, &did, &config);

        assert_eq!(ErrorCode::UnknownCryptoTypeError, result.unwrap_err());
    }

    #[test]
    fn replace_keys_start_invalid_did() {
        let wallet = Wallet::new();
        let result = Did::replace_keys_start(wallet.handle, DID_1, "{}");

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn replace_keys_start_async() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        Did::replace_keys_start_async(
            wallet.handle,
            &did,
            "{}",
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, new_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::replace_keys_start_async(
            INVALID_HANDLE,
            DID_1,
            "{}",
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, new_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
    }

    #[test]
    fn replace_keys_start_async_with_seed() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let config = json!({"seed": SEED_1}).to_string();

        Did::replace_keys_start_async(
            wallet.handle,
            &did,
            &config,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, new_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_timeout() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let new_verkey = Did::replace_keys_start_timeout(
            wallet.handle,
            &did,
            "{}",
            VALID_TIMEOUT
        ).unwrap();

        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_timeout_with_seed() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let config = json!({"seed": SEED_1}).to_string();

        let new_verkey = Did::replace_keys_start_timeout(
            wallet.handle,
            &did,
            &config,
            VALID_TIMEOUT
        ).unwrap();

        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_timeout_invalid_wallet() {
        let result = Did::replace_keys_start_timeout(
            INVALID_HANDLE,
            DID_1,
            "{}",
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn replace_keys_start_timeout_timeouts() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let result = Did::replace_keys_start_timeout(
            wallet.handle,
            &did,
            "{}",
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}