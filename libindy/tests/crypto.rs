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
extern crate ursa;
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

use api::INVALID_WALLET_HANDLE;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11];

fn setup_with_key(name: &str) -> (i32, String, String) {
    let (wallet_handle, wallet_config) = utils::setup_with_wallet(name);
    let verkey = crypto::create_key(wallet_handle, None).unwrap();
    (wallet_handle, verkey, wallet_config)
}

mod high_cases {
    use super::*;

    mod create_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_key_works_for_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_key_works_for_seed");

            let verkey = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_key_works_for_seed", &wallet_config);
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_key_works_without_seed");

            let verkey = crypto::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_key_works_without_seed", &wallet_config);
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_create_key_works_for_invalid_wallet_handle");

            let res = crypto::create_key(INVALID_WALLET_HANDLE, None);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_create_key_works_for_invalid_wallet_handle", &wallet_config);
        }
    }

    mod set_key_metadata {
        use super::*;

        #[test]
        fn indy_set_key_metadata_works() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_set_key_metadata_works");

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_key_metadata_works", &wallet_config);
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_set_key_metadata_works_for_replace");

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();
            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            crypto::set_key_metadata(wallet_handle, &verkey, new_metadata).unwrap();
            let updated_metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_key_metadata_works_for_replace", &wallet_config);
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_set_key_metadata_works_for_invalid_handle");

            let res = crypto::set_key_metadata(INVALID_WALLET_HANDLE, &verkey, METADATA);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_key_metadata_works_for_invalid_handle", &wallet_config);
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_set_key_metadata_works_for_empty_string");

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_set_key_metadata_works_for_empty_string", &wallet_config);
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_set_key_metadata_works_for_invalid_key");

            let res = crypto::set_key_metadata(wallet_handle, INVALID_BASE58_VERKEY, METADATA);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_set_key_metadata_works_for_invalid_key", &wallet_config);
        }
    }

    mod get_key_metadata {
        use super::*;

        #[test]
        fn indy_get_key_metadata_works() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_get_key_metadata_works");

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_key_metadata_works", &wallet_config);
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_get_key_metadata_works_for_empty_string");

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!("", metadata);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_key_metadata_works_for_empty_string", &wallet_config);
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_get_key_metadata_works_for_no_metadata");

            let res = crypto::get_key_metadata(wallet_handle, &verkey);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_key_metadata_works_for_no_metadata", &wallet_config);
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_get_key_metadata_works_for_invalid_handle");

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let res = crypto::get_key_metadata(INVALID_WALLET_HANDLE, &verkey);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_get_key_metadata_works_for_invalid_handle", &wallet_config);
        }
    }

    mod crypto_sign {
        use super::*;

        #[test]
        fn indy_crypto_sign_works() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_crypto_sign_works");

            let my_vk = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = crypto::sign(wallet_handle, &my_vk, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_sign_works", &wallet_config);
        }

        #[test]
        fn indy_crypto_sign_works_for_unknown_signer() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_crypto_sign_works_for_unknown_signer");

            let res = crypto::sign(wallet_handle, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_sign_works_for_unknown_signer", &wallet_config);
        }

        #[test]
        fn indy_crypto_sign_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_sign_works_for_invalid_wallet_handle");

            let res = crypto::sign(INVALID_WALLET_HANDLE, &verkey, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_sign_works_for_invalid_wallet_handle", &wallet_config);
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
            assert_code!(ErrorCode::UnknownCryptoTypeError, res);
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
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod auth_crypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_key() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_auth_crypt_works_for_created_key");

            crypto::auth_crypt(wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_auth_crypt_works_for_created_key", &wallet_config);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did() {
            let (sender_wallet_handle, wallet_config) = utils::setup_with_wallet("indy_crypto_auth_crypt_works_for_created_did");

            let (_, verkey) = did::create_and_store_my_did(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle, "indy_crypto_auth_crypt_works_for_created_did", &wallet_config);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did_as_cid() {
            let (sender_wallet_handle, sender_wallet_config) = utils::setup_with_wallet("indy_crypto_auth_crypt_works_for_created_did_as_cid");

            let (_, verkey) = did::create_my_did(sender_wallet_handle, &json!({ "seed": MY1_SEED, "cid": true }).to_string()).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle, "indy_crypto_auth_crypt_works_for_created_did_as_cid", &sender_wallet_config);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_unknown_sender_verkey() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_crypto_auth_crypt_works_for_unknown_sender_verkey");

            let res = crypto::auth_crypt(wallet_handle, VERKEY_MY2, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_auth_crypt_works_for_unknown_sender_verkey", &wallet_config);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_auth_crypt_works_for_invalid_wallet_handle");

            let res = crypto::auth_crypt(INVALID_WALLET_HANDLE, &verkey, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_auth_crypt_works_for_invalid_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_recipient_vk() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_auth_crypt_works_for_invalid_recipient_vk");

            let res = crypto::auth_crypt(wallet_handle, &verkey, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_auth_crypt_works_for_invalid_recipient_vk", &wallet_config);
        }
    }

    mod auth_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_decrypt_works() {
            let (sender_wallet_handle, sender_vk, sender_wallet_config) = setup_with_key("indy_crypto_auth_decrypt_works");

            let (recipient_wallet_handle, recipient_config) = wallet::create_and_open_default_wallet("indy_crypto_auth_decrypt_works").unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (vk, msg) = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(sender_vk, vk);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            wallet::delete_wallet(&recipient_config, WALLET_CREDENTIALS).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle, "indy_crypto_auth_decrypt_works", &sender_wallet_config);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_invalid_msg() {
            let (sender_wallet_handle, sender_wallet_config) = utils::setup_with_wallet("indy_crypto_auth_decrypt_works_for_invalid_msg");
            let (recipient_wallet_handle, recipient_wallet_config) = wallet::create_and_open_default_wallet("indy_crypto_auth_decrypt_works_for_invalid_msg").unwrap();

            let (recipient_did, recipient_vk) = did::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();
            did::store_their_did_from_parts(sender_wallet_handle, &recipient_did, &recipient_vk).unwrap();

            let encrypted_msg = format!(r#"{{"nonce":"Th7MpTaRZVRYnPiabds81Y12","sender":"{:?}","msg":"{:?}"}}"#, VERKEY, ENCRYPTED_MESSAGE.to_vec());

            let res = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            wallet::delete_wallet(&recipient_wallet_config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(sender_wallet_handle).unwrap();
            wallet::delete_wallet(&sender_wallet_config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_unknown_recipient_vk() {
            let (wallet_handle, sender_vk, wallet_config) = setup_with_key("indy_crypto_auth_decrypt_works_for_unknown_recipient_vk");

            let encrypted_msg = crypto::auth_crypt(wallet_handle, &sender_vk, &VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_auth_decrypt_works_for_unknown_recipient_vk", &wallet_config);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_invalid_handle() {
            let (sender_wallet_handle, sender_vk, sender_wallet_config) = setup_with_key("indy_crypto_auth_decrypt_works_invalid_handle");
            let (recipient_wallet_handle, recipient_wallet_config) = wallet::create_and_open_default_wallet("indy_crypto_auth_decrypt_works_invalid_handle").unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let res = crypto::auth_decrypt(recipient_wallet_handle + 1, &recipient_vk, &encrypted_msg);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            wallet::close_and_delete_wallet(recipient_wallet_handle, &recipient_wallet_config).unwrap();
            wallet::close_and_delete_wallet(sender_wallet_handle, &sender_wallet_config).unwrap();
        }
    }

    mod anon_crypt {
        use super::*;

        #[test]
        fn indy_anon_crypt_works() {
            utils::setup("indy_anon_crypt_works");

            crypto::anon_crypt(VERKEY_MY2, &MESSAGE.as_bytes()).unwrap();

            utils::tear_down("indy_anon_crypt_works");
        }

        #[test]
        fn indy_anon_crypt_works_for_invalid_their_vk() {
            utils::setup("indy_anon_crypt_works_for_invalid_their_vk");

            let res = crypto::anon_crypt(INVALID_VERKEY_LENGTH, &MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            let res = crypto::anon_crypt(INVALID_BASE58_VERKEY, &MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down("indy_anon_crypt_works_for_invalid_their_vk");
        }
    }

    mod anon_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_anon_decrypt_works() {
            let (sender_wallet_handle, _, wallet_config) = setup_with_key("indy_crypto_anon_decrypt_works");
            let (recipient_wallet_handle, recipient_wallet_config) = wallet::create_and_open_default_wallet("indy_crypto_anon_decrypt_works").unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::anon_crypt(&recipient_vk, MESSAGE.as_bytes()).unwrap();

            let msg = crypto::anon_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);

            wallet::close_wallet(sender_wallet_handle).unwrap();
            wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(recipient_wallet_handle).unwrap();
            wallet::delete_wallet(&recipient_wallet_config, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_invalid_msg() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_anon_decrypt_works_for_invalid_msg");

            let res = crypto::anon_decrypt(wallet_handle, &verkey, &"unencrypted message".as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_anon_decrypt_works_for_invalid_msg", &wallet_config);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_unknown_verkey() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_crypto_anon_decrypt_works_for_unknown_verkey");

            let encrypted_msg = crypto::anon_crypt(&VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_anon_decrypt_works_for_unknown_verkey", &wallet_config);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_invalid_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_crypto_anon_decrypt_works_invalid_handle");

            let encrypted_msg = crypto::anon_crypt(&verkey, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(INVALID_WALLET_HANDLE, &verkey, &encrypted_msg);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle, "indy_crypto_anon_decrypt_works_invalid_handle", &wallet_config);
        }
    }

    mod pack_message_authcrypt {
        use super::*;

        #[test]
        fn indy_pack_message_authcrypt_works() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_pack_message_authcrypt_works");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert!(res.is_ok());
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_authcrypt_works", &wallet_config);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_empty_message() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_pack_message_authcrypt_fails_empty_message");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::CommonInvalidParam3, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_authcrypt_fails_empty_message", &wallet_config);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_no_receivers() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_pack_message_authcrypt_fails_no_receivers");
            let receiver_keys = "[]";
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::CommonInvalidParam4, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_authcrypt_fails_no_receivers", &wallet_config);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_bad_wallet_handle() {
            let (wallet_handle, verkey, wallet_config) = setup_with_key("indy_pack_message_authcrypt_fails_bad_wallet_handle");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(INVALID_WALLET_HANDLE, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::WalletInvalidHandle, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_authcrypt_fails_bad_wallet_handle", &wallet_config);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_invalid_verkey() {
            let (wallet_handle, _, wallet_config) = setup_with_key("indy_pack_message_authcrypt_fails_invalid_verkey");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(INVALID_BASE58_VERKEY));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_authcrypt_fails_invalid_verkey", &wallet_config);
        }

    }

    mod pack_message_anoncrypt {
        use super::*;

        #[test]
        fn indy_pack_message_anon_works() {
            let (wallet_handle, _ , wallet_config) = setup_with_key("indy_pack_message_anon_works");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert!(res.is_ok());
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_anon_works", &wallet_config);
        }

        #[test]
        fn indy_pack_message_anoncrypt_fails_empty_message() {
            let (wallet_handle, _, wallet_config ) = setup_with_key("indy_pack_message_anoncrypt_fails_empty_message");
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert_code!(ErrorCode::CommonInvalidParam3, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_anoncrypt_fails_empty_message", &wallet_config);
        }

        #[test]
        fn indy_pack_message_anoncrypt_fails_no_receivers() {
            let (wallet_handle, _ , wallet_config) = setup_with_key("indy_pack_message_anoncrypt_fails_no_receivers");
            let receiver_keys = "[]";
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert_code!(ErrorCode::CommonInvalidParam4, res);
            utils::tear_down_with_wallet(wallet_handle, "indy_pack_message_anoncrypt_fails_no_receivers", &wallet_config);
        }

        #[test]
        fn indy_pack_message_anoncrypt_passes_bad_wallet_handle() {
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            //The wallet_handle and sender aren't used in this case, so any wallet_handle whether inited or not will work
            let res = crypto::pack_message(1, message, &receiver_keys, None);
            assert!(res.is_ok());
        }

    }

    mod unpack_message_authcrypt {
        use super::*;

        #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
        pub struct UnpackMessage {
            pub message: String,
            pub sender_verkey: String,
            pub recipient_verkey: String
        }

        #[test]
        fn indy_unpack_message_authcrypt_works() {
            //Test setup
            let (wallet_handle_sender, sender_verkey, wallet_config_sender) = setup_with_key("indy_unpack_message_authcrypt_works");
            let (wallet_handle_receiver, wallet_config_receiver) = wallet::create_and_open_default_wallet("indy_unpack_message_authcrypt_works").unwrap();
            let receiver_verkey = crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE, &receiver_verkey];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let pack_message = crypto::pack_message(wallet_handle_sender, AGENT_MESSAGE.as_bytes(), &receiver_keys, Some(&sender_verkey)).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice()).unwrap();
            let res_serialized : UnpackMessage = serde_json::from_slice(res.as_slice()).unwrap();

            //verify unpack ran correctly
            assert_eq!(res_serialized.message, AGENT_MESSAGE.to_string());
            assert_eq!(res_serialized.sender_verkey, sender_verkey);
            assert_eq!(res_serialized.recipient_verkey, receiver_verkey);

            //teardown
            wallet::close_wallet(wallet_handle_sender).unwrap();
            wallet::delete_wallet(&wallet_config_sender, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle_receiver).unwrap();
            wallet::delete_wallet(&wallet_config_receiver, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_unpack_message_authcrypt_fails_no_matching_key() {
            //Test Setup
            let (wallet_handle_sender, sender_verkey, wallet_config_sender) = setup_with_key("indy_unpack_message_authcrypt_fails_no_matching_key");
            let (wallet_handle_receiver, wallet_config_receiver) = wallet::create_and_open_default_wallet("indy_unpack_message_authcrypt_fails_no_matching_key").unwrap();
            crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let pack_message = crypto::pack_message(wallet_handle_sender, message, &receiver_keys, Some(&sender_verkey)).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice());

            assert_code!(ErrorCode::WalletItemNotFound, res);

            //teardown
            wallet::close_wallet(wallet_handle_sender).unwrap();
            wallet::delete_wallet(&wallet_config_sender, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle_receiver).unwrap();
            wallet::delete_wallet(&wallet_config_receiver, WALLET_CREDENTIALS).unwrap();
        }

    }

    mod unpack_message_anoncrypt {
        use super::*;

        #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
        pub struct UnpackMessage {
            pub message: String,
            pub recipient_verkey: String
        }

        #[test]
        fn indy_unpack_message_anoncrypt_works() {
            let (wallet_handle_sender, _, wallet_config_sender) = setup_with_key("indy_unpack_message_anoncrypt_works");
            let (wallet_handle_receiver, wallet_config_receiver) = wallet::create_and_open_default_wallet("indy_unpack_message_anoncrypt_works").unwrap();
            let receiver_verkey = crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE, &receiver_verkey];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let pack_message = crypto::pack_message(wallet_handle_sender, AGENT_MESSAGE.as_bytes(), &receiver_keys, None).unwrap();
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice()).unwrap();
            let res_serialized : UnpackMessage = serde_json::from_slice(res.as_slice()).unwrap();

            assert_eq!(res_serialized.message, AGENT_MESSAGE.to_string());
            assert_eq!(res_serialized.recipient_verkey, receiver_verkey);

            //teardown
            wallet::close_wallet(wallet_handle_sender).unwrap();
            wallet::delete_wallet(&wallet_config_sender, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle_receiver).unwrap();
            wallet::delete_wallet(&wallet_config_receiver, WALLET_CREDENTIALS).unwrap();
        }

        #[test]
        fn indy_unpack_message_anoncrypt_fails_no_matching_key() {
            //Test Setup
            let (wallet_handle_sender, _, wallet_config_sender) = setup_with_key("indy_unpack_message_anoncrypt_fails_no_matching_key");
            let (wallet_handle_receiver, wallet_config_receiver) = wallet::create_and_open_default_wallet("indy_unpack_message_anoncrypt_fails_no_matching_key").unwrap();
            crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let pack_message = crypto::pack_message(wallet_handle_sender, message, &receiver_keys, None).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice());

            assert_code!(ErrorCode::WalletItemNotFound, res);

            //teardown
            wallet::close_wallet(wallet_handle_sender).unwrap();
            wallet::delete_wallet(&wallet_config_sender, WALLET_CREDENTIALS).unwrap();
            wallet::close_wallet(wallet_handle_receiver).unwrap();
            wallet::delete_wallet(&wallet_config_receiver, WALLET_CREDENTIALS).unwrap();
        }

    }
}

mod load {
    extern crate rand;

    use super::*;

    use self::rand::RngCore;
    use self::rand::rngs::OsRng;

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
        utils::setup("parallel_auth_encrypt");

        let agent_cnt = std::env::var("AGENTS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(AGENT_CNT);
        let data_sz = std::env::var("DATA_SZ").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(DATA_SZ);
        let operations_cnt = std::env::var("OPERATIONS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(OPERATIONS_CNT);

        let mut agents = Vec::new();
        let mut os_rng = OsRng::new().unwrap();
        for i in 0..agent_cnt {
            let (wallet, wallet_config) = wallet::create_and_open_default_wallet(&format!("parallel_auth_encrypt-{}", i)).unwrap();
            let (_did, verkey) = did::create_and_store_my_did(wallet, None).unwrap();
            let mut data = vec![0u8; data_sz];
            os_rng.fill_bytes(&mut data.as_mut_slice());
            agents.push((wallet, verkey, data, wallet_config));
        }

        let start_time = SystemTime::now();

        let mut results = Vec::new();

        for (wallet, verkey, data, wallet_config) in agents {
            let thread = thread::spawn(move || {
                let mut time_diffs = Vec::new();
                for _ in 0..operations_cnt {
                    let time = SystemTime::now();
                    let _encrypted = crypto::auth_crypt(wallet, &verkey, &verkey, data.as_slice()).unwrap();
                    let time_diff = SystemTime::now().duration_since(time).unwrap();
                    time_diffs.push(time_diff);
                }

                wallet::close_wallet(wallet).unwrap();
                wallet::delete_wallet(&wallet_config, WALLET_CREDENTIALS).unwrap();
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

        utils::tear_down("parallel_auth_encrypt");
    }
}
