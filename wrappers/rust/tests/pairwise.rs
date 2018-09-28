#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
mod utils;

use utils::wallet::Wallet;
use utils::constants::{DID_TRUSTEE, VERKEY_TRUSTEE, METADATA, DID};

use indy::ErrorCode;
use std::time::Duration;
use std::sync::mpsc::channel;

mod create_pairwise {
    use super::*;

    #[test]
    pub fn create_pairwise_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();
    }

    #[test]
    pub fn create_pairwise_timeout_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), Duration::from_secs(5)).unwrap();
    }

    #[test]
    pub fn create_pairwise_async_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
    }

    #[test]
    pub fn create_pairwise_works_for_empty_metadata() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();
    }

    #[test]
    pub fn create_pairwise_timeout_works_for_empty_metadata() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, &did, None, Duration::from_secs(5)).unwrap();
    }

    #[test]
    pub fn create_pairwise_async_works_for_empty_metadata() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, &did, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
    }

    #[test]
    pub fn create_pairwise_works_for_not_found_my_did() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let ec = indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, DID, Some(METADATA)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_timeout_works_for_not_found_my_did() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let ec = indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, DID, Some(METADATA), Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_async_works_for_not_found_my_did() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, DID, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_works_for_not_found_their_did() {
        let wallet = Wallet::new();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        let ec = indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_timeout_works_for_not_found_their_did() {
        let wallet = Wallet::new();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        let ec = indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_async_works_for_not_found_their_did() {
        let wallet = Wallet::new();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn create_pairwise_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        let ec = indy::pairwise::Pairwise::create(wallet.handle + 1, DID_TRUSTEE, &did, Some(METADATA)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn create_pairwise_timeout_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        let ec = indy::pairwise::Pairwise::create_timeout(wallet.handle+1, DID_TRUSTEE, &did, Some(METADATA), Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn create_pairwise_async_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle+1, DID_TRUSTEE, &did, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn create_pairwise_works_for_twice() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let ec = indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemAlreadyExists);
    }

    #[test]
    pub fn create_pairwise_timeout_works_for_twice() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), Duration::from_secs(5)).unwrap();

        let ec = indy::pairwise::Pairwise::create_timeout(wallet.handle, DID_TRUSTEE, &did, None, Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletItemAlreadyExists);
    }

    #[test]
    pub fn create_pairwise_async_works_for_twice() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, &did, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let (sender_twice, receiver_twice) = channel();

        let cb = move |ec| {
            sender_twice.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::create_async(wallet.handle, DID_TRUSTEE, &did, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver_twice.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletItemAlreadyExists);
    }
}

mod list_pairwise {
    use super::*;

    #[test]
    pub fn list_pairwise_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let res = indy::pairwise::Pairwise::list(wallet.handle).unwrap();
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();

        assert_eq!(vec_res.len(), 1);
        assert!(vec_res.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, did, DID_TRUSTEE)));
    }

    #[test]
    pub fn list_pairwise_timeout_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let res = indy::pairwise::Pairwise::list_timeout(wallet.handle, Duration::from_secs(5)).unwrap();
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();

        assert_eq!(vec_res.len(), 1);
        assert!(vec_res.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, did, DID_TRUSTEE)));
    }

    #[test]
    pub fn list_pairwise_async_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::list_async(wallet.handle, cb);
        assert_eq!(ec, ErrorCode::Success);
        let (ec, res) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();
        assert_eq!(vec_res.len(), 1);
        assert!(vec_res.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, did, DID_TRUSTEE)));
    }

    #[test]
    pub fn list_pairwise_works_for_empty_result() {
        let wallet = Wallet::new();

        let res = indy::pairwise::Pairwise::list(wallet.handle).unwrap();
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();

        assert_eq!(vec_res.len(), 0);
    }

    #[test]
    pub fn list_pairwise_timeout_works_for_empty_result() {
        let wallet = Wallet::new();

        let res = indy::pairwise::Pairwise::list_timeout(wallet.handle, Duration::from_secs(5)).unwrap();
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();

        assert_eq!(vec_res.len(), 0);
    }

    #[test]
    pub fn list_pairwise_async_works_for_empty_result() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::list_async(wallet.handle, cb);
        assert_eq!(ec, ErrorCode::Success);
        let (ec, res) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
        let vec_res: Vec<String> = serde_json::from_str(&res).unwrap();
        assert_eq!(vec_res.len(), 0);
    }

    #[test]
    pub fn list_pairwise_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let ec = indy::pairwise::Pairwise::list(wallet.handle + 1).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn list_pairwise_timeout_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let ec = indy::pairwise::Pairwise::list_timeout(wallet.handle + 1, Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn list_pairwise_async_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::list_async(wallet.handle + 1, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, res) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
        assert_eq!("", res);
    }
}

