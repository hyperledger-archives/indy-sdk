extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::crypto::CryptoUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::constants::*;

use indy::api::ErrorCode;

use std::{thread, time};

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11];
pub const INVALID_BASE58_DID: &'static str = "invalid_base58string";

#[test]
fn indy_set_did_metadata_works() {
}

mod high_cases {
    use super::*;

    mod create_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_key_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = CryptoUtils::create_key(invalid_wallet_handle, None);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_key_metadata {
        use super::*;

        #[test]
        fn indy_set_key_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();
            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, new_metadata).unwrap();
            let updated_metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = CryptoUtils::set_key_metadata(invalid_wallet_handle, VERKEY, METADATA);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, "").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::set_key_metadata(wallet_handle, INVALID_BASE58_VERKEY, METADATA);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_key_metadata {
        use super::*;

        #[test]
        fn indy_get_key_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, "").unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!("", metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::get_key_metadata(wallet_handle, VERKEY);
            assert_eq!(ErrorCode::WalletNotFoundError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            let invalid_invalid_handle = wallet_handle + 1;
            let res = CryptoUtils::get_key_metadata(invalid_invalid_handle, VERKEY);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}