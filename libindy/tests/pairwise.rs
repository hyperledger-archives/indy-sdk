extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::did::DidUtils;
use utils::test::TestUtils;
use utils::pairwise::PairwiseUtils;
use utils::constants::*;

use indy::api::ErrorCode;


mod high_cases {
    use super::*;

    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_empty_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_my_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, DID, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_their_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::create_pairwise(wallet_handle, DID, &my_did, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::create_pairwise(wallet_handle + 1, DID_TRUSTEE, &my_did, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod list_pairwise {
        use super::*;

        #[test]
        fn indy_list_pairwise_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let list_pairwise_json = PairwiseUtils::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 1);
            assert!(list_pairwise.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, my_did, DID_TRUSTEE)));

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_list_pairwise_works_for_empty_result() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let list_pairwise_json = PairwiseUtils::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 0);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_list_pairwise_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::list_pairwise(wallet_handle + 1).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod pairwise_exists {
        use super::*;

        #[test]
        fn indy_is_pairwise_exists_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert!(PairwiseUtils::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_not_created() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert!(!PairwiseUtils::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::pairwise_exists(wallet_handle + 1, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_pairwise {
        use super::*;

        #[test]
        fn indy_get_pairwise_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let pairwise_info_json = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_json);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_pairwise_works_for_not_created_pairwise() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_pairwise_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::get_pairwise(wallet_handle + 1, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_set_pairwise_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let pairwise_info_without_metadata = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, my_did), pairwise_info_without_metadata);

            PairwiseUtils::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_without_metadata, pairwise_info_with_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_with_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_reset() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_with_metadata);

            PairwiseUtils::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, None).unwrap();

            let pairwise_info_without_metadata = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, my_did), pairwise_info_without_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_not_created_pairwise() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, Some(METADATA)).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::set_pairwise_metadata(wallet_handle + 1, DID_TRUSTEE, Some(METADATA)).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

