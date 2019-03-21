extern crate indy_crypto;
extern crate serde_json;
extern crate zeroize;
extern crate regex;

use self::regex::Regex;

use std::collections::HashMap;

use domain::crypto::key::{Key, KeyInfo, KeyMetadata};
use domain::crypto::pack::*;
use errors::prelude::*;
use services::crypto::CryptoService;
use services::wallet::{RecordOptions, WalletService};

use std::rc::Rc;
use std::str;
use utils::crypto::base64;
use utils::crypto::chacha20poly1305_ietf;
use domain::crypto::combo_box::ComboBox;
use api::WalletHandle;
use std::mem::uninitialized;
use serde_json::Value;


lazy_static! {
    static ref IDX_REGEX: Regex = Regex::new(r"^\$[0-9]+$").unwrap();
}


pub enum CryptoCommand {
    CreateKey(
        WalletHandle,
        KeyInfo, // key info
        Box<Fn(IndyResult<String /*verkey*/>) + Send>,
    ),
    SetKeyMetadata(
        WalletHandle,
        String, // verkey
        String, // metadata
        Box<Fn(IndyResult<()>) + Send>,
    ),
    GetKeyMetadata(
        WalletHandle,
        String, // verkey
        Box<Fn(IndyResult<String>) + Send>,
    ),
    CryptoSign(
        WalletHandle,
        String,  // my vk
        Vec<u8>, // msg
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    CryptoVerify(
        String,  // their vk
        Vec<u8>, // msg
        Vec<u8>, // signature
        Box<Fn(IndyResult<bool>) + Send>,
    ),
    AuthenticatedEncrypt(
        WalletHandle,
        String,  // my vk
        String,  // their vk
        Vec<u8>, // msg
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    AuthenticatedDecrypt(
        WalletHandle,
        String,  // my vk
        Vec<u8>, // encrypted msg
        Box<Fn(IndyResult<(String, Vec<u8>)>) + Send>,
    ),
    AnonymousEncrypt(
        String,  // their vk
        Vec<u8>, // msg
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    AnonymousDecrypt(
        WalletHandle,
        String,  // my vk
        Vec<u8>, // msg
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    PackMessage(
        Vec<u8>, // plaintext message
        String,  // list of receiver's keys
        Option<String>,  // senders verkey
        WalletHandle,
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    UnpackMessage(
        Vec<u8>, // JWE
        WalletHandle,
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    PostPCPackedMessage(
        Vec<u8>, // packed message
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    ForwardMessageWithCD(
        String, // type
        String, // to
        Vec<u8>, // packed message
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    PackMsgWithCts(
        Vec<u8>, // plaintext message
        String,  // list of receiver's keys
        Option<String>,  // senders verkey
        WalletHandle,
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    PrePCPackedMessage(
        Vec<u8>, // packed message
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
    RemoveCtsFromMsg(
        Vec<u8>, // JSON message
        Box<Fn(IndyResult<(Vec<u8>, Vec<u8>)>) + Send>,
    ),
    AddCtsToMsg(
        Vec<u8>, // JSON message
        Vec<u8>, // JSON Cts
        Box<Fn(IndyResult<Vec<u8>>) + Send>,
    ),
}

pub struct CryptoCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
}

impl CryptoCommandExecutor {
    pub fn new(
        wallet_service: Rc<WalletService>,
        crypto_service: Rc<CryptoService>,
    ) -> CryptoCommandExecutor {
        CryptoCommandExecutor {
            wallet_service,
            crypto_service,
        }
    }

    pub fn execute(&self, command: CryptoCommand) {
        match command {
            CryptoCommand::CreateKey(wallet_handle, key_info, cb) => {
                info!("CreateKey command received");
                cb(self.create_key(wallet_handle, &key_info));
            }
            CryptoCommand::SetKeyMetadata(wallet_handle, verkey, metadata, cb) => {
                info!("SetKeyMetadata command received");
                cb(self.set_key_metadata(wallet_handle, &verkey, &metadata));
            }
            CryptoCommand::GetKeyMetadata(wallet_handle, verkey, cb) => {
                info!("GetKeyMetadata command received");
                cb(self.get_key_metadata(wallet_handle, &verkey));
            }
            CryptoCommand::CryptoSign(wallet_handle, my_vk, msg, cb) => {
                info!("CryptoSign command received");
                cb(self.crypto_sign(wallet_handle, &my_vk, &msg));
            }
            CryptoCommand::CryptoVerify(their_vk, msg, signature, cb) => {
                info!("CryptoVerify command received");
                cb(self.crypto_verify(&their_vk, &msg, &signature));
            }
            CryptoCommand::AuthenticatedEncrypt(wallet_handle, my_vk, their_vk, msg, cb) => {
                info!("AuthenticatedEncrypt command received");
                cb(self.authenticated_encrypt(wallet_handle, &my_vk, &their_vk, &msg));
            }
            CryptoCommand::AuthenticatedDecrypt(wallet_handle, my_vk, encrypted_msg, cb) => {
                info!("AuthenticatedDecrypt command received");
                cb(self.authenticated_decrypt(wallet_handle, &my_vk, &encrypted_msg));
            }
            CryptoCommand::AnonymousEncrypt(their_vk, msg, cb) => {
                info!("AnonymousEncrypt command received");
                cb(self.anonymous_encrypt(&their_vk, &msg));
            }
            CryptoCommand::AnonymousDecrypt(wallet_handle, my_vk, encrypted_msg, cb) => {
                info!("AnonymousDecrypt command received");
                cb(self.anonymous_decrypt(wallet_handle, &my_vk, &encrypted_msg));
            }
            CryptoCommand::PackMessage(message, receivers, sender_vk, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.pack_msg(message, &receivers, sender_vk, wallet_handle));
            }
            CryptoCommand::UnpackMessage(jwe_json, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(jwe_json, wallet_handle));
            }
            CryptoCommand::PostPCPackedMessage(message, cb) => {
                info!("PostPCPackedMessage command received");
                // TODO: Don't take ownership of message but borrow it
                cb(Self::post_pc_packed_msg(&message));
            }
            CryptoCommand::ForwardMessageWithCD(typ, to, message, cb) => {
                info!("ForwardMessageWithCD command received");
                // TODO: Don't take ownership of message but borrow it
                cb(Self::forward_msg_with_cd(&typ, &to, &message));
            }
            CryptoCommand::PackMsgWithCts(message, receivers, sender_vk, wallet_handle, cb) => {
                info!("PackAlreadyPackedMessage command received");
                cb(self.pack_msg_with_cts(message, &receivers, sender_vk, wallet_handle));
            }
            CryptoCommand::PrePCPackedMessage(message, cb) => {
                info!("PrePCPackedMessage command received");
                // TODO: Don't take ownership of message but borrow it
                cb(Self::pre_pc_packed_msg(&message));
            }
            CryptoCommand::RemoveCtsFromMsg(message, cb) => {
                info!("RemoveCtsFromMsg command received");
                cb(Self::remove_cts_from_msg(&message));
            }
            CryptoCommand::AddCtsToMsg(message, cts, cb) => {
                info!("AddCtsToMsg command received");
                // TODO: Don't take ownership of message but borrow it
                cb(Self::add_cts_to_msg(&message, &cts));
            }
        };
    }

    fn create_key(&self, wallet_handle: WalletHandle, key_info: &KeyInfo) -> IndyResult<String> {
        debug!(
            "create_key >>> wallet_handle: {:?}, key_info: {:?}",
            wallet_handle,
            secret!(key_info)
        );

        let key = self.crypto_service.create_key(key_info)?;
        self.wallet_service
            .add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())?;

        let res = key.verkey.to_string();
        debug!("create_key <<< res: {:?}", res);
        Ok(res)
    }

    fn crypto_sign(&self, wallet_handle: WalletHandle, my_vk: &str, msg: &[u8]) -> IndyResult<Vec<u8>> {
        trace!(
            "crypto_sign >>> wallet_handle: {:?}, sender_vk: {:?}, msg: {:?}",
            wallet_handle, my_vk, msg
        );

        self.crypto_service.validate_key(my_vk)?;

        let key: Key = self.wallet_service.get_indy_object(
            wallet_handle,
            &my_vk,
            &RecordOptions::id_value(),
        )?;

        let res = self.crypto_service.sign(&key, msg)?;

        trace!("crypto_sign <<< res: {:?}", res);

        Ok(res)
    }

    fn crypto_verify(&self,
                     their_vk: &str,
                     msg: &[u8],
                     signature: &[u8]) -> IndyResult<bool> {
        trace!(
            "crypto_verify >>> their_vk: {:?}, msg: {:?}, signature: {:?}",
            their_vk, msg, signature
        );

        self.crypto_service.validate_key(their_vk)?;

        let res = self.crypto_service.verify(their_vk, msg, signature)?;

        trace!("crypto_verify <<< res: {:?}", res);

        Ok(res)
    }

    //TODO begin deprecation process this function. It will be replaced by pack
    fn authenticated_encrypt(
        &self,
        wallet_handle: WalletHandle,
        my_vk: &str,
        their_vk: &str,
        msg: &[u8],
    ) -> IndyResult<Vec<u8>> {
        trace!("authenticated_encrypt >>> wallet_handle: {:?}, my_vk: {:?}, their_vk: {:?}, msg: {:?}", wallet_handle, my_vk, their_vk, msg);

        self.crypto_service.validate_key(my_vk)?;
        self.crypto_service.validate_key(their_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(
            wallet_handle,
            my_vk,
            &RecordOptions::id_value(),
        )?;

        let msg = self.crypto_service.create_combo_box(&my_key, &their_vk, msg)?;

        let msg = msg.to_msg_pack()
            .map_err(|e| err_msg(IndyErrorKind::InvalidState, format!("Can't serialize ComboBox: {:?}", e)))?;

        let res = self.crypto_service.crypto_box_seal(&their_vk, &msg)?;

        trace!("authenticated_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    //TODO begin deprecation process this function. It will be replaced by unpack
    fn authenticated_decrypt(
        &self,
        wallet_handle: WalletHandle,
        my_vk: &str,
        msg: &[u8],
    ) -> IndyResult<(String, Vec<u8>)> {
        trace!("authenticated_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, msg: {:?}", wallet_handle, my_vk, msg);

        self.crypto_service.validate_key(my_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(
            wallet_handle,
            my_vk,
            &RecordOptions::id_value(),
        )?;

        let decrypted_msg = self.crypto_service.crypto_box_seal_open(&my_key, &msg)?;

        let parsed_msg = ComboBox::from_msg_pack(decrypted_msg.as_slice())
            .map_err(|err| err_msg(IndyErrorKind::InvalidStructure, format!("Can't deserialize ComboBox: {:?}", err)))?;

        let doc: Vec<u8> = base64::decode(&parsed_msg.msg)
            .map_err(|err| err_msg(IndyErrorKind::InvalidStructure, format!("Can't decode internal msg filed from base64 {}", err)))?;

        let nonce: Vec<u8> = base64::decode(&parsed_msg.nonce)
            .map_err(|err| err_msg(IndyErrorKind::InvalidStructure, format!("Can't decode nonce from base64 {}", err)))?;

        let decrypted_msg = self.crypto_service.crypto_box_open(&my_key, &parsed_msg.sender, &doc, &nonce)?;

        let res = (parsed_msg.sender, decrypted_msg);

        trace!("authenticated_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_encrypt(&self,
                         their_vk: &str,
                         msg: &[u8]) -> IndyResult<Vec<u8>> {
        trace!(
            "anonymous_encrypt >>> their_vk: {:?}, msg: {:?}",
            their_vk, msg
        );

        self.crypto_service.validate_key(their_vk)?;

        let res = self.crypto_service.crypto_box_seal(their_vk, &msg)?;

        trace!("anonymous_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_decrypt(&self,
                         wallet_handle: WalletHandle,
                         my_vk: &str,
                         encrypted_msg: &[u8]) -> IndyResult<Vec<u8>> {
        trace!(
            "anonymous_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, encrypted_msg: {:?}",
            wallet_handle, my_vk, encrypted_msg
        );

        self.crypto_service.validate_key(&my_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(
            wallet_handle,
            &my_vk,
            &RecordOptions::id_value(),
        )?;

        let res = self
            .crypto_service
            .crypto_box_seal_open(&my_key, &encrypted_msg)?;

        trace!("anonymous_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn set_key_metadata(&self, wallet_handle: WalletHandle, verkey: &str, metadata: &str) -> IndyResult<()> {
        debug!(
            "set_key_metadata >>> wallet_handle: {:?}, verkey: {:?}, metadata: {:?}",
            wallet_handle, verkey, metadata
        );

        self.crypto_service.validate_key(verkey)?;

        let metadata = KeyMetadata {
            value: metadata.to_string(),
        };

        self.wallet_service
            .upsert_indy_object(wallet_handle, &verkey, &metadata)?;

        debug!("set_key_metadata <<<");

        Ok(())
    }

    fn get_key_metadata(&self, wallet_handle: WalletHandle, verkey: &str) -> IndyResult<String> {
        debug!(
            "get_key_metadata >>> wallet_handle: {:?}, verkey: {:?}",
            wallet_handle, verkey
        );

        self.crypto_service.validate_key(verkey)?;

        let metadata = self.wallet_service.get_indy_object::<KeyMetadata>(
            wallet_handle,
            &verkey,
            &RecordOptions::id_value(),
        )?;

        let res = metadata.value;

        debug!("get_key_metadata <<< res: {:?}", res);

        Ok(res)
    }

    //TODO: Refactor pack to be more modular to version changes or crypto_scheme changes
    //this match statement is super messy, but the easiest way to comply with current architecture
    pub fn pack_msg(
        &self,
        message: Vec<u8>,
        receivers: &str,
        sender_vk: Option<String>,
        wallet_handle: WalletHandle,
    ) -> IndyResult<Vec<u8>> {

        //parse receivers to structs
        let receiver_list: Vec<String> = serde_json::from_str(receivers).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize receiver list of keys {}",
                err
            ))
        })?;

        //break early and error out if no receivers keys are provided
        if receiver_list.is_empty() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "No receiver keys found"
            )));
        }

        //generate content encryption key that will encrypt `message`
        let cek = chacha20poly1305_ietf::gen_key();

        let (json_str, base64_protected) = if let Some(sender_vk) = sender_vk {
            self.crypto_service.validate_key(&sender_vk)?;

            //returns authcrypted pack_message format. See Wire message format HIPE for details
            self._prepare_protected_authcrypt(&cek, receiver_list, &sender_vk, wallet_handle)?
        } else {
            //returns anoncrypted pack_message format. See Wire message format HIPE for details
            self._prepare_protected_anoncrypt(&cek, receiver_list)?
        };

        // Use AEAD to encrypt `message` with "protected" data as "associated data"
        let (ciphertext, iv, tag) =
            self.crypto_service
                .encrypt_plaintext(message, &json_str, &cek);

        self._format_pack_message(&base64_protected, &ciphertext, &iv, &tag)
    }

    pub fn unpack_msg(&self, jwe_json: Vec<u8>, wallet_handle: WalletHandle) -> IndyResult<Vec<u8>> {
        //serialize JWE to struct
        let jwe_struct: JWE = serde_json::from_slice(jwe_json.as_slice()).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize JWE {}",
                err
            ))
        })?;
        //decode protected data
        let protected_decoded_vec = base64::decode_urlsafe(&jwe_struct.protected)?;
        let protected_decoded_str = String::from_utf8(protected_decoded_vec).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to utf8 encode data {}",
                err
            ))
        })?;
        //convert protected_data_str to struct
        let protected_struct: Protected = serde_json::from_str(&protected_decoded_str).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize protected data {}",
                err
            ))
        })?;

        //extract recipient that matches a key in the wallet
        let (recipient, is_auth_recipient) = self._find_correct_recipient(protected_struct, wallet_handle)?;

        //get cek and sender data
        let (sender_verkey_option, cek) = if is_auth_recipient {
            self._unpack_cek_authcrypt(recipient.clone(), wallet_handle)
        } else {
            self._unpack_cek_anoncrypt(recipient.clone(), wallet_handle)
        }?; //close cek and sender_data match statement

        //decrypt message
        let message = self.crypto_service.decrypt_ciphertext(
            &jwe_struct.ciphertext,
            &protected_decoded_str,
            &jwe_struct.iv,
            &jwe_struct.tag,
            &cek,
        )?;

        //serialize and return decrypted message
        let res = UnpackMessage {
            message,
            sender_verkey: sender_verkey_option,
            recipient_verkey: recipient.header.kid
        };

        return serde_json::to_vec(&res).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize message {}",
                err
            ))
        });
    }

    pub fn post_pc_packed_msg(packed_msg: &[u8]) -> IndyResult<Vec<u8>> {
        let jwe_cd_struct: JWEWithCD = serde_json::from_slice(packed_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize JWE {}",
                err
            ))
        })?;

        let (iv, tag, ciphertext) = (jwe_cd_struct.iv, jwe_cd_struct.tag, jwe_cd_struct.ciphertext);

        let b1 = IDX_REGEX.is_match(&iv);
        let b2 = IDX_REGEX.is_match(&tag);
        let b3 = IDX_REGEX.is_match(&ciphertext);

        match (b1, b2, b3) {
            (false, false, false) => {
                let mut cts: Vec<EmbeddedCT> = match jwe_cd_struct.ciphertexts {
                    Some(cts) => {
                        let ect: Vec<EmbeddedCT> = serde_json::from_str(&cts).map_err(|err| {
                            err_msg(IndyErrorKind::InvalidStructure, format!(
                                "Failed to deserialize EmbeddedCT vector {}",
                                err
                            ))
                        })?;
                        ect
                    },
                    None => Vec::<EmbeddedCT>::new()
                };
                let next_idx_str = format!("${}", cts.len());
                let new_emd_ct = EmbeddedCT {
                    iv,
                    ciphertext,
                    tag
                };
                cts.push(new_emd_ct);
                let new_jwe_cd = JWEWithCD {
                    protected: jwe_cd_struct.protected,
                    iv: next_idx_str.clone(),
                    ciphertext: next_idx_str.clone(),
                    tag: next_idx_str.clone(),
                    ciphertexts: Some(serde_json::to_string(&cts).map_err(|err| {
                        err_msg(IndyErrorKind::InvalidStructure, format!(
                            "Failed to serialize EmbeddedCT vector {}",
                            err
                        ))
                    })?)
                };
                return serde_json::to_vec(&new_jwe_cd).map_err(|err| {
                    err_msg(IndyErrorKind::InvalidStructure, format!(
                        "Failed to serialize JWEWithCD {}",
                        err
                    ))
                });
            },
            (true, true, true) => {
                if iv == tag && tag == ciphertext {
                    // Already processed so return original message
                    println!("Already processed so return original message");
                    return Ok(packed_msg.to_vec());
                } else {
                    return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                        "all iv, tag, ciphertext should have same index"
                    )));
                }
            },
            (_, _, _) => {
                return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                    "all iv, tag, ciphertext should either have index or not"
                )));
            }
        }
    }

    pub fn forward_msg_with_cd(typ: &str, to: &str, packed_msg: &[u8]) -> IndyResult<Vec<u8>> {
        let mut jwe_cd_struct: JWEWithCD = serde_json::from_slice(packed_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize JWE {}",
                err
            ))
        })?;

        let cts = jwe_cd_struct.ciphertexts.clone();
        jwe_cd_struct.ciphertexts = None;
        let msg = serde_json::to_string(&jwe_cd_struct).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize JWEWithCD {}",
                err
            ))
        })?;

        let fwd = ForwardWithCD {
            msg_type: typ.to_string(),
            fwd: to.to_string(),
            msg,
            ciphertexts: cts
        };

        serde_json::to_vec(&fwd).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize ForwardWithCD {}",
                err
            ))
        })
    }

    // This can be merged with `pack_msg`
    pub fn pack_msg_with_cts(
        &self,
        packed_msg: Vec<u8>,
        receivers: &str,
        sender_vk: Option<String>,
        wallet_handle: WalletHandle,
    ) -> IndyResult<Vec<u8>> {
        let mut msg_json: Value = serde_json::from_slice(&packed_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize message {}",
                err
            ))
        })?;

        let msg_obj = match msg_json.as_object_mut() {
            Some(o) => o,
            None => return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "Expected an object"
            )))
        };
        // TODO: "$ciphertexts" should be a constant
        let special_key = "$ciphertexts";

        if msg_obj.contains_key(special_key) {
            println!("Found special key in pack_already_packed");
            let cts = msg_obj.remove(special_key).unwrap();
            let arg = serde_json::to_vec(&msg_obj).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to serialize ForwardWithCD {}",
                    err
                ))
            })?;
            let new_msg = self.pack_msg(arg, receivers,
                                        sender_vk, wallet_handle)?;
            let mut new_msg_json: Value = serde_json::from_slice(&new_msg).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to deserialize message {}",
                    err
                ))
            })?;
            let new_msg_obj = new_msg_json.as_object_mut().unwrap();
            new_msg_obj.insert(special_key.to_string(), cts);
            serde_json::to_vec(&new_msg_obj).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to serialize {}",
                    err
                ))
            })
        } else {
            println!("Did not find special key in pack_already_packed");
            self.pack_msg(packed_msg, receivers, sender_vk, wallet_handle)
        }
    }

    pub fn remove_cts_from_msg(json_msg: &[u8]) -> IndyResult<(Vec<u8>, Vec<u8>)> {
        let mut json: Value = serde_json::from_slice(&json_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize message to json {}",
                err
            ))
        })?;

        let msg_obj = match json.as_object_mut() {
            Some(o) => o,
            None => return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "Expected an object"
            )))
        };

        // TODO: "$ciphertexts" should be a constant
        let special_key = "$ciphertexts";
        if msg_obj.contains_key(special_key) {
            println!("Found special key in pack_already_packed");
            let cts = msg_obj.remove(special_key).unwrap();

            let new_msg = serde_json::to_vec(&msg_obj).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to serialize msg {}",
                    err
                ))
            })?;
            let cts_msg = serde_json::to_vec(&cts).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to serialize msg {}",
                    err
                ))
            })?;
            Ok((new_msg, cts_msg))
            //Ok((new_msg, Some(cts_msg)))
        } else {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "no $ciphertexts found"
            )));
            //Ok((json_msg, None))
        }
    }

    pub fn add_cts_to_msg(json_msg: &[u8], json_cts: &[u8]) -> IndyResult<Vec<u8>> {
        let mut json: Value = serde_json::from_slice(&json_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize message to json {}",
                err
            ))
        })?;

        let msg_obj = match json.as_object_mut() {
            Some(o) => o,
            None => return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "Expected an object"
            )))
        };

        // TODO: "$ciphertexts" should be a constant
        let special_key = "$ciphertexts";
        if msg_obj.contains_key(special_key) {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                "Already contains $ciphertexts"
            )));
        } else {
            let json_cts: Value = serde_json::from_slice(&json_cts).map_err(|err| {
                err_msg(IndyErrorKind::InvalidStructure, format!(
                    "Failed to deserialize message to json {}",
                    err
                ))
            })?;
            msg_obj.insert(special_key.to_string(), json_cts);
        }

        serde_json::to_vec(&msg_obj).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize {}",
                err
            ))
        })
    }

    pub fn pre_pc_packed_msg(packed_msg: &[u8]) -> IndyResult<Vec<u8>> {
        let jwe_cd_struct: JWEWithCD = serde_json::from_slice(&packed_msg).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to deserialize message to JWEWithCD {}",
                err
            ))
        })?;

        let (iv, tag, ciphertext) = (jwe_cd_struct.iv, jwe_cd_struct.tag, jwe_cd_struct.ciphertext);

        let b1 = IDX_REGEX.is_match(&iv);
        let b2 = IDX_REGEX.is_match(&tag);
        let b3 = IDX_REGEX.is_match(&ciphertext);

        match (b1, b2, b3) {
            (true, true, true) => {
                if iv == tag && tag == ciphertext {
                    let mut cts: Vec<EmbeddedCT> = match jwe_cd_struct.ciphertexts {
                        Some(cts) => {
                            let ect: Vec<EmbeddedCT> = serde_json::from_str(&cts).map_err(|err| {
                                err_msg(IndyErrorKind::InvalidStructure, format!(
                                    "Failed to deserialize EmbeddedCT vector {}",
                                    err
                                ))
                            })?;
                            ect
                        },
                        None => return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                            "no ciphetexts for preprocessing"
                        )))
                    };

                    let idx_str = format!("${}", cts.len()-1);
                    if iv != idx_str {
                        return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                            "wrong index"
                        )))
                    }

                    let ct = cts.pop().unwrap();
                    let new_cts = match cts.len() {
                        0 => None,
                        _ => {
                            Some(serde_json::to_string(&cts).map_err(|err| {
                                err_msg(IndyErrorKind::InvalidStructure, format!(
                                    "Failed to serialize EmbeddedCT vector {}",
                                    err
                                ))
                            })?)
                        }
                    };
                    serde_json::to_vec(&JWEWithCD {
                        protected: jwe_cd_struct.protected,
                        iv: ct.iv,
                        ciphertext: ct.ciphertext,
                        tag: ct.tag,
                        ciphertexts: new_cts
                    }).map_err(|err| {
                        err_msg(IndyErrorKind::InvalidStructure, format!(
                            "Failed to serialize {}",
                            err
                        ))
                    })

                } else {
                    return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                        "all iv, tag, ciphertext should have same index"
                    )));
                }
            },
            (_, _, _) => {
                return Err(err_msg(IndyErrorKind::InvalidStructure, format!(
                    "all iv, tag, ciphertext should have index"
                )));
            }
        }
    }

    fn _prepare_protected_anoncrypt(&self,
                                    cek: &chacha20poly1305_ietf::Key,
                                    receiver_list: Vec<String>,
    ) -> IndyResult<(String, String)> {
        let mut encrypted_recipients_struct : Vec<Recipient> = vec![];

        for their_vk in receiver_list {
            //encrypt sender verkey
            let enc_cek = self.crypto_service.crypto_box_seal(&their_vk, &cek[..])?;

            //create recipient struct and push to encrypted list
            encrypted_recipients_struct.push(Recipient {
                encrypted_key: base64::encode_urlsafe(enc_cek.as_slice()),
                header: Header {
                    kid: their_vk,
                    sender: None,
                    iv: None
                },
            });
        } // end for-loop
        Ok(self._base64_encode_protected(encrypted_recipients_struct, false)?)
    }

    fn _prepare_protected_authcrypt(&self,
                                    cek: &chacha20poly1305_ietf::Key,
                                    receiver_list: Vec<String>, sender_vk: &str,
                                    wallet_handle: WalletHandle,
    ) -> IndyResult<(String, String)> {
        let mut encrypted_recipients_struct : Vec<Recipient> = vec![];

        //get my_key from my wallet
        let my_key = self.wallet_service.get_indy_object(
            wallet_handle,
            sender_vk,
            &RecordOptions::id_value()
        )?;

        //encrypt cek for recipient
        for their_vk in receiver_list {
            let (enc_cek, iv) = self.crypto_service.crypto_box(&my_key, &their_vk, &cek[..])?;

            let enc_sender = self.crypto_service.crypto_box_seal(&their_vk, sender_vk.as_bytes())?;

            //create recipient struct and push to encrypted list
            encrypted_recipients_struct.push(Recipient {
                encrypted_key: base64::encode_urlsafe(enc_cek.as_slice()),
                header: Header {
                    kid: their_vk,
                    sender: Some(base64::encode_urlsafe(enc_sender.as_slice())),
                    iv: Some(base64::encode_urlsafe(iv.as_slice()))
                },
            });
        } // end for-loop

        Ok(self._base64_encode_protected(encrypted_recipients_struct, true)?)
    }

    fn _base64_encode_protected(&self, encrypted_recipients_struct: Vec<Recipient>, alg_is_authcrypt: bool) -> IndyResult<(String, String)> {
        let alg_val = if alg_is_authcrypt { String::from("Authcrypt") } else { String::from("Anoncrypt") };

        //structure protected and base64URL encode it
        let protected_struct = Protected {
            enc: "xchacha20poly1305_ietf".to_string(),
            typ: "JWM/1.0".to_string(),
            alg: alg_val,
            recipients: encrypted_recipients_struct,
        };
        let protected_json = serde_json::to_string(&protected_struct).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize protected field {}",
                err
            ))
        })?;

        let protected_json_b64 = base64::encode_urlsafe(protected_json.as_bytes());
        Ok((protected_json, protected_json_b64))
    }

    fn _format_pack_message(
        &self,
        base64_protected: &str,
        ciphertext: &str,
        iv: &str,
        tag: &str
    ) -> IndyResult<Vec<u8>> {

        //serialize pack message and return as vector of bytes
        let jwe_struct = JWE {
            protected: base64_protected.to_string(),
            iv: iv.to_string(),
            ciphertext: ciphertext.to_string(),
            tag: tag.to_string()
        };

        /*let x = serde_json::to_string(&jwe_struct).unwrap();
        println!("js_l={}", x.len());
        println!("js={}", x);*/

        serde_json::to_vec(&jwe_struct).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!(
                "Failed to serialize JWE {}",
                err
            ))
        })
    }

    fn _find_correct_recipient(&self, protected_struct: Protected, wallet_handle: WalletHandle) -> IndyResult<(Recipient, bool)>{
        for recipient in protected_struct.recipients {
            let my_key_res = self.wallet_service.get_indy_object::<Key>(
                wallet_handle,
                &recipient.header.kid,
                &RecordOptions::id_value()
            );


            if my_key_res.is_ok() {
                return Ok((recipient.clone(), recipient.header.sender.is_some()))
            }
        }
        return Err(IndyError::from(IndyErrorKind::WalletItemNotFound));
    }

    fn _unpack_cek_authcrypt(&self, recipient: Recipient, wallet_handle: WalletHandle) -> IndyResult<(Option<String>, chacha20poly1305_ietf::Key)> {
        let encrypted_key_vec = base64::decode_urlsafe(&recipient.encrypted_key)?;
        let iv = base64::decode_urlsafe(&recipient.header.iv.unwrap())?;
        let enc_sender_vk = base64::decode_urlsafe(&recipient.header.sender.unwrap())?;

        //get my private key
        let my_key = self.wallet_service.get_indy_object(
            wallet_handle,
            &recipient.header.kid,
            &RecordOptions::id_value(),
        )?;

        //decrypt sender_vk
        let sender_vk_vec = self.crypto_service.crypto_box_seal_open(&my_key, enc_sender_vk.as_slice())?;
        let sender_vk = String::from_utf8(sender_vk_vec)
            .map_err(|err| err_msg(IndyErrorKind::InvalidStructure, format!("Failed to utf-8 encode sender_vk {}", err)))?;

        //decrypt cek
        let cek_as_vec = self.crypto_service.crypto_box_open(
            &my_key,
            &sender_vk,
            encrypted_key_vec.as_slice(),
            iv.as_slice())?;

        //convert cek to chacha Key struct
        let cek: chacha20poly1305_ietf::Key =
            chacha20poly1305_ietf::Key::from_slice(&cek_as_vec[..]).map_err(
                |err| {
                    err_msg(IndyErrorKind::InvalidStructure, format!("Failed to decrypt cek {}", err))
                },
            )?;

        Ok((Some(sender_vk), cek))
    }

    fn _unpack_cek_anoncrypt(&self, recipient: Recipient, wallet_handle: WalletHandle) -> IndyResult<(Option<String>, chacha20poly1305_ietf::Key)> {
        let encrypted_key_vec = base64::decode_urlsafe(&recipient.encrypted_key)?;

        //get my private key
        let my_key : Key = self.wallet_service.get_indy_object(
            wallet_handle,
            &recipient.header.kid,
            &RecordOptions::id_value(),
        )?;

        //decrypt cek
        let cek_as_vec = self.crypto_service
            .crypto_box_seal_open(&my_key, encrypted_key_vec.as_slice())?;

        //convert cek to chacha Key struct
        let cek: chacha20poly1305_ietf::Key =
            chacha20poly1305_ietf::Key::from_slice(&cek_as_vec[..]).map_err(
                |err| {
                    err_msg(IndyErrorKind::InvalidStructure, format!("Failed to decrypt cek {}", err))
                },
            )?;

        Ok((None, cek))
    }

}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indy_crypto_post_pc_packed_msg_works() {

    }
}*/
