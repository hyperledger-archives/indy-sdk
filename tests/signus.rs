// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::wallet::WalletUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;

use sovrin::api::ErrorCode;

mod high_cases {
    use super::*;

    mod create_my_did {
        use super::*;

        #[test]
        fn sovrin_create_my_did_works_for_empty_did_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            assert_eq!(my_did.len(), 22);
            assert_eq!(my_verkey.len(), 44);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_with_seed() {
            TestUtils::cleanup_storage();
            let pool_name = "sovrin_create_my_did_works_with_seed";

            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").unwrap();

            assert_eq!(my_did, "NcYxiDXkpYi6ov5FcYDi1e");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_as_cid() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", \"cid\":true}").unwrap();

            assert_eq!(my_did, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_with_cid_false() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", \"cid\":false}").unwrap();

            assert_eq!(my_did, "NcYxiDXkpYi6ov5FcYDi1e");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_with_passed_did() {
            TestUtils::cleanup_storage();
            let did = "8wZcEriaNLNKtteJvx7f8i".to_string();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle,
                                                                        &format!("{{\"did\":\"{}\",\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}}", did)).unwrap();

            assert_eq!(my_did, "8wZcEriaNLNKtteJvx7f8i");
            assert_eq!(my_verkey, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW");

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::create_my_did(invalid_wallet_handle, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":123}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn sovrin_replace_keys_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let (my_verkey, my_pk) = SignusUtils::replace_keys(wallet_handle, &my_did, "{}").unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_replace_keys_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let some_did = "invalid_base58_string";
            let res = SignusUtils::replace_keys(wallet_handle, some_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_replace_keys_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::replace_keys(invalid_wallet_handle, &my_did, "{}");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn sovrin_store_their_did_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;

            SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_store_their_did_works_for_invalid_json() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"field":"value"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_store_their_did_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i"}"#;

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::store_their_did(invalid_wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_store_their_did_works_with_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn sovrin_sign_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\"}").unwrap();

            let message = r#"{
                "reqId":1496822211362017764,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"VsKV7grR1BUE29mG2Fm2kX",
                    "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                }
            }"#;

            let expected_signature = r#""signature":"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW""#;

            let msg = SignusUtils::sign(wallet_handle, &my_did, message).unwrap();

            assert!(msg.contains(expected_signature));

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_sign_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let did = "some_did";

            let message = r#"{
                "reqId":1495034346617224651,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
                }
            }"#;

            let res = SignusUtils::sign(wallet_handle, &did, message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").unwrap();

            let message = r#"{
                "reqId":1495034346617224651,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
                }
            }"#;

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::sign(invalid_wallet_handle, &my_did, message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod verify {
        use super::*;

        #[test]
        fn sovrin_verify_works_for_verkey_cached_in_wallet() {
            TestUtils::cleanup_storage();
            let pool_name = "sovrin_verify_works_for_verkey_cached_in_wallet";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (did, verkey, pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\"}").unwrap();
            let identity_json = format!(r#"{{"did":"{}", "verkey":"{}"}}"#, did, verkey);

            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let message = r#"{
                "reqId":1496822211362017764,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"VsKV7grR1BUE29mG2Fm2kX",
                    "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                },
                "signature":"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
            }"#;

            let msg = SignusUtils::verify(wallet_handle, pool_handle, "V4SGRU86Z58d6TV7PBUe6f", message).unwrap();
            assert!(msg);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_verify_works_for_get_verkey_from_ledger() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_verify_works_for_get_verkey_from_ledger";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) =
                SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let message = r#"{"reqId":1496822211362017764}"#;
            let signed_messege = SignusUtils::sign(wallet_handle, &my_did, message).unwrap();

            let identity_json = format!(r#"{{"did":"{}"}}"#, my_did);
            SignusUtils::store_their_did(wallet_handle, &identity_json).unwrap();

            let valid = SignusUtils::verify(wallet_handle, pool_handle, &my_did, &signed_messege).unwrap();
            assert!(valid);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_verify_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "sovrin_verify_works_for_invalid_wallet_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let message = r#"{
                "reqId":1496822211362017764,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"VsKV7grR1BUE29mG2Fm2kX",
                    "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                },
                "signature":"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
            }"#;

            let invalid_wallet_handle = wallet_handle + 1;
            let res = SignusUtils::verify(invalid_wallet_handle, pool_handle, "did", message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_verify_works_for_invalid_pool_handle() {
            TestUtils::cleanup_storage();
            let pool_name = "sovrin_verify_works_for_invalid_pool_handle";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let message = r#"{
                "reqId":1496822211362017764,
                "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                "operation":{
                    "type":"1",
                    "dest":"VsKV7grR1BUE29mG2Fm2kX",
                    "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                },
                "signature":"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
            }"#;

            let invalid_pool_handle = pool_handle + 1;
            let res = SignusUtils::verify(wallet_handle, invalid_pool_handle, "did", message);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }

    }
}

mod medium_cases {
    use super::*;

    mod create_my_did {
        use super::*;

        #[test]
        fn sovrin_create_my_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();
            let crypto_type = "type";

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, &format!("{{\"crypto_type\":\"{}\"}}", crypto_type));
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_for_invalid_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"seed\"}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_create_my_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let res = SignusUtils::create_my_did(wallet_handle, "{\"did\":\"invalid_base_58_did\"}");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod replace_keys {
        use super::*;

        #[test]
        fn sovrin_replace_keys_works_for_not_exists_did() {
            TestUtils::cleanup_storage();
            let did = "8wZcEriaNLNKtteJvx7f8i";

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            //TODO may be we must return WalletNotFound in case if key not exists in wallet
            SignusUtils::replace_keys(wallet_handle, did, "{}").unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod store_their_did {
        use super::*;

        #[test]
        fn sovrin_store_their_did_works_for_invalid_crypto_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"8wZcEriaNLNKtteJvx7f8i", "crypto_type":"type"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::SignusUnknownCryptoError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_store_their_did_works_for_invalid_did() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"invalid_base58_string"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_store_their_did_works_for_invalid_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            let identity_json = r#"{"did":"did", "verkey":"verkey"}"#;

            let res = SignusUtils::store_their_did(wallet_handle, identity_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }
}