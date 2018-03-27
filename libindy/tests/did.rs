extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::did::DidUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::constants::*;
use utils::types::ResponseType;

use indy::api::ErrorCode;

#[cfg(feature = "local_nodes_pool")]
use std::thread;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16, 240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21, 52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5];

mod high_cases {
    use super::*;

    mod key_for_did {
        use super::*;

        #[test]
        fn indy_key_for_did_works_for_my_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = DidUtils::key_for_did(-1, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_their_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = DidUtils::key_for_did(-1, wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_get_key_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let trustee_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(trustee_wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (did, verkey) = DidUtils::create_and_store_my_did(trustee_wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &did, Some(&verkey), None, None).unwrap();
            let nym_resp = LedgerUtils::sign_and_submit_request(pool_handle, trustee_wallet_handle, &trustee_did, &nym_request).unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&did, &did).unwrap();
            LedgerUtils::submit_request_with_retries(pool_handle, &get_nym_request, &nym_resp).unwrap();

            let received_verkey = DidUtils::key_for_did(pool_handle, wallet_handle, &did).unwrap();
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

            let res = DidUtils::key_for_did(pool_handle, wallet_handle, DID);
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

            let res = DidUtils::key_for_did(pool_handle, wallet_handle, DID_TRUSTEE);
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

            let res = DidUtils::key_for_did(pool_handle + 1, wallet_handle, DID_TRUSTEE);
            assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = DidUtils::key_for_did(-1, wallet_handle + 1, &did);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod key_for_local_did {
        use super::*;

        #[test]
        fn indy_key_for_local_did_works_for_my_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = DidUtils::key_for_local_did(wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_local_did_works_for_their_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = DidUtils::key_for_local_did(wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_local_did_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::key_for_local_did(wallet_handle, DID);
            assert_eq!(ErrorCode::WalletNotFoundError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_key_for_local_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = DidUtils::key_for_local_did(wallet_handle + 1, &did);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_set_endpoint_for_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_replace() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());

            let new_endpoint = "10.10.10.1:9710";
            DidUtils::set_endpoint_for_did(wallet_handle, DID, new_endpoint, VERKEY_MY2).unwrap();
            let (updated_endpoint, updated_key) = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, DID).unwrap();
            assert_eq!(new_endpoint, updated_endpoint);
            assert_eq!(VERKEY_MY2, updated_key.unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::set_endpoint_for_did(wallet_handle, INVALID_BASE58_DID, ENDPOINT, VERKEY);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_transport_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_BASE58_VERKEY);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            let res = DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_VERKEY_LENGTH);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::set_endpoint_for_did(wallet_handle + 1, DID, ENDPOINT, VERKEY);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_get_endpoint_for_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let (endpoint, key) = DidUtils::get_endpoint_for_did(wallet_handle, -1, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let attrib_data = format!(r#"{{"endpoint":{{"ha":"{}", "verkey":"{}"}}}}"#, ENDPOINT, VERKEY_TRUSTEE);
            let attrib_request = LedgerUtils::build_attrib_request(&trustee_did, &trustee_did,
                                                                   None, Some(&attrib_data), None).unwrap();

            let attrib_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            let get_attrib_req = LedgerUtils::build_get_attrib_request(&trustee_did, &trustee_did, Some("endpoint"), None, None).unwrap();
            LedgerUtils::submit_request_with_retries(pool_handle, &get_attrib_req, &attrib_req_resp).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));

            let (endpoint, key) = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, &trustee_did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY_TRUSTEE, key.unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger_for_address_only() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let attrib_data = format!(r#"{{"endpoint":{{"ha":"{}"}}}}"#, ENDPOINT);
            let attrib_request = LedgerUtils::build_attrib_request(&trustee_did, &trustee_did,
                                                                   None, Some(&attrib_data), None).unwrap();

            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));

            let (endpoint, key) = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, &trustee_did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(None, key);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, DID);
            assert_eq!(ErrorCode::CommonInvalidState, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_poll_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle + 1, DID);
            assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let res = DidUtils::get_endpoint_for_did(wallet_handle + 1, -1, DID);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_endpoint_for_did_works_incompatible_wallet_and_pool() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool", None).unwrap();

            let res = DidUtils::get_endpoint_for_did(wallet_handle, pool_handle, DID);
            assert_eq!(ErrorCode::WalletIncompatiblePoolError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_did_metadata {
        use super::*;

        #[test]
        fn indy_set_did_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();
            let metadata = DidUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            DidUtils::set_did_metadata(wallet_handle, DID, new_metadata).unwrap();
            let updated_metadata = DidUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_did_metadata(wallet_handle, DID, "").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::set_did_metadata(wallet_handle, INVALID_BASE58_DID, METADATA);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = DidUtils::set_did_metadata(invalid_wallet_handle, DID, METADATA);
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

            DidUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            let metadata = DidUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_did_metadata(wallet_handle, DID, "").unwrap();

            let metadata = DidUtils::get_did_metadata(wallet_handle, DID).unwrap();
            assert_eq!("", metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_no_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::get_did_metadata(wallet_handle, DID);
            assert_eq!(ErrorCode::WalletNotFoundError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_did_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            let res = DidUtils::get_did_metadata(wallet_handle + 1, DID);
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

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(my_did, DID_MY1);
            assert_eq!(my_verkey, VERKEY_MY1);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_as_cid() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();

            assert_eq!(my_did, VERKEY);
            assert_eq!(my_verkey, VERKEY);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            TestUtils::cleanup_storage();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle,
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

            DidUtils::create_my_did(wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::create_my_did(wallet_handle + 1, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_duplicate() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();
            let res = DidUtils::create_my_did(wallet_handle, &format!(r#"{{"did":{:?}}}"#, my_did));
            assert_eq!(res.unwrap_err(), ErrorCode::DidAlreadyExistsError);

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

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_ne!(new_verkey, my_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let res = DidUtils::replace_keys_start(wallet_handle + 1, &my_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_start_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = DidUtils::replace_keys_start(wallet_handle, &my_did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
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

            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let new_verkey = DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_ne!(new_verkey, my_verkey);

            DidUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_without_calling_replace_start() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(DidUtils::replace_keys_apply(wallet_handle, &my_did).unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_eq!(DidUtils::replace_keys_apply(wallet_handle, DID).unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_apply_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_eq!(DidUtils::replace_keys_apply(wallet_handle + 1, &my_did).unwrap_err(), ErrorCode::WalletInvalidHandle);

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
            DidUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, VERKEY);
            DidUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_verkey_with_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = VERKEY.to_owned() + ":ed25519";
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, verkey);
            DidUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::create_my_did(wallet_handle, r#"{"seed":"seed"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, DID);
            let res = DidUtils::store_their_did(wallet_handle + 1, &identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_abbreviated_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"~NcYxiDXkpYi6ov5FcYDi1e"}"#;
            DidUtils::store_their_did(wallet_handle, identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::create_my_did(wallet_handle, r#"{"seed":123}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = DidUtils::store_their_did(wallet_handle, &format!(r#"{{"did":"{:?}"}}"#, INVALID_BASE58_DID));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"did", "verkey":"invalid_base58string"}"#;

            let res = DidUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_store_their_did_works_for_verkey_with_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = VERKEY.to_owned() + ":crypto_type";
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, DID, verkey);
            let res = DidUtils::store_their_did(wallet_handle, &identity_json);
            assert_eq!(ErrorCode::UnknownCryptoTypeError, res.unwrap_err());

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
            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            // 4. Generate my did
            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            // 5. Send Nym request to Ledger
            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            // 6. Start replacing of keys
            let new_verkey = DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            // 7. Send Nym request to Ledger with new verkey
            let nym_request = LedgerUtils::build_nym_request(&my_did, &my_did, Some(&new_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();

            // 8. Send Schema request before apply replacing of keys
            let schema_request = LedgerUtils::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            // 9. Apply replacing of keys
            DidUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

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

            let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            DidUtils::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();
            DidUtils::replace_keys_apply(wallet_handle, &my_did).unwrap();

            let schema_request = LedgerUtils::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            PoolUtils::check_response_type(&response, ResponseType::REQNACK);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod abbreviate_verkey {
        use super::*;

        #[test]
        fn indy_abbreviate_verkey_works_for_abbr_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = DidUtils::create_my_did(wallet_handle, "{}").unwrap();

            let abbr_verkey = DidUtils::abbreviate_verkey(&did, &verkey).unwrap();

            assert_ne!(verkey, abbr_verkey);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_not_abbr_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (did, verkey) = DidUtils::create_my_did(wallet_handle, &format!(r#"{{"did":{:?}}}"#, DID_TRUSTEE)).unwrap();

            let full_verkey = DidUtils::abbreviate_verkey(&did, &verkey).unwrap();

            assert_eq!(verkey, full_verkey);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_invalid_did() {
            let res = DidUtils::abbreviate_verkey(INVALID_BASE58_DID, VERKEY_TRUSTEE);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_invalid_verkey() {
            let res = DidUtils::abbreviate_verkey(DID_TRUSTEE, INVALID_BASE58_VERKEY);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());
        }
    }
}
