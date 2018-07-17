extern crate indy;
extern crate base64;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate named_type;
#[macro_use]
extern crate named_type_derive;
extern crate byteorder;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::crypto::CryptoUtils;
use utils::did::DidUtils;
use utils::test::TestUtils;
use utils::constants::*;

use indy::api::ErrorCode;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11];

mod high_cases {
    use super::*;

    mod create_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_key_works_for_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = CryptoUtils::create_key(wallet_handle + 1, None);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod set_key_metadata {
        use super::*;

        #[test]
        fn indy_set_key_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();
            let metadata = CryptoUtils::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            CryptoUtils::set_key_metadata(wallet_handle, &verkey, new_metadata).unwrap();
            let updated_metadata = CryptoUtils::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let res = CryptoUtils::set_key_metadata(wallet_handle + 1, &verkey, METADATA);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = CryptoUtils::set_key_metadata(wallet_handle, INVALID_BASE58_VERKEY, METADATA);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_key_metadata {
        use super::*;

        #[test]
        fn indy_get_key_metadata_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!("", metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let res = CryptoUtils::get_key_metadata(wallet_handle, &verkey);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let res = CryptoUtils::get_key_metadata(wallet_handle + 1, &verkey);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_sign {
        use super::*;

        #[test]
        fn indy_crypto_sign_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = CryptoUtils::sign(wallet_handle, &my_vk, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_sign_works_for_unknown_signer() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = CryptoUtils::sign(wallet_handle, VERKEY, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletItemNotFound);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let res = CryptoUtils::sign(wallet_handle + 1, &my_vk, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_verify {
        use super::*;

        #[test]
        fn indy_crypto_verify_works() {
            let valid = CryptoUtils::verify(&VERKEY_MY1, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_correct_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":ed25519";
            let valid = CryptoUtils::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_invalid_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":unknown_crypto";
            let res = CryptoUtils::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(ErrorCode::UnknownCryptoTypeError, res.unwrap_err());
        }


        #[test]
        fn indy_crypto_verify_works_for_other_signer() {
            let valid = CryptoUtils::verify(&VERKEY_MY2, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(!valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_invalid_signature_len() {
            let signature: Vec<u8> = vec![20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190];
            let res = CryptoUtils::verify(&VERKEY_MY1, MESSAGE.as_bytes(), &signature);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());
        }
    }

    mod auth_crypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_key() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            CryptoUtils::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (_, verkey) = DidUtils::create_and_store_my_did(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            CryptoUtils::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did_as_cid() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (_, verkey) = DidUtils::create_my_did(sender_wallet_handle, &format!(r#"{{"seed":"{}", "cid":true}}"#, MY1_SEED)).unwrap();

            CryptoUtils::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_unknown_sender_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = CryptoUtils::auth_crypt(wallet_handle, VERKEY_MY2, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = CryptoUtils::auth_crypt(invalid_wallet_handle, &verkey, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_recipient_vk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::auth_crypt(wallet_handle, &verkey, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod auth_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_decrypt_works() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let sender_vk = CryptoUtils::create_key(sender_wallet_handle, Some(MY1_SEED)).unwrap();
            let recipient_vk = CryptoUtils::create_key(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = CryptoUtils::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (vk, msg) = CryptoUtils::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(sender_vk, vk);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_invalid_msg() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let (recipient_did, recipient_vk) = DidUtils::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();
            DidUtils::store_their_did_from_parts(sender_wallet_handle, &recipient_did, &recipient_vk).unwrap();

            let encrypted_msg = format!(r#"{{"nonce":"Th7MpTaRZVRYnPiabds81Y12","sender":"{:?}","msg":"{:?}"}}"#, VERKEY, ENCRYPTED_MESSAGE.to_vec());

            let res = CryptoUtils::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_unknown_recipient_vk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let sender_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let encrypted_msg = CryptoUtils::auth_crypt(wallet_handle, &sender_vk, &VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let sender_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            let recipient_vk = CryptoUtils::create_key(wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = CryptoUtils::auth_crypt(wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::auth_decrypt(wallet_handle + 1, &recipient_vk, &encrypted_msg);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod anon_crypt {
        use super::*;

        #[test]
        fn indy_anon_crypt_works() {
            TestUtils::cleanup_storage();

            CryptoUtils::anon_crypt(VERKEY_MY2, &MESSAGE.as_bytes()).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_anon_crypt_works_for_invalid_their_vk() {
            TestUtils::cleanup_storage();

            let res = CryptoUtils::anon_crypt(INVALID_VERKEY_LENGTH, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            let res = CryptoUtils::anon_crypt(INVALID_BASE58_VERKEY, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            TestUtils::cleanup_storage();
        }
    }

    mod anon_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_anon_decrypt_works() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = CryptoUtils::anon_crypt(&verkey, MESSAGE.as_bytes()).unwrap();

            let msg = CryptoUtils::anon_decrypt(recipient_wallet_handle, &verkey, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_invalid_msg() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = "unencrypted message";
            let res = CryptoUtils::anon_decrypt(wallet_handle, &verkey, &encrypted_msg.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_unknown_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let encrypted_msg = CryptoUtils::anon_crypt(&VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = CryptoUtils::anon_crypt(&verkey, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::anon_decrypt(wallet_handle + 1, &verkey, &encrypted_msg);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod load {
    extern crate rand;

    use super::*;

    use self::rand::{Rng, OsRng};

    use std::cmp::max;
    use std::thread;
    use std::time::{Duration, SystemTime};

    const AGENT_CNT: usize = 10;
    const DATA_SZ: usize = 10 * 1024;
    const OPERATIONS_CNT: usize = 10;

    /**
     Environment variables can be used for tuning this test:
     - AGENTS_CNT - count of parallel agents
     - OPERATIONS_CNT - operations per agent (consequence in same agent)
     - DATA_SZ - data size for encryption
    */
    #[test]
    fn parallel_auth_encrypt() {
        TestUtils::cleanup_storage();

        let agent_cnt = std::env::var("AGENTS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(AGENT_CNT);
        let data_sz = std::env::var("DATA_SZ").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(DATA_SZ);
        let operations_cnt = std::env::var("OPERATIONS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(OPERATIONS_CNT);

        let mut agents = Vec::new();
        let mut os_rng = OsRng::new().unwrap();
        for _ in 0..agent_cnt {
            let wallet = WalletUtils::create_and_open_default_wallet().unwrap();
            let (_did, verkey) = DidUtils::create_and_store_my_did(wallet, None).unwrap();
            let mut data = vec![0u8; data_sz];
            os_rng.fill_bytes(&mut data.as_mut_slice());
            agents.push((wallet, verkey, data));
        }

        let start_time = SystemTime::now();

        let mut results = Vec::new();

        for (wallet, verkey, data) in agents {
            let thread = thread::spawn(move || {
                let mut time_diffs = Vec::new();
                for _ in 0..operations_cnt {
                    let time = SystemTime::now();
                    let _encrypted = CryptoUtils::auth_crypt(wallet, &verkey, &verkey, data.as_slice()).unwrap();
                    let time_diff = SystemTime::now().duration_since(time).unwrap();
                    time_diffs.push(time_diff);
                }

                WalletUtils::close_wallet(wallet).unwrap();
                time_diffs
            });
            results.push(thread);
        }

        let mut all_diffs = Vec::new();
        for result in results {
            all_diffs.push(result.join().unwrap());
        }
        let total_duration = SystemTime::now().duration_since(start_time).unwrap();

        let mut time_diff_max = Duration::from_secs(0);
        let mut time_sum_diff = Duration::from_secs(0);
        for time_diffs in all_diffs {
            warn!("{:?}", time_diffs);
            time_diff_max = time_diffs.iter().fold(time_diff_max, |acc, cur| max(acc, *cur));
            time_sum_diff = time_diffs.iter().fold(time_sum_diff, |acc, cur| acc + *cur);
        }

        warn!("================= Settings =================\n\
        Agent cnt:               \t{:?}\n\
        Operations per agent cnt:\t{:?}\n\
        Data size:               \t{:?}",
              agent_cnt, operations_cnt, data_sz);

        warn!("================= Summary =================\n\
        Max pending:   \t{:?}\n\
        Total ops cnt: \t{:?}\n\
        Sum pending:   \t{:?}\n\
        Total duration:\t{:?}",
              time_diff_max, agent_cnt * operations_cnt, time_sum_diff, total_duration);

        TestUtils::cleanup_storage();
    }
}
