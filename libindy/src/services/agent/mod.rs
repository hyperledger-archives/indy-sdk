use core::result;
use domain::crypto::key::Key;
use domain::agent::*;
use errors::agent::AgentError;
use serde_json;
use services::crypto::CryptoService;
use services::wallet::{RecordOptions, WalletService};
use std::rc::Rc;
use utils::crypto::base58;
use utils::crypto::base64;
use utils::crypto::xsalsa20;
use utils::crypto::xsalsa20::{create_key, gen_nonce};

type Result<T> = result::Result<T, AgentError>;

pub struct AgentService {}

impl AgentService {
    pub fn new() -> AgentService {
        AgentService {}
    }

    pub fn get_recipient_header(
        &self,
        recipient_verkey: &str,
        recipients_list: Vec<Recipient>,
    ) -> Result<Recipient> {
        for recipient in recipients_list {
            if &recipient.header.kid == recipient_verkey {
                return Ok(recipient);
            }
        }

        Err(AgentError::UnpackError(format!(
            "Failed to find a matching header"
        )))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use domain::agent::{Recipient, Header};

    #[test]
    pub fn test_get_recipient_header_works(){
        let service = AgentService::new();

        let verkey = "key_of_recipient1".to_string();

        let recp1 : Recipient = Recipient {
            encrypted_key: "unrelevant data".to_string(),
            header: Header {
                sender : "unrelevant for this test".to_string(),
                kid : "key_of_recipient1".to_string(),
            }
        };

        let recp2 : Recipient = Recipient {
            encrypted_key: "unrelevant data".to_string(),
            header: Header {
                sender : "unrelevant for this test".to_string(),
                kid : "key_of_recipient2".to_string(),
            }
        };

        let test_recp_list : Vec<Recipient> = vec![recp1, recp2];

        let returned_recipient = service.get_recipient_header(&verkey.clone(), test_recp_list).unwrap();

        assert_eq!(returned_recipient.header.kid, verkey);
    }

    #[test]
    pub fn test_get_recipient_header_unpack_error(){
        let service = AgentService::new();

        let verkey = "key_of_recipient1".to_string();

        let recp1 : Recipient = Recipient {
            encrypted_key: "unrelevant data".to_string(),
            header: Header {
                sender : "unrelevant for this test".to_string(),
                kid : "key_does_not_match1".to_string(),
            }
        };

        let recp2 : Recipient = Recipient {
            encrypted_key: "unrelevant data".to_string(),
            header: Header {
                sender : "unrelevant for this test".to_string(),
                kid : "key_does_not_match2".to_string(),
            }
        };

        let test_recp_list : Vec<Recipient> = vec![recp1, recp2];

        let returned_recipient = service.get_recipient_header(&verkey.clone(), test_recp_list);

        assert!(returned_recipient.is_err());
    }
}

//pub mod tests {
//
//    // TODO Fix texts so only one wallet is used to speed up tests
//    //unit tests
//
//
//    /* component test useful to identify if unpack is breaking or if pack is breaking. If unpack is
//     * breaking both this test and the tests below will fail. If only pack is breaking, only this test
//     * will fail.
//     */
//
//    // Integration tests
//
//    pub fn test_single_anon_pack_message_and_unpack_msg_success() {
//        _cleanup();
//        //setup generic data to test
//        let expected_message = "Hello World";
//
//        //setup route_service
//        let rs: Rc<AgentService> = Rc::new(AgentService::new());
//        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
//        let ws: Rc<WalletService> = Rc::new(WalletService::new());
//
//        //setup wallets
//        let (recv_wallet_handle, _, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
//
//        //setup recv_keys list
//        let recv_verkey: &str = recv_key.verkey.as_ref();
//        let recv_keys: Vec<&str> = vec![recv_verkey];
//
//        //pack then unpack message
//        let packed_msg = rs
//            .anon_pack_msg(expected_message, recv_keys, cs.clone())
//            .unwrap();
//
//        let (message, _) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key.verkey,
//                recv_wallet_handle,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //verify same plaintext goes in and comes out
//        assert_eq!(expected_message.to_string(), message);
//    }
//
//
//    pub fn test_single_auth_pack_msg_and_unpack_msg_success() {
//        _cleanup();
//        //setup generic data to test
//        let expected_message = "Hello World";
//
//        //setup route_service
//        let rs: Rc<AgentService> = Rc::new(AgentService::new());
//        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
//        let ws: Rc<WalletService> = Rc::new(WalletService::new());
//
//        //setup wallets
//        let (recv_wallet_handle, _, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
//        let (send_wallet_handle, _, send_key) = _setup_send_wallet(ws.clone(), cs.clone());
//
//        //setup recv_keys list
//        let recv_verkey: &str = recv_key.verkey.as_ref();
//        let recv_keys: Vec<&str> = vec![recv_verkey];
//
//        //pack then unpack message
//        let packed_msg = rs
//            .auth_pack_msg(
//                expected_message,
//                recv_keys,
//                &send_key.verkey,
//                send_wallet_handle,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        let (message, sender_vk) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key.verkey,
//                recv_wallet_handle,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //verify same plaintext goes in and comes out
//        assert_eq!(expected_message.to_string(), message);
//        assert_eq!(sender_vk, send_key.verkey);
//    }
//
//
//    pub fn test_pack_and_unpack_msg_success_multi_anoncrypt() {
//        _cleanup();
//        //setup generic data to test
//        let plaintext = "Hello World";
//
//        //setup route_service
//        let rs: Rc<AgentService> = Rc::new(AgentService::new());
//        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
//        let ws: Rc<WalletService> = Rc::new(WalletService::new());
//
//        //setup recv_keys to use with pack_msg
//        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
//        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
//        let recv_keys = vec![
//            recv_key1_before_wallet_setup.verkey.as_ref(),
//            recv_key2_before_wallet_setup.verkey.as_ref(),
//        ];
//
//        //setup send wallet then pack message
//        let (send_wallet_handle, _, _) = _setup_send_wallet(ws.clone(), cs.clone());
//        let packed_msg = rs.anon_pack_msg(plaintext, recv_keys, cs.clone()).unwrap();
//        let _result1 = ws.close_wallet(send_wallet_handle);
//
//        //setup recv_wallet1 and unpack message then verify plaintext
//        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
//        let (unpacked_msg1, _) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key1.verkey,
//                recv_wallet_handle1,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //test first recipient
//        assert_eq!(plaintext.to_string(), unpacked_msg1);
//        let _result2 = ws.close_wallet(recv_wallet_handle1);
//
//        //setup recv_wallet2 and unpack message then verify plaintext
//        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
//        let (unpacked_msg2, _) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key2.verkey,
//                recv_wallet_handle2,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //test second recipient
//        assert_eq!(plaintext, &unpacked_msg2);
//        let _result2 = ws.close_wallet(recv_wallet_handle2);
//    }
//
//
//    pub fn test_pack_and_unpack_msg_success_multi_authcrypt() {
//        _cleanup();
//        //setup generic data to test
//        let plaintext = "Hello World";
//
//        //setup route_service
//        let rs: Rc<AgentService> = Rc::new(AgentService::new());
//        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
//        let ws: Rc<WalletService> = Rc::new(WalletService::new());
//
//        //setup recv_keys to use with pack_msg
//        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
//        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
//        let recv_keys = vec![
//            recv_key1_before_wallet_setup.verkey.as_ref(),
//            recv_key2_before_wallet_setup.verkey.as_ref(),
//        ];
//
//        //setup send wallet then pack message
//        let (send_wallet_handle, _, expected_send_key) = _setup_send_wallet(ws.clone(), cs.clone());
//        let packed_msg = rs.auth_pack_msg(plaintext, recv_keys, &expected_send_key.verkey, send_wallet_handle, ws.clone(), cs.clone()).unwrap();
//        let _result1 = ws.close_wallet(send_wallet_handle);
//
//        //setup recv_wallet1 and unpack message then verify plaintext
//        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
//        let (unpacked_msg1, send_vk_1) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key1.verkey,
//                recv_wallet_handle1,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //test first recipient
//        assert_eq!(plaintext.to_string(), unpacked_msg1);
//        assert_eq!(&expected_send_key.verkey, &send_vk_1);
//        let _result2 = ws.close_wallet(recv_wallet_handle1);
//
//        //setup recv_wallet2 and unpack message then verify plaintext
//        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
//        let (unpacked_msg2, send_vk_2) = rs
//            .unpack_msg(
//                &packed_msg,
//                &recv_key2.verkey,
//                recv_wallet_handle2,
//                ws.clone(),
//                cs.clone(),
//            )
//            .unwrap();
//
//        //test second recipient
//        assert_eq!(plaintext, &unpacked_msg2);
//        assert_eq!(&expected_send_key.verkey, &send_vk_2);
//        let _result2 = ws.close_wallet(recv_wallet_handle2);
//    }
//}