extern crate indy_crypto;
extern crate serde_json;

use std::collections::HashMap;

use errors::indy::IndyError;
use errors::crypto::CryptoError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use domain::crypto::key::{KeyInfo, Key, KeyMetadata};
use domain::crypto::pack::*;
use services::wallet::{WalletService, RecordOptions};
use services::crypto::CryptoService;

use std::rc::Rc;
use std::str;
use std::result;
use utils::crypto::chacha20poly1305_ietf;
use utils::crypto::base64;

type Result<T> = result::Result<T, IndyError>;

pub enum CryptoCommand {
    CreateKey(
        i32, // wallet handle
        KeyInfo, // key info
        Box<Fn(Result<String/*verkey*/>) + Send>),
    SetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        String, // metadata
        Box<Fn(Result<()>) + Send>),
    GetKeyMetadata(
        i32, // wallet handle
        String, // verkey
        Box<Fn(Result<String>) + Send>),
    CryptoSign(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>>) + Send>),
    CryptoVerify(
        String, // their vk
        Vec<u8>, // msg
        Vec<u8>, // signature
        Box<Fn(Result<bool>) + Send>),
    AuthenticatedEncrypt(
        i32, // wallet handle
        String, // my vk
        String, // their vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>>) + Send>),
    AuthenticatedDecrypt(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // encrypted msg
        Box<Fn(Result<(String, Vec<u8>)>) + Send>),
    AnonymousEncrypt(
        String, // their vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>>) + Send>),
    AnonymousDecrypt(
        i32, // wallet handle
        String, // my vk
        Vec<u8>, // msg
        Box<Fn(Result<Vec<u8>>) + Send>),
    PackMessage(
        Vec<u8>, // plaintext message
        String, // list of receiver's keys
        Option<String>, // senders object
        i32, //wallet handle
        Box<Fn(Result<String /*JWM serialized as string*/>) + Send>,
    ),
    UnpackMessage(
        String, // JWE
        i32,    // wallet handle
        Box<Fn(Result<(String, /*plaintext*/ String, /*sender_vk*/)>) + Send>,
    ),
}

pub struct CryptoCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
}

