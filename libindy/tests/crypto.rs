#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

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
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::{wallet, crypto, did};
use utils::constants::*;

use self::indy::ErrorCode;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11];

fn setup_with_key() -> (i32, String) {
    let wallet_handle = utils::setup_with_wallet();
    let verkey = crypto::create_key(wallet_handle, None).unwrap();
    (wallet_handle, verkey)
}

mod high_cases {
    use super::*;

    mod create_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_key_works_for_seed() {
            let wallet_handle = utils::setup_with_wallet();

            let verkey = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            let wallet_handle = utils::setup_with_wallet();

            let verkey = crypto::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::create_key(wallet_handle + 1, None);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod set_key_metadata {
        use super::*;

        #[test]
        fn indy_set_key_metadata_works() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();
            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            crypto::set_key_metadata(wallet_handle, &verkey, new_metadata).unwrap();
            let updated_metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::set_key_metadata(wallet_handle + 1, &verkey, METADATA);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::set_key_metadata(wallet_handle, INVALID_BASE58_VERKEY, METADATA);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod get_key_metadata {
        use super::*;

        #[test]
        fn indy_get_key_metadata_works() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!("", metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::get_key_metadata(wallet_handle, &verkey);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let res = crypto::get_key_metadata(wallet_handle + 1, &verkey);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod crypto_sign {
        use super::*;

        #[test]
        fn indy_crypto_sign_works() {
            let wallet_handle = utils::setup_with_wallet();

            let my_vk = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = crypto::sign(wallet_handle, &my_vk, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_sign_works_for_unknown_signer() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::sign(wallet_handle, VERKEY, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletItemNotFound);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_sign_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::sign(wallet_handle + 1, &verkey, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod crypto_verify {
        use super::*;

        #[test]
        fn indy_crypto_verify_works() {
            let valid = crypto::verify(&VERKEY_MY1, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_correct_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":ed25519";
            let valid = crypto::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_invalid_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":unknown_crypto";
            let res = crypto::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(ErrorCode::UnknownCryptoTypeError, res.unwrap_err());
        }


        #[test]
        fn indy_crypto_verify_works_for_other_signer() {
            let valid = crypto::verify(&VERKEY_MY2, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(!valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_invalid_signature_len() {
            let signature: Vec<u8> = vec![20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190];
            let res = crypto::verify(&VERKEY_MY1, MESSAGE.as_bytes(), &signature);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());
        }
    }

    mod auth_crypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_key() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::auth_crypt(wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did() {
            let sender_wallet_handle = utils::setup_with_wallet();

            let (_, verkey) = did::create_and_store_my_did(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did_as_cid() {
            let sender_wallet_handle = utils::setup_with_wallet();

            let (_, verkey) = did::create_my_did(sender_wallet_handle, &json!({ "seed": MY1_SEED, "cid": true }).to_string()).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_unknown_sender_verkey() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::auth_crypt(wallet_handle, VERKEY_MY2, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::auth_crypt(wallet_handle + 1, &verkey, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_recipient_vk() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::auth_crypt(wallet_handle, &verkey, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod auth_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_decrypt_works() {
            let (sender_wallet_handle, sender_vk) = setup_with_key();

            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (vk, msg) = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(sender_vk, vk);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_invalid_msg() {
            let sender_wallet_handle = utils::setup_with_wallet();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();

            let (recipient_did, recipient_vk) = did::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();
            did::store_their_did_from_parts(sender_wallet_handle, &recipient_did, &recipient_vk).unwrap();

            let encrypted_msg = format!(r#"{{"nonce":"Th7MpTaRZVRYnPiabds81Y12","sender":"{:?}","msg":"{:?}"}}"#, VERKEY, ENCRYPTED_MESSAGE.to_vec());

            let res = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_unknown_recipient_vk() {
            let (wallet_handle, sender_vk) = setup_with_key();

            let encrypted_msg = crypto::auth_crypt(wallet_handle, &sender_vk, &VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_invalid_handle() {
            let (sender_wallet_handle, sender_vk) = setup_with_key();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let res = crypto::auth_decrypt(recipient_wallet_handle + 1, &recipient_vk, &encrypted_msg);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }
    }

    mod anon_crypt {
        use super::*;

        #[test]
        fn indy_anon_crypt_works() {
            utils::setup();

            crypto::anon_crypt(VERKEY_MY2, &MESSAGE.as_bytes()).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_anon_crypt_works_for_invalid_their_vk() {
            utils::setup();

            let res = crypto::anon_crypt(INVALID_VERKEY_LENGTH, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            let res = crypto::anon_crypt(INVALID_BASE58_VERKEY, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down();
        }
    }

    mod anon_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_anon_decrypt_works() {
            let (sender_wallet_handle, _) = setup_with_key();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::anon_crypt(&recipient_vk, MESSAGE.as_bytes()).unwrap();

            let msg = crypto::anon_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_invalid_msg() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::anon_decrypt(wallet_handle, &verkey, &"unencrypted message".as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_unknown_verkey() {
            let wallet_handle = utils::setup_with_wallet();

            let encrypted_msg = crypto::anon_crypt(&VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let encrypted_msg = crypto::anon_crypt(&verkey, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle + 1, &verkey, &encrypted_msg);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
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
        utils::setup();

        let agent_cnt = std::env::var("AGENTS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(AGENT_CNT);
        let data_sz = std::env::var("DATA_SZ").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(DATA_SZ);
        let operations_cnt = std::env::var("OPERATIONS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(OPERATIONS_CNT);

        let mut agents = Vec::new();
        let mut os_rng = OsRng::new().unwrap();
        for _ in 0..agent_cnt {
            let wallet = wallet::create_and_open_default_wallet().unwrap();
            let (_did, verkey) = did::create_and_store_my_did(wallet, None).unwrap();
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
                    let _encrypted = crypto::auth_crypt(wallet, &verkey, &verkey, data.as_slice()).unwrap();
                    let time_diff = SystemTime::now().duration_since(time).unwrap();
                    time_diffs.push(time_diff);
                }

                wallet::close_wallet(wallet).unwrap();
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

        utils::tear_down();
    }
}
