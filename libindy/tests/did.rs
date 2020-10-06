#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as indy;
extern crate indyrs as api;

use crate::utils::{did, pool, ledger};
use crate::utils::constants::*;
use crate::utils::types::ResponseType;
use crate::utils::Setup;

use self::indy::ErrorCode;

use crate::api::{INVALID_WALLET_HANDLE, INVALID_POOL_HANDLE};

#[cfg(feature = "local_nodes_pool")]
use std::thread;
use std::collections::HashMap;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16, 240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21, 52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5];

mod high_cases {
    use super::*;

    mod key_for_did {
        use super::*;

        #[test]
        fn indy_key_for_did_works_for_my_did() {
            let setup = Setup::wallet();

            let (did, verkey) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = did::key_for_did(-1, setup.wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);
        }

        #[test]
        fn indy_key_for_did_works_for_their_did() {
            let setup = Setup::wallet();

            did::store_their_did_from_parts(setup.wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = did::key_for_did(-1, setup.wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);
        }

        #[test]
        fn indy_key_for_did_works_for_get_key_from_ledger() {
            let setup = Setup::wallet_and_pool();

            let received_verkey = did::key_for_did(setup.pool_handle, setup.wallet_handle, DID_TRUSTEE).unwrap();
            assert_eq!(VERKEY_TRUSTEE.to_string(), received_verkey);
        }

        #[test]
        fn indy_key_for_did_works_for_unknown_did() {
            let setup = Setup::wallet_and_pool();

            let res = did::key_for_did(setup.pool_handle, setup.wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_key_for_did_works_for_fully_qualified_my_did() {
            let setup = Setup::wallet();

            let (did, verkey) = did::create_and_store_my_did_v1(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = did::key_for_did(-1, setup.wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);
        }

        #[test]
        fn indy_key_for_did_works_for_fully_qualified_their_did() {
            let setup = Setup::wallet();

            did::store_their_did_from_parts(setup.wallet_handle, DID_V1, VERKEY).unwrap();

            let received_verkey = did::key_for_did(-1, setup.wallet_handle, DID_V1).unwrap();
            assert_eq!(VERKEY, received_verkey);
        }
    }

    mod key_for_local_did {
        use super::*;

        #[test]
        fn indy_key_for_local_did_works_for_my_did() {
            let setup = Setup::did();

            let received_verkey = did::key_for_local_did(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(setup.verkey, received_verkey);
        }

        #[test]
        fn indy_key_for_local_did_works_for_their_did() {
            let setup = Setup::wallet();

            did::store_their_did_from_parts(setup.wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = did::key_for_local_did(setup.wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);
        }

        #[test]
        fn indy_key_for_local_did_works_for_unknown_did() {
            let setup = Setup::wallet();

            let res = did::key_for_local_did(setup.wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_key_for_local_did_works_for_fully_qualified_my_did() {
            let setup = Setup::did_fully_qualified();

            let received_verkey = did::key_for_local_did(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(setup.verkey, received_verkey);
        }
    }

    mod set_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_set_endpoint_for_did_works() {
            let setup = Setup::wallet();
            did::set_endpoint_for_did(setup.wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_fully_qualified_did() {
            let setup = Setup::wallet();
            did::set_endpoint_for_did(setup.wallet_handle, DID_V1, ENDPOINT, VERKEY).unwrap();
        }
    }

    mod get_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_get_endpoint_for_did_works() {
            let setup = Setup::wallet();

            did::set_endpoint_for_did(setup.wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = did::get_endpoint_for_did(setup.wallet_handle, -1, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());
        }

        #[test]
        fn indy_get_endpoint_for_did_works_for_fully_qualified_did() {
            let setup = Setup::wallet();

            did::set_endpoint_for_did(setup.wallet_handle, DID_V1, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = did::get_endpoint_for_did(setup.wallet_handle, -1, DID_V1).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger() {
            let setup = Setup::new_identity();

            let attrib_data = json!({"endpoint": {"ha": ENDPOINT, "verkey": VERKEY_TRUSTEE}}).to_string();
            let attrib_request = ledger::build_attrib_request(&setup.did, &setup.did, None, Some(&attrib_data), None).unwrap();
            ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &setup.did, &attrib_request).unwrap();

            thread::sleep(std::time::Duration::from_secs(1));

            let (endpoint, key) = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, &setup.did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY_TRUSTEE, key.unwrap());
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger_for_address_only() {
            let setup = Setup::new_identity();

            let attrib_data = json!({"endpoint": {"ha": ENDPOINT}}).to_string();
            let attrib_request = ledger::build_attrib_request(&setup.did, &setup.did, None, Some(&attrib_data), None).unwrap();
            ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &setup.did, &attrib_request).unwrap();

            thread::sleep(std::time::Duration::from_secs(1));

            let (endpoint, key) = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, &setup.did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(None, key);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_for_unknown_did() {
            let setup = Setup::wallet_and_pool();

            let res = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, DID);
            assert_code!(ErrorCode::CommonInvalidState, res);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_poll_handle() {
            let setup = Setup::wallet();

            let res = did::get_endpoint_for_did(setup.wallet_handle, INVALID_POOL_HANDLE, DID);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_wallet_handle() {
            Setup::empty();

            let res = did::get_endpoint_for_did(INVALID_WALLET_HANDLE, -1, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod set_did_metadata {
        use super::*;

        #[test]
        fn indy_set_did_metadata_works() {
            let setup = Setup::did();
            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
        }

        #[test]
        fn indy_set_did_metadata_works_for_fully_qualified_did() {
            let setup = Setup::did_fully_qualified();
            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
        }

        #[test]
        fn indy_set_did_metadata_works_for_their_did() {
            let setup = Setup::wallet();
            did::store_their_did_from_parts(setup.wallet_handle, DID, VERKEY).unwrap();
            did::set_did_metadata(setup.wallet_handle, DID, METADATA).unwrap();
        }

        #[test]
        fn indy_set_did_metadata_works_for_replace() {
            let setup = Setup::did();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
            let metadata = did::get_did_metadata(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            did::set_did_metadata(setup.wallet_handle, &setup.did, new_metadata).unwrap();
            let updated_metadata = did::get_did_metadata(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(new_metadata, updated_metadata);
        }

        #[test]
        fn indy_set_did_metadata_works_for_empty_string() {
            let setup = Setup::did();
            did::set_did_metadata(setup.wallet_handle, &setup.did, "").unwrap();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_did() {
            let setup = Setup::wallet();

            let res = did::set_did_metadata(setup.wallet_handle, INVALID_BASE58_DID, METADATA);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_set_did_metadata_works_for_unknown_did() {
            let setup = Setup::wallet();
            did::set_did_metadata(setup.wallet_handle, &DID, METADATA).unwrap();
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_handle() {
            Setup::empty();

            let res = did::set_did_metadata(INVALID_WALLET_HANDLE, DID_TRUSTEE, METADATA);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod get_did_metadata {
        use super::*;

        #[test]
        fn indy_get_did_metadata_works() {
            let setup = Setup::did();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();

            let metadata = did::get_did_metadata(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);
        }

        #[test]
        fn indy_get_did_metadata_works_for_fully_qualified_did() {
            let setup = Setup::did_fully_qualified();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();

            let metadata = did::get_did_metadata(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);
        }

        #[test]
        fn indy_get_did_metadata_works_for_their_did() {
            let setup = Setup::wallet();

            did::store_their_did_from_parts(setup.wallet_handle, DID, VERKEY).unwrap();

            did::set_did_metadata(setup.wallet_handle, DID, METADATA).unwrap();

            let metadata = did::get_did_metadata(setup.wallet_handle, DID).unwrap();
            assert_eq!(METADATA.to_string(), metadata);
        }

        #[test]
        fn indy_get_did_metadata_works_for_no_metadata() {
            let setup = Setup::did();

            let res = did::get_did_metadata(setup.wallet_handle, &setup.did);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_get_did_metadata_works_for_unknown_did() {
            let setup = Setup::wallet();

            let res = did::get_did_metadata(setup.wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod get_my_did_metadata {
        use super::*;

        #[test]
        fn indy_get_my_did_metadata_works() {
            let setup = Setup::did();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
            did::get_my_did_with_metadata(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_get_my_did_metadata_works_for_fullq_qualified_did() {
            let setup = Setup::did_fully_qualified();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
            did::get_my_did_with_metadata(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_get_my_did_metadata_works_for_no_metadata() {
            let setup = Setup::did();
            did::get_my_did_with_metadata(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_get_my_did_metadata_works_with_temp_verkey() {
            let setup = Setup::did();

            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();
            did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();
            did::get_my_did_with_metadata(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_get_my_did_metadata_works_for_unknown_did() {
            let setup = Setup::wallet();

            let res = did::get_my_did_with_metadata(setup.wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod create_my_did {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_my_did_works_for_empty_json() {
            let setup = Setup::wallet();

            let (my_did, my_verkey) = did::create_my_did(setup.wallet_handle, "{}").unwrap();
            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);
        }

        #[test]
        fn indy_create_my_did_works_for_fully_qualified() {
            let setup = Setup::wallet();

            let my_did_json = json!({"method_name": DEFAULT_METHOD_NAME}).to_string();
            let (my_did, my_verkey) = did::create_my_did(setup.wallet_handle, &my_did_json).unwrap();

            assert!(my_did.starts_with(DEFAULT_PREFIX));
            assert_eq!(my_did.replace(DEFAULT_PREFIX, "").from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);
        }

        #[test]
        fn indy_create_my_did_works_for_several_dids_but_different_methods() {
            let setup = Setup::wallet();

            let (my_did_1, my_verkey_1) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let (my_did_2, my_verkey_2) = did::create_and_store_my_did_v1(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let my_did_json = json!({"method_name": "indy", "seed": MY1_SEED}).to_string();
            let (my_did_3, my_verkey_3) = did::create_my_did(setup.wallet_handle, &my_did_json).unwrap();

            assert_eq!(my_did_1.from_base58().unwrap().len(), 16);
            assert!(my_did_2.starts_with(DEFAULT_PREFIX));
            assert!(my_did_3.starts_with("did:indy:"));

            assert_eq!(my_verkey_1, my_verkey_2);
            assert_eq!(my_verkey_2, my_verkey_3);

            assert_eq!(my_verkey_1, did::key_for_local_did(setup.wallet_handle, &my_did_1).unwrap());
            assert_eq!(my_verkey_2, did::key_for_local_did(setup.wallet_handle, &my_did_2).unwrap());
            assert_eq!(my_verkey_3, did::key_for_local_did(setup.wallet_handle, &my_did_3).unwrap());
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            let setup = Setup::wallet();

            let (my_did, my_verkey) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(my_did, DID_MY1);
            assert_eq!(my_verkey, VERKEY_MY1);
        }

        #[test]
        fn indy_create_my_did_works_with_hex_seed() {
            let setup = Setup::wallet();

            let (my_did, my_verkey) = did::create_and_store_my_did(setup.wallet_handle, Some("94a823a6387cdd30d8f7687d95710ebab84c6e277b724790a5b221440beb7df6")).unwrap();
            assert_eq!(my_did, "HWvjYf77k1dqQAk6sE4gaS");
            assert_eq!(my_verkey, "A16wi1xHBu5KT4SqNhZXrKZfoQbXJCbDozgSTJhUgu9x");
        }

        #[test]
        fn indy_create_my_did_works_for_duplicate() {
            let setup = Setup::wallet();

            let (did, verkey) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let (dup_did, dup_verkey) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(did, dup_did);
            assert_eq!(verkey, dup_verkey);

            let res = did::create_my_did(setup.wallet_handle, &json!({"did": did}).to_string());
            assert_code!(ErrorCode::DidAlreadyExistsError, res);
        }
    }

    mod replace_keys_start {
        use super::*;

        #[test]
        fn indy_replace_keys_start_works() {
            let setup = Setup::did();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();
            assert_ne!(new_verkey, setup.verkey);
        }

        #[test]
        fn indy_replace_keys_start_works_for_fully_qualified() {
            let setup = Setup::did_fully_qualified();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();
            assert_ne!(new_verkey, setup.verkey);
        }

        #[test]
        fn indy_replace_keys_start_works_for_seed() {
            let setup = Setup::did();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, VERKEY);
            assert_ne!(setup.verkey, new_verkey);
        }
    }

    mod replace_keys_apply {
        use super::*;

        #[test]
        fn indy_replace_keys_apply_works() {
            let setup = Setup::did();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();

            assert_ne!(new_verkey, setup.verkey);

            did::replace_keys_apply(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_replace_keys_apply_works_for_fully_qualified() {
            let setup = Setup::did_fully_qualified();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();

            assert_ne!(new_verkey, setup.verkey);

            did::replace_keys_apply(setup.wallet_handle, &setup.did).unwrap();
        }

        #[test]
        fn indy_replace_keys_apply_works_without_calling_replace_start() {
            let setup = Setup::did();

            let res = did::replace_keys_apply(setup.wallet_handle, &setup.did);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_replace_keys_apply_works_for_unknown_did() {
            let setup = Setup::wallet();

            let res = did::replace_keys_apply(setup.wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_replace_keys_works_for_two_dids_have_same_verkey() {
            let setup = Setup::wallet();

            let (my_did_1, my_verkey_1) = did::create_and_store_my_did(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let (my_did_2, my_verkey_2) = did::create_and_store_my_did_v1(setup.wallet_handle, Some(MY1_SEED)).unwrap();

            let _ = did::replace_keys_start(setup.wallet_handle, &my_did_1, "{}").unwrap();
            did::replace_keys_apply(setup.wallet_handle, &my_did_1).unwrap();

            assert_ne!(my_verkey_1, did::key_for_local_did(setup.wallet_handle, &my_did_1).unwrap());
            assert_eq!(my_verkey_2, did::key_for_local_did(setup.wallet_handle, &my_did_2).unwrap());
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_did_only() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();
        }

        #[test]
        fn indy_store_their_did_works_for_fully_qualified_did_only() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID_V1}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();
        }

        #[test]
        fn indy_store_their_did_works_for_verkey() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID, "verkey": VERKEY}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();
        }

        #[test]
        fn indy_store_their_did_works_twice() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID, "verkey": VERKEY}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();

            let identity_json = json!({"did": DID, "verkey": VERKEY_TRUSTEE}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();

            let verkey = did::key_for_local_did(setup.wallet_handle, DID).unwrap();
            assert_eq!(VERKEY_TRUSTEE, verkey);
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_demo() {
            // 1. Create and open pool
            // 2. Create and open wallet
            // 3. Generate did from Trustee seed
            // 4. Generate my did
            // 5. Send Nym request to Ledger
            let setup = Setup::new_identity();

            // 6. Start replacing of keys
            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();

            // 7. Send Nym request to Ledger with new verkey
            let nym_request = ledger::build_nym_request(&setup.did, &setup.did, Some(&new_verkey), None, None).unwrap();
            ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &setup.did, &nym_request).unwrap();

            // 8. Send Schema request before apply replacing of keys
            let schema_request = ledger::build_schema_request(&setup.did, SCHEMA_DATA).unwrap();
            let response = ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &setup.did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            // 9. Apply replacing of keys
            did::replace_keys_apply(setup.wallet_handle, &setup.did).unwrap();

            // 10. Send Schema request
            ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &setup.did, &schema_request).unwrap();
        }
    }

    mod abbreviate_verkey {
        use super::*;

        #[test]
        fn indy_abbreviate_verkey_works_for_abbr_key() {
            let setup = Setup::did();

            let abbr_verkey = did::abbreviate_verkey(&setup.did, &setup.verkey).unwrap();
            assert_ne!(setup.verkey, abbr_verkey);
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_abbr_key_for_fully_qualified_did() {
            let setup = Setup::did_fully_qualified();

            let abbr_verkey = did::abbreviate_verkey(&setup.did, &setup.verkey).unwrap();
            assert_ne!(setup.verkey, abbr_verkey);
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_not_abbr_key() {
            let setup = Setup::wallet();

            let (did, verkey) = did::create_my_did(setup.wallet_handle, &format!(r#"{{"did":{:?}}}"#, DID_TRUSTEE)).unwrap();

            let full_verkey = did::abbreviate_verkey(&did, &verkey).unwrap();

            assert_eq!(verkey, full_verkey);
        }
    }

    mod qualify_did {
        use super::*;

        const CUSTOM_METHOD: &str = "peer";

        #[test]
        fn qualify_did_for_appending_prefix() {
            let setup = Setup::new_identity();

            let full_qualified_did = did::qualify_did(setup.wallet_handle, &setup.did, DEFAULT_METHOD_NAME).unwrap();
            assert_eq!(full_qualified_did, format!("{}{}", DEFAULT_PREFIX, setup.did));
        }

        #[test]
        fn qualify_did_for_updating_prefix() {
            let setup = Setup::did();

            let full_qualified_did = did::qualify_did(setup.wallet_handle, &setup.did, DEFAULT_METHOD_NAME).unwrap();

            let new_full_qualified_did = did::qualify_did(setup.wallet_handle, &full_qualified_did, CUSTOM_METHOD).unwrap();
            assert_eq!(new_full_qualified_did, format!("did:{}:{}", CUSTOM_METHOD, setup.did));
        }

        #[test]
        fn qualify_did_for_keeping_related_entities() {
            let setup = Setup::new_identity();

            // set Metadata
            did::set_did_metadata(setup.wallet_handle, &setup.did, METADATA).unwrap();

            // set Endpoint
            did::set_endpoint_for_did(setup.wallet_handle, &setup.did, ENDPOINT, VERKEY).unwrap();

            // set Temporary Verkey
            let temp_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, "{}").unwrap();

            // set Pairwise
            did::store_their_did(setup.wallet_handle, &json!({"did": DID}).to_string()).unwrap();
            utils::pairwise::create_pairwise(setup.wallet_handle, DID, &setup.did, None).unwrap();

            let identity_json = json!({"did": DID_TRUSTEE, "verkey": VERKEY_TRUSTEE}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();
            utils::pairwise::create_pairwise(setup.wallet_handle, DID_TRUSTEE, &setup.did, None).unwrap();

            let full_qualified_did = did::qualify_did(setup.wallet_handle, &setup.did, DEFAULT_METHOD_NAME).unwrap();
            assert_eq!(full_qualified_did, format!("{}{}", DEFAULT_PREFIX, setup.did));

            {
                // check key for did
                let res = did::key_for_local_did(setup.wallet_handle, &setup.did);
                assert_code!(ErrorCode::WalletItemNotFound, res);

                let verkey = did::key_for_local_did(setup.wallet_handle, &full_qualified_did).unwrap();
                assert_eq!(setup.verkey, verkey);
            }

            {
                // check did metadata
                let res = did::get_did_metadata(setup.wallet_handle, &setup.did);
                assert_code!(ErrorCode::WalletItemNotFound, res);

                let meta = did::get_did_metadata(setup.wallet_handle, &full_qualified_did).unwrap();
                assert_eq!(METADATA.to_string(), meta);
            }

            {
                // check endpoint
                let res = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, &setup.did);
                assert_code!(ErrorCode::CommonInvalidState, res); // TODO: IS is correct code WalletItemNotFound LedgerNotFound?

                let (endpoint, verkey) = did::get_endpoint_for_did(setup.wallet_handle, INVALID_POOL_HANDLE, &full_qualified_did).unwrap();
                assert_eq!(ENDPOINT.to_string(), endpoint);
                assert_eq!(VERKEY.to_string(), verkey.unwrap());
            }

            {
                // check temporary key
                let res = did::get_my_did_with_metadata(setup.wallet_handle, &setup.did);
                assert_code!(ErrorCode::WalletItemNotFound, res);

                let meta = did::get_my_did_with_metadata(setup.wallet_handle, &full_qualified_did).unwrap();
                let meta: serde_json::Value = serde_json::from_str(&meta).unwrap();
                assert_eq!(temp_verkey, meta["tempVerkey"].as_str().unwrap().to_string());
            }

            {
                // check pairwise 1
                let pairwise = utils::pairwise::get_pairwise(setup.wallet_handle, DID).unwrap();
                let pairwise: serde_json::Value = serde_json::from_str(&pairwise).unwrap();
                assert_eq!(full_qualified_did, pairwise["my_did"].as_str().unwrap().to_string());

                // check pairwise 2
                let pairwise = utils::pairwise::get_pairwise(setup.wallet_handle, DID_TRUSTEE).unwrap();
                let pairwise: serde_json::Value = serde_json::from_str(&pairwise).unwrap();
                assert_eq!(full_qualified_did, pairwise["my_did"].as_str().unwrap().to_string());
            }
        }
    }
}

#[cfg(not(feature = "only_high_cases"))]
mod medium_cases {
    use super::*;

    mod key_for_did {
        use super::*;

        #[test]
        fn indy_key_for_did_works_for_invalid_pool_handle() {
            let setup = Setup::wallet();

            let res = did::key_for_did(INVALID_POOL_HANDLE, setup.wallet_handle, DID_TRUSTEE);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = did::key_for_did(-1, INVALID_WALLET_HANDLE, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod key_for_local_did {
        use super::*;

        #[test]
        fn indy_key_for_local_did_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = did::key_for_local_did(INVALID_WALLET_HANDLE, DID_TRUSTEE);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod set_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_set_endpoint_for_did_works_for_replace() {
            let setup = Setup::wallet_and_pool();

            did::set_endpoint_for_did(setup.wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());

            let new_endpoint = "10.10.10.1:9710";
            did::set_endpoint_for_did(setup.wallet_handle, DID, new_endpoint, VERKEY_MY2).unwrap();
            let (updated_endpoint, updated_key) = did::get_endpoint_for_did(setup.wallet_handle, setup.pool_handle, DID).unwrap();
            assert_eq!(new_endpoint, updated_endpoint);
            assert_eq!(VERKEY_MY2, updated_key.unwrap());
        }
    }

    mod get_did_metadata {
        use super::*;

        #[test]
        fn indy_get_did_metadata_works_for_empty_string() {
            let setup = Setup::did();

            did::set_did_metadata(setup.wallet_handle, &setup.did, "").unwrap();
            let metadata = did::get_did_metadata(setup.wallet_handle, &setup.did).unwrap();
            assert_eq!("", metadata);
        }

        #[test]
        fn indy_get_did_metadata_works_for_invalid_handle() {
            Setup::empty();

            let res = did::get_did_metadata(INVALID_WALLET_HANDLE, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod create_my_did {
        use super::*;

        #[test]
        fn indy_create_my_did_works_as_cid() {
            let setup = Setup::wallet();

            let (my_did, my_verkey) = did::create_my_did(setup.wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();
            assert_eq!(my_did, VERKEY);
            assert_eq!(my_verkey, VERKEY);
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            let setup = Setup::wallet();

            let (my_did, my_verkey) = did::create_my_did(setup.wallet_handle, &format!(r#"{{"did":"{}","seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#, DID)).unwrap();
            assert_eq!(my_did, DID);
            assert_eq!(my_verkey, VERKEY);
        }

        #[test]
        fn indy_create_my_did_works_for_exists_crypto_type() {
            let setup = Setup::wallet();

            did::create_my_did(setup.wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = did::create_my_did(INVALID_WALLET_HANDLE, "{}");
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod replace_keys_start {
        use super::*;

        #[test]
        fn indy_replace_keys_start_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = did::replace_keys_start(INVALID_WALLET_HANDLE, DID, "{}");
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_replace_keys_start_works_for_seed() {
            let setup = Setup::did();

            let new_verkey = did::replace_keys_start(setup.wallet_handle, &setup.did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, VERKEY);
            assert_ne!(setup.verkey, new_verkey);
        }
    }

    mod replace_keys_apply {
        use super::*;

        #[test]
        fn indy_replace_keys_apply_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = did::replace_keys_apply(INVALID_WALLET_HANDLE, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_verkey_with_crypto_type() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID, "verkey": VERKEY.to_owned() + ":ed25519"}).to_string();
            did::store_their_did(setup.wallet_handle, &identity_json).unwrap();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_seed() {
            let setup = Setup::wallet();

            let res = did::create_my_did(setup.wallet_handle, r#"{"seed":"seed"}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_wallet_handle() {
            Setup::empty();

            let identity_json = json!({"did": DID}).to_string();
            let res = did::store_their_did(INVALID_WALLET_HANDLE, &identity_json);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_store_their_did_works_for_abbreviated_verkey() {
            let setup = Setup::wallet();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"~NcYxiDXkpYi6ov5FcYDi1e"}"#;
            did::store_their_did(setup.wallet_handle, identity_json).unwrap();
        }

        #[test]
        fn indy_store_their_did_works_for_abbreviated_verkey_for_fully_qualified() {
            let setup = Setup::wallet();

            let identity_json = r#"{"did":"did:sov:8wZcEriaNLNKtteJvx7f8i", "verkey":"~NcYxiDXkpYi6ov5FcYDi1e"}"#;
            did::store_their_did(setup.wallet_handle, identity_json).unwrap();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_json() {
            let setup = Setup::wallet();

            let res = did::create_my_did(setup.wallet_handle, r#"{"seed":123}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": INVALID_BASE58_DID}).to_string();
            let res = did::store_their_did(setup.wallet_handle, &identity_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_verkey() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": "did", "verkey":"invalid_base58string"}).to_string();
            let res = did::store_their_did(setup.wallet_handle, &identity_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_store_their_did_works_for_verkey_with_invalid_crypto_type() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID, "verkey": VERKEY.to_owned() + ":crypto_type"}).to_string();
            let res = did::store_their_did(setup.wallet_handle, &identity_json);
            assert_code!(ErrorCode::UnknownCryptoTypeError, res);
        }

        #[test]
        fn indy_store_my_did_works_for_is_802() {
            let setup = Setup::wallet();

            let identity_json = json!({"did": DID}).to_string();

            // 1. Try 'createAndStoreMyDid' operation with say did1 and verkey1
            did::create_my_did(setup.wallet_handle, &identity_json).unwrap();

            // 2. Repeat above operation (with same did and ver key used in #1)
            // but this time catch and swallow the exception (it will throw the exception WalletItemAlreadyExistsException)
            let res = did::create_my_did(setup.wallet_handle, &identity_json);
            assert_code!(ErrorCode::DidAlreadyExistsError, res);

            // 3. Then, now if you try 'createAndStoreMyDid' operation
            // (either with same did and verkey or you can choose different did and verkey),
            // in IS-802 it fails with error 'Storage error occurred during wallet operation.'
            let res = did::create_my_did
                (setup.wallet_handle, &identity_json);
            assert_code!(ErrorCode::DidAlreadyExistsError, res);
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_without_nym_transaction() {
            let setup = Setup::wallet_and_pool();

            let (my_did, _) = did::create_store_and_publish_my_did_from_trustee(setup.wallet_handle, setup.pool_handle).unwrap();

            did::replace_keys_start(setup.wallet_handle, &my_did, "{}").unwrap();
            did::replace_keys_apply(setup.wallet_handle, &my_did).unwrap();

            let schema_request = ledger::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let response = ledger::sign_and_submit_request(setup.pool_handle, setup.wallet_handle, &my_did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);
        }
    }


    mod abbreviate_verkey {
        use super::*;

        #[test]
        fn indy_abbreviate_verkey_works_for_invalid_did() {
            let res = did::abbreviate_verkey(INVALID_BASE58_DID, VERKEY_TRUSTEE);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_invalid_verkey() {
            let res = did::abbreviate_verkey(DID_TRUSTEE, INVALID_BASE58_VERKEY);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod list_my_dids_with_meta{
        use super::*;

        #[test]
        fn indy_list_dids(){
            let setup = Setup::did();
            let dids = did::list_my_dids_with_meta(setup.wallet_handle).unwrap();
            let info_list: serde_json::Value = serde_json::from_str(&dids).unwrap();
            assert_eq!(info_list.as_array().unwrap().len(), 1);
            assert!(info_list[0]["metadata"].is_null());
            assert_eq!(setup.verkey, info_list[0]["verkey"].as_str().unwrap().to_string());
        }

        #[test]
        fn indy_list_dids_after_creating_dids(){
            let setup = Setup::wallet();
            let mut did2verkey: HashMap<String, String> = HashMap::new();
            for _x in 0..10{
                let (my_did, my_verkey) = did::create_my_did(setup.wallet_handle, "{}").unwrap();
                did::set_did_metadata(setup.wallet_handle, &my_did, METADATA).unwrap();
                did2verkey.insert(String::from(my_did), String::from(my_verkey));
            }
            let dids = did::list_my_dids_with_meta(setup.wallet_handle).unwrap();
            let info_list: serde_json::Value = serde_json::from_str(&dids).unwrap();
            assert_eq!(info_list.as_array().unwrap().len(), 10);
            for info in info_list.as_array().unwrap() {
                assert_eq!(info["metadata"].as_str().unwrap().to_string(),
                           METADATA.to_string());
                assert_eq!(&info["verkey"].as_str().unwrap().to_string(),
                           did2verkey.get(&(info["did"]).as_str().unwrap().to_string()).unwrap());
            }
        }

        #[test]
        fn indy_list_dids_after_replace_keys_start(){
            let setup = Setup::wallet();
            let mut did2tempverkey: HashMap<String, String> = HashMap::new();
            for _x in 0..10{
                let (my_did, _my_verkey) = did::create_my_did(setup.wallet_handle, "{}").unwrap();
                let temp_verkey = did::replace_keys_start(setup.wallet_handle, &my_did, "{}").unwrap();
                did2tempverkey.insert(String::from(&my_did), temp_verkey);
            }
            let dids = did::list_my_dids_with_meta(setup.wallet_handle).unwrap();
            let info_list: serde_json::Value = serde_json::from_str(&dids).unwrap();
            for info in info_list.as_array().unwrap() {
                let did = info["did"].as_str().unwrap().to_string();
                assert_eq!(&info["tempVerkey"].as_str().unwrap().to_string(),
                           did2tempverkey.get(&did).unwrap());
            }
        }
    }
}
