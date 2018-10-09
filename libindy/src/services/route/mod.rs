use core::result;
use domain::crypto::key::Key;
use domain::route::*;
use errors::route::RouteError;
use serde_json;
use services::crypto::CryptoService;
use services::wallet::{RecordOptions, WalletService};
use std::rc::Rc;
use utils::crypto::base58;
use utils::crypto::base64;
use utils::crypto::xsalsa20;
use utils::crypto::xsalsa20::{create_key, gen_nonce};

type Result<T> = result::Result<T, RouteError>;

pub struct RouteService {}

impl RouteService {
    pub fn new() -> RouteService {
        RouteService {}
    }

    pub fn auth_pack_msg(
        &self,
        message: &str,
        recv_keys: Vec<&str>,
        sender_vk: &str,
        wallet_handle: i32,
        ws: Rc<WalletService>,
        cs: Rc<CryptoService>,
    ) -> Result<String> {
        //encrypt ciphertext
        let (sym_key, iv, ciphertext) = self.encrypt_ciphertext(message);

        //convert sender_vk to Key
        let my_key = &ws
            .get_indy_object(wallet_handle, sender_vk, &RecordOptions::id_value())
            .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

        //encrypt ceks
        let mut auth_recipients = vec![];

        for their_vk in recv_keys {
            auth_recipients.push(
                self.auth_encrypt_recipient(my_key, their_vk, &sym_key, cs.clone())
                    .map_err(|err| {
                        RouteError::PackError(format!("Failed to push auth recipient {}", err))
                    })?,
            );
        }

        //serialize AuthAMES
        let auth_ames_struct = AuthAMES {
            recipients: auth_recipients,
            ver: "AuthAMES/1.0/".to_string(),
            enc: "xsalsa20poly1305".to_string(),
            ciphertext: base64::encode(ciphertext.as_slice()),
            iv: base64::encode(&iv[..]),
        };
        serde_json::to_string(&auth_ames_struct)
            .map_err(|err| RouteError::PackError(format!("Failed to serialize authAMES {}", err)))
    }

    pub fn anon_pack_msg(
        &self,
        message: &str,
        recv_keys: Vec<&str>,
        cs: Rc<CryptoService>,
    ) -> Result<String> {
        //encrypt ciphertext
        let (sym_key, iv, ciphertext) = self.encrypt_ciphertext(message);

        //encrypt ceks
        let mut anon_recipients: Vec<AnonRecipient> = vec![];
        for their_vk in recv_keys {
            let anon_recipient =
                self.anon_encrypt_recipient(their_vk, sym_key.clone(), cs.clone())?;
            anon_recipients.push(anon_recipient);
        }

        //serialize AnonAMES
        let anon_ames_struct = AnonAMES {
            recipients: anon_recipients,
            ver: "AnonAMES/1.0/".to_string(),
            enc: "xsalsa20poly1305".to_string(),
            ciphertext: base64::encode(ciphertext.as_slice()),
            iv: base64::encode(&iv[..]),
        };
        serde_json::to_string(&anon_ames_struct)
            .map_err(|err| RouteError::PackError(format!("Failed to serialize anonAMES {}", err)))
    }

    pub fn unpack_msg(
        &self,
        ames_json_str: &str,
        my_vk: &str,
        wallet_handle: i32,
        ws: Rc<WalletService>,
        cs: Rc<CryptoService>,
    ) -> Result<(String, String)> {
        //check if authAMES or anonAMES
        if ames_json_str.contains("AuthAMES/1.0/") {
            self.auth_unpack(ames_json_str, my_vk, wallet_handle, ws, cs)
        } else if ames_json_str.contains("AnonAMES/1.0/") {
            self.anon_unpack(ames_json_str, my_vk, wallet_handle, ws, cs)
        } else {
            Err(RouteError::UnpackError(format!(
                "Failed to unpack - unidentified ver provided"
            )))
        }
    }

    fn auth_unpack(
        &self,
        ames_json_str: &str,
        my_vk: &str,
        wallet_handle: i32,
        ws: Rc<WalletService>,
        cs: Rc<CryptoService>,
    ) -> Result<(String, String)> {
        //deserialize json string to struct
        let auth_ames_struct: AuthAMES = serde_json::from_str(ames_json_str).map_err(|err| {
            RouteError::SerializationError(format!("Failed to deserialize auth ames {}", err))
        })?;

        //get recipient struct that matches my_vk parameter
        let recipient_struct =
            self.get_auth_recipient_header(my_vk, auth_ames_struct.recipients)?;

        //get key to use for decryption
        let my_key: &Key = &ws
            .get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())
            .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

