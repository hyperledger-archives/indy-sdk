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
use utils::constants::{
    DID_1,
    SEED_1,
    VERKEY_1,
    METADATA,
    VERKEY_ABV_1,
    INVALID_HANDLE
};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

const VALID_TIMEOUT: Duration = Duration::from_secs(5);
const INVALID_TIMEOUT: Duration = Duration::from_micros(1);

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

#[cfg(test)]
mod replace_keys_apply {
    use super::*;

    fn setup() -> (Wallet, String, String) {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        (wallet, did, verkey)
    }

    #[inline]
    fn start_key_replacement(wallet: &Wallet, did: &str) {
        let config = json!({"seed": SEED_1}).to_string();
        Did::replace_keys_start(wallet.handle, did, &config).unwrap();
    }

    #[test]
    fn replace_keys_apply() {
        let (wallet, did, verkey) = setup();
        start_key_replacement(&wallet, &did);

        let result = Did::replace_keys_apply(wallet.handle, &did);

        assert_eq!((), result.unwrap());

        let new_verkey = Did::get_ver_key_local(wallet.handle, &did).unwrap();

        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_apply_without_replace_keys_start() {
        let (wallet, did, _) = setup();

        let result = Did::replace_keys_apply(wallet.handle, &did);

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn replace_keys_apply_invalid_did() {
        let wallet = Wallet::new();

        let result = Did::replace_keys_apply(wallet.handle, DID_1);

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn replace_keys_apply_invalid_wallet() {
        let result = Did::replace_keys_apply(INVALID_HANDLE, DID_1);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn replace_keys_apply_async() {
        let (wallet, did, verkey) = setup();
        let (sender, receiver) = channel();
        start_key_replacement(&wallet, &did);

        Did::replace_keys_apply_async(
            wallet.handle,
            &did,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        let new_verkey = Did::get_ver_key_local(wallet.handle, &did).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_apply_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::replace_keys_apply_async(
            INVALID_HANDLE,
            DID_1,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
    }

    #[test]
    fn replace_keys_apply_timeout() {
        let (wallet, did, verkey) = setup();
        start_key_replacement(&wallet, &did);

        let result = Did::replace_keys_apply_timeout(
            wallet.handle,
            &did,
            VALID_TIMEOUT
        );
        let new_verkey = Did::get_ver_key_local(wallet.handle, &did).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_apply_timeout_invalid_wallet() {
        let result = Did::replace_keys_apply_timeout(
            INVALID_HANDLE,
            DID_1,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn replace_keys_apply_timeout_timeouts() {
        let result = Did::replace_keys_apply_timeout(
            INVALID_HANDLE,
            DID_1,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_store_their_did {
    use super::*;

    #[test]
    fn store_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": VERKEY_1}).to_string();

        let result = Did::store_their_did(wallet.handle, &config);
    
        assert_eq!((), result.unwrap());

        let verkey = Did::get_ver_key_local(wallet.handle, VERKEY_1).unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_with_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        let result = Did::store_their_did(wallet.handle, &config);
    
        assert_eq!((), result.unwrap());

        let verkey = Did::get_ver_key_local(wallet.handle, DID_1).unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_with_crypto_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": format!("{}:ed25519", VERKEY_1)
        }).to_string();

        let result = Did::store_their_did(wallet.handle, &config);

        assert_eq!((), result.unwrap());

        let verkey = Did::get_ver_key_local(wallet.handle, DID_1).unwrap();

        assert_eq!(format!("{}:ed25519", VERKEY_1), verkey);
    }

    #[test]
    fn store_their_did_empty_identify_json() {
        let wallet = Wallet::new();

        let result = Did::store_their_did(wallet.handle, "{}");

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn store_their_did_invalid_handle() {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        let result = Did::store_their_did(INVALID_HANDLE, &config);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn store_their_did_abbreviated_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": "8wZcEriaNLNKtteJvx7f8i",
            "verkey": "~NcYxiDXkpYi6ov5FcYDi1e"
        }).to_string();

        let result = Did::store_their_did(wallet.handle, &config);
        
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn store_their_did_invalid_did() {
        let wallet = Wallet::new();
        let config = json!({"did": "InvalidDid"}).to_string();

        let result = Did::store_their_did(wallet.handle, &config);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn store_their_did_with_invalid_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": "InvalidVerkey"
        }).to_string();

        let result = Did::store_their_did(wallet.handle, &config);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn store_their_did_with_invalid_crypto_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": format!("{}:bad_crypto_type", VERKEY_1)
        }).to_string();

        let result = Did::store_their_did(wallet.handle, &config);

        assert_eq!(ErrorCode::UnknownCryptoTypeError, result.unwrap_err());
    }

    #[test]
    fn store_their_did_duplicate() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        Did::store_their_did(wallet.handle, &config).unwrap();

        let result = Did::store_their_did(wallet.handle, &config);

        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err());
    }

    #[test]
    /*
    This test resulted from the ticket https://jira.hyperledger.org/browse/IS-802
    Previously, an error was being thrown because rollback wasn't happening.
    This test ensures the error is no longer occuring.
    */
    fn store_their_did_multiple_error_fixed() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        Did::store_their_did(wallet.handle, &config).unwrap();

        let result = Did::store_their_did(wallet.handle, &config);
        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err());

        let result = Did::store_their_did(wallet.handle, &config);
        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err());
    }

