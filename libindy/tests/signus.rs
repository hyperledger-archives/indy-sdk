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

mod high_cases {
    use super::*;

    mod create_my_did {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_my_did_works_for_empty_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.from_base58().unwrap().len(), 16);
            assert_eq!(my_verkey.from_base58().unwrap().len(), 32);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();

            assert_eq!(my_did, "NcYxiDXkpYi6ov5FcYDi1e");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_as_cid() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cid":true}"#).unwrap();

            assert_eq!(my_did, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_with_passed_did() {
            TestUtils::cleanup_storage();
            let did = "8wZcEriaNLNKtteJvx7f8i".to_string();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle,
                                                                    &format!(r#"{{"did":"{}","seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}}"#, did)).unwrap();

            assert_eq!(my_did, did);
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_exists_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            SignusUtils::create_my_did(wallet_handle, r#"{"crypto_type":"ed25519"}"#).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::create_my_did(invalid_wallet_handle, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, "{}").unwrap();

            assert!(new_verkey != my_verkey);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let res = SignusUtils::replace_keys(wallet_handle, "invalid_base58_string", "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::replace_keys(invalid_wallet_handle, &my_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"seed":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).unwrap();
            assert_eq!(new_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            assert!(my_verkey != new_verkey);

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"field":"value"}"#;
            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;
            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::store_their_did(invalid_wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_with_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_without_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}"#;
            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa", "crypto_type": "ed25519"}"#;
            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn indy_sign_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();

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

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let message = r#"{"reqId":1495034346617224651}"#;

            let res = SignusUtils::sign(wallet_handle, "did", message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, r#"{}"#).unwrap();

            let message = r#"{"reqId":1495034346617224651,}"#;

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::sign(invalid_wallet_handle, &my_did, message);
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

            let (did, verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
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

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("00000000000000000000000000000My1")).unwrap();

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

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("00000000000000000000000000000My1")).unwrap();

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

            let (did, verkey, _) = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"000000000000000000000000Trustee1"}"#).unwrap();
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
}

mod medium_cases {
    use super::*;

    mod create_my_did {
        use super::*;

        #[test]
        fn indy_create_my_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"crypto_type":"type"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"seed":"seed"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"did":"invalid_base58_did"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_my_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, r#"{"seed":123}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn indy_replace_keys_works_for_not_exists_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            //TODO may be we must return WalletNotFound in case if key not exists in wallet
            SignusUtils::replace_keys(wallet_handle, "8wZcEriaNLNKtteJvx7f8i", "{}").unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_correct_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (new_verkey, _) = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"crypto_type":"ed25519"}"#).unwrap();

            assert!(my_verkey != new_verkey);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_replace_keys_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let res = SignusUtils::replace_keys(wallet_handle, &my_did, r#"{"crypto_type":"type"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn indy_store_their_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "crypto_type":"type"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"invalid_base58_string"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_store_their_did_works_for_invalid_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

            let identity_json = r#"{"did":"did", "verkey":"verkey"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

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

            let (did, verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
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

            let (my_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("00000000000000000000000000000My1")).unwrap();

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
        fn indy_verify_works_for_no_nym_in_wallet() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_verify_works_for_no_nym_in_wallet";

            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, None).unwrap();

            let (trustee_did, _, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("000000000000000000000000Trustee1")).unwrap();
            let (my_did, my_verkey, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some("00000000000000000000000000000My1")).unwrap();

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
}
