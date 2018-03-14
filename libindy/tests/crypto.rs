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
use utils::crypto::CryptoUtils;
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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();
            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, new_metadata).unwrap();
            let updated_metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::set_key_metadata(wallet_handle + 1, VERKEY, METADATA);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, "").unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, "").unwrap();

            let metadata = CryptoUtils::get_key_metadata(wallet_handle, VERKEY).unwrap();
            assert_eq!("", metadata);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::get_key_metadata(wallet_handle, VERKEY);
            assert_eq!(ErrorCode::KeyNotFoundInWalletError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            CryptoUtils::set_key_metadata(wallet_handle, VERKEY, METADATA).unwrap();

            let res = CryptoUtils::get_key_metadata(wallet_handle + 1, VERKEY);
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

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = CryptoUtils::crypto_sign(wallet_handle, &my_vk, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_sign_works_for_unknow_signer() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::crypto_sign(wallet_handle, VERKEY, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::KeyNotFoundInWalletError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_sign_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, None).unwrap();

            let res = CryptoUtils::crypto_sign(wallet_handle + 1, &my_vk, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_verify {
        use super::*;

        #[test]
        fn indy_crypto_verify_works() {
            let valid = CryptoUtils::crypto_verify(&VERKEY_MY1, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_correct_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":ed25519";
            let valid = CryptoUtils::crypto_verify(&verkey, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_invalid_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":unknown_crypto";
            let res = CryptoUtils::crypto_verify(&verkey, MESSAGE.as_bytes(), SIGNATURE);
            assert_eq!(ErrorCode::SignusUnknownCryptoError, res.unwrap_err());
        }


        #[test]
        fn indy_crypto_verify_works_for_other_signer() {
            let valid = CryptoUtils::crypto_verify(&VERKEY_MY2, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(!valid);
        }
    }

    mod crypto_box {
        use super::*;

        #[test]
        fn indy_crypto_box_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            CryptoUtils::crypto_box(wallet_handle, &my_vk, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_works_for_unknown_coder() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::crypto_box(wallet_handle, VERKEY_MY1, VERKEY_MY2, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::KeyNotFoundInWalletError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box(wallet_handle + 1, &my_vk, VERKEY_MY2, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_box_open {
        use super::*;

        #[test]
        fn indy_crypto_box_open_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let decrypted_message = CryptoUtils::crypto_box_open(wallet_handle, &my_vk, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, NONCE).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), decrypted_message);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_unknown_my_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::crypto_box_open(wallet_handle, VERKEY_MY1, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::KeyNotFoundInWalletError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_other_coder_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box_open(wallet_handle, &my_vk, VERKEY_MY2, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_nonce_not_correspond_message() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let nonce = "acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7";
            let res = CryptoUtils::crypto_box_open(wallet_handle, &my_vk, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, nonce.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box_open(wallet_handle + 1, &my_vk, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_box_seal {
        use super::*;

        #[test]
        fn indy_crypto_box_seal_works() {
            TestUtils::cleanup_storage();

            CryptoUtils::crypto_box_seal(VERKEY_MY1, MESSAGE.as_bytes()).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_seal_works_for_invalid_key() {
            TestUtils::cleanup_storage();

            let res = CryptoUtils::crypto_box_seal(INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_box_seal_open {
        use super::*;

        #[test]
        fn indy_crypto_box_seal_open_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let encrypted_message = CryptoUtils::crypto_box_seal(&verkey, MESSAGE.as_bytes()).unwrap();

            let decrypted_message = CryptoUtils::crypto_box_seal_open(wallet_handle, &verkey, &encrypted_message).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), decrypted_message);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_seal_open_works_for_other_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let encrypted_message = CryptoUtils::crypto_box_seal(VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box_seal_open(wallet_handle, &verkey, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_seal_open_works_for_unknown_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let encrypted_message = CryptoUtils::crypto_box_seal(VERKEY_MY1, MESSAGE.as_bytes()).unwrap();
            let res = CryptoUtils::crypto_box_seal_open(wallet_handle, VERKEY_MY1, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::KeyNotFoundInWalletError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_seal_open_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let verkey = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let encrypted_message = CryptoUtils::crypto_box_seal(&verkey, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::crypto_box_seal_open(wallet_handle + 1, &verkey, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod crypto_verify {
        use super::*;

        #[test]
        fn indy_crypto_verify_works_for_invalid_signature_len() {
            let signature: Vec<u8> = vec![20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190];
            let res = CryptoUtils::crypto_verify(&VERKEY_MY1, MESSAGE.as_bytes(), &signature);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());
        }
    }

    mod crypto_box {
        use super::*;

        #[test]
        fn indy_crypto_box_works_for_invalid_my_pk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::crypto_box(wallet_handle, INVALID_BASE58_VERKEY, VERKEY_MY2, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_works_for_invalid_their_pk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box(wallet_handle, &my_vk, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod crypto_box_open {
        use super::*;


        #[test]
        fn indy_crypto_box_open_works_for_invalid_my_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = CryptoUtils::crypto_box_open(wallet_handle, INVALID_BASE58_VERKEY, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_invalid_their_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = CryptoUtils::crypto_box_open(wallet_handle, &my_vk, INVALID_BASE58_VERKEY, ENCRYPTED_MESSAGE, NONCE);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_crypto_box_open_works_for_invalid_nonce() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let my_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let nonce = vec![24, 99, 107, 70, 58, 6, 252, 149, 225];
            let res = CryptoUtils::crypto_box_open(wallet_handle, &my_vk, VERKEY_TRUSTEE, ENCRYPTED_MESSAGE, &nonce);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }


    mod crypto_box_seal_open {
        use super::*;

        #[test]
        fn indy_crypto_box_seal_open_works_for_invalid_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let encrypted_message = CryptoUtils::crypto_box_seal(VERKEY_MY1, MESSAGE.as_bytes()).unwrap();

            let res = CryptoUtils::crypto_box_seal_open(wallet_handle, INVALID_BASE58_VERKEY, &encrypted_message);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}