mod pairwise_exists {
    use super::*;

    #[test]
    pub fn pairwise_exists_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        assert!(indy::pairwise::Pairwise::does_exist(wallet.handle, DID_TRUSTEE).unwrap());
    }

    #[test]
    pub fn pairwise_exists_timeout_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        assert!(indy::pairwise::Pairwise::does_exist_timeout(wallet.handle, DID_TRUSTEE, Duration::from_secs(5)).unwrap());
    }

    #[test]
    pub fn pairwise_exists_async_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, exists| {
            sender.send((ec, exists)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::does_exist_async(wallet.handle, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, exists) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
        assert!(exists);
    }

    #[test]
    pub fn pairwise_exists_works_for_not_created() {
        let wallet = Wallet::new();

        assert!(!indy::pairwise::Pairwise::does_exist(wallet.handle, DID_TRUSTEE).unwrap());
    }

    #[test]
    pub fn pairwise_exists_timeout_works_for_not_created() {
        let wallet = Wallet::new();

        assert!(!indy::pairwise::Pairwise::does_exist_timeout(wallet.handle, DID_TRUSTEE, Duration::from_secs(5)).unwrap());
    }

    #[test]
    pub fn pairwise_exists_async_works_for_not_created() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();

        let cb = move |ec, exists| {
            sender.send((ec, exists)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::does_exist_async(wallet.handle, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, exists) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
        assert!(!exists);
    }

    #[test]
    pub fn pairwise_exists_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, indy::pairwise::Pairwise::does_exist(wallet.handle + 1, DID_TRUSTEE).unwrap_err());
    }

    #[test]
    pub fn pairwise_exists_timeout_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        assert_eq!(ErrorCode::WalletInvalidHandle, indy::pairwise::Pairwise::does_exist_timeout(wallet.handle+1, DID_TRUSTEE, Duration::from_secs(5)).unwrap_err());
    }

    #[test]
    pub fn pairwise_exists_async_works_for_invalid_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, exists| {
            sender.send((ec, exists)).unwrap();
        };

        let ec = indy::pairwise::Pairwise::does_exist_async(wallet.handle+1, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, exists) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
        assert_eq!(false, exists);
    }
}

mod get_pairwise {
    use super::*;

