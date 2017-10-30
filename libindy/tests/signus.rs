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
use utils::signus::SignusUtils;
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

    mod key_for_did {
        use super::*;

        #[test]
        fn indy_key_for_did_works_for_my_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = SignusUtils::key_for_did(pool_handle, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_their_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, VERKEY);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let received_verkey = SignusUtils::key_for_did(pool_handle, wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_get_key_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let trustee_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(trustee_wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did, verkey) = SignusUtils::create_and_store_my_did(trustee_wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &did, Some(&verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet_handle, &trustee_did, &nym_request).unwrap();

            let received_verkey = SignusUtils::key_for_did(pool_handle, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::key_for_did(pool_handle, wallet_handle, DID);
            assert_eq!(ErrorCode::CommonInvalidState, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_incompatible_wallet_and_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other pool", None).unwrap();

            let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = SignusUtils::key_for_did(pool_handle, wallet_handle, &did);
            assert_eq!(ErrorCode::WalletIncompatiblePoolError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::key_for_did(invalid_pool_handle, wallet_handle, &did);
            assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::key_for_did(pool_handle, invalid_wallet_handle, &did);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_set_endpoint_for_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = SignusUtils::get_endpoint_for_did(wallet_handle, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key);

            let new_endpoint = "10.10.10.1:9710";
            SignusUtils::set_endpoint_for_did(wallet_handle, DID, new_endpoint, VERKEY_FOR_MY2_SEED).unwrap();
            let (updated_endpoint, updated_key) = SignusUtils::get_endpoint_for_did(wallet_handle, DID).unwrap();
            assert_eq!(new_endpoint, updated_endpoint);
            assert_eq!(VERKEY_FOR_MY2_SEED, updated_key);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::set_endpoint_for_did(wallet_handle, INVALID_BASE58_DID, ENDPOINT, VERKEY);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_transport_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_BASE58_VERKEY);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            let res = SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_VERKEY_LENGTH);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::set_endpoint_for_did(invalid_wallet_handle, DID, ENDPOINT, VERKEY);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_get_endpoint_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let (endpoint, key) = SignusUtils::get_endpoint_for_did(wallet_handle, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::get_endpoint_for_did(wallet_handle, DID);
            assert_eq!(ErrorCode::WalletNotFoundError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::get_endpoint_for_did(invalid_wallet_handle, DID);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_did_metadata {
        use super::*;

        #[test]
        fn indy_set_did_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();
            let metadata = SignusUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            SignusUtils::set_did_metadata(wallet_handle, DID, new_metadata).unwrap();
            let updated_metadata = SignusUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, "").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::set_did_metadata(wallet_handle, INVALID_BASE58_DID, METADATA);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::set_did_metadata(invalid_wallet_handle, DID, METADATA);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_did_metadata {
        use super::*;

        #[test]
        fn indy_get_did_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            let metadata = SignusUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, "").unwrap();

            let metadata = SignusUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!("", metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_no_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::get_did_metadata(wallet_handle, DID);
            assert_eq!(ErrorCode::WalletNotFoundError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            let invalid_invalid_handle = wallet_handle + 1;
            let res = SignusUtils::get_did_metadata(invalid_invalid_handle, DID);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod create_my_did {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_my_did_works_for_empty_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(my_did, DID_MY1);
            assert_eq!(my_verkey, VERKEY_MY1);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_as_cid() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();

            assert_eq!(my_did, VERKEY);
            assert_eq!(my_verkey, VERKEY);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            TestUtils::cleanup_storage();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle,
                                                                 &format!(r#"{{"did":"{}","seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#, DID)).unwrap();

            assert_eq!(my_did, DID);
            assert_eq!(my_verkey, VERKEY);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_exists_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_my_did(wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::create_my_did(invalid_wallet_handle, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys_start {
        use super::*;

        #[test]
        fn indy_replace_keys_start_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_ne!(new_verkey, my_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::replace_keys_start(invalid_wallet_handle, &my_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = SignusUtils::replace_keys_start(wallet_handle, &my_did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, VERKEY);
            assert_ne!(my_verkey, new_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys_apply {
        use super::*;

        #[test]
        fn indy_replace_keys_apply_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert!(new_verkey != my_verkey);

            SignusUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_without_calling_replace_start() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(SignusUtils::replace_keys_apply(wallet_handle, &my_did).unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_eq!(SignusUtils::replace_keys_apply(wallet_handle, DID).unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(SignusUtils::replace_keys_apply(invalid_wallet_handle, &my_did).unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_did_only() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, DID);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, VERKEY);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_verkey_with_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = VERKEY.to_owned() + ":ed25519";
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"field":"value"}"#;
            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, DID);
            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::store_their_did(invalid_wallet_handle, &identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_abbreviated_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"~NcYxiDXkpYi6ov5FcYDi1e"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_without_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"verkey":"{}"}}"#, VERKEY);
            let res = SignusUtils::store_their_did(wallet_handle, &identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn indy_sign_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = SignusUtils::sign(wallet_handle, &my_did, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_unknow_signer() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::sign(wallet_handle, DID, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::sign(invalid_wallet_handle, &my_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod verify {
        use super::*;

        #[test]
        fn indy_verify_works_for_verkey_cached_in_wallet() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);

            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &did, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_verkey_with_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let verkey = verkey + ":ed25519";
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &did, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_verkey_with_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let verkey = verkey + ":crypto_type";
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::verify(wallet_handle, pool_handle, &did, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(ErrorCode::SignusUnknownCryptoError, res.unwrap_err());

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_expired_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, Some(r#"{"freshness_time":1}"#)).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, my_did, my_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            thread::sleep(time::Duration::from_secs(2));

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::verify(invalid_wallet_handle, pool_handle, DID, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::verify(wallet_handle, invalid_pool_handle, DID, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_other_signer() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_my_did(wallet_handle, &format!(r#"{{"seed":"{}"}}"#, TRUSTEE_SEED)).unwrap();
            let (other_did, other_verkey) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"000000000000000000000000Steward1"}"#).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, other_did, other_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let signature = SignusUtils::sign(wallet_handle, &did, MESSAGE.as_bytes()).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &other_did,
                                            MESSAGE.as_bytes(), &signature).unwrap();
            assert!(!valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod encrypt {
        use super::*;

        #[test]
        fn indy_encrypt_works_for_pk_cached_in_wallet() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_get_nym_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &their_did, Some(&their_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            SignusUtils::encrypt(wallet_handle, pool_handle, &trustee_did, &their_did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_expired_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            WalletUtils::create_wallet(POOL, WALLET, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, Some(r#"{"freshness_time":1}"#)).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &their_did, Some(&their_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            thread::sleep(time::Duration::from_secs(2));

            SignusUtils::encrypt(wallet_handle, pool_handle, &trustee_did, &their_did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::encrypt(invalid_wallet_handle, pool_handle, &my_did, &their_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::encrypt(wallet_handle, invalid_pool_handle, &my_did, &their_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod decrypt {
        use super::*;

        #[test]
        fn indy_decrypt_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let decrypted_message = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE).unwrap();

            assert_eq!(MESSAGE.as_bytes().to_vec(), decrypted_message);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_other_coder() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, my_did, my_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let (encrypted_message, nonce) = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &my_did, MESSAGE.as_bytes()).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, &encrypted_message, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_nonce_not_correspond_message() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let nonce = "acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7";
            let res = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, nonce.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::decrypt(invalid_wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod encrypt_sealed {
        use super::*;

        #[test]
        fn indy_encrypt_sealed_works_for_pk_cached_in_wallet() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_sealed_works_for_get_nym_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &did.clone(), Some(&verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_sealed_works_for_expired_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            WalletUtils::create_wallet(POOL, "wallet1", None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet("wallet1", Some(r#"{"freshness_time":1}"#)).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &did.clone(), Some(&verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            thread::sleep(time::Duration::from_secs(2));

            SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_sealed_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::encrypt_sealed(invalid_wallet_handle, pool_handle, &did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_sealed_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::encrypt_sealed(wallet_handle, invalid_pool_handle, &did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod decrypt_sealed {
        use super::*;

        #[test]
        fn indy_decrypt_sealed_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let encrypted_message = SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();
            let decrypted_message = SignusUtils::decrypt_sealed(wallet_handle, &did, &encrypted_message).unwrap();

            assert_eq!(MESSAGE.as_bytes().to_vec(), decrypted_message);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_sealed_works_for_other_coder() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (did2, verkey2) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did2, verkey2);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let encrypted_message = SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();

            let res = SignusUtils::decrypt_sealed(wallet_handle, &did2, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_sealed_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let encrypted_message = SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes()).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::decrypt_sealed(invalid_wallet_handle, &did, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod create_my_did {
        use super::*;

        #[test]
        fn indy_create_my_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"crypto_type":"type"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"seed"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, &format!(r#"{{"did":"{:?}"}}"#, INVALID_BASE58_DID));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"seed":123}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys_start {
        use super::*;

        #[test]
        fn indy_replace_keys_start_works_for_not_exists_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::replace_keys_start(wallet_handle, DID, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = SignusUtils::replace_keys_start(wallet_handle, &my_did, r#"{"crypto_type":"ed25519"}"#).unwrap();
            assert!(my_verkey != new_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let res = SignusUtils::replace_keys_start(wallet_handle, &my_did, r#"{"crypto_type":"type"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::store_their_did(wallet_handle, &format!(r#"{{"did":"{:?}"}}"#, INVALID_BASE58_DID));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] /* FIXME */
        fn indy_store_their_did_works_for_invalid_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"did", "verkey":"invalid_base58string"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod verify {
        use super::*;

        #[test]
        fn indy_verify_works_for_invalid_signature_len() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);

            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let signature: Vec<u8> = vec![20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190];

            let res = SignusUtils::verify(wallet_handle, pool_handle, &did, MESSAGE.as_bytes(), &signature);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_nym_from_ledger_with_incompatible_wallet() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool_name", None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = SignusUtils::verify(wallet_handle, pool_handle, &my_did, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_ledger_not_found_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"0000000000000000000000000000Fake"}"#).unwrap();

            let res = SignusUtils::verify(wallet_handle, pool_handle, &my_did, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState); //TODO maybe we need add LedgerNotFound error

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_nym_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod encrypt {
        use super::*;

        #[test]
        fn indy_encrypt_works_for_unknow_my_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, DID, &their_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_get_nym_from_ledger_with_incompatible_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool", None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, their_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_not_found_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod decrypt {
        use super::*;

        #[test]
        fn indy_decrypt_works_for_unknown_my_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, pool_handle, DID, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_unknown_coder_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY2_SEED)).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_saved_coder_nym_without_pk() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, their_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_invalid_nonce_len() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let nonce = vec![24, 99, 107, 70, 58, 6, 252, 149, 225];

            let res = SignusUtils::decrypt(wallet_handle, pool_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod encrypt_sealed {
        use super::*;

        #[test]
        fn indy_encrypt_sealed_works_for_get_nym_from_ledger_with_incompatible_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool", None).unwrap();

            let (did, verkey) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_sealed_works_for_not_found_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let res = SignusUtils::encrypt_sealed(wallet_handle, pool_handle, &did, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod decrypt_sealed {
        use super::*;

        #[test]
        fn indy_decrypt_sealed_works_for_unknown_coder_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::decrypt_sealed(wallet_handle, DID, ENCRYPTED_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_sealed_works_for_saved_coder_nym_without_pk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::decrypt_sealed(wallet_handle, &did, ENCRYPTED_MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_demo() {
            TestUtils::cleanup_storage();

            // 1. Create and open pool
            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            // 2. Create and open wallet
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            // 3. Generate did from Trustee seed
            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            // 4. Generate my did
            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            // 5. Send Nym request to Ledger
            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            // 6. Start replacing of keys
            let new_verkey = SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            // 7. Send Nym request to Ledger with new verkey
            let nym_request = LedgerUtils::build_nym_request(&my_did, &my_did, Some(&new_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();

            // 8. Send Schema request before apply replacing of keys
            let schema_request = LedgerUtils::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            // 9. Apply replacing of keys
            SignusUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

            // 10. Send Schema request
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_without_nym_transaction() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            SignusUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();
            SignusUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
