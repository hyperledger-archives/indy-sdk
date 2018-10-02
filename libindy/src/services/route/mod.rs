use domain::route::*;
use domain::crypto::key::Key;
use domain::crypto::combo_box::ComboBox;
use errors::route::RouteError;
use errors::common::CommonError;
use services::crypto::CryptoService;
use services::wallet::{WalletService, RecordOptions};
use utils::crypto::base64::{decode, encode};
use utils::crypto::xsalsa20::{encrypt_payload, decrypt_payload};
use utils::serialization::jwm::*;
use std::rc::Rc;

pub struct RouteService { }

impl RouteService {
    pub fn new() -> RouteService {
        RouteService {}
    }

    pub fn pack_msg(&self, plaintext: &str, recv_keys: &Vec<String>, my_vk: Option<&str>, is_authcrypt: bool,
                    wallet_handle: i32, ws: Rc<WalletService>, cs: Rc<CryptoService>) -> Result<String, RouteError> {
        //encrypt plaintext
        let encrypted_payload = encrypt_payload(plaintext);

        //convert str to Key
        let key = match my_vk {
            Some(vk) => Some(ws.get_indy_object(wallet_handle, vk,
                                        &RecordOptions::id_value())
                               .map_err(|err| RouteError::UnpackError(format!("Can't find key: {:?}", err)))?),
            None => None
        };

        //encrypt content_encryption_keys
        let encrypted_ceks = self.encrypt_ceks(recv_keys, is_authcrypt, key, &encrypted_payload.sym_key, cs.clone())?;

        //create jwm string
        match recv_keys.len() {
            //handles plaintext case
            0 => Err(RouteError::PackError("No receiving keys provided".to_string())),
            //handles multi key case (JSON Serialization)
            _ => {
                json_serialize_jwm(&recv_keys,
                                        &encrypted_ceks,
                                        my_vk,
                                        &encode(&encrypted_payload.ciphertext),
                                        &encode(&encrypted_payload.iv),
                                        &encode(&encrypted_payload.tag),
                                        is_authcrypt)
            }
        }
    }

    pub fn unpack_msg(&self, ames_json_str: &str, my_vk: &str, wallet_handle: i32,
                      ws: Rc<WalletService>, cs: Rc<CryptoService>) -> Result<String, RouteError> {
        //Deserialize AMESJson
        let ames_json_struct = json_deserialize_jwm(ames_json_str)?;

        let my_key = ws.get_indy_object(wallet_handle, my_vk,
                                        &RecordOptions::id_value())
            .map_err(|err| RouteError::UnpackError(format!("Can't find key: {:?}", err)))?;

        let ames_decrypt_data = self.get_ames_data_to_decrypt(ames_json_struct, my_vk)?;

        let sym_key = self.get_sym_key(&my_key,
                                       &ames_decrypt_data.cek,
                                       ames_decrypt_data.header,
                                       cs.clone())?;

        //format payload to decrypt
        let payload = Payload {
            iv: ames_decrypt_data.iv,
            tag: ames_decrypt_data.tag,
            ciphertext: ames_decrypt_data.ciphertext,
            sym_key
        };

        //decrypt ciphertext
        decrypt_payload(&payload)
    }


    fn get_ames_data_to_decrypt(&self, jwm : AMESJson, my_vk: &str) -> Result<AMESData, RouteError> {
        //finds the recipient index that matches the verkey passed in to the recipient verkey field
        let recipient_index = jwm.recipients.iter()
            .position(|ref recipient| recipient.header.kid == my_vk);
        match recipient_index {
            Some(v) => {
                Ok(AMESData {
                    header: jwm.recipients[v].header.clone(),
                    cek: decode(&jwm.recipients[v].cek)?,
                    ciphertext: decode(&jwm.ciphertext)?,
                    iv: decode(&jwm.iv)?,
                    tag: decode(&jwm.tag)?
                })
            },
            //if no matching index is found return an error
            _ => Err(RouteError::UnpackError("The message doesn't include a header with this verkey".to_string()))
        }
    }

