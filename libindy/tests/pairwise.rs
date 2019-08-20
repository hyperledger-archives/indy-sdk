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
use utils::Setup;

use self::indy::ErrorCode;

mod high_cases {
    use super::*;

    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, Some(METADATA)).unwrap();
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_my_did() {
            let setup = Setup::wallet();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            let res = pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, DID, None);
            assert_code!(ErrorCode::WalletItemNotFound,res);
        }

        #[test]
        fn indy_create_pairwise_works_for_not_found_their_did() {
            let setup = Setup::did();

            let res = pairwise::create_pairwise(setup.wallet_handle, DID, &setup.did, None);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod list_pairwise {
        use super::*;

        #[test]
        fn indy_list_pairwise_works() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None).unwrap();

            let list_pairwise_json = pairwise::list_pairwise(setup.wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 1);
            assert!(list_pairwise.contains(&format!(r#"{{"my_did":"{}","their_did":"{}"}}"#, setup.did, DID_TRUSTEE)));
        }

        #[test]
        fn indy_list_pairwise_works_for_empty_result() {
            let setup = Setup::wallet();

            let list_pairwise_json = pairwise::list_pairwise(setup.wallet_handle).unwrap();
            let list_pairwise: Vec<String> = serde_json::from_str(&list_pairwise_json).unwrap();

            assert_eq!(list_pairwise.len(), 0);
        }
    }

    mod pairwise_exists {
        use super::*;

        #[test]
        fn indy_is_pairwise_exists_works() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None).unwrap();

            assert!(pairwise::pairwise_exists(setup.wallet_handle, DID_TRUSTEE).unwrap());
        }

        #[test]
        fn indy_is_pairwise_exists_works_for_not_created() {
            let setup = Setup::wallet();

            assert!(!pairwise::pairwise_exists(setup.wallet_handle, DID_TRUSTEE).unwrap());
        }
    }

    mod get_pairwise {
        use super::*;

        #[test]
        fn indy_get_pairwise_works() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, Some(METADATA)).unwrap();

            let pairwise_info_json = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, setup.did, METADATA), pairwise_info_json);
        }

        #[test]
        fn indy_get_pairwise_works_for_not_created_pairwise() {
            let setup = Setup::wallet();

            let res = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod set_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_set_pairwise_metadata_works() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None).unwrap();

            let pairwise_info_without_metadata = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, setup.did), pairwise_info_without_metadata);

            pairwise::set_pairwise_metadata(setup.wallet_handle, DID_TRUSTEE, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_without_metadata, pairwise_info_with_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, setup.did, METADATA), pairwise_info_with_metadata);
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_not_created_pairwise() {
            let setup = Setup::wallet();

            let res = pairwise::set_pairwise_metadata(setup.wallet_handle, DID_TRUSTEE, Some(METADATA));
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }
}

#[cfg(not(feature = "only_high_cases"))]
mod medium_cases {
    use super::*;
    use api::INVALID_WALLET_HANDLE;


    mod create_pairwise {
        use super::*;

        #[test]
        fn indy_create_pairwise_works_for_empty_metadata() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None).unwrap();
        }

        #[test]
        fn indy_create_pairwise_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = pairwise::create_pairwise(INVALID_WALLET_HANDLE, DID_TRUSTEE, DID, None);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_create_pairwise_works_for_twice() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, Some(METADATA)).unwrap();

            let res = pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);
        }
    }

    mod list_pairwise {
        use super::*;

        #[test]
        fn indy_list_pairwise_works_for_invalid_handle() {
            Setup::empty();

            let res = pairwise::list_pairwise(INVALID_WALLET_HANDLE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod pairwise_exists {
        use super::*;

        #[test]
        fn indy_is_pairwise_exists_works_for_invalid_handle() {
            Setup::empty();

            let res = pairwise::pairwise_exists(INVALID_WALLET_HANDLE, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod get_pairwise {
        use super::*;

        #[test]
        fn indy_get_pairwise_works_for_invalid_handle() {
            Setup::empty();

            let res = pairwise::get_pairwise(INVALID_WALLET_HANDLE, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod set_pairwise_metadata {
        use super::*;

        #[test]
        fn indy_set_pairwise_metadata_works_for_reset() {
            let setup = Setup::did();

            did::store_their_did_from_parts(setup.wallet_handle, DID_TRUSTEE, VERKEY_TRUSTEE).unwrap();

            pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, Some(METADATA)).unwrap();

            let pairwise_info_with_metadata = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(format!(r#"{{"my_did":"{}","metadata":"{}"}}"#, setup.did, METADATA), pairwise_info_with_metadata);

            pairwise::set_pairwise_metadata(setup.wallet_handle, DID_TRUSTEE, None).unwrap();

            let pairwise_info_without_metadata = pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_ne!(pairwise_info_with_metadata, pairwise_info_without_metadata);
            assert_eq!(format!(r#"{{"my_did":"{}"}}"#, setup.did), pairwise_info_without_metadata);
        }

        #[test]
        fn indy_set_pairwise_metadata_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = pairwise::set_pairwise_metadata(INVALID_WALLET_HANDLE, DID_TRUSTEE, Some(METADATA));
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }
}