        //decrypt recipient header
        let (ephem_sym_key, sender_vk) =
            self.auth_decrypt_recipient(my_key, recipient_struct, cs)?;

        // decode
        let message = self.decrypt_ciphertext(
            &auth_ames_struct.ciphertext,
            &auth_ames_struct.iv,
            &ephem_sym_key,
        )?;

        Ok((message, sender_vk))
    }

    fn anon_unpack(
        &self,
        ames_json_str: &str,
        my_vk: &str,
        wallet_handle: i32,
        ws: Rc<WalletService>,
        cs: Rc<CryptoService>,
    ) -> Result<(String, String)> {
        //deserialize json string to struct
        let auth_ames_struct: AnonAMES = serde_json::from_str(ames_json_str).map_err(|err| {
            RouteError::SerializationError(format!("Failed to deserialize auth ames {}", err))
        })?;

        //get recipient struct that matches my_vk parameter
        let recipient_struct =
            self.get_anon_recipient_header(my_vk, auth_ames_struct.recipients)?;

        //get key to use for decryption
        let my_key: &Key = &ws
            .get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())
            .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

        //decrypt recipient header
        let ephem_sym_key = self.anon_decrypt_recipient(my_key, recipient_struct, cs)?;

        //decrypt message
        let message = self.decrypt_ciphertext(
            &auth_ames_struct.ciphertext,
            &auth_ames_struct.iv,
            &ephem_sym_key,
        )?;

        //return message and no key
        Ok((message, "".to_string()))
    }

    /* Authcrypt helper function section */
    pub fn auth_encrypt_recipient(
        &self,
        my_key: &Key,
        recp_vk: &str,
        sym_key: &xsalsa20::Key,
        cs: Rc<CryptoService>,
    ) -> Result<AuthRecipient> {
        //encrypt sym_key for recipient
        let (e_cek, cek_nonce) = cs.encrypt(my_key, recp_vk, &sym_key[..]).map_err(|err| {
            RouteError::EncryptionError(format!("Failed to auth encrypt cek {}", err))
        })?;

        //serialize enc_header
        let sender_vk_bytes = base58::decode(&my_key.verkey).map_err(|err| {
            RouteError::SerializationError(format!("Failed to serialize cek {}", err))
        })?;

        //encrypt enc_from
        let enc_from = cs
            .encrypt_sealed(recp_vk, sender_vk_bytes.as_slice())
            .map_err(|err| {
                RouteError::EncryptionError(format!("Failed to encrypt sender verkey {}", err))
            })?;;

        //create struct
        let auth_recipient = AuthRecipient {
            enc_from: base64::encode(enc_from.as_slice()),
            e_cek: base64::encode(e_cek.as_slice()),
            cek_nonce: base64::encode(cek_nonce.as_slice()),
            to: recp_vk.to_string(),
        };

        //return AuthRecipient struct
        Ok(auth_recipient)
    }

    pub fn auth_decrypt_recipient(
        &self,
        my_key: &Key,
        auth_recipient: AuthRecipient,
        cs: Rc<CryptoService>,
    ) -> Result<(xsalsa20::Key, String)> {
        //decode enc_from
        let enc_from_bytes = base64::decode(&auth_recipient.enc_from)
            .map_err(|err| RouteError::DecodeError(format!("Failed to decode enc_from {}", err)))?;

        //decrypt enc_from
        let sender_vk_as_vec =
            cs.decrypt_sealed(my_key, enc_from_bytes.as_ref())
                .map_err(|err| {
                    RouteError::EncryptionError(format!("Failed to decrypt sender verkey {}", err))
                })?;

        //encode sender_vk to base58 to use to decrypt cek
        let sender_vk = base58::encode(sender_vk_as_vec.as_ref());

        //decode e_cek
        let e_cek_as_vec = base64::decode(&auth_recipient.e_cek)
            .map_err(|err| RouteError::DecodeError(format!("Failed to decode e_cek {}", err)))?;
        let e_cek: &[u8] = e_cek_as_vec.as_ref();

        //type convert cek_nonce
        let cek_nonce_as_vec = base64::decode(&auth_recipient.cek_nonce).map_err(|err| {
            RouteError::DecodeError(format!("Failed to decode cek_nonce {}", err))
        })?;
        let cek_nonce: &[u8] = cek_nonce_as_vec.as_ref();

        //decrypt cek
        let decrypted_cek = cs
            .decrypt(my_key, &sender_vk, e_cek, cek_nonce)
            .map_err(|err| {
                RouteError::EncryptionError(format!("Failed to auth decrypt cek {}", err))
            })?;

        //convert to secretbox key
        let sym_key = xsalsa20::Key::from_slice(&decrypted_cek[..]).map_err(|err| {
            RouteError::EncryptionError(format!("Failed to unpack sym_key {}", err))
        })?;

        //TODO Verify key is in DID Doc

        //return key to decrypt ciphertext and the key used to decrypt with
        Ok((sym_key, sender_vk))
    }

    fn get_auth_recipient_header(
        &self,
        recp_vk: &str,
        auth_recipients: Vec<AuthRecipient>,
    ) -> Result<AuthRecipient> {
        let my_vk_as_string = recp_vk.to_string();
        for auth_recipient in auth_recipients {
            if auth_recipient.to == my_vk_as_string {
                return Ok(auth_recipient);
            }
        }

        return Err(RouteError::UnpackError(format!(
            "Failed to find a matching header"
        )));
    }

    /* Authcrypt helper function section */
    fn anon_encrypt_recipient(
        &self,
        recp_vk: &str,
        sym_key: xsalsa20::Key,
        cs: Rc<CryptoService>,
    ) -> Result<AnonRecipient> {
        //encrypt cek
        let cek = cs.encrypt_sealed(recp_vk, &sym_key[..]).map_err(|err| {
            RouteError::PackError(format!("Failed to encrypt anon recipient {}", err))
        })?;

        //generate struct
        let anon_recipient = AnonRecipient {
            to: recp_vk.to_string(),
            cek: base64::encode(cek.as_slice()),
        };

        Ok(anon_recipient)
    }

    fn anon_decrypt_recipient(
        &self,
        my_key: &Key,
        anon_recipient: AnonRecipient,
        cs: Rc<CryptoService>,
    ) -> Result<xsalsa20::Key> {
        let cek_as_vec = base64::decode(&anon_recipient.cek).map_err(|err| {
            RouteError::DecodeError(format!(
                "Failed to decode cek for anon_decrypt_recipient {}",
                err
            ))
        })?;
        let cek_as_bytes = cek_as_vec.as_ref();

        let decrypted_cek = cs
            .decrypt_sealed(my_key, cek_as_bytes)
            .map_err(|err| RouteError::EncryptionError(format!("Failed to decrypt cek {}", err)))?;

        //convert to secretbox key
        let sym_key = xsalsa20::Key::from_slice(&decrypted_cek[..]).map_err(|err| {
            RouteError::EncryptionError(format!("Failed to unpack sym_key {}", err))
        })?;

        //return key
        Ok(sym_key)
    }

    fn get_anon_recipient_header(
        &self,
        recp_vk: &str,
        anon_recipients: Vec<AnonRecipient>,
    ) -> Result<AnonRecipient> {
        let my_vk_as_string = recp_vk.to_string();
        for recipient in anon_recipients {
            if recipient.to == my_vk_as_string {
                return Ok(recipient);
            }
        }

        return Err(RouteError::UnpackError(format!(
            "Failed to find a matching header"
        )));
    }

    /* ciphertext helper functions*/
    fn decrypt_ciphertext(
        &self,
        ciphertext: &str,
        iv: &str,
        sym_key: &xsalsa20::Key,
    ) -> Result<String> {
        //convert IV from &str to &Nonce
        let iv_as_vec = base64::decode(iv)
            .map_err(|err| RouteError::DecodeError(format!("Failed to decode IV {}", err)))?;
        let iv_as_slice = iv_as_vec.as_slice();
        let nonce = xsalsa20::Nonce::from_slice(iv_as_slice).map_err(|err| {
            RouteError::UnpackError(format!("Failed to convert IV to Nonce type {}", err))
        })?;

        //convert ciphertext to bytes
        let ciphertext_as_vec = base64::decode(ciphertext).map_err(|err| {
            RouteError::DecodeError(format!("Failed to decode ciphertext {}", err))
        })?;
        let ciphertext_as_bytes = ciphertext_as_vec.as_ref();

        //decrypt message
        let plaintext_bytes =
            xsalsa20::decrypt(sym_key, &nonce, ciphertext_as_bytes).map_err(|err| {
                RouteError::EncryptionError(format!("Failed to decrypt ciphertext {}", err))
            })?;

        //convert message to readable (UTF-8) string
        String::from_utf8(plaintext_bytes).map_err(|err| {
            RouteError::DecodeError(format!("Failed to convert message to UTF-8 {}", err))
        })
    }

    fn encrypt_ciphertext(&self, ciphertext: &str) -> (xsalsa20::Key, xsalsa20::Nonce, Vec<u8>) {
        let sym_key = create_key();
        let iv = gen_nonce();
        let message = xsalsa20::encrypt(&sym_key, &iv, ciphertext.as_bytes());

        (sym_key, iv, message)
    }
}