    fn get_sym_key(&self, key: &Key, cek: &[u8], header: Header,
                       crypto_service: Rc<CryptoService>) -> Result<Vec<u8>, RouteError> {
        match header.alg.as_ref() {
            //handles authdecrypting content encryption key
            "x-auth" => {
                let decrypted_header = crypto_service.decrypt_sealed(key, cek)
                .map_err( | err | RouteError::EncryptionError(format ! ("Can't decrypt cek: {:?}", err)))?;
                let parsed_msg = ComboBox::from_msg_pack(decrypted_header.as_slice())
                .map_err( | err | RouteError::UnpackError(format !("Can't deserialize ComboBox: {:?}", err)))?;
                let cek: Vec <u8> = decode( &parsed_msg.msg)
                .map_err( | err | RouteError::UnpackError(format ! ("Can't decode cek msg filed from base64 {}", err)))?;
                let nonce: Vec <u8> = decode( & parsed_msg.nonce)
                .map_err( | err | RouteError::UnpackError(format ! ("Can't decode cek nonce from base64 {}", err)))?;
    
                match &header.jwk {
                    Some(jwk) => Ok(crypto_service.decrypt( key, jwk, & cek, &nonce)
                        .map_err( | err | RouteError::EncryptionError(format ! ("{}", err)))?),
                    None => Err(RouteError::MissingKeyError("jwk not included to decrypt".to_string()))
                }
    
            },
    
            //handles anondecrypting content encryption algorithms
            "x-anon" => Ok(crypto_service.decrypt_sealed(key, cek)
                .map_err( | err | RouteError::EncryptionError(format ! ("{}", err)))?),
    
            //handles all other unrecognized content encryption algorithms
            _ => Err(RouteError::EncryptionError("Unexpected Content Encryption Algorithm".to_string()))
        }
    }
    
    fn encrypt_ceks(&self, recv_keys: &Vec<String>, is_authcrypt: bool, key : Option<Key>, sym_key: &[u8],
                        crypto_service: Rc<CryptoService>) -> Result<Vec<String>, RouteError>{
        let mut enc_ceks : Vec<(String)> = vec![];
    
        if is_authcrypt {
            // if authcrypting get key to encrypt, if there's not one throw error
            if key.is_some() {
                let my_key = &key.unwrap();
                for their_vk in recv_keys {
                    let cek_combo_box = crypto_service.create_combo_box(my_key, &their_vk, sym_key)
                        .map_err(|e| RouteError::EncryptionError(format!("Can't encrypt content encryption key: {:?}", e)))?;
                    let msg_pack = cek_combo_box.to_msg_pack()
                        .map_err(|e| CommonError::InvalidState(format!("Can't serialize ComboBox: {:?}", e)))?;
                    let cek_as_bytes = crypto_service.encrypt_sealed(&their_vk, &msg_pack)
                        .map_err(|e| RouteError::EncryptionError(format!("Failed to encrypt content encryption key ComboBox: {:?}", e)))?;
                    enc_ceks.push(encode(&cek_as_bytes));
                }
            } else {
                return Err(RouteError::MissingKeyError("invalid key parameter, unable to encrypt CEKs".to_string()))
            }
        }else {
            //handles anoncrypt flow of encrypting content keys
            for their_vk in recv_keys {
                let cek_as_bytes = crypto_service.encrypt_sealed(&their_vk, sym_key)
                    .map_err(|e| RouteError::EncryptionError(format!("Failed to encrypt cek: {:?}", e)))?;
                enc_ceks.push(encode(&cek_as_bytes));
            }
        }
        Ok(enc_ceks.to_owned())
    }
}

#[cfg(test)]
pub mod tests {
    use services::wallet::WalletService;
    use services::crypto::CryptoService;
    use domain::crypto::key::*;
    use domain::crypto::did::{Did, MyDidInfo};
    use super::{RouteService};
    use std::collections::HashMap;
    use utils::inmem_wallet::InmemWallet;
    use utils::test;
    use std::rc::Rc;
    use domain::wallet::Config;
    use domain::wallet::Credentials;
    use domain::wallet::KeyDerivationMethod;
    use domain::route::*;
    use utils::crypto::base64::decode;

    // TODO Fix texts so only one wallet is used to speed up tests

    //unit tests
    #[test]
    pub fn test_get_ames_data_to_decrypt_success() {
        let expected_recp_vk = "2M2U2FRSvkk5tHRALQn3Jy1YjjWtkpZ3xZyDjEuEZzko";
        let expected_recp_cek = "0PkLL5bi04zuvIg5P6qnlct-aYIq_MD1ODnO-EE7XEyQHnSszh2uWfbiKUZs4pYppHy9yjEBB3JOe0reTHSkNuX46b6MyYjU_Ld4p4ISC7g=";
        let expected_ciphertext = "-_Hdq304MkI9vOQ=";
        let expected_iv = "jrsxpWDdn06GVlrK43qQZLf5t1n4wA4o";
        let expected_tag = "k_HE0Mz0dBhaO5N-GgODYQ==";
        let header = Header::new_anoncrypt_header(expected_recp_vk);
        let recipient = Recipient::new(header.clone(), expected_recp_cek.to_string());

        let ames_json = AMESJson{
            recipients: vec![recipient],
            ciphertext: expected_ciphertext.to_string(),
            iv: expected_iv.to_string(),
            tag: expected_tag.to_string()
        };

        let expected_output : AMESData = AMESData{
            header,
            cek: decode(expected_recp_cek).unwrap(),
            ciphertext: decode(expected_ciphertext).unwrap(),
            iv: decode(expected_iv).unwrap(),
            tag: decode(expected_tag).unwrap()
        };

        let route_service = RouteService::new();
        let ames_decrypt_data = route_service.get_ames_data_to_decrypt(ames_json, expected_recp_vk).unwrap();

        assert_eq!(expected_output, ames_decrypt_data);
    }