    #[test]
    fn store_their_did_async_with_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        let (sender, receiver) = channel();

        Did::store_their_did_async(
            wallet.handle,
            &config,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);

        let verkey = Did::get_ver_key_local(wallet.handle, DID_1).unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_async_invalid_wallet() {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        let (sender, receiver) = channel();

        Did::store_their_did_async(
            INVALID_HANDLE,
            &config,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
    }
    
    #[test]
    fn store_their_did_timeout_with_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        let result = Did::store_their_did_timeout(
            wallet.handle,
            &config,
            VALID_TIMEOUT
        );

        assert_eq!((), result.unwrap());

        let verkey = Did::get_ver_key_local(wallet.handle, DID_1).unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_timeout_invalid_wallet() {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        let result = Did::store_their_did_timeout(
            INVALID_HANDLE,
            &config,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn store_their_did_timeout_timeouts() {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        let result = Did::store_their_did_timeout(
            INVALID_HANDLE,
            &config,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err())
    }
}

#[cfg(test)]
mod test_get_verkey_local {
    use super::*;

    #[test]
    fn get_verkey_local_my_did() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let stored_verkey = Did::get_ver_key_local(wallet.handle, &did).unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_local_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();

        let stored_verkey = Did::get_ver_key_local(wallet.handle, DID_1).unwrap();

        assert_eq!(VERKEY_1, stored_verkey);
    }

    #[test]
    fn get_verkey_local_invalid_did() {
        let wallet = Wallet::new();
        let result = Did::get_ver_key_local(wallet.handle, DID_1);

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn get_verkey_local_invalid_wallet() {
        let result = Did::get_ver_key_local(INVALID_HANDLE, DID_1);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_verkey_local_async() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        Did::get_ver_key_local_async(
            wallet.handle,
            &did,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, stored_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(VERKEY_1, stored_verkey);
    }

    #[test]
    fn get_verkey_local_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::get_ver_key_local_async(
            INVALID_HANDLE,
            DID_1,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, stored_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
        assert_eq!(String::from(""), stored_verkey);
    }

    #[test]
    fn get_verkey_local_timeout() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let stored_verkey = Did::get_ver_key_local_timeout(
            wallet.handle,
            &did,
            VALID_TIMEOUT
        ).unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_local_timeout_invalid_wallet() {
        let result = Did::get_ver_key_local_timeout(
            INVALID_HANDLE,
            DID_1,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_verkey_local_timeout_timeouts() {
        let result = Did::get_ver_key_local_timeout(
            INVALID_HANDLE,
            DID_1,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_get_verkey_ledger {
    use super::*;
    use indy::ledger::Ledger;

    #[test]
    fn get_verkey_my_did() {
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let stored_verkey = Did::get_ver_key(
            -1,
            wallet.handle,
            &did
        ).unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();

        let stored_verkey = Did::get_ver_key(
            -1,
            wallet.handle,
            DID_1,
        ).unwrap();

        assert_eq!(VERKEY_1, stored_verkey);
    }

    #[test]
    fn get_verkey_not_on_ledger() {
        let wallet = Wallet::new();
        let wallet2 = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 0,
            num_users: 0,
            num_nodes: 4
        });
        let pool_handle = setup.pool_handle.unwrap();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let result = Did::get_ver_key(
            pool_handle,
            wallet2.handle,
            &did
        );

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn get_verkey_on_ledger() {
        let wallet = Wallet::new();
        let wallet2 = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 1,
            num_users: 1,
            num_nodes: 4
        });
        let pool_handle = setup.pool_handle.unwrap();
        let user = &setup.users.as_ref().unwrap()[0];

        let ledger_verkey = Did::get_ver_key(
            pool_handle,
            wallet2.handle,
            &user.did
        ).unwrap();

        assert_eq!(ledger_verkey, user.verkey);
    }

    #[test]
    fn get_verkey_invalid_pool() {
        let wallet = Wallet::new();

        let result = Did::get_ver_key(-1, wallet.handle, DID_1);

        assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, result.unwrap_err());
    }

    #[test]
    fn get_verkey_invalid_wallet() {
        let result = Did::get_ver_key(-1, INVALID_HANDLE, DID_1);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_verkey_async_my_did() {
        let (sender, receiver) = channel();
        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        Did::get_ver_key_async(
            -1,
            wallet.handle,
            &did,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, stored_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::get_ver_key_async(
            -1,
            INVALID_HANDLE,
            DID_1,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, stored_verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
        assert_eq!(String::from(""), stored_verkey);
    }

    #[test]
    fn get_verkey_timeout_my_did() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let stored_verkey = Did::get_ver_key_timeout(
            -1,
            wallet.handle,
            &did,
            VALID_TIMEOUT
        ).unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_timeout_invalid_wallet() {
        let result = Did::get_ver_key_timeout(
            -1,
            INVALID_HANDLE,
            DID_1,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_verkey_timeout_timeouts() {
        let result = Did::get_ver_key_timeout(
            -1,
            INVALID_HANDLE,
            DID_1,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_set_metadata {
    use super::*;

    #[inline]
    fn setup() -> (Wallet, String) {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        (wallet, did)
    }

    #[test]
    fn set_metadata_my_did() {
        let (wallet, did) = setup();

        let result = Did::set_metadata(wallet.handle, &did, METADATA);
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();

        let result = Did::set_metadata(wallet.handle, DID_1, METADATA);
        let metadata = Did::get_metadata(wallet.handle, DID_1).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_replace_metadata() {
        let (wallet, did) = setup();

        Did::set_metadata(wallet.handle, &did, METADATA).unwrap();
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!(METADATA, metadata);

        let next_metadata = "replacement metadata";
        Did::set_metadata(wallet.handle, &did, next_metadata).unwrap();
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!(next_metadata, metadata);
    }

    #[test]
    fn set_metadata_empty_string() {
        let (wallet, did) = setup();

        let result = Did::set_metadata(wallet.handle, &did, "");
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!("", metadata);
    }

    #[test]
    fn set_metadata_invalid_did() {
        let wallet = Wallet::new();

        let result = Did::set_metadata(wallet.handle, "InvalidDid", METADATA);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn set_metadata_unknown_did() {
        let wallet = Wallet::new();

        let result = Did::set_metadata(wallet.handle, DID_1, METADATA);
        let metadata = Did::get_metadata(wallet.handle, DID_1).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_invalid_wallet() {
        let result = Did::set_metadata(INVALID_HANDLE, DID_1, METADATA);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn set_metadata_async_my_did() {
        let (sender, receiver) = channel();
        let (wallet, did) = setup();

        let result = Did::set_metadata_async(
            wallet.handle,
            &did,
            METADATA,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::set_metadata_async(
            INVALID_HANDLE,
            DID_1,
            METADATA,
            move |ec| sender.send(ec).unwrap()
        );

        let ec = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
    }

    #[test]
    fn set_metadata_timeout_my_did() {
        let (wallet, did) = setup();

        let result = Did::set_metadata_timeout(
            wallet.handle,
            &did,
            METADATA,
            VALID_TIMEOUT
        );
        let metadata = Did::get_metadata(wallet.handle, &did).unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_timeout_invalid_wallet() {
        let result = Did::set_metadata_timeout(
            INVALID_HANDLE,
            DID_1,
            METADATA,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn set_metadata_timeout_timeouts() {
        let result = Did::set_metadata_timeout(
            INVALID_HANDLE,
            DID_1,
            METADATA,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_get_metadata {
    use super::*;

    #[inline]
    fn setup() -> (Wallet, String) {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        
        (wallet, did)
    }

    #[test]
    fn get_metadata_my_did() {
        let (wallet, did) = setup();
        Did::set_metadata(wallet.handle, &did, METADATA).unwrap();

        let result = Did::get_metadata(wallet.handle, &did);

        assert_eq!(METADATA, result.unwrap());
    }

    #[test]
    fn get_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();
        Did::set_metadata(wallet.handle, DID_1, METADATA).unwrap();

        let result = Did::get_metadata(wallet.handle, DID_1);

        assert_eq!(METADATA, result.unwrap());
    }

    #[test]
    fn get_metadata_empty_string() {
        let (wallet, did) = setup();
        Did::set_metadata(wallet.handle, &did, "").unwrap();

        let result = Did::get_metadata(wallet.handle, &did);

        assert_eq!(String::from(""), result.unwrap());
    }

    #[test]
    fn get_metadata_no_metadata_set() {
        let (wallet, did) = setup();

        let result = Did::get_metadata(wallet.handle, &did);

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn get_metadata_unknown_did() {
        let wallet = Wallet::new();

        let result = Did::get_metadata(wallet.handle, DID_1);

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err());
    }

    #[test]
    fn get_metadata_invalid_wallet() {
        let result = Did::get_metadata(INVALID_HANDLE, DID_1);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_metadata_async_my_did() {
        let (sender, receiver) = channel();
        let (wallet, did) = setup();
        Did::set_metadata(wallet.handle, &did, METADATA).unwrap();

        Did::get_metadata_async(
            wallet.handle,
            &did,
            move |ec, metadata| sender.send((ec, metadata)).unwrap()
        );

        let (ec, metadata) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn get_metadata_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::get_metadata_async(
            INVALID_HANDLE,
            DID_1,
            move |ec, metadata| sender.send((ec, metadata)).unwrap()
        );

        let (ec, metadata) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
        assert_eq!("", &metadata);
    }

    #[test]
    fn get_metadata_timeout_my_did() {
        let (wallet, did) = setup();
        Did::set_metadata(wallet.handle, &did, METADATA).unwrap();

        let result = Did::get_metadata_timeout(
            wallet.handle,
            &did,
            VALID_TIMEOUT
        );

        assert_eq!(METADATA, result.unwrap());
    }

    #[test]
    fn get_metadata_timeout_invalid_wallet() {
        let result = Did::get_metadata_timeout(
            INVALID_HANDLE,
            DID_1,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn get_metadata_timeout_timeouts() {
        let result = Did::get_metadata_timeout(
            INVALID_HANDLE,
            DID_1,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_set_endpoint {
    use super::*;

    #[test]
    pub fn set_endpoint_succeeds() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        match indy::did::Did::set_endpoint(wallet.handle, &did, "192.168.1.10", &verkey) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "set_endpoint_works failed {:?}", ec)
            }
        }

    }

    #[test]
    pub fn set_endpoint_timeout_succeeds() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        match indy::did::Did::set_endpoint_timeout(wallet.handle, &did, "192.168.1.10", &verkey, VALID_TIMEOUT) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "set_endpoint_works failed {:?}", ec)
            }
        }
    }

    #[test]
    pub fn set_endpoint_timeout_fails_invalid_timeout() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        match indy::did::Did::set_endpoint_timeout(wallet.handle, &did, "192.168.1.10", &verkey, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "set_endpoint_timeout failed to return error code other than SUCCESS");
            }
            Err(ec) => {
                if ec != indy::ErrorCode::CommonIOError {
                    assert!(false, "set_endpoint_timeout failed error_code = {:?}", ec);
                }
            }
        }
    }

    #[test]
    pub fn set_endpoint_async_succeeds() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        Did::set_endpoint_async(wallet.handle, &did, "192.168.1.10", &verkey, cb);
        let error_code = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(error_code, indy::ErrorCode::Success, "set_endpoint_async_succeeds failed {:?}", error_code);

    }
}

#[cfg(test)]
mod test_get_endpoint {
    use super::*;

    #[test]
    pub fn get_endpoint_succeeds() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        match indy::did::Did::set_endpoint(wallet.handle, &did, end_point_address, &verkey) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "get_endpoint_works failed set_endpoint {:?}", ec)
            }
        }

        let pool_handle = indy::pool::Pool::open_ledger(&pool_setup.pool_name, None).unwrap();
        let mut test_succeeded : bool = false;
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::Did::get_endpoint(wallet.handle, pool_handle, &did) {
            Ok(ret_address) => {

                let (address, _) = Some(ret_address).unwrap();

                if end_point_address.to_string() == address {
                    test_succeeded = true;
                }
            },
            Err(ec) => {
                error_code = ec;
            }
        }

        indy::pool::Pool::close(pool_handle).unwrap();

        if indy::ErrorCode::Success != error_code {
            assert!(false, "get_endpoint_works failed error code {:?}", error_code);
        }

        if false == test_succeeded {
            assert!(false, "get_endpoint_works failed to successfully compare end_point address");
        }
    }

    #[test]
    pub fn get_endpoint_timeout_succeeds() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        match indy::did::Did::set_endpoint(wallet.handle, &did, end_point_address, &verkey) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "get_endpoint_works failed at set endpoint {:?}", ec)
            }
        }

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&pool_setup.pool_name, None).unwrap();
        let mut test_succeeded : bool = false;
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::Did::get_endpoint_timeout(wallet.handle, pool_handle, &did, VALID_TIMEOUT) {
            Ok(ret_address) => {

                let (address, _) = Some(ret_address).unwrap();

                if end_point_address.to_string() == address {
                    test_succeeded = true;
                }
            },
            Err(ec) => {
                error_code = ec;
            }
        }

        indy::pool::Pool::close(pool_handle).unwrap();

        if indy::ErrorCode::Success != error_code {
            assert!(false, "get_endpoint_timeout_succeeds failed error code {:?}", error_code);
        }

        if false == test_succeeded {
            assert!(false, "get_endpoint_timeout_succeeds failed to successfully compare end_point address");
        }
    }

    #[test]
    pub fn get_endpoint_async_success() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        match indy::did::Did::set_endpoint(wallet.handle, &did, end_point_address, &verkey) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "get_endpoint_async failed set_endpoint {:?}", ec)
            }
        }

        let pool_handle = indy::pool::Pool::open_ledger(&pool_setup.pool_name, None).unwrap();
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        let cb = move |ec, end_point, ver_key| {
            sender.send((ec, end_point, ver_key)).unwrap();
        };

        Did::get_endpoint_async(wallet.handle, pool_handle, &did, cb);
        let (error_code, _, _) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
        assert_eq!(error_code, indy::ErrorCode::Success, "get_endpoint_async failed {:?}", error_code);
    }

    /// ----------------------------------------------------------------------------------------
    /// get_endpoint_timeout_fails_invalid_timeout uses an impossibly small time out to trigger error
    /// get_endpoint_timeout should return error code since the timeout triggers
    /// ----------------------------------------------------------------------------------------
    #[test]
    pub fn get_endpoint_timeout_fails_invalid_timeout() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        match indy::did::Did::set_endpoint(wallet.handle, &did, end_point_address, &verkey) {
            Ok(_) => {}
            Err(ec) => {
                assert!(false, "get_endpoint_timeout_fails_invalid_timeout failed at set endpoint {:?}", ec)
            }
        }

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&pool_setup.pool_name, None).unwrap();
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::Did::get_endpoint_timeout(wallet.handle, pool_handle, &did, INVALID_TIMEOUT) {
            Ok(_) => {

            },
            Err(ec) => {
                error_code = ec;
            }
        }

        indy::pool::Pool::close(pool_handle).unwrap();

        assert_eq!(error_code, indy::ErrorCode::CommonIOError);
    }

    /// ----------------------------------------------------------------------------------------
    /// get_endpoint_fails_no_set doesnt call set_endpoint before calling get_endpoint.
    /// get_endpoint should return error code since the endpoint has not been set
    /// ----------------------------------------------------------------------------------------
    #[test]
    pub fn get_endpoint_fails_no_set() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&pool_setup.pool_name, None).unwrap();
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::Did::get_endpoint(wallet.handle, pool_handle, &did) {
            Ok(ret_address) => { },
            Err(ec) => {
                error_code = ec;
            }
        }

        indy::pool::Pool::close(pool_handle).unwrap();

        assert_eq!(error_code, indy::ErrorCode::CommonInvalidState);
    }
}