    #[test]
    pub fn get_pairwise_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let pairwise_info_json = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_json);
    }

    #[test]
    pub fn get_pairwise_timeout_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let pairwise_info_json = indy::pairwise::Pairwise::get_timeout(wallet.handle, DID_TRUSTEE, Duration::from_secs(5)).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_json);
    }

    #[test]
    pub fn get_pairwise_async_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap()
        };

        let ec = indy::pairwise::Pairwise::get_async(wallet.handle, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pairwise_info_json) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_json);
    }

    #[test]
    pub fn get_pairwise_works_for_not_created_pairwise() {
        let wallet = Wallet::new();

        let ec = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn get_pairwise_timeout_works_for_not_created_pairwise() {
        let wallet = Wallet::new();

        let ec= indy::pairwise::Pairwise::get_timeout(wallet.handle, DID_TRUSTEE, Duration::from_secs(5)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn get_pairwise_async_works_for_not_created_pairwise() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap()
        };

        let ec = indy::pairwise::Pairwise::get_async(wallet.handle, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn get_pairwise_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let ec = indy::pairwise::Pairwise::get(wallet.handle + 1, DID_TRUSTEE).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn get_pairwise_timeout_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let ec = indy::pairwise::Pairwise::get_timeout(wallet.handle + 1, DID_TRUSTEE, Duration::from_secs(5)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn get_pairwise_async_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, res| {
            sender.send((ec, res)).unwrap()
        };

        let ec = indy::pairwise::Pairwise::get_async(wallet.handle + 1, DID_TRUSTEE, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }
}

mod set_pairwise_metadata {
    use super::*;

    #[test]
    pub fn set_pairwise_metadata_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_without_metadata);

        indy::pairwise::Pairwise::set_metadata(wallet.handle, DID_TRUSTEE, Some(METADATA)).unwrap();

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_timeout_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_without_metadata);

        indy::pairwise::Pairwise::set_metadata_timeout(wallet.handle, DID_TRUSTEE, Some(METADATA), Duration::from_secs(5)).unwrap();

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_async_works() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_without_metadata);

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::set_metadata_async(wallet.handle, DID_TRUSTEE, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_works_for_reset() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_without_metadata);

        indy::pairwise::Pairwise::set_metadata(wallet.handle, DID_TRUSTEE, None).unwrap();

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_timeout_works_for_reset() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_without_metadata);

        indy::pairwise::Pairwise::set_metadata_timeout(wallet.handle, DID_TRUSTEE, None, Duration::from_secs(5)).unwrap();

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_async_works_for_reset() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, Some(METADATA)).unwrap();

        let pairwise_info_without_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();
        assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, did, METADATA), pairwise_info_without_metadata);

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::set_metadata_async(wallet.handle, DID_TRUSTEE, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let pairwise_info_with_metadata = indy::pairwise::Pairwise::get(wallet.handle, DID_TRUSTEE).unwrap();

        assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
        assert_eq!(format!(r#"{{"my_did":"{}"}}"#, did), pairwise_info_with_metadata);
    }

    #[test]
    pub fn set_pairwise_metadata_works_for_not_created_pairwise() {
        let wallet = Wallet::new();

        let ec = indy::pairwise::Pairwise::set_metadata(wallet.handle, DID_TRUSTEE, Some(METADATA)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn set_pairwise_metadata_timeout_works_for_not_created_pairwise() {
        let wallet = Wallet::new();

        let ec = indy::pairwise::Pairwise::set_metadata_timeout(wallet.handle, DID_TRUSTEE, Some(METADATA), Duration::from_secs(5)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn set_pairwise_metadata_async_works_for_not_created_pairwise() {
        let wallet = Wallet::new();
        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::set_metadata_async(wallet.handle, DID_TRUSTEE, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletItemNotFound);
    }

    #[test]
    pub fn set_pairwise_metadata_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let ec = indy::pairwise::Pairwise::set_metadata(wallet.handle + 1, DID_TRUSTEE, Some(METADATA)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn set_pairwise_metadata_timeout_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();

        let ec = indy::pairwise::Pairwise::set_metadata_timeout(wallet.handle + 1, DID_TRUSTEE, Some(METADATA), Duration::from_secs(5)).unwrap_err();

        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }

    #[test]
    pub fn set_pairwise_metadata_async_works_for_invalid_wallet_handle() {
        let wallet = Wallet::new();
        let their_identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
        indy::did::Did::store_their_did(wallet.handle, &their_identity_json).unwrap();

        let (did, _) = indy::did::Did::new(wallet.handle, "{}").unwrap();
        indy::pairwise::Pairwise::create(wallet.handle, DID_TRUSTEE, &did, None).unwrap();


        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pairwise::Pairwise::set_metadata_async(wallet.handle + 1, DID_TRUSTEE, Some(METADATA), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::WalletInvalidHandle);
    }
}