        #[test]
    pub fn test_get_ames_data_to_decrypt_failure() {
        let bad_key = "BAD_KEY";
        let expected_recp_vk = "2M2U2FRSvkk5tHRALQn3Jy1YjjWtkpZ3xZyDjEuEZzko";
        let expected_recp_cek = "0PkLL5bi04zuvIg5P6qnlct-aYIq_MD1ODnO-EE7XEyQHnSszh2uWfbiKUZs4pYppHy9yjEBB3JOe0reTHSkNuX46b6MyYjU_Ld4p4ISC7g=";
        let expected_ciphertext = "-_Hdq304MkI9vOQ=";
        let expected_iv = "jrsxpWDdn06GVlrK43qQZLf5t1n4wA4o";
        let expected_tag = "k_HE0Mz0dBhaO5N-GgODYQ==";
        let header = Header::new_anoncrypt_header(expected_recp_vk);
        let recipient = Recipient::new(header.clone(), expected_recp_cek.to_string());

        let ames_json = AMESJson{
            recipients: vec![recipient],
            ciphertext: expected_ciphertext.to_string(),
            iv: expected_iv.to_string(),
            tag: expected_tag.to_string()
        };

        let rs = RouteService::new();
        let ames_decrypt_data = rs.get_ames_data_to_decrypt(ames_json, bad_key);

        assert!(ames_decrypt_data.is_err());
    }

    #[test]
    pub fn test_get_sym_key_success() {
        let rs = RouteService::new();
        let cs = CryptoService::new();

        //create key
        let key_info = KeyInfo {seed: None, crypto_type: None };
        let key = cs.create_key(&key_info)?;
        rs.get_sym_key()
    }
//
//    pub fn test_encrypt_ceks_success() {
//
//    }


    /* component test useful to identify if unpack is breaking or if pack is breaking. If unpack is
    * breaking both this test and the tests below will fail. If only pack is breaking, only this test
    * will fail.
    */

    #[test]
    pub fn test_unpack_msg_success_multi_anoncrypt() {
        _cleanup();

        let jwm = json!({"recipients":[
        {"header":
            {"typ":"x-b64nacl",
            "alg":"x-anon",
            "enc":"xsalsa20poly1305",
            "kid":"2M2U2FRSvkk5tHRALQn3Jy1YjjWtkpZ3xZyDjEuEZzko",
            "jwk": null},
        "cek":"0PkLL5bi04zuvIg5P6qnlct-aYIq_MD1ODnO-EE7XEyQHnSszh2uWfbiKUZs4pYppHy9yjEBB3JOe0reTHSkNuX46b6MyYjU_Ld4p4ISC7g="
        },
        {"header":
            {"typ":"x-b64nacl",
            "alg":"x-anon",
            "enc":"xsalsa20poly1305",
            "kid":"H9teBJHh4YUrbzpSMJyWRJcCQnuu4gzppbx9owvWFv8c",
            "jwk":null},
        "cek":"ivudsdb1tbK78ih3rbFbutlK9jpV2y_20vHDBRq-Ijo2VrJRruvTqu2wIyuqI0gfq5fOcEAvSuKNEMS0msJbhsVhQ_pmu5hcab7THda-yfM="
        }],
    "ciphertext":"-_Hdq304MkI9vOQ=",
    "iv":"jrsxpWDdn06GVlrK43qQZLf5t1n4wA4o",
    "tag":"k_HE0Mz0dBhaO5N-GgODYQ=="}).to_string();

        //setup services
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //run tests
        let (wallet_handle, _ , recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let plaintext = rs.unpack_msg(&jwm, &recv_key.verkey, wallet_handle, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, "Hello World".to_string());
    }

    // Integration tests
    #[test]
    pub fn test_pack_msg_success_single_anoncrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let is_authcrypt = false;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, _, _) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (send_wallet_handle , _, _) = _setup_send_wallet(ws.clone(), cs.clone());


        //setup recv_keys to use with pack_msg
        let (_ , recv_key) = _recv_did1(cs.clone());
        let recv_keys = vec![recv_key.verkey.clone()];

