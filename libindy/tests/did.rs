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

use utils::{wallet, did, pool, ledger};
use utils::constants::*;
use utils::types::ResponseType;

use self::indy::ErrorCode;

use api::{INVALID_WALLET_HANDLE, INVALID_POOL_HANDLE};

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
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_key_for_did_works_for_my_did");

            let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = did::key_for_did(-1, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_did_works_for_my_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_did_works_for_their_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_key_for_did_works_for_their_did");

            did::store_their_did_from_parts(wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = did::key_for_did(-1, wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_did_works_for_their_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_did_works_for_get_key_from_ledger() {
            let (trustee_wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_key_for_did_works_for_get_key_from_ledger");
            let (did, verkey) = did::create_and_store_my_did(trustee_wallet_handle, None).unwrap();

            let nym_request = ledger::build_nym_request(&trustee_did, &did, Some(&verkey), None, None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, trustee_wallet_handle, &trustee_did, &nym_request).unwrap();

            let get_nym_request = ledger::build_get_nym_request(Some(&did), &did).unwrap();
            ledger::submit_request_with_retries(pool_handle, &get_nym_request, &nym_resp).unwrap();

            wallet::close_wallet(trustee_wallet_handle).unwrap();
            wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();

            let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet("indy_key_for_did_works_for_get_key_from_ledger_2").unwrap();
            let received_verkey = did::key_for_did(pool_handle, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_key_for_did_works_for_get_key_from_ledger", &wallet_config);
        }

        #[test]
        fn indy_key_for_did_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_key_for_did_works_for_unknown_did");

            let res = did::key_for_did(pool_handle, wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_key_for_did_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_pool_handle() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_key_for_did_works_for_invalid_pool_handle");

            let res = did::key_for_did(INVALID_POOL_HANDLE, wallet_handle, DID_TRUSTEE);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_key_for_did_works_for_invalid_pool_handle", &wallet_config);
        }

        #[test]
        fn indy_key_for_did_works_for_invalid_wallet_handle() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_key_for_did_works_for_invalid_wallet_handle");

            let res = did::key_for_did(-1, INVALID_WALLET_HANDLE, &did);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_did_works_for_invalid_wallet_handle", &wallet_config);
        }
    }

    mod key_for_local_did {
        use super::*;

        #[test]
        fn indy_key_for_local_did_works_for_my_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_key_for_local_did_works_for_my_did");

            let (did, verkey) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = did::key_for_local_did(wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_local_did_works_for_my_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_local_did_works_for_their_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_key_for_local_did_works_for_their_did");

            did::store_their_did_from_parts(wallet_handle, DID, VERKEY).unwrap();

            let received_verkey = did::key_for_local_did(wallet_handle, DID).unwrap();
            assert_eq!(VERKEY, received_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_local_did_works_for_their_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_local_did_works_for_unknown_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_key_for_local_did_works_for_unknown_did");

            let res = did::key_for_local_did(wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_local_did_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_key_for_local_did_works_for_invalid_wallet_handle() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_key_for_local_did_works_for_invalid_wallet_handle");

            let res = did::key_for_local_did(INVALID_WALLET_HANDLE, &did);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_key_for_local_did_works_for_invalid_wallet_handle", &wallet_config);
        }
    }

    mod set_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_set_endpoint_for_did_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_endpoint_for_did_works");

            did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_endpoint_for_did_works", &wallet_config);
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_replace() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_set_endpoint_for_did_works_for_replace");

            did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();
            let (endpoint, key) = did::get_endpoint_for_did(wallet_handle, pool_handle, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());

            let new_endpoint = "10.10.10.1:9710";
            did::set_endpoint_for_did(wallet_handle, DID, new_endpoint, VERKEY_MY2).unwrap();
            let (updated_endpoint, updated_key) = did::get_endpoint_for_did(wallet_handle, pool_handle, DID).unwrap();
            assert_eq!(new_endpoint, updated_endpoint);
            assert_eq!(VERKEY_MY2, updated_key.unwrap());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_set_endpoint_for_did_works_for_replace", &wallet_config);
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_endpoint_for_did_works_for_invalid_did");

            let res = did::set_endpoint_for_did(wallet_handle, INVALID_BASE58_DID, ENDPOINT, VERKEY);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_endpoint_for_did_works_for_invalid_did", &wallet_config);
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_transport_key() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_endpoint_for_did_works_for_invalid_transport_key");

            let res = did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_BASE58_VERKEY);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            let res = did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, INVALID_VERKEY_LENGTH);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_endpoint_for_did_works_for_invalid_transport_key", &wallet_config);
        }

        #[test]
        fn indy_set_endpoint_for_did_works_for_invalid_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_endpoint_for_did_works_for_invalid_handle");

            let res = did::set_endpoint_for_did(INVALID_WALLET_HANDLE, DID, ENDPOINT, VERKEY);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_endpoint_for_did_works_for_invalid_handle", &wallet_config);
        }
    }

    mod get_endpoint_for_did {
        use super::*;

        #[test]
        fn indy_get_endpoint_for_did_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_get_endpoint_for_did_works");

            did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let (endpoint, key) = did::get_endpoint_for_did(wallet_handle, -1, DID).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY, key.unwrap());

            utils::tear_down_with_wallet(wallet_handle, "indy_get_endpoint_for_did_works", &wallet_config);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_get_endpoint_for_did_works_from_ledger");

            let attrib_data = json!({"endpoint": {"ha": ENDPOINT, "verkey": VERKEY_TRUSTEE}}).to_string();
            let attrib_request = ledger::build_attrib_request(&trustee_did, &trustee_did, None, Some(&attrib_data), None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));

            let (endpoint, key) = did::get_endpoint_for_did(wallet_handle, pool_handle, &trustee_did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(VERKEY_TRUSTEE, key.unwrap());

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_endpoint_for_did_works_from_ledger", &wallet_config);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_from_ledger_for_address_only() {
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_get_endpoint_for_did_works_from_ledger_for_address_only");

            let attrib_data = json!({"endpoint": {"ha": ENDPOINT}}).to_string();
            let attrib_request = ledger::build_attrib_request(&trustee_did, &trustee_did, None, Some(&attrib_data), None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &attrib_request).unwrap();

            thread::sleep(std::time::Duration::from_millis(1000));

            let (endpoint, key) = did::get_endpoint_for_did(wallet_handle, pool_handle, &trustee_did).unwrap();
            assert_eq!(ENDPOINT, endpoint);
            assert_eq!(None, key);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_endpoint_for_did_works_from_ledger_for_address_only", &wallet_config);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_for_unknown_did() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_endpoint_for_did_works_for_unknown_did");

            let res = did::get_endpoint_for_did(wallet_handle, pool_handle, DID);
            assert_code!(ErrorCode::CommonInvalidState, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_endpoint_for_did_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_poll_handle() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_endpoint_for_did_works_invalid_poll_handle");

            let res = did::get_endpoint_for_did(wallet_handle, INVALID_POOL_HANDLE, DID);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_endpoint_for_did_works_invalid_poll_handle", &wallet_config);
        }

        #[test]
        fn indy_get_endpoint_for_did_works_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_get_endpoint_for_did_works_invalid_wallet_handle");

            did::set_endpoint_for_did(wallet_handle, DID, ENDPOINT, VERKEY).unwrap();

            let res = did::get_endpoint_for_did(INVALID_WALLET_HANDLE, -1, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_endpoint_for_did_works_invalid_wallet_handle", &wallet_config);
        }
    }

    mod set_did_metadata {
        use super::*;

        #[test]
        fn indy_set_did_metadata_works() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_set_did_metadata_works");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_their_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_did_metadata_works_for_their_did");

            did::store_their_did_from_parts(wallet_handle, DID, VERKEY).unwrap();

            did::set_did_metadata(wallet_handle, DID, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_their_did", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_replace() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_set_did_metadata_works_for_replace");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();
            let metadata = did::get_did_metadata(wallet_handle, &did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            did::set_did_metadata(wallet_handle, &did, new_metadata).unwrap();
            let updated_metadata = did::get_did_metadata(wallet_handle, &did).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_replace", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_empty_string() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_set_did_metadata_works_for_empty_string");

            did::set_did_metadata(wallet_handle, &did, "").unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_empty_string", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_did_metadata_works_for_invalid_did");

            let res = did::set_did_metadata(wallet_handle, INVALID_BASE58_DID, METADATA);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_invalid_did", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_unknown_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_did_metadata_works_for_unknown_did");

            did::set_did_metadata(wallet_handle, &DID, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_set_did_metadata_works_for_invalid_handle() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_set_did_metadata_works_for_invalid_handle");

            let res = did::set_did_metadata(INVALID_WALLET_HANDLE, &did, METADATA);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_did_metadata_works_for_invalid_handle", &wallet_config);
        }
    }

    mod get_did_metadata {
        use super::*;

        #[test]
        fn indy_get_did_metadata_works() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_did_metadata_works");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let metadata = did::get_did_metadata(wallet_handle, &did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works", &wallet_config);
        }

        #[test]
        fn indy_get_did_metadata_works_for_their_did() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_did_metadata_works_for_their_did");

            did::store_their_did_from_parts(wallet_handle, &did, VERKEY).unwrap();

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let metadata = did::get_did_metadata(wallet_handle, &did).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works_for_their_did", &wallet_config);
        }

        #[test]
        fn indy_get_did_metadata_works_for_empty_string() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_did_metadata_works_for_empty_string");

            did::set_did_metadata(wallet_handle, &did, "").unwrap();

            let metadata = did::get_did_metadata(wallet_handle, &did).unwrap();
            assert_eq!("", metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works_for_empty_string", &wallet_config);
        }

        #[test]
        fn indy_get_did_metadata_works_for_no_metadata() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_did_metadata_works_for_no_metadata");

            let res = did::get_did_metadata(wallet_handle, &did);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works_for_no_metadata", &wallet_config);
        }

        #[test]
        fn indy_get_did_metadata_works_for_unknown_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_get_did_metadata_works_for_unknown_did");

            let res = did::get_did_metadata(wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_get_did_metadata_works_for_invalid_handle() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_did_metadata_works_for_invalid_handle");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            let res = did::get_did_metadata(INVALID_WALLET_HANDLE, &did);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_did_metadata_works_for_invalid_handle", &wallet_config);
        }
    }

    mod get_my_did_metadata {
        use super::*;

        #[test]
        fn indy_get_my_did_metadata_works() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_my_did_metadata_works");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();

            did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_get_my_did_metadata_works", &wallet_config);
        }


        #[test]
        fn indy_get_my_did_metadata_works_for_no_metadata() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_my_did_metadata_works_for_no_metadata");

            did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_get_my_did_metadata_works_for_no_metadata", &wallet_config);
        }

        #[test]
        fn indy_get_my_did_metadata_works_with_temp_verkey() {
            let (wallet_handle, did, wallet_config) = utils::setup_did("indy_get_my_did_metadata_works_with_temp_verkey");

            did::set_did_metadata(wallet_handle, &did, METADATA).unwrap();
            did::replace_keys_start(wallet_handle, &did, "{}").unwrap();

            did::get_my_did_with_metadata(wallet_handle, &did).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_get_my_did_metadata_works_with_temp_verkey", &wallet_config);
        }

        #[test]
        fn indy_get_my_did_metadata_works_for_unknown_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_get_my_did_metadata_works_for_unknown_did");

            let res = did::get_my_did_with_metadata(wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_my_did_metadata_works_for_unknown_did", &wallet_config);
        }
    }

    mod create_my_did {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_my_did_works_for_empty_json() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_empty_json");

            let (my_did, my_verkey) = did::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_empty_json", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_with_seed");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            assert_eq!(my_did, DID_MY1);
            assert_eq!(my_verkey, VERKEY_MY1);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_with_seed", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_with_hex_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_with_hex_seed");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, Some("94a823a6387cdd30d8f7687d95710ebab84c6e277b724790a5b221440beb7df6")).unwrap();

            assert_eq!(my_did, "HWvjYf77k1dqQAk6sE4gaS");
            assert_eq!(my_verkey, "A16wi1xHBu5KT4SqNhZXrKZfoQbXJCbDozgSTJhUgu9x");

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_with_hex_seed", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_as_cid() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_as_cid");

            let (my_did, my_verkey) = did::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();

            assert_eq!(my_did, VERKEY);
            assert_eq!(my_verkey, VERKEY);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_as_cid", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_with_passed_did");

            let (my_did, my_verkey) = did::create_my_did(wallet_handle, &format!(r#"{{"did":"{}","seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#, DID)).unwrap();
            assert_eq!(my_did, DID);
            assert_eq!(my_verkey, VERKEY);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_with_passed_did", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_for_exists_crypto_type() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_exists_crypto_type");

            did::create_my_did(wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_exists_crypto_type", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_invalid_wallet_handle");

            let res = did::create_my_did(INVALID_WALLET_HANDLE, "{}");
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_for_duplicate() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_duplicate");

            let (my_did, _) = did::create_my_did(wallet_handle, "{}").unwrap();
            let res = did::create_my_did(wallet_handle, &format!(r#"{{"did":{:?}}}"#, my_did));
            assert_code!(ErrorCode::DidAlreadyExistsError, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_duplicate", &wallet_config);
        }
    }

    mod replace_keys_start {
        use super::*;

        #[test]
        fn indy_replace_keys_start_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_start_works");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let new_verkey = did::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_ne!(new_verkey, my_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_start_works", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_start_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_start_works_for_invalid_wallet_handle");

            let (my_did, _) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let res = did::replace_keys_start(INVALID_WALLET_HANDLE, &my_did, "{}");
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_start_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_start_works_for_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_start_works_for_seed");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let new_verkey = did::replace_keys_start(wallet_handle, &my_did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, VERKEY);
            assert_ne!(my_verkey, new_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_start_works_for_seed", &wallet_config);
        }
    }

    mod replace_keys_apply {
        use super::*;

        #[test]
        fn indy_replace_keys_apply_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_apply_works");

            let (my_did, my_verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();

            let new_verkey = did::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            assert_ne!(new_verkey, my_verkey);

            did::replace_keys_apply(wallet_handle, &my_did).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_apply_works", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_apply_works_without_calling_replace_start() {
            let (wallet_handle, my_did, wallet_config) = utils::setup_did("indy_replace_keys_apply_works_without_calling_replace_start");

            let res = did::replace_keys_apply(wallet_handle, &my_did);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_apply_works_without_calling_replace_start", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_apply_works_for_unknown_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_apply_works_for_unknown_did");

            let res = did::replace_keys_apply(wallet_handle, DID);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_apply_works_for_unknown_did", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_apply_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_replace_keys_apply_works_for_invalid_wallet_handle");

            let res = did::replace_keys_apply(INVALID_WALLET_HANDLE, DID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_replace_keys_apply_works_for_invalid_wallet_handle", &wallet_config);
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_did_only() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_did_only");

            let identity_json = json!({"did": DID}).to_string();
            did::store_their_did(wallet_handle, &identity_json).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_did_only", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_verkey() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_verkey");

            let identity_json = json!({"did": DID, "verkey": VERKEY}).to_string();
            did::store_their_did(wallet_handle, &identity_json).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_verkey", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_verkey_with_crypto_type() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_verkey_with_crypto_type");

            let identity_json = json!({"did": DID, "verkey": VERKEY.to_owned() + ":ed25519"}).to_string();
            did::store_their_did(wallet_handle, &identity_json).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_verkey_with_crypto_type", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_invalid_seed");

            let res = did::create_my_did(wallet_handle, r#"{"seed":"seed"}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_invalid_seed", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_invalid_wallet_handle");

            let identity_json = json!({"did": DID}).to_string();
            let res = did::store_their_did(INVALID_WALLET_HANDLE, &identity_json);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_abbreviated_verkey() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_abbreviated_verkey");

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"~NcYxiDXkpYi6ov5FcYDi1e"}"#;
            did::store_their_did(wallet_handle, identity_json).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_abbreviated_verkey", &wallet_config);
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_json() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_my_did_works_for_invalid_json");

            let res = did::create_my_did(wallet_handle, r#"{"seed":123}"#);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_my_did_works_for_invalid_json", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_invalid_did");

            let identity_json = json!({"did": INVALID_BASE58_DID}).to_string();
            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_invalid_did", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_verkey() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_invalid_verkey");

            let identity_json = json!({"did": "did", "verkey":"invalid_base58string"}).to_string();
            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_invalid_verkey", &wallet_config);
        }


        #[test]
        fn indy_store_their_did_works_for_verkey_with_invalid_crypto_type() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_verkey_with_invalid_crypto_type");

            let identity_json = json!({"did": DID, "verkey": VERKEY.to_owned() + ":crypto_type"}).to_string();
            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::UnknownCryptoTypeError, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_verkey_with_invalid_crypto_type", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_twice() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_twice");

            let identity_json = json!({"did": DID}).to_string();
            did::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_twice", &wallet_config);
        }

        #[test]
        fn indy_store_their_did_works_for_is_802() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_store_their_did_works_for_is_802");

            let identity_json = json!({"did": DID, "verkey": VERKEY}).to_string();

            // 1. Try 'storeTheirDid' operation with say did1 and verkey1
            did::store_their_did(wallet_handle, &identity_json).unwrap();

            // 2. Repeat above operation (with same did and ver key used in #1)
            // but this time catch and swallow the exception (it will throw the exception WalletItemAlreadyExistsException)
            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);

            // 3. Then, now if you try 'storeTheirDid' operation
            // (either with same did and verkey or you can choose different did and verkey),
            // in IS-802 it fails with error 'Storage error occurred during wallet operation.'
            let res = did::store_their_did(wallet_handle, &identity_json);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_store_their_did_works_for_is_802", &wallet_config);
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_demo() {
            // 1. Create and open pool
            // 2. Create and open wallet
            // 3. Generate did from Trustee seed
            let (wallet_handle, pool_handle, trustee_did, wallet_config) = utils::setup_trustee("indy_replace_keys_demo");

            // 4. Generate my did
            let (my_did, my_verkey) = did::create_my_did(wallet_handle, "{}").unwrap();

            // 5. Send Nym request to Ledger
            let nym_request = ledger::build_nym_request(&trustee_did, &my_did, Some(&my_verkey), None, None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            // 6. Start replacing of keys
            let new_verkey = did::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();

            // 7. Send Nym request to Ledger with new verkey
            let nym_request = ledger::build_nym_request(&my_did, &my_did, Some(&new_verkey), None, None).unwrap();
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request).unwrap();

            // 8. Send Schema request before apply replacing of keys
            let schema_request = ledger::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            // 9. Apply replacing of keys
            did::replace_keys_apply(wallet_handle, &my_did).unwrap();

            // 10. Send Schema request
            ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_replace_keys_demo", &wallet_config);
        }

        #[test]
        fn indy_replace_keys_without_nym_transaction() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_replace_keys_without_nym_transaction");

            let (my_did, _) = did::create_store_and_publish_my_did_from_trustee(wallet_handle, pool_handle).unwrap();

            did::replace_keys_start(wallet_handle, &my_did, "{}").unwrap();
            did::replace_keys_apply(wallet_handle, &my_did).unwrap();

            let schema_request = ledger::build_schema_request(&my_did, SCHEMA_DATA).unwrap();
            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            pool::check_response_type(&response, ResponseType::REQNACK);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_replace_keys_without_nym_transaction", &wallet_config);
        }
    }

    mod abbreviate_verkey {
        use super::*;

        #[test]
        fn indy_abbreviate_verkey_works_for_abbr_key() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_abbreviate_verkey_works_for_abbr_key");

            let (did, verkey) = did::create_my_did(wallet_handle, "{}").unwrap();

            let abbr_verkey = did::abbreviate_verkey(&did, &verkey).unwrap();

            assert_ne!(verkey, abbr_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_abbreviate_verkey_works_for_abbr_key", &wallet_config);
        }

        #[test]
        fn indy_abbreviate_verkey_works_for_not_abbr_key() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_abbreviate_verkey_works_for_not_abbr_key");

            let (did, verkey) = did::create_my_did(wallet_handle, &format!(r#"{{"did":{:?}}}"#, DID_TRUSTEE)).unwrap();

            let full_verkey = did::abbreviate_verkey(&did, &verkey).unwrap();

            assert_eq!(verkey, full_verkey);

            utils::tear_down_with_wallet(wallet_handle, "indy_abbreviate_verkey_works_for_not_abbr_key", &wallet_config);
        }

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
}