#[cfg(test)]
mod test_abbreviate_verkey {
    use super::*;

    #[test]
    fn abbreviate_verkey_abbreviated() {
        let result = Did::abbreviate_verkey(DID_1, VERKEY_1);
        assert_eq!(VERKEY_ABV_1, result.unwrap());
    }

    #[test]
    fn abbreviate_verkey_full_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1}).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        let result = Did::abbreviate_verkey(&did, &verkey);

        assert_eq!(verkey, result.unwrap());
    }

    #[test]
    fn abbreviate_verkey_invalid_did() {
        let result = Did::abbreviate_verkey("InvalidDid", VERKEY_1);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn abbreviate_verkey_invalid_verkey() {
        let result = Did::abbreviate_verkey(DID_1, "InvalidVerkey");
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn abbreviate_verkey_async_abbreviated() {
        let (sender, receiver) = channel();
        
        Did::abbreviate_verkey_async(
            DID_1,
            VERKEY_1,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::Success, ec);
        assert_eq!(VERKEY_ABV_1, verkey);
    }

    #[test]
    fn abbreviate_verkey_async_invalid_did() {
        let (sender, receiver) = channel();

        Did::abbreviate_verkey_async(
            "InvalidDid",
            VERKEY_1,
            move |ec, verkey| sender.send((ec, verkey)).unwrap()
        );

        let (ec, verkey) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::CommonInvalidStructure, ec);
        assert_eq!("", verkey);
    }

    #[test]
    fn abbreviate_verkey_timeout_abbreviated() {
        let result = Did::abbreviate_verkey_timeout(
            DID_1,
            VERKEY_1,
            VALID_TIMEOUT
        );

        assert_eq!(VERKEY_ABV_1, result.unwrap());
    }

    #[test]
    fn abbreviate_verkey_timeout_invalid_did() {
        let result = Did::abbreviate_verkey_timeout(
            "InvalidDid",
            VERKEY_1,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn abbreviate_verkey_timeout_timeouts() {
        let result = Did::abbreviate_verkey_timeout(
            DID_1,
            VERKEY_1,
            INVALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_list_with_metadata {
    use super::*;

    fn setup_multiple(wallet: &Wallet) -> Vec<serde_json::Value> {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();
        let (did1, verkey1) = Did::new(wallet.handle, "{}").unwrap();
        let (did2, verkey2) = Did::new(wallet.handle, "{}").unwrap();
        Did::set_metadata(wallet.handle, &did1, METADATA).unwrap();

        let expected = vec![
            json!({
                "did": did1,
                "verkey": verkey1,
                "metadata": Some(METADATA.to_string())
            }),
            json!({
                "did": did2,
                "verkey": verkey2,
                "metadata": null
            })
        ];

        expected
    }

    fn assert_multiple(json: String, expected: Vec<serde_json::Value>) {
        let dids: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(expected.len(), dids.len());

        for did in expected {
            assert!(dids.contains(&did));
        }
    }

    #[test]
    fn list_with_metadata_no_dids() {
        let wallet = Wallet::new();

        let result = Did::list_with_metadata(wallet.handle);

        assert_eq!("[]", result.unwrap());
    }

    #[test]
    fn list_with_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        Did::store_their_did(wallet.handle, &config).unwrap();

        let result = Did::list_with_metadata(wallet.handle);

        assert_eq!("[]", result.unwrap());
    }

    #[test]
    fn list_with_metadata_cryptonym() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1, "cid": true}).to_string();
        Did::new(wallet.handle, &config).unwrap();

        let json = Did::list_with_metadata(wallet.handle).unwrap();
        let dids: serde_json::Value = serde_json::from_str(&json).unwrap();

        let expected = json!([{
            "did": VERKEY_1,
            "verkey": VERKEY_1,
            "metadata": null
        }]);

        assert_eq!(expected, dids);
    }

    #[test]
    fn list_with_metadata_did_with_metadata() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        Did::new(wallet.handle, &config).unwrap();
        Did::set_metadata(wallet.handle, DID_1, METADATA).unwrap();

        let json = Did::list_with_metadata(wallet.handle).unwrap();
        let dids: serde_json::Value = serde_json::from_str(&json).unwrap();

        let expected = json!([{
            "did": DID_1,
            "verkey": VERKEY_1,
            "metadata": METADATA
        }]);

        assert_eq!(expected, dids);
    }

    #[test]
    fn list_with_metadata_multiple_dids() {
        let wallet = Wallet::new();
        let expected = setup_multiple(&wallet);
       
        let dids = Did::list_with_metadata(wallet.handle).unwrap();

        assert_multiple(dids, expected);
    }

    #[test]
    fn list_with_metadata_invalid_wallet() {
        let result = Did::list_with_metadata(INVALID_HANDLE);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn list_with_metadata_async_multiple_dids() {
        let (sender, receiver) = channel();
        let wallet = Wallet::new();
        let expected = setup_multiple(&wallet);

        Did::list_with_metadata_async(
            wallet.handle,
            move |ec, list| sender.send((ec, list)).unwrap()
        );

        let (ec, list) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_multiple(list, expected);
    }

    #[test]
    fn list_with_metadata_async_invalid_wallet() {
        let (sender, receiver) = channel();

        Did::list_with_metadata_async(
            INVALID_HANDLE,
            move |ec, list| sender.send((ec, list)).unwrap()
        );

        let (ec, list) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, ec);
        assert_eq!("", list);
    }

    #[test]
    fn list_with_metadata_timeout_multiple_dids() {
        let wallet = Wallet::new();
        let expected = setup_multiple(&wallet);

        let json = Did::list_with_metadata_timeout(
            wallet.handle,
            VALID_TIMEOUT
        ).unwrap();

        assert_multiple(json, expected);
    }

    #[test]
    fn list_with_metadata_timeout_invalid_wallet() {
        let result = Did::list_with_metadata_timeout(INVALID_HANDLE, VALID_TIMEOUT);
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn list_with_metadata_timeout_timeouts() {
        let result = Did::list_with_metadata_timeout(INVALID_HANDLE, INVALID_TIMEOUT);
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_get_my_metadata {
    use super::*;

    #[test]
    pub fn get_my_metadata_success() {
        let wallet = Wallet::new();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        match Did::get_my_metadata(wallet.handle, &did) {
            Ok(s) => {},
            Err(ec) => {
                assert!(false, "get_my_metadata_success failed with error code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn get_my_metadata_async_success() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let cb = move |ec, data| {
            sender.send((ec, data)).unwrap();
        };

        Did::get_my_metadata_async(wallet.handle, &did, cb);
        let (error_code, meta_data) = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(error_code, indy::ErrorCode::Success, "get_my_metadata_async_success failed error_code {:?}", error_code);
    }

    #[test]
    pub fn get_my_metadata_timeout_success() {
        let wallet = Wallet::new();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        match Did::get_my_metadata_timeout(wallet.handle, &did, VALID_TIMEOUT) {
            Ok(s) => {},
            Err(ec) => {
                assert!(false, "get_my_metadata_timeout_success failed with error code {:?}", ec);
            }
        }
    }

        #[test]
    pub fn get_my_metadata_invalid_timeout_error() {
        let wallet = Wallet::new();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        match Did::get_my_metadata_timeout(wallet.handle, &did, INVALID_TIMEOUT) {
            Ok(s) => {
                assert!(false, "get_my_metadata_invalid_timeout_error failed to timeout");
            },
            Err(ec) => {
                assert_eq!(ec, indy::ErrorCode::CommonIOError, "get_my_metadata_invalid_timeout_error failed with error code {:?}", ec);
            }
        }
    }
}
