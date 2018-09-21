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
use std::collections::HashMap;
use services::wallet::WalletRecord;

pub struct RouteService { }

impl RouteService {
    pub fn new() -> RouteService {
        RouteService {}
    }

    pub fn unpack_msg(&self, json_jwm: &str, my_vk: &str, wallet_handle: i32, 
                      ws: Rc<WalletService>, cs: Rc<CryptoService>) -> Result<String, RouteError> {
        //check if jwm or jwm_compact
        let jwm_struct = match json_jwm.contains("recipients") {
            true => json_deserialize_jwm(json_jwm)?,
            false => deserialize_jwm_compact(json_jwm)?
        };

        let jwm_data = self.get_jwm_data(jwm_struct, my_vk)?;
        let my_key = self.get_key_from_str(my_vk, wallet_handle, ws.clone())?;
        let sym_key = self.get_sym_key(&my_key, &jwm_data.cek, jwm_data.header, cs.clone())?;
        //format payload to decrypt
        let payload = Payload {
            iv: jwm_data.iv,
            tag: jwm_data.tag,
            ciphertext: jwm_data.ciphertext,
            sym_key
        };

        //decrypt ciphertext
        decrypt_payload(&payload)
    }


    // This API call is made to encrypt both Application layer messages and Transport layer
// messages. The purpose of it is to take a message and wrap it up so that it can be fed into
// send_msg and on the other end unpack_msg can be called on it.
    pub fn pack_msg(&self, plaintext: &str, recv_keys: &Vec<String>, my_vk: Option<&str>, auth: bool,
                    wallet_handle: i32, ws: Rc<WalletService>, cs: Rc<CryptoService>) -> Result<String, RouteError> {
        //encrypt plaintext
        let encrypted_payload = encrypt_payload(plaintext);

        //convert str to Key
        let key = match my_vk {
            Some(vk) => Some(self.get_key_from_str(vk, wallet_handle, ws.clone())?),
            None => None
        };

        //encrypt content_encryption_keys
        let encrypted_ceks = self.encrypt_ceks(recv_keys, auth, key, &encrypted_payload.sym_key, cs.clone())?;

        //create jwm string
        match recv_keys.len() {
            //handles plaintext case
            0 => Err(RouteError::PackError("No receiving keys provided".to_string())),
            //handles single key case (compact serialization)
            1 => {
                serialize_jwm_compact(&recv_keys[0],
                                           &encrypted_ceks[0],
                                           my_vk,
                                           &encode(&encrypted_payload.ciphertext),
                                           &encode(&encrypted_payload.iv),
                                           &encode(&encrypted_payload.tag),
                                           auth)
            },
            //handles multi key case (JSON Serialization)
            _ => {
                json_serialize_jwm(&recv_keys,
                                        &encrypted_ceks,
                                        my_vk,
                                        &encode(&encrypted_payload.ciphertext),
                                        &encode(&encrypted_payload.iv),
                                        &encode(&encrypted_payload.tag),
                                        auth)
            }
        }
    }
    
    pub fn add_route(&self, did_with_key_frag : &str, endpoint : &str,
                     wallet_handle:i32, wallet_service: Rc<WalletService>) -> Result<(), RouteError> {
        wallet_service.add_record(wallet_handle, "route_table", did_with_key_frag, endpoint, &HashMap::new())
            .map_err(|err | RouteError::TableError(format!("Failed to add route: {:?}", err)))
    }

    pub fn lookup_route(&self, did_with_key_frag : &str,
                        wallet_handle : i32, wallet_service: Rc<WalletService>) -> Result<String, RouteError> {
        let options_json = json!({"retrieveType": false,"retrieveValue": true,"retrieveTags": false}).to_string();
        let wallet_record : WalletRecord = wallet_service.get_record(wallet_handle, "route_table",
                                                      did_with_key_frag, &options_json)
            .map_err(|err|RouteError::TableError(format!("Failed to locate value in route table: {:?}", err)))?;

        match wallet_record.get_value() {
                Some(value) =>  Ok(value.to_string()),
                None => Err(RouteError::TableError(format!("Failed to locate value in route table")))
        }
    }

    pub fn remove_route(&self, did_with_key_frag : &str,
                        wallet_handle : i32, wallet_service: Rc<WalletService>) -> Result<(), RouteError> {
        wallet_service.delete_record(wallet_handle, "route_table", did_with_key_frag)
            .map_err(|err|RouteError::TableError(format!("Failed to remove route: {:?}", err)))
    }