#[cfg(test)]
pub mod tests {
    use super::RouteService;
    use domain::crypto::did::{Did, MyDidInfo};
    use domain::crypto::key::Key;
    use domain::wallet::Config;
    use domain::wallet::Credentials;
    use domain::wallet::KeyDerivationMethod;
    use services::crypto::CryptoService;
    use services::wallet::WalletService;
    use std::collections::HashMap;
    use std::rc::Rc;
    use utils::crypto::xsalsa20;
    use utils::inmem_wallet::InmemWallet;
    use utils::test;

    // TODO Fix texts so only one wallet is used to speed up tests
    //unit tests
    #[test]
    pub fn test_auth_encrypt_decrypt_recipient() {
        _cleanup();
        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (_, _, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (_, _, expected_send_key) = _setup_send_wallet(ws.clone(), cs.clone());

        //setup recv_keys to use with pack_msg
        let recv_verkey: &str = recv_key.verkey.as_ref();

        //sym_key
        let sym_key = xsalsa20::create_key();

        //pack then unpack message
        let auth_recipient = rs
            .auth_encrypt_recipient(&expected_send_key, &recv_verkey, &sym_key, cs.clone())
            .unwrap();
        let (expected_sym_key, sender_vk) = rs
            .auth_decrypt_recipient(&recv_key, auth_recipient, cs.clone())
            .unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(expected_sym_key, sym_key);
        assert_eq!(expected_send_key.verkey, sender_vk);
    }

    /* component test useful to identify if unpack is breaking or if pack is breaking. If unpack is
     * breaking both this test and the tests below will fail. If only pack is breaking, only this test
     * will fail.
     */

    // Integration tests
    #[test]
    pub fn test_single_anon_pack_message_and_unpack_msg_success() {
        _cleanup();
        //setup generic data to test
        let expected_message = "Hello World";

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, _, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());

        //setup recv_keys list
        let recv_verkey: &str = recv_key.verkey.as_ref();
        let recv_keys: Vec<&str> = vec![recv_verkey];

        //pack then unpack message
        let packed_msg = rs
            .anon_pack_msg(expected_message, recv_keys, cs.clone())
            .unwrap();

        let (message, _) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key.verkey,
                recv_wallet_handle,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(expected_message.to_string(), message);
    }

