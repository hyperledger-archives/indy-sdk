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
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::{did, pairwise};
use utils::constants::*;

use self::indy::ErrorCode;
use api::INVALID_WALLET_HANDLE;

mod high_cases {
    use super::*;

    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_create_pairwise_works");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works", &wallet_config);
        }

        #[test]
        fn indy_create_pairwise_works_for_empty_metadata() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_create_pairwise_works_for_empty_metadata");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works_for_empty_metadata", &wallet_config);
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_my_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_pairwise_works_for_not_found_my_did");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            let res = pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, DID, None);
            assert_code!(ErrorCode::WalletItemNotFound,res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works_for_not_found_my_did", &wallet_config);
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_their_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_pairwise_works_for_not_found_their_did");

            let (my_did, _) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = pairwise::create_pairwise(wallet_handle, DID, &my_did, None);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works_for_not_found_their_did", &wallet_config);
        }

        #[test]
        fn indy_create_pairwise_works_for_invalid_wallet_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_create_pairwise_works_for_invalid_wallet_handle");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            let res = pairwise::create_pairwise(INVALID_WALLET_HANDLE, DID_TRUSTEE, &my_did, None);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_create_pairwise_works_for_twice() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_create_pairwise_works_for_twice");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let res = pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_pairwise_works_for_twice", &wallet_config);
        }
    }

    mod list_pairwise {
        use super::*;

        #[test]
        fn indy_list_pairwise_works() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_list_pairwise_works");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let list_pairwise_json = pairwise::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 1);
            assert!(list_pairwise.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, my_did, DID_TRUSTEE)));

            utils::tear_down_with_wallet(wallet_handle, "indy_list_pairwise_works", &wallet_config);
        }

        #[test]
        fn indy_list_pairwise_works_for_empty_result() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_list_pairwise_works_for_empty_result");

            let list_pairwise_json = pairwise::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 0);

            utils::tear_down_with_wallet(wallet_handle, "indy_list_pairwise_works_for_empty_result", &wallet_config);
        }

        #[test]
        fn indy_list_pairwise_works_for_invalid_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_list_pairwise_works_for_invalid_handle");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let res = pairwise::list_pairwise(INVALID_WALLET_HANDLE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_list_pairwise_works_for_invalid_handle", &wallet_config);
        }
    }

    mod pairwise_exists {
        use super::*;

        #[test]
        fn indy_is_pairwise_exists_works() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_is_pairwise_exists_works");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            assert!(pairwise::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            utils::tear_down_with_wallet(wallet_handle, "indy_is_pairwise_exists_works", &wallet_config);
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_not_created() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_is_pairwise_exists_works_for_not_created");

            assert!(!pairwise::pairwise_exists(wallet_handle, DID_TRUSTEE).unwrap());

            utils::tear_down_with_wallet(wallet_handle, "indy_is_pairwise_exists_works_for_not_created", &wallet_config);
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_invalid_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_is_pairwise_exists_works_for_invalid_handle");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let res = pairwise::pairwise_exists(INVALID_WALLET_HANDLE, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_is_pairwise_exists_works_for_invalid_handle", &wallet_config);
        }
    }

    mod get_pairwise {
        use super::*;

        #[test]
        fn indy_get_pairwise_works() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_get_pairwise_works");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let pairwise_info_json = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_json);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_pairwise_works", &wallet_config);
        }

        #[test]
        fn indy_get_pairwise_works_for_not_created_pairwise() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_get_pairwise_works_for_not_created_pairwise");

            let res = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_pairwise_works_for_not_created_pairwise", &wallet_config);
        }

        #[test]
        fn indy_get_pairwise_works_for_invalid_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_get_pairwise_works_for_invalid_handle");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let res = pairwise::get_pairwise(INVALID_WALLET_HANDLE, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_pairwise_works_for_invalid_handle", &wallet_config);
        }
    }

    mod set_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_set_pairwise_metadata_works() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_set_pairwise_metadata_works");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let pairwise_info_without_metadata = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, my_did), pairwise_info_without_metadata);

            pairwise::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_without_metadata, pairwise_info_with_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_with_metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_pairwise_metadata_works", &wallet_config);
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_reset() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_set_pairwise_metadata_works_for_reset");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, my_did, METADATA), pairwise_info_with_metadata);

            pairwise::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, None).unwrap();

            let pairwise_info_without_metadata = pairwise::get_pairwise(wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, my_did), pairwise_info_without_metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_pairwise_metadata_works_for_reset", &wallet_config);
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_not_created_pairwise() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_pairwise_metadata_works_for_not_created_pairwise");

            let res = pairwise::set_pairwise_metadata(wallet_handle, DID_TRUSTEE, Some(METADATA));
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_pairwise_metadata_works_for_not_created_pairwise", &wallet_config);
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_invalid_wallet_handle() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_set_pairwise_metadata_works_for_invalid_wallet_handle");

            did::store_their_did_from_parts(wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(wallet_handle, DID_TRUSTEE, &my_did, None).unwrap();

            let res = pairwise::set_pairwise_metadata(INVALID_WALLET_HANDLE, DID_TRUSTEE, Some(METADATA));
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_pairwise_metadata_works_for_invalid_wallet_handle", &wallet_config);
        }
    }
}