        //pack then unpack message
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,None, is_authcrypt,
                                                send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let unpacked_msg = rs.unpack_msg(&packed_msg, &recv_key.verkey,
                                                    recv_wallet_handle, ws.clone(), cs.clone()).unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(plaintext, &unpacked_msg);
    }

    #[test]
    pub fn test_pack_msg_success_single_authcrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let is_authcrypt = true;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, _, _) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (send_wallet_handle , _, send_key) = _setup_send_wallet(ws.clone(), cs.clone());


        //setup recv_keys to use with pack_msg
        let (_ , recv_key) = _recv_did1(cs.clone());
        let recv_keys = vec![recv_key.verkey.clone()];

        //pack then unpack message
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), is_authcrypt,
                                                send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let unpacked_msg = rs.unpack_msg(&packed_msg, &recv_key.verkey,
                                                    recv_wallet_handle, ws.clone(), cs.clone()).unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(plaintext, &unpacked_msg);
    }

    #[test]
    pub fn test_pack_and_unpack_msg_success_multi_anoncrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let is_authcrypt = false;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup recv_keys to use with pack_msg
        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
        let recv_keys = vec![recv_key1_before_wallet_setup.verkey, recv_key2_before_wallet_setup.verkey];

        //setup send wallet then pack message
        let (send_wallet_handle , _, send_key) = _setup_send_wallet(ws.clone(), cs.clone());
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), is_authcrypt,
                                                send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let _result1 = ws.close_wallet(send_wallet_handle);

        //setup recv_wallet1 and unpack message then verify plaintext
        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let unpacked_msg1 = rs.unpack_msg(&packed_msg, &recv_key1.verkey,
                                                     recv_wallet_handle1, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, &unpacked_msg1);
        let _result2 = ws.close_wallet(recv_wallet_handle1);


        //setup recv_wallet2 and unpack message then verify plaintext
        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
        let unpacked_msg2 = rs.unpack_msg(&packed_msg, &recv_key2.verkey,
                                                     recv_wallet_handle2, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, &unpacked_msg2);
    }

    #[test]
    pub fn test_pack_and_unpack_msg_success_multi_authcrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let is_authcrypt = true;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup recv_keys to use with pack_msg
        let (_, recv_key1_before_wallet_setup) = _recv_did1(cs.clone());
        let (_, recv_key2_before_wallet_setup) = _recv_did2(cs.clone());
        let recv_keys = vec![recv_key1_before_wallet_setup.verkey, recv_key2_before_wallet_setup.verkey];

        //setup send wallet then pack message
        let (send_wallet_handle , _, send_key) = _setup_send_wallet(ws.clone(), cs.clone());
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), is_authcrypt,
                                                send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let _result1 = ws.close_wallet(send_wallet_handle);

        //setup recv_wallet1 and unpack message then verify plaintext
        let (recv_wallet_handle1, _, recv_key1) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let unpacked_msg1 = rs.unpack_msg(&packed_msg, &recv_key1.verkey,
                                                     recv_wallet_handle1, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, &unpacked_msg1);
        let _result2 = ws.close_wallet(recv_wallet_handle1);


        //setup recv_wallet2 and unpack message then verify plaintext
        let (recv_wallet_handle2, _, recv_key2) = _setup_recv_wallet2(ws.clone(), cs.clone());
        let unpacked_msg2 = rs.unpack_msg(&packed_msg, &recv_key2.verkey,
                                                     recv_wallet_handle2, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, &unpacked_msg2);
    }

    fn _setup_send_wallet(ws: Rc<WalletService>, cs : Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _send_did1(cs.clone());
        let _result = ws.create_wallet(&_send_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_send_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new()).unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new()).unwrap();
        (wallet_handle, did, key)
    }

    fn _setup_recv_wallet1(ws: Rc<WalletService>, cs : Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _recv_did1(cs.clone());
        let _result = ws.create_wallet(&_recv_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_recv_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new()).unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new()).unwrap();
        (wallet_handle, did, key)
    }

    fn _setup_recv_wallet2(ws: Rc<WalletService>, cs : Rc<CryptoService>) -> (i32, Did, Key) {
        let (did, key) = _recv_did2(cs.clone());
        let _result = ws.create_wallet(&_recv_config(), &_credentials());
        let wallet_handle = ws.open_wallet(&_recv_config(), &_credentials()).unwrap();
        ws.add_indy_object(wallet_handle, &did.did, &did, &HashMap::new()).unwrap();
        ws.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new()).unwrap();
        (wallet_handle, did, key)
    }

    fn _send_did1(service : Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo { did: None, cid: None, seed: Some("000000000000000000000000000SEND1".to_string()), crypto_type: None };
        service.create_my_did(&did_info).unwrap()
    }

    fn _recv_did1(service : Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo { did: None, cid: None, seed: Some("000000000000000000000000000RECV1".to_string()), crypto_type: None };
        service.create_my_did(&did_info).unwrap()
    }

    fn _recv_did2(service : Rc<CryptoService>) -> (Did, Key) {
        let did_info = MyDidInfo {did: None, cid: None, seed: Some("000000000000000000000000000RECV2".to_string()), crypto_type: None };
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
        }).to_string()
    }
}