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

use indy::api::ErrorCode;

use std::{thread, time};

pub const POOL: &'static str = "pool_1";
pub const MESSAGE: &'static str = r#"{"reqId":1496822211362017764}"#;
pub const ENCRYPTED_MESSAGE: &'static str = "4SWFzd3sx7xNemZEtktt3s558Fa28fGbauAZv9NRQjQhHq9bwT8uBnACQJAKzZ";
pub const NONCE: &'static str = "Dd3vSQgDdADJGoxb6BPcWU6wkEMqSeFwv";
pub const TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";
pub const MY1_SEED: &'static str = "00000000000000000000000000000My1";


mod high_cases {
    use super::*;

    mod create_my_did {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_my_did_works_for_empty_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();

            assert_eq!(my_did, "NcYxiDXkpYi6ov5FcYDi1e");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_as_cid() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();

            assert_eq!(my_did, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            TestUtils::cleanup_storage();
            let did = "8wZcEriaNLNKtteJvx7f8i".to_string();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle,
                                                                    &format!(r#"{{"did":"{}","seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#, did)).unwrap();

            assert_eq!(my_did, did);
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_exists_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            SignusUtils::create_my_did(wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();

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

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, "{}").unwrap();

            assert!(new_verkey != my_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::replace_keys(wallet_handle, "invalid_base58_string", "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::replace_keys(invalid_wallet_handle, &my_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");
            assert_ne!(my_verkey, new_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

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

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;
            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::store_their_did(invalid_wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_with_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_without_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}"#;
            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa", "crypto_type": "ed25519"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;

            let expected_signature = r#"R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa"#;

            let signature = SignusUtils::sign(wallet_handle, &my_did, message).unwrap();
            assert_eq!(expected_signature, signature);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_unknow_signer() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = SignusUtils::sign(wallet_handle, "did", MESSAGE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::sign(invalid_wallet_handle, &my_did, MESSAGE);
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
            let pool_name = "indy_verify_works_for_verkey_cached_in_wallet";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (did, verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);

            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;

            let signature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa";

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &did, message, signature).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_verkey_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_get_verkey_from_ledger";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let identity_json = format!(r#"{{"did":"{}"}}"#, my_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();


            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, message, signature).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_expired_nym() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_expired_nym";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            WalletUtils::create_wallet(pool_name, "wallet1", None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet("wallet1", Some(r#"{"freshness_time":1}"#)).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, my_did, my_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            thread::sleep(time::Duration::from_secs(2));

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, &message, signature).unwrap();
            assert!(valid);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_verify_works_for_invalid_wallet_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::verify(invalid_wallet_handle, pool_handle, "did", message, signature);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_verify_works_for_invalid_pool_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::verify(wallet_handle, invalid_pool_handle, "did", message, signature);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_other_signer() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_verify_works_for_other_signer";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (did, verkey, _) = SignusUtils::create_my_did(wallet_handle, &format!(r#"{{"seed":"{}"}}"#, TRUSTEE_SEED)).unwrap();
            let (other_did, other_verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"000000000000000000000000Steward1"}"#).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, other_did, other_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;

            let signature = SignusUtils::sign(wallet_handle, &did, message).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &other_did, message, &signature).unwrap();
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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_get_pk_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &their_did.clone(), Some(&their_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, their_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            SignusUtils::encrypt(wallet_handle, pool_handle, &trustee_did, &their_did, MESSAGE).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_get_nym_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &their_did.clone(), Some(&their_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            SignusUtils::encrypt(wallet_handle, pool_handle, &trustee_did, &their_did, MESSAGE).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_expired_nym() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            WalletUtils::create_wallet(POOL, "wallet1", None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet("wallet1", Some(r#"{"freshness_time":1}"#)).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &their_did.clone(), Some(&their_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            thread::sleep(time::Duration::from_secs(2));

            SignusUtils::encrypt(wallet_handle, pool_handle, &trustee_did, &their_did, MESSAGE).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_encrypt_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::encrypt(invalid_wallet_handle, pool_handle, &my_did, &their_did, MESSAGE);
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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::encrypt(wallet_handle, invalid_pool_handle, &my_did, &their_did, MESSAGE);
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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let decrypted_message = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE).unwrap();

            assert_eq!(MESSAGE, decrypted_message);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_other_coder() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, my_did, my_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let (encrypted_message, nonce) = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &my_did, MESSAGE).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, &encrypted_message, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_nonce_not_correspond_message() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let nonce = "acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7";

            let res = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::decrypt(invalid_wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

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

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"did":"invalid_base58_did"}"#);
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

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_works_for_not_exists_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            //TODO may be we must return WalletNotFound in case if key not exists in wallet
            SignusUtils::replace_keys(wallet_handle, "8wZcEriaNLNKtteJvx7f8i", "{}").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"crypto_type":"ed25519"}"#).unwrap();
            assert!(my_verkey != new_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let res = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"crypto_type":"type"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "crypto_type":"type"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"invalid_base58_string"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let identity_json = r#"{"did":"did", "verkey":"verkey"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();


            TestUtils::cleanup_storage();
        }
    }

    mod verify {
        use super::*;

        #[test]
        fn indy_verify_works_for_invalid_signature_string() {
            TestUtils::cleanup_storage();
            let pool_name = "indy_verify_works_for_invalid_message";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (did, verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);

            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let message = r#""reqId":1496822211362017764"#;
            let signature = "tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr";

            let res = SignusUtils::verify(wallet_handle, pool_handle, &did, message, signature);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_nym_from_ledger_with_incompatible_wallet() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_get_nym_from_ledger_with_incompatible_wallet";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("other_pool_name", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let identity_json = format!(r#"{{"did":"{}"}}"#, my_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::verify(wallet_handle, pool_handle, &my_did, message, signature);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletIncompatiblePoolError);

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_ledger_not_found_nym() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_get_ledger_not_found_nym";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"0000000000000000000000000000Fake"}"#).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let identity_json = format!(r#"{{"did":"{}"}}"#, my_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::verify(wallet_handle, pool_handle, &my_did, message, signature);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState); //TODO maybe we need add LedgerNotFound error

            PoolUtils::close(pool_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_verify_works_for_get_nym_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_no_nym_in_wallet";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, message, signature).unwrap();
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

            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, "BzfFCYk", &their_did, MESSAGE);
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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, their_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE);
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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
            let (their_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();

            let res = SignusUtils::encrypt(wallet_handle, pool_handle, &my_did, &their_did, MESSAGE);
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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, "BzfFCYk", &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_unknown_coder_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_saved_coder_nym_without_pk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, their_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let res = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_decrypt_works_for_invalid_nonce() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
            let (their_did, their_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();

            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, their_did, their_verkey);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let nonce = "acS2SQg";

            let res = SignusUtils::decrypt(wallet_handle, &my_did, &their_did, ENCRYPTED_MESSAGE, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