    pub fn update_route(&self, did_with_key_frag : &str, new_endpoint : &str,
                        wallet_handle : i32, wallet_service: Rc<WalletService>) -> Result<(), RouteError> {
        wallet_service.update_record_value(wallet_handle, "route_table", did_with_key_frag, new_endpoint)
            .map_err(|err| RouteError::TableError(format!("Failed to update route: {:?}", err)))
    }

    fn get_jwm_data(&self, jwm : AMES, my_vk: &str) -> Result<AMESData, RouteError> {
        match jwm {
            AMES::JWMFull(jwmf) => {
                //finds the recipient index that matches the verkey passed in to the recipient verkey field
                let recipient_index = jwmf.recipients.iter()
                    .position(|ref recipient| recipient.header.kid == my_vk);
                match recipient_index {
                    Some(v) => {
                        Ok(AMESData {
                            header: jwmf.recipients[v].header.clone(),
                            cek: decode(&jwmf.recipients[v].cek)?,
                            ciphertext: decode(&jwmf.ciphertext)?,
                            iv: decode(&jwmf.iv)?,
                            tag: decode(&jwmf.tag)?
                        })
                    },
                    //if no matching index is found return an error
                    _ => Err(RouteError::UnpackError("The message doesn't include a header with this verkey".to_string()))
                }
            },
    
            AMES::JWMCompact(jwmc) => {
                if jwmc.header.kid == my_vk {
                    Ok(AMESData {
                        header: jwmc.header,
                        cek: decode(&jwmc.cek)?,
                        ciphertext: decode(&jwmc.ciphertext)?,
                        iv: decode(&jwmc.iv)?,
                        tag: decode(&jwmc.tag)?
                    })
                } else {
                    Err(RouteError::UnpackError("The message doesn't include a header with this verkey".to_string()))
                }
            }
        }
    }
    
    fn get_key_from_str(&self, my_vk : &str, wallet_handle: i32,
                            wallet_service: Rc<WalletService>) -> Result<Key, RouteError> {
        wallet_service.get_indy_object(wallet_handle,
                                       my_vk,
                                       &RecordOptions::id_value(),
                                       &mut String::new())
        .map_err(|err| RouteError::UnpackError(format!("Can't find key: {:?}", err)))
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
                let cek: Vec < u8 > = decode( &parsed_msg.msg)
                .map_err( | err | RouteError::UnpackError(format ! ("Can't decode cek msg filed from base64 {}", err)))?;
                let nonce: Vec <u8 > = decode( & parsed_msg.nonce)
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
    
    fn encrypt_ceks(&self, recv_keys: &Vec<String>, auth: bool, key : Option<Key>, sym_key: &[u8],
                        crypto_service: Rc<CryptoService>) -> Result<Vec<String>, RouteError>{
        let mut enc_ceks : Vec<(String)> = vec![];
    
        if auth {
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


//
//pub fn get_next_hop(did_with_key_frag: &str) -> (&str, &str) {
////DID#key is a reference identifier to the next hop
////their_vk is used to encrypt the message
////endpoint is the endpoint which the message is being sent to.
////called by send_msg()
////returns (endpoint, their_vk)
//
//}

#[cfg(test)]
pub mod tests {
    use services::wallet::WalletService;
    use services::crypto::CryptoService;
    use domain::crypto::key::Key;
    use domain::crypto::did::{Did, MyDidInfo};
    use super::{RouteService};
    use std::collections::HashMap;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;
    use std::rc::Rc;

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
        let (wallet_handle, recv_did, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let plaintext = rs.unpack_msg(&jwm, &recv_key.verkey, wallet_handle, ws.clone(), cs.clone()).unwrap();
        assert_eq!(plaintext, "Hello World".to_string());
    }

    #[test]
    pub fn test_pack_and_unpack_msg_success_multi_anoncrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let auth = false;

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
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), auth,
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
        let auth = true;

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
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), auth,
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
    pub fn test_pack_msg_success_single_anoncrypt(){
        _cleanup();
        //setup generic data to test
        let plaintext = "Hello World";
        let auth = false;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, recv_did, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (send_wallet_handle , send_did, send_key) = _setup_send_wallet(ws.clone(), cs.clone());


        //setup recv_keys to use with pack_msg
        let (_ , recv_key) = _recv_did1(cs.clone());
        let recv_keys = vec![recv_key.verkey.clone()];

        //pack then unpack message
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,None, auth,
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
        let auth = true;

        //setup route_service
        let rs: Rc<RouteService> = Rc::new(RouteService::new());
        let cs: Rc<CryptoService> = Rc::new(CryptoService::new());
        let ws: Rc<WalletService> = Rc::new(WalletService::new());

        //setup wallets
        let (recv_wallet_handle, recv_did, recv_key) = _setup_recv_wallet1(ws.clone(), cs.clone());
        let (send_wallet_handle , send_did, send_key) = _setup_send_wallet(ws.clone(), cs.clone());


        //setup recv_keys to use with pack_msg
        let (_ , recv_key) = _recv_did1(cs.clone());
        let recv_keys = vec![recv_key.verkey.clone()];

        //pack then unpack message
        let packed_msg = rs.pack_msg(plaintext, &recv_keys,Some(&send_key.verkey), auth,
                                                send_wallet_handle, ws.clone(), cs.clone()).unwrap();
        let unpacked_msg = rs.unpack_msg(&packed_msg, &recv_key.verkey,
                                                    recv_wallet_handle, ws.clone(), cs.clone()).unwrap();

        //verify same plaintext goes in and comes out
        assert_eq!(plaintext, &unpacked_msg);
    }

    #[test]
    fn test_lookup_route_fail() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let ws: Rc<WalletService> = Rc::new(WalletService::new());
        let rs: Rc<RouteService> = Rc::new(RouteService::new());

        ws.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = ws.open_wallet(&_config(), &_credentials()).unwrap();

        let endpoint_lookup = rs.lookup_route(did_with_key_frag, wallet_handle, ws.clone());
        assert!(endpoint_lookup.is_err());
    }

    #[test]
    fn test_add_and_lookup_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let ws: Rc<WalletService> = Rc::new(WalletService::new());
        let rs: Rc<RouteService> = Rc::new(RouteService::new());

        ws.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = ws.open_wallet(&_config(), &_credentials()).unwrap();

        let _result = rs.add_route(did_with_key_frag, endpoint, wallet_handle, ws.clone());
        let endpoint_lookup = rs.lookup_route(did_with_key_frag, wallet_handle, ws.clone()).unwrap();
        assert_eq!(&endpoint_lookup, endpoint);
    }