impl CryptoCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
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
            CryptoCommand::PackMessage(message, receivers, sender, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.pack_msg(message, &receivers, sender, wallet_handle));
            }
            CryptoCommand::UnpackMessage(jwe, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&jwe, wallet_handle));
            }
        };
    }

    fn create_key(&self, wallet_handle: i32, key_info: &KeyInfo) -> Result<String> {
        debug!("create_key >>> wallet_handle: {:?}, key_info: {:?}", wallet_handle, secret!(key_info));

        let key = self.crypto_service.create_key(key_info)?;
        self.wallet_service.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new())?;

        let res = key.verkey;
        debug!("create_key <<< res: {:?}", res);
        Ok(res)
    }

    fn crypto_sign(&self,
                   wallet_handle: i32,
                   my_vk: &str,
                   msg: &[u8]) -> Result<Vec<u8>> {
        debug!("crypto_sign >>> wallet_handle: {:?}, sender_vk: {:?}, msg: {:?}", wallet_handle, my_vk, msg);

        self.crypto_service.validate_key(my_vk)?;

        let key: Key = self.wallet_service.get_indy_object(wallet_handle, &my_vk, &RecordOptions::id_value())?;

        let res = self.crypto_service.sign(&key, msg)?;

        debug!("crypto_sign <<< res: {:?}", res);

        Ok(res)
    }

    fn crypto_verify(&self,
                     their_vk: &str,
                     msg: &[u8],
                     signature: &[u8]) -> Result<bool> {
        debug!("crypto_verify >>> their_vk: {:?}, msg: {:?}, signature: {:?}", their_vk, msg, signature);

        self.crypto_service.validate_key(their_vk)?;

        let res = self.crypto_service.verify(their_vk, msg, signature)?;

        debug!("crypto_verify <<< res: {:?}", res);

        Ok(res)
    }

    fn authenticated_encrypt(&self,
                             wallet_handle: i32,
                             my_vk: &str,
                             their_vk: &str,
                             msg: &[u8]) -> Result<Vec<u8>> {
        debug!("authenticated_encrypt >>> wallet_handle: {:?}, my_vk: {:?}, their_vk: {:?}, msg: {:?}", wallet_handle, my_vk, their_vk, msg);

        self.crypto_service.validate_key(my_vk)?;
        self.crypto_service.validate_key(their_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())?;

        let res = self.crypto_service.authenticated_encrypt(&my_key, their_vk, msg)?;

        debug!("authenticated_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn authenticated_decrypt(&self,
                             wallet_handle: i32,
                             my_vk: &str,
                             msg: &[u8]) -> Result<(String, Vec<u8>)> {
        debug!("authenticated_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, msg: {:?}", wallet_handle, my_vk, msg);

        self.crypto_service.validate_key(my_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())?;

        let res = self.crypto_service.authenticated_decrypt(&my_key, &msg)?;

        debug!("authenticated_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_encrypt(&self,
                         their_vk: &str,
                         msg: &[u8]) -> Result<Vec<u8>> {
        debug!("anonymous_encrypt >>> their_vk: {:?}, msg: {:?}", their_vk, msg);

        self.crypto_service.validate_key(their_vk)?;

        let res = self.crypto_service.crypto_box_seal(their_vk, &msg)?;

        debug!("anonymous_encrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn anonymous_decrypt(&self,
                         wallet_handle: i32,
                         my_vk: &str,
                         encrypted_msg: &[u8]) -> Result<Vec<u8>> {
        debug!("anonymous_decrypt >>> wallet_handle: {:?}, my_vk: {:?}, encrypted_msg: {:?}", wallet_handle, my_vk, encrypted_msg);

        self.crypto_service.validate_key(&my_vk)?;

        let my_key: Key = self.wallet_service.get_indy_object(wallet_handle, &my_vk, &RecordOptions::id_value())?;

        let res = self.crypto_service.crypto_box_seal_open(&my_key, &encrypted_msg)?;

        debug!("anonymous_decrypt <<< res: {:?}", res);

        Ok(res)
    }

    fn set_key_metadata(&self, wallet_handle: i32, verkey: &str, metadata: &str) -> Result<()> {
        debug!("set_key_metadata >>> wallet_handle: {:?}, verkey: {:?}, metadata: {:?}", wallet_handle, verkey, metadata);

        self.crypto_service.validate_key(verkey)?;

        let metadata = KeyMetadata {value: metadata.to_string()};

        self.wallet_service.upsert_indy_object(wallet_handle, &verkey, &metadata)?;

        debug!("set_key_metadata <<<");

        Ok(())
    }

    fn get_key_metadata(&self,
                        wallet_handle: i32,
                        verkey: &str) -> Result<String> {
        debug!("get_key_metadata >>> wallet_handle: {:?}, verkey: {:?}", wallet_handle, verkey);

        self.crypto_service.validate_key(verkey)?;

        let metadata = self.wallet_service.get_indy_object::<KeyMetadata>(wallet_handle, &verkey, &RecordOptions::id_value())?;

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
        wallet_handle: i32
    ) -> Result<String> {

        //generate symmetrical key
        let sym_key = chacha20poly1305_ietf::gen_key();

        //list of ceks used to construct JWE later
        let mut encrypted_recipients_struct: Vec<Recipient> = vec![];

        match sender_vk {

            Some(verkey) => {
                //TODO find more readable way to perform authcrypt funtionality something like private command function

                //get my_key from my wallet
                let my_key = self.wallet_service
                    .get_indy_object(wallet_handle, &verkey, &RecordOptions::id_value())?;

                //parse receivers to structs
                let receiver_list : Vec<String> = serde_json::from_str(receivers)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to deserialize receiver list of keys {}", err)
                                )
                            )
                        )
                    )?;

                //encrypt sym_key for recipient
                for their_vk in receiver_list {
                    let enc_sym_key = self.crypto_service
                        .authenticated_encrypt(&my_key, &their_vk, &sym_key[..])?;

                    //create recipient struct and push to encrypted list
                    encrypted_recipients_struct.push(
                        Recipient {
                            encrypted_key: base64::encode(enc_sym_key.as_slice()),
                            header: Header {
                                kid: base64::encode(&their_vk.as_bytes())
                            }
                        });
                } // end for-loop

                //structure protected and base64URL encode it
                let protected_struct = Protected {
                    enc: "xchacha20poly1305".to_string(),
                    typ: "JWM/1.0".to_string(),
                    alg: "Authcrypt".to_string(),
                    recipients: encrypted_recipients_struct,
                };
                let protected_encoded = serde_json::to_string(&protected_struct)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to serialize protected field {}", err))))
                    )?;

                let base64_protected = base64::encode(protected_encoded.as_bytes());

                // encrypt ciphertext and integrity protect "protected" field
                let (iv, ciphertext, tag) = self.crypto_service
                    .encrypt_plaintext(message, &base64_protected, &sym_key);

                //construct JWE struct
                let jwe_struct = JWE {
                    protected : base64_protected,
                    iv,
                    ciphertext,
                    tag
                };

                //convert JWE struct to a string and return
                serde_json::to_string(&jwe_struct)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to serialize JWE {}", err)
                                )
                            )
                        )
                    )
            },

            None => {
                //TODO find more readable way to perform anoncrypt funtionality something like private command function

                //parse receivers to structs
                let receiver_list : Vec<String> = serde_json::from_str(receivers)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to deserialize receiver list of keys {}", err)
                                )
                            )
                        )
                    )?;

                //encrypt sym_key for recipient
                for their_vk in receiver_list {

                    //encrypt sender verkey
                    let enc_sym_key = self.crypto_service
                        .crypto_box_seal(&their_vk, &sym_key[..])?;

                    //create recipient struct and push to encrypted list
                    encrypted_recipients_struct.push(
                        Recipient {
                            encrypted_key: base64::encode(enc_sym_key.as_slice()),
                            header: Header {
                                kid: base64::encode(&their_vk.as_bytes())
                            }
                        });
                } // end for-loop

                //structure protected and base64URL encode it
                let protected_struct = Protected {
                    enc: "xchacha20poly1305".to_string(),
                    typ: "JWM/1.0".to_string(),
                    alg: "Anoncrypt".to_string(),
                    recipients: encrypted_recipients_struct,
                };
                let protected_encoded = base64::encode(serde_json::to_string(&protected_struct)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to serialize JWE {}", err)
                                )
                            )
                        )
                    )?.as_bytes());

                // encrypt ciphertext
                let (iv, ciphertext, tag) = self.crypto_service
                    .encrypt_plaintext(message, &protected_encoded, &sym_key);

                //serialize JWE struct
                let jwe_struct = JWE {
                    protected : protected_encoded,
                    iv,
                    ciphertext,
                    tag
                };

                //return JWE_json
                return serde_json::to_string(&jwe_struct)
                    .map_err(|err|
                        IndyError::CryptoError(
                            CryptoError::CommonError(
                                CommonError::InvalidStructure(
                                    format!("Failed to serialize JWE {}", err)
                                )
                            )
                        )
                    )
            }
        }
    }

    pub fn unpack_msg(
        &self,
        jwe_json: &str,
        wallet_handle: i32
    ) -> Result<(String, String)> {

        //serialize JWE to struct
        let jwe_struct : JWE = serde_json::from_str(jwe_json)
            .map_err(|err|
                IndyError::CryptoError(
                    CryptoError::CommonError(
                        CommonError::InvalidStructure(
                            format!("Failed to deserialize auth ames {}", err)
                        )
                    )
                )
            )?;

        //decode protected data
        let protected_decoded_vec = base64::decode(&jwe_struct.protected)?;
        let protected_decoded_str = String::from_utf8(protected_decoded_vec)
            .map_err(|err|
                IndyError::CryptoError(
                    CryptoError::CommonError(
                        CommonError::InvalidStructure(
                            format!("Failed to utf8 encode data {}", err)
                        )
                    )
                )
            )?;

        //convert protected_data_str to struct
        let protected_struct : Protected = serde_json::from_str(&protected_decoded_str)
            .map_err(|err|
                IndyError::CryptoError(
                    CryptoError::CommonError(
                        CommonError::InvalidStructure(
                            format!("Failed to deserialize protected data {}", err)
                        )
                    )
                )
            )?;

        //search through recipients_list and check if one of the kid matches a verkey in the wallet
        for recipient in protected_struct.recipients {
            let key_in_wallet_result = self.wallet_service
                .get_indy_object::<KeyMetadata>(wallet_handle, &recipient.header.kid, &RecordOptions::id_value());

            if key_in_wallet_result.is_ok() {
                //TODO change to move this to a separate function and return recipient rather than
                // putting logic inside the for loop for loops have no way to return values in rust

                //decode encrypted_key
                let encrypted_key_vec = base64::decode(&recipient.encrypted_key)?;

                //get sym_key and sender data
                let (sender, sym_key) = match protected_struct.alg.as_ref() {
                    "Authcrypt" => {

                        //get my key based on kid
                        let my_key = self.wallet_service
                            .get_indy_object(wallet_handle, &recipient.header.kid, &RecordOptions::id_value())?;

                        //decrypt sym_key
                        let (sender_vk , sym_key_as_vec) = self.crypto_service
                            .authenticated_decrypt(&my_key, encrypted_key_vec.as_slice())?;

                        //convert to chacha Key struct
                        let sym_key : chacha20poly1305_ietf::Key = chacha20poly1305_ietf::Key::from_slice(&sym_key_as_vec[..])
                            .map_err(|err|
                                IndyError::CryptoError(
                                    CryptoError::CommonError(
                                        CommonError::InvalidStructure(
                                            format!("Failed to decrypt sym_key {}", err)
                                        )
                                    )
                                )
                            )?;

                        Ok((sender_vk, sym_key))
                    },

                    "Anoncrypt" => {
                        //get my private key
                        let my_key = self.wallet_service
                            .get_indy_object(wallet_handle, &recipient.header.kid, &RecordOptions::id_value())?;

                        //decrypt sym_key
                        let sym_key_as_vec = self.crypto_service
                            .crypto_box_seal_open(&my_key, encrypted_key_vec.as_slice())?;

                        //convert to chacha Key struct
                        let sym_key : chacha20poly1305_ietf::Key = chacha20poly1305_ietf::Key::from_slice(&sym_key_as_vec[..])
                            .map_err(|err|
                                IndyError::CryptoError(
                                    CryptoError::CommonError(
                                        CommonError::InvalidStructure(
                                            format!("Failed to decrypt sym_key {}", err)
                                        )
                                    )
                                )
                            )?;

                        Ok((String::from(""), sym_key ))
                    },

                    _ => Err(
                            IndyError::CryptoError(
                                CryptoError::CommonError(
                                    CommonError::InvalidStructure(
                                        format!("Failed to deserialize sym_key encryption alg")
                                    )
                                )
                            )
                        )
                }?;

                let message = self.crypto_service
                    .decrypt_ciphertext(&jwe_struct.ciphertext,
                                        &jwe_struct.protected,
                                        &jwe_struct.iv,
                                        &jwe_struct.tag,
                                        &sym_key)?;

                return Ok((message, sender))
            } // close if statement if a kid matches a verkey found in wallet
        } // close for loop searching through recipients on kid

        //If it gets to this point no verkey was found in wallet that matches a kid
        return Err(IndyError::WalletError(WalletError::ItemNotFound))
    }
}
