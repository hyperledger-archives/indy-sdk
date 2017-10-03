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
use utils::pairwise::PairwiseUtils;

use indy::api::ErrorCode;

pub const POOL: &'static str = "pool_1";
pub const TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";
pub const MY1_SEED: &'static str = "00000000000000000000000000000My1";


mod high_cases {
    use super::*;


    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_my_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::create_pairwise(wallet_handle, &their_did, "NcYxiDXkpYi6ov5FcYDi1e").unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_their_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::create_pairwise(wallet_handle, "NcYxiDXkpYi6ov5FcYDi1e", &my_did).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_pairwise_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::create_pairwise(invalid_wallet_handle, "NcYxiDXkpYi6ov5FcYDi1e", &my_did).unwrap_err());

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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did.clone(), their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let list_pairwise_json = PairwiseUtils::list_pairwise(wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 1);
            assert!(list_pairwise.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, my_did, their_did)));

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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did.clone(), their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::list_pairwise(invalid_wallet_handle).unwrap_err());

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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            assert!(PairwiseUtils::pairwise_exists(wallet_handle, &their_did).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_not_created() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            assert!(!PairwiseUtils::pairwise_exists(wallet_handle, &their_did).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert!(!PairwiseUtils::pairwise_exists(invalid_wallet_handle, &their_did).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod pairwise_get_my_did {
        use super::*;

        #[test]
        fn indy_pairwise_get_my_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let my_did_json = PairwiseUtils::pairwise_get_my_did(wallet_handle, &their_did).unwrap();
            let my_did_info: MyPublicDid = serde_json::from_str(&my_did_json).unwrap();

            assert_eq!(my_did, my_did_info.did);
            assert_eq!(my_verkey, my_did_info.verkey);
            assert_eq!(my_pk, my_did_info.pk);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_pairwise_get_my_did_works_for_not_created_pairwise() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::pairwise_get_my_did(wallet_handle, &their_did).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_pairwise_get_my_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::pairwise_get_my_did(invalid_wallet_handle, &their_did).unwrap_err());

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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            PairwiseUtils::set_pairwise_metadata(wallet_handle, &their_did, "metadata").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_not_created_pairwise() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::set_pairwise_metadata(wallet_handle, &their_did, "metadata").unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::set_pairwise_metadata(invalid_wallet_handle, &their_did, "metadata").unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_get_pairwise_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let metadata = "some_metadata";
            PairwiseUtils::set_pairwise_metadata(wallet_handle, &their_did, metadata).unwrap();

            assert_eq!(metadata, PairwiseUtils::get_pairwise_metadata(wallet_handle, &their_did).unwrap());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_pairwise_metadata_works_for_empty_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            assert_eq!(ErrorCode::CommonInvalidStructure, PairwiseUtils::get_pairwise_metadata(wallet_handle, &their_did).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_pairwise_metadata_works_for_not_created_pairwise() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            assert_eq!(ErrorCode::WalletNotFoundError, PairwiseUtils::get_pairwise_metadata(wallet_handle, &their_did).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_pairwise_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            PairwiseUtils::create_pairwise(wallet_handle, &their_did, &my_did).unwrap();

            let metadata = "some_metadata";
            PairwiseUtils::set_pairwise_metadata(wallet_handle, &their_did, metadata).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            assert_eq!(ErrorCode::WalletInvalidHandle, PairwiseUtils::get_pairwise_metadata(invalid_wallet_handle, &their_did).unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyPublicDid {
    pub did: String,
    pub crypto_type: String,
    pub pk: String,
    pub verkey: String
}

impl MyPublicDid {
    pub fn new(did: String, crypto_type: String, pk: String, verkey: String) -> MyPublicDid {
        MyPublicDid {
            did: did,
            crypto_type: crypto_type,
            pk: pk,
            verkey: verkey
        }
    }
}

