#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indy;
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::did::DidUtils;
use utils::pairwise::PairwiseUtils;
use utils::constants::*;

use indy::api::ErrorCode;


mod high_cases {
    use super::*;

    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_pairwise_works_for_empty_metadata() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_my_did() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            assert_eq!(ErrorCode::WalletItemNotFound, PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, DID, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_their_did() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletItemNotFound, PairwiseUtils::create_pairwise(wallet_handle, DID, &my_did, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_pairwise_works_for_invalid_wallet_handle() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::create_pairwise(wallet_handle + 1, DID_TRUSTEE, &my_did, None).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_create_pairwise_works_for_twice() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let res = PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None);
            assert_eq!(ErrorCode::WalletItemAlreadyExists, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }
    }

    mod list_pairwise {
        use super::*;

        #[test]
        fn indy_list_pairwise_works() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let list_pairwise_json = PairwiseUtils::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 1);
            assert!(list_pairwise.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, my_did, DID_TRUSTEE)));

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_list_pairwise_works_for_empty_result() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let list_pairwise_json = PairwiseUtils::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 0);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_list_pairwise_works_for_invalid_handle() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::list_pairwise(wallet_handle + 1).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }
    }

    mod pairwise_exists {
        use super::*;

        #[test]
        fn indy_is_pairwise_exists_works() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert!(PairwiseUtils::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_not_created() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            assert!(!PairwiseUtils::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_invalid_handle() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::pairwise_exists(wallet_handle + 1, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }
    }

    mod get_pairwise {
        use super::*;

        #[test]
        fn indy_get_pairwise_works() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let pairwise_info_json = PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_json);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_get_pairwise_works_for_not_created_pairwise() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            assert_eq!(ErrorCode::WalletItemNotFound, PairwiseUtils::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_get_pairwise_works_for_invalid_handle() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::get_pairwise(wallet_handle + 1, DID_TRUSTEE).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }
    }

    mod set_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_set_pairwise_metadata_works() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

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

            utils::tear_down();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_reset() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

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

            utils::tear_down();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_not_created_pairwise() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletItemNotFound, PairwiseUtils::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, Some(METADATA)).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_invalid_wallet_handle() {
            utils::setup();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (my_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            DidUtils::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::set_pairwise_metadata(wallet_handle + 1, DID_TRUSTEE, Some(METADATA)).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }
    }
}