    #[test]
    pub fn test_single_auth_pack_msg_and_unpack_msg_success() {
        _cleanup();
        //setup generic data to test
        let expected_message = "Hello World";

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, _, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (send_wallet_handle, _, send_key) = _setup_send_wallet(ws.clone(), cs.clone());

        //setup recv_keys list
        let recv_verkey: &str = recv_key.verkey.as_ref();
        let recv_keys: Vec<&str> = vec![recv_verkey];

        //pack then unpack message
        let packed_msg = rs
            .auth_pack_msg(
                expected_message,
                recv_keys,
                &send_key.verkey,
                send_wallet_handle,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        let (message, sender_vk) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key.verkey,
                recv_wallet_handle,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(expected_message.to_string(), message);
        assert_eq!(sender_vk, send_key.verkey);
    }

    #[test]
    pub fn test_pack_and_unpack_msg_success_multi_anoncrypt() {
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup recv_keys to use with pack_msg
        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
        let recv_keys = vec![
            recv_key1_before_wallet_setup.verkey.as_ref(),
            recv_key2_before_wallet_setup.verkey.as_ref(),
        ];

        //setup send wallet then pack message
        let (send_wallet_handle, _, _) = _setup_send_wallet(ws.clone(), cs.clone());
        let packed_msg = rs.anon_pack_msg(plaintext, recv_keys, cs.clone()).unwrap();
        let _result1 = ws.close_wallet(send_wallet_handle);

        //setup recv_wallet1 and unpack message then verify plaintext
        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (unpacked_msg1, _) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key1.verkey,
                recv_wallet_handle1,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //test first recipient
        assert_eq!(plaintext.to_string(), unpacked_msg1);
        let _result2 = ws.close_wallet(recv_wallet_handle1);

        //setup recv_wallet2 and unpack message then verify plaintext
        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
        let (unpacked_msg2, _) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key2.verkey,
                recv_wallet_handle2,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //test second recipient
        assert_eq!(plaintext, &unpacked_msg2);
        let _result2 = ws.close_wallet(recv_wallet_handle2);
    }

    #[test]
    pub fn test_pack_and_unpack_msg_success_multi_authcrypt() {
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup recv_keys to use with pack_msg
        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
        let recv_keys = vec![
            recv_key1_before_wallet_setup.verkey.as_ref(),
            recv_key2_before_wallet_setup.verkey.as_ref(),
        ];

        //setup send wallet then pack message
        let (send_wallet_handle, _, expected_send_key) = _setup_send_wallet(ws.clone(), cs.clone());
        let packed_msg = rs.auth_pack_msg(plaintext, recv_keys, &expected_send_key.verkey, send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let _result1 = ws.close_wallet(send_wallet_handle);

        //setup recv_wallet1 and unpack message then verify plaintext
        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (unpacked_msg1, send_vk_1) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key1.verkey,
                recv_wallet_handle1,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //test first recipient
        assert_eq!(plaintext.to_string(), unpacked_msg1);
        assert_eq!(&expected_send_key.verkey, &send_vk_1);
        let _result2 = ws.close_wallet(recv_wallet_handle1);

        //setup recv_wallet2 and unpack message then verify plaintext
        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
        let (unpacked_msg2, send_vk_2) = rs
            .unpack_msg(
                &packed_msg,
                &recv_key2.verkey,
                recv_wallet_handle2,
                ws.clone(),
                cs.clone(),
            )
            .unwrap();

        //test second recipient
        assert_eq!(plaintext, &unpacked_msg2);
        assert_eq!(&expected_send_key.verkey, &send_vk_2);
        let _result2 = ws.close_wallet(recv_wallet_handle2);
    }

    fn _setup_send_wallet(ws: Rc<WalletService>, cs: Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _send_did1(cs.clone());
        let _result = ws.create_wallet(&_send_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_send_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new())
            .unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())
            .unwrap();
        (wallet_handle, did, key)
    }

    fn _setup_recv_wallet1(ws: Rc<WalletService>, cs: Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _recv_did1(cs.clone());
        let _result = ws.create_wallet(&_recv_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_recv_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new())
            .unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())
            .unwrap();
        (wallet_handle, did, key)
    }

