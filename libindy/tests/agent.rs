extern crate indy;
extern crate base64;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate zmq_pw as zmq;

use indy::api::ErrorCode;

#[macro_use]
mod utils;

use utils::agent::AgentUtils;
use utils::pool::PoolUtils;
use utils::crypto::CryptoUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;
use utils::wallet::WalletUtils;
use utils::constants::*;


mod high_cases {
    use super::*;

    mod prep_msg {
        use super::*;

        fn check_message(sender_vk: &str, encrypted_msg: &Vec<u8>) {
            let recipient_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (recipient_did, _) = SignusUtils::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let decrypted_message = SignusUtils::decrypt_sealed(recipient_wallet_handle, &recipient_did, encrypted_msg).unwrap();
            let decrypted_msg_json = std::str::from_utf8(&decrypted_message).unwrap();
            let decrypted_msg: serde_json::Value = serde_json::from_str(decrypted_msg_json).unwrap();

            assert_eq!(true, decrypted_msg["auth"].as_bool().unwrap());
            assert_eq!(sender_vk, decrypted_msg["sender"].as_str().unwrap());
            decrypted_msg["nonce"].as_str().unwrap();
            decrypted_msg["msg"].as_str().unwrap();

            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();
        }

        #[test]
        fn indy_prep_msg_works_for_created_key() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let sender_vk = CryptoUtils::create_key(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_msg(sender_wallet_handle, &sender_vk, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();
            check_message(&sender_vk, &encrypted_msg);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_msg_works_for_created_did() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (_, sender_vk) = SignusUtils::create_and_store_my_did(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_msg(sender_wallet_handle, &sender_vk, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();
            check_message(&sender_vk, &encrypted_msg);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_msg_works_for_created_did_as_cid() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (_, sender_vk) = SignusUtils::create_my_did(sender_wallet_handle, &format!(r#"{{"seed":"{}", "cid":true}}"#, MY1_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_msg(sender_wallet_handle, &sender_vk, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();
            check_message(&sender_vk, &encrypted_msg);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_msg_works_for_unknown_sender_verkey() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = AgentUtils::prep_msg(wallet_handle, VERKEY_MY2, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::KeyNotFoundInWalletError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_msg_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let sender_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AgentUtils::prep_msg(invalid_wallet_handle, &sender_vk, VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_msg_works_for_invalid_recipient_vk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let sender_vk = CryptoUtils::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let res = AgentUtils::prep_msg(wallet_handle, &sender_vk, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod prep_anonymous_msg {
        use super::*;
        use base64;

        fn check_message(encrypted_msg: &Vec<u8>) {
            let recipient_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (recipient_did, _) = SignusUtils::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let decrypted_message = SignusUtils::decrypt_sealed(recipient_wallet_handle, &recipient_did, encrypted_msg).unwrap();
            let decrypted_msg_json = std::str::from_utf8(&decrypted_message).unwrap();
            let decrypted_msg: serde_json::Value = serde_json::from_str(decrypted_msg_json).unwrap();

            assert_eq!(false, decrypted_msg["auth"].as_bool().unwrap());
            assert_eq!(MESSAGE.as_bytes().to_vec(), base64::decode(decrypted_msg["msg"].as_str().unwrap()).unwrap());

            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();
        }

        #[test]
        fn indy_prep_anonymous_msg_works() {
            TestUtils::cleanup_storage();

            let encrypted_msg = AgentUtils::prep_anonymous_msg(VERKEY_MY2, &MESSAGE.as_bytes()).unwrap();
            check_message(&encrypted_msg);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_prep_anonymous_msg_works_for_invalid_recipient_vk() {
            TestUtils::cleanup_storage();

            let res = AgentUtils::prep_anonymous_msg(INVALID_VERKEY_LENGTH, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            let res = AgentUtils::prep_anonymous_msg(INVALID_BASE58_VERKEY, &MESSAGE.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            TestUtils::cleanup_storage();
        }
    }

    mod parse_msg {
        use super::*;

        #[test]
        fn indy_parse_msg_works_for_authenticated_message() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let sender_vk = CryptoUtils::create_key(sender_wallet_handle, Some(MY1_SEED)).unwrap();
            let recipient_vk = CryptoUtils::create_key(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_msg(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (vk, msg) = AgentUtils::parse_msg(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(sender_vk, vk.unwrap());

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_parse_msg_works_for_anonymous_message() {
            TestUtils::cleanup_storage();

            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let recipient_vk = CryptoUtils::create_key(recipient_wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_anonymous_msg(&recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (sender_vk, msg) = AgentUtils::parse_msg(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(None, sender_vk);

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_parse_msg_works_for_invalid_authenticated_msg() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            let sender_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let recipient_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let (recipient_did, recipient_vk) = SignusUtils::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();
            SignusUtils::store_their_did_from_parts(sender_wallet_handle, &recipient_did, &recipient_vk).unwrap();

            let msg = format!(r#"{{"auth":true,"nonce":"Th7MpTaRZVRYnPiabds81Y12","sender":"{:?}","msg":"unencrypted message"}}"#, VERKEY);
            let encrypted_msg = SignusUtils::encrypt_sealed(sender_wallet_handle, pool_handle, &recipient_did, msg.as_bytes()).unwrap();

            let res = AgentUtils::parse_msg(recipient_wallet_handle, &recipient_vk, &encrypted_msg);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(sender_wallet_handle).unwrap();
            WalletUtils::close_wallet(recipient_wallet_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_parse_msg_works_for_invalid_anonymous_msg() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let recipient_vk = CryptoUtils::create_key(wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = "unencrypted message";
            let res = AgentUtils::parse_msg(wallet_handle, &recipient_vk, &encrypted_msg.as_bytes());
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_parse_msg_works_for_unknown_recipient_vk() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let encrypted_msg = AgentUtils::prep_anonymous_msg(VERKEY, &MESSAGE.as_bytes()).unwrap();

            let res = AgentUtils::parse_msg(wallet_handle, VERKEY, &encrypted_msg);
            assert_eq!(ErrorCode::KeyNotFoundInWalletError, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_parse_msg_msg_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let recipient_vk = CryptoUtils::create_key(wallet_handle, Some(MY2_SEED)).unwrap();

            let encrypted_msg = AgentUtils::prep_anonymous_msg(&recipient_vk, &MESSAGE.as_bytes()).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AgentUtils::parse_msg(invalid_wallet_handle, &recipient_vk, &encrypted_msg);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