    #[test]
    fn test_add_remove_then_lookup_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let ws: Rc<WalletService> = Rc::new(WalletService::new());
        let rs: Rc<RouteService> = Rc::new(RouteService::new());

        ws.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = ws.open_wallet(&_config(), &_credentials()).unwrap();

        let _result1 = rs.add_route(did_with_key_frag, endpoint, wallet_handle, ws.clone());
        let _result2 = rs.remove_route(did_with_key_frag, wallet_handle, ws.clone());
        let endpoint_lookup = rs.lookup_route(did_with_key_frag, wallet_handle, ws.clone());
        assert!(endpoint_lookup.is_err());
    }

    #[test]
    fn test_update_route_success() {
        _cleanup();
        let did_with_key_frag : &str = &"did:sov:NCjtLejiBg18RAV9mefAQT#1";
        let endpoint : &str = &"http://localhost:8080";
        let ws: Rc<WalletService> = Rc::new(WalletService::new());
        let rs: Rc<RouteService> = Rc::new(RouteService::new());

        ws.create_wallet(&_config(), &_credentials()).unwrap();
        let wallet_handle = ws.open_wallet(&_config(), &_credentials()).unwrap();

        let _result = rs.add_route(did_with_key_frag, endpoint, wallet_handle, ws.clone()).unwrap();
        let _fail_if_none = rs.update_route(did_with_key_frag, &"http://localhost:8081", wallet_handle, ws.clone()).unwrap();
        let endpoint_lookup = rs.lookup_route(did_with_key_frag, wallet_handle, ws.clone()).unwrap();
        assert_eq!(&endpoint_lookup, "http://localhost:8081");
    }

    //TODO write these tests
//    pub fn test_get_jwm_data_success() {
//
//    }
//
//    pub fn test_get_key_from_str_success() {
//
//    }
//
//    pub fn test_get_sym_key_success() {
//
//    }
//
//    pub fn test_encrypt_ceks_success() {
//
//    }

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

    fn _send_config() -> String {
        json!({"id": "send1"}).to_string()
    }

    fn _recv_config() -> String {
        json!({"id": "recv1"}).to_string()
    }

    fn _config() -> String {
        json!({"id": "w1"}).to_string()
    }

    fn _credentials() -> String {
        json!({"key": "my_key"}).to_string()
    }

    fn _cleanup() {
        TestUtils::cleanup_storage();
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