    fn _setup_recv_wallet2(ws: Rc<WalletService>, cs: Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _recv_did2(cs.clone());
        let _result = ws.create_wallet(&_recv_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_recv_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new())
            .unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())
            .unwrap();
        (wallet_handle, did, key)
    }

    fn _send_did1(service: Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo {
            did: None,
            cid: None,
            seed: Some("000000000000000000000000000SEND1".to_string()),
            crypto_type: None,
        };
        service.create_my_did(&did_info).unwrap()
    }

    fn _recv_did1(service: Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo {
            did: None,
            cid: None,
            seed: Some("000000000000000000000000000RECV1".to_string()),
            crypto_type: None,
        };
        service.create_my_did(&did_info).unwrap()
    }

    fn _recv_did2(service: Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo {
            did: None,
            cid: None,
            seed: Some("000000000000000000000000000RECV2".to_string()),
            crypto_type: None,
        };
        service.create_my_did(&did_info).unwrap()
    }

    fn _send_config() -> Config {
        Config {
            id: "w1".to_string(),
            storage_type: None,
            storage_config: None,
        }
    }

    fn _recv_config() -> Config {
        Config {
            id: "recv1".to_string(),
            storage_type: None,
            storage_config: None,
        }
    }

    fn _config() -> Config {
        Config {
            id: "w1".to_string(),
            storage_type: None,
            storage_config: None,
        }
    }

    fn _credentials() -> Credentials {
        Credentials {
            key: "my_key".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
            rekey_derivation_method: KeyDerivationMethod::ARGON2I_MOD,
        }
    }

    fn _cleanup() {
        test::cleanup_storage();
        InmemWallet::cleanup();
    }

    fn _fetch_options(type_: bool, value: bool, tags: bool) -> String {
        json!({
          "retrieveType": type_,
          "retrieveValue": value,
          "retrieveTags": tags,
        })
        .to_string()
    }
}
